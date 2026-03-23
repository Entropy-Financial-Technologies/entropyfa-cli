//! Golden / snapshot tests for key engine computations.
//!
//! Each test constructs a request using the embedded reference data and asserts
//! that the computed results match hand-verified known-correct values.  If any
//! assertion fails it means the computation or the embedded data has changed in
//! a way that breaks previously validated answers.

// ---------------------------------------------------------------------------
// 1. Federal tax: $200K single wages, standard deduction, 2026
// ---------------------------------------------------------------------------

#[test]
fn federal_tax_200k_single_wages_2026() {
    use entropyfa_engine::compute::tax::federal::run_federal_tax;
    use entropyfa_engine::data::tax::federal as tax_data;
    use entropyfa_engine::data::types::FilingStatus;
    use entropyfa_engine::models::tax_request::*;

    let status = FilingStatus::Single;

    let req = FederalTaxRequest {
        filing_status: "single".to_string(),
        tax_year: 2026,
        income: IncomeBreakdown {
            wages: 200_000.0,
            ..IncomeBreakdown::default()
        },
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
            ordinary_brackets: tax_data::brackets(status),
            capital_gains_brackets: tax_data::capital_gains_brackets(status),
            standard_deduction: tax_data::standard_deductions(status),
            capital_loss_limit: tax_data::capital_loss_limit(status),
            niit: tax_data::niit(status),
            payroll: tax_data::payroll(status),
            salt: Some(tax_data::salt_deduction_parameters(status)),
        },
    };

    let res = run_federal_tax(&req);

    // Gross income = wages only
    assert_eq!(res.gross_income, 200_000.0);

    // No adjustments for a pure W-2 earner
    assert_eq!(res.adjustments_total, 0.0);

    // AGI = gross - adjustments
    assert_eq!(res.agi, 200_000.0);

    // Standard deduction for single 2026 = $16,100
    assert_eq!(res.deduction_used, 16_100.0);
    assert_eq!(res.deduction_method, "standard");

    // Taxable income = AGI - standard deduction = 183,900
    assert_eq!(res.taxable_income, 183_900.0);

    // No preferential income (no LTCG, no qualified dividends)
    assert_eq!(res.preferential_income, 0.0);
    assert_eq!(res.ordinary_taxable, 183_900.0);

    // Ordinary income tax (2026 single brackets):
    //   10% on 12,400    =  1,240.00
    //   12% on 38,000    =  4,560.00
    //   22% on 55,300    = 12,166.00
    //   24% on 78,200    = 18,768.00
    //   Total            = 36,734.00
    assert_eq!(res.ordinary_income_tax, 36_734.0);
    assert_eq!(res.capital_gains_tax, 0.0);
    assert_eq!(res.niit, 0.0);
    assert_eq!(res.total_income_tax, 36_734.0);

    // Effective rate = 36,734 / 200,000 = 0.18367 -> round4 = 0.1837
    assert_eq!(res.effective_rate, 0.1837);

    // Marginal ordinary rate: 183,900 falls in the 24% bracket (105,700 - 201,775)
    assert_eq!(res.marginal_ordinary_rate, 0.24);

    // Payroll taxes
    // SS = 6.2% * min(200,000, 184,500) = 6.2% * 184,500 = 11,439.00
    assert_eq!(res.payroll_tax.employee_social_security, 11_439.0);
    // Medicare = 1.45% * 200,000 = 2,900.00
    assert_eq!(res.payroll_tax.employee_medicare, 2_900.0);
    // No additional Medicare (wages = 200,000 = threshold, excess = 0)
    assert_eq!(res.payroll_tax.additional_medicare, 0.0);
}

// ---------------------------------------------------------------------------
// 2. Federal tax: $500K MFJ with wages + LTCG + qualified divs
// ---------------------------------------------------------------------------

