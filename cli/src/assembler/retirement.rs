use entropyfa_engine::data::tax::federal;
use entropyfa_engine::data::types::{DataError, FilingStatus};
use entropyfa_engine::models::retirement_rmd::{
    RetirementRmdRequest, RetirementRmdScheduleRequest,
};
use entropyfa_engine::models::roth_conversion::{
    RothConversionRequest, RothConversionStrategyRequest,
};
use entropyfa_engine::models::tax_request::{
    Adjustments, DeductionConfig, IncomeBreakdown, TaxParameters,
};
use serde_json::{json, Value};

use super::parse_filing_status;
use crate::support::load_rmd_reference_bundle;
use crate::support::reference_paths::resolve_compute_reference_root;

pub fn assemble_rmd(input: &Value) -> Result<RetirementRmdRequest, String> {
    let normalized = normalize_rmd_input(input)?;
    serde_json::from_value(normalized).map_err(|e| format!("invalid RMD input: {e}"))
}

pub fn assemble_rmd_schedule(input: &Value) -> Result<RetirementRmdScheduleRequest, String> {
    let normalized = normalize_rmd_input(input)?;
    serde_json::from_value(normalized).map_err(|e| format!("invalid RMD schedule input: {e}"))
}

/// Assemble a Roth conversion request from user JSON + embedded tax data.
pub fn assemble_roth(input: &Value) -> Result<RothConversionRequest, String> {
    let fs = parse_filing_status(input)?;
    let tax_year = input["tax_year"].as_u64().unwrap_or(2026) as u32;
    let tax_params = build_tax_params(fs, tax_year)?;

    let income: IncomeBreakdown = serde_json::from_value(
        input
            .get("income")
            .cloned()
            .unwrap_or(Value::Object(Default::default())),
    )
    .map_err(|e| format!("invalid income: {e}"))?;

    let adjustments: Adjustments = serde_json::from_value(
        input
            .get("adjustments")
            .cloned()
            .unwrap_or(Value::Object(Default::default())),
    )
    .map_err(|e| format!("invalid adjustments: {e}"))?;

    let deductions: DeductionConfig = if let Some(v) = input.get("deductions") {
        serde_json::from_value(v.clone()).map_err(|e| format!("invalid deductions: {e}"))?
    } else {
        DeductionConfig {
            method: "standard".to_string(),
            itemized_amount: None,
            spouse_itemizes: None,
            state_local_income_or_sales_tax: None,
            real_property_tax: None,
            personal_property_tax: None,
            other_itemized_deductions: None,
        }
    };

    let traditional_ira_balance = input["traditional_ira_balance"]
        .as_f64()
        .ok_or("missing required field: traditional_ira_balance")?;

    let conversion_amount = input.get("conversion_amount").and_then(|v| v.as_f64());
    let nondeductible_basis = input["nondeductible_basis"].as_f64().unwrap_or(0.0);
    let total_traditional_ira_value = input
        .get("total_traditional_ira_value")
        .and_then(|v| v.as_f64());

    let irmaa_brackets = input
        .get("irmaa_brackets")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    let gross_social_security_benefit = input
        .get("gross_social_security_benefit")
        .and_then(|v| v.as_f64());

    let ss_taxation_params = input
        .get("ss_taxation_params")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    Ok(RothConversionRequest {
        filing_status: fs.to_string(),
        tax_year,
        income,
        adjustments,
        deductions,
        tax_parameters: tax_params,
        conversion_amount,
        traditional_ira_balance,
        nondeductible_basis,
        total_traditional_ira_value,
        irmaa_brackets,
        gross_social_security_benefit,
        ss_taxation_params,
    })
}

