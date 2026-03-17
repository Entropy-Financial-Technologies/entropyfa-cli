use serde_json::{json, Value};

pub fn pension_schema() -> Value {
    json!({
        "command": "pension-comparison",
        "description": "Compare pension lump sum vs annuity options with tax impact, present value, and Monte Carlo analysis",
        "when_to_use": "When a user is deciding between taking a pension lump sum or annuity payments. Analyzes present value, breakeven age, tax impact, survivor benefits, and Monte Carlo success rates.",
        "gather_from_user": {
            "required": [
                "filing_status",
                "retiree_age: current age of the retiree",
                "annuity_options: array of pension payment options to compare",
                "mortality_table: actuarial mortality data (unisex_table required, male/female optional)",
                "investment_return: {annual_mean, annual_std_dev} for lump sum scenario"
            ],
            "if_applicable": [
                "lump_sum_amount: if lump sum option is available",
                "spouse_age, spouse_gender: for joint survivor analysis",
                "retiree_gender: for gender-specific mortality",
                "discount_rate: for present value calculation (default 5%)",
                "inflation_rate, cola_rate: for real-value analysis",
                "income fields (wages, etc.): for tax impact analysis",
                "deductions.itemized_amount or detailed itemized fields for SALT-aware tax modeling",
                "gross_social_security_benefit + ss_taxation_params: for SS interaction",
                "num_simulations, seed: Monte Carlo parameters",
                "rollover_to_ira: whether lump sum rolls to IRA (default true)",
                "uniform_lifetime_table: for RMD projections on lump sum"
            ]
        },
        "input_schema": {
            "type": "object",
            "required": ["filing_status", "retiree_age", "annuity_options", "mortality_table", "investment_return"],
            "properties": {
                "filing_status": {"type": "string"},
                "retiree_age": {"type": "integer"},
                "retiree_gender": {"type": "string", "enum": ["male", "female"]},
                "spouse_age": {"type": "integer"},
                "spouse_gender": {"type": "string"},
                "lump_sum_amount": {"type": "number"},
                "annuity_options": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["option_type", "monthly_payment"],
                        "properties": {
                            "option_type": {"type": "string", "enum": ["life_only", "joint_survivor", "period_certain", "life_with_period_certain", "level_income"]},
                            "monthly_payment": {"type": "number"},
                            "survivor_pct": {"type": "number"},
                            "period_certain_months": {"type": "integer"}
                        }
                    }
                },
                "investment_return": {
                    "type": "object",
                    "properties": {
                        "annual_mean": {"type": "number"},
                        "annual_std_dev": {"type": "number"}
                    }
                },
                "mortality_table": {
                    "type": "object",
                    "required": ["unisex_table"],
                    "properties": {
                        "unisex_table": {"type": "array", "items": {"type": "object", "properties": {"age": {"type": "integer"}, "qx": {"type": "number"}}}},
                        "male_table": {"type": "array"},
                        "female_table": {"type": "array"}
                    }
                },
                "discount_rate": {"type": "number", "default": 0.05},
                "inflation_rate": {"type": "number", "default": 0},
                "cola_rate": {"type": "number", "default": 0},
                "income": {"type": "object"},
                "deductions": {
                    "type": "object",
                    "properties": {
                        "method": {"type": "string", "enum": ["standard", "itemized"], "default": "standard"},
                        "itemized_amount": {"type": "number"},
                        "spouse_itemizes": {"type": "boolean"},
                        "state_local_income_or_sales_tax": {"type": "number"},
                        "real_property_tax": {"type": "number"},
                        "personal_property_tax": {"type": "number"},
                        "other_itemized_deductions": {"type": "number"}
                    }
                },
                "num_simulations": {"type": "integer"},
                "seed": {"type": "integer"},
                "rollover_to_ira": {"type": "boolean", "default": true}
            }
        },
        "output_summary": {
            "options_analyzed": "Detailed analysis of each annuity option",
            "lump_sum_monte_carlo": "Monte Carlo simulation of lump sum scenario",
            "comparison_summary": "Side-by-side comparison of all options",
            "tax_impact": "First-year tax analysis for annuity vs lump sum",
            "after_tax_projection": "Cumulative after-tax income over time",
            "survivor_analysis": "Comparison of survivor benefits (if spouse provided)"
        },
        "example": {
            "input": {
                "filing_status": "married_filing_jointly",
                "retiree_age": 62,
                "lump_sum_amount": 500000,
                "annuity_options": [{"option_type": "life_only", "monthly_payment": 3000}],
                "investment_return": {"annual_mean": 0.07, "annual_std_dev": 0.12},
                "mortality_table": {"unisex_table": [{"age": 62, "qx": 0.008}]}
            },
            "command": "entropyfa compute pension-comparison --json '{\"filing_status\":\"married_filing_jointly\",\"retiree_age\":62,\"lump_sum_amount\":500000,\"annuity_options\":[{\"option_type\":\"life_only\",\"monthly_payment\":3000}],\"investment_return\":{\"annual_mean\":0.07,\"annual_std_dev\":0.12},\"mortality_table\":{\"unisex_table\":[{\"age\":62,\"qx\":0.008}]}}'"
        },
        "related_commands": ["federal-tax", "projection", "rmd"]
    })
}
