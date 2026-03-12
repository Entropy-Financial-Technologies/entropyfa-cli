use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EstateTaxResponse {
    pub gross_estate: f64,
    pub total_deductions: f64,
    pub taxable_estate: f64,
    pub adjusted_taxable_gifts: f64,
    pub tax_base: f64,
    pub tentative_tax: f64,
    pub gift_tax_credit: f64,
    pub applicable_credit_amount: f64,
    pub dsue_credit: f64,
    pub net_estate_tax: f64,
    pub effective_rate: f64,
    pub marginal_rate: f64,
    pub exemption_amount: f64,
    pub bracket_detail: Vec<EstateBracketDetail>,
    pub deductions_detail: EstateDeductionsDetail,
}

#[derive(Debug, Serialize)]
pub struct EstateBracketDetail {
    pub rate: f64,
    pub amount_in_bracket: f64,
    pub tax_in_bracket: f64,
}

#[derive(Debug, Serialize)]
pub struct EstateDeductionsDetail {
    pub marital: f64,
    pub charitable: f64,
    pub debts_and_expenses: f64,
    pub state_death_tax: f64,
    pub other: f64,
    pub total: f64,
}
