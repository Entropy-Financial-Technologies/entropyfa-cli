use std::collections::BTreeMap;

use crate::models::simulation_request::SimulationRequest;
use crate::models::simulation_response::{LinearResult, LinearTimeSeries, PeriodDetail};

use super::buckets::{apply_monthly_returns, BucketState, HouseholdBucketState};
use super::normalized::normalize_request;
use super::path_simulator::resolve_monthly_cash_flows;
use super::tax::{
    compute_annual_household_tax, pay_annual_tax, record_bucket_withdrawals_for_tax,
    AnnualTaxAccumulator, DEFAULT_SIMULATION_TAX_YEAR,
};
use super::withdrawals::fund_household_deficit;

pub fn run_linear(req: &SimulationRequest) -> LinearResult {
    let normalized =
        normalize_request(req).expect("linear request should satisfy simulation contract");
    let total_months = normalized.time_horizon_months;
    let report_bucket_balances = !req.buckets.is_empty();

    let monthly_cash_flows = resolve_monthly_cash_flows(&req.cash_flows, total_months);
    let monthly_returns: Vec<f64> = normalized
        .buckets
        .iter()
        .map(|bucket| (1.0 + bucket.return_assumption.annual_mean).powf(1.0 / 12.0) - 1.0)
        .collect();

    let mut state = HouseholdBucketState {
        buckets: normalized
            .buckets
            .iter()
            .map(|bucket| BucketState {
                id: bucket.id.clone(),
                bucket_type: bucket.bucket_type.clone(),
                balance: bucket.starting_balance,
                realized_gain_ratio: bucket.realized_gain_ratio,
            })
            .collect(),
        household_cash: 0.0,
    };

    let mut path = Vec::with_capacity(total_months as usize + 1);
    path.push(total_balance(&state));

    let contribution_bucket_id = normalized.buckets.first().map(|bucket| bucket.id.clone());
    let mut monthly_bucket_detail = Vec::with_capacity(total_months as usize);
    let mut annual_tax_accumulator = AnnualTaxAccumulator::default();
    let tax_enabled = normalized.tax_policy.mode != "none";
    let modeled_tax_inflation_rate = normalized
        .tax_policy
        .modeled_tax_inflation_rate
        .unwrap_or(0.0);
    let filing_status = normalized.filing_status.as_deref().unwrap_or("single");

    for (month_index, monthly_cash_flow) in monthly_cash_flows.iter().enumerate() {
        apply_monthly_returns(&mut state, &monthly_returns);

        let mut bucket_withdrawals = BTreeMap::new();
        if *monthly_cash_flow > 0.0 {
            if let Some(bucket_id) = contribution_bucket_id.as_deref() {
                if let Some(bucket) = state.bucket_mut(bucket_id) {
                    bucket.balance += *monthly_cash_flow;
                }
            }
        } else if *monthly_cash_flow < 0.0 {
            let withdrawal = fund_household_deficit(
                &mut state,
                monthly_cash_flow.abs(),
                &normalized.spending_policy.withdrawal_order,
            );
            if tax_enabled {
                record_bucket_withdrawals_for_tax(
                    &mut annual_tax_accumulator,
                    &state,
                    &withdrawal.withdrawals_by_bucket,
                );
            }
            bucket_withdrawals = withdrawal.withdrawals_by_bucket;
        }

        let mut annual_tax_paid = 0.0;
        let is_year_end =
            (month_index + 1).is_multiple_of(12) || month_index + 1 == total_months as usize;
        if tax_enabled && is_year_end {
            let tax_year = DEFAULT_SIMULATION_TAX_YEAR + (month_index as u32 / 12);
            let tax_result = compute_annual_household_tax(
                &annual_tax_accumulator,
                filing_status,
                tax_year,
                modeled_tax_inflation_rate,
            )
            .expect("simulation tax request should be supported");
            annual_tax_paid = pay_annual_tax(
                &mut state,
                tax_result.total_tax,
                normalized
                    .spending_policy
                    .rebalance_tax_withholding_from
                    .as_deref(),
                &normalized.spending_policy.withdrawal_order,
            );
            annual_tax_accumulator = AnnualTaxAccumulator::default();
        }

        path.push(total_balance(&state));
        monthly_bucket_detail.push(MonthlyBucketDetail {
            bucket_withdrawals,
            ending_balances_by_bucket: balances_by_bucket(&state),
            annual_tax_paid,
        });
    }

    // Compute contributions and withdrawals
    let total_contributions: f64 = monthly_cash_flows.iter().filter(|&&v| v > 0.0).sum();
    let total_withdrawals: f64 = monthly_cash_flows
        .iter()
        .filter(|&&v| v < 0.0)
        .sum::<f64>()
        .abs();

    let starting_balance = path[0];
    let final_balance = *path.last().unwrap();
    let net_cash_flow: f64 = monthly_cash_flows.iter().sum();
    let total_return_earned = final_balance - starting_balance - net_cash_flow;

    // Sample time-series at annual intervals
    let mut months = Vec::new();
    let mut balances = Vec::new();

    let mut sample_months: Vec<u32> = (0..=total_months).step_by(12).collect();
    if *sample_months.last().unwrap_or(&0) != total_months {
        sample_months.push(total_months);
    }

    for &month in &sample_months {
        months.push(month);
        balances.push(round2(path[month as usize]));
    }

    let annual_detail = if req.include_detail {
        Some(compute_period_detail(
            &path,
            &monthly_cash_flows,
            &monthly_bucket_detail,
            &req.detail_granularity,
            report_bucket_balances,
        ))
    } else {
        None
    };

    LinearResult {
        final_balance: round2(final_balance),
        time_series: LinearTimeSeries {
            months,
            balance: balances,
        },
        total_contributions: round2(total_contributions),
        total_withdrawals: round2(total_withdrawals),
        total_return_earned: round2(total_return_earned.max(0.0)),
        ending_balances_by_bucket: report_bucket_balances
            .then(|| round_map(&balances_by_bucket(&state))),
        annual_detail,
    }
}

