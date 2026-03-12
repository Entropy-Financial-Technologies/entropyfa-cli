use crate::data::types::FilingStatus;

/// A single IRMAA bracket tier.
#[derive(Debug, Clone)]
pub struct IrmaaBracket {
    pub magi_min: f64,
    pub magi_max: Option<f64>,
    pub monthly_surcharge: f64,
}

/// IRMAA brackets for 2026 (using 2025 published values as placeholder,
/// since 2026 CMS brackets are based on 2024 MAGI and published late in prior year).
///
/// Base Part B premium is approximately $185.00/month (not included in surcharge).
pub fn brackets(status: FilingStatus) -> Vec<IrmaaBracket> {
    match status {
        FilingStatus::Single
        | FilingStatus::HeadOfHousehold
        | FilingStatus::QualifyingSurvivingSpouse => vec![
            IrmaaBracket {
                magi_min: 0.0,
                magi_max: Some(106_000.0),
                monthly_surcharge: 0.0,
            },
            IrmaaBracket {
                magi_min: 106_000.0,
                magi_max: Some(133_000.0),
                monthly_surcharge: 74.00,
            },
            IrmaaBracket {
                magi_min: 133_000.0,
                magi_max: Some(167_000.0),
                monthly_surcharge: 185.00,
            },
            IrmaaBracket {
                magi_min: 167_000.0,
                magi_max: Some(200_000.0),
                monthly_surcharge: 295.90,
            },
            IrmaaBracket {
                magi_min: 200_000.0,
                magi_max: Some(500_000.0),
                monthly_surcharge: 406.90,
            },
            IrmaaBracket {
                magi_min: 500_000.0,
                magi_max: None,
                monthly_surcharge: 444.30,
            },
        ],
        FilingStatus::MarriedFilingJointly => vec![
            IrmaaBracket {
                magi_min: 0.0,
                magi_max: Some(212_000.0),
                monthly_surcharge: 0.0,
            },
            IrmaaBracket {
                magi_min: 212_000.0,
                magi_max: Some(266_000.0),
                monthly_surcharge: 74.00,
            },
            IrmaaBracket {
                magi_min: 266_000.0,
                magi_max: Some(334_000.0),
                monthly_surcharge: 185.00,
            },
            IrmaaBracket {
                magi_min: 334_000.0,
                magi_max: Some(400_000.0),
                monthly_surcharge: 295.90,
            },
            IrmaaBracket {
                magi_min: 400_000.0,
                magi_max: Some(750_000.0),
                monthly_surcharge: 406.90,
            },
            IrmaaBracket {
                magi_min: 750_000.0,
                magi_max: None,
                monthly_surcharge: 444.30,
            },
        ],
        FilingStatus::MarriedFilingSeparately => vec![
            IrmaaBracket {
                magi_min: 0.0,
                magi_max: Some(106_000.0),
                monthly_surcharge: 0.0,
            },
            IrmaaBracket {
                magi_min: 106_000.0,
                magi_max: Some(167_000.0),
                monthly_surcharge: 295.90,
            },
            IrmaaBracket {
                magi_min: 167_000.0,
                magi_max: None,
                monthly_surcharge: 444.30,
            },
        ],
    }
}

/// Base Part B monthly premium (approximate, 2026 placeholder).
pub fn base_part_b_premium() -> f64 {
    185.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_irmaa_tiers() {
        let b = brackets(FilingStatus::Single);
        assert_eq!(b.len(), 6);
        assert_eq!(b[0].monthly_surcharge, 0.0);
        assert_eq!(b[5].monthly_surcharge, 444.30);
    }

    #[test]
    fn mfj_irmaa_tiers() {
        let b = brackets(FilingStatus::MarriedFilingJointly);
        assert_eq!(b.len(), 6);
        assert_eq!(b[0].magi_max, Some(212_000.0));
    }

    #[test]
    fn mfs_irmaa_only_three_tiers() {
        let b = brackets(FilingStatus::MarriedFilingSeparately);
        assert_eq!(b.len(), 3);
    }

    #[test]
    fn base_premium() {
        assert_eq!(base_part_b_premium(), 185.0);
    }
}
