use serde_json::{json, Value};

pub fn simulate_schema() -> Value {
    json!({
        "command": "projection",
        "description": "Run Monte Carlo and deterministic projection of portfolio balance over time",
        "when_to_use": "When a user wants to project investment growth, model retirement withdrawals, or assess probability of running out of money. By default this returns both Monte Carlo and linear results, and prints a terminal dashboard to stderr when run in a terminal.",
        "gather_from_user": {
            "required": [
                "starting_balance: initial portfolio value",
                "time_horizon_months: projection length in months",
                "return_assumption: {annual_mean, annual_std_dev}"
            ],
            "if_applicable": [
                "mode: 'both' (default), 'monte_carlo', or 'linear'",
                "num_simulations: number of Monte Carlo trials (default 10000, max 100000)",
                "seed: for reproducible results",
                "cash_flows: array of periodic deposits/withdrawals",
                "include_detail: true for period-by-period breakdown (or --detail flag)",
                "detail_granularity: 'annual' (default) or 'monthly' (or --detail-granularity flag)",
                "sample_paths: return N evenly-spaced simulation paths (or --sample-paths flag)",
                "path_indices: return specific simulation paths by index (or --path-indices flag)",
                "custom_percentiles: extra percentile time series 0-100 (or --percentiles flag)"
            ]
        },
        "input_schema": {
            "type": "object",
            "required": ["starting_balance", "time_horizon_months", "return_assumption"],
            "properties": {
                "mode": {"type": "string", "enum": ["monte_carlo", "linear", "both"], "default": "both"},
                "starting_balance": {"type": "number"},
                "time_horizon_months": {"type": "integer", "minimum": 1, "maximum": 1200},
                "return_assumption": {
                    "type": "object",
                    "required": ["annual_mean", "annual_std_dev"],
                    "properties": {
                        "annual_mean": {"type": "number", "description": "Expected annual return (e.g. 0.07 for 7%)"},
                        "annual_std_dev": {"type": "number", "description": "Annual volatility (e.g. 0.15 for 15%)"}
                    }
                },
                "cash_flows": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["amount", "frequency"],
                        "properties": {
                            "amount": {"type": "number", "description": "Positive = deposit, negative = withdrawal"},
                            "frequency": {"type": "string", "enum": ["monthly", "annually"]},
                            "start_month": {"type": "integer"},
                            "end_month": {"type": "integer"}
                        }
                    }
                },
                "num_simulations": {"type": "integer", "default": 10000},
                "seed": {"type": "integer"},
                "include_detail": {"type": "boolean", "default": false},
                "detail_granularity": {"type": "string", "default": "annual"},
                "sample_paths": {"type": "integer", "description": "Return N evenly-spaced simulation paths"},
                "path_indices": {"type": "array", "items": {"type": "integer"}, "description": "Return specific simulation paths by index"},
                "custom_percentiles": {"type": "array", "items": {"type": "integer", "minimum": 0, "maximum": 100}, "description": "Extra percentile time series (0-100)"}
            }
        },
        "output_summary": {
            "monte_carlo.percentiles": "p5/p10/p25/p50/p75/p90/p95 final balances",
            "monte_carlo.success_rate": "Probability of not depleting funds",
            "monte_carlo.time_series": "Balance percentiles over time",
            "monte_carlo.sample_paths": "Individual simulation paths (if requested)",
            "monte_carlo.custom_percentile_series": "Custom percentile time series (if requested)",
            "linear.final_balance": "Deterministic ending balance",
            "linear.total_return_earned": "Cumulative investment returns",
            "terminal_dashboard": "Rendered to stderr automatically when Monte Carlo output is available and stderr is a terminal"
        },
        "example": {
            "input": {
                "starting_balance": 1000000,
                "time_horizon_months": 360,
                "return_assumption": {"annual_mean": 0.07, "annual_std_dev": 0.15},
                "cash_flows": [{"amount": -4000, "frequency": "monthly"}]
            },
            "command": "entropyfa compute projection --json '{\"starting_balance\":1000000,\"time_horizon_months\":360,\"return_assumption\":{\"annual_mean\":0.07,\"annual_std_dev\":0.15},\"cash_flows\":[{\"amount\":-4000,\"frequency\":\"monthly\"}]}'"
        },
        "related_commands": ["goal-solver", "pension-comparison"]
    })
}

