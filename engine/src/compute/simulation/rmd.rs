use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::data::retirement::rmd_rules::distribution_rules;
use crate::models::retirement_rmd::{AgeDistributionPeriod, RequiredBeginningRules, RmdParameters};

use super::buckets::HouseholdBucketState;
use super::normalized::{BucketType, NormalizedSimulationRequest};

static RMD_RULES: OnceLock<RmdParameters> = OnceLock::new();

const DEFAULT_RMD_DISTRIBUTION_MONTH: u32 = 12;

#[derive(Debug, Clone, Default)]
pub struct AnnualRmdResult {
    pub total_rmd: f64,
    pub withdrawals_by_bucket: BTreeMap<String, f64>,
}

pub fn compute_household_rmd_for_year(
    req: &NormalizedSimulationRequest,
    tax_year: u32,
    state: &HouseholdBucketState,
) -> Result<AnnualRmdResult, String> {
    if configured_distribution_month(req).is_none() {
        return Ok(AnnualRmdResult::default());
    }

    let Some(owner_birth_year) = household_rmd_birth_year(req) else {
        return Ok(AnnualRmdResult::default());
    };
    if tax_year < owner_birth_year {
        return Ok(AnnualRmdResult::default());
    }

    let rules = rmd_rules();
    let owner_age = tax_year - owner_birth_year;
    let start_age =
        find_start_age(owner_birth_year, &rules.required_beginning).ok_or_else(|| {
            format!(
                "no RMD start age rule matched birth year {}",
                owner_birth_year
            )
        })?;
    if owner_age < start_age {
        return Ok(AnnualRmdResult::default());
    }

    let distribution_period = age_distribution_period(&rules.uniform_lifetime_table, owner_age)
        .ok_or_else(|| {
            format!("no uniform lifetime distribution period found for age={owner_age}")
        })?;

    let mut result = AnnualRmdResult::default();
    for bucket in state
        .buckets
        .iter()
        .filter(|bucket| bucket.bucket_type == BucketType::TaxDeferred && bucket.balance > 0.0)
    {
        let rmd_amount = round2((bucket.balance / distribution_period).min(bucket.balance));
        if rmd_amount <= 0.0 {
            continue;
        }

        result.total_rmd += rmd_amount;
        result
            .withdrawals_by_bucket
            .insert(bucket.id.clone(), rmd_amount);
    }

    result.total_rmd = round2(result.total_rmd);
    Ok(result)
}

pub fn configured_distribution_month(req: &NormalizedSimulationRequest) -> Option<u32> {
    req.rmd_policy
        .as_ref()
        .filter(|policy| policy.enabled)
        .map(|policy| {
            policy
                .distribution_month
                .unwrap_or(DEFAULT_RMD_DISTRIBUTION_MONTH)
        })
}

pub fn apply_rmd_withdrawals(
    state: &mut HouseholdBucketState,
    schedule: &AnnualRmdResult,
) -> BTreeMap<String, f64> {
    let mut applied = BTreeMap::new();

    for (bucket_id, scheduled_amount) in &schedule.withdrawals_by_bucket {
        let withdrawn = apply_bucket_rmd(state, bucket_id, *scheduled_amount);
        if withdrawn <= 0.0 {
            continue;
        }

        state.household_cash += withdrawn;
        applied.insert(bucket_id.clone(), round2(withdrawn));
    }

    applied
}

fn apply_bucket_rmd(
    state: &mut HouseholdBucketState,
    bucket_id: &str,
    scheduled_amount: f64,
) -> f64 {
    let Some(bucket) = state.bucket_mut(bucket_id) else {
        return 0.0;
    };

    let withdrawn = bucket.balance.min(scheduled_amount.max(0.0));
    bucket.balance -= withdrawn;
    withdrawn
}

fn household_rmd_birth_year(req: &NormalizedSimulationRequest) -> Option<u32> {
    req.household
        .as_ref()?
        .birth_years
        .as_ref()?
        .iter()
        .copied()
        .min()
}

