/// Social Security full retirement age rule for a birth-year range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FullRetirementAgeRule {
    pub birth_year_min: Option<u32>,
    pub birth_year_max: Option<u32>,
    pub full_retirement_age_years: u32,
    pub full_retirement_age_months: u32,
}

/// Social Security full retirement age rules for retirement and spousal benefits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullRetirementAgeRules {
    pub benefit_scope: &'static str,
    pub january_1_births_use_prior_year: bool,
    pub rules: Vec<FullRetirementAgeRule>,
}

pub fn full_retirement_age_rules() -> FullRetirementAgeRules {
    FullRetirementAgeRules {
        benefit_scope: "retirement_and_spousal",
        january_1_births_use_prior_year: true,
        rules: vec![
            FullRetirementAgeRule {
                birth_year_min: None,
                birth_year_max: Some(1937),
                full_retirement_age_years: 65,
                full_retirement_age_months: 0,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1938),
                birth_year_max: Some(1938),
                full_retirement_age_years: 65,
                full_retirement_age_months: 2,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1939),
                birth_year_max: Some(1939),
                full_retirement_age_years: 65,
                full_retirement_age_months: 4,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1940),
                birth_year_max: Some(1940),
                full_retirement_age_years: 65,
                full_retirement_age_months: 6,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1941),
                birth_year_max: Some(1941),
                full_retirement_age_years: 65,
                full_retirement_age_months: 8,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1942),
                birth_year_max: Some(1942),
                full_retirement_age_years: 65,
                full_retirement_age_months: 10,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1943),
                birth_year_max: Some(1954),
                full_retirement_age_years: 66,
                full_retirement_age_months: 0,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1955),
                birth_year_max: Some(1955),
                full_retirement_age_years: 66,
                full_retirement_age_months: 2,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1956),
                birth_year_max: Some(1956),
                full_retirement_age_years: 66,
                full_retirement_age_months: 4,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1957),
                birth_year_max: Some(1957),
                full_retirement_age_years: 66,
                full_retirement_age_months: 6,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1958),
                birth_year_max: Some(1958),
                full_retirement_age_years: 66,
                full_retirement_age_months: 8,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1959),
                birth_year_max: Some(1959),
                full_retirement_age_years: 66,
                full_retirement_age_months: 10,
            },
            FullRetirementAgeRule {
                birth_year_min: Some(1960),
                birth_year_max: None,
                full_retirement_age_years: 67,
                full_retirement_age_months: 0,
            },
        ],
    }
}

pub fn full_retirement_age_for_birth_year(birth_year: u32) -> Option<(u32, u32)> {
    let rules = full_retirement_age_rules();
    rules.rules.into_iter().find_map(|rule| {
        let min_ok = rule.birth_year_min.is_none_or(|min| birth_year >= min);
        let max_ok = rule.birth_year_max.is_none_or(|max| birth_year <= max);
        if min_ok && max_ok {
            Some((rule.full_retirement_age_years, rule.full_retirement_age_months))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fra_rules_include_january_1_note() {
        let rules = full_retirement_age_rules();
        assert_eq!(rules.january_1_births_use_prior_year, true);
        assert_eq!(rules.benefit_scope, "retirement_and_spousal");
    }

    #[test]
    fn fra_birth_year_1937() {
        assert_eq!(full_retirement_age_for_birth_year(1937), Some((65, 0)));
    }

    #[test]
    fn fra_birth_year_1959() {
        assert_eq!(full_retirement_age_for_birth_year(1959), Some((66, 10)));
    }

    #[test]
    fn fra_birth_year_1960_and_later() {
        assert_eq!(full_retirement_age_for_birth_year(1960), Some((67, 0)));
        assert_eq!(full_retirement_age_for_birth_year(1985), Some((67, 0)));
    }
}