#[test]
fn federal_tax_500k_mfj_mixed_income_2026() {
    use entropyfa_engine::compute::tax::federal::run_federal_tax;
    use entropyfa_engine::data::tax::federal as tax_data;
    use entropyfa_engine::data::types::FilingStatus;
    use entropyfa_engine::models::tax_request::*;

    let status = FilingStatus::MarriedFilingJointly;

    let req = FederalTaxRequest {
        filing_status: "married_filing_jointly".to_string(),
        tax_year: 2026,
        income: IncomeBreakdown {
            wages: 300_000.0,
            long_term_capital_gains: 150_000.0,
            qualified_dividends: 50_000.0,
            ..IncomeBreakdown::default()
        },
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
            ordinary_brackets: tax_data::brackets(status),
            capital_gains_brackets: tax_data::capital_gains_brackets(status),
            standard_deduction: tax_data::standard_deductions(status),
            capital_loss_limit: tax_data::capital_loss_limit(status),
            niit: tax_data::niit(status),
            payroll: tax_data::payroll(status),
            salt: Some(tax_data::salt_deduction_parameters(status)),
        },
    };

    let res = run_federal_tax(&req);

    // Gross income = 300,000 + 150,000 + 50,000 = 500,000
    assert_eq!(res.gross_income, 500_000.0);
    assert_eq!(res.agi, 500_000.0);

    // Standard deduction MFJ 2026 = 32,200
    assert_eq!(res.deduction_used, 32_200.0);

    // Taxable income = 500,000 - 32,200 = 467,800
    assert_eq!(res.taxable_income, 467_800.0);

    // Preferential income = QD(50,000) + LTCG(150,000) = 200,000
    assert_eq!(res.preferential_income, 200_000.0);

    // Ordinary taxable = 467,800 - 200,000 = 267,800
    assert_eq!(res.ordinary_taxable, 267_800.0);

    // Total income tax must be positive and include CG component
    assert!(res.total_income_tax > 0.0);
    assert!(res.capital_gains_tax > 0.0);

    // Ordinary income tax on 267,800 (MFJ 2026 brackets):
    //   10% on 24,800     =  2,480.00
    //   12% on 76,000     =  9,120.00
    //   22% on 110,600    = 24,332.00
    //   24% on 56,400     = 13,536.00
    //   Total             = 49,468.00
    assert_eq!(res.ordinary_income_tax, 49_468.0);

    // Capital gains tax (stacking from 267,800 to 467,800):
    //   0% bracket 0-98,900: entirely below stack start (267,800), contributes 0
    //   15% bracket 98,900-613,700: overlap = 467,800 - 267,800 = 200,000 at 15% = 30,000
    assert_eq!(res.capital_gains_tax, 30_000.0);

    // NIIT: AGI(500,000) > MFJ threshold(250,000)
    //   NII = QD(50,000) + LTCG(150,000) = 200,000
    //   excess AGI = 500,000 - 250,000 = 250,000
    //   NIIT = 3.8% * min(200,000, 250,000) = 3.8% * 200,000 = 7,600
    assert_eq!(res.niit, 7_600.0);

    // Total income tax = 49,468 + 30,000 + 7,600 = 87,068
    assert_eq!(res.total_income_tax, 87_068.0);

    // Marginal ordinary rate: 267,800 in 24% bracket (211,400-403,550)
    assert_eq!(res.marginal_ordinary_rate, 0.24);
}

// ---------------------------------------------------------------------------
// 3. Estate tax: $20M estate, 2026
// ---------------------------------------------------------------------------

