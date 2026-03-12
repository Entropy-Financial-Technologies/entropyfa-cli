use serde::Deserialize;

use crate::models::tax_request::TaxBracket;

#[derive(Debug, Deserialize)]
pub struct EstateTaxRequest {
    pub gross_estate: f64,
    #[serde(default)]
    pub deductions: EstateDeductions,
    #[serde(default)]
    pub adjusted_taxable_gifts: f64,
    #[serde(default)]
    pub gift_tax_paid: f64,
    #[serde(default)]
    pub deceased_spouse_unused_exclusion: f64,
    pub estate_tax_parameters: EstateTaxParameters,
}

#[derive(Debug, Deserialize, Default)]
pub struct EstateDeductions {
    #[serde(default)]
    pub marital: f64,
    #[serde(default)]
    pub charitable: f64,
    #[serde(default)]
    pub debts_and_expenses: f64,
    #[serde(default)]
    pub state_death_tax: f64,
    #[serde(default)]
    pub other: f64,
}

#[derive(Debug, Deserialize)]
pub struct EstateTaxParameters {
    pub exemption_amount: f64,
    pub applicable_credit_amount: f64,
    pub brackets: Vec<TaxBracket>,
}
