use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct RetirementRmdRequest {
    pub calculation_year: u32,
    pub prior_year_end_balance: f64,
    pub account_type: String,

    pub owner_birth_date: Option<String>,
    pub owner_is_alive: Option<bool>,
    pub owner_death_year: Option<u32>,
    pub owner_died_before_required_beginning_date: Option<bool>,

    pub beneficiary_birth_date: Option<String>,
    pub beneficiary_class: Option<String>,
    pub beneficiary_election: Option<String>,
    pub beneficiary_majority_year: Option<u32>,

    pub spouse_birth_date: Option<String>,
    pub spouse_is_sole_beneficiary: Option<bool>,

    pub is_still_working: Option<bool>,
    pub is_five_percent_owner: Option<bool>,
    pub pre_1987_403b_balance: Option<f64>,

    pub rmd_parameters: RmdParameters,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RetirementRmdScheduleRequest {
    pub calculation_year: u32,
    pub prior_year_end_balance: f64,
    pub account_type: String,

    pub owner_birth_date: Option<String>,
    pub owner_is_alive: Option<bool>,
    pub owner_death_year: Option<u32>,
    pub owner_died_before_required_beginning_date: Option<bool>,

    pub beneficiary_birth_date: Option<String>,
    pub beneficiary_class: Option<String>,
    pub beneficiary_election: Option<String>,
    pub beneficiary_majority_year: Option<u32>,

    pub spouse_birth_date: Option<String>,
    pub spouse_is_sole_beneficiary: Option<bool>,

    pub is_still_working: Option<bool>,
    pub is_five_percent_owner: Option<bool>,
    pub pre_1987_403b_balance: Option<f64>,

    pub annual_growth_rate: Option<f64>,
    pub end_age: Option<u32>,
    pub max_years: Option<u32>,

    pub rmd_parameters: RmdParameters,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RmdParameters {
    pub uniform_lifetime_table: Vec<AgeDistributionPeriod>,
    pub joint_life_table: Vec<JointDistributionPeriod>,
    pub single_life_table: Vec<AgeDistributionPeriod>,
    pub required_beginning: RequiredBeginningRules,
    pub account_rules: AccountRules,
    pub beneficiary_rules: BeneficiaryRules,
    pub ten_year_rule: TenYearRule,
    #[serde(default)]
    pub relief_years: Vec<u32>,
    pub pre_1987_403b_rules: Pre1987Rules,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgeDistributionPeriod {
    pub age: u32,
    pub distribution_period: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JointDistributionPeriod {
    pub owner_age: u32,
    pub spouse_age: u32,
    pub distribution_period: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RequiredBeginningRules {
    pub start_age_rules: Vec<StartAgeRule>,
    pub first_distribution_deadline: String,
    #[serde(default)]
    pub still_working_exception_plan_categories: Vec<String>,
    #[serde(default)]
    pub still_working_exception_eligible_account_types: Vec<String>,
    pub still_working_exception_disallowed_for_five_percent_owners: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StartAgeRule {
    pub birth_year_min: Option<u32>,
    pub birth_year_max: Option<u32>,
    pub start_age: u32,
    #[serde(default)]
    pub guidance_status: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AccountRules {
    #[serde(default)]
    pub owner_required_account_types: Vec<String>,
    #[serde(default)]
    pub owner_exempt_account_types: Vec<String>,
    #[serde(default)]
    pub inherited_account_types: Vec<String>,
    pub supports_pre_1987_403b_exclusion: bool,
    #[serde(default)]
    pub designated_roth_owner_exemption_effective_year: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BeneficiaryRules {
    #[serde(default)]
    pub beneficiary_categories: Vec<String>,
    #[serde(default)]
    pub recognized_beneficiary_classes: Vec<String>,
    #[serde(default)]
    pub eligible_designated_beneficiary_classes: Vec<String>,
    #[serde(default)]
    pub life_expectancy_method_by_class: std::collections::HashMap<String, String>,
    pub minor_child_majority_age: u32,
    pub spouse_delay_allowed: bool,
    pub non_designated_beneficiary_rules: NonDesignatedBeneficiaryRules,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NonDesignatedBeneficiaryRules {
    pub when_owner_died_before_required_beginning_date: String,
    pub when_owner_died_on_or_after_required_beginning_date: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TenYearRule {
    pub terminal_year: u32,
    pub annual_distributions_required_when_owner_died_on_or_after_rbd: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Pre1987Rules {
    pub exclude_until_age: u32,
}

#[derive(Debug, Serialize)]
pub struct RetirementRmdResponse {
    pub calculation_year: u32,
    pub scenario_class: String,
    pub rmd_required: bool,
    pub rmd_amount: f64,
    pub applicable_balance: f64,
    pub distribution_period: Option<f64>,
    pub table_used: Option<String>,
    pub rule_path: String,
    pub decision_trace: Vec<String>,
    pub citations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RetirementRmdScheduleResponse {
    pub start_year: u32,
    pub end_year: u32,
    pub annual_growth_rate: f64,
    pub rows: Vec<RetirementRmdScheduleRow>,
    pub projection_convention: String,
}

#[derive(Debug, Serialize)]
pub struct RetirementRmdScheduleRow {
    pub year: u32,
    pub age: Option<u32>,
    pub start_balance: f64,
    pub applicable_balance: f64,
    pub rmd_required: bool,
    pub distribution_period: Option<f64>,
    pub rmd_amount: f64,
    pub end_balance: f64,
    pub rule_path: String,
}
