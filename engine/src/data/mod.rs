pub mod gifting;
pub mod insurance;
pub mod pension;
pub mod rates;
pub mod retirement;
pub mod social_security;
pub mod tax;
pub mod taxonomy;
pub mod types;

use serde_json::{json, Value};
use types::{CoverageFilter, DataError, FilingStatus, LookupParams};

/// Data module version.
pub fn data_version() -> &'static str {
    "0.1.3"
}

/// Years for which embedded data is available.
pub fn supported_years() -> Vec<u32> {
    vec![2025, 2026]
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
        "tax" | "retirement" | "social_security" | "insurance" | "pension" | "rates"
        | "gifting" => {}
        _ => return Err(DataError::UnknownCategory(category.to_string())),
    }

    let entry = taxonomy::all_entries()
        .into_iter()
        .find(|entry| entry.category == category && entry.key == key)
        .ok_or_else(|| DataError::UnknownKey(key.to_string()))?;
    if !entry.years.contains(&year) {
        return Err(DataError::UnsupportedYear(year));
    }

    match category {
        "tax" => lookup_tax(key, year, params),
        "retirement" => lookup_retirement(key, params),
        "social_security" => lookup_social_security(key, params),
        "insurance" => lookup_insurance(key, params),
        "pension" => lookup_pension(key, params),
        "rates" => lookup_rates(key, params),
        "gifting" => lookup_gifting(key, params),
        _ => unreachable!("category was validated above"),
    }
}

// ---------------------------------------------------------------------------
// Internal dispatchers by category
// ---------------------------------------------------------------------------

