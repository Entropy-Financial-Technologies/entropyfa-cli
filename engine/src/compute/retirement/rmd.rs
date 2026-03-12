use crate::models::retirement_rmd::{
    AgeDistributionPeriod, RequiredBeginningRules, RetirementRmdRequest, RetirementRmdResponse,
    RetirementRmdScheduleRequest, RetirementRmdScheduleResponse, RetirementRmdScheduleRow,
};

const CITATIONS: [&str; 9] = [
    "retirement.rmd_uniform_lifetime_table",
    "retirement.rmd_joint_life_table",
    "retirement.rmd_single_life_table",
    "retirement.rmd_required_beginning_rules",
    "retirement.rmd_account_applicability_rules",
    "retirement.rmd_beneficiary_class_rules",
    "retirement.rmd_10_year_rule_requirements",
    "retirement.rmd_exception_relief_rules",
    "retirement.rmd_403b_pre1987_rules",
];

#[derive(Debug, Clone)]
struct RmdComputation {
    scenario_class: String,
    rmd_required: bool,
    rmd_amount: f64,
    applicable_balance: f64,
    distribution_period: Option<f64>,
    table_used: Option<String>,
    rule_path: String,
    decision_trace: Vec<String>,
    age: Option<u32>,
}

pub fn run_retirement_rmd(req: &RetirementRmdRequest) -> Result<RetirementRmdResponse, String> {
    let computation = compute_rmd_for_year(
        req,
        req.calculation_year,
        req.prior_year_end_balance.max(0.0),
    )?;

    Ok(RetirementRmdResponse {
        calculation_year: req.calculation_year,
        scenario_class: computation.scenario_class,
        rmd_required: computation.rmd_required,
        rmd_amount: round2(computation.rmd_amount),
        applicable_balance: round2(computation.applicable_balance),
        distribution_period: computation.distribution_period.map(round4),
        table_used: computation.table_used,
        rule_path: computation.rule_path,
        decision_trace: computation.decision_trace,
        citations: CITATIONS.iter().map(|s| s.to_string()).collect(),
    })
}

pub fn run_retirement_rmd_schedule(
    req: &RetirementRmdScheduleRequest,
) -> Result<RetirementRmdScheduleResponse, String> {
    let annual_growth_rate = req.annual_growth_rate.unwrap_or(0.0);
    let max_years = req.max_years.unwrap_or(120);

    let template_req = RetirementRmdRequest {
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
    };

    let mut balance = req.prior_year_end_balance.max(0.0);
    let mut rows = Vec::new();

    let mut year = req.calculation_year;
    let mut iterations = 0u32;

    while iterations < max_years && balance > 0.0 {
        let computation = compute_rmd_for_year(&template_req, year, balance)?;

        let rmd_amount = if computation.rmd_required {
            computation.rmd_amount.min(balance)
        } else {
            0.0
        };

        // Projection convention: RMD first, then growth.
        let end_balance = ((balance - rmd_amount).max(0.0) * (1.0 + annual_growth_rate)).max(0.0);

        rows.push(RetirementRmdScheduleRow {
            year,
            age: computation.age,
            start_balance: round2(balance),
            applicable_balance: round2(computation.applicable_balance),
            rmd_required: computation.rmd_required,
            distribution_period: computation.distribution_period.map(round4),
            rmd_amount: round2(rmd_amount),
            end_balance: round2(end_balance),
            rule_path: computation.rule_path,
        });

        balance = end_balance;
        iterations += 1;

        if reached_schedule_end_age(req, computation.age) {
            break;
        }

        year += 1;
    }

    let end_year = rows
        .last()
        .map(|row| row.year)
        .unwrap_or(req.calculation_year);

    Ok(RetirementRmdScheduleResponse {
        start_year: req.calculation_year,
        end_year,
        annual_growth_rate: round6(annual_growth_rate),
        rows,
        projection_convention: "rmd_first_then_growth".to_string(),
    })
}

fn reached_schedule_end_age(req: &RetirementRmdScheduleRequest, age: Option<u32>) -> bool {
    let limit = req.end_age.unwrap_or(120);
    age.map(|a| a >= limit).unwrap_or(false)
}

fn compute_rmd_for_year(
    req: &RetirementRmdRequest,
    calculation_year: u32,
    prior_year_end_balance: f64,
) -> Result<RmdComputation, String> {
    let owner_alive = req.owner_is_alive.unwrap_or(req.owner_death_year.is_none());
    let account_type = normalize_value(&req.account_type);

    let inherited_by_type = req
        .rmd_parameters
        .account_rules
        .inherited_account_types
        .iter()
        .any(|a| normalize_value(a) == account_type);

    let beneficiary_scenario = !owner_alive || req.owner_death_year.is_some() || inherited_by_type;

    if beneficiary_scenario {
        compute_beneficiary_rmd(req, calculation_year, prior_year_end_balance)
    } else {
        compute_owner_rmd(req, calculation_year, prior_year_end_balance)
    }
}