#[test]
fn estate_tax_20m_estate_2026() {
    use entropyfa_engine::compute::tax::estate::run_estate_tax;
    use entropyfa_engine::data::tax::estate as estate_data;
    use entropyfa_engine::models::estate_tax_request::*;

    let req = EstateTaxRequest {
        gross_estate: 20_000_000.0,
        deductions: EstateDeductions {
            marital: 0.0,
            charitable: 0.0,
            debts_and_expenses: 100_000.0,
            state_death_tax: 0.0,
            other: 0.0,
        },
        adjusted_taxable_gifts: 0.0,
        gift_tax_paid: 0.0,
        deceased_spouse_unused_exclusion: 0.0,
        estate_tax_parameters: EstateTaxParameters {
            exemption_amount: estate_data::exemption(),
            applicable_credit_amount: estate_data::applicable_credit(),
            brackets: estate_data::brackets(),
        },
    };

    let res = run_estate_tax(&req);

    // Taxable estate = 20,000,000 - 100,000 = 19,900,000
    assert_eq!(res.taxable_estate, 19_900_000.0);

    // Tax base = taxable estate + adjusted taxable gifts = 19,900,000
    assert_eq!(res.tax_base, 19_900_000.0);

    // Tentative tax:
    //   First $1M via graduated brackets = 345,800
    //   40% on remaining $18,900,000     = 7,560,000
    //   Total tentative                  = 7,905,800
    assert_eq!(res.tentative_tax, 7_905_800.0);

    // Applicable credit = 5,945,800 (2026 embedded data)
    assert_eq!(res.applicable_credit_amount, 5_945_800.0);

    // Net estate tax = 7,905,800 - 5,945,800 = 1,960,000
    assert_eq!(res.net_estate_tax, 1_960_000.0);

    // Estate tax owed must be positive
    assert!(res.net_estate_tax > 0.0);

    // Exemption is $15M for 2026
    assert_eq!(res.exemption_amount, 15_000_000.0);

    // Marginal rate at this estate size = 40%
    assert_eq!(res.marginal_rate, 0.40);

    // Effective rate = 1,960,000 / 20,000,000 = 0.098
    assert_eq!(res.effective_rate, 0.098);
}

// ---------------------------------------------------------------------------
// 4. RMD: age 75, $1M balance, uniform lifetime table
// ---------------------------------------------------------------------------

#[test]
fn rmd_age_75_uniform_table_2026() {
    use entropyfa_engine::compute::retirement::rmd::run_retirement_rmd;
    use entropyfa_engine::data::retirement::rmd_rules;
    use entropyfa_engine::models::retirement_rmd::*;

    let rules = rmd_rules::distribution_rules();

    let req = RetirementRmdRequest {
        calculation_year: 2026,
        prior_year_end_balance: 1_000_000.0,
        account_type: "traditional_ira".to_string(),
        owner_birth_date: Some("1951-06-15".to_string()),
        owner_is_alive: Some(true),
        owner_death_year: None,
        owner_died_before_required_beginning_date: None,
        beneficiary_birth_date: None,
        beneficiary_class: None,
        beneficiary_election: None,
        beneficiary_majority_year: None,
        spouse_birth_date: None,
        spouse_is_sole_beneficiary: None,
        is_still_working: None,
        is_five_percent_owner: None,
        pre_1987_403b_balance: None,
        rmd_parameters: rules,
    };

    let res = run_retirement_rmd(&req).expect("RMD computation should succeed");

    // Age in year: 2026 - 1951 = 75
    // Birth year 1951 => start_age = 73 (rule: 1951-1959 => 73)
    // RMD start year = 1951 + 73 = 2024
    // 2026 >= 2024, so RMD is required
    assert!(res.rmd_required);
    assert_eq!(res.scenario_class, "owner_lifetime");

    // Uniform lifetime table for age 75: distribution_period = 24.6
    assert_eq!(res.distribution_period, Some(24.6));
    assert_eq!(res.table_used.as_deref(), Some("uniform_lifetime_table"));

    // RMD = 1,000,000 / 24.6 = 40,650.406504... -> round2 = 40,650.41
    assert_eq!(res.rmd_amount, 40_650.41);

    // Applicable balance equals the full prior year end balance
    assert_eq!(res.applicable_balance, 1_000_000.0);

    let json = serde_json::to_value(&res).expect("response should serialize");
    let obj = json.as_object().expect("RMD response should be an object");
    assert_eq!(obj.get("references_used"), Some(&serde_json::json!([])));
    assert_eq!(obj.get("assumptions_used"), Some(&serde_json::json!({})));
    assert_eq!(obj.get("overrides_used"), Some(&serde_json::json!({})));

    // RMD should be in a reasonable range
    assert!(res.rmd_amount > 40_000.0);
    assert!(res.rmd_amount < 42_000.0);
}
