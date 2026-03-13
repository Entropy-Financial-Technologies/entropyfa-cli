use entropyfa_engine::data::taxonomy;
use entropyfa_engine::data::types::FilingStatus;
use serde_json::json;

use crate::output;

/// Handle `data lookup --category <cat> --key <key> [--year <y>] [--filing-status <fs>]`
pub fn run_lookup(category: &str, key: &str, year: u32, filing_status: Option<&str>) {
    // Validate the category/key exists in the taxonomy
    let entries = taxonomy::all_entries();
    let entry = entries
        .iter()
        .find(|e| e.category == category && e.key == key);

    let entry = match entry {
        Some(e) => e,
        None => {
            output::print_error(
                "not_found",
                &format!("No data entry for category={category}, key={key}"),
            );
            std::process::exit(1);
        }
    };

    if !entry.years.contains(&year) {
        output::print_error(
            "unsupported_year",
            &format!(
                "Year {year} not available for {category}/{key}. Available: {:?}",
                entry.years
            ),
        );
        std::process::exit(1);
    }

    // Dispatch to the appropriate data function
    let result = dispatch_lookup(category, key, filing_status);
    match result {
        Ok(data) => output::print_success(data),
        Err(e) => {
            output::print_error("lookup_error", &e);
            std::process::exit(1);
        }
    }
}

/// Handle `data coverage [--category <cat>] [--year <y>]`
pub fn run_coverage(category: Option<&str>, year: Option<u32>) {
    let entries = taxonomy::all_entries();
    let filtered: Vec<_> = entries
        .into_iter()
        .filter(|e| {
            if let Some(cat) = category {
                if e.category != cat {
                    return false;
                }
            }
            if let Some(y) = year {
                if !e.years.contains(&y) {
                    return false;
                }
            }
            true
        })
        .map(|e| {
            json!({
                "category": e.category,
                "key": e.key,
                "years": e.years,
                "params": e.params,
                "description": e.description,
                "tags": e.tags,
                "related_topics": e.related_topics,
            })
        })
        .collect();

    output::print_success(json!({
        "count": filtered.len(),
        "entries": filtered,
    }));
}

/// Dispatch a lookup to the correct embedded data function.
fn dispatch_lookup(
    category: &str,
    key: &str,
    filing_status: Option<&str>,
) -> Result<serde_json::Value, String> {
    use entropyfa_engine::data::tax::{estate, federal};

    match (category, key) {
        ("tax", "federal_income_tax_brackets") => {
            let fs = require_filing_status(filing_status)?;
            let brackets = federal::brackets(fs);
            Ok(json!(brackets
                .iter()
                .map(|b| json!({"min": b.min, "max": b.max, "rate": b.rate}))
                .collect::<Vec<_>>()))
        }
        ("tax", "federal_standard_deductions") => {
            let fs = require_filing_status(filing_status)?;
            Ok(json!({"amount": federal::standard_deductions(fs)}))
        }
        ("tax", "federal_capital_gains_brackets") => {
            let fs = require_filing_status(filing_status)?;
            let brackets = federal::capital_gains_brackets(fs);
            Ok(json!(brackets
                .iter()
                .map(|b| json!({"min": b.min, "max": b.max, "rate": b.rate}))
                .collect::<Vec<_>>()))
        }
        ("tax", "federal_net_investment_income_tax") => {
            let fs = require_filing_status(filing_status)?;
            let niit = federal::niit(fs);
            Ok(json!({"rate": niit.rate, "threshold": niit.threshold}))
        }
        ("tax", "federal_payroll_tax_parameters") => {
            let fs = require_filing_status(filing_status)?;
            let p = federal::payroll(fs);
            Ok(json!({
                "social_security_rate": p.social_security_rate,
                "social_security_wage_base": p.social_security_wage_base,
                "self_employment_tax_rate": p.self_employment_tax_rate,
                "medicare_rate": p.medicare_rate,
                "self_employment_medicare_rate": p.self_employment_medicare_rate,
                "additional_medicare_rate": p.additional_medicare_rate,
                "additional_medicare_threshold": p.additional_medicare_threshold,
            }))
        }
        ("tax", "federal_capital_loss_limit") => {
            let fs = require_filing_status(filing_status)?;
            Ok(json!({"limit": federal::capital_loss_limit(fs)}))
        }
        ("tax", "federal_qbi_deduction") => {
            let fs = require_filing_status(filing_status)?;
            let q = federal::qbi_deduction(fs);
            Ok(json!({
                "deduction_rate": q.deduction_rate,
                "threshold": q.threshold,
                "phase_in_range_end": q.phase_in_range_end,
                "minimum_qbi_deduction": q.minimum_qbi_deduction,
                "minimum_qbi_amount": q.minimum_qbi_amount,
            }))
        }
        ("tax", "federal_estate_exemption") => Ok(json!({"amount": estate::exemption()})),
        ("tax", "federal_estate_brackets") => {
            let brackets = estate::brackets();
            Ok(json!(brackets
                .iter()
                .map(|b| json!({"min": b.min, "max": b.max, "rate": b.rate}))
                .collect::<Vec<_>>()))
        }
        ("tax", "federal_estate_applicable_credit") => {
            Ok(json!({"amount": estate::applicable_credit()}))
        }
        _ => Err(format!(
            "Lookup not yet implemented for {category}/{key}. Use `data coverage` to see available entries."
        )),
    }
}

fn require_filing_status(fs: Option<&str>) -> Result<FilingStatus, String> {
    let s = fs.ok_or("--filing-status is required for this lookup")?;
    FilingStatus::parse(s).map_err(|e| e.to_string())
}
