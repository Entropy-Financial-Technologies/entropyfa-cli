use rand::Rng;

use crate::models::simulation_request::{CashFlow, ReturnAssumption, SimulationRequest};
use crate::models::simulation_response::MonteCarloResult;
use crate::models::solver_request::{SolverBounds, SolverRequest};
use crate::models::solver_response::SolverResponse;

use super::linear::run_linear;
use super::monte_carlo::run_monte_carlo;

pub fn run_solver(req: &SolverRequest) -> SolverResponse {
    let variable = req.solve_for.variable.as_str();
    // Generate a stable seed for this solve run — random if not provided
    let seed = req.seed.unwrap_or_else(|| rand::thread_rng().gen());
    let is_integer = variable == "time_horizon_months";
    let increasing = is_increasing(variable);

    let bounds = req
        .bounds
        .clone()
        .unwrap_or_else(|| default_bounds(variable));
    let max_iterations = req.max_iterations.unwrap_or(50);
    let tolerance = req
        .tolerance
        .unwrap_or_else(|| default_tolerance(&req.target.metric));

    let mut lo = bounds.lower;
    let mut hi = bounds.upper;
    let mut iterations: u32 = 0;
    let mut best_value = lo;
    let mut best_result = run_mc_at(req, seed, lo);
    let mut best_metric = extract_metric(
        &best_result,
        &req.target.metric,
        req.target.percentile.as_deref(),
    );

    for _ in 0..max_iterations {
        iterations += 1;

        let mid = if is_integer {
            ((lo + hi) / 2.0).round()
        } else {
            (lo + hi) / 2.0
        };

        let result = run_mc_at(req, seed, mid);
        let metric_value = extract_metric(
            &result,
            &req.target.metric,
            req.target.percentile.as_deref(),
        );

        best_value = mid;
        best_result = result;
        best_metric = metric_value;

        // Check convergence
        if (metric_value - req.target.value).abs() <= tolerance {
            return build_response(
                req,
                seed,
                best_value,
                best_metric,
                iterations,
                true,
                best_result,
            );
        }

        // For integer variables, terminate when range collapses
        if is_integer && (hi - lo) <= 1.0 {
            break;
        }

        // Bisect: determine which half contains the target
        let metric_too_low = metric_value < req.target.value;
        // If increasing monotonicity and metric too low, we need higher values
        // If decreasing monotonicity and metric too low, we need lower values
        if metric_too_low == increasing {
            lo = mid;
        } else {
            hi = mid;
        }

        // For continuous variables, check if range is negligibly small
        if !is_integer && (hi - lo).abs() < 1e-10 {
            break;
        }
    }

    let converged = (best_metric - req.target.value).abs() <= tolerance;
    build_response(
        req,
        seed,
        best_value,
        best_metric,
        iterations,
        converged,
        best_result,
    )
}

fn run_mc_at(req: &SolverRequest, seed: u64, candidate: f64) -> MonteCarloResult {
    let sim_req = build_sim_request(req, seed, candidate);
    run_monte_carlo(&sim_req)
}

fn build_sim_request(req: &SolverRequest, seed: u64, candidate: f64) -> SimulationRequest {
    let variable = req.solve_for.variable.as_str();

    let mut starting_balance = req.starting_balance;
    let mut time_horizon_months = req.time_horizon_months;
    let mut return_assumption = req.return_assumption.clone();
    let mut cash_flows = req.cash_flows.clone();

    match variable {
        "starting_balance" => {
            starting_balance = candidate;
        }
        "time_horizon_months" => {
            time_horizon_months = candidate.round().max(1.0) as u32;
        }
        "annual_return" => {
            return_assumption = ReturnAssumption {
                annual_mean: candidate,
                annual_std_dev: return_assumption.annual_std_dev,
            };
        }
        "cash_flow_amount" => {
            let idx = req.solve_for.cash_flow_index.unwrap_or(0);
            if idx < cash_flows.len() {
                cash_flows[idx] = CashFlow {
                    amount: candidate,
                    frequency: cash_flows[idx].frequency.clone(),
                    start_month: cash_flows[idx].start_month,
                    end_month: cash_flows[idx].end_month,
                };
            }
        }
        _ => {}
    }

    SimulationRequest {
        mode: Some("monte_carlo".into()),
        num_simulations: req.num_simulations.or(Some(10000)),
        seed: Some(seed),
        starting_balance: Some(starting_balance),
        buckets: vec![],
        time_horizon_months,
        return_assumption: Some(return_assumption),
        cash_flows,
        filing_status: None,
        household: None,
        spending_policy: None,
        tax_policy: None,
        rmd_policy: None,
        include_detail: false,
        detail_granularity: "annual".to_string(),
        sample_paths: None,
        path_indices: None,
        custom_percentiles: None,
    }
}

