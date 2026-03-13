pub mod insurance;
pub mod pension;
pub mod retirement;
pub mod social_security;
pub mod tax;
pub mod taxonomy;
pub mod types;

use serde_json::{json, Value};
use types::{CoverageFilter, DataError, FilingStatus, LookupParams};

/// Data module version.
pub fn data_version() -> &'static str {
    "0.1.0"
}

/// Years for which embedded data is available.
pub fn supported_years() -> Vec<u32> {
    vec![2026]
}

/// Return all coverage entries, optionally filtered.
pub fn coverage(filter: Option<&CoverageFilter>) -> Vec<taxonomy::CoverageEntry> {
    let entries = taxonomy::all_entries();
    match filter {
        None => entries,
        Some(f) => entries
            .into_iter()
            .filter(|e| {
                let cat_ok = f.category.as_ref().is_none_or(|c| e.category == *c);
                let tag_ok = f
                    .tag
                    .as_ref()
                    .is_none_or(|t| e.tags.iter().any(|et| et == t));
                cat_ok && tag_ok
            })
            .collect(),
    }
}

/// String-based lookup dispatcher for CLI `data lookup` commands.
///
/// Routes `category/key` combinations to the typed domain functions and
/// serializes results as `serde_json::Value`.
pub fn lookup(
    category: &str,
    key: &str,
    year: u32,
    params: &LookupParams,
) -> Result<Value, DataError> {
    if !supported_years().contains(&year) {
        return Err(DataError::UnsupportedYear(year));
    }

    match category {
        "tax" => lookup_tax(key, params),
        "retirement" => lookup_retirement(key, params),
        "social_security" => lookup_social_security(key, params),
        "insurance" => lookup_insurance(key, params),
        "pension" => lookup_pension(key, params),
        _ => Err(DataError::UnknownCategory(category.to_string())),
    }
}

// ---------------------------------------------------------------------------
// Internal dispatchers by category
// ---------------------------------------------------------------------------

