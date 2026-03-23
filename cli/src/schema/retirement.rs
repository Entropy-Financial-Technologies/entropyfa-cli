use serde_json::{json, Value};

pub fn rmd_schema() -> Value {
    json!({
        "command": "rmd",
        "description": "Calculate the Required Minimum Distribution for a single year",
        "when_to_use": "When a user needs to know their RMD amount for a specific year. Handles owner-lifetime, inherited IRA, spouse beneficiary, and SECURE Act scenarios.",
        "gather_from_user": {
            "required": [
                "calculation_year: the tax year",
                "prior_year_end_balance: account balance as of Dec 31 of the prior year",
                "account_type: e.g. traditional_ira, 401k, 403b, inherited_ira",
                "owner_birth_date: YYYY-MM-DD format",
                "rmd_parameters: full RMD tables and rules (uniform_lifetime_table, joint_life_table, single_life_expectancy_table, required_beginning, account_rules, beneficiary_rules, ten_year_rule, pre_1987_403b_rules)"
            ],
            "if_applicable": [
                "spouse_birth_date, spouse_is_sole_beneficiary",
                "beneficiary_birth_date, beneficiary_class (for inherited accounts)",
                "is_still_working, is_five_percent_owner (for employer plans)",
                "owner_is_alive, owner_death_year (for inherited scenarios)",
                "pre_1987_403b_balance"
            ]
        },
        "input_schema": {
            "type": "object",
            "required": ["calculation_year", "prior_year_end_balance", "account_type", "rmd_parameters"],
            "properties": {
                "calculation_year": {"type": "integer"},
                "prior_year_end_balance": {"type": "number"},
                "account_type": {"type": "string"},
                "owner_birth_date": {"type": "string", "format": "date"},
                "owner_is_alive": {"type": "boolean"},
                "owner_death_year": {"type": "integer"},
                "beneficiary_birth_date": {"type": "string", "format": "date"},
                "beneficiary_class": {"type": "string"},
                "spouse_birth_date": {"type": "string", "format": "date"},
                "spouse_is_sole_beneficiary": {"type": "boolean"},
                "is_still_working": {"type": "boolean"},
                "is_five_percent_owner": {"type": "boolean"},
                "rmd_parameters": {"type": "object", "description": "Full RMD tables and distribution rules"}
            }
        },
        "output_summary": {
            "rmd_required": "Whether an RMD is required this year",
            "rmd_amount": "The calculated RMD amount",
            "distribution_period": "Divisor used from the applicable table",
            "table_used": "Which IRS table was applied",
            "rule_path": "Decision path through SECURE Act rules",
            "decision_trace": "Step-by-step reasoning"
        },
        "example": {
            "input": {
                "calculation_year": 2026,
                "prior_year_end_balance": 500000,
                "account_type": "traditional_ira",
                "owner_birth_date": "1953-06-15"
            },
            "command": "entropyfa compute rmd --json '{\"calculation_year\":2026,\"prior_year_end_balance\":500000,\"account_type\":\"traditional_ira\",\"owner_birth_date\":\"1953-06-15\"}'"
        },
        "related_commands": ["rmd-schedule", "roth-conversion", "roth-conversion-strategy"]
    })
}

pub fn rmd_schedule_schema() -> Value {
    json!({
        "command": "rmd-schedule",
        "description": "Project multi-year RMD schedule with balance depletion over time",
        "when_to_use": "When a user wants to see how their RMDs will look over many years, project account balance, or plan withdrawal strategy over their lifetime.",
        "gather_from_user": {
            "required": [
                "Same as rmd, plus:",
                "annual_growth_rate: assumed investment return (e.g. 0.06 for 6%)"
            ],
            "if_applicable": [
                "end_age: age to project through (default: based on table)",
                "max_years: alternative to end_age"
            ]
        },
        "input_schema": {
            "type": "object",
            "required": ["calculation_year", "prior_year_end_balance", "account_type", "rmd_parameters"],
            "properties": {
                "calculation_year": {"type": "integer"},
                "prior_year_end_balance": {"type": "number"},
                "account_type": {"type": "string"},
                "owner_birth_date": {"type": "string", "format": "date"},
                "annual_growth_rate": {"type": "number"},
                "end_age": {"type": "integer"},
                "max_years": {"type": "integer"},
                "rmd_parameters": {"type": "object"}
            }
        },
        "output_summary": {
            "rows": "Array of yearly projections with balance, RMD, and end balance",
            "start_year": "First year of projection",
            "end_year": "Last year of projection"
        },
        "example": {
            "input": {
                "calculation_year": 2026,
                "prior_year_end_balance": 1000000,
                "account_type": "traditional_ira",
                "owner_birth_date": "1955-03-20",
                "annual_growth_rate": 0.06
            },
            "command": "entropyfa compute rmd-schedule --json '{\"calculation_year\":2026,\"prior_year_end_balance\":1000000,\"account_type\":\"traditional_ira\",\"owner_birth_date\":\"1955-03-20\",\"annual_growth_rate\":0.06}'"
        },
        "related_commands": ["rmd", "roth-conversion-strategy"]
    })
}

