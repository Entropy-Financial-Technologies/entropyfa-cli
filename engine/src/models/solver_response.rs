use serde::Serialize;

use super::simulation_response::{LinearResult, MonteCarloResult};

#[derive(Debug, Serialize)]
pub struct SolverResponse {
    pub solved_value: f64,
    pub solved_variable: String,
    pub target_metric: String,
    pub target_value: f64,
    pub achieved_value: f64,
    pub iterations: u32,
    pub converged: bool,
    pub computation_time_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_result: Option<MonteCarloResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<LinearResult>,
}
