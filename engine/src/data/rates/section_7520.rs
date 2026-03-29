// IRS §7520 interest rates by month, reviewed artifacts.
// Rate = 120% of mid-term AFR (annual compounding), rounded to nearest 0.2%.

// BEGIN SECTION 7520 MONTHS

pub fn rate_2026_01() -> f64 {
    4.6
}

pub fn rate_2026_02() -> f64 {
    4.6
}

pub fn rate_2026_03() -> f64 {
    4.8
}

// END SECTION 7520 MONTHS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_7520_2026_01_rate() {
        assert_eq!(rate_2026_01(), 4.6);
    }

    #[test]
    fn section_7520_2026_02_rate() {
        assert_eq!(rate_2026_02(), 4.6);
    }

    #[test]
    fn section_7520_2026_03_rate() {
        assert_eq!(rate_2026_03(), 4.8);
    }
}