fn compute_owner_rmd(
    req: &RetirementRmdRequest,
    calculation_year: u32,
    prior_year_end_balance: f64,
) -> Result<RmdComputation, String> {
    let mut trace = vec!["scenario=owner_lifetime".to_string()];
    let account_type = normalize_value(&req.account_type);

    if req
        .rmd_parameters
        .account_rules
        .owner_exempt_account_types
        .iter()
        .any(|a| normalize_value(a) == account_type)
    {
        trace.push("account_exempt_for_owner=true".to_string());
        return Ok(no_rmd(
            "owner_lifetime",
            prior_year_end_balance,
            "owner_account_exempt",
            trace,
            None,
        ));
    }

    if account_type == "designated_roth_plan_account" {
        if let Some(start_year) = req
            .rmd_parameters
            .required_beginning
            .designated_roth_owner_rmd_start_year
        {
            if calculation_year < start_year {
                trace.push(format!(
                    "designated_roth_owner_rmd_start_year={} not reached",
                    start_year
                ));
                return Ok(no_rmd(
                    "owner_lifetime",
                    prior_year_end_balance,
                    "designated_roth_owner_not_required_yet",
                    trace,
                    None,
                ));
            }
        }
    }

    let owner_birth_date = req
        .owner_birth_date
        .as_ref()
        .ok_or_else(|| "owner_birth_date is required for owner-lifetime RMD".to_string())?;
    let owner_birth_year = parse_year(owner_birth_date)
        .ok_or_else(|| "owner_birth_date must be YYYY-MM-DD".to_string())?;
    let owner_age = age_in_year(owner_birth_date, calculation_year)
        .ok_or_else(|| "owner_birth_date is after calculation_year".to_string())?;

    let start_age = find_start_age(owner_birth_year, &req.rmd_parameters.required_beginning)
        .ok_or_else(|| {
            format!(
                "No RMD start age rule matched birth year {}",
                owner_birth_year
            )
        })?;
    let rmd_start_year = owner_birth_year + start_age;

    trace.push(format!("owner_age={}", owner_age));
    trace.push(format!("start_age={}", start_age));
    trace.push(format!("rmd_start_year={}", rmd_start_year));

    if calculation_year < rmd_start_year {
        return Ok(no_rmd(
            "owner_lifetime",
            prior_year_end_balance,
            "before_required_beginning_year",
            trace,
            Some(owner_age),
        ));
    }

    if applies_still_working_exception(req, &account_type) {
        trace.push("still_working_exception_applied=true".to_string());
        return Ok(no_rmd(
            "owner_lifetime",
            prior_year_end_balance,
            "still_working_exception",
            trace,
            Some(owner_age),
        ));
    }

    let mut applicable_balance = prior_year_end_balance.max(0.0);

    if account_type == "403b"
        && req
            .rmd_parameters
            .account_rules
            .supports_pre_1987_403b_exclusion
    {
        if let Some(pre_1987_balance) = req.pre_1987_403b_balance {
            let exclusion_age = req.rmd_parameters.pre_1987_403b_rules.exclude_until_age;
            if owner_age < exclusion_age && pre_1987_balance > 0.0 {
                let excluded = pre_1987_balance.min(applicable_balance);
                applicable_balance -= excluded;
                trace.push(format!(
                    "pre_1987_403b_exclusion_applied={} exclusion_age={}",
                    round2(excluded),
                    exclusion_age
                ));
            }
        }
    }

    if applicable_balance <= 0.0 {
        return Ok(no_rmd(
            "owner_lifetime",
            applicable_balance,
            "no_distributable_balance",
            trace,
            Some(owner_age),
        ));
    }

    let use_joint_table = match (
        req.spouse_is_sole_beneficiary.unwrap_or(false),
        req.spouse_birth_date.as_ref(),
    ) {
        (true, Some(spouse_birth_date)) => {
            if let Some(spouse_age) = age_in_year(spouse_birth_date, calculation_year) {
                let spouse_more_than_10_younger = owner_age >= spouse_age + 11;
                trace.push(format!("spouse_age={}", spouse_age));
                trace.push(format!(
                    "spouse_more_than_10_younger={}",
                    spouse_more_than_10_younger
                ));
                spouse_more_than_10_younger
            } else {
                false
            }
        }
        _ => false,
    };

    let (distribution_period, table_used, rule_path) = if use_joint_table {
        let spouse_age = age_in_year(
            req.spouse_birth_date
                .as_ref()
                .expect("spouse_birth_date present by match"),
            calculation_year,
        )
        .ok_or_else(|| "spouse_birth_date is after calculation_year".to_string())?;

        let divisor =
            joint_distribution_period(&req.rmd_parameters.joint_life_table, owner_age, spouse_age)
                .ok_or_else(|| {
                    format!(
                        "No joint life distribution period found for owner_age={} spouse_age={}",
                        owner_age, spouse_age
                    )
                })?;
        (divisor, "joint_life_table", "owner_joint_life_table")
    } else {
        let divisor =
            age_distribution_period(&req.rmd_parameters.uniform_lifetime_table, owner_age)
                .ok_or_else(|| {
                    format!(
                        "No uniform lifetime distribution period found for age={}",
                        owner_age
                    )
                })?;
        (
            divisor,
            "uniform_lifetime_table",
            "owner_uniform_lifetime_table",
        )
    };

    let rmd_amount = applicable_balance / distribution_period;

    trace.push(format!("table_used={}", table_used));
    trace.push(format!(
        "distribution_period={}",
        round4(distribution_period)
    ));

    Ok(RmdComputation {
        scenario_class: "owner_lifetime".to_string(),
        rmd_required: true,
        rmd_amount,
        applicable_balance,
        distribution_period: Some(distribution_period),
        table_used: Some(table_used.to_string()),
        rule_path: rule_path.to_string(),
        decision_trace: trace,
        age: Some(owner_age),
    })
}

