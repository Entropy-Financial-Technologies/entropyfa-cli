use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FederalTaxResponse {
    pub gross_income: f64,
    pub adjustments_total: f64,
    pub agi: f64,
    pub deduction_used: f64,
    pub deduction_method: String,
    pub taxable_income: f64,

    pub ordinary_taxable: f64,
    pub preferential_income: f64,

    pub ordinary_income_tax: f64,
    pub capital_gains_tax: f64,
    pub niit: f64,
    pub total_income_tax: f64,

    pub payroll_tax: PayrollTaxDetail,

    pub total_tax: f64,
    pub effective_rate: f64,
    pub effective_rate_with_payroll: f64,
    pub marginal_ordinary_rate: f64,
    pub marginal_capital_gains_rate: f64,

    pub ordinary_bracket_detail: Vec<BracketDetail>,
    pub capital_gains_bracket_detail: Vec<BracketDetail>,
}

#[derive(Debug, Serialize)]
pub struct PayrollTaxDetail {
    pub employee_social_security: f64,
    pub employee_medicare: f64,
    pub se_social_security: f64,
    pub se_medicare: f64,
    pub additional_medicare: f64,
    pub self_employment_tax_deduction: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BracketDetail {
    pub rate: f64,
    pub income_in_bracket: f64,
    pub tax_in_bracket: f64,
}
