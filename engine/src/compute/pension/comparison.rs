use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

use crate::compute::retirement::roth_conversion::compute_taxable_ss;
use crate::compute::tax::federal::run_federal_tax;
use crate::models::pension_request::*;
use crate::models::pension_response::*;
use crate::models::roth_conversion::UniformLifetimeEntry;
use crate::models::tax_request::{FederalTaxRequest, IncomeBreakdown};
use crate::models::tax_response::FederalTaxResponse;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn round4(v: f64) -> f64 {
    (v * 10000.0).round() / 10000.0
}

/// Select the best mortality table for a given gender.
/// Uses gender-specific table if provided and available, falls back to unisex.
pub fn select_mortality_table<'a>(
    table: &'a MortalityTable,
    gender: Option<&str>,
) -> &'a [MortalityEntry] {
    match gender {
        Some("male") => table.male_table.as_deref().unwrap_or(&table.unisex_table),
        Some("female") => table.female_table.as_deref().unwrap_or(&table.unisex_table),
        _ => &table.unisex_table,
    }
}

/// Look up qx for a given age from a mortality table.
fn lookup_qx(table: &[MortalityEntry], age: u32) -> f64 {
    table
        .iter()
        .find(|e| e.age == age)
        .map(|e| e.qx)
        .unwrap_or(1.0) // if age not found, assume certain death
}

/// Compute monthly survival probabilities from a qx table.
/// Returns Vec of length max_months+1 where survival[0] = 1.0 (alive at start).
/// survival[m] = probability of surviving from start_age through month m.
pub fn compute_monthly_survival(
    qx_table: &[MortalityEntry],
    start_age: u32,
    max_months: u32,
) -> Vec<f64> {
    let mut survival = Vec::with_capacity(max_months as usize + 1);
    survival.push(1.0);

    let mut cumulative = 1.0;
    for m in 0..max_months {
        let current_age = start_age + m / 12;
        let qx = lookup_qx(qx_table, current_age);
        // Monthly survival rate: (1 - qx)^(1/12)
        let monthly_rate = (1.0 - qx).powf(1.0 / 12.0);
        cumulative *= monthly_rate;
        survival.push(cumulative);
    }

    survival
}

/// Compute the expected death age (50% survival age) from survival curve.
fn expected_death_age(survival: &[f64], start_age: u32) -> u32 {
    for (m, &s) in survival.iter().enumerate() {
        if s < 0.5 {
            return start_age + (m as u32) / 12;
        }
    }
    start_age + (survival.len() as u32) / 12
}

fn option_id(opt: &AnnuityOption, idx: usize) -> String {
    opt.id
        .as_deref()
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("option-{}", idx + 1))
}

fn selected_option(req: &PensionComparisonRequest) -> (usize, String, &AnnuityOption) {
    if let Some(ref selected_id) = req.selected_option_id {
        if let Some((idx, opt)) = req
            .annuity_options
            .iter()
            .enumerate()
            .find(|(_, opt)| opt.id.as_deref().map(str::trim) == Some(selected_id.trim()))
        {
            return (idx, option_id(opt, idx), opt);
        }
    }

    let opt = req
        .annuity_options
        .first()
        .expect("validated pension comparison request has at least one option");
    (0, option_id(opt, 0), opt)
}

fn annual_payment_for_year(
    option: &AnnuityOption,
    year_index: u32,
    global_cola_rate: f64,
    start_age: u32,
) -> f64 {
    let start_month = year_index * 12 + 1;
    let end_month = (year_index + 1) * 12;
    (start_month..=end_month)
        .map(|month| compute_payment(option, month, global_cola_rate, start_age))
        .sum()
}

fn simplified_method_expected_payments(retiree_age: u32, spouse_age: Option<u32>) -> f64 {
    match spouse_age {
        Some(spouse) => match retiree_age + spouse {
            0..=110 => 410.0,
            111..=120 => 360.0,
            121..=130 => 310.0,
            131..=140 => 260.0,
            _ => 210.0,
        },
        None => match retiree_age {
            0..=54 => 360.0,
            55..=59 => 310.0,
            60..=64 => 260.0,
            65..=69 => 210.0,
            _ => 160.0,
        },
    }
}

fn monthly_annuity_basis_exclusion(basis: f64, retiree_age: u32, spouse_age: Option<u32>) -> f64 {
    if basis <= 0.0 {
        return 0.0;
    }
    basis / simplified_method_expected_payments(retiree_age, spouse_age)
}

fn annual_basis_exclusion(
    remaining_basis: &mut f64,
    monthly_exclusion: f64,
    months: u32,
    annual_gross: f64,
) -> f64 {
    if *remaining_basis <= 0.0 || monthly_exclusion <= 0.0 || annual_gross <= 0.0 {
        return 0.0;
    }

    let exclusion = (monthly_exclusion * months as f64)
        .min(*remaining_basis)
        .min(annual_gross);
    *remaining_basis = (*remaining_basis - exclusion).max(0.0);
    exclusion
}

fn early_distribution_penalty(req: &PensionComparisonRequest, taxable_amount: f64) -> f64 {
    if taxable_amount <= 0.0 {
        return 0.0;
    }

    if req.retiree_age < 55 {
        return taxable_amount * 0.10;
    }

    if req.retiree_age < 60 {
        let has_rule55 = req.separation_age.is_some_and(|sa| sa >= 55);
        if !has_rule55 {
            return taxable_amount * 0.10;
        }
    }

    0.0
}

fn approximate_first_rmd_age(req: &PensionComparisonRequest) -> u32 {
    let approx_birth_year = req.tax_year.saturating_sub(req.retiree_age);
    if approx_birth_year >= 1960 {
        75
    } else {
        73
    }
}

fn lookup_distribution_period(table: Option<&[UniformLifetimeEntry]>, age: u32) -> f64 {
    if let Some(rows) = table {
        if let Some(row) = rows.iter().find(|row| row.age == age) {
            return row.distribution_period;
        }
        if let Some(last) = rows.last() {
            if age > last.age {
                return last.distribution_period;
            }
        }
    }

    match age {
        72 => 27.4,
        73 => 26.5,
        74 => 25.5,
        75 => 24.6,
        76 => 23.7,
        77 => 22.9,
        78 => 22.0,
        79 => 21.1,
        80 => 20.2,
        81 => 19.4,
        82 => 18.5,
        83 => 17.7,
        84 => 16.8,
        85 => 16.0,
        86 => 15.2,
        87 => 14.4,
        88 => 13.7,
        89 => 12.9,
        90 => 12.2,
        _ if age > 90 => (120.0 - age as f64).max(1.9),
        _ => 27.4,
    }
}

fn taxable_account_total_yield(assumptions: &TaxableAccountAssumptions) -> f64 {
    assumptions.taxable_interest_yield
        + assumptions.ordinary_dividend_yield
        + assumptions.qualified_dividend_yield
        + assumptions.short_term_capital_gain_yield
        + assumptions.long_term_capital_gain_yield
}

fn household_horizon_years(
    req: &PensionComparisonRequest,
    retiree_surv: &[f64],
    spouse_surv: Option<&[f64]>,
) -> (u32, u32, Option<u32>) {
    let retiree_expected_death_age = req
        .retiree_life_expectancy_override
        .unwrap_or_else(|| expected_death_age(retiree_surv, req.retiree_age));
    let retiree_horizon_years = retiree_expected_death_age.saturating_sub(req.retiree_age);

    let spouse_expected_death_age = req.spouse_age.map(|spouse_age| {
        req.spouse_life_expectancy_override.unwrap_or_else(|| {
            spouse_surv
                .map(|curve| expected_death_age(curve, spouse_age))
                .unwrap_or(spouse_age)
        })
    });

    let spouse_horizon_years = req
        .spouse_age
        .zip(spouse_expected_death_age)
        .map(|(spouse_age, death_age)| death_age.saturating_sub(spouse_age))
        .unwrap_or(0);

    (
        retiree_horizon_years.max(spouse_horizon_years),
        retiree_expected_death_age,
        spouse_expected_death_age,
    )
}

// ---------------------------------------------------------------------------
// Option resolution + COLA/payment helpers
// ---------------------------------------------------------------------------

/// Resolve aliases to canonical option types and compute derived fields.
/// Returns a new AnnuityOption with normalized option_type and filled defaults.
pub fn resolve_option(opt: &AnnuityOption) -> AnnuityOption {
    let mut resolved = opt.clone();

    match opt.option_type.as_str() {
        "joint_50" => {
            resolved.option_type = "joint_survivor".into();
            if resolved.survivor_pct.is_none() && resolved.survivor_monthly_payment.is_none() {
                resolved.survivor_pct = Some(0.5);
            }
        }
        "joint_75" => {
            resolved.option_type = "joint_survivor".into();
            if resolved.survivor_pct.is_none() && resolved.survivor_monthly_payment.is_none() {
                resolved.survivor_pct = Some(0.75);
            }
        }
        "joint_100" => {
            resolved.option_type = "joint_survivor".into();
            if resolved.survivor_pct.is_none() && resolved.survivor_monthly_payment.is_none() {
                resolved.survivor_pct = Some(1.0);
            }
        }
        "period_certain_10" => {
            resolved.option_type = "period_certain".into();
            if resolved.period_certain_months.is_none() {
                resolved.period_certain_months = Some(120);
            }
        }
        "period_certain_20" => {
            resolved.option_type = "period_certain".into();
            if resolved.period_certain_months.is_none() {
                resolved.period_certain_months = Some(240);
            }
        }
        _ => {}
    }

    // For joint_survivor: compute survivor_monthly_payment from survivor_pct if not explicit
    if resolved.option_type == "joint_survivor" && resolved.survivor_monthly_payment.is_none() {
        if let Some(pct) = resolved.survivor_pct {
            resolved.survivor_monthly_payment = Some(opt.monthly_payment * pct);
        }
    }

    resolved
}

/// Apply COLA to a base payment amount for a given month.
fn apply_cola(base: f64, month: u32, cola: &ColaConfig, start_age: u32) -> f64 {
    if month < cola.start_delay_months {
        return base;
    }
    let cola_month = month - cola.start_delay_months;

    // Determine effective COLA months (freeze at end_age if applicable)
    let effective_cola_months = if let Some(end_age) = cola.end_age {
        let age_at_month = start_age + month / 12;
        if age_at_month > end_age {
            // Freeze at level reached at end_age
            let months_to_end = (end_age.saturating_sub(start_age)) * 12;
            months_to_end.saturating_sub(cola.start_delay_months)
        } else {
            cola_month
        }
    } else {
        cola_month
    };

    let year = effective_cola_months / 12;
    let rate = match cola.cap_rate {
        Some(cap) => cola.rate.min(cap),
        None => cola.rate,
    };
    base * (1.0 + rate).powi(year as i32)
}

