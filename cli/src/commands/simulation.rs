use std::time::Instant;

use crate::{assembler, input, output, schema};
use entropyfa_engine::models::simulation_response::SimulationResponse;

pub fn run_simulate(
    schema_flag: bool,
    json_input: Option<String>,
    _chart: bool,
    detail: bool,
    detail_granularity: Option<String>,
    sample_paths: Option<usize>,
    path_indices: Option<Vec<usize>>,
    percentiles: Option<Vec<u32>>,
) {
    if schema_flag {
        output::print_success(schema::simulation::simulate_schema());
        return;
    }

    let mut input = input::parse_json_arg(json_input, "projection");
    if input.get("mode").is_none() {
        input["mode"] = serde_json::json!("both");
    }

    // CLI flags override JSON fields
    if detail {
        input["include_detail"] = serde_json::json!(true);
    }
    if let Some(ref gran) = detail_granularity {
        input["detail_granularity"] = serde_json::json!(gran);
    }
    if let Some(n) = sample_paths {
        input["sample_paths"] = serde_json::json!(n);
    }
    if let Some(ref indices) = path_indices {
        input["path_indices"] = serde_json::json!(indices);
    }
    if let Some(ref pcts) = percentiles {
        input["custom_percentiles"] = serde_json::json!(pcts);
    }

    let mode = input
        .get("mode")
        .and_then(|value| value.as_str())
        .unwrap_or("both");

    // Auto-request dashboard percentile bands for visualization.
    if mode == "monte_carlo" || mode == "both" {
        let mut chart_percentiles: Vec<u32> = input
            .get("custom_percentiles")
            .and_then(|v| v.as_array())
            .map(|vals| {
                vals.iter()
                    .filter_map(|v| v.as_u64())
                    .map(|v| v as u32)
                    .collect()
            })
            .unwrap_or_default();

        for pct in [20, 80] {
            if !chart_percentiles.contains(&pct) {
                chart_percentiles.push(pct);
            }
        }
        chart_percentiles.sort_unstable();
        chart_percentiles.dedup();
        input["custom_percentiles"] = serde_json::json!(chart_percentiles);
    }

    match assembler::simulation::assemble_simulate(&input) {
        Ok(req) => {
            let errors = entropyfa_engine::validation::validate_simulation_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }

            let mode = req.mode.as_deref().unwrap_or("both");
            let start = Instant::now();

            let monte_carlo = if mode == "monte_carlo" || mode == "both" {
                Some(entropyfa_engine::compute::simulation::monte_carlo::run_monte_carlo(&req))
            } else {
                None
            };
            let linear = if mode == "linear" || mode == "both" {
                Some(entropyfa_engine::compute::simulation::linear::run_linear(
                    &req,
                ))
            } else {
                None
            };
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;

            if let Some(mc) = monte_carlo.as_ref() {
                crate::chart::render_projection_dashboard(&req, mc, linear.as_ref(), elapsed);
            }

            let response = serde_json::to_value(SimulationResponse {
                request_echo: req,
                computation_time_ms: elapsed,
                monte_carlo,
                linear,
            })
            .expect("serialize SimulationResponse");

            output::print_success(response);
        }
        Err(e) => {
            output::print_error("assembly_error", &e);
            std::process::exit(1);
        }
    }
}

pub fn run_solve(schema_flag: bool, json_input: Option<String>) {
    if schema_flag {
        output::print_success(schema::simulation::solve_schema());
        return;
    }

    let input = input::parse_json_arg(json_input, "goal-solver");
    match assembler::simulation::assemble_solve(&input) {
        Ok(req) => {
            let errors = entropyfa_engine::validation::validate_solver_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }
            let result = entropyfa_engine::compute::simulation::solver::run_solver(&req);
            output::print_success(serde_json::to_value(&result).expect("serialize SolverResponse"));
        }
        Err(e) => {
            output::print_error("assembly_error", &e);
            std::process::exit(1);
        }
    }
}
