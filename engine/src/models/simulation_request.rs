use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRequest {
    pub mode: Option<String>,
    pub num_simulations: Option<u32>,
    pub seed: Option<u64>,
    pub starting_balance: f64,
    pub time_horizon_months: u32,
    pub return_assumption: ReturnAssumption,
    #[serde(default)]
    pub cash_flows: Vec<CashFlow>,
    #[serde(default)]
    pub include_detail: bool,
    #[serde(default = "default_granularity")]
    pub detail_granularity: String,
}

fn default_granularity() -> String {
    "annual".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnAssumption {
    pub annual_mean: f64,
    pub annual_std_dev: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlow {
    pub amount: f64,
    pub frequency: String,
    pub start_month: Option<u32>,
    pub end_month: Option<u32>,
}
