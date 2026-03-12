use serde::{Deserialize, Serialize};

use super::simulation_request::{CashFlow, ReturnAssumption};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverRequest {
    pub solve_for: SolveFor,
    pub target: SolverTarget,
    pub mode: Option<String>,
    pub num_simulations: Option<u32>,
    pub seed: Option<u64>,
    pub starting_balance: f64,
    pub time_horizon_months: u32,
    pub return_assumption: ReturnAssumption,
    #[serde(default)]
    pub cash_flows: Vec<CashFlow>,
    pub bounds: Option<SolverBounds>,
    pub max_iterations: Option<u32>,
    pub tolerance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolveFor {
    pub variable: String,
    pub cash_flow_index: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverTarget {
    pub metric: String,
    pub value: f64,
    pub percentile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverBounds {
    pub lower: f64,
    pub upper: f64,
}