pub fn roth_schema() -> Value {
    json!({
        "command": "roth-conversion",
        "description": "Analyze the tax impact of a single-year Roth conversion",
        "when_to_use": "When a user asks about converting traditional IRA to Roth, wants to know the tax cost of a conversion, or needs bracket-space analysis for optimal conversion amount.",
        "gather_from_user": {
            "required": [
                "filing_status",
                "traditional_ira_balance: current traditional IRA balance",
                "At least one income field"
            ],
            "if_applicable": [
                "tax_year: tax year for embedded federal tax parameters (defaults to 2026; 2025 currently has ordinary brackets only)",
                "conversion_amount: specific amount to convert (auto-fills bracket if omitted)",
                "nondeductible_basis: non-deductible IRA contributions (Form 8606)",
                "total_traditional_ira_value: for pro-rata rule if multiple IRA types",
                "deductions.itemized_amount or detailed itemized fields for SALT-aware tax modeling",
                "irmaa_brackets: for Medicare surcharge analysis",
                "gross_social_security_benefit: for SS taxation interaction",
                "ss_taxation_params: thresholds for SS taxation"
            ]
        },
        "input_schema": {
            "type": "object",
            "required": ["filing_status", "traditional_ira_balance"],
            "properties": {
                "filing_status": {"type": "string"},
                "tax_year": {"type": "integer", "default": 2026},
                "traditional_ira_balance": {"type": "number"},
                "conversion_amount": {"type": "number"},
                "nondeductible_basis": {"type": "number", "default": 0},
                "total_traditional_ira_value": {"type": "number"},
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
                "irmaa_brackets": {"type": "object"},
                "gross_social_security_benefit": {"type": "number"},
                "ss_taxation_params": {"type": "object"}
            }
        },
        "output_summary": {
            "baseline": "Tax liability without conversion",
            "with_conversion": "Tax liability with conversion",
            "conversion_tax_cost": "Additional tax from conversion",
            "effective_rate_on_conversion": "Tax cost / conversion amount",
            "bracket_space": "Room in each bracket for conversion",
            "pro_rata": "Form 8606 pro-rata calculation",
            "irmaa_impact": "Medicare surcharge impact (if brackets provided)"
        },
        "example": {
            "input": {
                "filing_status": "married_filing_jointly",
                "income": {"wages": 80000},
                "traditional_ira_balance": 500000,
                "conversion_amount": 50000
            },
            "command": "entropyfa compute roth-conversion --json '{\"filing_status\":\"married_filing_jointly\",\"income\":{\"wages\":80000},\"traditional_ira_balance\":500000,\"conversion_amount\":50000}'"
        },
        "related_commands": ["roth-conversion-strategy", "federal-tax", "rmd"]
    })
}

pub fn roth_strategy_schema() -> Value {
    json!({
        "command": "roth-conversion-strategy",
        "description": "Multi-year Roth conversion strategy with projected balances and RMD comparison",
        "when_to_use": "When a user wants to optimize Roth conversions over multiple years, compare convert vs. do-nothing scenarios, or find the breakeven year for a conversion strategy.",
        "gather_from_user": {
            "required": [
                "filing_status",
                "traditional_ira_balance",
                "owner_birth_date: YYYY-MM-DD",
                "annual_growth_rate: assumed return (e.g. 0.06)",
                "projection_years: number of years to project",
                "strategy: one of 'fill_bracket', 'fixed_amount', 'full_conversion'"
            ],
            "if_applicable": [
                "tax_year: starting tax year for embedded federal tax parameters (defaults to 2026; 2025 currently has ordinary brackets only)",
                "target_bracket_rate: for fill_bracket strategy (e.g. 0.24)",
                "fixed_annual_conversion: for fixed_amount strategy",
                "roth_ira_balance: current Roth balance",
                "deductions.itemized_amount or detailed itemized fields for SALT-aware tax modeling",
                "income_events: future income changes (e.g. retirement, SS start)",
                "uniform_lifetime_table + rmd_start_age: for RMD projections"
            ]
        },
        "input_schema": {
            "type": "object",
            "required": ["filing_status", "traditional_ira_balance", "owner_birth_date", "annual_growth_rate", "projection_years", "strategy"],
            "properties": {
                "filing_status": {"type": "string"},
                "tax_year": {"type": "integer", "default": 2026},
                "traditional_ira_balance": {"type": "number"},
                "roth_ira_balance": {"type": "number", "default": 0},
                "owner_birth_date": {"type": "string", "format": "date"},
                "annual_growth_rate": {"type": "number"},
                "projection_years": {"type": "integer"},
                "strategy": {"type": "string", "enum": ["fill_bracket", "fixed_amount", "full_conversion"]},
                "target_bracket_rate": {"type": "number"},
                "fixed_annual_conversion": {"type": "number"},
                "income_events": {"type": "array"},
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
                "uniform_lifetime_table": {"type": "array"},
                "rmd_start_age": {"type": "integer"}
            }
        },
        "output_summary": {
            "annual_detail": "Year-by-year conversion amounts, tax costs, and balances",
            "convert_scenario": "Final balances and total taxes in conversion scenario",
            "do_nothing_scenario": "Final balances and total taxes without conversion",
            "comparison.net_tax_savings": "Total tax savings from converting",
            "comparison.breakeven_year": "Year when conversion becomes advantageous"
        },
        "example": {
            "input": {
                "filing_status": "married_filing_jointly",
                "traditional_ira_balance": 800000,
                "owner_birth_date": "1962-01-15",
                "annual_growth_rate": 0.06,
                "projection_years": 20,
                "strategy": "fill_bracket",
                "target_bracket_rate": 0.24
            },
            "command": "entropyfa compute roth-conversion-strategy --json '{\"filing_status\":\"married_filing_jointly\",\"traditional_ira_balance\":800000,\"owner_birth_date\":\"1962-01-15\",\"annual_growth_rate\":0.06,\"projection_years\":20,\"strategy\":\"fill_bracket\",\"target_bracket_rate\":0.24}'"
        },
        "related_commands": ["roth-conversion", "rmd", "rmd-schedule", "federal-tax"]
    })
}