/// Compute the payment amount for a given month, handling COLA + level income.
/// Uses per-option COLA if present, otherwise falls back to global cola_rate.
fn compute_payment(
    option: &AnnuityOption,
    month: u32,
    global_cola_rate: f64,
    start_age: u32,
) -> f64 {
    // Level income: switch to reduced payment after level_income_age
    let base = if option.option_type == "level_income" {
        if let (Some(li_age), Some(li_payment)) =
            (option.level_income_age, option.level_income_reduced_payment)
        {
            let age_at_month = start_age + month / 12;
            if age_at_month >= li_age {
                li_payment
            } else {
                option.monthly_payment
            }
        } else {
            option.monthly_payment
        }
    } else {
        option.monthly_payment
    };

    // Apply COLA
    if let Some(ref cola) = option.cola {
        apply_cola(base, month, cola, start_age)
    } else if global_cola_rate != 0.0 {
        let global_cola = ColaConfig {
            rate: global_cola_rate,
            start_delay_months: 0,
            cap_rate: None,
            end_age: None,
        };
        apply_cola(base, month, &global_cola, start_age)
    } else {
        base
    }
}

/// Compute survivor payment for a given month (applies same COLA logic).
fn compute_survivor_payment(
    option: &AnnuityOption,
    month: u32,
    global_cola_rate: f64,
    start_age: u32,
) -> f64 {
    let base = option.survivor_monthly_payment.unwrap_or(0.0);
    if let Some(ref cola) = option.cola {
        apply_cola(base, month, cola, start_age)
    } else if global_cola_rate != 0.0 {
        let global_cola = ColaConfig {
            rate: global_cola_rate,
            start_delay_months: 0,
            cap_rate: None,
            end_age: None,
        };
        apply_cola(base, month, &global_cola, start_age)
    } else {
        base
    }
}

/// Compute popup payment for a given month.
fn compute_popup_payment(
    option: &AnnuityOption,
    month: u32,
    global_cola_rate: f64,
    start_age: u32,
) -> f64 {
    let base = option
        .popup_monthly_payment
        .unwrap_or(option.monthly_payment);
    if let Some(ref cola) = option.cola {
        apply_cola(base, month, cola, start_age)
    } else if global_cola_rate != 0.0 {
        let global_cola = ColaConfig {
            rate: global_cola_rate,
            start_delay_months: 0,
            cap_rate: None,
            end_age: None,
        };
        apply_cola(base, month, &global_cola, start_age)
    } else {
        base
    }
}

// ---------------------------------------------------------------------------
// Annuity PV calculations
// ---------------------------------------------------------------------------

/// Compute present value of an annuity option.
pub fn annuity_pv(
    option: &AnnuityOption,
    retiree_surv: &[f64],
    spouse_surv: Option<&[f64]>,
    discount_rate: f64,
    cola_rate: f64,
    max_months: u32,
    start_age: u32,
) -> f64 {
    let resolved = resolve_option(option);
    let monthly_discount = (1.0 + discount_rate).powf(1.0 / 12.0);
    let months = max_months.min(retiree_surv.len() as u32 - 1);

    match resolved.option_type.as_str() {
        "single_life" | "level_income" => {
            let mut pv = 0.0;
            for m in 1..=months {
                let payment = compute_payment(&resolved, m, cola_rate, start_age);
                let discount = monthly_discount.powi(-(m as i32));
                pv += payment * retiree_surv[m as usize] * discount;
            }
            pv
        }
        "joint_survivor" => {
            let spouse_s = match spouse_surv {
                Some(s) => s,
                None => return 0.0,
            };
            let joint_months = months.min(spouse_s.len() as u32 - 1);
            let has_popup = resolved.popup_monthly_payment.is_some();

            let mut pv = 0.0;
            for m in 1..=joint_months {
                let discount = monthly_discount.powi(-(m as i32));
                let p_retiree_alive = retiree_surv[m as usize];
                let p_retiree_dead = 1.0 - p_retiree_alive;
                let p_spouse_alive = spouse_s[m as usize];
                let p_spouse_dead = 1.0 - p_spouse_alive;

                if has_popup {
                    // Both alive: normal payment
                    let payment = compute_payment(&resolved, m, cola_rate, start_age);
                    pv += payment * p_retiree_alive * p_spouse_alive * discount;

                    // Retiree alive, spouse dead: popup payment
                    let popup_pay = compute_popup_payment(&resolved, m, cola_rate, start_age);
                    pv += popup_pay * p_retiree_alive * p_spouse_dead * discount;

                    // Retiree dead, spouse alive: survivor payment
                    let surv_pay = compute_survivor_payment(&resolved, m, cola_rate, start_age);
                    pv += surv_pay * p_retiree_dead * p_spouse_alive * discount;
                } else {
                    // Without popup: payment while retiree alive (simplified)
                    let payment = compute_payment(&resolved, m, cola_rate, start_age);
                    pv += payment * p_retiree_alive * discount;

                    // Survivor payment when retiree dead, spouse alive
                    let surv_pay = compute_survivor_payment(&resolved, m, cola_rate, start_age);
                    pv += surv_pay * p_retiree_dead * p_spouse_alive * discount;
                }
            }
            pv
        }
        "period_certain" => {
            let certain_months = resolved.period_certain_months.unwrap_or(0);
            let mut pv = 0.0;
            for m in 1..=months {
                let payment = compute_payment(&resolved, m, cola_rate, start_age);
                let discount = monthly_discount.powi(-(m as i32));

                if m <= certain_months {
                    // Guaranteed period: no mortality weighting
                    pv += payment * discount;
                } else {
                    // After guarantee: mortality-weighted
                    pv += payment * retiree_surv[m as usize] * discount;
                }
            }
            pv
        }
        _ => {
            // Default: treat as single life
            let mut pv = 0.0;
            for m in 1..=months {
                let payment = compute_payment(&resolved, m, cola_rate, start_age);
                let discount = monthly_discount.powi(-(m as i32));
                pv += payment * retiree_surv[m as usize] * discount;
            }
            pv
        }
    }
}

// ---------------------------------------------------------------------------
// Break-even age
// ---------------------------------------------------------------------------

