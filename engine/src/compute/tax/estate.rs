use crate::models::estate_tax_request::EstateTaxRequest;
use crate::models::estate_tax_response::{
    EstateBracketDetail, EstateDeductionsDetail, EstateTaxResponse,
};
use crate::models::tax_request::TaxBracket;

pub fn run_estate_tax(req: &EstateTaxRequest) -> EstateTaxResponse {
    let params = &req.estate_tax_parameters;
    let ded = &req.deductions;

    // Step 1: Total deductions
    let total_deductions =
        ded.marital + ded.charitable + ded.debts_and_expenses + ded.state_death_tax + ded.other;

    // Step 2: Taxable estate
    let taxable_estate = (req.gross_estate - total_deductions).max(0.0);

    // Step 3: Tax base = taxable estate + adjusted taxable gifts
    let tax_base = taxable_estate + req.adjusted_taxable_gifts;

    // Step 4: Tentative tax on the full tax base
    let (tentative_tax, bracket_detail) = apply_estate_brackets(&params.brackets, tax_base);

    // Step 5: Subtract gift tax paid (credit for gift taxes on adjusted taxable gifts)
    let after_gift_tax = (tentative_tax - req.gift_tax_paid).max(0.0);

    // Step 6: Subtract applicable credit amount (unified credit)
    let after_unified_credit = (after_gift_tax - params.applicable_credit_amount).max(0.0);

    // Step 7: DSUE portability credit
    let dsue_credit = if req.deceased_spouse_unused_exclusion > 0.0 {
        // Credit = tax on (exemption + DSUE) - tax on (exemption alone)
        let (tax_with_dsue, _) = apply_estate_brackets(
            &params.brackets,
            params.exemption_amount + req.deceased_spouse_unused_exclusion,
        );
        let (tax_without_dsue, _) =
            apply_estate_brackets(&params.brackets, params.exemption_amount);
        (tax_with_dsue - tax_without_dsue).max(0.0)
    } else {
        0.0
    };

    // Step 8: Net estate tax
    let net_estate_tax = (after_unified_credit - dsue_credit).max(0.0);

    // Step 9: Rates
    let effective_rate = if req.gross_estate > 0.0 {
        net_estate_tax / req.gross_estate
    } else {
        0.0
    };
    let marginal_rate = find_marginal_rate(&params.brackets, tax_base);

    let deductions_detail = EstateDeductionsDetail {
        marital: round2(ded.marital),
        charitable: round2(ded.charitable),
        debts_and_expenses: round2(ded.debts_and_expenses),
        state_death_tax: round2(ded.state_death_tax),
        other: round2(ded.other),
        total: round2(total_deductions),
    };

    EstateTaxResponse {
        gross_estate: round2(req.gross_estate),
        total_deductions: round2(total_deductions),
        taxable_estate: round2(taxable_estate),
        adjusted_taxable_gifts: round2(req.adjusted_taxable_gifts),
        tax_base: round2(tax_base),
        tentative_tax: round2(tentative_tax),
        gift_tax_credit: round2(req.gift_tax_paid),
        applicable_credit_amount: round2(params.applicable_credit_amount),
        dsue_credit: round2(dsue_credit),
        net_estate_tax: round2(net_estate_tax),
        effective_rate: round4(effective_rate),
        marginal_rate,
        exemption_amount: round2(params.exemption_amount),
        bracket_detail: bracket_detail
            .into_iter()
            .filter(|b| b.amount_in_bracket > 0.0)
            .collect(),
        deductions_detail,
    }
}

fn apply_estate_brackets(brackets: &[TaxBracket], amount: f64) -> (f64, Vec<EstateBracketDetail>) {
    let mut total_tax = 0.0;
    let mut details = Vec::new();

    for bracket in brackets {
        let bracket_max = bracket.max.unwrap_or(f64::MAX);
        let taxable_in_bracket = (amount.min(bracket_max) - bracket.min).max(0.0);
        let tax = taxable_in_bracket * bracket.rate;
        total_tax += tax;
        details.push(EstateBracketDetail {
            rate: bracket.rate,
            amount_in_bracket: round2(taxable_in_bracket),
            tax_in_bracket: round2(tax),
        });
    }

    (total_tax, details)
}

