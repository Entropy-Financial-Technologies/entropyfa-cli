use crate::data::types::{DataError, FilingStatus};

/// A single IRMAA bracket tier.
#[derive(Debug, Clone)]
pub struct IrmaaBracket {
    pub magi_min: f64,
    pub magi_max: Option<f64>,
    pub monthly_surcharge: f64,
}

/// Generated from reviewed artifact for insurance irmaa_brackets.
pub fn brackets(
    status: FilingStatus,
    lived_with_spouse_during_year: Option<bool>,
) -> Result<Vec<IrmaaBracket>, DataError> {
    match status {
        FilingStatus::Single => Ok(vec![
            IrmaaBracket {
                magi_min: 0.0,
                magi_max: Some(109000.0),
                monthly_surcharge: 0.0,
            },
            IrmaaBracket {
                magi_min: 109000.0,
                magi_max: Some(137000.0),
                monthly_surcharge: 81.2,
            },
            IrmaaBracket {
                magi_min: 137000.0,
                magi_max: Some(171000.0),
                monthly_surcharge: 202.9,
            },
            IrmaaBracket {
                magi_min: 171000.0,
                magi_max: Some(205000.0),
                monthly_surcharge: 324.6,
            },
            IrmaaBracket {
                magi_min: 205000.0,
                magi_max: Some(500000.0),
                monthly_surcharge: 446.3,
            },
            IrmaaBracket {
                magi_min: 500000.0,
                magi_max: None,
                monthly_surcharge: 487.0,
            },
        ]),
        FilingStatus::MarriedFilingJointly => Ok(vec![
            IrmaaBracket {
                magi_min: 0.0,
                magi_max: Some(218000.0),
                monthly_surcharge: 0.0,
            },
            IrmaaBracket {
                magi_min: 218000.0,
                magi_max: Some(274000.0),
                monthly_surcharge: 81.2,
            },
            IrmaaBracket {
                magi_min: 274000.0,
                magi_max: Some(342000.0),
                monthly_surcharge: 202.9,
            },
            IrmaaBracket {
                magi_min: 342000.0,
                magi_max: Some(410000.0),
                monthly_surcharge: 324.6,
            },
            IrmaaBracket {
                magi_min: 410000.0,
                magi_max: Some(750000.0),
                monthly_surcharge: 446.3,
            },
            IrmaaBracket {
                magi_min: 750000.0,
                magi_max: None,
                monthly_surcharge: 487.0,
            },
        ]),
        FilingStatus::HeadOfHousehold => Ok(vec![
            IrmaaBracket {
                magi_min: 0.0,
                magi_max: Some(109000.0),
                monthly_surcharge: 0.0,
            },
            IrmaaBracket {
                magi_min: 109000.0,
                magi_max: Some(137000.0),
                monthly_surcharge: 81.2,
            },
            IrmaaBracket {
                magi_min: 137000.0,
                magi_max: Some(171000.0),
                monthly_surcharge: 202.9,
            },
            IrmaaBracket {
                magi_min: 171000.0,
                magi_max: Some(205000.0),
                monthly_surcharge: 324.6,
            },
            IrmaaBracket {
                magi_min: 205000.0,
                magi_max: Some(500000.0),
                monthly_surcharge: 446.3,
            },
            IrmaaBracket {
                magi_min: 500000.0,
                magi_max: None,
                monthly_surcharge: 487.0,
            },
        ]),
        FilingStatus::QualifyingSurvivingSpouse => Ok(vec![
            IrmaaBracket {
                magi_min: 0.0,
                magi_max: Some(109000.0),
                monthly_surcharge: 0.0,
            },
            IrmaaBracket {
                magi_min: 109000.0,
                magi_max: Some(137000.0),
                monthly_surcharge: 81.2,
            },
            IrmaaBracket {
                magi_min: 137000.0,
                magi_max: Some(171000.0),
                monthly_surcharge: 202.9,
            },
            IrmaaBracket {
                magi_min: 171000.0,
                magi_max: Some(205000.0),
                monthly_surcharge: 324.6,
            },
            IrmaaBracket {
                magi_min: 205000.0,
                magi_max: Some(500000.0),
                monthly_surcharge: 446.3,
            },
            IrmaaBracket {
                magi_min: 500000.0,
                magi_max: None,
                monthly_surcharge: 487.0,
            },
        ]),
        FilingStatus::MarriedFilingSeparately => match lived_with_spouse_during_year {
            Some(true) => Ok(vec![
            IrmaaBracket {
                magi_min: 0.0,
                magi_max: Some(109000.0),
                monthly_surcharge: 0.0,
            },
            IrmaaBracket {
                magi_min: 109000.0,
                magi_max: Some(391000.0),
                monthly_surcharge: 446.3,
            },
            IrmaaBracket {
                magi_min: 391000.0,
                magi_max: None,
                monthly_surcharge: 487.0,
            },
            ]),
            Some(false) => Ok(vec![
            IrmaaBracket {
                magi_min: 0.0,
                magi_max: Some(109000.0),
                monthly_surcharge: 0.0,
            },
            IrmaaBracket {
                magi_min: 109000.0,
                magi_max: Some(137000.0),
                monthly_surcharge: 81.2,
            },
            IrmaaBracket {
                magi_min: 137000.0,
                magi_max: Some(171000.0),
                monthly_surcharge: 202.9,
            },
            IrmaaBracket {
                magi_min: 171000.0,
                magi_max: Some(205000.0),
                monthly_surcharge: 324.6,
            },
            IrmaaBracket {
                magi_min: 205000.0,
                magi_max: Some(500000.0),
                monthly_surcharge: 446.3,
            },
            IrmaaBracket {
                magi_min: 500000.0,
                magi_max: None,
                monthly_surcharge: 487.0,
            },
            ]),
            None => Err(DataError::InvalidParams("lived_with_spouse_during_year parameter is required for married_filing_separately IRMAA lookups".to_string())),
        },
    }
}

pub fn base_part_b_premium() -> f64 {
    202.9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_irmaa_tiers() {
        let b = brackets(FilingStatus::Single, None).unwrap();
        assert_eq!(b.len(), 6);
        assert_eq!(b[0].monthly_surcharge, 0.0);
    }

    #[test]
    fn mfj_irmaa_tiers() {
        let b = brackets(FilingStatus::MarriedFilingJointly, None).unwrap();
        assert_eq!(b.len(), 6);
        assert!(b[0].magi_max.is_some());
    }

    #[test]
    fn mfs_lived_with_spouse_only_three_tiers() {
        let b = brackets(FilingStatus::MarriedFilingSeparately, Some(true)).unwrap();
        assert_eq!(b.len(), 3);
    }

    #[test]
    fn mfs_lived_apart_uses_individual_tiers() {
        let b = brackets(FilingStatus::MarriedFilingSeparately, Some(false)).unwrap();
        assert_eq!(b.len(), 6);
    }

    #[test]
    fn mfs_requires_lived_with_spouse_flag() {
        assert!(brackets(FilingStatus::MarriedFilingSeparately, None).is_err());
    }

    #[test]
    fn base_premium() {
        assert_eq!(base_part_b_premium(), 202.9);
    }
}