fn compute_beneficiary_rmd(
    req: &RetirementRmdRequest,
    calculation_year: u32,
    prior_year_end_balance: f64,
) -> Result<RmdComputation, String> {
    let mut trace = vec!["scenario=beneficiary_post_death".to_string()];

    let owner_death_year = req
        .owner_death_year
        .ok_or_else(|| "owner_death_year is required for beneficiary scenarios".to_string())?;

    if calculation_year <= owner_death_year {
        trace.push("calculation_year_before_or_equal_owner_death_year=true".to_string());
        return Ok(no_rmd(
            "beneficiary_post_death",
            prior_year_end_balance,
            "beneficiary_year_before_first_distribution",
            trace,
            None,
        ));
    }

    let beneficiary_class =
        normalize_value(req.beneficiary_class.as_ref().ok_or_else(|| {
            "beneficiary_class is required for beneficiary scenarios".to_string()
        })?);

    if !req
        .rmd_parameters
        .beneficiary_rules
        .beneficiary_classes
        .iter()
        .any(|c| normalize_value(c) == beneficiary_class)
    {
        return Err(format!(
            "beneficiary_class '{}' is not recognized by reference rules",
            beneficiary_class
        ));
    }

    trace.push(format!("beneficiary_class={}", beneficiary_class));

    let owner_died_before_rbd = req
        .owner_died_before_required_beginning_date
        .unwrap_or(false);
    let owner_died_on_or_after_rbd = !owner_died_before_rbd;

    let beneficiary_election = normalize_value(req.beneficiary_election.as_deref().unwrap_or(""));

    if beneficiary_class == "spouse" && beneficiary_election == "treat_as_own" {
        trace.push("beneficiary_election=treat_as_own".to_string());
        return compute_spousal_treat_as_own(req, calculation_year, prior_year_end_balance, trace);
    }

    let is_edb = req
        .rmd_parameters
        .beneficiary_rules
        .eligible_designated_beneficiary_classes
        .iter()
        .any(|c| normalize_value(c) == beneficiary_class);

    if beneficiary_class == "non_designated_beneficiary" {
        if owner_died_before_rbd {
            return compute_terminal_year_only(
                "beneficiary_post_death",
                prior_year_end_balance,
                calculation_year,
                owner_death_year,
                5,
                "beneficiary_non_designated_five_year_rule",
                trace,
                None,
            );
        }

        return compute_owner_remaining_life_expectancy(
            req,
            calculation_year,
            prior_year_end_balance,
            owner_death_year,
            trace,
        );
    }

    if beneficiary_class == "other_designated_beneficiary"
        || (!is_edb && beneficiary_class != "spouse")
    {
        return compute_ten_year_distribution(
            req,
            calculation_year,
            prior_year_end_balance,
            owner_death_year,
            owner_died_on_or_after_rbd,
            "beneficiary_ten_year_rule",
            trace,
            None,
        );
    }

    if beneficiary_class == "minor_child" {
        let majority_year = resolve_majority_year(req, owner_death_year)?;
        trace.push(format!("majority_year={}", majority_year));

        if calculation_year > majority_year {
            return compute_ten_year_distribution(
                req,
                calculation_year,
                prior_year_end_balance,
                majority_year,
                owner_died_on_or_after_rbd,
                "beneficiary_minor_child_post_majority_ten_year_rule",
                trace,
                req.beneficiary_birth_date
                    .as_ref()
                    .and_then(|d| age_in_year(d, calculation_year)),
            );
        }
    }

    if beneficiary_election == "ten_year" {
        return compute_ten_year_distribution(
            req,
            calculation_year,
            prior_year_end_balance,
            owner_death_year,
            owner_died_on_or_after_rbd,
            "beneficiary_elected_ten_year_rule",
            trace,
            req.beneficiary_birth_date
                .as_ref()
                .and_then(|d| age_in_year(d, calculation_year)),
        );
    }

    compute_life_expectancy_distribution(
        req,
        calculation_year,
        prior_year_end_balance,
        owner_death_year,
        beneficiary_class,
        trace,
    )
}

