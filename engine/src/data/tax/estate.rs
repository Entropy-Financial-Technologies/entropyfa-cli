use crate::models::tax_request::TaxBracket;

// ---------------------------------------------------------------------------
// Federal estate tax (2026)
// ---------------------------------------------------------------------------

/// Basic exclusion amount (exemption) for 2026, reviewed artifact.
pub fn exemption() -> f64 {
    15000000.0
}

/// Applicable credit amount for 2026.
/// Applicable credit amount for 2026, reviewed artifact.
pub fn applicable_credit() -> f64 {
    5945800.0
}

pub fn brackets() -> Vec<TaxBracket> {
    vec![
        TaxBracket {
            min: 0.0,
            max: Some(10000.0),
            rate: 0.18,
        },
        TaxBracket {
            min: 10000.0,
            max: Some(20000.0),
            rate: 0.2,
        },
        TaxBracket {
            min: 20000.0,
            max: Some(40000.0),
            rate: 0.22,
        },
        TaxBracket {
            min: 40000.0,
            max: Some(60000.0),
            rate: 0.24,
        },
        TaxBracket {
            min: 60000.0,
            max: Some(80000.0),
            rate: 0.26,
        },
        TaxBracket {
            min: 80000.0,
            max: Some(100000.0),
            rate: 0.28,
        },
        TaxBracket {
            min: 100000.0,
            max: Some(150000.0),
            rate: 0.3,
        },
        TaxBracket {
            min: 150000.0,
            max: Some(250000.0),
            rate: 0.32,
        },
        TaxBracket {
            min: 250000.0,
            max: Some(500000.0),
            rate: 0.34,
        },
        TaxBracket {
            min: 500000.0,
            max: Some(750000.0),
            rate: 0.37,
        },
        TaxBracket {
            min: 750000.0,
            max: Some(1000000.0),
            rate: 0.39,
        },
        TaxBracket {
            min: 1000000.0,
            max: None,
            rate: 0.4,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemption_2026() {
        assert_eq!(exemption(), 15000000.0);
    }

    #[test]
    fn applicable_credit_2026() {
        assert_eq!(applicable_credit(), 5945800.0);
    }

    #[test]
    fn bracket_count() {
        let b = brackets();
        assert_eq!(b.len(), 12);
        assert_eq!(b[0].rate, 0.18);
        assert_eq!(b[b.len() - 1].rate, 0.4);
        assert_eq!(b[b.len() - 1].max, None);
    }

    #[test]
    fn brackets_contiguous() {
        let b = brackets();
        for i in 1..b.len() {
            assert_eq!(b[i].min, b[i - 1].max.unwrap());
        }
    }
}
