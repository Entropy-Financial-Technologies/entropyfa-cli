use crate::models::tax_request::{FederalTaxRequest, TaxBracket};
use crate::models::tax_response::{BracketDetail, FederalTaxResponse, PayrollTaxDetail};

pub fn run_federal_tax(req: &FederalTaxRequest) -> FederalTaxResponse {
    let income = &req.income;
    let params = &req.tax_parameters;
    let payroll = &params.payroll;

    // Step 1: Capital loss limit
    let net_capital_gains = income.short_term_capital_gains + income.long_term_capital_gains;
    let capped_net_cg = if net_capital_gains < 0.0 {
        net_capital_gains.max(-params.capital_loss_limit)
    } else {
        net_capital_gains
    };

    // Step 2: Payroll taxes
    // Employee FICA on wages
    let wages = income.wages.max(0.0);
    let employee_ss = payroll.social_security_rate * wages.min(payroll.social_security_wage_base);
    let employee_medicare = payroll.medicare_rate * wages;

    // Self-employment tax
    let se_income_positive = income.self_employment_income.max(0.0);
    let se_base = se_income_positive * 0.9235;
    let remaining_wage_room = (payroll.social_security_wage_base - wages).max(0.0);
    let se_ss = payroll.self_employment_tax_rate * se_base.min(remaining_wage_room);
    let se_medicare = payroll.self_employment_medicare_rate * se_base;

    // Additional Medicare Tax (0.9% on earned income above threshold)
    let combined_earned = wages + se_income_positive;
    let additional_medicare = payroll.additional_medicare_rate
        * (combined_earned - payroll.additional_medicare_threshold).max(0.0);

    // SE tax deduction = half of (SE_SS + SE_Medicare), excludes Additional Medicare
    let se_tax_deduction = 0.5 * (se_ss + se_medicare);

    let payroll_total = employee_ss + employee_medicare + se_ss + se_medicare + additional_medicare;

    let payroll_detail = PayrollTaxDetail {
        employee_social_security: round2(employee_ss),
        employee_medicare: round2(employee_medicare),
        se_social_security: round2(se_ss),
        se_medicare: round2(se_medicare),
        additional_medicare: round2(additional_medicare),
        self_employment_tax_deduction: round2(se_tax_deduction),
        total: round2(payroll_total),
    };

    // Step 3: AGI
    // Gross income = all income fields (with capped net CG replacing raw CG fields)
    // We need to reconstruct: remove raw STCG + LTCG, add capped_net_cg
    let gross_income = wages
        + income.self_employment_income
        + income.taxable_interest
        + income.ordinary_dividends
        + income.qualified_dividends
        + capped_net_cg
        + income.taxable_ira_distributions
        + income.taxable_pensions
        + income.taxable_social_security
        + income.other_income;
    // tax_exempt_interest is excluded from gross income

    let adjustments_from_input = req.adjustments.hsa_deduction
        + req.adjustments.ira_deduction
        + req.adjustments.student_loan_interest
        + req.adjustments.other_adjustments;
    let adjustments_total = adjustments_from_input + se_tax_deduction;

    let agi = gross_income - adjustments_total;

    // Step 4: Deduction
    let is_mfs = req.filing_status == "married_filing_separately";
    let force_itemized = is_mfs && req.deductions.spouse_itemizes.unwrap_or(false);

    let (deduction_used, deduction_method) =
        if req.deductions.method == "itemized" || force_itemized {
            let amount = compute_itemized_deduction(req, agi);
            (amount, "itemized".to_string())
        } else {
            (params.standard_deduction, "standard".to_string())
        };

    // Step 5: Taxable income and split
    let taxable_income = (agi - deduction_used).max(0.0);

    // Preferential = QD + positive LTCG only
    let positive_ltcg = income.long_term_capital_gains.max(0.0);
    let preferential_raw = income.qualified_dividends + positive_ltcg;
    let preferential_income = preferential_raw.min(taxable_income);
    let ordinary_taxable = taxable_income - preferential_income;

    // Step 6: Ordinary income tax
    let (ordinary_income_tax, ordinary_bracket_detail) =
        apply_brackets(&params.ordinary_brackets, ordinary_taxable);

    // Step 7: Capital gains tax (stacking method)
    let (capital_gains_tax, cg_bracket_detail) = apply_cg_brackets(
        &params.capital_gains_brackets,
        ordinary_taxable,
        preferential_income,
    );

    // Step 8: Safety check — use lesser of preferential method vs all-ordinary
    let (all_ordinary_tax, _) = apply_brackets(&params.ordinary_brackets, taxable_income);
    let preferential_method_tax = ordinary_income_tax + capital_gains_tax;

    let (final_ordinary_tax, final_cg_tax) = if all_ordinary_tax < preferential_method_tax {
        // All-ordinary is cheaper; attribute it all as ordinary, zero CG
        (all_ordinary_tax, 0.0)
    } else {
        (ordinary_income_tax, capital_gains_tax)
    };

    // Step 9: NIIT
    // Net investment income = interest + dividends (ordinary + qualified) + CG (after loss limit, floored at 0)
    let net_investment_income = (income.taxable_interest
        + income.ordinary_dividends
        + income.qualified_dividends
        + capped_net_cg)
        .max(0.0);
    let magi = agi; // for domestic taxpayers
    let niit =
        params.niit.rate * net_investment_income.min((magi - params.niit.threshold).max(0.0));

    // Step 10: Totals
    let total_income_tax = final_ordinary_tax + final_cg_tax + niit;
    let total_tax = total_income_tax + payroll_total;
    let effective_rate = total_income_tax / agi.max(1.0);
    let effective_rate_with_payroll = total_tax / agi.max(1.0);

    let marginal_ordinary_rate = find_marginal_rate(&params.ordinary_brackets, ordinary_taxable);
    let stack_top = ordinary_taxable + preferential_income;
    let marginal_cg_rate_base = find_marginal_rate(&params.capital_gains_brackets, stack_top);
    let niit_marginal = if magi > params.niit.threshold {
        params.niit.rate
    } else {
        0.0
    };
    let marginal_capital_gains_rate = marginal_cg_rate_base + niit_marginal;

    FederalTaxResponse {
        gross_income: round2(gross_income),
        adjustments_total: round2(adjustments_total),
        agi: round2(agi),
        deduction_used: round2(deduction_used),
        deduction_method,
        taxable_income: round2(taxable_income),
        ordinary_taxable: round2(ordinary_taxable),
        preferential_income: round2(preferential_income),
        ordinary_income_tax: round2(final_ordinary_tax),
        capital_gains_tax: round2(final_cg_tax),
        niit: round2(niit),
        total_income_tax: round2(total_income_tax),
        payroll_tax: payroll_detail,
        total_tax: round2(total_tax),
        effective_rate: round4(effective_rate),
        effective_rate_with_payroll: round4(effective_rate_with_payroll),
        marginal_ordinary_rate,
        marginal_capital_gains_rate,
        ordinary_bracket_detail: ordinary_bracket_detail
            .into_iter()
            .filter(|b| b.income_in_bracket > 0.0)
            .collect(),
        capital_gains_bracket_detail: cg_bracket_detail
            .into_iter()
            .filter(|b| b.income_in_bracket > 0.0)
            .collect(),
    }
}

