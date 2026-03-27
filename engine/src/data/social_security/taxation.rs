use crate::data::types::{DataError, FilingStatus};

/// Social Security benefit taxation thresholds (statutory, unchanged since 1993).
#[derive(Debug, Clone)]
pub struct SsTaxationThresholds {
    pub base_amount: f64,
    pub upper_amount: f64,
    pub max_taxable_pct_below_upper: f64,
    pub max_taxable_pct_above_upper: f64,
}

// Social Security benefit taxation thresholds (2026, reviewed artifact).
// Social Security benefit taxation thresholds (2026, reviewed artifact).
pub fn thresholds(
    status: FilingStatus,
    lived_with_spouse_during_year: Option<bool>,
) -> Result<SsTaxationThresholds, DataError> {
    match status {
        FilingStatus::Single => Ok(SsTaxationThresholds {
            base_amount: 25000.0,
            upper_amount: 34000.0,
            max_taxable_pct_below_upper: 0.5,
            max_taxable_pct_above_upper: 0.85,
        }),
        FilingStatus::MarriedFilingJointly => Ok(SsTaxationThresholds {
            base_amount: 32000.0,
            upper_amount: 44000.0,
            max_taxable_pct_below_upper: 0.5,
            max_taxable_pct_above_upper: 0.85,
        }),
        FilingStatus::HeadOfHousehold => Ok(SsTaxationThresholds {
            base_amount: 25000.0,
            upper_amount: 34000.0,
            max_taxable_pct_below_upper: 0.5,
            max_taxable_pct_above_upper: 0.85,
        }),
        FilingStatus::QualifyingSurvivingSpouse => Ok(SsTaxationThresholds {
            base_amount: 25000.0,
            upper_amount: 34000.0,
            max_taxable_pct_below_upper: 0.5,
            max_taxable_pct_above_upper: 0.85,
        }),
        FilingStatus::MarriedFilingSeparately => match lived_with_spouse_during_year {
            Some(true) => Ok(SsTaxationThresholds {
                base_amount: 0.0,
                upper_amount: 0.0,
                max_taxable_pct_below_upper: 0.5,
                max_taxable_pct_above_upper: 0.85,
            }),
            Some(false) => Ok(SsTaxationThresholds {
                base_amount: 25000.0,
                upper_amount: 34000.0,
                max_taxable_pct_below_upper: 0.5,
                max_taxable_pct_above_upper: 0.85,
            }),
            None => Err(DataError::InvalidParams("lived_with_spouse_during_year parameter is required for married_filing_separately Social Security benefit taxation lookups".to_string())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BEGIN GENERATED TAXATION TESTS
    #[test]
    fn single_thresholds() {
        let t = thresholds(FilingStatus::Single, None).unwrap();
        assert_eq!(t.base_amount, 25000.0);
        assert_eq!(t.upper_amount, 34000.0);
    }

    #[test]
    fn mfj_thresholds() {
        let t = thresholds(FilingStatus::MarriedFilingJointly, None).unwrap();
        assert_eq!(t.base_amount, 32000.0);
        assert_eq!(t.upper_amount, 44000.0);
    }

    #[test]
    fn mfs_lived_with_spouse_thresholds() {
        let t = thresholds(FilingStatus::MarriedFilingSeparately, Some(true)).unwrap();
        assert_eq!(t.base_amount, 0.0);
        assert_eq!(t.upper_amount, 0.0);
    }

    #[test]
    fn mfs_lived_apart_thresholds() {
        let t = thresholds(FilingStatus::MarriedFilingSeparately, Some(false)).unwrap();
        assert_eq!(t.base_amount, 25000.0);
        assert_eq!(t.upper_amount, 34000.0);
    }
    // END GENERATED TAXATION TESTS
}
