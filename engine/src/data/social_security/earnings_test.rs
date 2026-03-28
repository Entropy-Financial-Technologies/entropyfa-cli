/// Social Security retirement earnings test thresholds for 2026, reviewed artifact.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SsEarningsTestThresholds {
    pub under_fra_annual_exempt_amount: f64,
    pub under_fra_monthly_exempt_amount: f64,
    pub year_of_fra_annual_exempt_amount: f64,
    pub year_of_fra_monthly_exempt_amount: f64,
    pub under_fra_reduction_rate: f64,
    pub year_of_fra_reduction_rate: f64,
}

pub fn thresholds() -> SsEarningsTestThresholds {
    SsEarningsTestThresholds {
        under_fra_annual_exempt_amount: 24480.0,
        under_fra_monthly_exempt_amount: 2040.0,
        year_of_fra_annual_exempt_amount: 65160.0,
        year_of_fra_monthly_exempt_amount: 5430.0,
        under_fra_reduction_rate: 0.5,
        year_of_fra_reduction_rate: 0.3333,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn earnings_test_thresholds_2026() {
        let t = thresholds();
        assert_eq!(t.under_fra_annual_exempt_amount, 24480.0);
        assert_eq!(t.under_fra_monthly_exempt_amount, 2040.0);
        assert_eq!(t.year_of_fra_annual_exempt_amount, 65160.0);
        assert_eq!(t.year_of_fra_monthly_exempt_amount, 5430.0);
        assert_eq!(t.under_fra_reduction_rate, 0.5);
        assert_eq!(t.year_of_fra_reduction_rate, 0.3333);
    }
}
