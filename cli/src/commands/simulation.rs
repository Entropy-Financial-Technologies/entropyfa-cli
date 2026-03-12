use serde_json::json;
use std::time::Instant;

use crate::{assembler, input, output, schema};

pub fn run_simulate(schema_flag: bool) {
    if schema_flag {
        output::print_success(schema::simulation::simulate_schema());
        return;
    }

    let input = input::read_stdin_json().unwrap_or(json!({}));
    match assembler::simulation::assemble_simulate(&input) {
        Ok(req) => {
            let errors = entropyfa_engine::validation::validate_simulation_request(&req);
            if !errors.is_empty() {
                output::print_error("validation_error", &errors.join("; "));
                std::process::exit(1);
            }

            let mode = req.mode.as_deref().unwrap_or("monte_carlo");
            let start = Instant::now();

            let response = match mode {
                "linear" => {
                    let linear = entropyfa_engine::compute::simulation::linear::run_linear(&req);
                    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                    serde_json::json!({
                        "request_echo": req,
                        "computation_time_ms": elapsed,
                        "linear": serde_json::to_value(&linear).expect("serialize LinearResult"),
                    })
                }
                _ => {
                    let mc =
                        entropyfa_engine::compute::simulation::monte_carlo::run_monte_carlo(&req);
                    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                    serde_json::json!({
                        "request_echo": req,
                        "computation_time_ms": elapsed,
                        "monte_carlo": serde_json::to_value(&mc).expect("serialize MonteCarloResult"),
                    })
                }
            };

            output::print_success(response);
        }
        Err(e) => {
            output::print_error("assembly_error", &e);
            std::process::exit(1);
        }
    }
}

pub fn run_solve(schema_flag: bool) {
    if schema_flag {
        output::print_success(schema::simulation::solve_schema());
        return;
    }

    let input = input::read_stdin_json().unwrap_or(json!({}));
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