fn extract_metric(result: &MonteCarloResult, metric: &str, percentile: Option<&str>) -> f64 {
    match metric {
        "success_rate" => result.success_rate,
        "percentile_balance" => match percentile.unwrap_or("p50") {
            "p5" => result.percentiles.p5,
            "p10" => result.percentiles.p10,
            "p25" => result.percentiles.p25,
            "p50" => result.percentiles.p50,
            "p75" => result.percentiles.p75,
            "p90" => result.percentiles.p90,
            "p95" => result.percentiles.p95,
            _ => result.percentiles.p50,
        },
        _ => 0.0,
    }
}

/// Returns true if increasing the variable increases the metric (positive monotonicity).
fn is_increasing(variable: &str) -> bool {
    match variable {
        "starting_balance" => true,
        "cash_flow_amount" => true,
        "annual_return" => true,
        "time_horizon_months" => false,
        _ => true,
    }
}

fn default_bounds(variable: &str) -> SolverBounds {
    match variable {
        "starting_balance" => SolverBounds {
            lower: 0.0,
            upper: 10_000_000.0,
        },
        "cash_flow_amount" => SolverBounds {
            lower: -50_000.0,
            upper: 50_000.0,
        },
        "annual_return" => SolverBounds {
            lower: -0.10,
            upper: 0.20,
        },
        "time_horizon_months" => SolverBounds {
            lower: 12.0,
            upper: 720.0,
        },
        _ => SolverBounds {
            lower: 0.0,
            upper: 1_000_000.0,
        },
    }
}

fn default_tolerance(metric: &str) -> f64 {
    match metric {
        "success_rate" => 0.005,
        "percentile_balance" => 1000.0,
        _ => 0.005,
    }
}

