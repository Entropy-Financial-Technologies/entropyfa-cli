use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

use crate::models::simulation_request::SimulationRequest;
use crate::models::simulation_response::MonteCarloResult;

use super::path_simulator::{resolve_monthly_cash_flows, simulate_path};
use super::statistics::compute_mc_stats;

pub fn run_monte_carlo(req: &SimulationRequest) -> MonteCarloResult {
    let num_sims = req.num_simulations.unwrap_or(10000);
    let total_months = req.time_horizon_months;
    let base_seed = req.seed.unwrap_or_else(|| rand::thread_rng().gen());

    // Convert annual to monthly
    let monthly_mean = (1.0 + req.return_assumption.annual_mean).powf(1.0 / 12.0) - 1.0;
    let monthly_std = req.return_assumption.annual_std_dev / 12.0_f64.sqrt();

    let monthly_cash_flows = resolve_monthly_cash_flows(&req.cash_flows, total_months);

    // Run simulations in parallel with rayon
    let paths: Vec<Vec<f64>> = (0..num_sims)
        .into_par_iter()
        .map(|i| {
            let mut rng = ChaCha8Rng::seed_from_u64(base_seed.wrapping_add(i as u64));
            let normal = Normal::new(monthly_mean, monthly_std).unwrap();

            let returns: Vec<f64> = (0..total_months).map(|_| normal.sample(&mut rng)).collect();

            simulate_path(req.starting_balance, &monthly_cash_flows, &returns)
        })
        .collect();

    let mut terminal_balances: Vec<f64> = paths.iter().map(|p| *p.last().unwrap()).collect();

    compute_mc_stats(
        &mut terminal_balances,
        &paths,
        total_months,
        num_sims,
        req.include_detail,
        &req.detail_granularity,
        &monthly_cash_flows,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::simulation_request::{CashFlow, ReturnAssumption, SimulationRequest};

    fn base_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("monte_carlo".into()),
            num_simulations: Some(1000),
            seed: Some(12345),
            starting_balance: 500_000.0,
            time_horizon_months: 360,
            return_assumption: ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            },
            cash_flows: vec![CashFlow {
                amount: -2000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            include_detail: false,
            detail_granularity: "annual".to_string(),
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
            starting_balance: 100_000.0,
            time_horizon_months: 12,
            return_assumption: ReturnAssumption {
                annual_mean: 0.06,
                annual_std_dev: 0.0,
            },
            cash_flows: vec![],
            include_detail: false,
            detail_granularity: "annual".to_string(),
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
}
