use entropyfa_engine::data::tax::federal;
use entropyfa_engine::data::types::{DataError, FilingStatus};
use entropyfa_engine::models::pension_request::PensionComparisonRequest;
use serde_json::Value;

use super::parse_filing_status;

/// Assemble a PensionComparisonRequest from user JSON + embedded tax data.
///
/// Mortality tables, IRMAA brackets, and SS taxation params must be provided
/// in the input JSON since the data module doesn't yet expose typed functions
/// for these. The CLI schema documents the expected shape.
pub fn assemble_pension(input: &Value) -> Result<PensionComparisonRequest, String> {
    let fs = parse_filing_status(input)?;
    let tax_year = input["tax_year"].as_u64().unwrap_or(2026) as u32;

    // Build tax_parameters JSON from embedded data
    let tax_params_json = build_tax_params_json(fs, tax_year)?;

    // Overlay into input
    let mut input_clone = input.clone();
    if let Value::Object(ref mut map) = input_clone {
        map.insert("tax_parameters".to_string(), tax_params_json);
        map.insert("filing_status".to_string(), Value::String(fs.to_string()));
        map.insert("tax_year".to_string(), Value::from(tax_year));
    }

    serde_json::from_value(input_clone).map_err(|e| format!("invalid pension input: {e}"))
}

/// Serialize tax params to JSON manually since TaxParameters doesn't derive Serialize.
fn build_tax_params_json(fs: FilingStatus, tax_year: u32) -> Result<Value, String> {
    let brackets = federal::brackets_for_year(tax_year, fs)
        .map_err(|err| format_tax_data_error("federal_income_tax_brackets", tax_year, err))?;
    let cg_brackets = federal::capital_gains_brackets_for_year(tax_year, fs).map_err(|err| {
        format_tax_data_error("federal_capital_gains_brackets", tax_year, err)
    })?;
    let std_ded = federal::standard_deductions_for_year(tax_year, fs)
        .map_err(|err| format_tax_data_error("federal_standard_deductions", tax_year, err))?;
    let niit = federal::niit_for_year(tax_year, fs)
        .map_err(|err| format_tax_data_error("federal_net_investment_income_tax", tax_year, err))?;
    let payroll = federal::payroll_for_year(tax_year, fs)
        .map_err(|err| format_tax_data_error("federal_payroll_tax_parameters", tax_year, err))?;
    let cap_loss = federal::capital_loss_limit_for_year(tax_year, fs)
        .map_err(|err| format_tax_data_error("federal_capital_loss_limit", tax_year, err))?;
    let salt = federal::salt_deduction_parameters_for_year(tax_year, fs).map_err(|err| {
        format_tax_data_error("federal_salt_deduction_parameters", tax_year, err)
    })?;

    Ok(serde_json::json!({
        "ordinary_brackets": brackets.iter().map(|b| serde_json::json!({
            "min": b.min,
            "max": b.max,
            "rate": b.rate,
        })).collect::<Vec<_>>(),
        "capital_gains_brackets": cg_brackets.iter().map(|b| serde_json::json!({
            "min": b.min,
            "max": b.max,
            "rate": b.rate,
        })).collect::<Vec<_>>(),
        "standard_deduction": std_ded,
        "capital_loss_limit": cap_loss,
        "niit": {
            "rate": niit.rate,
            "threshold": niit.threshold,
        },
        "payroll": {
            "social_security_rate": payroll.social_security_rate,
            "social_security_wage_base": payroll.social_security_wage_base,
            "self_employment_tax_rate": payroll.self_employment_tax_rate,
            "medicare_rate": payroll.medicare_rate,
            "self_employment_medicare_rate": payroll.self_employment_medicare_rate,
            "additional_medicare_rate": payroll.additional_medicare_rate,
            "additional_medicare_threshold": payroll.additional_medicare_threshold,
        },
        "salt": {
            "cap_amount": salt.cap_amount,
            "phaseout_threshold": salt.phaseout_threshold,
            "phaseout_rate": salt.phaseout_rate,
            "floor_amount": salt.floor_amount,
        },
    }))
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
