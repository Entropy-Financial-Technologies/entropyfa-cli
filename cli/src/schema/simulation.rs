use serde_json::{json, Value};

pub fn simulate_schema() -> Value {
    json!({
        "command": "projection",
        "description": "Run Monte Carlo and deterministic projection of portfolio balance over time",
        "when_to_use": "When a user wants to project investment growth using either legacy aggregate inputs or bucketed household accounts, model retirement withdrawals, or assess probability of running out of money. Bucketed requests can include buckets, spending_policy, tax_policy, and rmd_policy. By default this returns both Monte Carlo and linear results as JSON. If you also want the human-facing terminal dashboard, add --visual. The dashboard stays aggregate-only for now, so there are no per-bucket charts yet. The same JSON envelope can optionally be POSTed with --result-hook-url.",
        "gather_from_user": {
            "required": [
                "time_horizon_months: projection length in months",
                "Either starting_balance + return_assumption for legacy aggregate requests, or buckets for household-account requests"
            ],
            "if_applicable": [
                "mode: 'both' (default), 'monte_carlo', or 'linear'",
                "num_simulations: number of Monte Carlo trials (default 10000, max 100000)",
                "seed: for reproducible results",
                "starting_balance: initial portfolio value for legacy aggregate requests",
                "return_assumption: {annual_mean, annual_std_dev} for legacy aggregate requests",
                "buckets: household account list with id, bucket_type, starting_balance, and return_assumption",
                "spending_policy: withdrawal_order and optional rebalance_tax_withholding_from",
                "tax_policy: optional modeled tax settings for bucketed requests",
                "rmd_policy: optional RMD settings for bucketed requests",
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
            "required": ["time_horizon_months"],
            "oneOf": [
                {
                    "title": "legacy_aggregate_request",
                    "required": ["starting_balance", "return_assumption"]
                },
                {
                    "title": "bucketed_household_request",
                    "required": ["buckets"]
                }
            ],
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
                "buckets": {
                    "type": "array",
                    "description": "Household account buckets used for bucketed simulations. Do not use the reserved __household_cash__ id.",
                    "items": {
                        "type": "object",
                        "required": ["id", "bucket_type", "starting_balance", "return_assumption"],
                        "properties": {
                            "id": {
                                "type": "string",
                                "description": "Unique bucket id. Reserved id: __household_cash__"
                            },
                            "bucket_type": {
                                "type": "string",
                                "enum": ["taxable", "tax_deferred", "tax_free", "cash"]
                            },
                            "starting_balance": {"type": "number"},
                            "return_assumption": {
                                "type": "object",
                                "required": ["annual_mean", "annual_std_dev"],
                                "properties": {
                                    "annual_mean": {"type": "number"},
                                    "annual_std_dev": {"type": "number"}
                                }
                            },
                            "realized_gain_ratio": {
                                "type": "number",
                                "minimum": 0.0,
                                "maximum": 1.0
                            },
                            "withdrawal_priority": {"type": "integer"}
                        }
                    }
                },
                "spending_policy": {
                    "type": "object",
                    "properties": {
                        "withdrawal_order": {
                            "type": "array",
                            "items": {"type": "string"}
                        },
                        "rebalance_tax_withholding_from": {"type": "string"}
                    }
                },
                "tax_policy": {
                    "type": "object",
                    "properties": {
                        "mode": {"type": "string"},
                        "modeled_tax_inflation_rate": {"type": "number"}
                    }
                },
                "rmd_policy": {
                    "type": "object",
                    "properties": {
                        "enabled": {"type": "boolean"},
                        "distribution_month": {"type": "integer", "minimum": 1, "maximum": 12}
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
            "monte_carlo.bucket_terminal_percentiles": "Bucket-level terminal balance percentiles, including the synthetic __household_cash__ summary bucket",
            "monte_carlo.bucket_depletion_counts": "How often each bucket is depleted across simulations",
            "linear.final_balance": "Deterministic ending balance",
            "linear.total_return_earned": "Cumulative investment returns",
            "linear.ending_balances_by_bucket": "Deterministic ending balances by bucket",
            "terminal_dashboard": "Rendered to stderr only when --visual is set, Monte Carlo output is available, and stderr is a terminal. The dashboard stays aggregate-only and does not show per-bucket charts yet.",
            "webhook_delivery": "If --result-hook-url is set, the same success or error envelope is POSTed as application/json"
        },
        "example": {
            "input": {
                "buckets": [
                    {
                        "id": "taxable",
                        "bucket_type": "taxable",
                        "starting_balance": 600000,
                        "return_assumption": {"annual_mean": 0.07, "annual_std_dev": 0.15},
                        "realized_gain_ratio": 0.25
                    },
                    {
                        "id": "ira",
                        "bucket_type": "tax_deferred",
                        "starting_balance": 400000,
                        "return_assumption": {"annual_mean": 0.06, "annual_std_dev": 0.10}
                    }
                ],
                "spending_policy": {"withdrawal_order": ["taxable", "ira"]},
                "time_horizon_months": 360,
                "cash_flows": [{"amount": -4000, "frequency": "monthly"}]
            },
            "command": "entropyfa compute projection --json '{\"buckets\":[{\"id\":\"taxable\",\"bucket_type\":\"taxable\",\"starting_balance\":600000,\"return_assumption\":{\"annual_mean\":0.07,\"annual_std_dev\":0.15},\"realized_gain_ratio\":0.25},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":400000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.10}}],\"spending_policy\":{\"withdrawal_order\":[\"taxable\",\"ira\"]},\"time_horizon_months\":360,\"cash_flows\":[{\"amount\":-4000,\"frequency\":\"monthly\"}]}'",
            "visual_command": "entropyfa compute projection --visual --json '{\"buckets\":[{\"id\":\"taxable\",\"bucket_type\":\"taxable\",\"starting_balance\":600000,\"return_assumption\":{\"annual_mean\":0.07,\"annual_std_dev\":0.15},\"realized_gain_ratio\":0.25},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":400000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.10}}],\"spending_policy\":{\"withdrawal_order\":[\"taxable\",\"ira\"]},\"time_horizon_months\":360,\"cash_flows\":[{\"amount\":-4000,\"frequency\":\"monthly\"}]}'"
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