fn build_response(
    req: &SolverRequest,
    seed: u64,
    solved_value: f64,
    achieved_value: f64,
    iterations: u32,
    converged: bool,
    mc_result: MonteCarloResult,
) -> SolverResponse {
    let rounded_value = if req.solve_for.variable == "time_horizon_months" {
        solved_value.round()
    } else {
        (solved_value * 100.0).round() / 100.0
    };

    let mode = req.mode.as_deref().unwrap_or("both");

    let simulation_result = if mode == "monte_carlo" || mode == "both" {
        Some(mc_result)
    } else {
        None
    };

    let linear = if mode == "linear" || mode == "both" {
        let linear_req = build_sim_request(req, seed, solved_value);
        Some(run_linear(&linear_req))
    } else {
        None
    };

    SolverResponse {
        solved_value: rounded_value,
        solved_variable: req.solve_for.variable.clone(),
        target_metric: req.target.metric.clone(),
        target_value: req.target.value,
        achieved_value,
        iterations,
        converged,
        computation_time_ms: 0.0, // filled in by the route handler
        simulation_result,
        linear,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::simulation_request::CashFlow;
    use crate::models::solver_request::{SolveFor, SolverTarget};

    fn base_solver_request() -> SolverRequest {
        SolverRequest {
            solve_for: SolveFor {
                variable: "cash_flow_amount".into(),
                cash_flow_index: Some(0),
            },
            target: SolverTarget {
                metric: "success_rate".into(),
                value: 0.90,
                percentile: None,
            },
            mode: None,
            num_simulations: Some(1000),
            seed: Some(42),
            starting_balance: 500_000.0,
            time_horizon_months: 360,
            return_assumption: ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            },
            cash_flows: vec![CashFlow {
                amount: -3000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            bounds: Some(SolverBounds {
                lower: -10000.0,
                upper: 0.0,
            }),
            max_iterations: Some(50),
            tolerance: Some(0.01),
        }
    }

    #[test]
    fn test_solver_converges_cash_flow() {
        let req = base_solver_request();
        let result = run_solver(&req);
        assert!(result.converged, "solver should converge");
        assert!(result.iterations > 0);
        assert!(result.solved_value < 0.0, "withdrawal should be negative");
        assert!(
            (result.achieved_value - 0.90).abs() <= 0.01,
            "achieved {:.4} should be near target 0.90",
            result.achieved_value
        );
    }

    #[test]
    fn test_solver_deterministic() {
        let req = base_solver_request();
        let r1 = run_solver(&req);
        let r2 = run_solver(&req);
        assert_eq!(r1.solved_value, r2.solved_value);
        assert_eq!(r1.iterations, r2.iterations);
        assert_eq!(r1.achieved_value, r2.achieved_value);
    }

    #[test]
    fn test_solver_starting_balance() {
        let req = SolverRequest {
            solve_for: SolveFor {
                variable: "starting_balance".into(),
                cash_flow_index: None,
            },
            target: SolverTarget {
                metric: "success_rate".into(),
                value: 0.90,
                percentile: None,
            },
            mode: None,
            num_simulations: Some(1000),
            seed: Some(42),
            starting_balance: 500_000.0,
            time_horizon_months: 360,
            return_assumption: ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            },
            cash_flows: vec![CashFlow {
                amount: -2000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            bounds: None,
            max_iterations: Some(50),
            tolerance: Some(0.01),
        };
        let result = run_solver(&req);
        assert!(
            result.converged,
            "solver should converge for starting_balance"
        );
        assert!(result.solved_value > 0.0);
    }

    #[test]
    fn test_solver_time_horizon_integer() {
        let req = SolverRequest {
            solve_for: SolveFor {
                variable: "time_horizon_months".into(),
                cash_flow_index: None,
            },
            target: SolverTarget {
                metric: "success_rate".into(),
                value: 0.90,
                percentile: None,
            },
            mode: None,
            num_simulations: Some(1000),
            seed: Some(42),
            starting_balance: 500_000.0,
            time_horizon_months: 360,
            return_assumption: ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            },
            cash_flows: vec![CashFlow {
                amount: -2000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            bounds: None,
            max_iterations: Some(50),
            tolerance: Some(0.01),
        };
        let result = run_solver(&req);
        // Solved value should be an integer (for months)
        assert_eq!(result.solved_value, result.solved_value.round());
    }

    #[test]
    fn test_solver_percentile_balance_target() {
        let req = SolverRequest {
            solve_for: SolveFor {
                variable: "cash_flow_amount".into(),
                cash_flow_index: Some(0),
            },
            target: SolverTarget {
                metric: "percentile_balance".into(),
                value: 100_000.0,
                percentile: Some("p50".into()),
            },
            mode: None,
            num_simulations: Some(1000),
            seed: Some(42),
            starting_balance: 500_000.0,
            time_horizon_months: 360,
            return_assumption: ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            },
            cash_flows: vec![CashFlow {
                amount: -3000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            bounds: Some(SolverBounds {
                lower: -10000.0,
                upper: 0.0,
            }),
            max_iterations: Some(50),
            tolerance: Some(5000.0),
        };
        let result = run_solver(&req);
        assert!(
            result.converged,
            "solver should converge for percentile_balance"
        );
    }

    #[test]
    fn test_build_sim_request_cash_flow() {
        let req = base_solver_request();
        let sim = build_sim_request(&req, 42, -2500.0);
        assert_eq!(sim.cash_flows[0].amount, -2500.0);
        assert_eq!(sim.starting_balance, Some(500_000.0));
        assert_eq!(sim.seed, Some(42));
    }

    #[test]
    fn test_build_sim_request_starting_balance() {
        let mut req = base_solver_request();
        req.solve_for.variable = "starting_balance".into();
        let sim = build_sim_request(&req, 42, 750_000.0);
        assert_eq!(sim.starting_balance, Some(750_000.0));
    }

    #[test]
    fn test_build_sim_request_annual_return() {
        let mut req = base_solver_request();
        req.solve_for.variable = "annual_return".into();
        let sim = build_sim_request(&req, 42, 0.05);
        assert_eq!(sim.return_assumption.as_ref().unwrap().annual_mean, 0.05);
        assert_eq!(sim.return_assumption.as_ref().unwrap().annual_std_dev, 0.15);
    }

    #[test]
    fn test_build_sim_request_time_horizon() {
        let mut req = base_solver_request();
        req.solve_for.variable = "time_horizon_months".into();
        let sim = build_sim_request(&req, 42, 240.0);
        assert_eq!(sim.time_horizon_months, 240);
    }

    #[test]
    fn test_extract_metric_success_rate() {
        let req = base_solver_request();
        let sim_req = build_sim_request(&req, 42, -2000.0);
        let result = run_monte_carlo(&sim_req);
        let val = extract_metric(&result, "success_rate", None);
        assert!((0.0..=1.0).contains(&val));
    }

    #[test]
    fn test_is_increasing() {
        assert!(is_increasing("starting_balance"));
        assert!(is_increasing("cash_flow_amount"));
        assert!(is_increasing("annual_return"));
        assert!(!is_increasing("time_horizon_months"));
    }

    #[test]
    fn test_default_bounds() {
        let b = default_bounds("starting_balance");
        assert_eq!(b.lower, 0.0);
        assert_eq!(b.upper, 10_000_000.0);

        let b = default_bounds("cash_flow_amount");
        assert_eq!(b.lower, -50_000.0);
        assert_eq!(b.upper, 50_000.0);
    }
}
