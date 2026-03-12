use crate::models::retirement_rmd::{
    AccountRules, BeneficiaryRules, Pre1987Rules, RequiredBeginningRules, RmdParameters,
    StartAgeRule, TenYearRule,
};
use std::collections::HashMap;

use super::rmd_tables;

/// Build the full RMD parameter set for 2026, compatible with the existing
/// `RmdParameters` struct in `models/retirement_rmd.rs`.
pub fn distribution_rules() -> RmdParameters {
    let mut life_expectancy_methods = HashMap::new();
    life_expectancy_methods.insert("spouse".to_string(), "single_life_recalculated".to_string());
    life_expectancy_methods.insert(
        "minor_child".to_string(),
        "single_life_non_recalculated".to_string(),
    );
    life_expectancy_methods.insert(
        "disabled".to_string(),
        "single_life_non_recalculated".to_string(),
    );
    life_expectancy_methods.insert(
        "chronically_ill".to_string(),
        "single_life_non_recalculated".to_string(),
    );
    life_expectancy_methods.insert(
        "not_more_than_10_years_younger".to_string(),
        "single_life_non_recalculated".to_string(),
    );
    life_expectancy_methods.insert(
        "designated_beneficiary".to_string(),
        "ten_year_rule".to_string(),
    );

    RmdParameters {
        uniform_lifetime_table: rmd_tables::uniform_lifetime(),
        joint_life_table: rmd_tables::joint_life(),
        single_life_table: rmd_tables::single_life(),
        required_beginning: RequiredBeginningRules {
            start_age_rules: vec![
                StartAgeRule {
                    birth_year_min: None,
                    birth_year_max: Some(1950),
                    start_age: 72,
                },
                StartAgeRule {
                    birth_year_min: Some(1951),
                    birth_year_max: Some(1959),
                    start_age: 73,
                },
                StartAgeRule {
                    birth_year_min: Some(1960),
                    birth_year_max: None,
                    start_age: 75,
                },
            ],
            first_distribution_deadline: "april_1_following_year".to_string(),
            still_working_exception_account_types: vec!["401k".to_string(), "403b".to_string()],
            still_working_exception_disallowed_for_five_percent_owners: true,
            designated_roth_owner_rmd_start_year: Some(2024),
        },
        account_rules: AccountRules {
            owner_required_account_types: vec![
                "traditional_ira".to_string(),
                "401k".to_string(),
                "403b".to_string(),
                "457b".to_string(),
            ],
            owner_exempt_account_types: vec!["roth_ira".to_string()],
            inherited_account_types: vec![
                "inherited_ira".to_string(),
                "inherited_roth_ira".to_string(),
                "inherited_401k".to_string(),
                "inherited_roth_401k".to_string(),
            ],
            supports_pre_1987_403b_exclusion: true,
        },
        beneficiary_rules: BeneficiaryRules {
            beneficiary_classes: vec![
                "spouse".to_string(),
                "minor_child".to_string(),
                "disabled".to_string(),
                "chronically_ill".to_string(),
                "not_more_than_10_years_younger".to_string(),
                "designated_beneficiary".to_string(),
            ],
            eligible_designated_beneficiary_classes: vec![
                "spouse".to_string(),
                "minor_child".to_string(),
                "disabled".to_string(),
                "chronically_ill".to_string(),
                "not_more_than_10_years_younger".to_string(),
            ],
            life_expectancy_method_by_class: life_expectancy_methods,
            minor_child_majority_age: 21,
            spouse_delay_allowed: true,
        },
        ten_year_rule: TenYearRule {
            terminal_year: 10,
            annual_distributions_required_when_owner_died_on_or_after_rbd: true,
        },
        relief_years: vec![2021, 2022, 2023, 2024],
        pre_1987_403b_rules: Pre1987Rules {
            exclude_until_age: 75,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distribution_rules_required_beginning_ages() {
        let rules = distribution_rules();
        let ages = &rules.required_beginning.start_age_rules;
        assert_eq!(ages.len(), 3);

        // born <= 1950 → 72
        assert_eq!(ages[0].start_age, 72);
        assert_eq!(ages[0].birth_year_max, Some(1950));

        // born 1951-1959 → 73
        assert_eq!(ages[1].start_age, 73);
        assert_eq!(ages[1].birth_year_min, Some(1951));
        assert_eq!(ages[1].birth_year_max, Some(1959));

        // born >= 1960 → 75
        assert_eq!(ages[2].start_age, 75);
        assert_eq!(ages[2].birth_year_min, Some(1960));
    }

    #[test]
    fn still_working_exception() {
        let rules = distribution_rules();
        let acct_types = &rules
            .required_beginning
            .still_working_exception_account_types;
        assert!(acct_types.contains(&"401k".to_string()));
        assert!(acct_types.contains(&"403b".to_string()));
        assert!(!acct_types.contains(&"traditional_ira".to_string()));
    }

    #[test]
    fn ten_year_rule() {
        let rules = distribution_rules();
        assert_eq!(rules.ten_year_rule.terminal_year, 10);
        assert!(
            rules
                .ten_year_rule
                .annual_distributions_required_when_owner_died_on_or_after_rbd
        );
    }

    #[test]
    fn relief_years() {
        let rules = distribution_rules();
        assert_eq!(rules.relief_years, vec![2021, 2022, 2023, 2024]);
    }

    #[test]
    fn edb_classes() {
        let rules = distribution_rules();
        let edb = &rules
            .beneficiary_rules
            .eligible_designated_beneficiary_classes;
        assert_eq!(edb.len(), 5);
        assert!(edb.contains(&"spouse".to_string()));
        assert!(edb.contains(&"disabled".to_string()));
    }

    #[test]
    fn designated_roth_rmd_eliminated() {
        let rules = distribution_rules();
        assert_eq!(
            rules
                .required_beginning
                .designated_roth_owner_rmd_start_year,
            Some(2024)
        );
    }
}
