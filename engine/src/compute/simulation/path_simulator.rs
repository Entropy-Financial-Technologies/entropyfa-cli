use crate::models::simulation_request::CashFlow;

/// Resolve a list of CashFlow definitions into a Vec<f64> of net cash flow per month.
pub fn resolve_monthly_cash_flows(cash_flows: &[CashFlow], total_months: u32) -> Vec<f64> {
    let mut monthly = vec![0.0_f64; total_months as usize];

    for cf in cash_flows {
        let start = cf.start_month.unwrap_or(0);
        let end = cf.end_month.unwrap_or(total_months);

        for month in start..end.min(total_months) {
            let applies = match cf.frequency.as_str() {
                "monthly" => true,
                "quarterly" => (month - start) % 3 == 0,
                "annually" | "annual" => (month - start) % 12 == 0,
                "one_time" | "one-time" => month == start,
                _ => month == start, // treat unknown as one-time
            };
            if applies {
                monthly[month as usize] += cf.amount;
            }
        }
    }

    monthly
}

/// Simulate a single path: apply monthly returns and cash flows, floor at 0.
/// Returns a Vec<f64> of balances at the end of each month (length = total_months + 1,
/// where index 0 is the starting balance).
pub fn simulate_path(
    starting_balance: f64,
    monthly_cash_flows: &[f64],
    monthly_returns: &[f64],
) -> Vec<f64> {
    let n = monthly_cash_flows.len();
    let mut path = Vec::with_capacity(n + 1);
    path.push(starting_balance);

    let mut balance = starting_balance;
    for i in 0..n {
        // Apply return, then cash flow
        balance *= 1.0 + monthly_returns[i];
        balance += monthly_cash_flows[i];
        // Floor at 0 — can't go negative
        if balance < 0.0 {
            balance = 0.0;
        }
        path.push(balance);
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::simulation_request::CashFlow;

    #[test]
    fn test_monthly_cash_flows() {
        let cfs = vec![CashFlow {
            amount: -1000.0,
            frequency: "monthly".into(),
            start_month: Some(0),
            end_month: None,
        }];
        let resolved = resolve_monthly_cash_flows(&cfs, 12);
        assert_eq!(resolved.len(), 12);
        assert!(resolved.iter().all(|&v| v == -1000.0));
    }

    #[test]
    fn test_quarterly_cash_flows() {
        let cfs = vec![CashFlow {
            amount: -3000.0,
            frequency: "quarterly".into(),
            start_month: Some(0),
            end_month: None,
        }];
        let resolved = resolve_monthly_cash_flows(&cfs, 12);
        // Months 0, 3, 6, 9 should have the cash flow
        for (i, &v) in resolved.iter().enumerate() {
            if i % 3 == 0 {
                assert_eq!(v, -3000.0, "month {} should have quarterly flow", i);
            } else {
                assert_eq!(v, 0.0, "month {} should be zero", i);
            }
        }
    }

    #[test]
    fn test_annual_cash_flows() {
        let cfs = vec![CashFlow {
            amount: -12000.0,
            frequency: "annually".into(),
            start_month: Some(0),
            end_month: None,
        }];
        let resolved = resolve_monthly_cash_flows(&cfs, 24);
        assert_eq!(resolved[0], -12000.0);
        assert_eq!(resolved[12], -12000.0);
        assert_eq!(resolved[1], 0.0);
        assert_eq!(resolved[6], 0.0);
    }

    #[test]
    fn test_one_time_cash_flow() {
        let cfs = vec![CashFlow {
            amount: 50000.0,
            frequency: "one_time".into(),
            start_month: Some(6),
            end_month: None,
        }];
        let resolved = resolve_monthly_cash_flows(&cfs, 12);
        assert_eq!(resolved[6], 50000.0);
        assert_eq!(resolved[0], 0.0);
        assert_eq!(resolved[11], 0.0);
    }

    #[test]
    fn test_bounded_cash_flow() {
        let cfs = vec![CashFlow {
            amount: -1000.0,
            frequency: "monthly".into(),
            start_month: Some(3),
            end_month: Some(6),
        }];
        let resolved = resolve_monthly_cash_flows(&cfs, 12);
        assert_eq!(resolved[2], 0.0);
        assert_eq!(resolved[3], -1000.0);
        assert_eq!(resolved[4], -1000.0);
        assert_eq!(resolved[5], -1000.0);
        assert_eq!(resolved[6], 0.0); // end_month is exclusive
    }

    #[test]
    fn test_multiple_cash_flows_combine() {
        let cfs = vec![
            CashFlow {
                amount: -2000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            },
            CashFlow {
                amount: 500.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            },
        ];
        let resolved = resolve_monthly_cash_flows(&cfs, 6);
        assert!(resolved.iter().all(|&v| (v - (-1500.0)).abs() < 1e-10));
    }

    #[test]
    fn test_simulate_path_no_return_no_cashflow() {
        let cfs = vec![0.0; 12];
        let returns = vec![0.0; 12];
        let path = simulate_path(100_000.0, &cfs, &returns);
        assert_eq!(path.len(), 13);
        assert!((path[0] - 100_000.0).abs() < 1e-10);
        assert!((path[12] - 100_000.0).abs() < 1e-10);
    }

    #[test]
    fn test_simulate_path_with_return() {
        // 1% monthly return, no cash flows, for 1 month
        let cfs = vec![0.0];
        let returns = vec![0.01];
        let path = simulate_path(100_000.0, &cfs, &returns);
        assert!((path[1] - 101_000.0).abs() < 0.01);
    }

    #[test]
    fn test_simulate_path_floors_at_zero() {
        // Massive withdrawal that depletes the portfolio
        let cfs = vec![-200_000.0];
        let returns = vec![0.0];
        let path = simulate_path(100_000.0, &cfs, &returns);
        assert_eq!(path[1], 0.0);
    }

    #[test]
    fn test_simulate_path_compound_growth() {
        // 1% monthly for 12 months, no cash flows
        let cfs = vec![0.0; 12];
        let returns = vec![0.01; 12];
        let path = simulate_path(100_000.0, &cfs, &returns);
        let expected = 100_000.0 * 1.01_f64.powi(12);
        assert!((path[12] - expected).abs() < 0.01);
    }
}
