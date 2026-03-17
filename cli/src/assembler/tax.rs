use entropyfa_engine::data::tax::{estate, federal};
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

    let brackets = federal::brackets(fs);
    let cg_brackets = federal::capital_gains_brackets(fs);
    let std_ded = federal::standard_deductions(fs);
    let niit = federal::niit(fs);
    let payroll = federal::payroll(fs);
    let cap_loss = federal::capital_loss_limit(fs);
    let salt = federal::salt_deduction_parameters(fs);

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
        income,
        adjustments,
        deductions,
        tax_parameters: TaxParameters {
            ordinary_brackets: brackets,
            capital_gains_brackets: cg_brackets,
            standard_deduction: std_ded,
            capital_loss_limit: cap_loss,
            niit,
            payroll,
            salt: Some(salt),
        },
    })
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