pub fn solve_schema() -> Value {
    json!({
        "command": "goal-solver",
        "description": "Binary search solver to find the value of a variable that achieves a target metric",
        "when_to_use": "When a user asks 'How much can I withdraw monthly and still have a 90% success rate?' or 'What starting balance do I need?' The solver iterates to find the answer.",
        "gather_from_user": {
            "required": [
                "solve_for: {variable, cash_flow_index} - what to solve for",
                "target: {metric, value, percentile} - what to achieve",
                "starting_balance, time_horizon_months, return_assumption (same as projection)"
            ],
            "if_applicable": [
                "mode: 'monte_carlo' or 'linear'",
                "bounds: {lower, upper} search range",
                "max_iterations: convergence limit (default 50)",
                "tolerance: convergence threshold",
                "cash_flows: baseline cash flows",
                "num_simulations, seed"
            ]
        },
        "input_schema": {
            "type": "object",
            "required": ["solve_for", "target", "starting_balance", "time_horizon_months", "return_assumption"],
            "properties": {
                "solve_for": {
                    "type": "object",
                    "required": ["variable"],
                    "properties": {
                        "variable": {"type": "string", "enum": ["starting_balance", "cash_flow_amount", "time_horizon_months"]},
                        "cash_flow_index": {"type": "integer", "description": "Required when variable is cash_flow_amount"}
                    }
                },
                "target": {
                    "type": "object",
                    "required": ["metric", "value"],
                    "properties": {
                        "metric": {"type": "string", "enum": ["success_rate", "final_balance", "final_balance_percentile"]},
                        "value": {"type": "number"},
                        "percentile": {"type": "string", "description": "e.g. 'p50' for final_balance_percentile metric"}
                    }
                },
                "starting_balance": {"type": "number"},
                "time_horizon_months": {"type": "integer"},
                "return_assumption": {"type": "object"},
                "cash_flows": {"type": "array"},
                "bounds": {
                    "type": "object",
                    "properties": {
                        "lower": {"type": "number"},
                        "upper": {"type": "number"}
                    }
                },
                "max_iterations": {"type": "integer", "default": 50},
                "tolerance": {"type": "number"}
            }
        },
        "output_summary": {
            "solved_value": "The answer found by the solver",
            "solved_variable": "Which variable was solved for",
            "target_metric": "What was being optimized",
            "achieved_value": "Actual metric value at solution",
            "converged": "Whether the solver converged",
            "iterations": "Number of iterations used"
        },
        "example": {
            "input": {
                "solve_for": {"variable": "cash_flow_amount", "cash_flow_index": 0},
                "target": {"metric": "success_rate", "value": 0.90},
                "starting_balance": 1000000,
                "time_horizon_months": 360,
                "return_assumption": {"annual_mean": 0.07, "annual_std_dev": 0.15},
                "cash_flows": [{"amount": -5000, "frequency": "monthly"}]
            },
            "command": "entropyfa compute goal-solver --json '{\"solve_for\":{\"variable\":\"cash_flow_amount\",\"cash_flow_index\":0},\"target\":{\"metric\":\"success_rate\",\"value\":0.90},\"starting_balance\":1000000,\"time_horizon_months\":360,\"return_assumption\":{\"annual_mean\":0.07,\"annual_std_dev\":0.15},\"cash_flows\":[{\"amount\":-5000,\"frequency\":\"monthly\"}]}'"
        },
        "related_commands": ["projection"]
    })
}
