use std::collections::BTreeMap;

use crate::compute::tax::federal::run_federal_tax;
use crate::data::tax::federal::{
    brackets_for_year, capital_gains_brackets_for_year, capital_loss_limit_for_year, niit_for_year,
    payroll_for_year, salt_deduction_parameters_for_year, standard_deductions_for_year,
};
use crate::data::types::FilingStatus;
use crate::models::tax_request::{
    DeductionConfig, FederalTaxRequest, IncomeBreakdown, TaxParameters,
};

use super::buckets::{BucketState, HouseholdBucketState};
use super::normalized::BucketType;
use super::withdrawals::fund_household_deficit;

pub const DEFAULT_SIMULATION_TAX_YEAR: u32 = 2026;
const AUTHORITATIVE_TAX_YEAR: u32 = 2026;
const MAX_TAX_SETTLEMENT_ITERATIONS: usize = 32;

#[derive(Default, Debug, Clone)]
pub struct AnnualTaxAccumulator {
    pub ordinary_income: f64,
    pub realized_capital_gain: f64,
}

impl AnnualTaxAccumulator {
    pub fn record_tax_deferred_withdrawal(&mut self, _bucket_id: &str, amount: f64) {
        self.ordinary_income += amount.max(0.0);
    }

    pub fn record_tax_free_withdrawal(&mut self, _bucket_id: &str, _amount: f64) {}

    pub fn record_taxable_withdrawal(
        &mut self,
        _bucket_id: &str,
        amount: f64,
        realized_gain_ratio: f64,
    ) {
        self.realized_capital_gain += amount.max(0.0) * realized_gain_ratio.clamp(0.0, 1.0);
    }
}

#[derive(Debug, Clone)]
pub struct AnnualTaxResult {
    pub total_tax: f64,
    pub taxable_ordinary_income: f64,
    pub realized_capital_gain: f64,
    pub data_mode: String,
    pub authoritative_year: u32,
}

#[derive(Debug, Clone)]
pub struct AnnualTaxSettlement {
    pub annual_tax_paid: f64,
    pub tax_result: AnnualTaxResult,
    pub withdrawals_by_bucket: BTreeMap<String, f64>,
}

pub fn compute_annual_household_tax(
    agg: &AnnualTaxAccumulator,
    filing_status: &str,
    tax_year: u32,
    modeled_tax_inflation_rate: f64,
) -> Result<AnnualTaxResult, String> {
    let filing_status = FilingStatus::parse(filing_status).map_err(|err| err.to_string())?;
    let (tax_parameters, data_mode) =
        tax_parameters_for_year(filing_status, tax_year, modeled_tax_inflation_rate)?;

    let request = FederalTaxRequest {
        filing_status: filing_status.to_string(),
        tax_year,
        income: IncomeBreakdown {
            taxable_ira_distributions: agg.ordinary_income,
            long_term_capital_gains: agg.realized_capital_gain,
            ..IncomeBreakdown::default()
        },
        adjustments: Default::default(),
        deductions: DeductionConfig {
            method: "standard".into(),
            itemized_amount: None,
            spouse_itemizes: None,
            state_local_income_or_sales_tax: None,
            real_property_tax: None,
            personal_property_tax: None,
            other_itemized_deductions: None,
        },
        tax_parameters,
    };

    let response = run_federal_tax(&request);

    Ok(AnnualTaxResult {
        total_tax: response.total_tax,
        taxable_ordinary_income: response.ordinary_taxable,
        realized_capital_gain: round2(agg.realized_capital_gain),
        data_mode,
        authoritative_year: AUTHORITATIVE_TAX_YEAR,
    })
}

pub fn record_bucket_withdrawals_for_tax(
    agg: &mut AnnualTaxAccumulator,
    state: &HouseholdBucketState,
    withdrawals_by_bucket: &BTreeMap<String, f64>,
) {
    for (bucket_id, amount) in withdrawals_by_bucket {
        if let Some(bucket) = state.buckets.iter().find(|bucket| bucket.id == *bucket_id) {
            record_bucket_withdrawal_for_tax(agg, bucket, *amount);
        }
    }
}

pub fn record_bucket_withdrawal_for_tax(
    agg: &mut AnnualTaxAccumulator,
    bucket: &BucketState,
    amount: f64,
) {
    match bucket.bucket_type {
        BucketType::TaxDeferred => agg.record_tax_deferred_withdrawal(&bucket.id, amount),
        BucketType::TaxFree => agg.record_tax_free_withdrawal(&bucket.id, amount),
        BucketType::Taxable => agg.record_taxable_withdrawal(
            &bucket.id,
            amount,
            bucket.realized_gain_ratio.unwrap_or(0.0),
        ),
        BucketType::Cash => {}
    }
}