fn compute_spousal_treat_as_own(
    req: &RetirementRmdRequest,
    calculation_year: u32,
    prior_year_end_balance: f64,
    mut trace: Vec<String>,
) -> Result<RmdComputation, String> {
    let beneficiary_birth_date = req
        .beneficiary_birth_date
        .as_ref()
        .ok_or_else(|| "beneficiary_birth_date is required for spouse treat_as_own".to_string())?;
    let beneficiary_birth_year = parse_year(beneficiary_birth_date)
        .ok_or_else(|| "beneficiary_birth_date must be YYYY-MM-DD".to_string())?;
    let beneficiary_age = age_in_year(beneficiary_birth_date, calculation_year)
        .ok_or_else(|| "beneficiary_birth_date is after calculation_year".to_string())?;

    let start_age = find_start_age(
        beneficiary_birth_year,
        &req.rmd_parameters.required_beginning,
    )
    .ok_or_else(|| {
        format!(
            "No RMD start age rule matched beneficiary birth year {}",
            beneficiary_birth_year
        )
    })?;

    let start_year = beneficiary_birth_year + start_age;
    trace.push(format!("beneficiary_age={}", beneficiary_age));
    trace.push(format!("beneficiary_start_year={}", start_year));

    if calculation_year < start_year {
        return Ok(no_rmd(
            "beneficiary_post_death",
            prior_year_end_balance,
            "spouse_treat_as_own_before_start_year",
            trace,
            Some(beneficiary_age),
        ));
    }

    let divisor =
        age_distribution_period(&req.rmd_parameters.uniform_lifetime_table, beneficiary_age)
            .ok_or_else(|| {
                format!(
                    "No uniform lifetime distribution period found for beneficiary age={}",
                    beneficiary_age
                )
            })?;

    Ok(RmdComputation {
        scenario_class: "beneficiary_post_death".to_string(),
        rmd_required: true,
        rmd_amount: prior_year_end_balance / divisor,
        applicable_balance: prior_year_end_balance,
        distribution_period: Some(divisor),
        table_used: Some("uniform_lifetime_table".to_string()),
        rule_path: "spouse_treat_as_own_uniform_table".to_string(),
        decision_trace: trace,
        age: Some(beneficiary_age),
    })
}

