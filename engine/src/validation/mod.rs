use crate::models::estate_tax_request::EstateTaxRequest;
use crate::models::pension_request::PensionComparisonRequest;
use crate::models::retirement_rmd::{RetirementRmdRequest, RetirementRmdScheduleRequest};
use crate::models::roth_conversion::{RothConversionRequest, RothConversionStrategyRequest};
use crate::models::simulation_request::SimulationRequest;
use crate::models::solver_request::SolverRequest;
use crate::models::tax_request::{DeductionConfig, FederalTaxRequest, TaxParameters};
use std::collections::HashSet;

pub fn validate_simulation_request(req: &SimulationRequest) -> Vec<String> {
    let mut errors = validate_simulation_request_contract(req);

    if !req.buckets.is_empty() {
        errors.push(
            "bucketed simulation is not yet supported by the current execution engine".into(),
        );
    }

    errors
}

pub(crate) fn validate_simulation_request_contract(req: &SimulationRequest) -> Vec<String> {
    let mut errors = Vec::new();

    if req.buckets.is_empty() {
        if req.starting_balance.is_none() {
            errors.push("starting_balance is required for legacy requests".into());
        }
        if req.return_assumption.is_none() {
            errors.push("return_assumption is required for legacy requests".into());
        }
    }

    if let Some(starting_balance) = req.starting_balance {
        if starting_balance < 0.0 {
            errors.push("starting_balance must be non-negative".into());
        }
    }

    if req.time_horizon_months == 0 {
        errors.push("time_horizon_months must be greater than 0".into());
    }

    if req.time_horizon_months > 1200 {
        errors.push("time_horizon_months must be at most 1200 (100 years)".into());
    }

    if let Some(return_assumption) = req.return_assumption.as_ref() {
        if return_assumption.annual_std_dev < 0.0 {
            errors.push("annual_std_dev must be non-negative".into());
        }
    }

    let known_bucket_ids: HashSet<&str> = req
        .buckets
        .iter()
        .map(|bucket| bucket.id.as_str())
        .collect();
    let mut seen_bucket_ids = HashSet::new();

    for (i, bucket) in req.buckets.iter().enumerate() {
        if !seen_bucket_ids.insert(bucket.id.as_str()) {
            errors.push(format!(
                "buckets[{}].id '{}' is duplicate; bucket IDs must be unique",
                i, bucket.id
            ));
        }

        if bucket.starting_balance < 0.0 {
            errors.push(format!(
                "buckets[{}].starting_balance must be non-negative",
                i
            ));
        }

        if bucket.return_assumption.annual_std_dev < 0.0 {
            errors.push(format!(
                "buckets[{}].return_assumption.annual_std_dev must be non-negative",
                i
            ));
        }

        if !["taxable", "tax_deferred", "tax_free", "cash"].contains(&bucket.bucket_type.as_str()) {
            errors.push(format!(
                "buckets[{}].bucket_type must be one of: taxable, tax_deferred, tax_free, cash",
                i
            ));
        }

        if let Some(realized_gain_ratio) = bucket.realized_gain_ratio {
            if !(0.0..=1.0).contains(&realized_gain_ratio) {
                errors.push(format!(
                    "buckets[{}].realized_gain_ratio must be between 0.0 and 1.0",
                    i
                ));
            }
        }
    }

    if let Some(spending_policy) = req.spending_policy.as_ref() {
        for (i, bucket_id) in spending_policy.withdrawal_order.iter().enumerate() {
            if !known_bucket_ids.contains(bucket_id.as_str()) {
                errors.push(format!(
                    "spending_policy.withdrawal_order[{}] references unknown bucket id '{}'",
                    i, bucket_id
                ));
            }
        }

        if let Some(bucket_id) = spending_policy.rebalance_tax_withholding_from.as_deref() {
            if !known_bucket_ids.contains(bucket_id) {
                errors.push(format!(
                    "spending_policy.rebalance_tax_withholding_from references unknown bucket id '{}'",
                    bucket_id
                ));
            }
        }
    }

    if let Some(tax_policy) = req.tax_policy.as_ref() {
        if let Some(rate) = tax_policy.modeled_tax_inflation_rate {
            if rate < 0.0 {
                errors.push("tax_policy.modeled_tax_inflation_rate must be non-negative".into());
            }
        }
    }

    if let Some(rmd_policy) = req.rmd_policy.as_ref() {
        if let Some(month) = rmd_policy.distribution_month {
            if !(1..=12).contains(&month) {
                errors.push("rmd_policy.distribution_month must be between 1 and 12".into());
            }
        }
    }

    let num_sims = req.num_simulations.unwrap_or(10000);
    if num_sims == 0 {
        errors.push("num_simulations must be greater than 0".into());
    }
    if num_sims > 100_000 {
        errors.push("num_simulations must be at most 100000".into());
    }

    for (i, cf) in req.cash_flows.iter().enumerate() {
        if cf.start_month.unwrap_or(0) >= req.time_horizon_months {
            errors.push(format!(
                "cash_flows[{}].start_month must be less than time_horizon_months",
                i
            ));
        }
        if let Some(end) = cf.end_month {
            if end > req.time_horizon_months {
                errors.push(format!(
                    "cash_flows[{}].end_month must be at most time_horizon_months",
                    i
                ));
            }
            if end <= cf.start_month.unwrap_or(0) {
                errors.push(format!(
                    "cash_flows[{}].end_month must be greater than start_month",
                    i
                ));
            }
        }
        if cf.amount == 0.0 {
            errors.push(format!("cash_flows[{}].amount must be non-zero", i));
        }
    }

    let mode = req.mode.as_deref().unwrap_or("both");
    if !["monte_carlo", "linear", "both"].contains(&mode) {
        errors.push(format!(
            "mode must be one of: monte_carlo, linear, both (got '{}')",
            mode
        ));
    }

    if req.sample_paths.is_some() && req.path_indices.is_some() {
        errors.push("cannot specify both sample_paths and path_indices".into());
    }

    if let Some(count) = req.sample_paths {
        if count > num_sims as usize {
            errors.push(format!(
                "sample_paths ({}) must be at most num_simulations ({})",
                count, num_sims
            ));
        }
    }

    if let Some(ref indices) = req.path_indices {
        for (i, &idx) in indices.iter().enumerate() {
            if idx >= num_sims as usize {
                errors.push(format!(
                    "path_indices[{}] ({}) must be less than num_simulations ({})",
                    i, idx, num_sims
                ));
            }
        }
    }

    if let Some(ref pcts) = req.custom_percentiles {
        for (i, &p) in pcts.iter().enumerate() {
            if p > 100 {
                errors.push(format!(
                    "custom_percentiles[{}] ({}) must be between 0 and 100",
                    i, p
                ));
            }
        }
    }

    errors
}

