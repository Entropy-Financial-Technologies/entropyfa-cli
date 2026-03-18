use entropyfa_engine::data::tax::{estate, federal};
use entropyfa_engine::data::types::{DataError, FilingStatus};
use entropyfa_engine::models::estate_tax_request::{
    EstateDeductions, EstateTaxParameters, EstateTaxRequest,
};
use entropyfa_engine::models::tax_request::{
    Adjustments, DeductionConfig, FederalTaxRequest, IncomeBreakdown, TaxParameters,
};
use serde_json::Value;

use super::parse_filing_status;

/// Assemble a FederalTaxRequest from minimal user JSON + embedded reference data.
pub fn assemble_federal_tax(input: &Value) -> Result<FederalTaxRequest, String> {
    let fs = parse_filing_status(input)?;
    let tax_year = input["tax_year"].as_u64().unwrap_or(2026) as u32;

    let tax_parameters = build_tax_params(fs, tax_year)?;

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

    let deductions: DeductionConfig = if let Some(ded_val) = input.get("deductions") {
        serde_json::from_value(ded_val.clone()).map_err(|e| format!("invalid deductions: {e}"))?
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

    Ok(FederalTaxRequest {
        filing_status: fs.to_string(),
        tax_year,
        income,
        adjustments,
        deductions,
        tax_parameters,
    })
}

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

/// Assemble an EstateTaxRequest from minimal user JSON + embedded reference data.
pub fn assemble_estate_tax(input: &Value) -> Result<EstateTaxRequest, String> {
    let gross_estate = input["gross_estate"]
        .as_f64()
        .ok_or("missing required field: gross_estate")?;

    let deductions: EstateDeductions = serde_json::from_value(
        input
            .get("deductions")
            .cloned()
            .unwrap_or(Value::Object(Default::default())),
    )
    .map_err(|e| format!("invalid deductions: {e}"))?;

    let adjusted_taxable_gifts = input["adjusted_taxable_gifts"].as_f64().unwrap_or(0.0);
    let gift_tax_paid = input["gift_tax_paid"].as_f64().unwrap_or(0.0);
    let dsue = input["deceased_spouse_unused_exclusion"]
        .as_f64()
        .unwrap_or(0.0);

    Ok(EstateTaxRequest {
        gross_estate,
        deductions,
        adjusted_taxable_gifts,
        gift_tax_paid,
        deceased_spouse_unused_exclusion: dsue,
        estate_tax_parameters: EstateTaxParameters {
            exemption_amount: estate::exemption(),
            applicable_credit_amount: estate::applicable_credit(),
            brackets: estate::brackets(),
        },
    })
}