fn compute_life_expectancy_distribution(
    req: &RetirementRmdRequest,
    calculation_year: u32,
    prior_year_end_balance: f64,
    owner_death_year: u32,
    beneficiary_class: String,
    mut trace: Vec<String>,
) -> Result<RmdComputation, String> {
    let beneficiary_birth_date = req.beneficiary_birth_date.as_ref().ok_or_else(|| {
        "beneficiary_birth_date is required for life-expectancy scenarios".to_string()
    })?;
    let beneficiary_age = age_in_year(beneficiary_birth_date, calculation_year)
        .ok_or_else(|| "beneficiary_birth_date is after calculation_year".to_string())?;

    let method = req
        .rmd_parameters
        .beneficiary_rules
        .life_expectancy_method_by_class
        .get(&beneficiary_class)
        .cloned()
        .unwrap_or_else(|| {
            if beneficiary_class == "spouse" {
                "recalculate_annually".to_string()
            } else {
                "subtract_one".to_string()
            }
        });

    let first_distribution_year = owner_death_year + 1;

    if beneficiary_class == "spouse"
        && method == "delay_until_owner_rbd"
        && req.rmd_parameters.beneficiary_rules.spouse_delay_allowed
    {
        let owner_birth_year = req
            .owner_birth_date
            .as_ref()
            .and_then(|d| parse_year(d))
            .ok_or_else(|| {
                "owner_birth_date is required for spouse delay-until-owner-rbd election".to_string()
            })?;

        let owner_start_age =
            find_start_age(owner_birth_year, &req.rmd_parameters.required_beginning).ok_or_else(
                || {
                    format!(
                        "No RMD start age rule matched owner birth year {}",
                        owner_birth_year
                    )
                },
            )?;

        let owner_start_year = owner_birth_year + owner_start_age;
        if calculation_year < owner_start_year {
            trace.push(format!(
                "spouse_delay_until_owner_rbd owner_start_year={}",
                owner_start_year
            ));
            return Ok(no_rmd(
                "beneficiary_post_death",
                prior_year_end_balance,
                "spouse_delay_until_owner_rbd",
                trace,
                Some(beneficiary_age),
            ));
        }
    }

    let divisor = if method == "recalculate_annually" {
        age_distribution_period(&req.rmd_parameters.single_life_table, beneficiary_age).ok_or_else(
            || {
                format!(
                    "No single life distribution period found for beneficiary age={}",
                    beneficiary_age
                )
            },
        )?
    } else {
        let base_age = age_in_year(beneficiary_birth_date, first_distribution_year)
            .ok_or_else(|| "beneficiary birth date is after first distribution year".to_string())?;
        let base_divisor = age_distribution_period(&req.rmd_parameters.single_life_table, base_age)
            .ok_or_else(|| {
                format!(
                    "No single life distribution period found for base beneficiary age={}",
                    base_age
                )
            })?;

        let years_elapsed = (calculation_year - first_distribution_year) as f64;
        (base_divisor - years_elapsed).max(1.0)
    };

    trace.push(format!("life_expectancy_method={}", method));

    Ok(RmdComputation {
        scenario_class: "beneficiary_post_death".to_string(),
        rmd_required: true,
        rmd_amount: prior_year_end_balance / divisor,
        applicable_balance: prior_year_end_balance,
        distribution_period: Some(divisor),
        table_used: Some("single_life_table".to_string()),
        rule_path: "beneficiary_life_expectancy".to_string(),
        decision_trace: trace,
        age: Some(beneficiary_age),
    })
}

fn compute_owner_remaining_life_expectancy(
    req: &RetirementRmdRequest,
    calculation_year: u32,
    prior_year_end_balance: f64,
    owner_death_year: u32,
    mut trace: Vec<String>,
) -> Result<RmdComputation, String> {
    let owner_birth_date = req.owner_birth_date.as_ref().ok_or_else(|| {
        "owner_birth_date is required for non-designated beneficiary after RBD".to_string()
    })?;

    let owner_age_at_death = age_in_year(owner_birth_date, owner_death_year)
        .ok_or_else(|| "owner_birth_date is after owner_death_year".to_string())?;

    let base_divisor =
        age_distribution_period(&req.rmd_parameters.single_life_table, owner_age_at_death)
            .ok_or_else(|| {
                format!(
                    "No single life distribution period found for owner age at death {}",
                    owner_age_at_death
                )
            })?;

    let first_distribution_year = owner_death_year + 1;
    let years_elapsed = (calculation_year - first_distribution_year) as f64;
    let divisor = (base_divisor - years_elapsed).max(1.0);

    trace.push(format!("owner_age_at_death={}", owner_age_at_death));
    trace.push(format!("base_divisor={}", round4(base_divisor)));

    Ok(RmdComputation {
        scenario_class: "beneficiary_post_death".to_string(),
        rmd_required: true,
        rmd_amount: prior_year_end_balance / divisor,
        applicable_balance: prior_year_end_balance,
        distribution_period: Some(divisor),
        table_used: Some("single_life_table".to_string()),
        rule_path: "beneficiary_non_designated_owner_remaining_life".to_string(),
        decision_trace: trace,
        age: None,
    })
}