fn lookup_tax(key: &str, year: u32, params: &LookupParams) -> Result<Value, DataError> {
    match key {
        "federal_income_tax_brackets" => {
            let status = resolve_filing_status(params)?;
            let brackets = tax::federal::brackets_for_year(year, status)?;
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
        "federal_salt_deduction_parameters" => {
            let status = resolve_filing_status(params)?;
            let salt = tax::federal::salt_deduction_parameters(status);
            Ok(json!({
                "filing_status": status.to_string(),
                "cap_amount": salt.cap_amount,
                "phaseout_threshold": salt.phaseout_threshold,
                "phaseout_rate": salt.phaseout_rate,
                "floor_amount": salt.floor_amount,
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
                "required_beginning": {
                    "start_age_rules": rules.required_beginning.start_age_rules.iter().map(|r| json!({
                        "birth_year_min": r.birth_year_min,
                        "birth_year_max": r.birth_year_max,
                        "start_age": r.start_age,
                        "guidance_status": r.guidance_status,
                        "notes": r.notes,
                    })).collect::<Vec<_>>(),
                    "first_distribution_deadline": rules.required_beginning.first_distribution_deadline,
                    "still_working_exception": {
                        "eligible_plan_categories": rules.required_beginning.still_working_exception_plan_categories,
                        "eligible_account_types": rules.required_beginning.still_working_exception_eligible_account_types,
                        "disallowed_for_five_percent_owners": rules.required_beginning.still_working_exception_disallowed_for_five_percent_owners,
                    },
                },
                "account_applicability": {
                    "owner_required_account_types": rules.account_rules.owner_required_account_types,
                    "owner_exempt_account_types": rules.account_rules.owner_exempt_account_types,
                    "inherited_account_types": rules.account_rules.inherited_account_types,
                    "supports_pre_1987_403b_exclusion": rules.account_rules.supports_pre_1987_403b_exclusion,
                    "designated_roth_owner_exemption_effective_year": rules.account_rules.designated_roth_owner_exemption_effective_year,
                    "pre_1987_403b": {
                        "exclude_until_age": rules.pre_1987_403b_rules.exclude_until_age,
                    }
                },
                "beneficiary_distribution": {
                    "beneficiary_categories": rules.beneficiary_rules.beneficiary_categories,
                    "recognized_beneficiary_classes": rules.beneficiary_rules.recognized_beneficiary_classes,
                    "eligible_designated_beneficiary_classes": rules.beneficiary_rules.eligible_designated_beneficiary_classes,
                    "life_expectancy_method_by_class": rules.beneficiary_rules.life_expectancy_method_by_class,
                    "minor_child_majority_age": rules.beneficiary_rules.minor_child_majority_age,
                    "spouse_delay_allowed": rules.beneficiary_rules.spouse_delay_allowed,
                    "ten_year_rule": {
                        "terminal_year": rules.ten_year_rule.terminal_year,
                        "annual_distributions_required_when_owner_died_on_or_after_rbd": rules.ten_year_rule.annual_distributions_required_when_owner_died_on_or_after_rbd,
                    },
                    "non_designated_beneficiary_rules": {
                        "when_owner_died_before_required_beginning_date": rules.beneficiary_rules.non_designated_beneficiary_rules.when_owner_died_before_required_beginning_date,
                        "when_owner_died_on_or_after_required_beginning_date": rules.beneficiary_rules.non_designated_beneficiary_rules.when_owner_died_on_or_after_required_beginning_date
                    },
                    "relief_years": rules.relief_years,
                }
            }))
        }
        "contribution_limits" => {
            let l = retirement::contribution_limits::limits();
            Ok(json!({
                "elective_deferral_401k": l.elective_deferral_401k,
                "catch_up_401k_50_plus": l.catch_up_401k_50_plus,
                "catch_up_401k_60_to_63": l.catch_up_401k_60_to_63,
                "ira_contribution_limit": l.ira_contribution_limit,
                "ira_catch_up_50_plus": l.ira_catch_up_50_plus,
                "simple_elective_deferral": l.simple_elective_deferral,
                "simple_catch_up_50_plus": l.simple_catch_up_50_plus,
                "simple_catch_up_60_to_63": l.simple_catch_up_60_to_63,
                "sep_maximum_contribution": l.sep_maximum_contribution,
                "sep_minimum_compensation": l.sep_minimum_compensation,
                "annual_additions_limit_415c": l.annual_additions_limit_415c,
                "annual_compensation_limit": l.annual_compensation_limit,
                "defined_benefit_limit": l.defined_benefit_limit,
                "highly_compensated_threshold": l.highly_compensated_threshold,
                "key_employee_threshold": l.key_employee_threshold,
            }))
        }
        _ => Err(DataError::UnknownKey(key.to_string())),
    }
}

fn lookup_social_security(key: &str, params: &LookupParams) -> Result<Value, DataError> {
    match key {
        "full_retirement_age_rules" => {
            let rules = social_security::retirement_age::full_retirement_age_rules();
            Ok(json!({
                "benefit_scope": rules.benefit_scope,
                "january_1_births_use_prior_year": rules.january_1_births_use_prior_year,
                "rules": rules.rules.iter().map(|rule| json!({
                    "birth_year_min": rule.birth_year_min,
                    "birth_year_max": rule.birth_year_max,
                    "full_retirement_age_years": rule.full_retirement_age_years,
                    "full_retirement_age_months": rule.full_retirement_age_months,
                })).collect::<Vec<_>>(),
            }))
        }
        "retirement_earnings_test_thresholds" => {
            let t = social_security::earnings_test::thresholds();
            Ok(json!({
                "under_fra_annual_exempt_amount": t.under_fra_annual_exempt_amount,
                "under_fra_monthly_exempt_amount": t.under_fra_monthly_exempt_amount,
                "year_of_fra_annual_exempt_amount": t.year_of_fra_annual_exempt_amount,
                "year_of_fra_monthly_exempt_amount": t.year_of_fra_monthly_exempt_amount,
                "under_fra_reduction_rate": t.under_fra_reduction_rate,
                "year_of_fra_reduction_rate": t.year_of_fra_reduction_rate,
            }))
        }
        "benefit_taxation_thresholds" => {
            let status = resolve_filing_status(params)?;
            let lived_with_spouse_during_year =
                resolve_lived_with_spouse_during_year(params, status)?;
            let t = social_security::taxation::thresholds(status, lived_with_spouse_during_year)?;
            let mut result = json!({
                "filing_status": status.to_string(),
                "base_amount": t.base_amount,
                "upper_amount": t.upper_amount,
                "max_taxable_pct_below_upper": t.max_taxable_pct_below_upper,
                "max_taxable_pct_above_upper": t.max_taxable_pct_above_upper,
            });
            if let Some(lived_with_spouse_during_year) = lived_with_spouse_during_year {
                result["lived_with_spouse_during_year"] = json!(lived_with_spouse_during_year);
            }
            Ok(result)
        }
        _ => Err(DataError::UnknownKey(key.to_string())),
    }
}

fn lookup_insurance(key: &str, params: &LookupParams) -> Result<Value, DataError> {
    match key {
        "medicare_base_premiums" => {
            let premiums = insurance::medicare::base_premiums();
            Ok(json!({
                "part_b_standard_monthly_premium": premiums.part_b_standard_monthly_premium,
                "part_b_annual_deductible": premiums.part_b_annual_deductible,
                "part_d_base_beneficiary_premium": premiums.part_d_base_beneficiary_premium,
            }))
        }
        "irmaa_brackets" => {
            let status = resolve_filing_status(params)?;
            let lived_with_spouse_during_year =
                resolve_lived_with_spouse_during_year(params, status)?;
            let brackets = insurance::irmaa::brackets(status, lived_with_spouse_during_year)?;
            let mut result = json!({
                "filing_status": status.to_string(),
                "base_part_b_premium": insurance::irmaa::base_part_b_premium(),
                "brackets": brackets.iter().map(|b| json!({
                    "magi_min": b.magi_min,
                    "magi_max": b.magi_max,
                    "monthly_surcharge": b.monthly_surcharge,
                })).collect::<Vec<_>>(),
            });
            if let Some(lived_with_spouse_during_year) = lived_with_spouse_during_year {
                result["lived_with_spouse_during_year"] = json!(lived_with_spouse_during_year);
            }
            Ok(result)
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

fn lookup_rates(key: &str, _params: &LookupParams) -> Result<Value, DataError> {
    let afr = match key {
        "afr_2026_01" => rates::afr::afr_2026_01(),
        "afr_2026_02" => rates::afr::afr_2026_02(),
        "afr_2026_03" => rates::afr::afr_2026_03(),
        "section_7520_2026_01" => {
            return Ok(json!({ "rate": rates::section_7520::rate_2026_01() }))
        }
        "section_7520_2026_02" => {
            return Ok(json!({ "rate": rates::section_7520::rate_2026_02() }))
        }
        "section_7520_2026_03" => {
            return Ok(json!({ "rate": rates::section_7520::rate_2026_03() }))
        }
        _ => return Err(DataError::UnknownKey(key.to_string())),
    };
    Ok(json!({
        "short_term_annual": afr.short_term_annual,
        "short_term_semiannual": afr.short_term_semiannual,
        "short_term_quarterly": afr.short_term_quarterly,
        "short_term_monthly": afr.short_term_monthly,
        "mid_term_annual": afr.mid_term_annual,
        "mid_term_semiannual": afr.mid_term_semiannual,
        "mid_term_quarterly": afr.mid_term_quarterly,
        "mid_term_monthly": afr.mid_term_monthly,
        "long_term_annual": afr.long_term_annual,
        "long_term_semiannual": afr.long_term_semiannual,
        "long_term_quarterly": afr.long_term_quarterly,
        "long_term_monthly": afr.long_term_monthly,
    }))
}

fn lookup_gifting(key: &str, _params: &LookupParams) -> Result<Value, DataError> {
    match key {
        "federal_annual_exclusion" => {
            let e = gifting::annual_exclusion::exclusion();
            Ok(json!({
                "per_donee_exclusion": e.per_donee_exclusion,
                "non_citizen_spouse_exclusion": e.non_citizen_spouse_exclusion,
            }))
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

fn resolve_lived_with_spouse_during_year(
    params: &LookupParams,
    status: FilingStatus,
) -> Result<Option<bool>, DataError> {
    match status {
        FilingStatus::MarriedFilingSeparately => params
            .lived_with_spouse_during_year
            .ok_or_else(|| {
                DataError::InvalidParams(
                    "lived_with_spouse_during_year parameter is required when filing_status=married_filing_separately".to_string(),
                )
            })
            .map(Some),
        _ => {
            if params.lived_with_spouse_during_year.is_some() {
                Err(DataError::InvalidParams(
                    "lived_with_spouse_during_year is only valid when filing_status=married_filing_separately".to_string(),
                ))
            } else {
                Ok(None)
            }
        }
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
    fn supported_years_includes_2025() {
        assert!(supported_years().contains(&2025));
    }

    #[test]
    fn lookup_tax_brackets_single() {
        let params = LookupParams {
            filing_status: Some("single".to_string()),
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "federal_income_tax_brackets", 2026, &params).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 7);
        assert_eq!(arr[0]["rate"], 0.10);
    }

    #[test]
    fn lookup_tax_brackets_single_2025() {
        let params = LookupParams {
            filing_status: Some("single".to_string()),
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "federal_income_tax_brackets", 2025, &params).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 7);
        assert_eq!(arr[0]["max"], 11_925.0);
        assert_eq!(arr[5]["max"], 626_350.0);
    }

    #[test]
    fn lookup_tax_standard_deductions() {
        let params = LookupParams {
            filing_status: Some("mfj".to_string()),
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "federal_standard_deductions", 2026, &params).unwrap();
        assert_eq!(result["amount"], 32_200.0);
    }

    #[test]
    fn lookup_tax_salt_deduction_parameters() {
        let params = LookupParams {
            filing_status: Some("mfs".to_string()),
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "federal_salt_deduction_parameters", 2026, &params).unwrap();
        assert_eq!(result["filing_status"], "married_filing_separately");
        assert_eq!(result["cap_amount"], 20_200.0);
        assert_eq!(result["phaseout_threshold"], 252_500.0);
        assert_eq!(result["phaseout_rate"], 0.30);
        assert_eq!(result["floor_amount"], 5_000.0);
    }

    #[test]
    fn lookup_tax_niit() {
        let params = LookupParams {
            filing_status: Some("single".to_string()),
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "federal_net_investment_income_tax", 2026, &params).unwrap();
        assert_eq!(result["rate"], 0.038);
        assert_eq!(result["threshold"], 200_000.0);
    }

    #[test]
    fn lookup_estate_exemption() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "federal_estate_exemption", 2026, &params).unwrap();
        assert_eq!(result["exemption"], 15_000_000.0);
    }

    #[test]
    fn lookup_retirement_uniform() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
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
            lived_with_spouse_during_year: None,
        };
        let result = lookup(
            "social_security",
            "benefit_taxation_thresholds",
            2026,
            &params,
        )
        .unwrap();
        assert_eq!(result["base_amount"], 32_000.0);
    }

    #[test]
    fn lookup_ss_taxation_mfs_lived_with_spouse() {
        let params = LookupParams {
            filing_status: Some("mfs".to_string()),
            lived_with_spouse_during_year: Some(true),
        };
        let result = lookup(
            "social_security",
            "benefit_taxation_thresholds",
            2026,
            &params,
        )
        .unwrap();
        assert_eq!(result["lived_with_spouse_during_year"], true);
        assert_eq!(result["base_amount"], 0.0);
        assert_eq!(result["upper_amount"], 0.0);
    }

    #[test]
    fn lookup_ss_taxation_mfs_lived_apart() {
        let params = LookupParams {
            filing_status: Some("mfs".to_string()),
            lived_with_spouse_during_year: Some(false),
        };
        let result = lookup(
            "social_security",
            "benefit_taxation_thresholds",
            2026,
            &params,
        )
        .unwrap();
        assert_eq!(result["lived_with_spouse_during_year"], false);
        assert_eq!(result["base_amount"], 25_000.0);
        assert_eq!(result["upper_amount"], 34_000.0);
    }

    #[test]
    fn lookup_ss_taxation_mfs_requires_spouse_flag() {
        let params = LookupParams {
            filing_status: Some("mfs".to_string()),
            lived_with_spouse_during_year: None,
        };
        let result = lookup(
            "social_security",
            "benefit_taxation_thresholds",
            2026,
            &params,
        );
        assert!(result.is_err());
        let err = result.err().unwrap().to_string();
        assert!(err.contains("lived_with_spouse_during_year parameter is required"));
    }

    #[test]
    fn lookup_full_retirement_age_rules() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
        };
        let result = lookup(
            "social_security",
            "full_retirement_age_rules",
            2026,
            &params,
        )
        .unwrap();
        assert_eq!(result["benefit_scope"], "retirement_and_spousal");
        assert_eq!(result["january_1_births_use_prior_year"], true);
        let rules = result["rules"].as_array().unwrap();
        assert_eq!(rules[0]["birth_year_max"], 1937);
        assert_eq!(rules.last().unwrap()["birth_year_min"], 1960);
    }

    #[test]
    fn lookup_irmaa() {
        let params = LookupParams {
            filing_status: Some("single".to_string()),
            lived_with_spouse_during_year: None,
        };
        let result = lookup("insurance", "irmaa_brackets", 2026, &params).unwrap();
        assert_eq!(result["base_part_b_premium"], 202.9);
        let brackets = result["brackets"].as_array().unwrap();
        assert_eq!(brackets.len(), 6);
    }

    #[test]
    fn lookup_medicare_base_premiums() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
        };
        let result = lookup("insurance", "medicare_base_premiums", 2026, &params).unwrap();
        assert_eq!(result["part_b_standard_monthly_premium"], 202.9);
        assert_eq!(result["part_b_annual_deductible"], 283.0);
        assert_eq!(result["part_d_base_beneficiary_premium"], 38.99);
    }

    #[test]
    fn lookup_irmaa_mfs_lived_with_spouse() {
        let params = LookupParams {
            filing_status: Some("mfs".to_string()),
            lived_with_spouse_during_year: Some(true),
        };
        let result = lookup("insurance", "irmaa_brackets", 2026, &params).unwrap();
        let brackets = result["brackets"].as_array().unwrap();
        assert_eq!(result["lived_with_spouse_during_year"], true);
        assert_eq!(brackets.len(), 3);
    }

    #[test]
    fn lookup_irmaa_mfs_lived_apart() {
        let params = LookupParams {
            filing_status: Some("mfs".to_string()),
            lived_with_spouse_during_year: Some(false),
        };
        let result = lookup("insurance", "irmaa_brackets", 2026, &params).unwrap();
        let brackets = result["brackets"].as_array().unwrap();
        assert_eq!(result["lived_with_spouse_during_year"], false);
        assert_eq!(brackets.len(), 6);
    }

    #[test]
    fn lookup_irmaa_mfs_requires_spouse_flag() {
        let params = LookupParams {
            filing_status: Some("mfs".to_string()),
            lived_with_spouse_during_year: None,
        };
        let result = lookup("insurance", "irmaa_brackets", 2026, &params);
        assert!(matches!(result, Err(DataError::InvalidParams(_))));
    }

    #[test]
    fn lookup_pension_mortality() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
        };
        let result = lookup("pension", "mortality_417e", 2026, &params).unwrap();
        let arr = result.as_array().unwrap();
        assert!(!arr.is_empty());
    }

    #[test]
    fn lookup_unsupported_year() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "federal_income_tax_brackets", 2020, &params);
        assert!(matches!(result, Err(DataError::UnsupportedYear(2020))));
    }

    #[test]
    fn lookup_tax_entry_rejects_year_not_supported_for_entry() {
        let params = LookupParams {
            filing_status: Some("single".to_string()),
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "federal_standard_deductions", 2025, &params);
        assert!(matches!(result, Err(DataError::UnsupportedYear(2025))));
    }

    #[test]
    fn lookup_unknown_category() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
        };
        let result = lookup("foo", "bar", 2026, &params);
        assert!(matches!(result, Err(DataError::UnknownCategory(_))));
    }

    #[test]
    fn lookup_unknown_key() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "nonexistent", 2026, &params);
        assert!(matches!(result, Err(DataError::UnknownKey(_))));
    }

    #[test]
    fn lookup_missing_filing_status() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
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
            lived_with_spouse_during_year: None,
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
            lived_with_spouse_during_year: None,
        };
        let result = lookup("tax", "federal_capital_loss_limit", 2026, &params).unwrap();
        assert_eq!(result["limit"], 1_500.0);
    }

    #[test]
    fn lookup_distribution_rules() {
        let params = LookupParams {
            filing_status: None,
            lived_with_spouse_during_year: None,
        };
        let result = lookup("retirement", "distribution_rules", 2026, &params).unwrap();
        let start_age_rules = result["required_beginning"]["start_age_rules"]
            .as_array()
            .unwrap();
        assert_eq!(start_age_rules.len(), 4);
        assert_eq!(
            result["required_beginning"]["still_working_exception"]["eligible_plan_categories"],
            json!([
                "401k",
                "403b",
                "profit_sharing",
                "other_defined_contribution_plan"
            ])
        );
        assert_eq!(
            result["account_applicability"]["designated_roth_owner_exemption_effective_year"],
            json!(2024)
        );
        assert_eq!(
            result["account_applicability"]["owner_exempt_account_types"],
            json!(["roth_ira", "designated_roth_plan_account"])
        );
        assert_eq!(
            result["beneficiary_distribution"]["non_designated_beneficiary_rules"]
                ["when_owner_died_before_required_beginning_date"],
            json!("five_year_rule")
        );
        assert_eq!(
            result["beneficiary_distribution"]["relief_years"],
            json!([2021, 2022, 2023, 2024])
        );
    }
}