pub fn validate_solver_request(req: &SolverRequest) -> Vec<String> {
    let mut errors = Vec::new();

    let valid_variables = [
        "starting_balance",
        "time_horizon_months",
        "annual_return",
        "cash_flow_amount",
    ];
    if !valid_variables.contains(&req.solve_for.variable.as_str()) {
        errors.push(format!(
            "solve_for.variable must be one of: {} (got '{}')",
            valid_variables.join(", "),
            req.solve_for.variable
        ));
    }

    if req.solve_for.variable == "cash_flow_amount" {
        let idx = req.solve_for.cash_flow_index.unwrap_or(0);
        if idx >= req.cash_flows.len() {
            errors.push(format!(
                "solve_for.cash_flow_index {} is out of range (cash_flows has {} entries)",
                idx,
                req.cash_flows.len()
            ));
        }
    }

    let valid_metrics = ["success_rate", "percentile_balance"];
    if !valid_metrics.contains(&req.target.metric.as_str()) {
        errors.push(format!(
            "target.metric must be one of: {} (got '{}')",
            valid_metrics.join(", "),
            req.target.metric
        ));
    }

    if req.target.metric == "success_rate" && !(0.0..=1.0).contains(&req.target.value) {
        errors.push("target.value for success_rate must be between 0.0 and 1.0".into());
    }

    if req.target.metric == "percentile_balance" {
        let valid_percentiles = ["p5", "p10", "p25", "p50", "p75", "p90", "p95"];
        match &req.target.percentile {
            None => {
                errors
                    .push("target.percentile is required when metric is percentile_balance".into());
            }
            Some(p) => {
                if !valid_percentiles.contains(&p.as_str()) {
                    errors.push(format!(
                        "target.percentile must be one of: {} (got '{}')",
                        valid_percentiles.join(", "),
                        p
                    ));
                }
            }
        }
    }

    if let Some(ref bounds) = req.bounds {
        if bounds.lower >= bounds.upper {
            errors.push("bounds.lower must be less than bounds.upper".into());
        }
    }

    // Validate non-solved simulation params
    if req.solve_for.variable != "starting_balance" && req.starting_balance < 0.0 {
        errors.push("starting_balance must be non-negative".into());
    }

    if req.solve_for.variable != "time_horizon_months" {
        if req.time_horizon_months == 0 {
            errors.push("time_horizon_months must be greater than 0".into());
        }
        if req.time_horizon_months > 1200 {
            errors.push("time_horizon_months must be at most 1200 (100 years)".into());
        }
    }

    if req.return_assumption.annual_std_dev < 0.0 {
        errors.push("annual_std_dev must be non-negative".into());
    }

    let num_sims = req.num_simulations.unwrap_or(10000);
    if num_sims == 0 {
        errors.push("num_simulations must be greater than 0".into());
    }
    if num_sims > 100_000 {
        errors.push("num_simulations must be at most 100000".into());
    }

    let mode = req.mode.as_deref().unwrap_or("both");
    if !["monte_carlo", "linear", "both"].contains(&mode) {
        errors.push(format!(
            "mode must be one of: monte_carlo, linear, both (got '{}')",
            mode
        ));
    }

    errors
}

pub fn validate_federal_tax_request(req: &FederalTaxRequest) -> Vec<String> {
    let mut errors = Vec::new();

    let valid_statuses = [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ];
    if !valid_statuses.contains(&req.filing_status.as_str()) {
        errors.push(format!(
            "filing_status must be one of: {} (got '{}')",
            valid_statuses.join(", "),
            req.filing_status
        ));
    }

    // Income fields that must be >= 0
    let inc = &req.income;
    if inc.wages < 0.0 {
        errors.push("income.wages must be non-negative".into());
    }
    if inc.taxable_interest < 0.0 {
        errors.push("income.taxable_interest must be non-negative".into());
    }
    if inc.tax_exempt_interest < 0.0 {
        errors.push("income.tax_exempt_interest must be non-negative".into());
    }
    if inc.ordinary_dividends < 0.0 {
        errors.push("income.ordinary_dividends must be non-negative".into());
    }
    if inc.qualified_dividends < 0.0 {
        errors.push("income.qualified_dividends must be non-negative".into());
    }
    if inc.taxable_ira_distributions < 0.0 {
        errors.push("income.taxable_ira_distributions must be non-negative".into());
    }
    if inc.taxable_pensions < 0.0 {
        errors.push("income.taxable_pensions must be non-negative".into());
    }
    if inc.taxable_social_security < 0.0 {
        errors.push("income.taxable_social_security must be non-negative".into());
    }
    // self_employment_income, short_term_capital_gains, long_term_capital_gains, other_income can be negative

    let valid_methods = ["standard", "itemized"];
    if !valid_methods.contains(&req.deductions.method.as_str()) {
        errors.push(format!(
            "deductions.method must be 'standard' or 'itemized' (got '{}')",
            req.deductions.method
        ));
    }

    validate_deductions_and_salt(
        &req.filing_status,
        &req.deductions,
        &req.tax_parameters,
        &mut errors,
    );

    if req.tax_parameters.ordinary_brackets.is_empty() {
        errors.push("tax_parameters.ordinary_brackets must not be empty".into());
    }
    if req.tax_parameters.capital_gains_brackets.is_empty() {
        errors.push("tax_parameters.capital_gains_brackets must not be empty".into());
    }

    validate_deductions_and_salt(
        &req.filing_status,
        &req.deductions,
        &req.tax_parameters,
        &mut errors,
    );

    for (i, bracket) in req.tax_parameters.ordinary_brackets.iter().enumerate() {
        if bracket.rate < 0.0 || bracket.rate > 1.0 {
            errors.push(format!(
                "tax_parameters.ordinary_brackets[{}].rate must be between 0.0 and 1.0",
                i
            ));
        }
    }
    for (i, bracket) in req.tax_parameters.capital_gains_brackets.iter().enumerate() {
        if bracket.rate < 0.0 || bracket.rate > 1.0 {
            errors.push(format!(
                "tax_parameters.capital_gains_brackets[{}].rate must be between 0.0 and 1.0",
                i
            ));
        }
    }

    if req.tax_parameters.standard_deduction < 0.0 {
        errors.push("tax_parameters.standard_deduction must be non-negative".into());
    }
    if req.tax_parameters.capital_loss_limit < 0.0 {
        errors.push("tax_parameters.capital_loss_limit must be non-negative".into());
    }

    errors
}

