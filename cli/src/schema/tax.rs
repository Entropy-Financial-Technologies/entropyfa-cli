use serde_json::{json, Value};

pub fn federal_tax_schema() -> Value {
    json!({
        "command": "federal-tax",
        "description": "Compute federal income + payroll taxes for a given filing status and income breakdown",
        "when_to_use": "When a user asks about their federal tax liability, effective tax rate, marginal rate, or wants to understand how income is taxed across brackets. Also use for payroll tax (FICA/SE) calculations.",
        "gather_from_user": {
            "required": [
                "filing_status: one of single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse",
                "At least one income field (e.g. wages, self_employment_income, taxable_interest, etc.)"
            ],
            "if_applicable": [
                "qualified_dividends, long_term_capital_gains, short_term_capital_gains",
                "taxable_ira_distributions, taxable_pensions, taxable_social_security",
                "self_employment_income",
                "adjustments (hsa_deduction, ira_deduction, student_loan_interest)",
                "deductions.method: 'standard' or 'itemized' (defaults to standard)",
                "deductions.itemized_amount: total itemized deductions if method is 'itemized'"
            ]
        },
        "input_schema": {
            "type": "object",
            "required": ["filing_status"],
            "properties": {
                "filing_status": {"type": "string", "enum": ["single", "married_filing_jointly", "married_filing_separately", "head_of_household", "qualifying_surviving_spouse"]},
                "income": {
                    "type": "object",
                    "properties": {
                        "wages": {"type": "number", "default": 0},
                        "self_employment_income": {"type": "number", "default": 0},
                        "taxable_interest": {"type": "number", "default": 0},
                        "tax_exempt_interest": {"type": "number", "default": 0},
                        "ordinary_dividends": {"type": "number", "default": 0},
                        "qualified_dividends": {"type": "number", "default": 0},
                        "short_term_capital_gains": {"type": "number", "default": 0},
                        "long_term_capital_gains": {"type": "number", "default": 0},
                        "taxable_ira_distributions": {"type": "number", "default": 0},
                        "taxable_pensions": {"type": "number", "default": 0},
                        "taxable_social_security": {"type": "number", "default": 0},
                        "other_income": {"type": "number", "default": 0}
                    }
                },
                "adjustments": {
                    "type": "object",
                    "properties": {
                        "hsa_deduction": {"type": "number", "default": 0},
                        "ira_deduction": {"type": "number", "default": 0},
                        "student_loan_interest": {"type": "number", "default": 0},
                        "other_adjustments": {"type": "number", "default": 0}
                    }
                },
                "deductions": {
                    "type": "object",
                    "properties": {
                        "method": {"type": "string", "enum": ["standard", "itemized"], "default": "standard"},
                        "itemized_amount": {"type": "number"},
                        "spouse_itemizes": {"type": "boolean"}
                    }
                }
            }
        },
        "output_summary": {
            "gross_income": "Total gross income",
            "agi": "Adjusted gross income",
            "taxable_income": "Income subject to tax after deductions",
            "total_income_tax": "Federal income tax (ordinary + capital gains + NIIT)",
            "total_tax": "Income tax + payroll tax",
            "effective_rate": "Total income tax / gross income",
            "marginal_ordinary_rate": "Rate on next dollar of ordinary income",
            "marginal_capital_gains_rate": "Rate on next dollar of LTCG"
        },
        "example": {
            "input": {
                "filing_status": "married_filing_jointly",
                "income": {"wages": 150000, "qualified_dividends": 5000}
            },
            "command": "entropyfa compute federal-tax --json '{\"filing_status\":\"married_filing_jointly\",\"income\":{\"wages\":150000,\"qualified_dividends\":5000}}'"
        },
        "related_commands": ["estate-tax", "roth-conversion", "pension-comparison"]
    })
}

pub fn estate_tax_schema() -> Value {
    json!({
        "command": "estate-tax",
        "description": "Compute federal estate tax (Form 706) given a gross estate value",
        "when_to_use": "When a user asks about estate tax liability, exemption usage, or estate planning. Includes portability (DSUE) analysis.",
        "gather_from_user": {
            "required": [
                "gross_estate: total fair market value of the estate"
            ],
            "if_applicable": [
                "deductions.marital: marital deduction amount",
                "deductions.charitable: charitable deduction amount",
                "deductions.debts_and_expenses: debts, funeral, admin expenses",
                "adjusted_taxable_gifts: lifetime taxable gifts",
                "gift_tax_paid: gift tax previously paid",
                "deceased_spouse_unused_exclusion: DSUE (portability) amount"
            ]
        },
        "input_schema": {
            "type": "object",
            "required": ["gross_estate"],
            "properties": {
                "gross_estate": {"type": "number"},
                "deductions": {
                    "type": "object",
                    "properties": {
                        "marital": {"type": "number", "default": 0},
                        "charitable": {"type": "number", "default": 0},
                        "debts_and_expenses": {"type": "number", "default": 0},
                        "state_death_tax": {"type": "number", "default": 0},
                        "other": {"type": "number", "default": 0}
                    }
                },
                "adjusted_taxable_gifts": {"type": "number", "default": 0},
                "gift_tax_paid": {"type": "number", "default": 0},
                "deceased_spouse_unused_exclusion": {"type": "number", "default": 0}
            }
        },
        "output_summary": {
            "gross_estate": "Gross estate value",
            "taxable_estate": "Estate after deductions",
            "net_estate_tax": "Tax owed after credits",
            "effective_rate": "Net tax / gross estate",
            "marginal_rate": "Rate on next dollar",
            "exemption_amount": "2026 basic exclusion amount"
        },
        "example": {
            "input": {"gross_estate": 20000000, "deductions": {"marital": 5000000}},
            "command": "entropyfa compute estate-tax --json '{\"gross_estate\":20000000,\"deductions\":{\"marital\":5000000}}'"
        },
        "related_commands": ["federal-tax"]
    })
}
