use entropyfa_engine::data::tax::federal;
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

    // Build tax_parameters JSON from embedded data
    let tax_params_json = build_tax_params_json(fs);

    // Overlay into input
    let mut input_clone = input.clone();
    if let Value::Object(ref mut map) = input_clone {
        map.insert("tax_parameters".to_string(), tax_params_json);
        map.insert("filing_status".to_string(), Value::String(fs.to_string()));
    }

    serde_json::from_value(input_clone).map_err(|e| format!("invalid pension input: {e}"))
}

/// Serialize tax params to JSON manually since TaxParameters doesn't derive Serialize.
fn build_tax_params_json(fs: entropyfa_engine::data::types::FilingStatus) -> Value {
    let brackets = federal::brackets(fs);
    let cg_brackets = federal::capital_gains_brackets(fs);
    let std_ded = federal::standard_deductions(fs);
    let niit = federal::niit(fs);
    let payroll = federal::payroll(fs);
    let cap_loss = federal::capital_loss_limit(fs);

    serde_json::json!({
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
    })
}