fn validate_deductions_and_salt(
    filing_status: &str,
    deductions: &DeductionConfig,
    tax_parameters: &TaxParameters,
    errors: &mut Vec<String>,
) {
    let force_itemized =
        filing_status == "married_filing_separately" && deductions.spouse_itemizes == Some(true);
    let has_detailed = deductions.has_detailed_itemized_inputs();

    if deductions.itemized_amount.is_some() && has_detailed {
        errors.push(
            "deductions.itemized_amount cannot be combined with detailed itemized deduction fields"
                .into(),
        );
    }

    if deductions.method == "itemized" {
        match deductions.itemized_amount {
            None if !has_detailed => errors.push(
                "itemized deductions require deductions.itemized_amount or detailed itemized deduction fields"
                    .into(),
            ),
            Some(amt) if amt < 0.0 => {
                errors.push("deductions.itemized_amount must be non-negative".into())
            }
            _ => {}
        }
    } else if let Some(amt) = deductions.itemized_amount {
        if amt < 0.0 {
            errors.push("deductions.itemized_amount must be non-negative".into());
        }
    }

    for (field, value) in [
        (
            "deductions.state_local_income_or_sales_tax",
            deductions.state_local_income_or_sales_tax,
        ),
        ("deductions.real_property_tax", deductions.real_property_tax),
        (
            "deductions.personal_property_tax",
            deductions.personal_property_tax,
        ),
        (
            "deductions.other_itemized_deductions",
            deductions.other_itemized_deductions,
        ),
    ] {
        if let Some(amount) = value {
            if amount < 0.0 {
                errors.push(format!("{field} must be non-negative"));
            }
        }
    }

    if has_detailed
        && (deductions.method == "itemized" || force_itemized)
        && tax_parameters.salt.is_none()
    {
        errors.push(
            "tax_parameters.salt is required when detailed SALT itemized inputs are provided"
                .into(),
        );
    }

    if let Some(salt) = &tax_parameters.salt {
        if salt.cap_amount < 0.0 {
            errors.push("tax_parameters.salt.cap_amount must be non-negative".into());
        }
        if salt.phaseout_threshold < 0.0 {
            errors.push("tax_parameters.salt.phaseout_threshold must be non-negative".into());
        }
        if !(0.0..=1.0).contains(&salt.phaseout_rate) {
            errors.push("tax_parameters.salt.phaseout_rate must be between 0.0 and 1.0".into());
        }
        if salt.floor_amount < 0.0 {
            errors.push("tax_parameters.salt.floor_amount must be non-negative".into());
        }
        if salt.floor_amount > salt.cap_amount {
            errors.push(
                "tax_parameters.salt.floor_amount must be less than or equal to cap_amount".into(),
            );
        }
    }
}

pub fn validate_estate_tax_request(req: &EstateTaxRequest) -> Vec<String> {
    let mut errors = Vec::new();

    if req.gross_estate < 0.0 {
        errors.push("gross_estate must be non-negative".into());
    }

    if req.deductions.marital < 0.0 {
        errors.push("deductions.marital must be non-negative".into());
    }
    if req.deductions.charitable < 0.0 {
        errors.push("deductions.charitable must be non-negative".into());
    }
    if req.deductions.debts_and_expenses < 0.0 {
        errors.push("deductions.debts_and_expenses must be non-negative".into());
    }
    if req.deductions.state_death_tax < 0.0 {
        errors.push("deductions.state_death_tax must be non-negative".into());
    }
    if req.deductions.other < 0.0 {
        errors.push("deductions.other must be non-negative".into());
    }

    if req.adjusted_taxable_gifts < 0.0 {
        errors.push("adjusted_taxable_gifts must be non-negative".into());
    }
    if req.gift_tax_paid < 0.0 {
        errors.push("gift_tax_paid must be non-negative".into());
    }
    if req.deceased_spouse_unused_exclusion < 0.0 {
        errors.push("deceased_spouse_unused_exclusion must be non-negative".into());
    }

    if req.estate_tax_parameters.exemption_amount < 0.0 {
        errors.push("estate_tax_parameters.exemption_amount must be non-negative".into());
    }
    if req.estate_tax_parameters.applicable_credit_amount < 0.0 {
        errors.push("estate_tax_parameters.applicable_credit_amount must be non-negative".into());
    }
    if req.estate_tax_parameters.brackets.is_empty() {
        errors.push("estate_tax_parameters.brackets must not be empty".into());
    }
    for (i, bracket) in req.estate_tax_parameters.brackets.iter().enumerate() {
        if bracket.rate < 0.0 || bracket.rate > 1.0 {
            errors.push(format!(
                "estate_tax_parameters.brackets[{}].rate must be between 0.0 and 1.0",
                i
            ));
        }
    }

    errors
}

pub fn validate_retirement_rmd_request(req: &RetirementRmdRequest) -> Vec<String> {
    let mut errors = Vec::new();

    if req.calculation_year < 1900 || req.calculation_year > 2200 {
        errors.push("calculation_year must be between 1900 and 2200".into());
    }

    if req.prior_year_end_balance < 0.0 {
        errors.push("prior_year_end_balance must be non-negative".into());
    }

    if req.account_type.trim().is_empty() {
        errors.push("account_type is required".into());
    }

    let account_type = req.account_type.trim().to_lowercase();
    let mut configured_account_types: Vec<String> = req
        .rmd_parameters
        .account_rules
        .owner_required_account_types
        .iter()
        .chain(
            req.rmd_parameters
                .account_rules
                .owner_exempt_account_types
                .iter(),
        )
        .chain(
            req.rmd_parameters
                .account_rules
                .inherited_account_types
                .iter(),
        )
        .map(|value| value.trim().to_lowercase())
        .filter(|value| !value.is_empty())
        .collect();
    configured_account_types.sort();
    configured_account_types.dedup();

    if !account_type.is_empty()
        && !configured_account_types.is_empty()
        && !configured_account_types
            .iter()
            .any(|configured| configured == &account_type)
    {
        errors.push(format!(
            "account_type '{}' is not supported; expected one of: {}",
            req.account_type,
            configured_account_types.join(", ")
        ));
    }

    if let Some(beneficiary_election) = req.beneficiary_election.as_deref() {
        let election = beneficiary_election.trim().to_lowercase();
        if !election.is_empty()
            && !["treat_as_own", "ten_year", "delay_until_owner_rbd"].contains(&election.as_str())
        {
            errors.push(format!(
                "beneficiary_election '{}' is invalid; expected one of: treat_as_own, ten_year, delay_until_owner_rbd",
                beneficiary_election
            ));
        }
    }

    if let Some(beneficiary_class) = req.beneficiary_class.as_deref() {
        let class = beneficiary_class.trim().to_lowercase();
        let mut configured_classes: Vec<String> = req
            .rmd_parameters
            .beneficiary_rules
            .recognized_beneficiary_classes
            .iter()
            .map(|value| value.trim().to_lowercase())
            .filter(|value| !value.is_empty())
            .collect();
        configured_classes.sort();
        configured_classes.dedup();

        if !class.is_empty()
            && !configured_classes.is_empty()
            && !configured_classes
                .iter()
                .any(|configured| configured == &class)
        {
            errors.push(format!(
                "beneficiary_class '{}' is not supported; expected one of: {}",
                beneficiary_class,
                configured_classes.join(", ")
            ));
        }
    }

    if req.rmd_parameters.uniform_lifetime_table.is_empty() {
        errors.push("rmd_parameters.uniform_lifetime_table must not be empty".into());
    }
    if req.rmd_parameters.joint_life_table.is_empty() {
        errors.push("rmd_parameters.joint_life_table must not be empty".into());
    }
    if req.rmd_parameters.single_life_table.is_empty() {
        errors.push("rmd_parameters.single_life_table must not be empty".into());
    }
    if req
        .rmd_parameters
        .required_beginning
        .start_age_rules
        .is_empty()
    {
        errors.push("rmd_parameters.required_beginning.start_age_rules must not be empty".into());
    }

    for (i, row) in req.rmd_parameters.uniform_lifetime_table.iter().enumerate() {
        if row.distribution_period <= 0.0 {
            errors.push(format!(
                "rmd_parameters.uniform_lifetime_table[{}].distribution_period must be > 0",
                i
            ));
        }
    }

    for (i, row) in req.rmd_parameters.joint_life_table.iter().enumerate() {
        if row.distribution_period <= 0.0 {
            errors.push(format!(
                "rmd_parameters.joint_life_table[{}].distribution_period must be > 0",
                i
            ));
        }
    }

    for (i, row) in req.rmd_parameters.single_life_table.iter().enumerate() {
        if row.distribution_period <= 0.0 {
            errors.push(format!(
                "rmd_parameters.single_life_table[{}].distribution_period must be > 0",
                i
            ));
        }
    }

    let owner_alive = req.owner_is_alive.unwrap_or(req.owner_death_year.is_none());
    let account_type = req.account_type.trim();
    let likely_beneficiary = !owner_alive
        || req.owner_death_year.is_some()
        || req
            .rmd_parameters
            .account_rules
            .inherited_account_types
            .iter()
            .any(|a| a.eq_ignore_ascii_case(account_type));

    if likely_beneficiary {
        if req.owner_death_year.is_none() {
            errors.push("owner_death_year is required for beneficiary scenarios".into());
        }
        if req
            .beneficiary_class
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
        {
            errors.push("beneficiary_class is required for beneficiary scenarios".into());
        }
    } else if req
        .owner_birth_date
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        errors.push("owner_birth_date is required for owner scenarios".into());
    }

    errors
}