fn apply_brackets(brackets: &[TaxBracket], income: f64) -> (f64, Vec<BracketDetail>) {
    let mut total_tax = 0.0;
    let mut details = Vec::new();

    for bracket in brackets {
        let bracket_max = bracket.max.unwrap_or(f64::MAX);
        let taxable_in_bracket = (income.min(bracket_max) - bracket.min).max(0.0);
        let tax = taxable_in_bracket * bracket.rate;
        total_tax += tax;
        details.push(BracketDetail {
            rate: bracket.rate,
            income_in_bracket: round2(taxable_in_bracket),
            tax_in_bracket: round2(tax),
        });
    }

    (total_tax, details)
}

fn apply_cg_brackets(
    brackets: &[TaxBracket],
    stack_start: f64,
    preferential: f64,
) -> (f64, Vec<BracketDetail>) {
    let stack_end = stack_start + preferential;
    let mut total_tax = 0.0;
    let mut details = Vec::new();

    for bracket in brackets {
        let bracket_max = bracket.max.unwrap_or(f64::MAX);
        let effective_min = stack_start.max(bracket.min);
        let effective_max = stack_end.min(bracket_max);
        let income_in_bracket = (effective_max - effective_min).max(0.0);
        let tax = income_in_bracket * bracket.rate;
        total_tax += tax;
        details.push(BracketDetail {
            rate: bracket.rate,
            income_in_bracket: round2(income_in_bracket),
            tax_in_bracket: round2(tax),
        });
    }

    (total_tax, details)
}

