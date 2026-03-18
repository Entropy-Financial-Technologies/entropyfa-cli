/// Medicare base premiums and deductible for 2026, reviewed artifact.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MedicareBasePremiums {
    pub part_b_standard_monthly_premium: f64,
    pub part_b_annual_deductible: f64,
    pub part_d_base_beneficiary_premium: f64,
}

pub fn base_premiums() -> MedicareBasePremiums {
    MedicareBasePremiums {
        part_b_standard_monthly_premium: 202.9,
        part_b_annual_deductible: 283.0,
        part_d_base_beneficiary_premium: 38.99,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medicare_base_premiums_2026() {
        let premiums = base_premiums();
        assert_eq!(premiums.part_b_standard_monthly_premium, 202.9);
        assert_eq!(premiums.part_b_annual_deductible, 283.0);
        assert_eq!(premiums.part_d_base_beneficiary_premium, 38.99);
    }
}