#[allow(clippy::too_many_arguments)]
fn compute_ten_year_distribution(
    req: &RetirementRmdRequest,
    calculation_year: u32,
    prior_year_end_balance: f64,
    trigger_year: u32,
    owner_died_on_or_after_rbd: bool,
    rule_path: &str,
    mut trace: Vec<String>,
    age: Option<u32>,
) -> Result<RmdComputation, String> {
    let terminal_years = req.rmd_parameters.ten_year_rule.terminal_year;
    let years_elapsed = calculation_year.saturating_sub(trigger_year);

    trace.push(format!("trigger_year={}", trigger_year));
    trace.push(format!("years_elapsed={}", years_elapsed));
    trace.push(format!("terminal_years={}", terminal_years));

    if years_elapsed == 0 {
        return Ok(no_rmd(
            "beneficiary_post_death",
            prior_year_end_balance,
            "ten_year_initial_year",
            trace,
            age,
        ));
    }

    if years_elapsed >= terminal_years {
        return Ok(RmdComputation {
            scenario_class: "beneficiary_post_death".to_string(),
            rmd_required: true,
            rmd_amount: prior_year_end_balance,
            applicable_balance: prior_year_end_balance,
            distribution_period: Some(1.0),
            table_used: None,
            rule_path: format!("{}_terminal_distribution", rule_path),
            decision_trace: trace,
            age,
        });
    }

    let annual_required = req
        .rmd_parameters
        .ten_year_rule
        .annual_distributions_required_when_owner_died_on_or_after_rbd
        && owner_died_on_or_after_rbd
        && !req.rmd_parameters.relief_years.contains(&calculation_year);

    trace.push(format!("annual_required={}", annual_required));

    if !annual_required {
        return Ok(no_rmd(
            "beneficiary_post_death",
            prior_year_end_balance,
            &format!("{}_no_annual_distribution_required", rule_path),
            trace,
            age,
        ));
    }

    let beneficiary_birth_date = req.beneficiary_birth_date.as_ref().ok_or_else(|| {
        "beneficiary_birth_date is required when annual 10-year distributions apply".to_string()
    })?;
    let beneficiary_age = age_in_year(beneficiary_birth_date, calculation_year)
        .ok_or_else(|| "beneficiary_birth_date is after calculation_year".to_string())?;

    // TD 10001 §1.401(a)(9)-5(d)(1): use subtract-one method (initial life expectancy
    // from year after death, reduced by 1 each subsequent year).
    let first_distribution_year = trigger_year + 1;
    let base_age = age_in_year(beneficiary_birth_date, first_distribution_year)
        .ok_or_else(|| "beneficiary_birth_date is after first_distribution_year".to_string())?;
    let base_divisor = age_distribution_period(&req.rmd_parameters.single_life_table, base_age)
        .ok_or_else(|| {
            format!(
                "No single life distribution period found for base beneficiary age={}",
                base_age
            )
        })?;
    let years_elapsed = (calculation_year - first_distribution_year) as f64;
    let divisor = (base_divisor - years_elapsed).max(1.0);

    Ok(RmdComputation {
        scenario_class: "beneficiary_post_death".to_string(),
        rmd_required: true,
        rmd_amount: prior_year_end_balance / divisor,
        applicable_balance: prior_year_end_balance,
        distribution_period: Some(divisor),
        table_used: Some("single_life_table".to_string()),
        rule_path: format!("{}_annual_distribution", rule_path),
        decision_trace: trace,
        age: Some(beneficiary_age),
    })
}

#[allow(clippy::too_many_arguments)]
fn compute_terminal_year_only(
    scenario_class: &str,
    prior_year_end_balance: f64,
    calculation_year: u32,
    trigger_year: u32,
    terminal_years: u32,
    rule_path: &str,
    mut trace: Vec<String>,
    age: Option<u32>,
) -> Result<RmdComputation, String> {
    let years_elapsed = calculation_year.saturating_sub(trigger_year);
    trace.push(format!("trigger_year={}", trigger_year));
    trace.push(format!("years_elapsed={}", years_elapsed));
    trace.push(format!("terminal_years={}", terminal_years));

    if years_elapsed >= terminal_years {
        return Ok(RmdComputation {
            scenario_class: scenario_class.to_string(),
            rmd_required: true,
            rmd_amount: prior_year_end_balance,
            applicable_balance: prior_year_end_balance,
            distribution_period: Some(1.0),
            table_used: None,
            rule_path: format!("{}_terminal_distribution", rule_path),
            decision_trace: trace,
            age,
        });
    }

    Ok(no_rmd(
        scenario_class,
        prior_year_end_balance,
        &format!("{}_before_terminal_year", rule_path),
        trace,
        age,
    ))
}

fn resolve_majority_year(req: &RetirementRmdRequest, owner_death_year: u32) -> Result<u32, String> {
    if let Some(year) = req.beneficiary_majority_year {
        return Ok(year);
    }

    let beneficiary_birth_date = req
        .beneficiary_birth_date
        .as_ref()
        .ok_or_else(|| "beneficiary_birth_date or beneficiary_majority_year is required for minor_child scenarios".to_string())?;
    let birth_year = parse_year(beneficiary_birth_date)
        .ok_or_else(|| "beneficiary_birth_date must be YYYY-MM-DD".to_string())?;

    let majority_age = req
        .rmd_parameters
        .beneficiary_rules
        .minor_child_majority_age;
    let majority_year = birth_year + majority_age;

    Ok(majority_year.max(owner_death_year + 1))
}

fn applies_still_working_exception(req: &RetirementRmdRequest, account_type: &str) -> bool {
    if !req.is_still_working.unwrap_or(false) {
        return false;
    }

    let allowed_for_account = req
        .rmd_parameters
        .required_beginning
        .still_working_exception_account_types
        .iter()
        .any(|a| normalize_value(a) == account_type);

    if !allowed_for_account {
        return false;
    }

    if req
        .rmd_parameters
        .required_beginning
        .still_working_exception_disallowed_for_five_percent_owners
        && req.is_five_percent_owner.unwrap_or(false)
    {
        return false;
    }

    true
}

