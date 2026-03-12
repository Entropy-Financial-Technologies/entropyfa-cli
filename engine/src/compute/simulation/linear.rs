use crate::models::simulation_request::SimulationRequest;
use crate::models::simulation_response::{LinearResult, LinearTimeSeries, PeriodDetail};

use super::path_simulator::{resolve_monthly_cash_flows, simulate_path};

pub fn run_linear(req: &SimulationRequest) -> LinearResult {
    let total_months = req.time_horizon_months;

    // Convert annual to monthly
    let monthly_mean = (1.0 + req.return_assumption.annual_mean).powf(1.0 / 12.0) - 1.0;

    let monthly_cash_flows = resolve_monthly_cash_flows(&req.cash_flows, total_months);
    let monthly_returns = vec![monthly_mean; total_months as usize];

    let path = simulate_path(req.starting_balance, &monthly_cash_flows, &monthly_returns);

    // Compute contributions and withdrawals
    let total_contributions: f64 = monthly_cash_flows.iter().filter(|&&v| v > 0.0).sum();
    let total_withdrawals: f64 = monthly_cash_flows
        .iter()
        .filter(|&&v| v < 0.0)
        .sum::<f64>()
        .abs();

    let final_balance = *path.last().unwrap();
    let net_cash_flow: f64 = monthly_cash_flows.iter().sum();
    let total_return_earned = final_balance - req.starting_balance - net_cash_flow;

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
            &req.detail_granularity,
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
        annual_detail,
    }
}

pub fn compute_period_detail(
    path: &[f64],
    monthly_cash_flows: &[f64],
    granularity: &str,
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
        });

        period_start = period_end;
    }

    details
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::simulation_request::{CashFlow, ReturnAssumption, SimulationRequest};

    #[test]
    fn test_linear_no_cashflows() {
        let req = SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
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
    fn test_linear_with_contributions() {
        let req = SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: 100_000.0,
            time_horizon_months: 12,
            return_assumption: ReturnAssumption {
                annual_mean: 0.0,
                annual_std_dev: 0.0,
            },
            cash_flows: vec![CashFlow {
                amount: 1000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            include_detail: false,
            detail_granularity: "annual".to_string(),
        };
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
        let req = SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: 100_000.0,
            time_horizon_months: 12,
            return_assumption: ReturnAssumption {
                annual_mean: 0.0,
                annual_std_dev: 0.0,
            },
            cash_flows: vec![CashFlow {
                amount: -5000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            include_detail: false,
            detail_granularity: "annual".to_string(),
        };
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
        let req = SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: 100_000.0,
            time_horizon_months: 24,
            return_assumption: ReturnAssumption {
                annual_mean: 0.0,
                annual_std_dev: 0.0,
            },
            cash_flows: vec![],
            include_detail: false,
            detail_granularity: "annual".to_string(),
        };
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
        let req = SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: 100_000.0,
            time_horizon_months: 12,
            return_assumption: ReturnAssumption {
                annual_mean: 0.10,
                annual_std_dev: 0.0,
            },
            cash_flows: vec![],
            include_detail: false,
            detail_granularity: "annual".to_string(),
        };
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
        let req = SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: 100_000.0,
            time_horizon_months: 360,
            return_assumption: ReturnAssumption {
                annual_mean: 0.06,
                annual_std_dev: 0.0,
            },
            cash_flows: vec![CashFlow {
                amount: -500.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            include_detail: true,
            detail_granularity: "annual".to_string(),
        };
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
        let req = SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
            starting_balance: 100_000.0,
            time_horizon_months: 24,
            return_assumption: ReturnAssumption {
                annual_mean: 0.0,
                annual_std_dev: 0.0,
            },
            cash_flows: vec![],
            include_detail: true,
            detail_granularity: "monthly".to_string(),
        };
        let result = run_linear(&req);
        let detail = result.annual_detail.expect("detail should be present");
        assert_eq!(detail.len(), 24, "24 months = 24 monthly rows");
    }

    #[test]
    fn test_detail_not_included_by_default() {
        let req = SimulationRequest {
            mode: Some("linear".into()),
            num_simulations: None,
            seed: None,
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
        let result = run_linear(&req);
        assert!(result.annual_detail.is_none());
    }
}