/// Assemble a Roth conversion strategy request from user JSON + embedded tax data.
pub fn assemble_roth_strategy(input: &Value) -> Result<RothConversionStrategyRequest, String> {
    let fs = parse_filing_status(input)?;
    let tax_year = input["tax_year"].as_u64().unwrap_or(2026) as u32;
    let tax_params = build_tax_params(fs, tax_year)?;

    let income: IncomeBreakdown = serde_json::from_value(
        input
            .get("income")
            .cloned()
            .unwrap_or(Value::Object(Default::default())),
    )
    .map_err(|e| format!("invalid income: {e}"))?;

    let adjustments: Adjustments = serde_json::from_value(
        input
            .get("adjustments")
            .cloned()
            .unwrap_or(Value::Object(Default::default())),
    )
    .map_err(|e| format!("invalid adjustments: {e}"))?;

    let deductions: DeductionConfig = if let Some(v) = input.get("deductions") {
        serde_json::from_value(v.clone()).map_err(|e| format!("invalid deductions: {e}"))?
    } else {
        DeductionConfig {
            method: "standard".to_string(),
            itemized_amount: None,
            spouse_itemizes: None,
            state_local_income_or_sales_tax: None,
            real_property_tax: None,
            personal_property_tax: None,
            other_itemized_deductions: None,
        }
    };

    let traditional_ira_balance = input["traditional_ira_balance"]
        .as_f64()
        .ok_or("missing required field: traditional_ira_balance")?;
    let owner_birth_date = input["owner_birth_date"]
        .as_str()
        .ok_or("missing required field: owner_birth_date")?
        .to_string();
    let annual_growth_rate = input["annual_growth_rate"]
        .as_f64()
        .ok_or("missing required field: annual_growth_rate")?;
    let projection_years = input["projection_years"]
        .as_u64()
        .ok_or("missing required field: projection_years")? as u32;
    let strategy = input["strategy"]
        .as_str()
        .ok_or("missing required field: strategy")?
        .to_string();

    let roth_ira_balance = input["roth_ira_balance"].as_f64().unwrap_or(0.0);
    let nondeductible_basis = input["nondeductible_basis"].as_f64().unwrap_or(0.0);
    let total_traditional_ira_value = input
        .get("total_traditional_ira_value")
        .and_then(|v| v.as_f64());
    let target_bracket_rate = input.get("target_bracket_rate").and_then(|v| v.as_f64());
    let fixed_annual_conversion = input
        .get("fixed_annual_conversion")
        .and_then(|v| v.as_f64());

    let income_events = input
        .get("income_events")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    let irmaa_brackets = input
        .get("irmaa_brackets")
        .and_then(|v| serde_json::from_value(v.clone()).ok());
    let gross_social_security_benefit = input
        .get("gross_social_security_benefit")
        .and_then(|v| v.as_f64());
    let ss_taxation_params = input
        .get("ss_taxation_params")
        .and_then(|v| serde_json::from_value(v.clone()).ok());
    let uniform_lifetime_table = input
        .get("uniform_lifetime_table")
        .and_then(|v| serde_json::from_value(v.clone()).ok());
    let rmd_start_age = input
        .get("rmd_start_age")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);

    Ok(RothConversionStrategyRequest {
        filing_status: fs.to_string(),
        tax_year,
        income,
        adjustments,
        deductions,
        tax_parameters: tax_params,
        traditional_ira_balance,
        roth_ira_balance,
        nondeductible_basis,
        total_traditional_ira_value,
        owner_birth_date,
        annual_growth_rate,
        projection_years,
        strategy,
        target_bracket_rate,
        fixed_annual_conversion,
        income_events,
        irmaa_brackets,
        gross_social_security_benefit,
        ss_taxation_params,
        uniform_lifetime_table,
        rmd_start_age,
    })
}

/// Build TaxParameters from embedded data for a given filing status.
fn build_tax_params(fs: FilingStatus, tax_year: u32) -> Result<TaxParameters, String> {
    Ok(TaxParameters {
        ordinary_brackets: federal::brackets_for_year(tax_year, fs)
            .map_err(|err| format_tax_data_error("federal_income_tax_brackets", tax_year, err))?,
        capital_gains_brackets: federal::capital_gains_brackets_for_year(tax_year, fs).map_err(
            |err| format_tax_data_error("federal_capital_gains_brackets", tax_year, err),
        )?,
        standard_deduction: federal::standard_deductions_for_year(tax_year, fs)
            .map_err(|err| format_tax_data_error("federal_standard_deductions", tax_year, err))?,
        capital_loss_limit: federal::capital_loss_limit_for_year(tax_year, fs)
            .map_err(|err| format_tax_data_error("federal_capital_loss_limit", tax_year, err))?,
        niit: federal::niit_for_year(tax_year, fs).map_err(|err| {
            format_tax_data_error("federal_net_investment_income_tax", tax_year, err)
        })?,
        payroll: federal::payroll_for_year(tax_year, fs).map_err(|err| {
            format_tax_data_error("federal_payroll_tax_parameters", tax_year, err)
        })?,
        salt: Some(
            federal::salt_deduction_parameters_for_year(tax_year, fs).map_err(|err| {
                format_tax_data_error("federal_salt_deduction_parameters", tax_year, err)
            })?,
        ),
    })
}

fn format_tax_data_error(entry_key: &str, tax_year: u32, error: DataError) -> String {
    match error {
        DataError::UnsupportedYear(_) => format!(
            "embedded tax parameters incomplete for tax_year {}: tax/{} is not available",
            tax_year, entry_key
        ),
        other => format!(
            "failed to load embedded tax parameters for tax_year {} from tax/{}: {}",
            tax_year, entry_key, other
        ),
    }
}