fn find_start_age(birth_year: u32, rules: &RequiredBeginningRules) -> Option<u32> {
    let mut matched: Option<u32> = None;

    for rule in &rules.start_age_rules {
        let min_ok = rule.birth_year_min.is_none_or(|min| birth_year >= min);
        let max_ok = rule.birth_year_max.is_none_or(|max| birth_year <= max);

        if min_ok && max_ok {
            matched = Some(rule.start_age);
        }
    }

    matched
}

fn age_distribution_period(table: &[AgeDistributionPeriod], age: u32) -> Option<f64> {
    table
        .iter()
        .find(|row| row.age == age)
        .map(|row| row.distribution_period)
}

fn joint_distribution_period(
    table: &[crate::models::retirement_rmd::JointDistributionPeriod],
    owner_age: u32,
    spouse_age: u32,
) -> Option<f64> {
    table
        .iter()
        .find(|row| row.owner_age == owner_age && row.spouse_age == spouse_age)
        .map(|row| row.distribution_period)
}

fn no_rmd(
    scenario_class: &str,
    applicable_balance: f64,
    rule_path: &str,
    decision_trace: Vec<String>,
    age: Option<u32>,
) -> RmdComputation {
    RmdComputation {
        scenario_class: scenario_class.to_string(),
        rmd_required: false,
        rmd_amount: 0.0,
        applicable_balance,
        distribution_period: None,
        table_used: None,
        rule_path: rule_path.to_string(),
        decision_trace,
        age,
    }
}

fn normalize_value(value: &str) -> String {
    value.trim().to_lowercase()
}

fn parse_year(date: &str) -> Option<u32> {
    let year = date.split('-').next()?.trim();
    year.parse::<u32>().ok()
}

