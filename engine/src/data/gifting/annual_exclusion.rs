// Federal gift tax annual exclusion amounts for 2026, reviewed artifact.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GiftAnnualExclusion {
    pub per_donee_exclusion: f64,
    pub non_citizen_spouse_exclusion: f64,
}

pub fn exclusion() -> GiftAnnualExclusion {
    GiftAnnualExclusion {
        per_donee_exclusion: 19000.0,
        non_citizen_spouse_exclusion: 194000.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gift_annual_exclusion_2026() {
        let e = exclusion();
        assert_eq!(e.per_donee_exclusion, 19000.0);
        assert_eq!(e.non_citizen_spouse_exclusion, 194000.0);
    }
}
