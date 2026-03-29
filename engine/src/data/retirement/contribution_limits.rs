// Federal retirement plan contribution limits for 2026, reviewed artifact.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContributionLimits {
    pub elective_deferral_401k: f64,
    pub catch_up_401k_50_plus: f64,
    pub catch_up_401k_60_to_63: f64,
    pub ira_contribution_limit: f64,
    pub ira_catch_up_50_plus: f64,
    pub simple_elective_deferral: f64,
    pub simple_catch_up_50_plus: f64,
    pub simple_catch_up_60_to_63: f64,
    pub sep_maximum_contribution: f64,
    pub sep_minimum_compensation: f64,
    pub annual_additions_limit_415c: f64,
    pub annual_compensation_limit: f64,
    pub defined_benefit_limit: f64,
    pub highly_compensated_threshold: f64,
    pub key_employee_threshold: f64,
}

pub fn limits() -> ContributionLimits {
    ContributionLimits {
        elective_deferral_401k: 24500.0,
        catch_up_401k_50_plus: 8000.0,
        catch_up_401k_60_to_63: 11250.0,
        ira_contribution_limit: 7500.0,
        ira_catch_up_50_plus: 1100.0,
        simple_elective_deferral: 17000.0,
        simple_catch_up_50_plus: 4000.0,
        simple_catch_up_60_to_63: 5250.0,
        sep_maximum_contribution: 72000.0,
        sep_minimum_compensation: 800.0,
        annual_additions_limit_415c: 72000.0,
        annual_compensation_limit: 360000.0,
        defined_benefit_limit: 290000.0,
        highly_compensated_threshold: 160000.0,
        key_employee_threshold: 235000.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contribution_limits_2026() {
        let l = limits();
        assert_eq!(l.elective_deferral_401k, 24500.0);
        assert_eq!(l.catch_up_401k_50_plus, 8000.0);
        assert_eq!(l.catch_up_401k_60_to_63, 11250.0);
        assert_eq!(l.ira_contribution_limit, 7500.0);
        assert_eq!(l.ira_catch_up_50_plus, 1100.0);
        assert_eq!(l.simple_elective_deferral, 17000.0);
        assert_eq!(l.simple_catch_up_50_plus, 4000.0);
        assert_eq!(l.simple_catch_up_60_to_63, 5250.0);
        assert_eq!(l.sep_maximum_contribution, 72000.0);
        assert_eq!(l.sep_minimum_compensation, 800.0);
        assert_eq!(l.annual_additions_limit_415c, 72000.0);
        assert_eq!(l.annual_compensation_limit, 360000.0);
        assert_eq!(l.defined_benefit_limit, 290000.0);
        assert_eq!(l.highly_compensated_threshold, 160000.0);
        assert_eq!(l.key_employee_threshold, 235000.0);
    }
}
