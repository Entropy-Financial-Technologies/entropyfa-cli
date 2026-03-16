use crate::models::retirement_rmd::{
    AccountRules, BeneficiaryRules, NonDesignatedBeneficiaryRules, Pre1987Rules,
    RequiredBeginningRules, RmdParameters, StartAgeRule, TenYearRule,
};
use std::collections::HashMap;

use super::rmd_tables;

/// Build the full RMD parameter set for 2026, compatible with the existing
/// `RmdParameters` struct in `models/retirement_rmd.rs`.
/// Build the full RMD parameter set for 2026, compatible with the existing
/// `RmdParameters` struct in `models/retirement_rmd.rs`.
pub fn distribution_rules() -> RmdParameters {
    let mut life_expectancy_methods = HashMap::new();
    life_expectancy_methods.insert(
        "chronically_ill".to_string(),
        "single_life_non_recalculated".to_string(),
    );
    life_expectancy_methods.insert(
        "disabled".to_string(),
        "single_life_non_recalculated".to_string(),
    );
    life_expectancy_methods.insert(
        "minor_child".to_string(),
        "single_life_non_recalculated".to_string(),
    );
    life_expectancy_methods.insert(
        "not_more_than_10_years_younger".to_string(),
        "single_life_non_recalculated".to_string(),
    );
    life_expectancy_methods.insert(
        "other_designated_beneficiary".to_string(),
        "ten_year_rule".to_string(),
    );
    life_expectancy_methods.insert("spouse".to_string(), "single_life_recalculated".to_string());

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
                    guidance_status: Some("final".to_string()),
                    notes: Some("Operational 2026 bucket for births in or before 1950. IRS legacy guidance distinguishes people born before July 1, 1949 (age 70½) from people born on or after July 1, 1949 but before January 1, 1951 (age 72); that older cutoff is carried in this note rather than split into a separate public rule row.".to_string()),
                },
                StartAgeRule {
                    birth_year_min: Some(1951),
                    birth_year_max: Some(1958),
                    start_age: 73,
                    guidance_status: Some("final".to_string()),
                    notes: None,
                },
                StartAgeRule {
                    birth_year_min: Some(1959),
                    birth_year_max: Some(1959),
                    start_age: 73,
                    guidance_status: Some("interim_good_faith".to_string()),
                    notes: Some("IRS final regulations (TD 10001, 89 FR 58886) reserved the 1959 cohort; proposed regulations (89 FR 58644) specify applicable age 73. No final IRS resolution for that specific paragraph was found in the cited primary sources as of this extraction, so age 73 is carried here as the best supported proposed-rule reading with interim_good_faith status.".to_string()),
                },
                StartAgeRule {
                    birth_year_min: Some(1960),
                    birth_year_max: None,
                    start_age: 75,
                    guidance_status: Some("final".to_string()),
                    notes: None,
                },
            ],
            first_distribution_deadline: "april_1_following_year".to_string(),
            still_working_exception_plan_categories: vec![
                "401k".to_string(),
                "403b".to_string(),
                "profit_sharing".to_string(),
                "other_defined_contribution_plan".to_string(),
            ],
            still_working_exception_eligible_account_types: vec![
                "401k".to_string(),
                "403b".to_string(),
            ],
            still_working_exception_disallowed_for_five_percent_owners: true,
        },
        account_rules: AccountRules {
            owner_required_account_types: vec![
                "traditional_ira".to_string(),
                "sep_ira".to_string(),
                "simple_ira".to_string(),
                "401k".to_string(),
                "403b".to_string(),
                "457b".to_string(),
            ],
            owner_exempt_account_types: vec![
                "roth_ira".to_string(),
                "designated_roth_plan_account".to_string(),
            ],
            inherited_account_types: vec![
                "inherited_ira".to_string(),
                "inherited_roth_ira".to_string(),
                "inherited_401k".to_string(),
                "inherited_roth_401k".to_string(),
            ],
            supports_pre_1987_403b_exclusion: true,
            designated_roth_owner_exemption_effective_year: Some(2024),
        },
        beneficiary_rules: BeneficiaryRules {
            beneficiary_categories: vec![
                "eligible_designated_beneficiary".to_string(),
                "designated_beneficiary".to_string(),
                "non_designated_beneficiary".to_string(),
            ],
            recognized_beneficiary_classes: vec![
                "spouse".to_string(),
                "minor_child".to_string(),
                "disabled".to_string(),
                "chronically_ill".to_string(),
                "not_more_than_10_years_younger".to_string(),
                "other_designated_beneficiary".to_string(),
                "non_designated_beneficiary".to_string(),
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
            non_designated_beneficiary_rules: NonDesignatedBeneficiaryRules {
                when_owner_died_before_required_beginning_date: "five_year_rule".to_string(),
                when_owner_died_on_or_after_required_beginning_date: "owner_remaining_life_expectancy".to_string(),
            },
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
        assert_eq!(ages.len(), 4);

        // born <= 1950 → 72
        assert_eq!(ages[0].start_age, 72);
        assert_eq!(ages[0].birth_year_max, Some(1950));
        assert_eq!(ages[0].guidance_status.as_deref(), Some("final"));
        assert!(ages[0]
            .notes
            .as_deref()
            .unwrap()
            .contains("before July 1, 1949"));

        // born 1951-1958 → 73
        assert_eq!(ages[1].start_age, 73);
        assert_eq!(ages[1].birth_year_min, Some(1951));
        assert_eq!(ages[1].birth_year_max, Some(1958));
        assert_eq!(ages[1].guidance_status.as_deref(), Some("final"));

        // born 1959 → 73 under interim good-faith guidance
        assert_eq!(ages[2].start_age, 73);
        assert_eq!(ages[2].birth_year_min, Some(1959));
        assert_eq!(ages[2].birth_year_max, Some(1959));
        assert_eq!(
            ages[2].guidance_status.as_deref(),
            Some("interim_good_faith")
        );
        assert!(ages[2]
            .notes
            .as_deref()
            .unwrap()
            .contains("reserved the 1959 cohort"));

        // born >= 1960 → 75
        assert_eq!(ages[3].start_age, 75);
        assert_eq!(ages[3].birth_year_min, Some(1960));
        assert_eq!(ages[3].guidance_status.as_deref(), Some("final"));
    }

    #[test]
    fn still_working_exception() {
        let rules = distribution_rules();
        let plan_categories = &rules
            .required_beginning
            .still_working_exception_plan_categories;
        let account_types = &rules
            .required_beginning
            .still_working_exception_eligible_account_types;
        assert!(plan_categories.contains(&"401k".to_string()));
        assert!(plan_categories.contains(&"profit_sharing".to_string()));
        assert!(account_types.contains(&"401k".to_string()));
        assert!(account_types.contains(&"403b".to_string()));
        assert!(!account_types.contains(&"traditional_ira".to_string()));
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
        let categories = &rules.beneficiary_rules.beneficiary_categories;
        let edb = &rules
            .beneficiary_rules
            .eligible_designated_beneficiary_classes;
        let recognized = &rules.beneficiary_rules.recognized_beneficiary_classes;
        assert_eq!(categories.len(), 3);
        assert!(categories.contains(&"designated_beneficiary".to_string()));
        assert!(recognized.contains(&"other_designated_beneficiary".to_string()));
        assert!(recognized.contains(&"non_designated_beneficiary".to_string()));
        assert_eq!(edb.len(), 5);
        assert!(edb.contains(&"spouse".to_string()));
        assert!(edb.contains(&"disabled".to_string()));
    }

    #[test]
    fn designated_roth_owner_exemption_effective_year() {
        let rules = distribution_rules();
        assert!(!rules
            .account_rules
            .owner_required_account_types
            .contains(&"designated_roth_plan_account".to_string()));
        assert!(rules
            .account_rules
            .owner_exempt_account_types
            .contains(&"designated_roth_plan_account".to_string()));
        assert_eq!(
            rules
                .account_rules
                .designated_roth_owner_exemption_effective_year,
            Some(2024)
        );
    }
}
