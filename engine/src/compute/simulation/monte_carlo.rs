use std::collections::{BTreeMap, BTreeSet};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

use crate::models::simulation_request::SimulationRequest;
use crate::models::simulation_response::{MonteCarloResult, SamplePath};

use super::buckets::{apply_monthly_returns, BucketState, HouseholdBucketState};
use super::normalized::normalize_request;
use super::path_simulator::resolve_monthly_cash_flows;
use super::rmd::{
    apply_rmd_withdrawals, compute_household_rmd_for_year, configured_distribution_month,
};
use super::statistics::compute_mc_stats;
use super::tax::{
    record_bucket_withdrawals_for_tax, settle_annual_tax, AnnualTaxAccumulator,
    DEFAULT_SIMULATION_TAX_YEAR,
};
use super::withdrawals::fund_household_deficit;

const HOUSEHOLD_CASH_BUCKET_ID: &str = "__household_cash__";

pub fn run_monte_carlo(req: &SimulationRequest) -> MonteCarloResult {
    let normalized =
        normalize_request(req).expect("monte carlo request should satisfy simulation contract");
    let num_sims = normalized.num_simulations;
    let total_months = normalized.time_horizon_months;
    let base_seed = normalized.seed.unwrap_or_else(|| rand::thread_rng().gen());
    let monthly_cash_flows = resolve_monthly_cash_flows(&req.cash_flows, total_months);
    let tax_enabled = normalized.tax_policy.mode != "none";
    let filing_status = normalized
        .filing_status
        .clone()
        .unwrap_or_else(|| "single".into());
    let modeled_tax_inflation_rate = normalized
        .tax_policy
        .modeled_tax_inflation_rate
        .unwrap_or(0.0);
    let withholding_bucket_id = normalized
        .spending_policy
        .rebalance_tax_withholding_from
        .clone();
    let withdrawal_order = normalized.spending_policy.withdrawal_order.clone();
    let contribution_bucket_id = normalized.buckets.first().map(|bucket| bucket.id.clone());
    let report_bucket_outputs = !req.buckets.is_empty();
    let bucket_specs = normalized.buckets.clone();
    let bucket_ids: Vec<String> = bucket_specs
        .iter()
        .map(|bucket| bucket.id.clone())
        .collect();

    let path_results: Vec<BucketedMonteCarloPath> = (0..num_sims)
        .into_par_iter()
        .map(|i| {
            let mut rng = ChaCha8Rng::seed_from_u64(base_seed.wrapping_add(i as u64));
            simulate_bucketed_path(
                &bucket_specs,
                &monthly_cash_flows,
                contribution_bucket_id.as_deref(),
                &withdrawal_order,
                withholding_bucket_id.as_deref(),
                &filing_status,
                modeled_tax_inflation_rate,
                &normalized,
                tax_enabled,
                &mut rng,
            )
        })
        .collect();
    let paths: Vec<Vec<f64>> = path_results
        .iter()
        .map(|result| result.balances.clone())
        .collect();
    let monthly_tax_paid_paths: Vec<Vec<f64>> = path_results
        .iter()
        .map(|result| result.monthly_tax_paid.clone())
        .collect();
    let terminal_balances_by_bucket: Vec<BTreeMap<String, f64>> = path_results
        .iter()
        .map(|result| result.terminal_balances_by_bucket.clone())
        .collect();
    let depleted_buckets_by_path: Vec<BTreeSet<String>> = path_results
        .iter()
        .map(|result| result.depleted_buckets.clone())
        .collect();

    let extract_indices: Option<Vec<usize>> = if let Some(ref indices) = req.path_indices {
        Some(indices.clone())
    } else if let Some(count) = req.sample_paths {
        if count > 0 && num_sims > 0 {
            let step = num_sims as f64 / count as f64;
            Some((0..count).map(|i| (i as f64 * step) as usize).collect())
        } else {
            None
        }
    } else {
        None
    };

    let extracted_paths = extract_sample_paths(&paths, extract_indices, total_months, num_sims);
    let mut terminal_balances: Vec<f64> = paths.iter().map(|p| *p.last().unwrap()).collect();

    let mut result = compute_mc_stats(
        &mut terminal_balances,
        &paths,
        total_months,
        num_sims,
        req.include_detail,
        &req.detail_granularity,
        &monthly_cash_flows,
        tax_enabled.then_some(monthly_tax_paid_paths.as_slice()),
        report_bucket_outputs.then_some(terminal_balances_by_bucket.as_slice()),
        report_bucket_outputs.then_some(bucket_ids.as_slice()),
        report_bucket_outputs.then_some(depleted_buckets_by_path.as_slice()),
        req.custom_percentiles.as_deref(),
    );

    result.sample_paths = extracted_paths;
    result
}