fn age_in_year(date: &str, year: u32) -> Option<u32> {
    let birth_year = parse_year(date)?;
    if year < birth_year {
        return None;
    }

    Some(year - birth_year)
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn round4(v: f64) -> f64 {
    (v * 10000.0).round() / 10000.0
}

fn round6(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::retirement_rmd::*;
    use std::collections::HashMap;

    fn base_params() -> RmdParameters {
        RmdParameters {
            uniform_lifetime_table: vec![
                AgeDistributionPeriod {
                    age: 72,
                    distribution_period: 27.4,
                },
                AgeDistributionPeriod {
                    age: 73,
                    distribution_period: 26.5,
                },
                AgeDistributionPeriod {
                    age: 74,
                    distribution_period: 25.5,
                },
                AgeDistributionPeriod {
                    age: 75,
                    distribution_period: 24.6,
                },
                AgeDistributionPeriod {
                    age: 76,
                    distribution_period: 23.7,
                },
            ],
            joint_life_table: vec![
                JointDistributionPeriod {
                    owner_age: 75,
                    spouse_age: 64,
                    distribution_period: 27.4,
                },
                JointDistributionPeriod {
                    owner_age: 76,
                    spouse_age: 65,
                    distribution_period: 26.5,
                },
            ],
            single_life_table: vec![
                AgeDistributionPeriod {
                    age: 50,
                    distribution_period: 36.2,
                },
                AgeDistributionPeriod {
                    age: 51,
                    distribution_period: 35.3,
                },
                AgeDistributionPeriod {
                    age: 52,
                    distribution_period: 34.4,
                },
                AgeDistributionPeriod {
                    age: 75,
                    distribution_period: 14.8,
                },
                AgeDistributionPeriod {
                    age: 76,
                    distribution_period: 14.1,
                },
            ],
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
                still_working_exception_account_types: vec![
                    "401k".to_string(),
                    "403b".to_string(),
                    "457b".to_string(),
                    "designated_roth_plan_account".to_string(),
                ],
                still_working_exception_disallowed_for_five_percent_owners: true,
                designated_roth_owner_rmd_start_year: Some(2024),
            },
            account_rules: AccountRules {
                owner_required_account_types: vec![
                    "traditional_ira".to_string(),
                    "sep_ira".to_string(),
                    "simple_ira".to_string(),
                    "401k".to_string(),
                    "403b".to_string(),
                    "457b".to_string(),
                    "designated_roth_plan_account".to_string(),
                ],
                owner_exempt_account_types: vec!["roth_ira".to_string()],
                inherited_account_types: vec![
                    "inherited_ira".to_string(),
                    "inherited_plan".to_string(),
                ],
                supports_pre_1987_403b_exclusion: true,
            },
            beneficiary_rules: BeneficiaryRules {
                beneficiary_classes: vec![
                    "spouse".to_string(),
                    "minor_child".to_string(),
                    "disabled_or_chronically_ill".to_string(),
                    "not_more_than_10_years_younger".to_string(),
                    "other_designated_beneficiary".to_string(),
                    "non_designated_beneficiary".to_string(),
                ],
                eligible_designated_beneficiary_classes: vec![
                    "spouse".to_string(),
                    "minor_child".to_string(),
                    "disabled_or_chronically_ill".to_string(),
                    "not_more_than_10_years_younger".to_string(),
                ],
                life_expectancy_method_by_class: HashMap::from([
                    ("spouse".to_string(), "recalculate_annually".to_string()),
                    ("minor_child".to_string(), "subtract_one".to_string()),
                    (
                        "disabled_or_chronically_ill".to_string(),
                        "subtract_one".to_string(),
                    ),
                    (
                        "not_more_than_10_years_younger".to_string(),
                        "subtract_one".to_string(),
                    ),
                ]),
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

    fn base_owner_request() -> RetirementRmdRequest {
        RetirementRmdRequest {
            calculation_year: 2026,
            prior_year_end_balance: 100_000.0,
            account_type: "traditional_ira".to_string(),
            owner_birth_date: Some("1951-06-01".to_string()),
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
            rmd_parameters: base_params(),
        }
    }

    #[test]
    fn owner_uniform_table_rmd() {
        let req = base_owner_request();
        let result = run_retirement_rmd(&req).expect("expected owner rmd");

        assert!(result.rmd_required);
        assert_eq!("owner_uniform_lifetime_table", result.rule_path);
        assert_eq!(4065.04, result.rmd_amount);
    }

    #[test]
    fn owner_spouse_joint_table_when_spouse_more_than_ten_years_younger() {
        let mut req = base_owner_request();
        req.spouse_is_sole_beneficiary = Some(true);
        req.spouse_birth_date = Some("1962-01-01".to_string());

        let result = run_retirement_rmd(&req).expect("expected owner rmd with joint table");
        assert!(result.rmd_required);
        assert_eq!("owner_joint_life_table", result.rule_path);
        assert_eq!(3649.64, result.rmd_amount);
    }

    #[test]
    fn owner_roth_ira_not_required() {
        let mut req = base_owner_request();
        req.account_type = "roth_ira".to_string();

        let result = run_retirement_rmd(&req).expect("expected owner exempt result");
        assert!(!result.rmd_required);
        assert_eq!("owner_account_exempt", result.rule_path);
        assert_eq!(0.0, result.rmd_amount);
    }

    #[test]
    fn beneficiary_ten_year_terminal_distribution() {
        let mut req = base_owner_request();
        req.account_type = "inherited_ira".to_string();
        req.owner_is_alive = Some(false);
        req.owner_death_year = Some(2016);
        req.owner_died_before_required_beginning_date = Some(true);
        req.beneficiary_class = Some("other_designated_beneficiary".to_string());
        req.beneficiary_birth_date = Some("1980-01-01".to_string());
        req.calculation_year = 2026;

        let result = run_retirement_rmd(&req).expect("expected 10 year terminal distribution");
        assert!(result.rmd_required);
        assert!(result.rule_path.contains("terminal_distribution"));
        assert_eq!(100_000.0, result.rmd_amount);
    }

    #[test]
    fn non_designated_before_rbd_uses_five_year_rule() {
        let mut req = base_owner_request();
        req.account_type = "inherited_ira".to_string();
        req.owner_is_alive = Some(false);
        req.owner_death_year = Some(2024);
        req.owner_died_before_required_beginning_date = Some(true);
        req.beneficiary_class = Some("non_designated_beneficiary".to_string());
        req.calculation_year = 2026;

        let result = run_retirement_rmd(&req).expect("expected non-designated beneficiary result");
        assert!(!result.rmd_required);
        assert!(result
            .rule_path
            .contains("beneficiary_non_designated_five_year_rule"));
    }

    #[test]
    fn schedule_applies_rmd_then_growth() {
        let req = RetirementRmdScheduleRequest {
            calculation_year: 2026,
            prior_year_end_balance: 100_000.0,
            account_type: "traditional_ira".to_string(),
            owner_birth_date: Some("1951-06-01".to_string()),
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
            annual_growth_rate: Some(0.05),
            end_age: Some(76),
            max_years: Some(5),
            rmd_parameters: base_params(),
        };

        let result = run_retirement_rmd_schedule(&req).expect("expected schedule");
        assert_eq!(2, result.rows.len());

        // year 1: (100000 - 4065.04) * 1.05 = 100731.71
        assert_eq!(100731.71, result.rows[0].end_balance);
    }
}