fn rmd_rules() -> &'static RmdParameters {
    RMD_RULES.get_or_init(distribution_rules)
}

fn find_start_age(birth_year: u32, rules: &RequiredBeginningRules) -> Option<u32> {
    let mut matched = None;

    for rule in &rules.start_age_rules {
        let min_ok = rule.birth_year_min.is_none_or(|min| birth_year >= min);
        let max_ok = rule.birth_year_max.is_none_or(|max| birth_year <= max);
        if min_ok && max_ok {
            matched = Some(rule.start_age);
        }
    }

    matched
}

fn age_distribution_period(table: &[AgeDistributionPeriod], age: u32) -> Option<f64> {
    table
        .iter()
        .find(|row| row.age == age)
        .map(|row| row.distribution_period)
}

fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::compute_household_rmd_for_year;
    use crate::compute::simulation::buckets::{BucketState, HouseholdBucketState};
    use crate::compute::simulation::linear::run_linear;
    use crate::compute::simulation::normalized::{normalize_request, BucketType};
    use crate::models::simulation_request::{
        CashFlow, HouseholdConfig, ReturnAssumption, RmdPolicy, SimulationBucket,
        SimulationRequest, SpendingPolicy,
    };

    fn rmd_enabled_bucketed_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: None,
            buckets: vec![
                SimulationBucket {
                    id: "taxable".into(),
                    bucket_type: "taxable".into(),
                    starting_balance: 10_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.0,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: Some(0.0),
                    withdrawal_priority: Some(1),
                },
                SimulationBucket {
                    id: "ira".into(),
                    bucket_type: "tax_deferred".into(),
                    starting_balance: 100_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.0,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(2),
                },
            ],
            time_horizon_months: 12,
            return_assumption: None,
            cash_flows: vec![CashFlow {
                amount: -1_000.0,
                frequency: "one_time".into(),
                start_month: Some(0),
                end_month: None,
            }],
            filing_status: Some("single".into()),
            household: Some(HouseholdConfig {
                birth_years: Some(vec![1950]),
                retirement_month: Some(1),
            }),
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: vec!["taxable".into(), "ira".into()],
                rebalance_tax_withholding_from: None,
            }),
            tax_policy: None,
            rmd_policy: Some(RmdPolicy {
                enabled: true,
                distribution_month: Some(1),
            }),
            include_detail: true,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    fn bucket_state_from_request(req: &SimulationRequest) -> HouseholdBucketState {
        HouseholdBucketState {
            buckets: req
                .buckets
                .iter()
                .map(|bucket| BucketState {
                    id: bucket.id.clone(),
                    bucket_type: match bucket.bucket_type.as_str() {
                        "taxable" => BucketType::Taxable,
                        "tax_free" => BucketType::TaxFree,
                        "cash" => BucketType::Cash,
                        _ => BucketType::TaxDeferred,
                    },
                    balance: bucket.starting_balance,
                    realized_gain_ratio: bucket.realized_gain_ratio,
                })
                .collect(),
            household_cash: 0.0,
        }
    }

    #[test]
    fn test_rmd_is_generated_for_tax_deferred_bucket_when_enabled() {
        let req = rmd_enabled_bucketed_request();
        let normalized = normalize_request(&req).expect("request should normalize");
        let state = bucket_state_from_request(&req);

        let schedule =
            compute_household_rmd_for_year(&normalized, 2026, &state).expect("RMD should compute");

        assert!(schedule.total_rmd > 0.0);
    }

    #[test]
    fn test_rmd_forces_tax_deferred_withdrawal_before_normal_spending_order() {
        let req = rmd_enabled_bucketed_request();

        let result = run_linear(&req);
        let detail = result.annual_detail.expect("detail should exist");

        assert!(
            detail[0]
                .bucket_withdrawals
                .get("ira")
                .copied()
                .unwrap_or(0.0)
                > 0.0
        );
        assert_eq!(
            detail[0]
                .bucket_withdrawals
                .get("taxable")
                .copied()
                .unwrap_or(0.0),
            0.0
        );
    }
}
