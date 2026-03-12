use entropyfa_engine::models::simulation_request::SimulationRequest;
use entropyfa_engine::models::solver_request::SolverRequest;
use serde_json::Value;

/// Simulation requests are pass-through -- no reference data assembly needed.
pub fn assemble_simulate(input: &Value) -> Result<SimulationRequest, String> {
    serde_json::from_value(input.clone()).map_err(|e| format!("invalid simulation input: {e}"))
}

/// Solver requests are pass-through -- no reference data assembly needed.
pub fn assemble_solve(input: &Value) -> Result<SolverRequest, String> {
    serde_json::from_value(input.clone()).map_err(|e| format!("invalid solver input: {e}"))
}