fn find_marginal_rate(brackets: &[TaxBracket], amount: f64) -> f64 {
    if amount <= 0.0 {
        return brackets.first().map(|b| b.rate).unwrap_or(0.0);
    }
    for bracket in brackets.iter().rev() {
        if amount > bracket.min {
            return bracket.rate;
        }
    }
    brackets.first().map(|b| b.rate).unwrap_or(0.0)
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
    use crate::models::estate_tax_request::*;

    fn estate_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                min: 0.0,
                max: Some(10000.0),
                rate: 0.18,
            },
            TaxBracket {
                min: 10000.0,
                max: Some(20000.0),
                rate: 0.20,
            },
            TaxBracket {
                min: 20000.0,
                max: Some(40000.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 40000.0,
                max: Some(60000.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 60000.0,
                max: Some(80000.0),
                rate: 0.26,
            },
            TaxBracket {
                min: 80000.0,
                max: Some(100000.0),
                rate: 0.28,
            },
            TaxBracket {
                min: 100000.0,
                max: Some(150000.0),
                rate: 0.30,
            },
            TaxBracket {
                min: 150000.0,
                max: Some(250000.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 250000.0,
                max: Some(500000.0),
                rate: 0.34,
            },
            TaxBracket {
                min: 500000.0,
                max: Some(750000.0),
                rate: 0.37,
            },
            TaxBracket {
                min: 750000.0,
                max: Some(1000000.0),
                rate: 0.39,
            },
            TaxBracket {
                min: 1000000.0,
                max: None,
                rate: 0.40,
            },
        ]
    }

    // 2025 exemption and credit (credit = tentative tax on exemption amount)
    fn params_2025() -> EstateTaxParameters {
        EstateTaxParameters {
            exemption_amount: 13990000.0,
            applicable_credit_amount: 5541800.0,
            brackets: estate_brackets(),
        }
    }

    // 2026 sunset: exemption roughly halved
    fn params_2026() -> EstateTaxParameters {
        EstateTaxParameters {
            exemption_amount: 7000000.0,
            applicable_credit_amount: 2745800.0,
            brackets: estate_brackets(),
        }
    }

    fn make_request(
        gross_estate: f64,
        deductions: EstateDeductions,
        params: EstateTaxParameters,
    ) -> EstateTaxRequest {
        EstateTaxRequest {
            gross_estate,
            deductions,
            adjusted_taxable_gifts: 0.0,
            gift_tax_paid: 0.0,
            deceased_spouse_unused_exclusion: 0.0,
            estate_tax_parameters: params,
        }
    }

    // Test 1: Zero estate
    #[test]
    fn test_zero_estate() {
        let req = make_request(0.0, EstateDeductions::default(), params_2025());
        let res = run_estate_tax(&req);

        assert_eq!(res.gross_estate, 0.0);
        assert_eq!(res.taxable_estate, 0.0);
        assert_eq!(res.tax_base, 0.0);
        assert_eq!(res.net_estate_tax, 0.0);
        assert_eq!(res.effective_rate, 0.0);
    }

    // Test 2: Estate below exemption — no tax
    #[test]
    fn test_below_exemption() {
        let req = make_request(5_000_000.0, EstateDeductions::default(), params_2025());
        let res = run_estate_tax(&req);

        assert_eq!(res.gross_estate, 5_000_000.0);
        assert_eq!(res.taxable_estate, 5_000_000.0);
        assert_eq!(res.net_estate_tax, 0.0);
        // Tentative tax exists but is wiped out by the unified credit
        assert!(res.tentative_tax > 0.0);
    }

    // Test 3: Estate at exemption boundary — no tax
    #[test]
    fn test_at_exemption() {
        let req = make_request(13_990_000.0, EstateDeductions::default(), params_2025());
        let res = run_estate_tax(&req);

        assert_eq!(res.net_estate_tax, 0.0);
    }

    // Test 4: Estate above exemption — tax owed
    #[test]
    fn test_above_exemption() {
        // $15M estate, $13.99M exemption → $1.01M over exemption
        // All in the 40% bracket (above $1M), so tax ≈ $1.01M * 0.40 = $404,000
        // But the tentative_tax - credit calculation handles this differently:
        // tentative_tax on $15M, minus credit of $5,389,800
        let req = make_request(15_000_000.0, EstateDeductions::default(), params_2025());
        let res = run_estate_tax(&req);

        assert!(res.net_estate_tax > 0.0);
        // The tax should be 40% on the amount above the exemption
        // $15M - $13.99M = $1.01M excess, all at 40% = $404,000
        assert_eq!(res.net_estate_tax, 404000.0);
        assert_eq!(res.marginal_rate, 0.40);
    }

    // Test 5: Marital deduction — surviving spouse gets unlimited deduction
    #[test]
    fn test_marital_deduction() {
        let deductions = EstateDeductions {
            marital: 20_000_000.0,
            ..EstateDeductions::default()
        };
        let req = make_request(20_000_000.0, deductions, params_2025());
        let res = run_estate_tax(&req);

        assert_eq!(res.total_deductions, 20_000_000.0);
        assert_eq!(res.taxable_estate, 0.0);
        assert_eq!(res.net_estate_tax, 0.0);
    }

    // Test 6: DSUE portability
    #[test]
    fn test_dsue_portability() {
        // Surviving spouse has own $13.99M exemption + deceased spouse's unused $13.99M
        // Estate of $25M should be below combined exemptions
        let mut req = make_request(25_000_000.0, EstateDeductions::default(), params_2025());
        req.deceased_spouse_unused_exclusion = 13_990_000.0;
        let res = run_estate_tax(&req);

        // DSUE credit = tax_on(13.99M + 13.99M) - tax_on(13.99M)
        assert!(res.dsue_credit > 0.0);
        // With DSUE of full $13.99M, estate of $25M should owe no tax
        // Tentative on $25M = tax on first $1M via brackets + 40% on remaining $24M
        // Credit = $5,389,800 (exempts $13.99M)
        // DSUE credit = exempts another $13.99M
        // $25M < $27.98M combined exemptions, so no tax
        assert_eq!(res.net_estate_tax, 0.0);
    }

    // Test 7: Large estate ($50M)
    #[test]
    fn test_large_estate() {
        let req = make_request(50_000_000.0, EstateDeductions::default(), params_2025());
        let res = run_estate_tax(&req);

        assert!(res.net_estate_tax > 0.0);
        assert_eq!(res.marginal_rate, 0.40);
        // Effective rate should be positive but less than 40%
        assert!(res.effective_rate > 0.0);
        assert!(res.effective_rate < 0.40);
        // Tax = 40% of ($50M - $13.99M) = $14,404,000
        assert_eq!(res.net_estate_tax, 14_404_000.0);
    }

    // Test 8: All deductions combined
    #[test]
    fn test_all_deductions() {
        let deductions = EstateDeductions {
            marital: 5_000_000.0,
            charitable: 2_000_000.0,
            debts_and_expenses: 500_000.0,
            state_death_tax: 300_000.0,
            other: 200_000.0,
        };
        let req = make_request(20_000_000.0, deductions, params_2025());
        let res = run_estate_tax(&req);

        assert_eq!(res.total_deductions, 8_000_000.0);
        assert_eq!(res.taxable_estate, 12_000_000.0);
        // Below exemption of $13.99M, so no tax
        assert_eq!(res.net_estate_tax, 0.0);

        // Verify deductions detail
        assert_eq!(res.deductions_detail.marital, 5_000_000.0);
        assert_eq!(res.deductions_detail.charitable, 2_000_000.0);
        assert_eq!(res.deductions_detail.debts_and_expenses, 500_000.0);
        assert_eq!(res.deductions_detail.state_death_tax, 300_000.0);
        assert_eq!(res.deductions_detail.other, 200_000.0);
        assert_eq!(res.deductions_detail.total, 8_000_000.0);
    }

    // Test 9: Adjusted taxable gifts + gift tax credit
    #[test]
    fn test_adjusted_taxable_gifts() {
        // Estate of $10M + $5M in prior taxable gifts, $1.5M gift tax paid
        let mut req = make_request(10_000_000.0, EstateDeductions::default(), params_2025());
        req.adjusted_taxable_gifts = 5_000_000.0;
        req.gift_tax_paid = 1_500_000.0;
        let res = run_estate_tax(&req);

        assert_eq!(res.taxable_estate, 10_000_000.0);
        assert_eq!(res.tax_base, 15_000_000.0);
        assert_eq!(res.gift_tax_credit, 1_500_000.0);

        // Tentative tax on $15M minus $1.5M gift tax minus $5,389,800 credit
        // tentative_tax on $15M = brackets up to $1M + 40% on remaining $14M
        // = 345800 + 0.40 * 14_000_000 = 345800 + 5_600_000 = 5_945_800
        // after_gift_tax = 5_945_800 - 1_500_000 = 4_445_800 (but wait, max(0,...))
        // after_unified = max(0, 4_445_800 - 5_389_800) = 0
        assert_eq!(res.net_estate_tax, 0.0);
    }

    // Test 10: Post-sunset 2026 — lower exemption means more tax
    #[test]
    fn test_post_sunset_2026() {
        // Same $15M estate under 2026 sunset parameters
        let req = make_request(15_000_000.0, EstateDeductions::default(), params_2026());
        let res = run_estate_tax(&req);

        // With $7M exemption, there's much more tax
        // Tax = 40% of ($15M - $7M) = $3,200,000
        assert_eq!(res.net_estate_tax, 3_200_000.0);
        assert!(res.effective_rate > 0.20);
    }
}