#[derive(Debug, Clone)]
struct BucketedMonteCarloPath {
    balances: Vec<f64>,
    monthly_tax_paid: Vec<f64>,
    terminal_balances_by_bucket: BTreeMap<String, f64>,
    depleted_buckets: BTreeSet<String>,
}

#[allow(clippy::too_many_arguments)]
fn simulate_bucketed_path(
    bucket_specs: &[super::normalized::NormalizedBucket],
    monthly_cash_flows: &[f64],
    contribution_bucket_id: Option<&str>,
    withdrawal_order: &[String],
    withholding_bucket_id: Option<&str>,
    filing_status: &str,
    modeled_tax_inflation_rate: f64,
    normalized_req: &super::normalized::NormalizedSimulationRequest,
    tax_enabled: bool,
    rng: &mut ChaCha8Rng,
) -> BucketedMonteCarloPath {
    let total_months = monthly_cash_flows.len();
    let monthly_returns = bucket_specs
        .iter()
        .map(|bucket| {
            (
                (1.0 + bucket.return_assumption.annual_mean).powf(1.0 / 12.0) - 1.0,
                bucket.return_assumption.annual_std_dev / 12.0_f64.sqrt(),
            )
        })
        .collect::<Vec<_>>();
    let mut state = HouseholdBucketState {
        buckets: bucket_specs
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
    let mut balances = Vec::with_capacity(total_months + 1);
    let mut monthly_tax_paid = vec![0.0; total_months];
    let mut annual_tax_accumulator = AnnualTaxAccumulator::default();
    let mut depleted_buckets = BTreeSet::new();
    let rmd_distribution_month = configured_distribution_month(normalized_req);
    let mut prior_year_end_state = state.clone();

    balances.push(total_balance(&state));

    for (month_index, monthly_cash_flow) in monthly_cash_flows.iter().enumerate() {
        let sampled_returns = monthly_returns
            .iter()
            .map(|(mean, std_dev)| {
                if *std_dev <= 0.0 {
                    *mean
                } else {
                    Normal::new(*mean, *std_dev)
                        .expect("monthly return distribution should be valid")
                        .sample(rng)
                }
            })
            .collect::<Vec<_>>();
        apply_monthly_returns(&mut state, &sampled_returns);

        if *monthly_cash_flow > 0.0 {
            if let Some(bucket_id) = contribution_bucket_id {
                if let Some(bucket) = state.bucket_mut(bucket_id) {
                    bucket.balance += *monthly_cash_flow;
                }
            }
        }

        let tax_year = DEFAULT_SIMULATION_TAX_YEAR + (month_index as u32 / 12);
        let month_of_year = (month_index as u32 % 12) + 1;
        if rmd_distribution_month == Some(month_of_year) {
            let schedule =
                compute_household_rmd_for_year(normalized_req, tax_year, &prior_year_end_state)
                    .expect("simulation RMD request should be supported");
            let rmd_withdrawals = apply_rmd_withdrawals(&mut state, &schedule);
            if tax_enabled && !rmd_withdrawals.is_empty() {
                record_bucket_withdrawals_for_tax(
                    &mut annual_tax_accumulator,
                    &state,
                    &rmd_withdrawals,
                );
            }
        }

        if *monthly_cash_flow < 0.0 {
            let remaining_deficit = spend_household_cash(&mut state, monthly_cash_flow.abs());
            if remaining_deficit > 0.0 {
                let withdrawal =
                    fund_household_deficit(&mut state, remaining_deficit, withdrawal_order);
                if tax_enabled {
                    record_bucket_withdrawals_for_tax(
                        &mut annual_tax_accumulator,
                        &state,
                        &withdrawal.withdrawals_by_bucket,
                    );
                }
            }
        }

        let is_year_end = (month_index + 1).is_multiple_of(12) || month_index + 1 == total_months;
        if tax_enabled && is_year_end {
            let settlement = settle_annual_tax(
                &mut state,
                &annual_tax_accumulator,
                filing_status,
                tax_year,
                modeled_tax_inflation_rate,
                withholding_bucket_id,
                withdrawal_order,
            )
            .expect("simulation tax request should be supported");
            monthly_tax_paid[month_index] = settlement.annual_tax_paid;
            annual_tax_accumulator = AnnualTaxAccumulator::default();
        }
        if is_year_end {
            prior_year_end_state = state.clone();
        }

        for bucket in &state.buckets {
            if bucket.balance <= 0.0 {
                depleted_buckets.insert(bucket.id.clone());
            }
        }

        balances.push(total_balance(&state));
    }

    BucketedMonteCarloPath {
        balances,
        monthly_tax_paid,
        terminal_balances_by_bucket: balances_by_bucket(&state),
        depleted_buckets,
    }
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
    let mut balances = state
        .buckets
        .iter()
        .map(|bucket| (bucket.id.clone(), bucket.balance))
        .collect::<BTreeMap<_, _>>();
    balances.insert(HOUSEHOLD_CASH_BUCKET_ID.to_string(), state.household_cash);
    balances
}

fn extract_sample_paths(
    paths: &[Vec<f64>],
    extract_indices: Option<Vec<usize>>,
    total_months: u32,
    num_sims: u32,
) -> Option<Vec<SamplePath>> {
    extract_indices.map(|indices| {
        let mut sample_months: Vec<u32> = (0..=total_months).step_by(12).collect();
        if *sample_months.last().unwrap_or(&0) != total_months {
            sample_months.push(total_months);
        }

        indices
            .iter()
            .filter(|&&idx| idx < num_sims as usize)
            .map(|&idx| {
                let balances = sample_months
                    .iter()
                    .map(|&m| (paths[idx][m as usize] * 100.0).round() / 100.0)
                    .collect();
                SamplePath {
                    index: idx,
                    months: sample_months.clone(),
                    balances,
                }
            })
            .collect()
    })
}

fn spend_household_cash(state: &mut HouseholdBucketState, amount_needed: f64) -> f64 {
    let applied = state.household_cash.min(amount_needed.max(0.0));
    state.household_cash -= applied;
    amount_needed - applied
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute::simulation::linear::run_linear;
    use crate::models::simulation_request::{
        CashFlow, HouseholdConfig, ReturnAssumption, RmdPolicy, SimulationBucket,
        SimulationRequest, SpendingPolicy, TaxPolicy,
    };
    use serde_json::to_value;

    fn base_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("monte_carlo".into()),
            num_simulations: Some(1000),
            seed: Some(12345),
            starting_balance: Some(500_000.0),
            buckets: vec![],
            time_horizon_months: 360,
            return_assumption: Some(ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            }),
            cash_flows: vec![CashFlow {
                amount: -2000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            filing_status: None,
            household: None,
            spending_policy: None,
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".to_string(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    fn zero_vol_bucketed_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("monte_carlo".into()),
            num_simulations: Some(32),
            seed: Some(42),
            starting_balance: None,
            buckets: vec![
                SimulationBucket {
                    id: "taxable".into(),
                    bucket_type: "taxable".into(),
                    starting_balance: 60_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.0,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: Some(0.25),
                    withdrawal_priority: Some(1),
                },
                SimulationBucket {
                    id: "ira".into(),
                    bucket_type: "tax_deferred".into(),
                    starting_balance: 40_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.12,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(2),
                },
            ],
            time_horizon_months: 12,
            return_assumption: None,
            cash_flows: vec![CashFlow {
                amount: -500.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            filing_status: Some("single".into()),
            household: None,
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: vec!["taxable".into(), "ira".into()],
                rebalance_tax_withholding_from: None,
            }),
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    fn bucketed_mc_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("monte_carlo".into()),
            num_simulations: Some(32),
            seed: Some(7),
            starting_balance: None,
            buckets: vec![
                SimulationBucket {
                    id: "taxable".into(),
                    bucket_type: "taxable".into(),
                    starting_balance: 80_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.04,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: Some(0.25),
                    withdrawal_priority: Some(1),
                },
                SimulationBucket {
                    id: "ira".into(),
                    bucket_type: "tax_deferred".into(),
                    starting_balance: 120_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.06,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(2),
                },
            ],
            time_horizon_months: 12,
            return_assumption: None,
            cash_flows: vec![],
            filing_status: Some("single".into()),
            household: None,
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: vec!["taxable".into(), "ira".into()],
                rebalance_tax_withholding_from: None,
            }),
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    fn stressed_bucketed_mc_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("monte_carlo".into()),
            num_simulations: Some(32),
            seed: Some(11),
            starting_balance: None,
            buckets: vec![
                SimulationBucket {
                    id: "taxable".into(),
                    bucket_type: "taxable".into(),
                    starting_balance: 12_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.0,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: Some(0.25),
                    withdrawal_priority: Some(1),
                },
                SimulationBucket {
                    id: "ira".into(),
                    bucket_type: "tax_deferred".into(),
                    starting_balance: 18_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.0,
                        annual_std_dev: 0.0,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(2),
                },
            ],
            time_horizon_months: 24,
            return_assumption: None,
            cash_flows: vec![CashFlow {
                amount: -4_000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            filing_status: Some("single".into()),
            household: None,
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: vec!["taxable".into(), "ira".into()],
                rebalance_tax_withholding_from: None,
            }),
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    fn rmd_terminal_cash_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("monte_carlo".into()),
            num_simulations: Some(8),
            seed: Some(19),
            starting_balance: None,
            buckets: vec![SimulationBucket {
                id: "ira".into(),
                bucket_type: "tax_deferred".into(),
                starting_balance: 100_000.0,
                return_assumption: ReturnAssumption {
                    annual_mean: 0.0,
                    annual_std_dev: 0.0,
                },
                realized_gain_ratio: None,
                withdrawal_priority: Some(1),
            }],
            time_horizon_months: 12,
            return_assumption: None,
            cash_flows: vec![],
            filing_status: Some("single".into()),
            household: Some(HouseholdConfig {
                birth_years: Some(vec![1950]),
                retirement_month: Some(0),
            }),
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: vec!["ira".into()],
                rebalance_tax_withholding_from: None,
            }),
            tax_policy: None,
            rmd_policy: Some(RmdPolicy {
                enabled: true,
                distribution_month: Some(12),
            }),
            include_detail: false,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    fn depleted_then_refilled_bucket_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("monte_carlo".into()),
            num_simulations: Some(8),
            seed: Some(23),
            starting_balance: None,
            buckets: vec![SimulationBucket {
                id: "taxable".into(),
                bucket_type: "taxable".into(),
                starting_balance: 1_000.0,
                return_assumption: ReturnAssumption {
                    annual_mean: 0.0,
                    annual_std_dev: 0.0,
                },
                realized_gain_ratio: Some(0.0),
                withdrawal_priority: Some(1),
            }],
            time_horizon_months: 2,
            return_assumption: None,
            cash_flows: vec![
                CashFlow {
                    amount: -1_000.0,
                    frequency: "one_time".into(),
                    start_month: Some(0),
                    end_month: None,
                },
                CashFlow {
                    amount: 1_000.0,
                    frequency: "one_time".into(),
                    start_month: Some(1),
                    end_month: None,
                },
            ],
            filing_status: Some("single".into()),
            household: None,
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: vec!["taxable".into()],
                rebalance_tax_withholding_from: None,
            }),
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".into(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    #[test]
    fn test_deterministic_with_same_seed() {
        let req = base_request();
        let result1 = run_monte_carlo(&req);
        let result2 = run_monte_carlo(&req);
        assert_eq!(result1.mean, result2.mean);
        assert_eq!(result1.std_dev, result2.std_dev);
        assert_eq!(result1.percentiles.p50, result2.percentiles.p50);
    }

    #[test]
    fn test_different_seed_different_results() {
        let mut req1 = base_request();
        req1.seed = Some(111);
        let mut req2 = base_request();
        req2.seed = Some(222);
        let result1 = run_monte_carlo(&req1);
        let result2 = run_monte_carlo(&req2);
        // Results should differ (extremely unlikely to be identical)
        assert_ne!(result1.mean, result2.mean);
    }

    #[test]
    fn test_zero_volatility_matches_linear() {
        let req = SimulationRequest {
            mode: Some("monte_carlo".into()),
            num_simulations: Some(100),
            seed: Some(42),
            starting_balance: Some(100_000.0),
            buckets: vec![],
            time_horizon_months: 12,
            return_assumption: Some(ReturnAssumption {
                annual_mean: 0.06,
                annual_std_dev: 0.0,
            }),
            cash_flows: vec![],
            filing_status: None,
            household: None,
            spending_policy: None,
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".to_string(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        };
        let result = run_monte_carlo(&req);
        // With zero std dev, all paths should be identical
        assert!((result.std_dev).abs() < 0.01);
        // Mean should equal the deterministic result
        let monthly_return = 1.06_f64.powf(1.0 / 12.0) - 1.0;
        let expected = 100_000.0 * (1.0 + monthly_return).powi(12);
        assert!((result.mean - expected).abs() < 1.0);
    }

    #[test]
    fn test_zero_volatility_bucketed_monte_carlo_matches_linear() {
        let req = zero_vol_bucketed_request();

        let mc = run_monte_carlo(&req);
        let linear = run_linear(&req);

        assert!((mc.mean - linear.final_balance).abs() < 1.0);
    }

    #[test]
    fn test_bucketed_mc_reports_terminal_balances_by_bucket() {
        let req = bucketed_mc_request();

        let result = run_monte_carlo(&req);
        let json = to_value(&result).expect("monte carlo result should serialize");

        assert!(
            json["bucket_terminal_percentiles"]["taxable"]["p50"]
                .as_f64()
                .expect("bucket percentile should be present")
                >= 0.0
        );
    }

    #[test]
    fn test_bucketed_mc_failure_tracks_depletion_months() {
        let req = stressed_bucketed_mc_request();

        let result = run_monte_carlo(&req);

        assert!(!result.paths_depleted_by_month.is_empty());
    }

    #[test]
    fn test_bucketed_mc_terminal_bucket_summaries_include_household_cash() {
        let req = rmd_terminal_cash_request();

        let result = run_monte_carlo(&req);
        let bucket_percentiles = result
            .bucket_terminal_percentiles
            .as_ref()
            .expect("bucket terminal percentiles should be present");

        let cash_bucket = bucket_percentiles
            .get(HOUSEHOLD_CASH_BUCKET_ID)
            .expect("household cash summary bucket should be present");
        let ira_bucket = bucket_percentiles
            .get("ira")
            .expect("ira bucket should be present");

        assert!(cash_bucket.p50 > 0.0);
        assert!((ira_bucket.p50 + cash_bucket.p50 - result.mean).abs() < 0.01);
    }

    #[test]
    fn test_bucketed_mc_depletion_counts_track_refilled_buckets() {
        let req = depleted_then_refilled_bucket_request();

        let result = run_monte_carlo(&req);
        let bucket_percentiles = result
            .bucket_terminal_percentiles
            .as_ref()
            .expect("bucket terminal percentiles should be present");
        let bucket_depletion_counts = result
            .bucket_depletion_counts
            .as_ref()
            .expect("bucket depletion counts should be present");

        assert!(bucket_percentiles["taxable"].p50 > 0.0);
        assert_eq!(bucket_depletion_counts.get("taxable"), Some(&8));
    }

    #[test]
    fn test_success_rate_bounds() {
        let req = base_request();
        let result = run_monte_carlo(&req);
        assert!(result.success_rate >= 0.0);
        assert!(result.success_rate <= 1.0);
    }

    #[test]
    fn test_percentiles_ordered() {
        let req = base_request();
        let result = run_monte_carlo(&req);
        assert!(result.percentiles.p5 <= result.percentiles.p10);
        assert!(result.percentiles.p10 <= result.percentiles.p25);
        assert!(result.percentiles.p25 <= result.percentiles.p50);
        assert!(result.percentiles.p50 <= result.percentiles.p75);
        assert!(result.percentiles.p75 <= result.percentiles.p90);
        assert!(result.percentiles.p90 <= result.percentiles.p95);
    }

    #[test]
    fn test_time_series_has_correct_months() {
        let req = base_request();
        let result = run_monte_carlo(&req);
        assert_eq!(result.time_series.months[0], 0);
        assert_eq!(*result.time_series.months.last().unwrap(), 360);
        // All series should have the same length as months
        assert_eq!(
            result.time_series.p10.len(),
            result.time_series.months.len()
        );
        assert_eq!(
            result.time_series.p50.len(),
            result.time_series.months.len()
        );
        assert_eq!(
            result.time_series.p90.len(),
            result.time_series.months.len()
        );
    }

    #[test]
    fn test_mc_detail_annual() {
        let mut req = base_request();
        req.include_detail = true;
        let result = run_monte_carlo(&req);
        let detail = result.annual_detail.expect("detail should be present");
        assert_eq!(detail.len(), 30, "360 months / 12 = 30 annual rows");

        // Percentiles should be ordered at each row
        for row in &detail {
            assert!(row.balance_p10 <= row.balance_p25);
            assert!(row.balance_p25 <= row.balance_p50);
            assert!(row.balance_p50 <= row.balance_p75);
            assert!(row.balance_p75 <= row.balance_p90);
        }

        // Survival rate should be in [0, 1]
        for row in &detail {
            assert!(row.survival_rate >= 0.0 && row.survival_rate <= 1.0);
        }

        // Cumulative cash flow should grow (all negative cash flows)
        for i in 1..detail.len() {
            assert!(detail[i].cumulative_cash_flow <= detail[i - 1].cumulative_cash_flow);
        }
    }

    #[test]
    fn test_mc_detail_not_included_by_default() {
        let req = base_request();
        let result = run_monte_carlo(&req);
        assert!(result.annual_detail.is_none());
    }

    #[test]
    fn test_sample_paths_count() {
        let mut req = base_request();
        req.sample_paths = Some(3);
        let result = run_monte_carlo(&req);
        let paths = result.sample_paths.expect("sample_paths should be present");
        assert_eq!(paths.len(), 3);
        // Each path should have annual sample months
        for p in &paths {
            assert_eq!(p.months[0], 0);
            assert_eq!(*p.months.last().unwrap(), 360);
            assert_eq!(p.balances.len(), p.months.len());
        }
        // Indices should be evenly spaced
        assert_eq!(paths[0].index, 0);
    }

    #[test]
    fn test_path_indices_specific() {
        let mut req = base_request();
        req.path_indices = Some(vec![0, 99, 500]);
        let result = run_monte_carlo(&req);
        let paths = result.sample_paths.expect("sample_paths should be present");
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0].index, 0);
        assert_eq!(paths[1].index, 99);
        assert_eq!(paths[2].index, 500);
    }

    #[test]
    fn test_custom_percentiles() {
        let mut req = base_request();
        req.custom_percentiles = Some(vec![5, 30, 70, 95]);
        let result = run_monte_carlo(&req);
        let series = result
            .custom_percentile_series
            .expect("custom_percentile_series should be present");
        assert!(series.contains_key("p5"));
        assert!(series.contains_key("p30"));
        assert!(series.contains_key("p70"));
        assert!(series.contains_key("p95"));
        // All series should have the same length as time_series.months
        let expected_len = result.time_series.months.len();
        for (_, vals) in &series {
            assert_eq!(vals.len(), expected_len);
        }
    }

    #[test]
    fn test_no_sample_paths_by_default() {
        let req = base_request();
        let result = run_monte_carlo(&req);
        assert!(result.sample_paths.is_none());
        assert!(result.custom_percentile_series.is_none());
    }

    #[test]
    fn test_mc_bucketed_tax_deferred_withdrawal_reports_annual_tax_paid() {
        let req = SimulationRequest {
            mode: Some("monte_carlo".into()),
            num_simulations: Some(32),
            seed: Some(7),
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

        let result = run_monte_carlo(&req);
        let detail = result.annual_detail.expect("detail should be present");

        assert!(detail[0].annual_tax_paid > 0.0);
        assert!(result.mean < 10_000.0);
    }
}
