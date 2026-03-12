use crate::data::types::FilingStatus;

/// Social Security benefit taxation thresholds (statutory, unchanged since 1993).
#[derive(Debug, Clone)]
pub struct SsTaxationThresholds {
    pub base_amount: f64,
    pub upper_amount: f64,
    pub max_taxable_pct_below_upper: f64,
    pub max_taxable_pct_above_upper: f64,
}

pub fn thresholds(status: FilingStatus) -> SsTaxationThresholds {
    match status {
        FilingStatus::Single
        | FilingStatus::HeadOfHousehold
        | FilingStatus::QualifyingSurvivingSpouse => SsTaxationThresholds {
            base_amount: 25_000.0,
            upper_amount: 34_000.0,
            max_taxable_pct_below_upper: 0.50,
            max_taxable_pct_above_upper: 0.85,
        },
        FilingStatus::MarriedFilingJointly => SsTaxationThresholds {
            base_amount: 32_000.0,
            upper_amount: 44_000.0,
            max_taxable_pct_below_upper: 0.50,
            max_taxable_pct_above_upper: 0.85,
        },
        FilingStatus::MarriedFilingSeparately => SsTaxationThresholds {
            base_amount: 0.0,
            upper_amount: 0.0,
            max_taxable_pct_below_upper: 0.50,
            max_taxable_pct_above_upper: 0.85,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_thresholds() {
        let t = thresholds(FilingStatus::Single);
        assert_eq!(t.base_amount, 25_000.0);
        assert_eq!(t.upper_amount, 34_000.0);
        assert_eq!(t.max_taxable_pct_below_upper, 0.50);
        assert_eq!(t.max_taxable_pct_above_upper, 0.85);
    }

    #[test]
    fn mfj_thresholds() {
        let t = thresholds(FilingStatus::MarriedFilingJointly);
        assert_eq!(t.base_amount, 32_000.0);
        assert_eq!(t.upper_amount, 44_000.0);
    }

    #[test]
    fn mfs_zero_thresholds() {
        let t = thresholds(FilingStatus::MarriedFilingSeparately);
        assert_eq!(t.base_amount, 0.0);
        assert_eq!(t.upper_amount, 0.0);
    }
}
