use serde::Serialize;
use std::collections::BTreeMap;

use super::simulation_request::SimulationRequest;

#[derive(Debug, Serialize)]
pub struct SimulationResponse {
    pub request_echo: SimulationRequest,
    pub computation_time_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monte_carlo: Option<MonteCarloResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<LinearResult>,
}

#[derive(Debug, Serialize)]
pub struct MonteCarloResult {
    pub num_simulations: u32,
    pub percentiles: Percentiles,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub success_rate: f64,
    pub paths_depleted_by_month: BTreeMap<String, u32>,
    pub survival_by_year: Vec<f64>,
    pub balance_histogram: BalanceHistogram,
    pub time_series: TimeSeries,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_detail: Option<Vec<MonteCarloDetailRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_paths: Option<Vec<SamplePath>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_percentile_series: Option<BTreeMap<String, Vec<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_terminal_percentiles: Option<BTreeMap<String, BucketTerminalPercentiles>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_depletion_counts: Option<BTreeMap<String, u32>>,
}

#[derive(Debug, Serialize)]
pub struct BalanceHistogram {
    pub centers: Vec<f64>,
    pub percentages: Vec<f64>,
    pub depleted_pct: f64,
}

#[derive(Debug, Serialize)]
pub struct Percentiles {
    pub p5: f64,
    pub p10: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BucketTerminalPercentiles {
    pub p10: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
}

#[derive(Debug, Serialize)]
pub struct TimeSeries {
    pub months: Vec<u32>,
    pub p10: Vec<f64>,
    pub p25: Vec<f64>,
    pub p50: Vec<f64>,
    pub p75: Vec<f64>,
    pub p90: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct LinearResult {
    pub final_balance: f64,
    pub time_series: LinearTimeSeries,
    pub total_contributions: f64,
    pub total_withdrawals: f64,
    pub total_return_earned: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_balances_by_bucket: Option<BTreeMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_detail: Option<Vec<PeriodDetail>>,
}

#[derive(Debug, Serialize)]
pub struct LinearTimeSeries {
    pub months: Vec<u32>,
    pub balance: Vec<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PeriodDetail {
    pub period: u32,
    pub year: f64,
    pub beginning_balance: f64,
    pub contributions: f64,
    pub withdrawals: f64,
    pub investment_return: f64,
    pub ending_balance: f64,
    pub cumulative_contributions: f64,
    pub cumulative_withdrawals: f64,
    pub cumulative_return: f64,
    pub annual_tax_paid: f64,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub bucket_withdrawals: BTreeMap<String, f64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub ending_balances_by_bucket: BTreeMap<String, f64>,
}

#[derive(Debug, Serialize)]
pub struct SamplePath {
    pub index: usize,
    pub months: Vec<u32>,
    pub balances: Vec<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MonteCarloDetailRow {
    pub period: u32,
    pub year: f64,
    pub balance_p10: f64,
    pub balance_p25: f64,
    pub balance_p50: f64,
    pub balance_p75: f64,
    pub balance_p90: f64,
    pub net_cash_flow: f64,
    pub cumulative_cash_flow: f64,
    pub annual_tax_paid: f64,
    pub survival_rate: f64,
}