fn normalize_rmd_input(input: &Value) -> Result<Value, String> {
    if input.get("rmd_parameters").is_some() {
        return Ok(input.clone());
    }

    let calculation_year = input["calculation_year"]
        .as_u64()
        .ok_or_else(|| "missing required field: calculation_year".to_string())?
        as u32;
    let current_exe = std::env::current_exe().ok();
    let home_dir = std::env::var_os("HOME").map(std::path::PathBuf::from);
    let reference_root =
        resolve_compute_reference_root(current_exe.as_deref(), home_dir.as_deref());
    let bundle = load_rmd_reference_bundle(&reference_root.path, calculation_year)?;
    let rmd_parameters = rmd_parameters_to_value(&bundle.distribution_rules);

    let mut normalized = input.clone();
    match &mut normalized {
        Value::Object(map) => {
            map.insert("rmd_parameters".to_string(), rmd_parameters);
            Ok(normalized)
        }
        _ => Err("invalid RMD input: expected a JSON object".to_string()),
    }
}

fn rmd_parameters_to_value(
    params: &entropyfa_engine::models::retirement_rmd::RmdParameters,
) -> Value {
    json!({
        "uniform_lifetime_table": params.uniform_lifetime_table.iter().map(|row| json!({
            "age": row.age,
            "distribution_period": row.distribution_period,
        })).collect::<Vec<_>>(),
        "joint_life_table": params.joint_life_table.iter().map(|row| json!({
            "owner_age": row.owner_age,
            "spouse_age": row.spouse_age,
            "distribution_period": row.distribution_period,
        })).collect::<Vec<_>>(),
        "single_life_table": params.single_life_table.iter().map(|row| json!({
            "age": row.age,
            "distribution_period": row.distribution_period,
        })).collect::<Vec<_>>(),
        "required_beginning": {
            "start_age_rules": params.required_beginning.start_age_rules.iter().map(|rule| json!({
                "birth_year_min": rule.birth_year_min,
                "birth_year_max": rule.birth_year_max,
                "start_age": rule.start_age,
                "guidance_status": rule.guidance_status,
                "notes": rule.notes,
            })).collect::<Vec<_>>(),
            "first_distribution_deadline": params.required_beginning.first_distribution_deadline.clone(),
            "still_working_exception_plan_categories": params.required_beginning.still_working_exception_plan_categories.clone(),
            "still_working_exception_eligible_account_types": params.required_beginning.still_working_exception_eligible_account_types.clone(),
            "still_working_exception_disallowed_for_five_percent_owners": params.required_beginning.still_working_exception_disallowed_for_five_percent_owners,
        },
        "account_rules": {
            "owner_required_account_types": params.account_rules.owner_required_account_types.clone(),
            "owner_exempt_account_types": params.account_rules.owner_exempt_account_types.clone(),
            "inherited_account_types": params.account_rules.inherited_account_types.clone(),
            "supports_pre_1987_403b_exclusion": params.account_rules.supports_pre_1987_403b_exclusion,
            "designated_roth_owner_exemption_effective_year": params.account_rules.designated_roth_owner_exemption_effective_year,
        },
        "beneficiary_rules": {
            "beneficiary_categories": params.beneficiary_rules.beneficiary_categories.clone(),
            "recognized_beneficiary_classes": params.beneficiary_rules.recognized_beneficiary_classes.clone(),
            "eligible_designated_beneficiary_classes": params.beneficiary_rules.eligible_designated_beneficiary_classes.clone(),
            "life_expectancy_method_by_class": params.beneficiary_rules.life_expectancy_method_by_class.clone(),
            "minor_child_majority_age": params.beneficiary_rules.minor_child_majority_age,
            "spouse_delay_allowed": params.beneficiary_rules.spouse_delay_allowed,
            "non_designated_beneficiary_rules": {
                "when_owner_died_before_required_beginning_date": params.beneficiary_rules.non_designated_beneficiary_rules.when_owner_died_before_required_beginning_date.clone(),
                "when_owner_died_on_or_after_required_beginning_date": params.beneficiary_rules.non_designated_beneficiary_rules.when_owner_died_on_or_after_required_beginning_date.clone(),
            },
        },
        "ten_year_rule": {
            "terminal_year": params.ten_year_rule.terminal_year,
            "annual_distributions_required_when_owner_died_on_or_after_rbd": params.ten_year_rule.annual_distributions_required_when_owner_died_on_or_after_rbd,
        },
        "relief_years": params.relief_years.clone(),
        "pre_1987_403b_rules": {
            "exclude_until_age": params.pre_1987_403b_rules.exclude_until_age,
        },
    })
}