pub fn validate_retirement_rmd_schedule_request(req: &RetirementRmdScheduleRequest) -> Vec<String> {
    let mut errors = validate_retirement_rmd_request(&RetirementRmdRequest {
        calculation_year: req.calculation_year,
        prior_year_end_balance: req.prior_year_end_balance,
        account_type: req.account_type.clone(),
        owner_birth_date: req.owner_birth_date.clone(),
        owner_is_alive: req.owner_is_alive,
        owner_death_year: req.owner_death_year,
        owner_died_before_required_beginning_date: req.owner_died_before_required_beginning_date,
        beneficiary_birth_date: req.beneficiary_birth_date.clone(),
        beneficiary_class: req.beneficiary_class.clone(),
        beneficiary_election: req.beneficiary_election.clone(),
        beneficiary_majority_year: req.beneficiary_majority_year,
        spouse_birth_date: req.spouse_birth_date.clone(),
        spouse_is_sole_beneficiary: req.spouse_is_sole_beneficiary,
        is_still_working: req.is_still_working,
        is_five_percent_owner: req.is_five_percent_owner,
        pre_1987_403b_balance: req.pre_1987_403b_balance,
        rmd_parameters: req.rmd_parameters.clone(),
    });

    if let Some(rate) = req.annual_growth_rate {
        if !(-1.0..=10.0).contains(&rate) {
            errors.push("annual_growth_rate must be between -1.0 and 10.0".into());
        }
    }

    if let Some(end_age) = req.end_age {
        if !(18..=130).contains(&end_age) {
            errors.push("end_age must be between 18 and 130".into());
        }
    }

    if let Some(max_years) = req.max_years {
        if max_years == 0 || max_years > 130 {
            errors.push("max_years must be between 1 and 130".into());
        }
    }

    errors
}

pub fn validate_roth_conversion_request(req: &RothConversionRequest) -> Vec<String> {
    let mut errors = Vec::new();

    let valid_statuses = [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ];
    if !valid_statuses.contains(&req.filing_status.as_str()) {
        errors.push(format!(
            "filing_status must be one of: {} (got '{}')",
            valid_statuses.join(", "),
            req.filing_status
        ));
    }

    if req.traditional_ira_balance < 0.0 {
        errors.push("traditional_ira_balance must be non-negative".into());
    }

    if let Some(amt) = req.conversion_amount {
        if amt < 0.0 {
            errors.push("conversion_amount must be non-negative".into());
        }
        if amt > req.traditional_ira_balance {
            errors.push("conversion_amount cannot exceed traditional_ira_balance".into());
        }
    }

    if req.nondeductible_basis < 0.0 {
        errors.push("nondeductible_basis must be non-negative".into());
    }

    if let Some(total) = req.total_traditional_ira_value {
        if total < 0.0 {
            errors.push("total_traditional_ira_value must be non-negative".into());
        }
    }

    if req.tax_parameters.ordinary_brackets.is_empty() {
        errors.push("tax_parameters.ordinary_brackets must not be empty".into());
    }
    if req.tax_parameters.capital_gains_brackets.is_empty() {
        errors.push("tax_parameters.capital_gains_brackets must not be empty".into());
    }

    validate_deductions_and_salt(
        &req.filing_status,
        &req.deductions,
        &req.tax_parameters,
        &mut errors,
    );

    for (i, bracket) in req.tax_parameters.ordinary_brackets.iter().enumerate() {
        if bracket.rate < 0.0 || bracket.rate > 1.0 {
            errors.push(format!(
                "tax_parameters.ordinary_brackets[{}].rate must be between 0.0 and 1.0",
                i
            ));
        }
    }

    if let Some(ref brackets) = req.irmaa_brackets {
        for (i, tier) in brackets.tiers.iter().enumerate() {
            if tier.surcharge_part_b < 0.0 {
                errors.push(format!(
                    "irmaa_brackets.tiers[{}].surcharge_part_b must be non-negative",
                    i
                ));
            }
            if tier.surcharge_part_d < 0.0 {
                errors.push(format!(
                    "irmaa_brackets.tiers[{}].surcharge_part_d must be non-negative",
                    i
                ));
            }
        }
    }

    if let Some(ss) = req.gross_social_security_benefit {
        if ss < 0.0 {
            errors.push("gross_social_security_benefit must be non-negative".into());
        }
        if req.ss_taxation_params.is_none() {
            errors.push(
                "ss_taxation_params is required when gross_social_security_benefit is provided"
                    .into(),
            );
        }
    }

    errors
}