/// Invest lump sum, withdraw monthly payment, find age when balance hits zero.
/// Returns None if balance never depletes within max_months.
pub fn break_even_age(
    lump_sum: f64,
    option: &AnnuityOption,
    annual_return: f64,
    cola_rate: f64,
    start_age: u32,
    max_months: u32,
) -> Option<u32> {
    let resolved = resolve_option(option);
    let monthly_return = (1.0 + annual_return).powf(1.0 / 12.0) - 1.0;
    let mut balance = lump_sum;

    for m in 1..=max_months {
        let withdrawal = compute_payment(&resolved, m, cola_rate, start_age);
        balance *= 1.0 + monthly_return;
        balance -= withdrawal;
        if balance <= 0.0 {
            return Some(start_age + m / 12);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Monte Carlo: lump sum portfolio simulation
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
pub fn run_lump_sum_mc(
    lump_sum: f64,
    option: &AnnuityOption,
    cola_rate: f64,
    return_assumption: &ReturnAssumption,
    _retiree_surv: &[f64],
    max_months: u32,
    num_sims: u32,
    seed: u64,
    start_age: u32,
) -> LumpSumMonteCarloResult {
    let resolved = resolve_option(option);
    let monthly_mean = (1.0 + return_assumption.annual_mean).powf(1.0 / 12.0) - 1.0;
    let monthly_std = return_assumption.annual_std_dev / 12.0_f64.sqrt();

    // Run simulations in parallel
    let paths: Vec<Vec<f64>> = (0..num_sims)
        .into_par_iter()
        .map(|i| {
            let mut rng = ChaCha8Rng::seed_from_u64(seed.wrapping_add(i as u64));
            let normal = Normal::new(monthly_mean, monthly_std).unwrap();

            let mut balance = lump_sum;
            let mut path = Vec::with_capacity(max_months as usize + 1);
            path.push(balance);

            for m in 1..=max_months {
                let withdrawal = compute_payment(&resolved, m, cola_rate, start_age);
                let ret: f64 = normal.sample(&mut rng);
                balance *= 1.0 + ret;
                balance -= withdrawal;
                if balance < 0.0 {
                    balance = 0.0;
                }
                path.push(balance);
            }
            path
        })
        .collect();

    // Compute depletion months and terminal balances
    let mut depletion_ages: Vec<f64> = Vec::new();
    let mut terminal_balances: Vec<f64> = Vec::new();
    let mut annuity_wins = 0u32;

    for path in &paths {
        let terminal = *path.last().unwrap_or(&0.0);
        terminal_balances.push(terminal);

        // Find depletion month
        let depletion_month = path
            .iter()
            .skip(1)
            .position(|&b| b <= 0.0)
            .map(|m| m as u32 + 1);

        match depletion_month {
            Some(m) => {
                depletion_ages.push((start_age + m / 12) as f64);
                annuity_wins += 1; // portfolio depleted = annuity wins
            }
            None => {
                depletion_ages.push((start_age + max_months / 12) as f64);
            }
        }
    }

    // Success is defined against the configured projection horizon.
    let success_count = paths
        .iter()
        .filter(|p| {
            let idx = max_months as usize;
            p.get(idx).is_some_and(|&b| b > 0.0)
        })
        .count();
    let success_rate = success_count as f64 / num_sims as f64;

    depletion_ages.sort_by(|a, b| a.partial_cmp(b).unwrap());
    terminal_balances.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let depletion_age_percentiles = compute_percentiles(&depletion_ages);
    let residual_estate_percentiles = compute_percentiles(&terminal_balances);

    // Build time series (yearly snapshots)
    let mut time_series = Vec::new();
    let max_years = max_months / 12;
    let mut annuity_cumulative = 0.0;

    for y in 0..=max_years {
        let m = (y * 12) as usize;
        if y > 0 {
            // Sum 12 months of payments for this year
            let year_start_month = (y - 1) * 12 + 1;
            let year_end_month = y * 12;
            for month in year_start_month..=year_end_month {
                annuity_cumulative += compute_payment(&resolved, month, cola_rate, start_age);
            }
        }

        let mut balances_at_m: Vec<f64> = paths.iter().map(|p| *p.get(m).unwrap_or(&0.0)).collect();
        balances_at_m.sort_by(|a, b| a.partial_cmp(b).unwrap());

        time_series.push(TimeSeriesPoint {
            age: start_age + y,
            annuity_cumulative: round2(annuity_cumulative),
            lump_sum_balance_p10: round2(percentile_val(&balances_at_m, 10.0)),
            lump_sum_balance_p50: round2(percentile_val(&balances_at_m, 50.0)),
            lump_sum_balance_p90: round2(percentile_val(&balances_at_m, 90.0)),
        });
    }

    LumpSumMonteCarloResult {
        starting_balance: round2(lump_sum),
        monthly_withdrawal: round2(resolved.monthly_payment),
        num_simulations: num_sims,
        success_rate: round4(success_rate),
        depletion_age_percentiles,
        residual_estate_percentiles,
        probability_annuity_wins: round4(annuity_wins as f64 / num_sims as f64),
        time_series,
    }
}

fn compute_percentiles(sorted: &[f64]) -> Percentiles {
    Percentiles {
        p10: round2(percentile_val(sorted, 10.0)),
        p25: round2(percentile_val(sorted, 25.0)),
        p50: round2(percentile_val(sorted, 50.0)),
        p75: round2(percentile_val(sorted, 75.0)),
        p90: round2(percentile_val(sorted, 90.0)),
    }
}

fn percentile_val(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }
    let idx = p / 100.0 * (sorted.len() - 1) as f64;
    let lower = idx.floor() as usize;
    let upper = idx.ceil() as usize;
    if lower == upper {
        sorted[lower]
    } else {
        let frac = idx - lower as f64;
        sorted[lower] * (1.0 - frac) + sorted[upper] * frac
    }
}

#[derive(Debug, Clone, Copy)]
struct TaxSnapshot {
    total_income_tax: f64,
    effective_rate: f64,
    marginal_rate: f64,
    taxable_ss: f64,
    tax_delta: f64,
}

#[derive(Debug, Clone, Copy)]
struct FirstYearAnnuityTax {
    annual_gross: f64,
    snapshot: TaxSnapshot,
}

fn dynamic_taxable_ss(req: &PensionComparisonRequest, income: &IncomeBreakdown) -> f64 {
    req.ss_taxation_params
        .as_ref()
        .map(|params| {
            compute_taxable_ss(
                req.gross_social_security_benefit.unwrap_or(0.0),
                compute_agi_excl_ss(income),
                income.tax_exempt_interest,
                params,
            )
        })
        .unwrap_or(req.income.taxable_social_security)
}

fn run_tax_request(
    req: &PensionComparisonRequest,
    mut income: IncomeBreakdown,
) -> (FederalTaxResponse, f64) {
    let taxable_ss = dynamic_taxable_ss(req, &income);
    income.taxable_social_security = taxable_ss;
    let tax_req = FederalTaxRequest {
        filing_status: req.filing_status.clone(),
        income,
        adjustments: req.adjustments.clone(),
        deductions: req.deductions.clone(),
        tax_parameters: req.tax_parameters.clone(),
    };
    (run_federal_tax(&tax_req), taxable_ss)
}

fn baseline_tax_snapshot(req: &PensionComparisonRequest) -> TaxSnapshot {
    let (result, taxable_ss) = run_tax_request(req, req.income.clone());
    TaxSnapshot {
        total_income_tax: result.total_income_tax,
        effective_rate: result.effective_rate,
        marginal_rate: result.marginal_ordinary_rate,
        taxable_ss,
        tax_delta: 0.0,
    }
}

fn scenario_tax_snapshot(
    req: &PensionComparisonRequest,
    income: IncomeBreakdown,
    baseline: &TaxSnapshot,
    penalty: f64,
) -> TaxSnapshot {
    let (result, taxable_ss) = run_tax_request(req, income);
    TaxSnapshot {
        total_income_tax: result.total_income_tax + penalty,
        effective_rate: if penalty == 0.0 {
            result.effective_rate
        } else {
            (result.total_income_tax + penalty) / result.agi.max(1.0)
        },
        marginal_rate: result.marginal_ordinary_rate,
        taxable_ss,
        tax_delta: (result.total_income_tax - baseline.total_income_tax) + penalty,
    }
}

fn first_year_annuity_tax(
    req: &PensionComparisonRequest,
    option: &AnnuityOption,
    baseline: &TaxSnapshot,
) -> FirstYearAnnuityTax {
    let resolved = resolve_option(option);
    let annual_gross = annual_payment_for_year(&resolved, 0, req.cola_rate, req.retiree_age);
    let spouse_age_for_basis = if resolved.option_type == "joint_survivor" {
        req.spouse_age
    } else {
        None
    };
    let mut remaining_basis = req.after_tax_contribution_basis.max(0.0);
    let monthly_exclusion =
        monthly_annuity_basis_exclusion(remaining_basis, req.retiree_age, spouse_age_for_basis);
    let exclusion =
        annual_basis_exclusion(&mut remaining_basis, monthly_exclusion, 12, annual_gross);
    let taxable_pension = (annual_gross - exclusion).max(0.0);

    let mut income = req.income.clone();
    income.taxable_pensions += taxable_pension;

    FirstYearAnnuityTax {
        annual_gross,
        snapshot: scenario_tax_snapshot(req, income, baseline, 0.0),
    }
}

fn first_year_cashout_tax(
    req: &PensionComparisonRequest,
    baseline: &TaxSnapshot,
    amount: f64,
) -> (f64, TaxSnapshot) {
    let taxable_amount = (amount - req.after_tax_contribution_basis.max(0.0)).max(0.0);
    let mut income = req.income.clone();
    income.taxable_pensions += taxable_amount;
    let penalty = early_distribution_penalty(req, taxable_amount);
    let snapshot = scenario_tax_snapshot(req, income, baseline, penalty);
    (taxable_amount, snapshot)
}

fn project_annuity_path(
    req: &PensionComparisonRequest,
    option: &AnnuityOption,
    horizon_years: u32,
    baseline: &TaxSnapshot,
) -> AfterTaxScenarioProjection {
    let resolved = resolve_option(option);
    let spouse_age_for_basis = if resolved.option_type == "joint_survivor" {
        req.spouse_age
    } else {
        None
    };
    let mut remaining_basis = req.after_tax_contribution_basis.max(0.0);
    let monthly_exclusion =
        monthly_annuity_basis_exclusion(remaining_basis, req.retiree_age, spouse_age_for_basis);
    let mut cumulative_after_tax_nominal = 0.0;
    let mut cumulative_after_tax_real = 0.0;
    let mut cumulative_tax = 0.0;

    for year in 0..horizon_years {
        let annual_gross = annual_payment_for_year(&resolved, year, req.cola_rate, req.retiree_age);
        let exclusion =
            annual_basis_exclusion(&mut remaining_basis, monthly_exclusion, 12, annual_gross);
        let taxable_pension = (annual_gross - exclusion).max(0.0);

        let mut income = req.income.clone();
        income.taxable_pensions += taxable_pension;
        let snapshot = scenario_tax_snapshot(req, income, baseline, 0.0);
        let after_tax_income = (annual_gross - snapshot.tax_delta).max(0.0);

        cumulative_after_tax_nominal += after_tax_income;
        cumulative_after_tax_real +=
            after_tax_income / (1.0 + req.inflation_rate).powi(year as i32);
        cumulative_tax += snapshot.tax_delta;
    }

    AfterTaxScenarioProjection {
        cumulative_after_tax_income_nominal: round2(cumulative_after_tax_nominal),
        cumulative_after_tax_income_real: round2(cumulative_after_tax_real),
        cumulative_tax: round2(cumulative_tax),
        first_rmd_age: None,
        first_rmd_amount: None,
        net_investable_amount: None,
    }
}

fn project_ira_rollover_path(
    req: &PensionComparisonRequest,
    option: &AnnuityOption,
    horizon_years: u32,
    baseline: &TaxSnapshot,
) -> Option<AfterTaxScenarioProjection> {
    let mut balance = req.lump_sum_amount?;
    let resolved = resolve_option(option);
    let growth_factor = (1.0 + req.investment_return.annual_mean).max(0.0);
    let rmd_age = approximate_first_rmd_age(req);
    let mut remaining_basis = req.after_tax_contribution_basis.max(0.0);
    let mut cumulative_after_tax_nominal = 0.0;
    let mut cumulative_after_tax_real = 0.0;
    let mut cumulative_tax = 0.0;
    let mut first_rmd_age = None;
    let mut first_rmd_amount = None;

    for year in 0..horizon_years {
        let age = req.retiree_age + year;
        let annual_target =
            annual_payment_for_year(&resolved, year, req.cola_rate, req.retiree_age);
        let rmd_amount = if age >= rmd_age {
            balance / lookup_distribution_period(req.uniform_lifetime_table.as_deref(), age)
        } else {
            0.0
        };
        if first_rmd_age.is_none() && rmd_amount > 0.0 {
            first_rmd_age = Some(age);
            first_rmd_amount = Some(round2(rmd_amount));
        }

        let gross_distribution = annual_target.max(rmd_amount).min(balance);
        let nontaxable_distribution = if balance > 0.0 && remaining_basis > 0.0 {
            let basis_ratio = (remaining_basis / balance).clamp(0.0, 1.0);
            let nontaxable = gross_distribution * basis_ratio;
            remaining_basis = (remaining_basis - nontaxable).max(0.0);
            nontaxable
        } else {
            0.0
        };
        let taxable_distribution = (gross_distribution - nontaxable_distribution).max(0.0);

        let mut income = req.income.clone();
        income.taxable_ira_distributions += taxable_distribution;
        let snapshot = scenario_tax_snapshot(req, income, baseline, 0.0);
        let after_tax_income = (gross_distribution - snapshot.tax_delta).max(0.0);

        cumulative_after_tax_nominal += after_tax_income;
        cumulative_after_tax_real +=
            after_tax_income / (1.0 + req.inflation_rate).powi(year as i32);
        cumulative_tax += snapshot.tax_delta;

        balance = (balance - gross_distribution).max(0.0) * growth_factor;
    }

    Some(AfterTaxScenarioProjection {
        cumulative_after_tax_income_nominal: round2(cumulative_after_tax_nominal),
        cumulative_after_tax_income_real: round2(cumulative_after_tax_real),
        cumulative_tax: round2(cumulative_tax),
        first_rmd_age,
        first_rmd_amount,
        net_investable_amount: None,
    })
}

fn project_cash_out_path(
    req: &PensionComparisonRequest,
    option: &AnnuityOption,
    horizon_years: u32,
    baseline: &TaxSnapshot,
) -> Option<AfterTaxScenarioProjection> {
    let amount = req.lump_sum_amount?;
    let (_, upfront_tax) = first_year_cashout_tax(req, baseline, amount);
    let net_investable = (amount - upfront_tax.tax_delta.max(0.0)).max(0.0);
    let resolved = resolve_option(option);
    let assumptions = &req.taxable_account_assumptions;
    let growth_factor = (1.0 + req.investment_return.annual_mean).max(0.0);
    let taxable_yield_total = taxable_account_total_yield(assumptions);
    let mut balance = net_investable;
    let mut basis = net_investable;
    let mut cumulative_after_tax_nominal = 0.0;
    let mut cumulative_after_tax_real = 0.0;
    let mut cumulative_tax = upfront_tax.tax_delta.max(0.0);

    for year in 0..horizon_years {
        let start_balance = balance;
        let annual_target =
            annual_payment_for_year(&resolved, year, req.cola_rate, req.retiree_age);
        let market_value_before_withdrawal = start_balance * growth_factor;
        let basis_before_withdrawal = basis + start_balance * taxable_yield_total;
        let withdrawal = annual_target.min(market_value_before_withdrawal.max(0.0));
        let sale_gain_or_loss = if market_value_before_withdrawal > 0.0 {
            ((market_value_before_withdrawal - basis_before_withdrawal)
                / market_value_before_withdrawal)
                * withdrawal
        } else {
            0.0
        };
        let basis_reduction = if market_value_before_withdrawal > 0.0 {
            basis_before_withdrawal * (withdrawal / market_value_before_withdrawal)
        } else {
            0.0
        };

        let mut income = req.income.clone();
        income.taxable_interest += start_balance * assumptions.taxable_interest_yield;
        income.ordinary_dividends += start_balance * assumptions.ordinary_dividend_yield;
        income.qualified_dividends += start_balance * assumptions.qualified_dividend_yield;
        income.short_term_capital_gains +=
            start_balance * assumptions.short_term_capital_gain_yield;
        income.long_term_capital_gains +=
            start_balance * assumptions.long_term_capital_gain_yield + sale_gain_or_loss;

        let snapshot = scenario_tax_snapshot(req, income, baseline, 0.0);
        let taxes_paid = snapshot.tax_delta.max(0.0);
        let after_tax_income = (withdrawal - taxes_paid).max(0.0);

        cumulative_after_tax_nominal += after_tax_income;
        cumulative_after_tax_real +=
            after_tax_income / (1.0 + req.inflation_rate).powi(year as i32);
        cumulative_tax += taxes_paid;

        balance = (market_value_before_withdrawal - withdrawal - taxes_paid).max(0.0);
        basis = (basis_before_withdrawal - basis_reduction).max(0.0);
    }

    Some(AfterTaxScenarioProjection {
        cumulative_after_tax_income_nominal: round2(cumulative_after_tax_nominal),
        cumulative_after_tax_income_real: round2(cumulative_after_tax_real),
        cumulative_tax: round2(cumulative_tax),
        first_rmd_age: None,
        first_rmd_amount: None,
        net_investable_amount: Some(round2(net_investable)),
    })
}

// ---------------------------------------------------------------------------
// Tax scenarios
// ---------------------------------------------------------------------------

pub fn compute_tax_scenarios(req: &PensionComparisonRequest) -> TaxImpactSummary {
    let baseline = baseline_tax_snapshot(req);
    let (_, _, selected) = selected_option(req);
    let annuity_tax = first_year_annuity_tax(req, selected, &baseline);

    let annuity_scenario = TaxScenario {
        first_year_total_tax: round2(annuity_tax.snapshot.total_income_tax),
        first_year_effective_rate: round4(annuity_tax.snapshot.effective_rate),
        first_year_marginal_rate: annuity_tax.snapshot.marginal_rate,
        annual_tax_on_pension_income: round2(annuity_tax.snapshot.tax_delta),
        upfront_tax_cost: 0.0,
        ss_taxable_amount: round2(annuity_tax.snapshot.taxable_ss),
    };

    let lump_sum_ira_rollover = req.lump_sum_amount.map(|_| TaxScenario {
        first_year_total_tax: round2(baseline.total_income_tax),
        first_year_effective_rate: round4(baseline.effective_rate),
        first_year_marginal_rate: baseline.marginal_rate,
        annual_tax_on_pension_income: 0.0,
        upfront_tax_cost: 0.0,
        ss_taxable_amount: round2(baseline.taxable_ss),
    });

    let lump_sum_cash_out = req.lump_sum_amount.map(|amount| {
        let (_, cashout_tax) = first_year_cashout_tax(req, &baseline, amount);
        TaxScenario {
            first_year_total_tax: round2(cashout_tax.total_income_tax),
            first_year_effective_rate: round4(cashout_tax.effective_rate),
            first_year_marginal_rate: cashout_tax.marginal_rate,
            annual_tax_on_pension_income: round2(cashout_tax.tax_delta),
            upfront_tax_cost: round2(cashout_tax.tax_delta.max(0.0)),
            ss_taxable_amount: round2(cashout_tax.taxable_ss),
        }
    });

    TaxImpactSummary {
        annuity_scenario,
        lump_sum_ira_rollover,
        lump_sum_cash_out,
    }
}

/// Compute AGI excluding Social Security (for provisional income calculation)
fn compute_agi_excl_ss(income: &IncomeBreakdown) -> f64 {
    income.wages
        + income.self_employment_income
        + income.taxable_interest
        + income.ordinary_dividends
        + income.qualified_dividends
        + income.short_term_capital_gains
        + income.long_term_capital_gains
        + income.taxable_ira_distributions
        + income.taxable_pensions
        + income.other_income
}

// ---------------------------------------------------------------------------
// Survivor analysis
// ---------------------------------------------------------------------------

pub fn compute_survivor_analysis(
    req: &PensionComparisonRequest,
    option: &AnnuityOption,
    option_id_value: &str,
    retiree_surv: &[f64],
    spouse_surv: &[f64],
    spouse_expected_death_age: u32,
    mc_paths: Option<&Vec<Vec<f64>>>,
) -> SurvivorAnalysis {
    let retiree_death_age = req
        .retiree_life_expectancy_override
        .unwrap_or_else(|| expected_death_age(retiree_surv, req.retiree_age));

    let spouse_age = req.spouse_age.unwrap_or(req.retiree_age);
    let death_month = ((retiree_death_age - req.retiree_age) * 12) as usize;

    let discount = (1.0 + req.discount_rate).powf(1.0 / 12.0);
    let resolved = resolve_option(option);
    let mut survivor_payment_base = 0.0;
    let mut pv = 0.0;
    let max_survivor_months = (spouse_expected_death_age.saturating_sub(req.retiree_age)) * 12;

    if resolved.option_type == "joint_survivor" {
        survivor_payment_base = resolved.survivor_monthly_payment.unwrap_or(0.0);
        if survivor_payment_base > 0.0 {
            for m in 1..=max_survivor_months {
                let spouse_surv_idx = death_month + m as usize;
                if spouse_surv_idx >= spouse_surv.len() {
                    break;
                }
                let spouse_surv_at_death = spouse_surv[death_month.min(spouse_surv.len() - 1)];
                if spouse_surv_at_death <= 0.0 {
                    break;
                }
                let cond_surv = spouse_surv[spouse_surv_idx] / spouse_surv_at_death;
                let total_month = (retiree_death_age - req.retiree_age) * 12 + m;
                let payment = compute_survivor_payment(
                    &resolved,
                    total_month,
                    req.cola_rate,
                    req.retiree_age,
                );
                pv += payment * cond_surv * discount.powi(-(m as i32));
            }
        }
    } else if resolved.option_type == "period_certain" {
        let certain_months = resolved.period_certain_months.unwrap_or(0);
        if death_month < certain_months as usize {
            survivor_payment_base = compute_payment(
                &resolved,
                death_month as u32 + 1,
                req.cola_rate,
                req.retiree_age,
            );
            for total_month in (death_month as u32 + 1)..=certain_months {
                let payment =
                    compute_payment(&resolved, total_month, req.cola_rate, req.retiree_age);
                let months_after_death = total_month - death_month as u32;
                pv += payment * discount.powi(-(months_after_death as i32));
            }
        }
    }

    let lump_sum_residual_p50 = mc_paths.map(|paths| {
        let mut balances: Vec<f64> = paths
            .iter()
            .map(|p| *p.get(death_month).unwrap_or(&0.0))
            .collect();
        balances.sort_by(|a, b| a.partial_cmp(b).unwrap());
        round2(percentile_val(&balances, 50.0))
    });

    SurvivorAnalysis {
        spouse_age,
        spouse_expected_death_age,
        retiree_expected_death_age: retiree_death_age,
        options: vec![SurvivorOption {
            id: option_id_value.to_owned(),
            option_type: option.option_type.clone(),
            label: option.label.clone(),
            survivor_monthly_income: round2(survivor_payment_base),
            survivor_income_pv: round2(pv),
            lump_sum_residual_p50,
        }],
    }
}

// ---------------------------------------------------------------------------
// Cumulative income
// ---------------------------------------------------------------------------

pub fn build_cumulative_income(
    req: &PensionComparisonRequest,
    option: &AnnuityOption,
    horizon_years: u32,
) -> Vec<CumulativeIncomeYear> {
    let selected_option = resolve_option(option);

    let mut annuity_cum_nominal = 0.0;
    let mut annuity_cum_real = 0.0;
    let mut withdrawal_cum = 0.0;

    let mut result = Vec::new();

    for y in 0..=horizon_years {
        if y > 0 {
            // Sum 12 months of payments for this year
            let year_start_month = (y - 1) * 12 + 1;
            let year_end_month = y * 12;
            let mut annual_nominal = 0.0;
            for month in year_start_month..=year_end_month {
                annual_nominal +=
                    compute_payment(&selected_option, month, req.cola_rate, req.retiree_age);
            }
            annuity_cum_nominal += annual_nominal;

            let real_payment = annual_nominal / (1.0 + req.inflation_rate).powi(y as i32);
            annuity_cum_real += real_payment;

            withdrawal_cum += annual_nominal; // lump sum withdrawal matches annuity
        }

        result.push(CumulativeIncomeYear {
            age: req.retiree_age + y,
            year_index: y,
            annuity_cumulative_nominal: round2(annuity_cum_nominal),
            annuity_cumulative_real: round2(annuity_cum_real),
            lump_sum_withdrawal_cumulative: round2(withdrawal_cum),
        });
    }

    result
}

// ---------------------------------------------------------------------------
// Top-level orchestrator
// ---------------------------------------------------------------------------

pub fn run_pension_comparison(req: &PensionComparisonRequest) -> PensionComparisonResponse {
    let max_months = (120u32.saturating_sub(req.retiree_age)) * 12;

    // Build survival curves
    let retiree_table = select_mortality_table(&req.mortality_table, req.retiree_gender.as_deref());
    let retiree_surv = compute_monthly_survival(retiree_table, req.retiree_age, max_months);

    let spouse_surv = req.spouse_age.map(|sa| {
        let spouse_table =
            select_mortality_table(&req.mortality_table, req.spouse_gender.as_deref());
        let spouse_max = (120u32.saturating_sub(sa)) * 12;
        compute_monthly_survival(spouse_table, sa, spouse_max.max(max_months))
    });
    let (_, selected_option_id_used, selected) = selected_option(req);
    let (projection_horizon_years, retiree_expected_death_age, spouse_expected_death_age) =
        household_horizon_years(req, &retiree_surv, spouse_surv.as_deref());
    let projection_months = projection_horizon_years * 12;
    let projection_horizon_age = req.retiree_age + projection_horizon_years;
    let baseline_tax = baseline_tax_snapshot(req);
    let cashout_tax_snapshot = req
        .lump_sum_amount
        .map(|amount| first_year_cashout_tax(req, &baseline_tax, amount).1);
    let effective_lump_sum = req.lump_sum_amount.map(|amount| {
        if req.rollover_to_ira {
            amount
        } else {
            let upfront_tax = cashout_tax_snapshot
                .as_ref()
                .map(|snapshot| snapshot.tax_delta.max(0.0))
                .unwrap_or(0.0);
            (amount - upfront_tax).max(0.0)
        }
    });

    // Analyze each annuity option
    let options_analyzed: Vec<OptionAnalysis> = req
        .annuity_options
        .iter()
        .enumerate()
        .map(|(idx, opt)| {
            let resolved = resolve_option(opt);
            let pv = annuity_pv(
                opt,
                &retiree_surv,
                spouse_surv.as_deref(),
                req.discount_rate,
                req.cola_rate,
                max_months,
                req.retiree_age,
            );

            let be_age = effective_lump_sum.and_then(|ls| {
                break_even_age(
                    ls,
                    opt,
                    req.investment_return.annual_mean,
                    req.cola_rate,
                    req.retiree_age,
                    max_months,
                )
            });

            let life_exp_years = retiree_expected_death_age.saturating_sub(req.retiree_age);

            // Lifetime income uses compute_payment for accurate COLA/level_income
            let lifetime_nominal: f64 = (0..life_exp_years)
                .map(|y| {
                    let year_start = y * 12 + 1;
                    let year_end = (y + 1) * 12;
                    (year_start..=year_end)
                        .map(|m| compute_payment(&resolved, m, req.cola_rate, req.retiree_age))
                        .sum::<f64>()
                })
                .sum();

            let lifetime_real: f64 = (0..life_exp_years)
                .map(|y| {
                    let year_start = y * 12 + 1;
                    let year_end = (y + 1) * 12;
                    let annual: f64 = (year_start..=year_end)
                        .map(|m| compute_payment(&resolved, m, req.cola_rate, req.retiree_age))
                        .sum();
                    annual / (1.0 + req.inflation_rate).powi(y as i32)
                })
                .sum();

            let first_year_tax = first_year_annuity_tax(req, opt, &baseline_tax);
            let annual_tax = first_year_tax.snapshot.tax_delta;
            let eff_rate = if first_year_tax.annual_gross > 0.0 {
                annual_tax / first_year_tax.annual_gross
            } else {
                0.0
            };

            let pv_after_tax = pv * (1.0 - eff_rate);

            // Survivor PV for joint options
            let survivor_pv = if matches!(
                resolved.option_type.as_str(),
                "joint_survivor" | "period_certain"
            ) {
                spouse_surv.as_ref().and_then(|ss| {
                    spouse_expected_death_age.map(|spouse_death_age| {
                        compute_survivor_analysis(
                            req,
                            opt,
                            &option_id(opt, idx),
                            &retiree_surv,
                            ss,
                            spouse_death_age,
                            None,
                        )
                        .options[0]
                            .survivor_income_pv
                    })
                })
            } else {
                None
            };

            OptionAnalysis {
                id: option_id(opt, idx),
                option_type: opt.option_type.clone(),
                label: opt.label.clone(),
                monthly_payment: round2(opt.monthly_payment),
                present_value: round2(pv),
                present_value_after_tax: round2(pv_after_tax),
                break_even_age: be_age,
                lifetime_income_nominal: round2(lifetime_nominal),
                lifetime_income_real: round2(lifetime_real),
                annual_tax_estimate: round2(annual_tax),
                effective_tax_rate: round4(eff_rate),
                survivor_pv,
            }
        })
        .collect();

    let mc_result = effective_lump_sum.map(|ls| {
        let num_sims = req.num_simulations.unwrap_or(10000);
        let seed = req.seed.unwrap_or_else(|| rand::thread_rng().gen());

        run_lump_sum_mc(
            ls,
            selected,
            req.cola_rate,
            &req.investment_return,
            &retiree_surv,
            projection_months,
            num_sims,
            seed,
            req.retiree_age,
        )
    });

    // Comparison summary
    let comparison_summary: Vec<ComparisonRow> = options_analyzed
        .iter()
        .map(|oa| ComparisonRow {
            id: oa.id.clone(),
            option_type: oa.option_type.clone(),
            label: oa.label.clone(),
            present_value: oa.present_value,
            break_even_age: oa.break_even_age,
            lifetime_income_nominal: oa.lifetime_income_nominal,
            annual_tax_estimate: oa.annual_tax_estimate,
        })
        .collect();

    // Cumulative income
    let cumulative_income_by_year =
        build_cumulative_income(req, selected, projection_horizon_years);

    // Tax impact
    let tax_impact = compute_tax_scenarios(req);
    let after_tax_projection = AfterTaxProjection {
        selected_option_id: selected_option_id_used.clone(),
        horizon_age: projection_horizon_age,
        annuity: project_annuity_path(req, selected, projection_horizon_years, &baseline_tax),
        lump_sum_ira_rollover: project_ira_rollover_path(
            req,
            selected,
            projection_horizon_years,
            &baseline_tax,
        ),
        lump_sum_cash_out: project_cash_out_path(
            req,
            selected,
            projection_horizon_years,
            &baseline_tax,
        ),
    };

    // Survivor analysis
    let survivor_analysis = req.spouse_age.map(|_| {
        compute_survivor_analysis(
            req,
            selected,
            &selected_option_id_used,
            &retiree_surv,
            spouse_surv.as_ref().unwrap(),
            spouse_expected_death_age.unwrap_or(req.retiree_age),
            None,
        )
    });

    PensionComparisonResponse {
        selected_option_id_used,
        options_analyzed,
        lump_sum_monte_carlo: mc_result,
        comparison_summary,
        cumulative_income_by_year,
        tax_impact,
        after_tax_projection,
        survivor_analysis,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tax_request::*;

    fn sample_mortality_entries() -> Vec<MortalityEntry> {
        // Simplified table: ages 0-120 with increasing qx
        (0..=120)
            .map(|age| {
                let qx = if age == 0 {
                    0.005
                } else if age < 30 {
                    0.001 + (age as f64) * 0.0001
                } else {
                    0.001 + (age as f64 - 20.0).powi(2) * 0.00005
                };
                MortalityEntry {
                    age,
                    qx: qx.min(1.0),
                }
            })
            .collect()
    }

    fn make_mortality_table() -> MortalityTable {
        let entries = sample_mortality_entries();
        MortalityTable {
            male_table: Some(entries.clone()),
            female_table: Some(
                entries
                    .iter()
                    .map(|e| MortalityEntry {
                        age: e.age,
                        qx: e.qx * 0.85, // female mortality ~15% lower
                    })
                    .collect(),
            ),
            unisex_table: entries,
        }
    }

    fn zero_qx_table() -> Vec<MortalityEntry> {
        (0..=120)
            .map(|age| MortalityEntry { age, qx: 0.0 })
            .collect()
    }

    fn make_option(option_type: &str, monthly_payment: f64) -> AnnuityOption {
        AnnuityOption {
            id: None,
            option_type: option_type.into(),
            label: None,
            monthly_payment,
            survivor_pct: None,
            survivor_monthly_payment: None,
            popup_monthly_payment: None,
            period_certain_months: None,
            level_income_age: None,
            level_income_reduced_payment: None,
            cola: None,
        }
    }

    fn single_ordinary_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                min: 0.0,
                max: Some(11925.0),
                rate: 0.10,
            },
            TaxBracket {
                min: 11925.0,
                max: Some(48475.0),
                rate: 0.12,
            },
            TaxBracket {
                min: 48475.0,
                max: Some(103350.0),
                rate: 0.22,
            },
            TaxBracket {
                min: 103350.0,
                max: Some(197300.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 197300.0,
                max: Some(250525.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 250525.0,
                max: Some(626350.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 626350.0,
                max: None,
                rate: 0.37,
            },
        ]
    }

    fn single_cg_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                min: 0.0,
                max: Some(48350.0),
                rate: 0.0,
            },
            TaxBracket {
                min: 48350.0,
                max: Some(533400.0),
                rate: 0.15,
            },
            TaxBracket {
                min: 533400.0,
                max: None,
                rate: 0.20,
            },
        ]
    }

    fn default_payroll() -> PayrollParams {
        PayrollParams {
            social_security_rate: 0.062,
            social_security_wage_base: 176100.0,
            self_employment_tax_rate: 0.124,
            medicare_rate: 0.0145,
            self_employment_medicare_rate: 0.029,
            additional_medicare_rate: 0.009,
            additional_medicare_threshold: 200000.0,
        }
    }

    fn default_salt() -> SaltDeductionParams {
        SaltDeductionParams {
            cap_amount: 40400.0,
            phaseout_threshold: 505000.0,
            phaseout_rate: 0.30,
            floor_amount: 10000.0,
        }
    }

    fn default_tax_parameters() -> TaxParameters {
        TaxParameters {
            ordinary_brackets: single_ordinary_brackets(),
            capital_gains_brackets: single_cg_brackets(),
            standard_deduction: 15000.0,
            capital_loss_limit: 3000.0,
            niit: NiitParams {
                rate: 0.038,
                threshold: 200000.0,
            },
            payroll: default_payroll(),
            salt: Some(default_salt()),
        }
    }

    fn zero_income() -> IncomeBreakdown {
        IncomeBreakdown {
            wages: 0.0,
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
        }
    }

    // --- Mortality / survival tests ---

    #[test]
    fn test_select_mortality_table_male() {
        let table = make_mortality_table();
        let selected = select_mortality_table(&table, Some("male"));
        assert_eq!(selected.len(), table.male_table.as_ref().unwrap().len());
    }

    #[test]
    fn test_select_mortality_table_no_gender_uses_unisex() {
        let table = make_mortality_table();
        let selected = select_mortality_table(&table, None);
        assert_eq!(selected.len(), table.unisex_table.len());
    }

    #[test]
    fn test_select_mortality_table_fallback_to_unisex() {
        let table = MortalityTable {
            male_table: None,
            female_table: None,
            unisex_table: sample_mortality_entries(),
        };
        let selected = select_mortality_table(&table, Some("male"));
        assert_eq!(selected.len(), table.unisex_table.len());
    }

    #[test]
    fn test_compute_monthly_survival_starts_at_one() {
        let entries = sample_mortality_entries();
        let surv = compute_monthly_survival(&entries, 65, 120);
        assert_eq!(surv[0], 1.0);
        assert!(surv.len() == 121);
    }

    #[test]
    fn test_compute_monthly_survival_decreasing() {
        let entries = sample_mortality_entries();
        let surv = compute_monthly_survival(&entries, 65, 360);
        for i in 1..surv.len() {
            assert!(
                surv[i] <= surv[i - 1],
                "Survival should be non-increasing at month {}",
                i
            );
        }
    }

    #[test]
    fn test_compute_monthly_survival_zero_qx() {
        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 65, 120);
        // With zero mortality, survival should remain 1.0
        for &s in &surv {
            assert!((s - 1.0).abs() < 1e-10);
        }
    }

    // --- Annuity PV tests ---

    #[test]
    fn test_annuity_pv_single_life_zero_discount_zero_mortality() {
        let option = make_option("single_life", 1000.0);
        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 65, 120);

        // Zero discount, zero mortality: PV = payment * months
        let pv = annuity_pv(&option, &surv, None, 0.0, 0.0, 120, 65);
        assert!(
            (pv - 120000.0).abs() < 1.0,
            "PV should be ~120000, got {}",
            pv
        );
    }

    #[test]
    fn test_annuity_pv_single_life_with_discount() {
        let option = make_option("single_life", 1000.0);
        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 65, 120);

        let pv_no_disc = annuity_pv(&option, &surv, None, 0.0, 0.0, 120, 65);
        let pv_with_disc = annuity_pv(&option, &surv, None, 0.05, 0.0, 120, 65);

        assert!(pv_with_disc < pv_no_disc, "Discount should reduce PV");
    }

    #[test]
    fn test_annuity_pv_single_life_with_mortality() {
        let option = make_option("single_life", 1000.0);

        let entries_zero = zero_qx_table();
        let surv_zero = compute_monthly_survival(&entries_zero, 65, 360);

        let entries_real = sample_mortality_entries();
        let surv_real = compute_monthly_survival(&entries_real, 65, 360);

        let pv_no_mort = annuity_pv(&option, &surv_zero, None, 0.05, 0.0, 360, 65);
        let pv_with_mort = annuity_pv(&option, &surv_real, None, 0.05, 0.0, 360, 65);

        assert!(
            pv_with_mort < pv_no_mort,
            "Mortality should reduce PV: {} vs {}",
            pv_with_mort,
            pv_no_mort
        );
    }

    #[test]
    fn test_annuity_pv_with_cola() {
        let option = make_option("single_life", 1000.0);
        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 65, 240);

        let pv_no_cola = annuity_pv(&option, &surv, None, 0.0, 0.0, 240, 65);
        let pv_with_cola = annuity_pv(&option, &surv, None, 0.0, 0.03, 240, 65);

        assert!(
            pv_with_cola > pv_no_cola,
            "COLA should increase PV: {} vs {}",
            pv_with_cola,
            pv_no_cola
        );
    }

    #[test]
    fn test_annuity_pv_joint_survivor() {
        let mut option = make_option("joint_100", 2000.0);
        option.survivor_monthly_payment = Some(2000.0);
        let entries = zero_qx_table();
        let retiree_surv = compute_monthly_survival(&entries, 65, 240);
        let spouse_surv = compute_monthly_survival(&entries, 62, 240);

        let pv = annuity_pv(
            &option,
            &retiree_surv,
            Some(&spouse_surv),
            0.0,
            0.0,
            240,
            65,
        );

        // With zero mortality for both: full payment always paid (retiree alive),
        // plus survivor payment * P(retiree dead) * P(spouse alive) = 2000 * 0 * 1 = 0
        // So PV should be approximately 2000 * 240 = 480000
        assert!(
            (pv - 480000.0).abs() < 1.0,
            "Joint with zero mortality: PV should be ~480000, got {}",
            pv
        );
    }

    #[test]
    fn test_annuity_pv_joint_50_vs_100() {
        let entries = sample_mortality_entries();
        let retiree_surv = compute_monthly_survival(&entries, 65, 360);
        let spouse_surv = compute_monthly_survival(&entries, 62, 360);

        let mut opt_50 = make_option("joint_50", 2000.0);
        opt_50.survivor_monthly_payment = Some(1000.0);

        let mut opt_100 = make_option("joint_100", 2000.0);
        opt_100.survivor_monthly_payment = Some(2000.0);

        let pv_50 = annuity_pv(
            &opt_50,
            &retiree_surv,
            Some(&spouse_surv),
            0.05,
            0.0,
            360,
            65,
        );
        let pv_100 = annuity_pv(
            &opt_100,
            &retiree_surv,
            Some(&spouse_surv),
            0.05,
            0.0,
            360,
            65,
        );

        assert!(
            pv_100 > pv_50,
            "100% survivor should have higher PV than 50%: {} vs {}",
            pv_100,
            pv_50
        );
    }

    #[test]
    fn test_annuity_pv_period_certain() {
        let mut option = make_option("period_certain_10", 1000.0);
        option.period_certain_months = Some(120);

        // Use high mortality to make the difference visible
        let entries: Vec<MortalityEntry> = (0..=120)
            .map(|age| MortalityEntry {
                age,
                qx: if age >= 65 { 0.10 } else { 0.001 },
            })
            .collect();
        let surv = compute_monthly_survival(&entries, 65, 360);

        let pv_certain = annuity_pv(&option, &surv, None, 0.0, 0.0, 360, 65);

        // Single life without guarantee
        let single_opt = make_option("single_life", 1000.0);
        let pv_single = annuity_pv(&single_opt, &surv, None, 0.0, 0.0, 360, 65);

        assert!(
            pv_certain > pv_single,
            "Period certain should have higher PV with high mortality: {} vs {}",
            pv_certain,
            pv_single
        );

        // First 120 months guaranteed = at least $120,000
        assert!(pv_certain >= 120000.0);
    }

    // --- Break-even tests ---

    #[test]
    fn test_break_even_zero_return() {
        // $300,000 lump sum, $2,000/mo withdrawal, 0% return
        // Should deplete in 300000/2000 = 150 months = 12.5 years = age 77
        let option = make_option("single_life", 2000.0);
        let be = break_even_age(300_000.0, &option, 0.0, 0.0, 65, 600);
        assert_eq!(be, Some(77));
    }

    #[test]
    fn test_break_even_positive_return() {
        let option = make_option("single_life", 2000.0);
        let be_zero = break_even_age(300_000.0, &option, 0.0, 0.0, 65, 600);
        let be_pos = break_even_age(300_000.0, &option, 0.05, 0.0, 65, 600);

        // Positive return should push break-even later
        assert!(be_pos.unwrap() > be_zero.unwrap());
    }

    #[test]
    fn test_break_even_never_depletes() {
        // Very high return relative to withdrawal
        let option = make_option("single_life", 100.0);
        let be = break_even_age(1_000_000.0, &option, 0.10, 0.0, 65, 600);
        assert_eq!(be, None);
    }

    #[test]
    fn test_break_even_with_cola() {
        let option = make_option("single_life", 2000.0);
        let be_no_cola = break_even_age(300_000.0, &option, 0.04, 0.0, 65, 600);
        let be_cola = break_even_age(300_000.0, &option, 0.04, 0.03, 65, 600);

        // COLA increases withdrawals, so should deplete sooner
        assert!(
            be_cola.unwrap() < be_no_cola.unwrap(),
            "COLA should cause earlier depletion: {} vs {}",
            be_cola.unwrap(),
            be_no_cola.unwrap()
        );
    }

    // --- Monte Carlo tests ---

    #[test]
    fn test_mc_deterministic_seed() {
        let entries = sample_mortality_entries();
        let surv = compute_monthly_survival(&entries, 65, 360);
        let ret = ReturnAssumption {
            annual_mean: 0.07,
            annual_std_dev: 0.15,
        };
        let option = make_option("single_life", 2000.0);

        let r1 = run_lump_sum_mc(500_000.0, &option, 0.0, &ret, &surv, 360, 100, 12345, 65);
        let r2 = run_lump_sum_mc(500_000.0, &option, 0.0, &ret, &surv, 360, 100, 12345, 65);

        assert_eq!(r1.success_rate, r2.success_rate);
        assert_eq!(
            r1.residual_estate_percentiles.p50,
            r2.residual_estate_percentiles.p50
        );
    }

    #[test]
    fn test_mc_high_return_higher_success() {
        let entries = sample_mortality_entries();
        let surv = compute_monthly_survival(&entries, 65, 360);
        let option = make_option("single_life", 3000.0);

        let low_ret = ReturnAssumption {
            annual_mean: 0.03,
            annual_std_dev: 0.10,
        };
        let high_ret = ReturnAssumption {
            annual_mean: 0.10,
            annual_std_dev: 0.10,
        };

        let r_low = run_lump_sum_mc(500_000.0, &option, 0.0, &low_ret, &surv, 360, 500, 42, 65);
        let r_high = run_lump_sum_mc(500_000.0, &option, 0.0, &high_ret, &surv, 360, 500, 42, 65);

        assert!(
            r_high.success_rate >= r_low.success_rate,
            "Higher return should have >= success rate: {} vs {}",
            r_high.success_rate,
            r_low.success_rate
        );
    }

    #[test]
    fn test_mc_success_rate_bounds() {
        let entries = sample_mortality_entries();
        let surv = compute_monthly_survival(&entries, 65, 360);
        let ret = ReturnAssumption {
            annual_mean: 0.07,
            annual_std_dev: 0.15,
        };
        let option = make_option("single_life", 2000.0);

        let result = run_lump_sum_mc(500_000.0, &option, 0.0, &ret, &surv, 360, 200, 42, 65);
        assert!(result.success_rate >= 0.0 && result.success_rate <= 1.0);
        assert!(result.probability_annuity_wins >= 0.0 && result.probability_annuity_wins <= 1.0);
    }

    #[test]
    fn test_mc_time_series_length() {
        let entries = sample_mortality_entries();
        let surv = compute_monthly_survival(&entries, 65, 360);
        let ret = ReturnAssumption {
            annual_mean: 0.07,
            annual_std_dev: 0.15,
        };
        let option = make_option("single_life", 2000.0);

        let result = run_lump_sum_mc(500_000.0, &option, 0.0, &ret, &surv, 360, 100, 42, 65);
        // 360 months / 12 = 30 years + 1 for starting point
        assert_eq!(result.time_series.len(), 31);
        assert_eq!(result.time_series[0].age, 65);
        assert_eq!(result.time_series[30].age, 95);
    }

    // --- Tax scenario tests ---

    fn make_test_req_with_options(opts: Vec<AnnuityOption>) -> PensionComparisonRequest {
        PensionComparisonRequest {
            retiree_age: 65,
            retiree_gender: None,
            spouse_age: None,
            spouse_gender: None,
            selected_option_id: None,
            lump_sum_amount: Some(300_000.0),
            annuity_options: opts,
            investment_return: ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            },
            discount_rate: 0.05,
            filing_status: "single".into(),
            tax_year: 2026,
            income: IncomeBreakdown {
                wages: 30000.0,
                ..zero_income()
            },
            adjustments: Adjustments::default(),
            deductions: DeductionConfig {
                method: "standard".into(),
                itemized_amount: None,
                spouse_itemizes: None,
                state_local_income_or_sales_tax: None,
                real_property_tax: None,
                personal_property_tax: None,
                other_itemized_deductions: None,
            },
            tax_parameters: default_tax_parameters(),
            gross_social_security_benefit: None,
            ss_taxation_params: None,
            inflation_rate: 0.025,
            cola_rate: 0.0,
            retiree_life_expectancy_override: None,
            spouse_life_expectancy_override: None,
            rollover_to_ira: true,
            after_tax_contribution_basis: 0.0,
            separation_age: None,
            taxable_account_assumptions: TaxableAccountAssumptions::default(),
            num_simulations: Some(100),
            seed: Some(42),
            mortality_table: make_mortality_table(),
            uniform_lifetime_table: None,
        }
    }

    #[test]
    fn test_tax_pension_income_increases_tax() {
        let req = make_test_req_with_options(vec![make_option("single_life", 2000.0)]);

        let tax = compute_tax_scenarios(&req);

        // Annuity adds $24,000/yr pension income → should have higher tax
        assert!(tax.annuity_scenario.first_year_total_tax > 0.0);
        assert!(tax.annuity_scenario.annual_tax_on_pension_income > 0.0);

        // IRA rollover has no upfront cost
        assert_eq!(
            tax.lump_sum_ira_rollover.as_ref().unwrap().upfront_tax_cost,
            0.0
        );

        // Cash-out should have significant upfront tax
        assert!(tax.lump_sum_cash_out.as_ref().unwrap().upfront_tax_cost > 0.0);
    }

    #[test]
    fn test_tax_cashout_penalty_under_55() {
        let mut req = make_test_req_with_options(vec![make_option("single_life", 1000.0)]);
        req.retiree_age = 50;
        req.income = zero_income();
        req.lump_sum_amount = Some(100_000.0);

        let tax = compute_tax_scenarios(&req);
        let cashout = tax.lump_sum_cash_out.unwrap();

        // Under 55, no Rule of 55 → 10% penalty on $100k = $10k penalty
        // Total upfront should include both income tax and penalty
        assert!(
            cashout.upfront_tax_cost > 10000.0,
            "Should include 10% penalty"
        );
    }

    // --- Edge case tests ---

    #[test]
    fn test_no_spouse_skips_joint() {
        let mut option = make_option("joint_50", 2000.0);
        option.survivor_monthly_payment = Some(1000.0);
        let entries = sample_mortality_entries();
        let retiree_surv = compute_monthly_survival(&entries, 65, 360);

        // No spouse survival → should return 0
        let pv = annuity_pv(&option, &retiree_surv, None, 0.05, 0.0, 360, 65);
        assert_eq!(pv, 0.0);
    }

    #[test]
    fn test_no_lump_sum_no_mc() {
        let mut req = make_test_req_with_options(vec![make_option("single_life", 2000.0)]);
        req.retiree_gender = Some("male".into());
        req.lump_sum_amount = None;

        let result = run_pension_comparison(&req);
        assert!(result.lump_sum_monte_carlo.is_none());
        assert_eq!(result.options_analyzed.len(), 1);
        assert_eq!(result.options_analyzed[0].option_type, "single_life");
        assert!(result.options_analyzed[0].break_even_age.is_none()); // no lump sum
    }

    #[test]
    fn test_full_comparison_runs() {
        let mut opt_50 = make_option("joint_50", 2200.0);
        opt_50.survivor_monthly_payment = Some(1100.0);
        let mut opt_100 = make_option("joint_100", 1900.0);
        opt_100.survivor_monthly_payment = Some(1900.0);

        let req = PensionComparisonRequest {
            retiree_age: 62,
            retiree_gender: Some("male".into()),
            spouse_age: Some(60),
            spouse_gender: Some("female".into()),
            selected_option_id: None,
            lump_sum_amount: Some(400_000.0),
            annuity_options: vec![make_option("single_life", 2500.0), opt_50, opt_100],
            investment_return: ReturnAssumption {
                annual_mean: 0.07,
                annual_std_dev: 0.15,
            },
            discount_rate: 0.05,
            filing_status: "married_filing_jointly".into(),
            tax_year: 2026,
            income: IncomeBreakdown {
                wages: 50000.0,
                taxable_social_security: 15000.0,
                ..zero_income()
            },
            adjustments: Adjustments::default(),
            deductions: DeductionConfig {
                method: "standard".into(),
                itemized_amount: None,
                spouse_itemizes: None,
                state_local_income_or_sales_tax: None,
                real_property_tax: None,
                personal_property_tax: None,
                other_itemized_deductions: None,
            },
            tax_parameters: TaxParameters {
                ordinary_brackets: vec![
                    TaxBracket {
                        min: 0.0,
                        max: Some(23850.0),
                        rate: 0.10,
                    },
                    TaxBracket {
                        min: 23850.0,
                        max: Some(96950.0),
                        rate: 0.12,
                    },
                    TaxBracket {
                        min: 96950.0,
                        max: Some(206700.0),
                        rate: 0.22,
                    },
                    TaxBracket {
                        min: 206700.0,
                        max: None,
                        rate: 0.24,
                    },
                ],
                capital_gains_brackets: vec![
                    TaxBracket {
                        min: 0.0,
                        max: Some(96700.0),
                        rate: 0.0,
                    },
                    TaxBracket {
                        min: 96700.0,
                        max: Some(600050.0),
                        rate: 0.15,
                    },
                    TaxBracket {
                        min: 600050.0,
                        max: None,
                        rate: 0.20,
                    },
                ],
                standard_deduction: 30000.0,
                capital_loss_limit: 3000.0,
                niit: NiitParams {
                    rate: 0.038,
                    threshold: 250000.0,
                },
                payroll: PayrollParams {
                    social_security_rate: 0.062,
                    social_security_wage_base: 176100.0,
                    self_employment_tax_rate: 0.124,
                    medicare_rate: 0.0145,
                    self_employment_medicare_rate: 0.029,
                    additional_medicare_rate: 0.009,
                    additional_medicare_threshold: 250000.0,
                },
                salt: Some(default_salt()),
            },
            gross_social_security_benefit: None,
            ss_taxation_params: None,
            inflation_rate: 0.025,
            cola_rate: 0.0,
            retiree_life_expectancy_override: None,
            spouse_life_expectancy_override: None,
            rollover_to_ira: true,
            after_tax_contribution_basis: 0.0,
            separation_age: Some(62),
            taxable_account_assumptions: TaxableAccountAssumptions::default(),
            num_simulations: Some(200),
            seed: Some(42),
            mortality_table: make_mortality_table(),
            uniform_lifetime_table: None,
        };

        let result = run_pension_comparison(&req);

        // Basic structure checks
        assert_eq!(result.options_analyzed.len(), 3);
        assert!(result.lump_sum_monte_carlo.is_some());
        assert_eq!(result.comparison_summary.len(), 3);
        assert!(!result.cumulative_income_by_year.is_empty());
        assert!(result.survivor_analysis.is_some());

        // Single life should have highest monthly payment
        let single = &result.options_analyzed[0];
        assert_eq!(single.option_type, "single_life");
        assert_eq!(single.monthly_payment, 2500.0);
        assert!(single.present_value > 0.0);
        assert!(single.break_even_age.is_some());

        // Joint options should have survivor PV
        let joint_50 = &result.options_analyzed[1];
        assert!(joint_50.survivor_pv.is_some());

        let joint_100 = &result.options_analyzed[2];
        assert!(joint_100.survivor_pv.is_some());
        assert!(
            joint_100.survivor_pv.unwrap() > joint_50.survivor_pv.unwrap(),
            "100% survivor PV should exceed 50%"
        );

        // MC should show results
        let mc = result.lump_sum_monte_carlo.as_ref().unwrap();
        assert_eq!(mc.num_simulations, 200);
        assert!(mc.success_rate >= 0.0 && mc.success_rate <= 1.0);
    }

    #[test]
    fn test_exclusion_ratio_reduces_taxable() {
        let mut req = make_test_req_with_options(vec![make_option("single_life", 2000.0)]);
        req.income = zero_income();
        req.lump_sum_amount = Some(200_000.0);
        req.after_tax_contribution_basis = 50000.0;

        let tax = compute_tax_scenarios(&req);
        let cashout = tax.lump_sum_cash_out.unwrap();

        // $200k lump sum with $50k after-tax basis → only $150k taxable
        let mut req_no_basis = make_test_req_with_options(vec![make_option("single_life", 2000.0)]);
        req_no_basis.income = zero_income();
        req_no_basis.lump_sum_amount = Some(200_000.0);
        req_no_basis.after_tax_contribution_basis = 0.0;

        let tax_no_basis = compute_tax_scenarios(&req_no_basis);
        let cashout_no_basis = tax_no_basis.lump_sum_cash_out.unwrap();

        assert!(
            cashout.upfront_tax_cost < cashout_no_basis.upfront_tax_cost,
            "After-tax basis should reduce tax: {} vs {}",
            cashout.upfront_tax_cost,
            cashout_no_basis.upfront_tax_cost
        );
    }

    // --- Cumulative income tests ---

    #[test]
    fn test_cumulative_income_first_year_zero() {
        let mut req = make_test_req_with_options(vec![make_option("single_life", 1000.0)]);
        req.lump_sum_amount = None;
        req.investment_return = ReturnAssumption {
            annual_mean: 0.0,
            annual_std_dev: 0.0,
        };
        req.discount_rate = 0.0;
        req.inflation_rate = 0.0;
        req.cola_rate = 0.0;
        req.num_simulations = None;
        req.seed = None;

        let cum = build_cumulative_income(&req, &req.annuity_options[0], 20);
        assert_eq!(cum[0].annuity_cumulative_nominal, 0.0);
        assert_eq!(cum[0].age, 65);
        // After 1 year: $12,000
        assert!((cum[1].annuity_cumulative_nominal - 12000.0).abs() < 1.0);
    }

    // ================================================================
    // New tests for expanded pension features
    // ================================================================

    // --- Resolve option / alias tests ---

    #[test]
    fn test_resolve_joint_50_alias() {
        let opt = make_option("joint_50", 2000.0);
        let resolved = resolve_option(&opt);
        assert_eq!(resolved.option_type, "joint_survivor");
        assert_eq!(resolved.survivor_pct, Some(0.5));
        assert!((resolved.survivor_monthly_payment.unwrap() - 1000.0).abs() < 0.01);
    }

    #[test]
    fn test_resolve_joint_75_alias() {
        let opt = make_option("joint_75", 2000.0);
        let resolved = resolve_option(&opt);
        assert_eq!(resolved.option_type, "joint_survivor");
        assert_eq!(resolved.survivor_pct, Some(0.75));
        assert!((resolved.survivor_monthly_payment.unwrap() - 1500.0).abs() < 0.01);
    }

    #[test]
    fn test_resolve_period_certain_10_alias() {
        let opt = make_option("period_certain_10", 1500.0);
        let resolved = resolve_option(&opt);
        assert_eq!(resolved.option_type, "period_certain");
        assert_eq!(resolved.period_certain_months, Some(120));
    }

    #[test]
    fn test_resolve_period_certain_20_alias() {
        let opt = make_option("period_certain_20", 1500.0);
        let resolved = resolve_option(&opt);
        assert_eq!(resolved.option_type, "period_certain");
        assert_eq!(resolved.period_certain_months, Some(240));
    }

    // --- Joint survivor with custom survivor_pct ---

    #[test]
    fn test_joint_survivor_66_pct() {
        let mut opt = make_option("joint_survivor", 3000.0);
        opt.survivor_pct = Some(0.66);

        let resolved = resolve_option(&opt);
        assert_eq!(resolved.option_type, "joint_survivor");
        let expected_survivor = 3000.0 * 0.66;
        assert!((resolved.survivor_monthly_payment.unwrap() - expected_survivor).abs() < 0.01);
    }

    #[test]
    fn test_joint_survivor_explicit_payment_overrides_pct() {
        let mut opt = make_option("joint_survivor", 3000.0);
        opt.survivor_pct = Some(0.50);
        opt.survivor_monthly_payment = Some(2000.0);

        let resolved = resolve_option(&opt);
        // Explicit survivor_monthly_payment takes precedence
        assert_eq!(resolved.survivor_monthly_payment, Some(2000.0));
    }

    // --- Pop-up tests ---

    #[test]
    fn test_popup_increases_joint_pv() {
        let entries = sample_mortality_entries();
        let retiree_surv = compute_monthly_survival(&entries, 65, 360);
        let spouse_surv = compute_monthly_survival(&entries, 62, 360);

        let mut opt_no_popup = make_option("joint_survivor", 2000.0);
        opt_no_popup.survivor_pct = Some(0.50);

        let mut opt_popup = opt_no_popup.clone();
        opt_popup.popup_monthly_payment = Some(2500.0); // pops up to single-life level

        let pv_no_popup = annuity_pv(
            &opt_no_popup,
            &retiree_surv,
            Some(&spouse_surv),
            0.05,
            0.0,
            360,
            65,
        );
        let pv_popup = annuity_pv(
            &opt_popup,
            &retiree_surv,
            Some(&spouse_surv),
            0.05,
            0.0,
            360,
            65,
        );

        assert!(
            pv_popup > pv_no_popup,
            "Pop-up should increase PV: {} vs {}",
            pv_popup,
            pv_no_popup
        );
    }

    // --- Period certain custom months ---

    #[test]
    fn test_period_certain_15_years() {
        let mut opt = make_option("period_certain", 1000.0);
        opt.period_certain_months = Some(180); // 15 years

        let entries: Vec<MortalityEntry> = (0..=120)
            .map(|age| MortalityEntry {
                age,
                qx: if age >= 65 { 0.10 } else { 0.001 },
            })
            .collect();
        let surv = compute_monthly_survival(&entries, 65, 360);

        let pv = annuity_pv(&opt, &surv, None, 0.0, 0.0, 360, 65);

        // First 180 months guaranteed = at least $180,000
        assert!(
            pv >= 180000.0,
            "15yr certain should guarantee >= $180k, got {}",
            pv
        );

        // Compare with 10yr certain
        let mut opt_10 = make_option("period_certain", 1000.0);
        opt_10.period_certain_months = Some(120);
        let pv_10 = annuity_pv(&opt_10, &surv, None, 0.0, 0.0, 360, 65);

        assert!(
            pv > pv_10,
            "15yr certain should have higher PV than 10yr: {} vs {}",
            pv,
            pv_10
        );
    }

    // --- COLA config tests ---

    #[test]
    fn test_cola_start_delay() {
        let mut opt_immediate = make_option("single_life", 1000.0);
        opt_immediate.cola = Some(ColaConfig {
            rate: 0.03,
            start_delay_months: 0,
            cap_rate: None,
            end_age: None,
        });

        let mut opt_delayed = make_option("single_life", 1000.0);
        opt_delayed.cola = Some(ColaConfig {
            rate: 0.03,
            start_delay_months: 24,
            cap_rate: None,
            end_age: None,
        });

        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 65, 360);

        let pv_immediate = annuity_pv(&opt_immediate, &surv, None, 0.0, 0.0, 360, 65);
        let pv_delayed = annuity_pv(&opt_delayed, &surv, None, 0.0, 0.0, 360, 65);

        assert!(
            pv_immediate > pv_delayed,
            "Delayed COLA should have lower PV: {} vs {}",
            pv_immediate,
            pv_delayed
        );
    }

    #[test]
    fn test_cola_cap_rate() {
        let mut opt_uncapped = make_option("single_life", 1000.0);
        opt_uncapped.cola = Some(ColaConfig {
            rate: 0.05,
            start_delay_months: 0,
            cap_rate: None,
            end_age: None,
        });

        let mut opt_capped = make_option("single_life", 1000.0);
        opt_capped.cola = Some(ColaConfig {
            rate: 0.05,
            start_delay_months: 0,
            cap_rate: Some(0.03), // Effective rate = 3%
            end_age: None,
        });

        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 65, 360);

        let pv_uncapped = annuity_pv(&opt_uncapped, &surv, None, 0.0, 0.0, 360, 65);
        let pv_capped = annuity_pv(&opt_capped, &surv, None, 0.0, 0.0, 360, 65);

        assert!(
            pv_uncapped > pv_capped,
            "Capped COLA should have lower PV: {} vs {}",
            pv_uncapped,
            pv_capped
        );
    }

    #[test]
    fn test_cola_end_age() {
        let mut opt_no_end = make_option("single_life", 1000.0);
        opt_no_end.cola = Some(ColaConfig {
            rate: 0.03,
            start_delay_months: 0,
            cap_rate: None,
            end_age: None,
        });

        let mut opt_end_80 = make_option("single_life", 1000.0);
        opt_end_80.cola = Some(ColaConfig {
            rate: 0.03,
            start_delay_months: 0,
            cap_rate: None,
            end_age: Some(80), // COLA freezes at age 80
        });

        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 65, 360);

        let pv_no_end = annuity_pv(&opt_no_end, &surv, None, 0.0, 0.0, 360, 65);
        let pv_end_80 = annuity_pv(&opt_end_80, &surv, None, 0.0, 0.0, 360, 65);

        assert!(
            pv_no_end > pv_end_80,
            "COLA ending at 80 should have lower PV: {} vs {}",
            pv_no_end,
            pv_end_80
        );
    }

    #[test]
    fn test_per_option_cola_overrides_global() {
        let mut opt_with_cola = make_option("single_life", 1000.0);
        opt_with_cola.cola = Some(ColaConfig {
            rate: 0.05, // Higher per-option COLA
            start_delay_months: 0,
            cap_rate: None,
            end_age: None,
        });

        let opt_no_override = make_option("single_life", 1000.0);
        // This will use global_cola_rate = 0.02

        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 65, 240);

        let pv_per_option = annuity_pv(&opt_with_cola, &surv, None, 0.0, 0.02, 240, 65);
        let pv_global = annuity_pv(&opt_no_override, &surv, None, 0.0, 0.02, 240, 65);

        // Per-option 5% COLA should give higher PV than global 2%
        assert!(
            pv_per_option > pv_global,
            "Per-option 5% COLA should exceed global 2%: {} vs {}",
            pv_per_option,
            pv_global
        );
    }

    // --- Level income tests ---

    #[test]
    fn test_level_income_pv() {
        let mut opt_level = make_option("level_income", 3000.0);
        opt_level.level_income_age = Some(62);
        opt_level.level_income_reduced_payment = Some(1500.0);

        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 58, 480); // Start at 58

        let pv_level = annuity_pv(&opt_level, &surv, None, 0.0, 0.0, 480, 58);

        // At month m, age = 58 + m/12 (integer div). Age hits 62 at month 48.
        // Months 1-47 at $3000, months 48-480 at $1500.
        let expected = 47.0 * 3000.0 + 433.0 * 1500.0;
        assert!(
            (pv_level - expected).abs() < 1.0,
            "Level income PV should be ~{}, got {}",
            expected,
            pv_level
        );
    }

    #[test]
    fn test_level_income_higher_early_payments() {
        // Level income should have higher PV than if payments were always at the reduced level
        let mut opt_level = make_option("level_income", 3000.0);
        opt_level.level_income_age = Some(62);
        opt_level.level_income_reduced_payment = Some(1500.0);

        let opt_flat = make_option("single_life", 1500.0);

        let entries = zero_qx_table();
        let surv = compute_monthly_survival(&entries, 58, 480);

        let pv_level = annuity_pv(&opt_level, &surv, None, 0.05, 0.0, 480, 58);
        let pv_flat = annuity_pv(&opt_flat, &surv, None, 0.05, 0.0, 480, 58);

        assert!(
            pv_level > pv_flat,
            "Level income should have higher PV: {} vs {}",
            pv_level,
            pv_flat
        );
    }

    // --- Label passthrough ---

    #[test]
    fn test_label_passthrough() {
        let mut opt = make_option("single_life", 2000.0);
        opt.label = Some("My Custom Label".into());

        let mut req = make_test_req_with_options(vec![opt]);
        req.lump_sum_amount = None;

        let result = run_pension_comparison(&req);
        assert_eq!(
            result.options_analyzed[0].label,
            Some("My Custom Label".into())
        );
        assert_eq!(
            result.comparison_summary[0].label,
            Some("My Custom Label".into())
        );
    }

    #[test]
    fn test_validation_rejects_basis_above_lump_sum() {
        let mut req = make_test_req_with_options(vec![make_option("single_life", 2000.0)]);
        req.after_tax_contribution_basis = 350_000.0;

        let errors = crate::validation::validate_pension_comparison_request(&req);
        assert!(
            errors
                .iter()
                .any(|error| error.contains("after_tax_contribution_basis must not exceed")),
            "expected basis validation error, got {:?}",
            errors
        );
    }

    #[test]
    fn test_annuity_basis_recovery_reduces_first_year_tax() {
        let option = make_option("single_life", 2000.0);

        let mut req_with_basis = make_test_req_with_options(vec![option.clone()]);
        req_with_basis.income = zero_income();
        req_with_basis.after_tax_contribution_basis = 24_000.0;

        let baseline_with_basis = baseline_tax_snapshot(&req_with_basis);
        let with_basis = first_year_annuity_tax(&req_with_basis, &option, &baseline_with_basis);

        let mut req_no_basis = make_test_req_with_options(vec![option.clone()]);
        req_no_basis.income = zero_income();
        let baseline_no_basis = baseline_tax_snapshot(&req_no_basis);
        let no_basis = first_year_annuity_tax(&req_no_basis, &option, &baseline_no_basis);

        assert!(
            with_basis.snapshot.tax_delta < no_basis.snapshot.tax_delta,
            "expected basis recovery to reduce tax: {} vs {}",
            with_basis.snapshot.tax_delta,
            no_basis.snapshot.tax_delta
        );
    }

    #[test]
    fn test_selected_option_drives_mc_tax_and_cumulative_outputs() {
        let mut high = make_option("single_life", 3000.0);
        high.id = Some("high".into());

        let mut selected = make_option("single_life", 1800.0);
        selected.id = Some("selected".into());

        let mut req = make_test_req_with_options(vec![high, selected]);
        req.selected_option_id = Some("selected".into());

        let result = run_pension_comparison(&req);
        let selected_row = result
            .options_analyzed
            .iter()
            .find(|option| option.id == "selected")
            .unwrap();
        let mc = result.lump_sum_monte_carlo.as_ref().unwrap();

        assert_eq!(result.selected_option_id_used, "selected");
        assert_eq!(mc.monthly_withdrawal, 1800.0);
        assert!(
            (result.cumulative_income_by_year[1].annuity_cumulative_nominal - 21_600.0).abs() < 1.0
        );
        assert_eq!(
            result
                .tax_impact
                .annuity_scenario
                .annual_tax_on_pension_income,
            selected_row.annual_tax_estimate
        );
    }

    #[test]
    fn test_period_certain_survivor_value_survives_guarantee_window() {
        let mut option = make_option("period_certain", 1000.0);
        option.id = Some("period".into());
        option.period_certain_months = Some(120);

        let mut req = make_test_req_with_options(vec![option.clone()]);
        req.spouse_age = Some(60);
        req.retiree_life_expectancy_override = Some(66);
        req.spouse_life_expectancy_override = Some(90);

        let retiree_surv = compute_monthly_survival(&zero_qx_table(), req.retiree_age, 360);
        let spouse_surv = compute_monthly_survival(&zero_qx_table(), 60, 360);
        let survivor = compute_survivor_analysis(
            &req,
            &option,
            "period",
            &retiree_surv,
            &spouse_surv,
            90,
            None,
        );
        let survivor_option = &survivor.options[0];

        assert_eq!(survivor_option.survivor_monthly_income, 1000.0);
        assert!(survivor_option.survivor_income_pv > 80_000.0);
    }

    #[test]
    fn test_cashout_projection_uses_net_investable_balance() {
        let option = make_option("single_life", 1000.0);
        let mut req = make_test_req_with_options(vec![option.clone()]);
        req.retiree_age = 50;
        req.income = zero_income();
        req.rollover_to_ira = false;
        req.lump_sum_amount = Some(100_000.0);

        let result = run_pension_comparison(&req);
        let cash_out = result
            .after_tax_projection
            .lump_sum_cash_out
            .as_ref()
            .unwrap();
        let upfront_tax = result
            .tax_impact
            .lump_sum_cash_out
            .as_ref()
            .unwrap()
            .upfront_tax_cost;
        let mc = result.lump_sum_monte_carlo.as_ref().unwrap();
        let expected_net = round2(100_000.0 - upfront_tax);

        assert_eq!(cash_out.net_investable_amount.unwrap(), expected_net);
        assert_eq!(mc.starting_balance, expected_net);
    }
}