pub fn settle_annual_tax(
    state: &mut HouseholdBucketState,
    agg: &AnnualTaxAccumulator,
    filing_status: &str,
    tax_year: u32,
    modeled_tax_inflation_rate: f64,
    withholding_bucket_id: Option<&str>,
    withdrawal_order: &[String],
) -> Result<AnnualTaxSettlement, String> {
    let mut working_agg = agg.clone();
    let mut total_tax_paid = 0.0;
    let mut withdrawals_by_bucket = BTreeMap::new();
    let mut tax_result = compute_annual_household_tax(
        &working_agg,
        filing_status,
        tax_year,
        modeled_tax_inflation_rate,
    )?;

    for _ in 0..MAX_TAX_SETTLEMENT_ITERATIONS {
        let remaining_due = round2((tax_result.total_tax - total_tax_paid).max(0.0));
        if remaining_due <= 0.0 {
            return Ok(AnnualTaxSettlement {
                annual_tax_paid: total_tax_paid,
                tax_result,
                withdrawals_by_bucket,
            });
        }

        let payment = withdraw_amount_for_tax_payment(
            state,
            remaining_due,
            withholding_bucket_id,
            withdrawal_order,
        );
        if payment.amount_paid <= 0.0 {
            return Ok(AnnualTaxSettlement {
                annual_tax_paid: total_tax_paid,
                tax_result,
                withdrawals_by_bucket,
            });
        }

        total_tax_paid = round2(total_tax_paid + payment.amount_paid);
        merge_withdrawal_maps(&mut withdrawals_by_bucket, &payment.withdrawals_by_bucket);
        record_bucket_withdrawals_for_tax(&mut working_agg, state, &payment.withdrawals_by_bucket);
        tax_result = compute_annual_household_tax(
            &working_agg,
            filing_status,
            tax_year,
            modeled_tax_inflation_rate,
        )?;
    }

    Err(format!(
        "annual tax settlement did not converge within {} iterations",
        MAX_TAX_SETTLEMENT_ITERATIONS
    ))
}

#[derive(Debug, Clone, Default)]
struct TaxPaymentWithdrawal {
    amount_paid: f64,
    withdrawals_by_bucket: BTreeMap<String, f64>,
}

fn withdraw_amount_for_tax_payment(
    state: &mut HouseholdBucketState,
    amount_needed: f64,
    withholding_bucket_id: Option<&str>,
    withdrawal_order: &[String],
) -> TaxPaymentWithdrawal {
    let mut remaining = amount_needed.max(0.0);
    let mut withdrawals_by_bucket = BTreeMap::new();

    if remaining <= 0.0 {
        return TaxPaymentWithdrawal::default();
    }

    if let Some(bucket_id) = withholding_bucket_id {
        if let Some(bucket) = state.bucket_mut(bucket_id) {
            let paid = bucket.balance.min(remaining);
            if paid > 0.0 {
                bucket.balance -= paid;
                remaining -= paid;
                *withdrawals_by_bucket
                    .entry(bucket_id.to_string())
                    .or_insert(0.0) += paid;
            }
        }
    }

    if remaining > 0.0 {
        let paid_from_cash = state.household_cash.min(remaining);
        state.household_cash -= paid_from_cash;
        remaining -= paid_from_cash;
    }

    if remaining > 0.0 {
        let withdrawal = fund_household_deficit(state, remaining, withdrawal_order);
        merge_withdrawal_maps(
            &mut withdrawals_by_bucket,
            &withdrawal.withdrawals_by_bucket,
        );
        remaining = withdrawal.remaining_shortfall;
    }

    TaxPaymentWithdrawal {
        amount_paid: round2(amount_needed - remaining),
        withdrawals_by_bucket,
    }
}

fn merge_withdrawal_maps(target: &mut BTreeMap<String, f64>, source: &BTreeMap<String, f64>) {
    for (bucket_id, amount) in source {
        *target.entry(bucket_id.clone()).or_insert(0.0) += amount;
    }
}