pub fn validate_roth_conversion_strategy_request(
    req: &RothConversionStrategyRequest,
) -> Vec<String> {
    let mut errors = Vec::new();

    let valid_statuses = [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ];
    if !valid_statuses.contains(&req.filing_status.as_str()) {
        errors.push(format!(
            "filing_status must be one of: {} (got '{}')",
            valid_statuses.join(", "),
            req.filing_status
        ));
    }

    if req.traditional_ira_balance < 0.0 {
        errors.push("traditional_ira_balance must be non-negative".into());
    }

    if req.projection_years == 0 || req.projection_years > 50 {
        errors.push("projection_years must be between 1 and 50".into());
    }

    if !(-1.0..=1.0).contains(&req.annual_growth_rate) {
        errors.push("annual_growth_rate must be between -1.0 and 1.0".into());
    }

    let valid_strategies = ["fill_bracket", "fixed_amount"];
    if !valid_strategies.contains(&req.strategy.as_str()) {
        errors.push(format!(
            "strategy must be one of: {} (got '{}')",
            valid_strategies.join(", "),
            req.strategy
        ));
    }

    if req.strategy == "fill_bracket" {
        if req.target_bracket_rate.is_none() {
            errors.push("target_bracket_rate is required for fill_bracket strategy".into());
        } else if let Some(rate) = req.target_bracket_rate {
            let rates: Vec<f64> = req
                .tax_parameters
                .ordinary_brackets
                .iter()
                .map(|b| b.rate)
                .collect();
            if !rates.iter().any(|r| (r - rate).abs() < 1e-10) {
                errors.push(format!(
                    "target_bracket_rate {} must match a bracket rate",
                    rate
                ));
            }
        }
    }

    if req.strategy == "fixed_amount" {
        if let Some(amt) = req.fixed_annual_conversion {
            if amt < 0.0 {
                errors.push("fixed_annual_conversion must be non-negative".into());
            }
        }
    }

    if req.tax_parameters.ordinary_brackets.is_empty() {
        errors.push("tax_parameters.ordinary_brackets must not be empty".into());
    }
    if req.tax_parameters.capital_gains_brackets.is_empty() {
        errors.push("tax_parameters.capital_gains_brackets must not be empty".into());
    }

    validate_deductions_and_salt(
        &req.filing_status,
        &req.deductions,
        &req.tax_parameters,
        &mut errors,
    );

    // Validate owner_birth_date is parseable
    if req
        .owner_birth_date
        .split('-')
        .next()
        .and_then(|y| y.parse::<u32>().ok())
        .is_none()
    {
        errors.push(format!(
            "owner_birth_date '{}' is not a valid date (expected YYYY-MM-DD)",
            req.owner_birth_date
        ));
    }

    errors
}

