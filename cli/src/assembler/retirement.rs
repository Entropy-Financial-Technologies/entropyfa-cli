use entropyfa_engine::data::tax::federal;
use entropyfa_engine::models::retirement_rmd::{
    RetirementRmdRequest, RetirementRmdScheduleRequest,
};
use entropyfa_engine::models::roth_conversion::{
    RothConversionRequest, RothConversionStrategyRequest,
};
use entropyfa_engine::models::tax_request::{
    Adjustments, DeductionConfig, IncomeBreakdown, TaxParameters,
};
use serde_json::Value;

use super::parse_filing_status;

/// Assemble an RMD request. The RMD tables and rules must be provided in the
/// input JSON under `rmd_parameters` since the data module doesn't yet expose
/// them as typed functions. The CLI's schema tells the agent what to provide.
pub fn assemble_rmd(input: &Value) -> Result<RetirementRmdRequest, String> {
    serde_json::from_value(input.clone()).map_err(|e| format!("invalid RMD input: {e}"))
}

/// Assemble an RMD schedule request (same approach as single-year RMD).
pub fn assemble_rmd_schedule(input: &Value) -> Result<RetirementRmdScheduleRequest, String> {
    serde_json::from_value(input.clone()).map_err(|e| format!("invalid RMD schedule input: {e}"))
}

/// Assemble a Roth conversion request from user JSON + embedded tax data.
pub fn assemble_roth(input: &Value) -> Result<RothConversionRequest, String> {
    let fs = parse_filing_status(input)?;

    let tax_params = build_tax_params(fs);

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
        }
    };

    let traditional_ira_balance = input["traditional_ira_balance"]
        .as_f64()
        .ok_or("missing required field: traditional_ira_balance")?;

    let tax_year = input["tax_year"].as_u64().unwrap_or(2026) as u32;

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

    let tax_params = build_tax_params(fs);

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

    let tax_year = input["tax_year"].as_u64().unwrap_or(2026) as u32;
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
fn build_tax_params(fs: entropyfa_engine::data::types::FilingStatus) -> TaxParameters {
    TaxParameters {
        ordinary_brackets: federal::brackets(fs),
        capital_gains_brackets: federal::capital_gains_brackets(fs),
        standard_deduction: federal::standard_deductions(fs),
        capital_loss_limit: federal::capital_loss_limit(fs),
        niit: federal::niit(fs),
        payroll: federal::payroll(fs),
    }
}
