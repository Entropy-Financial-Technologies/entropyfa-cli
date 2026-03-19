use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

use crate::models::simulation_request::SimulationRequest;
use crate::models::simulation_response::{MonteCarloResult, SamplePath};

use super::buckets::{apply_monthly_returns, BucketState, HouseholdBucketState};
use super::normalized::normalize_request;
use super::path_simulator::{resolve_monthly_cash_flows, simulate_path};
use super::statistics::compute_mc_stats;
use super::tax::{
    compute_annual_household_tax, pay_annual_tax, record_bucket_withdrawals_for_tax,
    AnnualTaxAccumulator, DEFAULT_SIMULATION_TAX_YEAR,
};
use super::withdrawals::fund_household_deficit;

pub fn run_monte_carlo(req: &SimulationRequest) -> MonteCarloResult {
    let tax_enabled = req
        .tax_policy
        .as_ref()
        .map(|policy| policy.mode != "none")
        .unwrap_or(false);
    if tax_enabled {
        return run_monte_carlo_with_annual_tax(req);
    }

    let num_sims = req.num_simulations.unwrap_or(10000);
    let total_months = req.time_horizon_months;
    let base_seed = req.seed.unwrap_or_else(|| rand::thread_rng().gen());
    let starting_balance = req.legacy_starting_balance();
    let return_assumption = req.legacy_return_assumption();

    // Convert annual to monthly
    let monthly_mean = (1.0 + return_assumption.annual_mean).powf(1.0 / 12.0) - 1.0;
    let monthly_std = return_assumption.annual_std_dev / 12.0_f64.sqrt();

    let monthly_cash_flows = resolve_monthly_cash_flows(&req.cash_flows, total_months);

    // Run simulations in parallel with rayon
    let paths: Vec<Vec<f64>> = (0..num_sims)
        .into_par_iter()
        .map(|i| {
            let mut rng = ChaCha8Rng::seed_from_u64(base_seed.wrapping_add(i as u64));
            let normal = Normal::new(monthly_mean, monthly_std).unwrap();

            let returns: Vec<f64> = (0..total_months).map(|_| normal.sample(&mut rng)).collect();

            simulate_path(starting_balance, &monthly_cash_flows, &returns)
        })
        .collect();

    // Resolve which path indices to extract
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

    // Extract sample paths at annual intervals
    let extracted_paths: Option<Vec<SamplePath>> = extract_indices.map(|indices| {
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
    });

    let mut terminal_balances: Vec<f64> = paths.iter().map(|p| *p.last().unwrap()).collect();

    let mut result = compute_mc_stats(
        &mut terminal_balances,
        &paths,
        total_months,
        num_sims,
        req.include_detail,
        &req.detail_granularity,
        &monthly_cash_flows,
        None,
        req.custom_percentiles.as_deref(),
    );

    result.sample_paths = extracted_paths;
    result
}

#[derive(Debug, Clone)]
struct TaxAwareMonteCarloPath {
    balances: Vec<f64>,
    monthly_tax_paid: Vec<f64>,
}

fn run_monte_carlo_with_annual_tax(req: &SimulationRequest) -> MonteCarloResult {
    let normalized = normalize_request(req)
        .expect("tax-aware monte carlo request should satisfy simulation contract");
    let num_sims = normalized.num_simulations;
    let total_months = normalized.time_horizon_months;
    let base_seed = normalized.seed.unwrap_or_else(|| rand::thread_rng().gen());
    let monthly_cash_flows = resolve_monthly_cash_flows(&req.cash_flows, total_months);
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
    let bucket_specs = normalized.buckets.clone();

    let path_results: Vec<TaxAwareMonteCarloPath> = (0..num_sims)
        .into_par_iter()
        .map(|i| {
            let mut rng = ChaCha8Rng::seed_from_u64(base_seed.wrapping_add(i as u64));
            simulate_bucketed_tax_path(
                &bucket_specs,
                &monthly_cash_flows,
                contribution_bucket_id.as_deref(),
                &withdrawal_order,
                withholding_bucket_id.as_deref(),
                &filing_status,
                modeled_tax_inflation_rate,
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

    let extracted_paths: Option<Vec<SamplePath>> = extract_indices.map(|indices| {
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
    });

    let mut terminal_balances: Vec<f64> = paths.iter().map(|path| *path.last().unwrap()).collect();
    let mut result = compute_mc_stats(
        &mut terminal_balances,
        &paths,
        total_months,
        num_sims,
        req.include_detail,
        &req.detail_granularity,
        &monthly_cash_flows,
        Some(&monthly_tax_paid_paths),
        req.custom_percentiles.as_deref(),
    );

    result.sample_paths = extracted_paths;
    result
}

#[allow(clippy::too_many_arguments)]
fn simulate_bucketed_tax_path(
    bucket_specs: &[super::normalized::NormalizedBucket],
    monthly_cash_flows: &[f64],
    contribution_bucket_id: Option<&str>,
    withdrawal_order: &[String],
    withholding_bucket_id: Option<&str>,
    filing_status: &str,
    modeled_tax_inflation_rate: f64,
    rng: &mut ChaCha8Rng,
) -> TaxAwareMonteCarloPath {
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
        } else if *monthly_cash_flow < 0.0 {
            let withdrawal =
                fund_household_deficit(&mut state, monthly_cash_flow.abs(), withdrawal_order);
            record_bucket_withdrawals_for_tax(
                &mut annual_tax_accumulator,
                &state,
                &withdrawal.withdrawals_by_bucket,
            );
        }

        let is_year_end = (month_index + 1).is_multiple_of(12) || month_index + 1 == total_months;
        if is_year_end {
            let tax_year = DEFAULT_SIMULATION_TAX_YEAR + (month_index as u32 / 12);
            let tax_result = compute_annual_household_tax(
                &annual_tax_accumulator,
                filing_status,
                tax_year,
                modeled_tax_inflation_rate,
            )
            .expect("simulation tax request should be supported");
            monthly_tax_paid[month_index] = pay_annual_tax(
                &mut state,
                tax_result.total_tax,
                withholding_bucket_id,
                withdrawal_order,
            );
            annual_tax_accumulator = AnnualTaxAccumulator::default();
        }

        balances.push(total_balance(&state));
    }

    TaxAwareMonteCarloPath {
        balances,
        monthly_tax_paid,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::simulation_request::{
        CashFlow, ReturnAssumption, SimulationBucket, SimulationRequest, SpendingPolicy, TaxPolicy,
    };

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