fn find_marginal_rate(brackets: &[TaxBracket], income: f64) -> f64 {
    if income <= 0.0 {
        return brackets.first().map(|b| b.rate).unwrap_or(0.0);
    }
    for bracket in brackets.iter().rev() {
        if income > bracket.min {
            return bracket.rate;
        }
    }
    brackets.first().map(|b| b.rate).unwrap_or(0.0)
}

fn compute_itemized_deduction(req: &FederalTaxRequest, agi: f64) -> f64 {
    if let Some(amount) = req.deductions.itemized_amount {
        return amount;
    }

    if !req.deductions.has_detailed_itemized_inputs() {
        return 0.0;
    }

    let raw_salt = req.deductions.raw_salt_amount();
    let allowed_salt = req
        .tax_parameters
        .salt
        .as_ref()
        .map(|params| compute_allowed_salt_deduction(raw_salt, agi, params))
        .unwrap_or(raw_salt);

    req.deductions.other_itemized_amount() + allowed_salt
}

fn compute_allowed_salt_deduction(
    raw_salt: f64,
    agi: f64,
    params: &crate::models::tax_request::SaltDeductionParams,
) -> f64 {
    let phased_cap = (params.cap_amount
        - params.phaseout_rate * (agi - params.phaseout_threshold).max(0.0))
    .max(params.floor_amount);
    raw_salt.min(phased_cap.max(0.0))
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn round4(v: f64) -> f64 {
    (v * 10000.0).round() / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tax_request::*;

    // 2025 single brackets
    fn single_ordinary_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                min: 0.0,
                max: Some(11925.0),
                rate: 0.10,
            },
            TaxBracket {
                min: 11925.0,
                max: Some(48475.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 48475.0,
                max: Some(103350.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 103350.0,
                max: Some(197300.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 197300.0,
                max: Some(250525.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 250525.0,
                max: Some(626350.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 626350.0,
                max: None,
                rate: 0.37,
            },
        ]
    }

    fn single_cg_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                min: 0.0,
                max: Some(48350.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 48350.0,
                max: Some(533400.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 533400.0,
                max: None,
                rate: 0.20,
            },
        ]
    }

    #[allow(dead_code)]
    fn mfj_ordinary_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                min: 0.0,
                max: Some(23850.0),
                rate: 0.10,
            },
            TaxBracket {
                min: 23850.0,
                max: Some(96950.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 96950.0,
                max: Some(206700.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 206700.0,
                max: Some(394600.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 394600.0,
                max: Some(501050.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 501050.0,
                max: Some(751600.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 751600.0,
                max: None,
                rate: 0.37,
            },
        ]
    }

    #[allow(dead_code)]
    fn mfj_cg_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                min: 0.0,
                max: Some(96700.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 96700.0,
                max: Some(600050.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 600050.0,
                max: None,
                rate: 0.20,
            },
        ]
    }

    fn default_payroll() -> PayrollParams {
        PayrollParams {
            social_security_rate: 0.062,
            social_security_wage_base: 176100.0,
            self_employment_tax_rate: 0.124,
            medicare_rate: 0.0145,
            self_employment_medicare_rate: 0.029,
            additional_medicare_rate: 0.009,
            additional_medicare_threshold: 200000.0, // single
        }
    }

    fn default_salt() -> crate::models::tax_request::SaltDeductionParams {
        crate::models::tax_request::SaltDeductionParams {
            cap_amount: 40400.0,
            phaseout_threshold: 505000.0,
            phaseout_rate: 0.30,
            floor_amount: 10000.0,
        }
    }

    #[allow(dead_code)]
    fn mfj_payroll() -> PayrollParams {
        PayrollParams {
            additional_medicare_threshold: 250000.0,
            ..default_payroll()
        }
    }

    fn make_request(
        filing_status: &str,
        income: IncomeBreakdown,
        brackets: Vec<TaxBracket>,
        cg_brackets: Vec<TaxBracket>,
        standard_deduction: f64,
        payroll: PayrollParams,
        niit_threshold: f64,
    ) -> FederalTaxRequest {
        FederalTaxRequest {
            filing_status: filing_status.to_string(),
            income,
            adjustments: Adjustments::default(),
            deductions: DeductionConfig {
                method: "standard".to_string(),
                itemized_amount: None,
                spouse_itemizes: None,
                state_local_income_or_sales_tax: None,
                real_property_tax: None,
                personal_property_tax: None,
                other_itemized_deductions: None,
            },
            tax_parameters: TaxParameters {
                ordinary_brackets: brackets,
                capital_gains_brackets: cg_brackets,
                standard_deduction,
                capital_loss_limit: 3000.0,
                niit: NiitParams {
                    rate: 0.038,
                    threshold: niit_threshold,
                },
                payroll,
                salt: Some(default_salt()),
            },
        }
    }

    fn zero_income() -> IncomeBreakdown {
        IncomeBreakdown {
            wages: 0.0,
            self_employment_income: 0.0,
            taxable_interest: 0.0,
            tax_exempt_interest: 0.0,
            ordinary_dividends: 0.0,
            qualified_dividends: 0.0,
            short_term_capital_gains: 0.0,
            long_term_capital_gains: 0.0,
            taxable_ira_distributions: 0.0,
            taxable_pensions: 0.0,
            taxable_social_security: 0.0,
            other_income: 0.0,
        }
    }

    // Test 1: Zero income
    #[test]
    fn test_zero_income() {
        let req = make_request(
            "single",
            zero_income(),
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        assert_eq!(res.gross_income, 0.0);
        assert_eq!(res.agi, 0.0);
        assert_eq!(res.taxable_income, 0.0);
        assert_eq!(res.ordinary_income_tax, 0.0);
        assert_eq!(res.capital_gains_tax, 0.0);
        assert_eq!(res.niit, 0.0);
        assert_eq!(res.total_income_tax, 0.0);
        assert_eq!(res.payroll_tax.total, 0.0);
        assert_eq!(res.total_tax, 0.0);
    }

    // Test 2: Wages only, single filer, standard deduction
    #[test]
    fn test_wages_only_single() {
        let income = IncomeBreakdown {
            wages: 75000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        assert_eq!(res.gross_income, 75000.0);
        assert_eq!(res.agi, 75000.0);
        assert_eq!(res.taxable_income, 60000.0);
        assert_eq!(res.deduction_method, "standard");

        // 10% on 11925 = 1192.50
        // 12% on (48475 - 11925) = 36550 * 0.12 = 4386.00
        // 22% on (60000 - 48475) = 11525 * 0.22 = 2535.50
        // Total = 8114.00
        assert_eq!(res.ordinary_income_tax, 8114.0);
        assert_eq!(res.capital_gains_tax, 0.0);
        assert_eq!(res.niit, 0.0);

        // Payroll: SS = 0.062 * 75000 = 4650, Medicare = 0.0145 * 75000 = 1087.50
        assert_eq!(res.payroll_tax.employee_social_security, 4650.0);
        assert_eq!(res.payroll_tax.employee_medicare, 1087.5);
    }

    // Test 3: LTCG stacking
    #[test]
    fn test_ltcg_stacking() {
        let income = IncomeBreakdown {
            wages: 50000.0,
            long_term_capital_gains: 30000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // AGI = 80000, taxable = 65000
        // Preferential = 30000 (LTCG)
        // Ordinary taxable = 35000
        assert_eq!(res.ordinary_taxable, 35000.0);
        assert_eq!(res.preferential_income, 30000.0);

        // CG stacks from 35000 to 65000
        // 0% bracket goes to 48350 → 48350 - 35000 = 13350 at 0%
        // 15% bracket 48350+ → 65000 - 48350 = 16650 at 15% = 2497.50
        assert_eq!(res.capital_gains_tax, 2497.5);
    }

    // Test 4: Low income with LTCG — all in 0% bracket
    #[test]
    fn test_low_income_ltcg_zero_rate() {
        let income = IncomeBreakdown {
            wages: 20000.0,
            long_term_capital_gains: 10000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // AGI = 30000, taxable = 15000
        // Ordinary taxable = 5000, preferential = 10000
        // CG stacks 5000 to 15000 — all below 48350 → 0% CG rate
        assert_eq!(res.capital_gains_tax, 0.0);
    }

    // Test 5: Qualified dividends taxed at CG rates
    #[test]
    fn test_qualified_dividends_preferential() {
        let income = IncomeBreakdown {
            wages: 50000.0,
            qualified_dividends: 20000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // AGI = 70000, taxable = 55000
        // Preferential = 20000 (QD), ordinary = 35000
        assert_eq!(res.preferential_income, 20000.0);
        assert_eq!(res.ordinary_taxable, 35000.0);

        // CG stacks from 35000 to 55000
        // 0% up to 48350 → 13350 at 0%
        // 15% from 48350 to 55000 → 6650 at 15% = 997.50
        assert_eq!(res.capital_gains_tax, 997.5);
    }

    // Test 6: Safety check — min(preferential_method, all_ordinary)
    #[test]
    fn test_safety_check() {
        // This would mainly trigger with unusual bracket configurations.
        // With standard US brackets the preferential method is always <= all-ordinary.
        // Verify the mechanism works by checking total_income_tax <= all-ordinary tax.
        let income = IncomeBreakdown {
            wages: 50000.0,
            qualified_dividends: 10000.0,
            long_term_capital_gains: 5000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // Compute what all-ordinary would be: taxable = 50000
        let (all_ordinary, _) = apply_brackets(&single_ordinary_brackets(), res.taxable_income);
        // Preferential method should be <= all-ordinary
        let pref_method = res.ordinary_income_tax + res.capital_gains_tax;
        assert!(pref_method <= round2(all_ordinary) + 0.01);
    }

    // Test 7: NIIT triggered
    #[test]
    fn test_niit_triggered() {
        let income = IncomeBreakdown {
            wages: 250000.0,
            taxable_interest: 30000.0,
            qualified_dividends: 20000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // AGI = 300000, NIIT threshold = 200000
        // NII = 30000 + 20000 = 50000
        // NIIT = 0.038 * min(50000, 300000 - 200000) = 0.038 * 50000 = 1900
        assert_eq!(res.niit, 1900.0);
    }

    // Test 8: NIIT not triggered
    #[test]
    fn test_niit_not_triggered() {
        let income = IncomeBreakdown {
            wages: 140000.0,
            taxable_interest: 10000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // AGI = 150000 < 200000 threshold
        assert_eq!(res.niit, 0.0);
    }

    // Test 9: NIIT components — SE excluded, QD included, tax-exempt interest excluded
    #[test]
    fn test_niit_components() {
        let income = IncomeBreakdown {
            wages: 180000.0,
            self_employment_income: 50000.0,
            taxable_interest: 10000.0,
            tax_exempt_interest: 5000.0, // excluded from NII and gross income
            qualified_dividends: 15000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // NII = taxable_interest(10000) + QD(15000) = 25000
        // SE income is NOT in NII. Tax-exempt interest NOT in NII.
        // AGI = 180000 + 50000 + 10000 + 15000 - se_tax_deduction
        // SE base = 50000 * 0.9235 = 46175
        // SE SS = 0.124 * 46175 = 5725.70 (remaining wage room = 176100-180000 = 0, so actually 0!)
        // Wait — wages 180000 > 0 but < 176100? No, 180000 > 176100, so remaining wage room = 0
        // SE SS = 0
        // SE Medicare = 0.029 * 46175 = 1339.075
        // SE deduction = 0.5 * (0 + 1339.075) = 669.5375 → 669.54
        // AGI = 255000 - 669.54 = 254330.46
        // NIIT = 0.038 * min(25000, 254330.46 - 200000) = 0.038 * 25000 = 950
        assert_eq!(res.niit, 950.0);
    }

    // Test 10: SS caps at wage base
    #[test]
    fn test_ss_caps_at_wage_base() {
        let income = IncomeBreakdown {
            wages: 200000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // SS = 0.062 * 176100 = 10918.20 (capped, not 0.062 * 200000)
        assert_eq!(res.payroll_tax.employee_social_security, 10918.2);
    }

    // Test 11: Additional Medicare
    #[test]
    fn test_additional_medicare() {
        let income = IncomeBreakdown {
            wages: 250000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // Additional Medicare = 0.009 * (250000 - 200000) = 0.009 * 50000 = 450
        assert_eq!(res.payroll_tax.additional_medicare, 450.0);
    }

    // Test 12: SE tax calculation
    #[test]
    fn test_se_tax() {
        let income = IncomeBreakdown {
            self_employment_income: 100000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // SE base = 100000 * 0.9235 = 92350
        // SE SS = 0.124 * 92350 = 11451.40 (within wage base since no wages)
        // SE Medicare = 0.029 * 92350 = 2678.15
        assert_eq!(res.payroll_tax.se_social_security, 11451.4);
        assert_eq!(res.payroll_tax.se_medicare, 2678.15);
    }

    // Test 13: SE tax + wages — SS wage base reduced by W-2 wages
    #[test]
    fn test_se_tax_with_wages() {
        let income = IncomeBreakdown {
            wages: 150000.0,
            self_employment_income: 50000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // SE base = 50000 * 0.9235 = 46175
        // Remaining wage room = 176100 - 150000 = 26100
        // SE SS = 0.124 * min(46175, 26100) = 0.124 * 26100 = 3236.40
        // SE Medicare = 0.029 * 46175 = 1339.08 (rounded)
        assert_eq!(res.payroll_tax.se_social_security, 3236.4);
        // 0.029 * 46175 = 1339.075 → round to 1339.08
        assert_eq!(res.payroll_tax.se_medicare, 1339.08);
    }

    // Test 14: SE tax deduction — half of (SE_SS + SE_Medicare), excludes Additional Medicare
    #[test]
    fn test_se_tax_deduction() {
        let income = IncomeBreakdown {
            self_employment_income: 250000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // SE base = 250000 * 0.9235 = 230875
        // SE SS = 0.124 * min(230875, 176100) = 0.124 * 176100 = 21836.40
        // SE Medicare = 0.029 * 230875 = 6695.375 → 6695.38
        // SE deduction = 0.5 * (21836.40 + 6695.375) = 0.5 * 28531.775 = 14265.8875 → 14265.89
        assert_eq!(res.payroll_tax.self_employment_tax_deduction, 14265.89);
        // Additional Medicare = 0.009 * (250000 - 200000) = 450 — NOT in deduction
        assert_eq!(res.payroll_tax.additional_medicare, 450.0);
    }

    // Test 15: SE tax deduction reduces AGI → affects NIIT
    #[test]
    fn test_se_deduction_reduces_agi() {
        let income = IncomeBreakdown {
            self_employment_income: 100000.0,
            taxable_interest: 20000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // SE base = 92350
        // SE SS = 0.124 * 92350 = 11451.40
        // SE Medicare = 0.029 * 92350 = 2678.15
        // SE deduction = 0.5 * (11451.40 + 2678.15) = 7064.775 → 7064.78
        // Gross = 100000 + 20000 = 120000
        // AGI = 120000 - 7064.775 = 112935.225
        // AGI < 200000, so NIIT = 0
        assert!(res.agi < 200000.0);
        assert_eq!(res.niit, 0.0);
        // Verify SE deduction is in adjustments_total
        assert!(res.adjustments_total > 7000.0);
    }

    // Test 16: Itemized > standard → uses itemized
    #[test]
    fn test_itemized_deduction() {
        let income = IncomeBreakdown {
            wages: 100000.0,
            ..zero_income()
        };
        let mut req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        req.deductions = DeductionConfig {
            method: "itemized".to_string(),
            itemized_amount: Some(25000.0),
            spouse_itemizes: None,
            state_local_income_or_sales_tax: None,
            real_property_tax: None,
            personal_property_tax: None,
            other_itemized_deductions: None,
        };
        let res = run_federal_tax(&req);

        assert_eq!(res.deduction_method, "itemized");
        assert_eq!(res.deduction_used, 25000.0);
        assert_eq!(res.taxable_income, 75000.0);
    }

    // Test 17: MFS spouse_itemizes forces itemized
    #[test]
    fn test_mfs_spouse_itemizes() {
        let income = IncomeBreakdown {
            wages: 80000.0,
            ..zero_income()
        };
        let mut req = make_request(
            "married_filing_separately",
            income,
            single_ordinary_brackets(), // using single brackets for simplicity
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        req.deductions = DeductionConfig {
            method: "standard".to_string(),
            itemized_amount: Some(5000.0),
            spouse_itemizes: Some(true),
            state_local_income_or_sales_tax: None,
            real_property_tax: None,
            personal_property_tax: None,
            other_itemized_deductions: None,
        };
        let res = run_federal_tax(&req);

        // Forced itemized even though standard (15000) > itemized (5000)
        assert_eq!(res.deduction_method, "itemized");
        assert_eq!(res.deduction_used, 5000.0);
    }

    #[test]
    fn test_itemized_deduction_applies_salt_cap() {
        let income = IncomeBreakdown {
            wages: 100000.0,
            ..zero_income()
        };
        let mut req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        req.deductions = DeductionConfig {
            method: "itemized".to_string(),
            itemized_amount: None,
            spouse_itemizes: None,
            state_local_income_or_sales_tax: Some(25000.0),
            real_property_tax: Some(25000.0),
            personal_property_tax: Some(1000.0),
            other_itemized_deductions: Some(8000.0),
        };

        let res = run_federal_tax(&req);

        assert_eq!(res.deduction_method, "itemized");
        assert_eq!(res.deduction_used, 48400.0);
        assert_eq!(res.taxable_income, 51600.0);
    }

    #[test]
    fn test_itemized_deduction_applies_salt_phaseout_floor() {
        let income = IncomeBreakdown {
            wages: 800000.0,
            ..zero_income()
        };
        let mut req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        req.deductions = DeductionConfig {
            method: "itemized".to_string(),
            itemized_amount: None,
            spouse_itemizes: None,
            state_local_income_or_sales_tax: Some(30000.0),
            real_property_tax: Some(20000.0),
            personal_property_tax: None,
            other_itemized_deductions: None,
        };

        let res = run_federal_tax(&req);

        assert_eq!(res.deduction_method, "itemized");
        assert_eq!(res.deduction_used, 10000.0);
    }

    #[test]
    fn test_standard_deduction_ignores_detailed_itemized_inputs() {
        let income = IncomeBreakdown {
            wages: 100000.0,
            ..zero_income()
        };
        let mut req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        req.deductions = DeductionConfig {
            method: "standard".to_string(),
            itemized_amount: None,
            spouse_itemizes: None,
            state_local_income_or_sales_tax: Some(25000.0),
            real_property_tax: Some(25000.0),
            personal_property_tax: Some(1000.0),
            other_itemized_deductions: Some(8000.0),
        };

        let res = run_federal_tax(&req);

        assert_eq!(res.deduction_method, "standard");
        assert_eq!(res.deduction_used, 15000.0);
    }

    // Test 18: Capital loss limit
    #[test]
    fn test_capital_loss_limit() {
        let income = IncomeBreakdown {
            wages: 80000.0,
            short_term_capital_gains: -15000.0,
            long_term_capital_gains: -5000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // Net CG = -15000 + -5000 = -20000, capped at -3000
        // Gross = 80000 + (-3000) = 77000
        assert_eq!(res.gross_income, 77000.0);
    }

    // Test 19: Marginal rate identification
    #[test]
    fn test_marginal_rates() {
        let income = IncomeBreakdown {
            wages: 75000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // Taxable = 60000, which is in the 22% bracket (48475–103350)
        assert_eq!(res.marginal_ordinary_rate, 0.22);
    }

    // Test 20: High income ($1M+)
    #[test]
    fn test_high_income() {
        let income = IncomeBreakdown {
            wages: 800000.0,
            long_term_capital_gains: 300000.0,
            qualified_dividends: 50000.0,
            taxable_interest: 50000.0,
            ..zero_income()
        };
        let req = make_request(
            "single",
            income,
            single_ordinary_brackets(),
            single_cg_brackets(),
            15000.0,
            default_payroll(),
            200000.0,
        );
        let res = run_federal_tax(&req);

        // AGI = 1200000
        assert_eq!(res.agi, 1200000.0);

        // Verify marginal rates are top brackets
        assert_eq!(res.marginal_ordinary_rate, 0.37);
        // CG: stack_top = ordinary_taxable + preferential, in top CG bracket (20%) + NIIT (3.8%)
        assert!((res.marginal_capital_gains_rate - 0.238).abs() < 1e-10);

        // Total tax should be substantial
        assert!(res.total_tax > 300000.0);

        // Effective rate should be reasonable (between 25-40% for this income)
        assert!(res.effective_rate > 0.25);
        assert!(res.effective_rate < 0.40);

        // NIIT should be triggered
        assert!(res.niit > 0.0);
    }
}
