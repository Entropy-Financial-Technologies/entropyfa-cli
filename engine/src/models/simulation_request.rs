use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRequest {
    pub mode: Option<String>,
    pub num_simulations: Option<u32>,
    pub seed: Option<u64>,
    pub starting_balance: Option<f64>,
    #[serde(default)]
    pub buckets: Vec<SimulationBucket>,
    pub time_horizon_months: u32,
    pub return_assumption: Option<ReturnAssumption>,
    #[serde(default)]
    pub cash_flows: Vec<CashFlow>,
    pub filing_status: Option<String>,
    pub household: Option<HouseholdConfig>,
    pub spending_policy: Option<SpendingPolicy>,
    pub tax_policy: Option<TaxPolicy>,
    pub rmd_policy: Option<RmdPolicy>,
    #[serde(default)]
    pub include_detail: bool,
    #[serde(default = "default_granularity")]
    pub detail_granularity: String,
    #[serde(default)]
    pub sample_paths: Option<usize>,
    #[serde(default)]
    pub path_indices: Option<Vec<usize>>,
    #[serde(default)]
    pub custom_percentiles: Option<Vec<u32>>,
}

fn default_granularity() -> String {
    "annual".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationBucket {
    pub id: String,
    pub bucket_type: String,
    pub starting_balance: f64,
    pub return_assumption: ReturnAssumption,
    pub realized_gain_ratio: Option<f64>,
    pub withdrawal_priority: Option<u32>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseholdConfig {
    pub birth_years: Option<Vec<u32>>,
    pub retirement_month: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingPolicy {
    pub withdrawal_order: Vec<String>,
    pub rebalance_tax_withholding_from: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxPolicy {
    pub mode: String,
    pub modeled_tax_inflation_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RmdPolicy {
    pub enabled: bool,
    pub distribution_month: Option<u32>,
}

impl SimulationRequest {
    pub fn legacy_starting_balance(&self) -> f64 {
        self.starting_balance
            .expect("legacy starting_balance should be present")
    }

    pub fn legacy_return_assumption(&self) -> &ReturnAssumption {
        self.return_assumption
            .as_ref()
            .expect("legacy return_assumption should be present")
    }
}
