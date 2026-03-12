use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FederalTaxRequest {
    pub filing_status: String,
    pub income: IncomeBreakdown,
    #[serde(default)]
    pub adjustments: Adjustments,
    pub deductions: DeductionConfig,
    pub tax_parameters: TaxParameters,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct IncomeBreakdown {
    #[serde(default)]
    pub wages: f64,
    #[serde(default)]
    pub self_employment_income: f64,
    #[serde(default)]
    pub taxable_interest: f64,
    #[serde(default)]
    pub tax_exempt_interest: f64,
    #[serde(default)]
    pub ordinary_dividends: f64,
    #[serde(default)]
    pub qualified_dividends: f64,
    #[serde(default)]
    pub short_term_capital_gains: f64,
    #[serde(default)]
    pub long_term_capital_gains: f64,
    #[serde(default)]
    pub taxable_ira_distributions: f64,
    #[serde(default)]
    pub taxable_pensions: f64,
    #[serde(default)]
    pub taxable_social_security: f64,
    #[serde(default)]
    pub other_income: f64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Adjustments {
    #[serde(default)]
    pub hsa_deduction: f64,
    #[serde(default)]
    pub ira_deduction: f64,
    #[serde(default)]
    pub student_loan_interest: f64,
    #[serde(default)]
    pub other_adjustments: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeductionConfig {
    pub method: String,
    pub itemized_amount: Option<f64>,
    pub spouse_itemizes: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaxParameters {
    pub ordinary_brackets: Vec<TaxBracket>,
    pub capital_gains_brackets: Vec<TaxBracket>,
    pub standard_deduction: f64,
    pub capital_loss_limit: f64,
    pub niit: NiitParams,
    pub payroll: PayrollParams,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaxBracket {
    pub min: f64,
    pub max: Option<f64>,
    pub rate: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NiitParams {
    pub rate: f64,
    pub threshold: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PayrollParams {
    pub social_security_rate: f64,
    pub social_security_wage_base: f64,
    pub self_employment_tax_rate: f64,
    pub medicare_rate: f64,
    pub self_employment_medicare_rate: f64,
    pub additional_medicare_rate: f64,
    pub additional_medicare_threshold: f64,
}