fn tax_parameters_for_year(
    filing_status: FilingStatus,
    tax_year: u32,
    modeled_tax_inflation_rate: f64,
) -> Result<(TaxParameters, String), String> {
    if tax_year < AUTHORITATIVE_TAX_YEAR {
        return Err(format!(
            "simulation tax support starts at {} (got {})",
            AUTHORITATIVE_TAX_YEAR, tax_year
        ));
    }

    if tax_year == AUTHORITATIVE_TAX_YEAR {
        let params = authoritative_tax_parameters(filing_status, AUTHORITATIVE_TAX_YEAR)?;
        return Ok((params, "authoritative".into()));
    }

    let years_forward = tax_year - AUTHORITATIVE_TAX_YEAR;
    let inflation_factor = (1.0 + modeled_tax_inflation_rate.max(0.0)).powi(years_forward as i32);
    let mut params = authoritative_tax_parameters(filing_status, AUTHORITATIVE_TAX_YEAR)?;

    inflate_brackets(&mut params.ordinary_brackets, inflation_factor);
    inflate_brackets(&mut params.capital_gains_brackets, inflation_factor);
    params.standard_deduction *= inflation_factor;
    params.capital_loss_limit *= inflation_factor;
    params.niit.threshold *= inflation_factor;
    params.payroll.social_security_wage_base *= inflation_factor;
    params.payroll.additional_medicare_threshold *= inflation_factor;
    if let Some(salt) = params.salt.as_mut() {
        salt.cap_amount *= inflation_factor;
        salt.phaseout_threshold *= inflation_factor;
        salt.floor_amount *= inflation_factor;
    }

    Ok((params, "modeled".into()))
}

fn authoritative_tax_parameters(
    filing_status: FilingStatus,
    tax_year: u32,
) -> Result<TaxParameters, String> {
    Ok(TaxParameters {
        ordinary_brackets: brackets_for_year(tax_year, filing_status)
            .map_err(|err| err.to_string())?,
        capital_gains_brackets: capital_gains_brackets_for_year(tax_year, filing_status)
            .map_err(|err| err.to_string())?,
        standard_deduction: standard_deductions_for_year(tax_year, filing_status)
            .map_err(|err| err.to_string())?,
        capital_loss_limit: capital_loss_limit_for_year(tax_year, filing_status)
            .map_err(|err| err.to_string())?,
        niit: niit_for_year(tax_year, filing_status).map_err(|err| err.to_string())?,
        payroll: payroll_for_year(tax_year, filing_status).map_err(|err| err.to_string())?,
        salt: Some(
            salt_deduction_parameters_for_year(tax_year, filing_status)
                .map_err(|err| err.to_string())?,
        ),
    })
}

fn inflate_brackets(brackets: &mut [crate::models::tax_request::TaxBracket], factor: f64) {
    for bracket in brackets {
        bracket.min *= factor;
        if let Some(max) = bracket.max.as_mut() {
            *max *= factor;
        }
    }
}

fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute::simulation::normalized::BucketType;

    #[test]
    fn test_tax_deferred_withdrawals_are_taxed_as_ordinary_income() {
        let mut agg = AnnualTaxAccumulator::default();
        agg.record_tax_deferred_withdrawal("ira", 40_000.0);
        let result = compute_annual_household_tax(&agg, "single", 2026, 0.025).unwrap();
        assert!(result.total_tax > 0.0);
    }

    #[test]
    fn test_tax_free_withdrawals_do_not_increase_taxable_income() {
        let mut agg = AnnualTaxAccumulator::default();
        agg.record_tax_free_withdrawal("roth", 30_000.0);
        let result = compute_annual_household_tax(&agg, "single", 2026, 0.025).unwrap();
        assert_eq!(result.taxable_ordinary_income, 0.0);
    }

    #[test]
    fn test_taxable_withdrawals_use_realized_gain_ratio() {
        let mut agg = AnnualTaxAccumulator::default();
        agg.record_taxable_withdrawal("taxable", 10_000.0, 0.60);
        let result = compute_annual_household_tax(&agg, "single", 2026, 0.025).unwrap();
        assert!(result.realized_capital_gain >= 5_999.0);
    }

    #[test]
    fn test_tax_settlement_recomputes_for_tax_deferred_payment_source() {
        let mut agg = AnnualTaxAccumulator::default();
        agg.record_tax_deferred_withdrawal("ira", 40_000.0);
        let initial_tax = compute_annual_household_tax(&agg, "single", 2026, 0.025).unwrap();
        let mut state = HouseholdBucketState {
            buckets: vec![BucketState {
                id: "ira".into(),
                bucket_type: BucketType::TaxDeferred,
                balance: 10_000.0,
                realized_gain_ratio: None,
            }],
            household_cash: 0.0,
        };

        let settlement = settle_annual_tax(
            &mut state,
            &agg,
            "single",
            2026,
            0.025,
            Some("ira"),
            &["ira".to_string()],
        )
        .unwrap();

        assert!(settlement.annual_tax_paid > initial_tax.total_tax);
        assert!(settlement.withdrawals_by_bucket["ira"] > initial_tax.total_tax);
        assert_eq!(settlement.annual_tax_paid, settlement.tax_result.total_tax);
    }
}
