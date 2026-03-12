use crate::models::tax_request::TaxBracket;

// ---------------------------------------------------------------------------
// Federal estate tax (2026)
// ---------------------------------------------------------------------------

/// Basic exclusion amount (exemption) for 2026.
pub fn exemption() -> f64 {
    15_000_000.0
}

/// Applicable credit amount for 2026.
pub fn applicable_credit() -> f64 {
    5_945_800.0
}

/// Graduated estate tax brackets (statutory, unchanged since 2013).
pub fn brackets() -> Vec<TaxBracket> {
    vec![
        TaxBracket {
            min: 0.0,
            max: Some(10_000.0),
            rate: 0.18,
        },
        TaxBracket {
            min: 10_000.0,
            max: Some(20_000.0),
            rate: 0.20,
        },
        TaxBracket {
            min: 20_000.0,
            max: Some(40_000.0),
            rate: 0.22,
        },
        TaxBracket {
            min: 40_000.0,
            max: Some(60_000.0),
            rate: 0.24,
        },
        TaxBracket {
            min: 60_000.0,
            max: Some(80_000.0),
            rate: 0.26,
        },
        TaxBracket {
            min: 80_000.0,
            max: Some(100_000.0),
            rate: 0.28,
        },
        TaxBracket {
            min: 100_000.0,
            max: Some(150_000.0),
            rate: 0.30,
        },
        TaxBracket {
            min: 150_000.0,
            max: Some(250_000.0),
            rate: 0.32,
        },
        TaxBracket {
            min: 250_000.0,
            max: Some(500_000.0),
            rate: 0.34,
        },
        TaxBracket {
            min: 500_000.0,
            max: Some(750_000.0),
            rate: 0.37,
        },
        TaxBracket {
            min: 750_000.0,
            max: Some(1_000_000.0),
            rate: 0.39,
        },
        TaxBracket {
            min: 1_000_000.0,
            max: None,
            rate: 0.40,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemption_2026() {
        assert_eq!(exemption(), 15_000_000.0);
    }

    #[test]
    fn applicable_credit_2026() {
        assert_eq!(applicable_credit(), 5_945_800.0);
    }

    #[test]
    fn bracket_count() {
        let b = brackets();
        assert_eq!(b.len(), 12);
        assert_eq!(b[0].rate, 0.18);
        assert_eq!(b[11].rate, 0.40);
        assert_eq!(b[11].max, None);
    }

    #[test]
    fn brackets_contiguous() {
        let b = brackets();
        for i in 1..b.len() {
            assert_eq!(b[i].min, b[i - 1].max.unwrap());
        }
    }
}