fn lookup_tax(key: &str, params: &LookupParams) -> Result<Value, DataError> {
    match key {
        "federal_income_tax_brackets" => {
            let status = resolve_filing_status(params)?;
            let brackets = tax::federal::brackets(status);
            Ok(json!(brackets
                .iter()
                .map(|b| json!({
                    "min": b.min,
                    "max": b.max,
                    "rate": b.rate,
                }))
                .collect::<Vec<_>>()))
        }
        "federal_standard_deductions" => {
            let status = resolve_filing_status(params)?;
            Ok(json!({
                "filing_status": status.to_string(),
                "amount": tax::federal::standard_deductions(status),
            }))
        }
        "federal_capital_gains_brackets" => {
            let status = resolve_filing_status(params)?;
            let brackets = tax::federal::capital_gains_brackets(status);
            Ok(json!(brackets
                .iter()
                .map(|b| json!({
                    "min": b.min,
                    "max": b.max,
                    "rate": b.rate,
                }))
                .collect::<Vec<_>>()))
        }
        "federal_net_investment_income_tax" => {
            let status = resolve_filing_status(params)?;
            let n = tax::federal::niit(status);
            Ok(json!({
                "rate": n.rate,
                "threshold": n.threshold,
            }))
        }
        "federal_payroll_tax_parameters" => {
            let status = resolve_filing_status(params)?;
            let p = tax::federal::payroll(status);
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
        "federal_capital_loss_limit" => {
            let status = resolve_filing_status(params)?;
            Ok(json!({
                "filing_status": status.to_string(),
                "limit": tax::federal::capital_loss_limit(status),
            }))
        }
        "federal_qbi_deduction" => {
            let status = resolve_filing_status(params)?;
            let q = tax::federal::qbi_deduction(status);
            Ok(json!({
                "deduction_rate": q.deduction_rate,
                "threshold": q.threshold,
                "phase_in_range_end": q.phase_in_range_end,
                "minimum_qbi_deduction": q.minimum_qbi_deduction,
                "minimum_qbi_amount": q.minimum_qbi_amount,
            }))
        }
        "federal_estate_exemption" => Ok(json!({
            "exemption": tax::estate::exemption(),
        })),
        "federal_estate_brackets" => {
            let brackets = tax::estate::brackets();
            Ok(json!(brackets
                .iter()
                .map(|b| json!({
                    "min": b.min,
                    "max": b.max,
                    "rate": b.rate,
                }))
                .collect::<Vec<_>>()))
        }
        "federal_estate_applicable_credit" => Ok(json!({
            "applicable_credit": tax::estate::applicable_credit(),
        })),
        _ => Err(DataError::UnknownKey(key.to_string())),
    }
}

fn lookup_retirement(key: &str, _params: &LookupParams) -> Result<Value, DataError> {
    match key {
        "uniform_lifetime_table" => {
            let table = retirement::rmd_tables::uniform_lifetime();
            Ok(json!(table
                .iter()
                .map(|e| json!({ "age": e.age, "distribution_period": e.distribution_period }))
                .collect::<Vec<_>>()))
        }
        "single_life_table" => {
            let table = retirement::rmd_tables::single_life();
            Ok(json!(table
                .iter()
                .map(|e| json!({ "age": e.age, "distribution_period": e.distribution_period }))
                .collect::<Vec<_>>()))
        }
        "joint_life_table" => {
            let table = retirement::rmd_tables::joint_life();
            Ok(json!(table
                .iter()
                .map(|e| json!({
                    "owner_age": e.owner_age,
                    "spouse_age": e.spouse_age,
                    "distribution_period": e.distribution_period,
                }))
                .collect::<Vec<_>>()))
        }
        "distribution_rules" => {
            let rules = retirement::rmd_rules::distribution_rules();
            Ok(json!({
                "start_age_rules": rules.required_beginning.start_age_rules.iter().map(|r| json!({
                    "birth_year_min": r.birth_year_min,
                    "birth_year_max": r.birth_year_max,
                    "start_age": r.start_age,
                })).collect::<Vec<_>>(),
                "still_working_exception_account_types": rules.required_beginning.still_working_exception_account_types,
                "ten_year_rule": {
                    "terminal_year": rules.ten_year_rule.terminal_year,
                    "annual_distributions_required_when_owner_died_on_or_after_rbd": rules.ten_year_rule.annual_distributions_required_when_owner_died_on_or_after_rbd,
                },
                "relief_years": rules.relief_years,
                "beneficiary_classes": rules.beneficiary_rules.beneficiary_classes,
                "eligible_designated_beneficiary_classes": rules.beneficiary_rules.eligible_designated_beneficiary_classes,
            }))
        }
        _ => Err(DataError::UnknownKey(key.to_string())),
    }
}

fn lookup_social_security(key: &str, params: &LookupParams) -> Result<Value, DataError> {
    match key {
        "taxation_thresholds" => {
            let status = resolve_filing_status(params)?;
            let t = social_security::taxation::thresholds(status);
            Ok(json!({
                "filing_status": status.to_string(),
                "base_amount": t.base_amount,
                "upper_amount": t.upper_amount,
                "max_taxable_pct_below_upper": t.max_taxable_pct_below_upper,
                "max_taxable_pct_above_upper": t.max_taxable_pct_above_upper,
            }))
        }
        _ => Err(DataError::UnknownKey(key.to_string())),
    }
}

fn lookup_insurance(key: &str, params: &LookupParams) -> Result<Value, DataError> {
    match key {
        "irmaa_brackets" => {
            let status = resolve_filing_status(params)?;
            let brackets = insurance::irmaa::brackets(status);
            Ok(json!({
                "filing_status": status.to_string(),
                "base_part_b_premium": insurance::irmaa::base_part_b_premium(),
                "brackets": brackets.iter().map(|b| json!({
                    "magi_min": b.magi_min,
                    "magi_max": b.magi_max,
                    "monthly_surcharge": b.monthly_surcharge,
                })).collect::<Vec<_>>(),
            }))
        }
        _ => Err(DataError::UnknownKey(key.to_string())),
    }
}

fn lookup_pension(key: &str, _params: &LookupParams) -> Result<Value, DataError> {
    match key {
        "mortality_417e" => {
            let table = pension::mortality::table_417e();
            Ok(json!(table
                .iter()
                .map(|e| json!({ "age": e.age, "qx": e.qx }))
                .collect::<Vec<_>>()))
        }
        _ => Err(DataError::UnknownKey(key.to_string())),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn resolve_filing_status(params: &LookupParams) -> Result<FilingStatus, DataError> {
    match &params.filing_status {
        Some(s) => FilingStatus::parse(s),
        None => Err(DataError::InvalidParams(
            "filing_status parameter is required".to_string(),
        )),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_version_present() {
        assert!(!data_version().is_empty());
    }

    #[test]
    fn supported_years_includes_2026() {
        assert!(supported_years().contains(&2026));
    }

    #[test]
    fn lookup_tax_brackets_single() {
        let params = LookupParams {
            filing_status: Some("single".to_string()),
        };
        let result = lookup("tax", "federal_income_tax_brackets", 2026, &params).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 7);
        assert_eq!(arr[0]["rate"], 0.10);
    }

    #[test]
    fn lookup_tax_standard_deductions() {
        let params = LookupParams {
            filing_status: Some("mfj".to_string()),
        };
        let result = lookup("tax", "federal_standard_deductions", 2026, &params).unwrap();
        assert_eq!(result["amount"], 32_200.0);
    }

    #[test]
    fn lookup_tax_niit() {
        let params = LookupParams {
            filing_status: Some("single".to_string()),
        };
        let result = lookup("tax", "federal_net_investment_income_tax", 2026, &params).unwrap();
        assert_eq!(result["rate"], 0.038);
        assert_eq!(result["threshold"], 200_000.0);
    }

    #[test]
    fn lookup_estate_exemption() {
        let params = LookupParams {
            filing_status: None,
        };
        let result = lookup("tax", "federal_estate_exemption", 2026, &params).unwrap();
        assert_eq!(result["exemption"], 15_000_000.0);
    }

    #[test]
    fn lookup_retirement_uniform() {
        let params = LookupParams {
            filing_status: None,
        };
        let result = lookup("retirement", "uniform_lifetime_table", 2026, &params).unwrap();
        let arr = result.as_array().unwrap();
        assert!(!arr.is_empty());
        assert_eq!(arr[0]["age"], 72);
    }

    #[test]
    fn lookup_ss_taxation() {
        let params = LookupParams {
            filing_status: Some("mfj".to_string()),
        };
        let result = lookup("social_security", "taxation_thresholds", 2026, &params).unwrap();
        assert_eq!(result["base_amount"], 32_000.0);
    }

    #[test]
    fn lookup_irmaa() {
        let params = LookupParams {
            filing_status: Some("single".to_string()),
        };
        let result = lookup("insurance", "irmaa_brackets", 2026, &params).unwrap();
        assert_eq!(result["base_part_b_premium"], 185.0);
        let brackets = result["brackets"].as_array().unwrap();
        assert_eq!(brackets.len(), 6);
    }

    #[test]
    fn lookup_pension_mortality() {
        let params = LookupParams {
            filing_status: None,
        };
        let result = lookup("pension", "mortality_417e", 2026, &params).unwrap();
        let arr = result.as_array().unwrap();
        assert!(!arr.is_empty());
    }

    #[test]
    fn lookup_unsupported_year() {
        let params = LookupParams {
            filing_status: None,
        };
        let result = lookup("tax", "federal_income_tax_brackets", 2020, &params);
        assert!(matches!(result, Err(DataError::UnsupportedYear(2020))));
    }

    #[test]
    fn lookup_unknown_category() {
        let params = LookupParams {
            filing_status: None,
        };
        let result = lookup("foo", "bar", 2026, &params);
        assert!(matches!(result, Err(DataError::UnknownCategory(_))));
    }

    #[test]
    fn lookup_unknown_key() {
        let params = LookupParams {
            filing_status: None,
        };
        let result = lookup("tax", "nonexistent", 2026, &params);
        assert!(matches!(result, Err(DataError::UnknownKey(_))));
    }

    #[test]
    fn lookup_missing_filing_status() {
        let params = LookupParams {
            filing_status: None,
        };
        let result = lookup("tax", "federal_income_tax_brackets", 2026, &params);
        assert!(matches!(result, Err(DataError::InvalidParams(_))));
    }

    #[test]
    fn coverage_all() {
        let entries = coverage(None);
        assert!(entries.len() >= 15);
    }

    #[test]
    fn coverage_filter_category() {
        let filter = CoverageFilter {
            category: Some("tax".to_string()),
            tag: None,
        };
        let entries = coverage(Some(&filter));
        assert!(entries.iter().all(|e| e.category == "tax"));
        assert!(entries.len() >= 7);
    }

    #[test]
    fn coverage_filter_tag() {
        let filter = CoverageFilter {
            category: None,
            tag: Some("rmd".to_string()),
        };
        let entries = coverage(Some(&filter));
        assert!(entries.iter().all(|e| e.tags.contains(&"rmd".to_string())));
    }

    #[test]
    fn lookup_qbi_deduction() {
        let params = LookupParams {
            filing_status: Some("mfj".to_string()),
        };
        let result = lookup("tax", "federal_qbi_deduction", 2026, &params).unwrap();
        assert_eq!(result["deduction_rate"], 0.20);
        assert_eq!(result["threshold"], 403_500.0);
        assert_eq!(result["phase_in_range_end"], 553_500.0);
        assert_eq!(result["minimum_qbi_deduction"], 400.0);
        assert_eq!(result["minimum_qbi_amount"], 1_000.0);
    }

    #[test]
    fn lookup_capital_loss_limit() {
        let params = LookupParams {
            filing_status: Some("mfs".to_string()),
        };
        let result = lookup("tax", "federal_capital_loss_limit", 2026, &params).unwrap();
        assert_eq!(result["limit"], 1_500.0);
    }

    #[test]
    fn lookup_distribution_rules() {
        let params = LookupParams {
            filing_status: None,
        };
        let result = lookup("retirement", "distribution_rules", 2026, &params).unwrap();
        let start_age_rules = result["start_age_rules"].as_array().unwrap();
        assert_eq!(start_age_rules.len(), 3);
        assert_eq!(result["relief_years"], json!([2021, 2022, 2023, 2024]));
    }
}
