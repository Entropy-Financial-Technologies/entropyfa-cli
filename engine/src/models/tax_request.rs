use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct FederalTaxRequest {
    pub filing_status: String,
    #[serde(default = "default_tax_year")]
    pub tax_year: u32,
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
    pub state_local_income_or_sales_tax: Option<f64>,
    pub real_property_tax: Option<f64>,
    pub personal_property_tax: Option<f64>,
    pub other_itemized_deductions: Option<f64>,
}

impl DeductionConfig {
    pub fn has_detailed_itemized_inputs(&self) -> bool {
        self.state_local_income_or_sales_tax.is_some()
            || self.real_property_tax.is_some()
            || self.personal_property_tax.is_some()
            || self.other_itemized_deductions.is_some()
    }

    pub fn raw_salt_amount(&self) -> f64 {
        self.state_local_income_or_sales_tax.unwrap_or(0.0)
            + self.real_property_tax.unwrap_or(0.0)
            + self.personal_property_tax.unwrap_or(0.0)
    }

    pub fn other_itemized_amount(&self) -> f64 {
        self.other_itemized_deductions.unwrap_or(0.0)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaxParameters {
    pub ordinary_brackets: Vec<TaxBracket>,
    pub capital_gains_brackets: Vec<TaxBracket>,
    pub standard_deduction: f64,
    pub capital_loss_limit: f64,
    pub niit: NiitParams,
    pub payroll: PayrollParams,
    #[serde(default)]
    pub salt: Option<SaltDeductionParams>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SaltDeductionParams {
    pub cap_amount: f64,
    pub phaseout_threshold: f64,
    pub phaseout_rate: f64,
    pub floor_amount: f64,
}

fn default_tax_year() -> u32 {
    2026
}
