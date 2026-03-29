// HSA contribution limits and HDHP thresholds for 2026, reviewed artifact.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsaLimits {
    pub hsa_contribution_self_only: f64,
    pub hsa_contribution_family: f64,
    pub hsa_catch_up_55_plus: f64,
    pub hdhp_min_deductible_self_only: f64,
    pub hdhp_min_deductible_family: f64,
    pub hdhp_max_out_of_pocket_self_only: f64,
    pub hdhp_max_out_of_pocket_family: f64,
}

pub fn limits() -> HsaLimits {
    HsaLimits {
        hsa_contribution_self_only: 4400.0,
        hsa_contribution_family: 8750.0,
        hsa_catch_up_55_plus: 1000.0,
        hdhp_min_deductible_self_only: 1700.0,
        hdhp_min_deductible_family: 3400.0,
        hdhp_max_out_of_pocket_self_only: 8500.0,
        hdhp_max_out_of_pocket_family: 17000.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsa_limits_2026() {
        let l = limits();
        assert_eq!(l.hsa_contribution_self_only, 4400.0);
        assert_eq!(l.hsa_contribution_family, 8750.0);
        assert_eq!(l.hsa_catch_up_55_plus, 1000.0);
        assert_eq!(l.hdhp_min_deductible_self_only, 1700.0);
        assert_eq!(l.hdhp_min_deductible_family, 3400.0);
        assert_eq!(l.hdhp_max_out_of_pocket_self_only, 8500.0);
        assert_eq!(l.hdhp_max_out_of_pocket_family, 17000.0);
    }
}