pub fn validate_pension_comparison_request(req: &PensionComparisonRequest) -> Vec<String> {
    let mut errors = Vec::new();
    let mut explicit_option_ids = std::collections::HashSet::new();

    // Retiree age
    if req.retiree_age < 18 || req.retiree_age > 100 {
        errors.push("retiree_age must be between 18 and 100".into());
    }

    // Gender
    if let Some(ref g) = req.retiree_gender {
        if g != "male" && g != "female" {
            errors.push(format!(
                "retiree_gender must be 'male' or 'female' (got '{}')",
                g
            ));
        }
    }
    if let Some(ref g) = req.spouse_gender {
        if g != "male" && g != "female" {
            errors.push(format!(
                "spouse_gender must be 'male' or 'female' (got '{}')",
                g
            ));
        }
    }

    // Annuity options
    if req.annuity_options.is_empty() {
        errors.push("annuity_options must not be empty".into());
    }

    let valid_option_types = [
        "single_life",
        "joint_survivor",
        "joint_50",
        "joint_75",
        "joint_100",
        "period_certain",
        "period_certain_10",
        "period_certain_20",
        "level_income",
    ];

    for (i, opt) in req.annuity_options.iter().enumerate() {
        if let Some(ref id) = opt.id {
            if id.trim().is_empty() {
                errors.push(format!("annuity_options[{}].id must not be blank", i));
            } else if !explicit_option_ids.insert(id.clone()) {
                errors.push(format!("annuity_options[{}].id '{}' is duplicated", i, id));
            }
        }

        if !valid_option_types.contains(&opt.option_type.as_str()) {
            errors.push(format!(
                "annuity_options[{}].option_type must be one of: {} (got '{}')",
                i,
                valid_option_types.join(", "),
                opt.option_type
            ));
        }

        if opt.monthly_payment <= 0.0 {
            errors.push(format!(
                "annuity_options[{}].monthly_payment must be positive",
                i
            ));
        }

        // Joint options require spouse_age and (survivor_pct or survivor_monthly_payment)
        let is_joint = opt.option_type == "joint_survivor" || opt.option_type.starts_with("joint_");
        if is_joint {
            if req.spouse_age.is_none() {
                errors.push(format!(
                    "annuity_options[{}]: joint option '{}' requires spouse_age",
                    i, opt.option_type
                ));
            }
            // joint_survivor (non-alias) requires survivor_pct or survivor_monthly_payment
            if opt.option_type == "joint_survivor"
                && opt.survivor_pct.is_none()
                && opt.survivor_monthly_payment.is_none()
            {
                errors.push(format!(
                    "annuity_options[{}]: joint_survivor requires survivor_pct or survivor_monthly_payment",
                    i
                ));
            }
        }

        // survivor_pct validation
        if let Some(pct) = opt.survivor_pct {
            if !(0.0..=1.0).contains(&pct) {
                errors.push(format!(
                    "annuity_options[{}].survivor_pct must be between 0.0 and 1.0 (got {})",
                    i, pct
                ));
            }
        }

        // popup_monthly_payment only valid for joint options
        if let Some(popup) = opt.popup_monthly_payment {
            if popup <= 0.0 {
                errors.push(format!(
                    "annuity_options[{}].popup_monthly_payment must be positive",
                    i
                ));
            }
            if !is_joint {
                errors.push(format!(
                    "annuity_options[{}].popup_monthly_payment is only valid for joint options",
                    i
                ));
            }
        }

        // Period certain options require period_certain_months
        if opt.option_type == "period_certain" && opt.period_certain_months.is_none() {
            errors.push(format!(
                "annuity_options[{}]: period_certain requires period_certain_months",
                i
            ));
        }
        if let Some(pcm) = opt.period_certain_months {
            if pcm == 0 {
                errors.push(format!(
                    "annuity_options[{}].period_certain_months must be > 0",
                    i
                ));
            }
        }

        // Level income validation
        if opt.option_type == "level_income" {
            match opt.level_income_age {
                Some(age) if !(50..=80).contains(&age) => {
                    errors.push(format!(
                        "annuity_options[{}].level_income_age must be between 50 and 80 (got {})",
                        i, age
                    ));
                }
                None => {
                    errors.push(format!(
                        "annuity_options[{}]: level_income requires level_income_age",
                        i
                    ));
                }
                _ => {}
            }
            match opt.level_income_reduced_payment {
                Some(p) if p < 0.0 => {
                    errors.push(format!(
                        "annuity_options[{}].level_income_reduced_payment must be >= 0",
                        i
                    ));
                }
                None => {
                    errors.push(format!(
                        "annuity_options[{}]: level_income requires level_income_reduced_payment",
                        i
                    ));
                }
                _ => {}
            }
        }

        // Per-option COLA validation
        if let Some(ref cola) = opt.cola {
            if cola.rate < 0.0 || cola.rate > 0.10 {
                errors.push(format!(
                    "annuity_options[{}].cola.rate must be between 0.0 and 0.10 (got {})",
                    i, cola.rate
                ));
            }
            if let Some(cap) = cola.cap_rate {
                if !(0.0..=0.10).contains(&cap) {
                    errors.push(format!(
                        "annuity_options[{}].cola.cap_rate must be between 0.0 and 0.10 (got {})",
                        i, cap
                    ));
                }
            }
            if cola.start_delay_months > 240 {
                errors.push(format!(
                    "annuity_options[{}].cola.start_delay_months must be 0–240 (got {})",
                    i, cola.start_delay_months
                ));
            }
            if let Some(end_age) = cola.end_age {
                if end_age <= req.retiree_age {
                    errors.push(format!(
                        "annuity_options[{}].cola.end_age must be > retiree_age {} (got {})",
                        i, req.retiree_age, end_age
                    ));
                }
            }
        }
    }

    // Lump sum
    if let Some(ls) = req.lump_sum_amount {
        if ls <= 0.0 {
            errors.push("lump_sum_amount must be positive".into());
        }
        if req.after_tax_contribution_basis > ls {
            errors.push("after_tax_contribution_basis must not exceed lump_sum_amount".into());
        }
    }

    if let Some(ref selected_id) = req.selected_option_id {
        if !selected_id.trim().is_empty()
            && !req
                .annuity_options
                .iter()
                .any(|opt| opt.id.as_deref() == Some(selected_id.as_str()))
        {
            errors.push(format!(
                "selected_option_id '{}' does not match any annuity_options[].id",
                selected_id
            ));
        }
    }

    // Investment return
    if req.investment_return.annual_std_dev < 0.0 {
        errors.push("investment_return.annual_std_dev must be non-negative".into());
    }

    // Rates
    if req.discount_rate < 0.0 || req.discount_rate > 0.30 {
        errors.push("discount_rate must be between 0.0 and 0.30".into());
    }
    if req.inflation_rate < 0.0 || req.inflation_rate > 0.20 {
        errors.push("inflation_rate must be between 0.0 and 0.20".into());
    }
    if req.cola_rate < 0.0 || req.cola_rate > 0.10 {
        errors.push("cola_rate must be between 0.0 and 0.10".into());
    }

    let taxable = &req.taxable_account_assumptions;
    for (label, value) in [
        (
            "taxable_account_assumptions.taxable_interest_yield",
            taxable.taxable_interest_yield,
        ),
        (
            "taxable_account_assumptions.ordinary_dividend_yield",
            taxable.ordinary_dividend_yield,
        ),
        (
            "taxable_account_assumptions.qualified_dividend_yield",
            taxable.qualified_dividend_yield,
        ),
        (
            "taxable_account_assumptions.short_term_capital_gain_yield",
            taxable.short_term_capital_gain_yield,
        ),
        (
            "taxable_account_assumptions.long_term_capital_gain_yield",
            taxable.long_term_capital_gain_yield,
        ),
    ] {
        if !(0.0..=1.0).contains(&value) {
            errors.push(format!("{label} must be between 0.0 and 1.0"));
        }
    }

    // Mortality table
    if req.mortality_table.unisex_table.is_empty() {
        errors.push("mortality_table.unisex_table must not be empty".into());
    }
    for entry in &req.mortality_table.unisex_table {
        if entry.qx < 0.0 || entry.qx > 1.0 {
            errors.push(format!(
                "mortality_table.unisex_table: qx at age {} must be between 0.0 and 1.0 (got {})",
                entry.age, entry.qx
            ));
            break;
        }
    }

    // Filing status
    let valid_statuses = [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ];
    if !valid_statuses.contains(&req.filing_status.as_str()) {
        errors.push(format!(
            "filing_status must be one of: {} (got '{}')",
            valid_statuses.join(", "),
            req.filing_status
        ));
    }

    // Tax parameters
    if req.tax_parameters.ordinary_brackets.is_empty() {
        errors.push("tax_parameters.ordinary_brackets must not be empty".into());
    }

    // Simulation limits
    if let Some(n) = req.num_simulations {
        if n == 0 {
            errors.push("num_simulations must be greater than 0".into());
        }
        if n > 100_000 {
            errors.push("num_simulations must be at most 100000".into());
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::retirement_rmd::{
        AccountRules, AgeDistributionPeriod, BeneficiaryRules, JointDistributionPeriod,
        Pre1987Rules, RequiredBeginningRules, RetirementRmdRequest, RetirementRmdScheduleRequest,
        RmdParameters, StartAgeRule, TenYearRule,
    };
    use crate::models::simulation_request::{
        CashFlow, ReturnAssumption, SimulationBucket, SimulationRequest, SpendingPolicy,
    };
    use crate::models::solver_request::{SolveFor, SolverBounds, SolverRequest, SolverTarget};
    use crate::models::tax_request::{
        Adjustments, DeductionConfig, IncomeBreakdown, NiitParams, PayrollParams,
        SaltDeductionParams, TaxBracket, TaxParameters,
    };
    use std::collections::HashMap;

    fn valid_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("both".into()),
            num_simulations: Some(10000),
            seed: None,
            starting_balance: Some(500_000.0),
            buckets: vec![],
            time_horizon_months: 360,
            return_assumption: Some(ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            }),
            cash_flows: vec![CashFlow {
                amount: -2000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            filing_status: None,
            household: None,
            spending_policy: None,
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".to_string(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    fn valid_bucketed_request() -> SimulationRequest {
        SimulationRequest {
            mode: Some("both".into()),
            num_simulations: Some(1000),
            seed: Some(1),
            starting_balance: None,
            buckets: vec![
                SimulationBucket {
                    id: "taxable".into(),
                    bucket_type: "taxable".into(),
                    starting_balance: 60_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.06,
                        annual_std_dev: 0.10,
                    },
                    realized_gain_ratio: Some(0.20),
                    withdrawal_priority: Some(1),
                },
                SimulationBucket {
                    id: "tax_deferred".into(),
                    bucket_type: "tax_deferred".into(),
                    starting_balance: 40_000.0,
                    return_assumption: ReturnAssumption {
                        annual_mean: 0.05,
                        annual_std_dev: 0.08,
                    },
                    realized_gain_ratio: None,
                    withdrawal_priority: Some(2),
                },
            ],
            time_horizon_months: 360,
            return_assumption: None,
            cash_flows: vec![],
            filing_status: Some("single".into()),
            household: None,
            spending_policy: Some(SpendingPolicy {
                withdrawal_order: vec!["taxable".into(), "tax_deferred".into()],
                rebalance_tax_withholding_from: Some("taxable".into()),
            }),
            tax_policy: None,
            rmd_policy: None,
            include_detail: false,
            detail_granularity: "annual".to_string(),
            sample_paths: None,
            path_indices: None,
            custom_percentiles: None,
        }
    }

    #[test]
    fn test_valid_request_passes() {
        let errors = validate_simulation_request(&valid_request());
        assert!(errors.is_empty(), "expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_bucketed_request_rejected_by_current_execution_engine() {
        let errors = validate_simulation_request(&valid_bucketed_request());
        assert!(errors.iter().any(|e| e.contains("not yet supported")));
    }

    #[test]
    fn test_negative_balance() {
        let mut req = valid_request();
        req.starting_balance = Some(-100.0);
        let errors = validate_simulation_request(&req);
        assert!(errors.iter().any(|e| e.contains("starting_balance")));
    }

    #[test]
    fn test_zero_horizon() {
        let mut req = valid_request();
        req.time_horizon_months = 0;
        let errors = validate_simulation_request(&req);
        assert!(errors.iter().any(|e| e.contains("time_horizon_months")));
    }

    #[test]
    fn test_negative_std_dev() {
        let mut req = valid_request();
        req.return_assumption.as_mut().unwrap().annual_std_dev = -0.1;
        let errors = validate_simulation_request(&req);
        assert!(errors.iter().any(|e| e.contains("annual_std_dev")));
    }

    #[test]
    fn test_invalid_mode() {
        let mut req = valid_request();
        req.mode = Some("invalid".into());
        let errors = validate_simulation_request(&req);
        assert!(errors.iter().any(|e| e.contains("mode")));
    }

    #[test]
    fn test_zero_cash_flow_amount() {
        let mut req = valid_request();
        req.cash_flows[0].amount = 0.0;
        let errors = validate_simulation_request(&req);
        assert!(errors.iter().any(|e| e.contains("amount")));
    }

    #[test]
    fn test_collects_multiple_errors() {
        let mut req = valid_request();
        req.starting_balance = Some(-1.0);
        req.time_horizon_months = 0;
        req.return_assumption.as_mut().unwrap().annual_std_dev = -0.5;
        let errors = validate_simulation_request(&req);
        assert!(errors.len() >= 3);
    }

    #[test]
    fn test_duplicate_bucket_ids_rejected() {
        let mut req = valid_bucketed_request();
        req.buckets[1].id = "taxable".into();
        let errors = validate_simulation_request(&req);
        assert!(errors.iter().any(|e| e.contains("duplicate")));
    }

    #[test]
    fn test_unknown_bucket_references_in_spending_policy_rejected() {
        let mut req = valid_bucketed_request();
        req.spending_policy = Some(SpendingPolicy {
            withdrawal_order: vec!["missing_bucket".into()],
            rebalance_tax_withholding_from: Some("other_missing_bucket".into()),
        });
        let errors = validate_simulation_request(&req);
        assert!(errors.iter().any(|e| e.contains("withdrawal_order")));
        assert!(errors
            .iter()
            .any(|e| e.contains("rebalance_tax_withholding_from")));
    }

    fn valid_solver_request() -> SolverRequest {
        SolverRequest {
            solve_for: SolveFor {
                variable: "cash_flow_amount".into(),
                cash_flow_index: Some(0),
            },
            target: SolverTarget {
                metric: "success_rate".into(),
                value: 0.90,
                percentile: None,
            },
            mode: None,
            num_simulations: Some(1000),
            seed: Some(42),
            starting_balance: 500_000.0,
            time_horizon_months: 360,
            return_assumption: ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            },
            cash_flows: vec![CashFlow {
                amount: -3000.0,
                frequency: "monthly".into(),
                start_month: Some(0),
                end_month: None,
            }],
            bounds: Some(SolverBounds {
                lower: -10000.0,
                upper: 0.0,
            }),
            max_iterations: Some(50),
            tolerance: Some(0.005),
        }
    }

    fn valid_federal_tax_request() -> FederalTaxRequest {
        FederalTaxRequest {
            filing_status: "single".to_string(),
            tax_year: 2026,
            income: IncomeBreakdown {
                wages: 100_000.0,
                self_employment_income: 0.0,
                taxable_interest: 0.0,
                tax_exempt_interest: 0.0,
                ordinary_dividends: 0.0,
                qualified_dividends: 0.0,
                short_term_capital_gains: 0.0,
                long_term_capital_gains: 0.0,
                taxable_ira_distributions: 0.0,
                taxable_pensions: 0.0,
                taxable_social_security: 0.0,
                other_income: 0.0,
            },
            adjustments: Adjustments::default(),
            deductions: DeductionConfig {
                method: "standard".to_string(),
                itemized_amount: None,
                spouse_itemizes: None,
                state_local_income_or_sales_tax: None,
                real_property_tax: None,
                personal_property_tax: None,
                other_itemized_deductions: None,
            },
            tax_parameters: TaxParameters {
                ordinary_brackets: vec![TaxBracket {
                    min: 0.0,
                    max: Some(50_000.0),
                    rate: 0.10,
                }],
                capital_gains_brackets: vec![TaxBracket {
                    min: 0.0,
                    max: Some(50_000.0),
                    rate: 0.0,
                }],
                standard_deduction: 15_000.0,
                capital_loss_limit: 3_000.0,
                niit: NiitParams {
                    rate: 0.038,
                    threshold: 200_000.0,
                },
                payroll: PayrollParams {
                    social_security_rate: 0.062,
                    social_security_wage_base: 176_100.0,
                    self_employment_tax_rate: 0.124,
                    medicare_rate: 0.0145,
                    self_employment_medicare_rate: 0.029,
                    additional_medicare_rate: 0.009,
                    additional_medicare_threshold: 200_000.0,
                },
                salt: Some(SaltDeductionParams {
                    cap_amount: 40_400.0,
                    phaseout_threshold: 505_000.0,
                    phaseout_rate: 0.30,
                    floor_amount: 10_000.0,
                }),
            },
        }
    }

    #[test]
    fn test_valid_solver_request_passes() {
        let errors = validate_solver_request(&valid_solver_request());
        assert!(errors.is_empty(), "expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_solver_invalid_variable() {
        let mut req = valid_solver_request();
        req.solve_for.variable = "invalid".into();
        let errors = validate_solver_request(&req);
        assert!(errors.iter().any(|e| e.contains("solve_for.variable")));
    }

    #[test]
    fn test_solver_cash_flow_index_out_of_range() {
        let mut req = valid_solver_request();
        req.solve_for.cash_flow_index = Some(5);
        let errors = validate_solver_request(&req);
        assert!(errors.iter().any(|e| e.contains("cash_flow_index")));
    }

    #[test]
    fn test_solver_invalid_metric() {
        let mut req = valid_solver_request();
        req.target.metric = "bad_metric".into();
        let errors = validate_solver_request(&req);
        assert!(errors.iter().any(|e| e.contains("target.metric")));
    }

    #[test]
    fn test_solver_success_rate_out_of_range() {
        let mut req = valid_solver_request();
        req.target.value = 1.5;
        let errors = validate_solver_request(&req);
        assert!(errors.iter().any(|e| e.contains("success_rate")));
    }

    #[test]
    fn test_solver_percentile_balance_missing_percentile() {
        let mut req = valid_solver_request();
        req.target.metric = "percentile_balance".into();
        req.target.percentile = None;
        let errors = validate_solver_request(&req);
        assert!(errors.iter().any(|e| e.contains("percentile")));
    }

    #[test]
    fn test_solver_invalid_bounds_ordering() {
        let mut req = valid_solver_request();
        req.bounds = Some(SolverBounds {
            lower: 100.0,
            upper: 50.0,
        });
        let errors = validate_solver_request(&req);
        assert!(errors.iter().any(|e| e.contains("bounds")));
    }

    #[test]
    fn test_federal_tax_request_allows_detailed_itemized_inputs() {
        let mut req = valid_federal_tax_request();
        req.deductions.method = "itemized".to_string();
        req.deductions.state_local_income_or_sales_tax = Some(20_000.0);
        req.deductions.real_property_tax = Some(18_000.0);
        req.deductions.other_itemized_deductions = Some(5_000.0);

        let errors = validate_federal_tax_request(&req);
        assert!(errors.is_empty(), "expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_federal_tax_request_rejects_mixed_itemized_inputs() {
        let mut req = valid_federal_tax_request();
        req.deductions.method = "itemized".to_string();
        req.deductions.itemized_amount = Some(25_000.0);
        req.deductions.state_local_income_or_sales_tax = Some(10_000.0);

        let errors = validate_federal_tax_request(&req);
        assert!(errors
            .iter()
            .any(|e| e.contains("cannot be combined with detailed itemized deduction fields")));
    }

    #[test]
    fn test_federal_tax_request_requires_salt_params_for_detailed_inputs() {
        let mut req = valid_federal_tax_request();
        req.deductions.method = "itemized".to_string();
        req.deductions.state_local_income_or_sales_tax = Some(10_000.0);
        req.tax_parameters.salt = None;

        let errors = validate_federal_tax_request(&req);
        assert!(errors
            .iter()
            .any(|e| e.contains("tax_parameters.salt is required")));
    }

    fn valid_rmd_parameters() -> RmdParameters {
        RmdParameters {
            uniform_lifetime_table: vec![AgeDistributionPeriod {
                age: 75,
                distribution_period: 24.6,
            }],
            joint_life_table: vec![JointDistributionPeriod {
                owner_age: 75,
                spouse_age: 64,
                distribution_period: 27.4,
            }],
            single_life_table: vec![AgeDistributionPeriod {
                age: 50,
                distribution_period: 36.2,
            }],
            required_beginning: RequiredBeginningRules {
                start_age_rules: vec![StartAgeRule {
                    birth_year_min: Some(1951),
                    birth_year_max: Some(1959),
                    start_age: 73,
                    guidance_status: None,
                    notes: None,
                }],
                first_distribution_deadline: "april_1_following_year".to_string(),
                still_working_exception_plan_categories: vec!["401k".to_string()],
                still_working_exception_eligible_account_types: vec!["401k".to_string()],
                still_working_exception_disallowed_for_five_percent_owners: true,
            },
            account_rules: AccountRules {
                owner_required_account_types: vec!["traditional_ira".to_string()],
                owner_exempt_account_types: vec![
                    "roth_ira".to_string(),
                    "designated_roth_plan_account".to_string(),
                ],
                inherited_account_types: vec!["inherited_ira".to_string()],
                supports_pre_1987_403b_exclusion: true,
                designated_roth_owner_exemption_effective_year: Some(2024),
            },
            beneficiary_rules: BeneficiaryRules {
                beneficiary_categories: vec![
                    "eligible_designated_beneficiary".to_string(),
                    "non_designated_beneficiary".to_string(),
                ],
                recognized_beneficiary_classes: vec![
                    "spouse".to_string(),
                    "non_designated_beneficiary".to_string(),
                ],
                eligible_designated_beneficiary_classes: vec!["spouse".to_string()],
                life_expectancy_method_by_class: HashMap::new(),
                minor_child_majority_age: 21,
                spouse_delay_allowed: true,
                non_designated_beneficiary_rules:
                    crate::models::retirement_rmd::NonDesignatedBeneficiaryRules {
                        when_owner_died_before_required_beginning_date: "five_year_rule"
                            .to_string(),
                        when_owner_died_on_or_after_required_beginning_date:
                            "owner_remaining_life_expectancy".to_string(),
                    },
            },
            ten_year_rule: TenYearRule {
                terminal_year: 10,
                annual_distributions_required_when_owner_died_on_or_after_rbd: true,
            },
            relief_years: vec![],
            pre_1987_403b_rules: Pre1987Rules {
                exclude_until_age: 75,
            },
        }
    }

    #[test]
    fn test_valid_rmd_request_passes() {
        let req = RetirementRmdRequest {
            calculation_year: 2026,
            prior_year_end_balance: 100_000.0,
            account_type: "traditional_ira".to_string(),
            owner_birth_date: Some("1951-01-01".to_string()),
            owner_is_alive: Some(true),
            owner_death_year: None,
            owner_died_before_required_beginning_date: None,
            beneficiary_birth_date: None,
            beneficiary_class: None,
            beneficiary_election: None,
            beneficiary_majority_year: None,
            spouse_birth_date: None,
            spouse_is_sole_beneficiary: None,
            is_still_working: Some(false),
            is_five_percent_owner: Some(false),
            pre_1987_403b_balance: None,
            rmd_parameters: valid_rmd_parameters(),
        };

        let errors = validate_retirement_rmd_request(&req);
        assert!(errors.is_empty(), "expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_rmd_request_rejects_unsupported_account_type() {
        let req = RetirementRmdRequest {
            calculation_year: 2026,
            prior_year_end_balance: 100_000.0,
            account_type: "crypto_ira".to_string(),
            owner_birth_date: Some("1951-01-01".to_string()),
            owner_is_alive: Some(true),
            owner_death_year: None,
            owner_died_before_required_beginning_date: None,
            beneficiary_birth_date: None,
            beneficiary_class: None,
            beneficiary_election: None,
            beneficiary_majority_year: None,
            spouse_birth_date: None,
            spouse_is_sole_beneficiary: None,
            is_still_working: Some(false),
            is_five_percent_owner: Some(false),
            pre_1987_403b_balance: None,
            rmd_parameters: valid_rmd_parameters(),
        };

        let errors = validate_retirement_rmd_request(&req);
        assert!(errors.iter().any(|e| e.contains("not supported")));
    }

    #[test]
    fn test_rmd_request_rejects_invalid_beneficiary_election() {
        let req = RetirementRmdRequest {
            calculation_year: 2026,
            prior_year_end_balance: 100_000.0,
            account_type: "inherited_ira".to_string(),
            owner_birth_date: Some("1951-01-01".to_string()),
            owner_is_alive: Some(false),
            owner_death_year: Some(2020),
            owner_died_before_required_beginning_date: Some(false),
            beneficiary_birth_date: Some("1970-01-01".to_string()),
            beneficiary_class: Some("spouse".to_string()),
            beneficiary_election: Some("random_mode".to_string()),
            beneficiary_majority_year: None,
            spouse_birth_date: None,
            spouse_is_sole_beneficiary: None,
            is_still_working: Some(false),
            is_five_percent_owner: Some(false),
            pre_1987_403b_balance: None,
            rmd_parameters: valid_rmd_parameters(),
        };

        let errors = validate_retirement_rmd_request(&req);
        assert!(errors.iter().any(|e| e.contains("beneficiary_election")));
    }

    #[test]
    fn test_rmd_schedule_invalid_growth_rate() {
        let req = RetirementRmdScheduleRequest {
            calculation_year: 2026,
            prior_year_end_balance: 100_000.0,
            account_type: "traditional_ira".to_string(),
            owner_birth_date: Some("1951-01-01".to_string()),
            owner_is_alive: Some(true),
            owner_death_year: None,
            owner_died_before_required_beginning_date: None,
            beneficiary_birth_date: None,
            beneficiary_class: None,
            beneficiary_election: None,
            beneficiary_majority_year: None,
            spouse_birth_date: None,
            spouse_is_sole_beneficiary: None,
            is_still_working: Some(false),
            is_five_percent_owner: Some(false),
            pre_1987_403b_balance: None,
            annual_growth_rate: Some(20.0),
            end_age: Some(120),
            max_years: Some(20),
            rmd_parameters: valid_rmd_parameters(),
        };

        let errors = validate_retirement_rmd_schedule_request(&req);
        assert!(errors.iter().any(|e| e.contains("annual_growth_rate")));
    }
}
