use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PensionComparisonResponse {
    pub selected_option_id_used: String,
    pub options_analyzed: Vec<OptionAnalysis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lump_sum_monte_carlo: Option<LumpSumMonteCarloResult>,
    pub comparison_summary: Vec<ComparisonRow>,
    pub cumulative_income_by_year: Vec<CumulativeIncomeYear>,
    pub tax_impact: TaxImpactSummary,
    pub after_tax_projection: AfterTaxProjection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub survivor_analysis: Option<SurvivorAnalysis>,
}

#[derive(Debug, Serialize)]
pub struct OptionAnalysis {
    pub id: String,
    pub option_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub monthly_payment: f64,
    pub present_value: f64,
    pub present_value_after_tax: f64,
    pub break_even_age: Option<u32>,
    pub lifetime_income_nominal: f64,
    pub lifetime_income_real: f64,
    pub annual_tax_estimate: f64,
    pub effective_tax_rate: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub survivor_pv: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct LumpSumMonteCarloResult {
    pub starting_balance: f64,
    pub monthly_withdrawal: f64,
    pub num_simulations: u32,
    pub success_rate: f64,
    pub depletion_age_percentiles: Percentiles,
    pub residual_estate_percentiles: Percentiles,
    pub probability_annuity_wins: f64,
    pub time_series: Vec<TimeSeriesPoint>,
}

#[derive(Debug, Serialize)]
pub struct Percentiles {
    pub p10: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
}

#[derive(Debug, Serialize)]
pub struct TimeSeriesPoint {
    pub age: u32,
    pub annuity_cumulative: f64,
    pub lump_sum_balance_p10: f64,
    pub lump_sum_balance_p50: f64,
    pub lump_sum_balance_p90: f64,
}

#[derive(Debug, Serialize)]
pub struct ComparisonRow {
    pub id: String,
    pub option_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub present_value: f64,
    pub break_even_age: Option<u32>,
    pub lifetime_income_nominal: f64,
    pub annual_tax_estimate: f64,
}

#[derive(Debug, Serialize)]
pub struct CumulativeIncomeYear {
    pub age: u32,
    pub year_index: u32,
    pub annuity_cumulative_nominal: f64,
    pub annuity_cumulative_real: f64,
    pub lump_sum_withdrawal_cumulative: f64,
}

#[derive(Debug, Serialize)]
pub struct TaxImpactSummary {
    pub annuity_scenario: TaxScenario,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lump_sum_ira_rollover: Option<TaxScenario>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lump_sum_cash_out: Option<TaxScenario>,
}

#[derive(Debug, Serialize)]
pub struct TaxScenario {
    pub first_year_total_tax: f64,
    pub first_year_effective_rate: f64,
    pub first_year_marginal_rate: f64,
    pub annual_tax_on_pension_income: f64,
    pub upfront_tax_cost: f64,
    pub ss_taxable_amount: f64,
}

#[derive(Debug, Serialize)]
pub struct SurvivorAnalysis {
    pub spouse_age: u32,
    pub retiree_expected_death_age: u32,
    pub spouse_expected_death_age: u32,
    pub options: Vec<SurvivorOption>,
}

#[derive(Debug, Serialize)]
pub struct SurvivorOption {
    pub id: String,
    pub option_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub survivor_monthly_income: f64,
    pub survivor_income_pv: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lump_sum_residual_p50: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct AfterTaxProjection {
    pub selected_option_id: String,
    pub horizon_age: u32,
    pub annuity: AfterTaxScenarioProjection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lump_sum_ira_rollover: Option<AfterTaxScenarioProjection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lump_sum_cash_out: Option<AfterTaxScenarioProjection>,
}

#[derive(Debug, Serialize)]
pub struct AfterTaxScenarioProjection {
    pub cumulative_after_tax_income_nominal: f64,
    pub cumulative_after_tax_income_real: f64,
    pub cumulative_tax: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_rmd_age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_rmd_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_investable_amount: Option<f64>,
}