#[derive(Debug, Clone)]
struct MonthlyBucketDetail {
    bucket_withdrawals: BTreeMap<String, f64>,
    ending_balances_by_bucket: BTreeMap<String, f64>,
    annual_tax_paid: f64,
}

fn compute_period_detail(
    path: &[f64],
    monthly_cash_flows: &[f64],
    monthly_bucket_detail: &[MonthlyBucketDetail],
    granularity: &str,
    report_bucket_balances: bool,
) -> Vec<PeriodDetail> {
    let total_months = monthly_cash_flows.len();
    let step: usize = if granularity == "monthly" { 1 } else { 12 };
    let mut details = Vec::new();
    let mut cum_contributions = 0.0;
    let mut cum_withdrawals = 0.0;
    let mut cum_return = 0.0;

    let mut period_start = 0usize;
    while period_start < total_months {
        let period_end = (period_start + step).min(total_months);
        let beginning_balance = path[period_start];
        let ending_balance = path[period_end];

        let mut contributions = 0.0;
        let mut withdrawals = 0.0;
        for &cf in &monthly_cash_flows[period_start..period_end] {
            if cf > 0.0 {
                contributions += cf;
            } else {
                withdrawals += cf.abs();
            }
        }

        let net_cf = contributions - withdrawals;
        let investment_return = ending_balance - beginning_balance - net_cf;

        cum_contributions += contributions;
        cum_withdrawals += withdrawals;
        cum_return += investment_return;

        let bucket_withdrawals = if report_bucket_balances {
            let mut withdrawals_by_bucket = BTreeMap::new();
            for monthly_detail in &monthly_bucket_detail[period_start..period_end] {
                for (bucket_id, amount) in &monthly_detail.bucket_withdrawals {
                    *withdrawals_by_bucket
                        .entry(bucket_id.clone())
                        .or_insert(0.0) += amount;
                }
            }
            round_map(&withdrawals_by_bucket)
        } else {
            BTreeMap::new()
        };

        let ending_balances_by_bucket = if report_bucket_balances {
            monthly_bucket_detail[period_end - 1]
                .ending_balances_by_bucket
                .clone()
        } else {
            BTreeMap::new()
        };
        let annual_tax_paid: f64 = monthly_bucket_detail[period_start..period_end]
            .iter()
            .map(|detail| detail.annual_tax_paid)
            .sum();

        details.push(PeriodDetail {
            period: period_start as u32,
            year: period_start as f64 / 12.0,
            beginning_balance: round2(beginning_balance),
            contributions: round2(contributions),
            withdrawals: round2(withdrawals),
            investment_return: round2(investment_return),
            ending_balance: round2(ending_balance),
            cumulative_contributions: round2(cum_contributions),
            cumulative_withdrawals: round2(cum_withdrawals),
            cumulative_return: round2(cum_return),
            annual_tax_paid: round2(annual_tax_paid),
            bucket_withdrawals,
            ending_balances_by_bucket: round_map(&ending_balances_by_bucket),
        });

        period_start = period_end;
    }

    details
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn total_balance(state: &HouseholdBucketState) -> f64 {
    state
        .buckets
        .iter()
        .map(|bucket| bucket.balance)
        .sum::<f64>()
        + state.household_cash
}

fn balances_by_bucket(state: &HouseholdBucketState) -> BTreeMap<String, f64> {
    state
        .buckets
        .iter()
        .map(|bucket| (bucket.id.clone(), bucket.balance))
        .collect()
}

fn round_map(values: &BTreeMap<String, f64>) -> BTreeMap<String, f64> {
    values
        .iter()
        .map(|(key, value)| (key.clone(), round2(*value)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::simulation_request::{
        CashFlow, ReturnAssumption, SimulationBucket, SimulationRequest, SpendingPolicy, TaxPolicy,
    };
    use serde_json::Value;

    fn legacy_request(
        starting_balance: f64,
        time_horizon_months: u32,
        return_assumption: ReturnAssumption,
        cash_flows: Vec<CashFlow>,
        include_detail: bool,
        detail_granularity: &str,
    ) -> SimulationRequest {
        SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: Some(starting_balance),
            buckets: vec![],
            time_horizon_months,
            return_assumption: Some(return_assumption),
            cash_flows,
            filing_status: None,
            household: None,
            spending_policy: None,
            tax_policy: None,
            rmd_policy: None,
            include_detail,
            detail_granularity: detail_granularity.to_string(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    fn bucketed_request(
        buckets: Vec<SimulationBucket>,
        time_horizon_months: u32,
        cash_flows: Vec<CashFlow>,
        include_detail: bool,
        detail_granularity: &str,
        withdrawal_order: Vec<&str>,
    ) -> SimulationRequest {
        SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: None,
            buckets,
            time_horizon_months,
            return_assumption: None,
            cash_flows,
            filing_status: None,
            household: None,
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: withdrawal_order.into_iter().map(str::to_string).collect(),
                rebalance_tax_withholding_from: None,
            }),
            tax_policy: None,
            rmd_policy: None,
            include_detail,
            detail_granularity: detail_granularity.to_string(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    #[test]
    fn test_linear_no_cashflows() {
        let req = legacy_request(
            100_000.0,
            12,
            ReturnAssumption {
                annual_mean: 0.06,
                annual_std_dev: 0.0,
            },
            vec![],
            false,
            "annual",
        );
        let result = run_linear(&req);
        // 100,000 * (1.06)^1 ≈ 106,000
        let expected = 100_000.0 * 1.06_f64.powf(1.0);
        assert!(
            (result.final_balance - expected).abs() < 1.0,
            "expected ~{}, got {}",
            expected,
            result.final_balance
        );
        assert_eq!(result.total_contributions, 0.0);
        assert_eq!(result.total_withdrawals, 0.0);
    }

    #[test]
    fn test_linear_bucketed_projection_reports_ending_balances_by_bucket() {
        let req = bucketed_request(
            vec![
                SimulationBucket {
                    id: "taxable".into(),
                    bucket_type: "taxable".into(),
                    starting_balance: 100_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.0,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(1),
                },
                SimulationBucket {
                    id: "ira".into(),
                    bucket_type: "tax_deferred".into(),
                    starting_balance: 50_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.0,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(2),
                },
            ],
            12,
            vec![CashFlow {
                amount: -1_000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            false,
            "annual",
            vec!["taxable", "ira"],
        );

        let result = run_linear(&req);
        let json = serde_json::to_value(&result).expect("linear result should serialize");

        assert_eq!(result.final_balance, 138_000.0);
        assert_eq!(
            json["ending_balances_by_bucket"]["taxable"],
            Value::from(88_000.0)
        );
        assert_eq!(
            json["ending_balances_by_bucket"]["ira"],
            Value::from(50_000.0)
        );
    }

    #[test]
    fn test_linear_withdrawal_order_depletes_taxable_before_ira() {
        let req = bucketed_request(
            vec![
                SimulationBucket {
                    id: "taxable".into(),
                    bucket_type: "taxable".into(),
                    starting_balance: 25_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.0,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(1),
                },
                SimulationBucket {
                    id: "ira".into(),
                    bucket_type: "tax_deferred".into(),
                    starting_balance: 50_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.0,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(2),
                },
            ],
            1,
            vec![CashFlow {
                amount: -30_000.0,
                frequency: "one_time".into(),
                start_month: Some(0),
                end_month: None,
            }],
            true,
            "monthly",
            vec!["taxable", "ira"],
        );

        let result = run_linear(&req);
        let json = serde_json::to_value(&result).expect("linear result should serialize");
        let detail = &json["annual_detail"][0];

        assert_eq!(result.final_balance, 45_000.0);
        assert_eq!(
            detail["bucket_withdrawals"]["taxable"],
            Value::from(25_000.0)
        );
        assert_eq!(detail["bucket_withdrawals"]["ira"], Value::from(5_000.0));
        assert_eq!(
            detail["ending_balances_by_bucket"]["taxable"],
            Value::from(0.0)
        );
        assert_eq!(
            detail["ending_balances_by_bucket"]["ira"],
            Value::from(45_000.0)
        );
    }

    #[test]
    fn test_linear_bucketed_tax_deferred_withdrawal_reports_annual_tax_paid() {
        let req = SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: None,
            buckets: vec![SimulationBucket {
                id: "ira".into(),
                bucket_type: "tax_deferred".into(),
                starting_balance: 50_000.0,
                return_assumption: ReturnAssumption {
                    annual_mean: 0.0,
                    annual_std_dev: 0.0,
                },
                realized_gain_ratio: None,
                withdrawal_priority: Some(1),
            }],
            time_horizon_months: 12,
            return_assumption: None,
            cash_flows: vec![CashFlow {
                amount: -40_000.0,
                frequency: "one_time".into(),
                start_month: Some(0),
                end_month: None,
            }],
            filing_status: Some("single".into()),
            household: None,
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: vec!["ira".into()],
                rebalance_tax_withholding_from: Some("ira".into()),
            }),
            tax_policy: Some(TaxPolicy {
                mode: "modeled".into(),
                modeled_tax_inflation_rate: Some(0.025),
            }),
            rmd_policy: None,
            include_detail: true,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        };

        let result = run_linear(&req);
        let detail = result.annual_detail.expect("detail should be present");

        assert!(detail[0].annual_tax_paid > 0.0);
        assert!(result.final_balance < 10_000.0);
    }

    #[test]
    fn test_linear_with_contributions() {
        let req = legacy_request(
            100_000.0,
            12,
            ReturnAssumption {
                annual_mean: 0.0,
                annual_std_dev: 0.0,
            },
            vec![CashFlow {
                amount: 1000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            false,
            "annual",
        );
        let result = run_linear(&req);
        // Zero return, 1000/month for 12 months
        assert!(
            (result.final_balance - 112_000.0).abs() < 1.0,
            "expected 112000, got {}",
            result.final_balance
        );
        assert_eq!(result.total_contributions, 12_000.0);
        assert_eq!(result.total_withdrawals, 0.0);
    }

    #[test]
    fn test_linear_with_withdrawals() {
        let req = legacy_request(
            100_000.0,
            12,
            ReturnAssumption {
                annual_mean: 0.0,
                annual_std_dev: 0.0,
            },
            vec![CashFlow {
                amount: -5000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            false,
            "annual",
        );
        let result = run_linear(&req);
        // Zero return, -5000/month for 12 months = 100k - 60k = 40k
        assert!(
            (result.final_balance - 40_000.0).abs() < 1.0,
            "expected 40000, got {}",
            result.final_balance
        );
        assert_eq!(result.total_withdrawals, 60_000.0);
    }

    #[test]
    fn test_linear_time_series() {
        let req = legacy_request(
            100_000.0,
            24,
            ReturnAssumption {
                annual_mean: 0.0,
                annual_std_dev: 0.0,
            },
            vec![],
            false,
            "annual",
        );
        let result = run_linear(&req);
        assert_eq!(result.time_series.months, vec![0, 12, 24]);
        assert_eq!(result.time_series.balance.len(), 3);
        // All should be 100k with zero return and no cash flows
        for &b in &result.time_series.balance {
            assert!((b - 100_000.0).abs() < 1.0);
        }
    }

    #[test]
    fn test_linear_return_earned() {
        let req = legacy_request(
            100_000.0,
            12,
            ReturnAssumption {
                annual_mean: 0.10,
                annual_std_dev: 0.0,
            },
            vec![],
            false,
            "annual",
        );
        let result = run_linear(&req);
        // total_return_earned ≈ final_balance - starting_balance (no cash flows)
        let expected_return = result.final_balance - 100_000.0;
        assert!(
            (result.total_return_earned - expected_return).abs() < 1.0,
            "return_earned={}, expected={}",
            result.total_return_earned,
            expected_return
        );
    }

    #[test]
    fn test_linear_detail_annual() {
        let req = legacy_request(
            100_000.0,
            360,
            ReturnAssumption {
                annual_mean: 0.06,
                annual_std_dev: 0.0,
            },
            vec![CashFlow {
                amount: -500.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            true,
            "annual",
        );
        let result = run_linear(&req);
        let detail = result.annual_detail.expect("detail should be present");
        assert_eq!(detail.len(), 30, "30 years = 30 annual rows");

        // First row starts at beginning balance
        assert!((detail[0].beginning_balance - 100_000.0).abs() < 0.01);
        assert!((detail[0].withdrawals - 6000.0).abs() < 0.01); // 500/mo * 12

        // Each row's ending balance matches next row's beginning balance
        for i in 0..detail.len() - 1 {
            assert!(
                (detail[i].ending_balance - detail[i + 1].beginning_balance).abs() < 0.01,
                "row {}: ending {} != row {} beginning {}",
                i,
                detail[i].ending_balance,
                i + 1,
                detail[i + 1].beginning_balance
            );
        }

        // Last row's ending balance matches final_balance
        assert!((detail.last().unwrap().ending_balance - result.final_balance).abs() < 0.01);

        // Cumulative totals grow monotonically
        for i in 1..detail.len() {
            assert!(detail[i].cumulative_withdrawals >= detail[i - 1].cumulative_withdrawals);
        }
    }

    #[test]
    fn test_linear_detail_monthly() {
        let req = legacy_request(
            100_000.0,
            24,
            ReturnAssumption {
                annual_mean: 0.0,
                annual_std_dev: 0.0,
            },
            vec![],
            true,
            "monthly",
        );
        let result = run_linear(&req);
        let detail = result.annual_detail.expect("detail should be present");
        assert_eq!(detail.len(), 24, "24 months = 24 monthly rows");
    }

    #[test]
    fn test_detail_not_included_by_default() {
        let req = legacy_request(
            100_000.0,
            12,
            ReturnAssumption {
                annual_mean: 0.06,
                annual_std_dev: 0.0,
            },
            vec![],
            false,
            "annual",
        );
        let result = run_linear(&req);
        assert!(result.annual_detail.is_none());
    }
}
