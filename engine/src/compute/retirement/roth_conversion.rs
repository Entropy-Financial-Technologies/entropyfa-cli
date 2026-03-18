use crate::compute::tax::federal::run_federal_tax;
use crate::models::roth_conversion::*;
use crate::models::tax_request::{FederalTaxRequest, IncomeBreakdown, TaxBracket};

// ---------------------------------------------------------------------------
// Single-year analysis
// ---------------------------------------------------------------------------

pub fn analyze_roth_conversion(req: &RothConversionRequest) -> RothConversionResponse {
    let total_ira = req
        .total_traditional_ira_value
        .unwrap_or(req.traditional_ira_balance);

    // Pro-rata rule (Form 8606)
    let basis_ratio = if total_ira > 0.0 {
        req.nondeductible_basis / total_ira
    } else {
        0.0
    }
    .clamp(0.0, 1.0);

    // Baseline SS taxability
    let baseline_taxable_ss = compute_ss_taxability(req, 0.0);

    // Build baseline tax request and run
    let baseline_req = build_tax_request(req, 0.0, baseline_taxable_ss);
    let baseline_result = run_federal_tax(&baseline_req);

    // Determine conversion amount
    let conversion_amount = match req.conversion_amount {
        Some(amt) => amt.min(req.traditional_ira_balance).max(0.0),
        None => {
            // Auto-fill: compute room in current bracket
            auto_fill_bracket(
                req,
                &baseline_result.marginal_ordinary_rate,
                baseline_taxable_ss,
            )
        }
    };

    let taxable_conversion = conversion_amount * (1.0 - basis_ratio);
    let nontaxable_conversion = conversion_amount - taxable_conversion;

    // With-conversion SS taxability
    let conversion_taxable_ss = compute_ss_taxability(req, taxable_conversion);

    // Build with-conversion tax request and run
    let conversion_req = build_tax_request(req, taxable_conversion, conversion_taxable_ss);
    let conversion_result = run_federal_tax(&conversion_req);

    // Deltas
    let conversion_tax_cost =
        round2(conversion_result.total_income_tax - baseline_result.total_income_tax);
    let effective_rate_on_conversion = if conversion_amount > 0.0 {
        round4(conversion_tax_cost / conversion_amount)
    } else {
        0.0
    };

    // Marginal rate on conversion dollars (ordinary rate at with-conversion taxable income)
    let marginal_rate_on_conversion = conversion_result.marginal_ordinary_rate;

    // Pro-rata detail
    let pro_rata = ProRataDetail {
        total_traditional_ira_value: round2(total_ira),
        nondeductible_basis: round2(req.nondeductible_basis),
        basis_ratio: round4(basis_ratio),
        taxable_conversion: round2(taxable_conversion),
        nontaxable_conversion: round2(nontaxable_conversion),
    };

    // Bracket space
    let bracket_space = compute_bracket_space(
        &req.tax_parameters.ordinary_brackets,
        baseline_result.taxable_income - baseline_result.preferential_income,
        taxable_conversion,
    );

    // IRMAA impact
    let irmaa_impact = req
        .irmaa_brackets
        .as_ref()
        .map(|brackets| compute_irmaa_impact(brackets, baseline_result.agi, conversion_result.agi));

    // NIIT increase
    let niit_increase = round2(conversion_result.niit - baseline_result.niit);

    // Additional SS taxable
    let additional_ss_taxable = round2(conversion_taxable_ss - baseline_taxable_ss);

    let baseline_summary = make_tax_summary(&baseline_result, baseline_taxable_ss);
    let conversion_summary = make_tax_summary(&conversion_result, conversion_taxable_ss);

    RothConversionResponse {
        baseline: baseline_summary,
        with_conversion: conversion_summary,
        conversion_amount: round2(conversion_amount),
        conversion_tax_cost,
        effective_rate_on_conversion,
        marginal_rate_on_conversion,
        pro_rata,
        bracket_space,
        irmaa_impact,
        niit_increase,
        additional_ss_taxable,
    }
}

// ---------------------------------------------------------------------------
// Multi-year strategy
// ---------------------------------------------------------------------------

pub fn compute_roth_conversion_strategy(
    req: &RothConversionStrategyRequest,
) -> Result<RothConversionStrategyResponse, String> {
    let birth_year = parse_birth_year(&req.owner_birth_date)?;
    let start_year = req.tax_year;
    let growth = req.annual_growth_rate;
    let rmd_start_age = req.rmd_start_age.unwrap_or(73);

    let mut trad_convert = req.traditional_ira_balance;
    let mut roth_convert = req.roth_ira_balance;
    let mut trad_no_convert = req.traditional_ira_balance;
    let mut basis_remaining = req.nondeductible_basis;

    let mut annual_detail = Vec::new();
    let mut total_conversion_tax = 0.0;
    let mut total_rmd_tax_convert = 0.0;
    let mut total_rmd_tax_no_convert = 0.0;
    let mut total_rmds_convert = 0.0;
    let mut total_rmds_no_convert = 0.0;
    let mut cumulative_conversion_tax = 0.0;
    let mut cumulative_rmd_savings = 0.0;
    let mut breakeven_year: Option<u32> = None;

    for i in 0..req.projection_years {
        let year = start_year + i;
        let age = year - birth_year;

        // Apply income events to get this year's income
        let year_income = apply_income_events(&req.income, &req.income_events, year);

        // Determine conversion amount based on strategy
        let conversion_amount = match req.strategy.as_str() {
            "fill_bracket" => {
                let target_rate = req.target_bracket_rate.unwrap_or(0.22);
                compute_fill_bracket_amount(
                    req,
                    &year_income,
                    target_rate,
                    trad_convert,
                    basis_remaining,
                )
            }
            "fixed_amount" => {
                let fixed = req.fixed_annual_conversion.unwrap_or(0.0);
                fixed.min(trad_convert).max(0.0)
            }
            _ => 0.0,
        };

        // Pro-rata for this year's conversion
        let total_ira = req
            .total_traditional_ira_value
            .map(|v| v.max(trad_convert))
            .unwrap_or(trad_convert);
        let basis_ratio = if total_ira > 0.0 && basis_remaining > 0.0 {
            (basis_remaining / total_ira).min(1.0)
        } else {
            0.0
        };
        let taxable_conversion = conversion_amount * (1.0 - basis_ratio);
        let basis_used = conversion_amount * basis_ratio;

        // Single-year tax analysis for this conversion
        let (conv_tax_cost, conv_eff_rate) = if conversion_amount > 0.0 {
            let single_req = RothConversionRequest {
                filing_status: req.filing_status.clone(),
                tax_year: year,
                income: year_income.clone(),
                adjustments: req.adjustments.clone(),
                deductions: req.deductions.clone(),
                tax_parameters: req.tax_parameters.clone(),
                conversion_amount: Some(conversion_amount),
                traditional_ira_balance: trad_convert,
                nondeductible_basis: basis_remaining,
                total_traditional_ira_value: Some(total_ira),
                irmaa_brackets: req.irmaa_brackets.clone(),
                gross_social_security_benefit: req.gross_social_security_benefit,
                ss_taxation_params: req.ss_taxation_params.clone(),
            };
            let result = analyze_roth_conversion(&single_req);
            (
                result.conversion_tax_cost,
                result.effective_rate_on_conversion,
            )
        } else {
            (0.0, 0.0)
        };

        // Update balances for convert scenario
        trad_convert = (trad_convert - conversion_amount).max(0.0);
        roth_convert += conversion_amount;
        basis_remaining = (basis_remaining - basis_used).max(0.0);

        // RMDs for both scenarios
        let (rmd_convert, rmd_no_convert) = if age >= rmd_start_age {
            let dp_convert = lookup_distribution_period(req, age);
            let dp_no_convert = dp_convert; // same table
            let rmd_c = if dp_convert > 0.0 {
                trad_convert / dp_convert
            } else {
                0.0
            };
            let rmd_nc = if dp_no_convert > 0.0 {
                trad_no_convert / dp_no_convert
            } else {
                0.0
            };
            (rmd_c, rmd_nc)
        } else {
            (0.0, 0.0)
        };

        // Estimate RMD tax (use marginal rate from the conversion analysis or a simple estimate)
        let rmd_tax_rate = estimate_marginal_rate_on_rmd(req, &year_income);
        let rmd_tax_c = round2(rmd_convert * rmd_tax_rate);
        let rmd_tax_nc = round2(rmd_no_convert * rmd_tax_rate);

        // Deduct RMDs from trad balances
        trad_convert = (trad_convert - rmd_convert).max(0.0);
        trad_no_convert = (trad_no_convert - rmd_no_convert).max(0.0);

        // Grow balances
        trad_convert *= 1.0 + growth;
        roth_convert *= 1.0 + growth;
        trad_no_convert *= 1.0 + growth;

        total_conversion_tax += conv_tax_cost;
        total_rmd_tax_convert += rmd_tax_c;
        total_rmd_tax_no_convert += rmd_tax_nc;
        total_rmds_convert += rmd_convert;
        total_rmds_no_convert += rmd_no_convert;

        // Breakeven tracking
        cumulative_conversion_tax += conv_tax_cost;
        cumulative_rmd_savings += rmd_tax_nc - rmd_tax_c;
        if breakeven_year.is_none()
            && cumulative_rmd_savings >= cumulative_conversion_tax
            && cumulative_conversion_tax > 0.0
        {
            breakeven_year = Some(year);
        }

        annual_detail.push(StrategyYearDetail {
            year,
            age,
            conversion_amount: round2(conversion_amount),
            taxable_conversion: round2(taxable_conversion),
            conversion_tax_cost: round2(conv_tax_cost),
            effective_rate_on_conversion: round4(conv_eff_rate),
            trad_ira_balance_convert: round2(trad_convert),
            roth_ira_balance_convert: round2(roth_convert),
            trad_ira_balance_no_convert: round2(trad_no_convert),
            rmd_convert: round2(rmd_convert),
            rmd_no_convert: round2(rmd_no_convert),
            rmd_tax_convert: rmd_tax_c,
            rmd_tax_no_convert: rmd_tax_nc,
        });
    }

    let total_rmd_tax_savings = round2(total_rmd_tax_no_convert - total_rmd_tax_convert);
    let net_tax_savings = round2(total_rmd_tax_savings - total_conversion_tax);

    // After-tax totals: all balances + Roth (already tax-free)
    // For trad, estimate after-tax as balance * (1 - estimated_rate)
    let est_rate = estimate_marginal_rate_on_rmd(req, &req.income);
    let convert_after_tax = round2(trad_convert * (1.0 - est_rate) + roth_convert);
    let no_convert_after_tax = round2(
        trad_no_convert * (1.0 - est_rate)
            + req.roth_ira_balance * (1.0 + growth).powi(req.projection_years as i32),
    );

    Ok(RothConversionStrategyResponse {
        strategy: req.strategy.clone(),
        projection_years: req.projection_years,
        annual_growth_rate: growth,
        annual_detail,
        convert_scenario: ScenarioSummary {
            final_trad_ira_balance: round2(trad_convert),
            final_roth_ira_balance: round2(roth_convert),
            total_taxes_paid: round2(total_conversion_tax + total_rmd_tax_convert),
            total_rmds: round2(total_rmds_convert),
        },
        do_nothing_scenario: ScenarioSummary {
            final_trad_ira_balance: round2(trad_no_convert),
            final_roth_ira_balance: round2(
                req.roth_ira_balance * (1.0 + growth).powi(req.projection_years as i32),
            ),
            total_taxes_paid: round2(total_rmd_tax_no_convert),
            total_rmds: round2(total_rmds_no_convert),
        },
        comparison: StrategyComparison {
            total_conversion_tax: round2(total_conversion_tax),
            total_rmd_tax_savings,
            net_tax_savings,
            breakeven_year,
            convert_after_tax_total: convert_after_tax,
            no_convert_after_tax_total: no_convert_after_tax,
        },
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn build_tax_request(
    req: &RothConversionRequest,
    taxable_conversion: f64,
    taxable_ss: f64,
) -> FederalTaxRequest {
    let mut income = req.income.clone();
    income.taxable_ira_distributions += taxable_conversion;
    income.taxable_social_security = taxable_ss;

    FederalTaxRequest {
        filing_status: req.filing_status.clone(),
        tax_year: req.tax_year,
        income,
        adjustments: req.adjustments.clone(),
        deductions: req.deductions.clone(),
        tax_parameters: req.tax_parameters.clone(),
    }
}

/// Compute taxable Social Security benefits using IRS Pub 915 provisional income formula.
/// If no gross_social_security_benefit is provided, returns the caller-supplied value.
fn compute_ss_taxability(req: &RothConversionRequest, additional_income: f64) -> f64 {
    let gross_ss = match req.gross_social_security_benefit {
        Some(ss) if ss > 0.0 => ss,
        _ => return req.income.taxable_social_security,
    };
    let params = match &req.ss_taxation_params {
        Some(p) => p,
        None => return req.income.taxable_social_security,
    };

    // AGI excluding taxable SS + additional conversion income
    let agi_ex_ss = compute_agi_excluding_ss(&req.income, &req.adjustments) + additional_income;
    compute_taxable_ss(gross_ss, agi_ex_ss, req.income.tax_exempt_interest, params)
}

/// IRS Pub 915 worksheet: computes taxable portion of SS benefits.
pub fn compute_taxable_ss(
    gross_ss: f64,
    agi_excluding_ss: f64,
    tax_exempt_interest: f64,
    params: &SsTaxationParams,
) -> f64 {
    if gross_ss <= 0.0 {
        return 0.0;
    }

    let provisional = agi_excluding_ss + tax_exempt_interest + 0.5 * gross_ss;

    if provisional <= params.base_threshold {
        0.0
    } else if provisional <= params.upper_threshold {
        let taxable = params.max_taxable_pct_below_upper * gross_ss;
        let amount = 0.5 * (provisional - params.base_threshold);
        taxable.min(amount).max(0.0)
    } else {
        let max_taxable = params.max_taxable_pct_above_upper * gross_ss;
        let tier1 = 0.5 * (params.upper_threshold - params.base_threshold);
        let tier2 = params.max_taxable_pct_above_upper * (provisional - params.upper_threshold);
        max_taxable.min(tier1 + tier2).max(0.0)
    }
}

/// Compute AGI excluding Social Security (for provisional income calculation).
fn compute_agi_excluding_ss(
    income: &IncomeBreakdown,
    adjustments: &crate::models::tax_request::Adjustments,
) -> f64 {
    let gross_ex_ss = income.wages
        + income.self_employment_income
        + income.taxable_interest
        + income.ordinary_dividends
        + income.qualified_dividends
        + income.short_term_capital_gains
        + income.long_term_capital_gains
        + income.taxable_ira_distributions
        + income.taxable_pensions
        + income.other_income;
    let adj = adjustments.hsa_deduction
        + adjustments.ira_deduction
        + adjustments.student_loan_interest
        + adjustments.other_adjustments;
    gross_ex_ss - adj
}

fn auto_fill_bracket(
    req: &RothConversionRequest,
    current_marginal: &f64,
    baseline_taxable_ss: f64,
) -> f64 {
    let baseline_req = build_tax_request(req, 0.0, baseline_taxable_ss);
    let baseline = run_federal_tax(&baseline_req);
    let ordinary_taxable = baseline.taxable_income - baseline.preferential_income;

    // Find the top of the current bracket
    for bracket in &req.tax_parameters.ordinary_brackets {
        if (bracket.rate - current_marginal).abs() < 1e-10 {
            let bracket_max = bracket.max.unwrap_or(f64::MAX);
            let room = (bracket_max - ordinary_taxable).max(0.0);
            return room.min(req.traditional_ira_balance);
        }
    }

    0.0
}

fn compute_bracket_space(
    brackets: &[TaxBracket],
    ordinary_taxable: f64,
    taxable_conversion: f64,
) -> Vec<BracketSpaceDetail> {
    let mut details = Vec::new();
    let mut cumulative_conversion = 0.0;
    let mut cumulative_tax = 0.0;
    let mut conversion_remaining = taxable_conversion;

    for bracket in brackets {
        let bracket_max = bracket.max.unwrap_or(f64::MAX);
        let bracket_width = bracket_max - bracket.min;
        let already_used = (ordinary_taxable - bracket.min).max(0.0).min(bracket_width);
        let room = bracket_width - already_used;

        let conversion_in_bracket = conversion_remaining.min(room).max(0.0);
        let tax_cost = conversion_in_bracket * bracket.rate;
        conversion_remaining -= conversion_in_bracket;
        cumulative_conversion += conversion_in_bracket;
        cumulative_tax += tax_cost;

        details.push(BracketSpaceDetail {
            rate: bracket.rate,
            bracket_min: bracket.min,
            bracket_max: bracket.max,
            already_used: round2(already_used),
            room_available: round2(room),
            conversion_in_bracket: round2(conversion_in_bracket),
            tax_cost_in_bracket: round2(tax_cost),
            cumulative_conversion: round2(cumulative_conversion),
            cumulative_tax_cost: round2(cumulative_tax),
        });
    }

    details
}

fn compute_irmaa_impact(
    brackets: &IrmaaBrackets,
    baseline_agi: f64,
    conversion_agi: f64,
) -> IrmaaImpact {
    let (base_idx, base_surcharge) =
        find_irmaa_tier(&brackets.tiers, baseline_agi, brackets.persons);
    let (conv_idx, conv_surcharge) =
        find_irmaa_tier(&brackets.tiers, conversion_agi, brackets.persons);

    IrmaaImpact {
        baseline_magi: round2(baseline_agi),
        conversion_magi: round2(conversion_agi),
        baseline_tier_index: base_idx,
        conversion_tier_index: conv_idx,
        baseline_annual_surcharge: round2(base_surcharge),
        conversion_annual_surcharge: round2(conv_surcharge),
        incremental_annual_surcharge: round2(conv_surcharge - base_surcharge),
        persons: brackets.persons,
    }
}

fn find_irmaa_tier(tiers: &[IrmaaTier], magi: f64, persons: u32) -> (usize, f64) {
    let mut result_idx = 0;
    let mut surcharge = 0.0;

    for (i, tier) in tiers.iter().enumerate() {
        let ceiling = tier.magi_ceiling.unwrap_or(f64::MAX);
        if magi >= tier.magi_floor && magi < ceiling {
            result_idx = i;
            surcharge = (tier.surcharge_part_b + tier.surcharge_part_d) * 12.0 * persons as f64;
            break;
        }
        // If above all tiers, use the last one
        if i == tiers.len() - 1 && magi >= tier.magi_floor {
            result_idx = i;
            surcharge = (tier.surcharge_part_b + tier.surcharge_part_d) * 12.0 * persons as f64;
        }
    }

    (result_idx, surcharge)
}

fn make_tax_summary(
    res: &crate::models::tax_response::FederalTaxResponse,
    taxable_ss: f64,
) -> TaxSummary {
    TaxSummary {
        agi: res.agi,
        taxable_income: res.taxable_income,
        ordinary_income_tax: res.ordinary_income_tax,
        capital_gains_tax: res.capital_gains_tax,
        niit: res.niit,
        total_income_tax: res.total_income_tax,
        total_tax: res.total_tax,
        effective_rate: res.effective_rate,
        marginal_ordinary_rate: res.marginal_ordinary_rate,
        taxable_social_security: round2(taxable_ss),
        ordinary_bracket_detail: res.ordinary_bracket_detail.clone(),
    }
}

fn parse_birth_year(date_str: &str) -> Result<u32, String> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.is_empty() {
        return Err("owner_birth_date is required".to_string());
    }
    parts[0]
        .parse::<u32>()
        .map_err(|_| format!("Invalid owner_birth_date: {}", date_str))
}

fn apply_income_events(
    base_income: &IncomeBreakdown,
    events: &[IncomeEvent],
    year: u32,
) -> IncomeBreakdown {
    let mut income = base_income.clone();
    for event in events {
        let end = event.end_year.unwrap_or(u32::MAX);
        if year >= event.start_year && year <= end {
            match event.income_field.as_str() {
                "wages" => income.wages = event.amount,
                "self_employment_income" => income.self_employment_income = event.amount,
                "taxable_interest" => income.taxable_interest = event.amount,
                "tax_exempt_interest" => income.tax_exempt_interest = event.amount,
                "ordinary_dividends" => income.ordinary_dividends = event.amount,
                "qualified_dividends" => income.qualified_dividends = event.amount,
                "short_term_capital_gains" => income.short_term_capital_gains = event.amount,
                "long_term_capital_gains" => income.long_term_capital_gains = event.amount,
                "taxable_ira_distributions" => income.taxable_ira_distributions = event.amount,
                "taxable_pensions" => income.taxable_pensions = event.amount,
                "taxable_social_security" => income.taxable_social_security = event.amount,
                "other_income" => income.other_income = event.amount,
                _ => {}
            }
        }
    }
    income
}

fn compute_fill_bracket_amount(
    req: &RothConversionStrategyRequest,
    income: &IncomeBreakdown,
    target_rate: f64,
    trad_balance: f64,
    _basis: f64,
) -> f64 {
    // Build a baseline tax request to find current ordinary taxable
    let baseline_req = FederalTaxRequest {
        filing_status: req.filing_status.clone(),
        tax_year: req.tax_year,
        income: income.clone(),
        adjustments: req.adjustments.clone(),
        deductions: req.deductions.clone(),
        tax_parameters: req.tax_parameters.clone(),
    };
    let baseline = run_federal_tax(&baseline_req);
    let ordinary_taxable = baseline.taxable_income - baseline.preferential_income;

    // Find top of target bracket and compute room from current ordinary_taxable
    let mut target_bracket_top = 0.0_f64;
    for bracket in &req.tax_parameters.ordinary_brackets {
        if bracket.rate <= target_rate {
            target_bracket_top = bracket.max.unwrap_or(f64::MAX);
        }
    }
    let fill_amount = (target_bracket_top - ordinary_taxable).max(0.0);
    fill_amount.min(trad_balance)
}

fn lookup_distribution_period(req: &RothConversionStrategyRequest, age: u32) -> f64 {
    if let Some(ref table) = req.uniform_lifetime_table {
        for entry in table {
            if entry.age == age {
                return entry.distribution_period;
            }
        }
        // If age exceeds table, use last entry
        if let Some(last) = table.last() {
            if age > last.age {
                return last.distribution_period;
            }
        }
    }
    // Default: approximate uniform lifetime table
    // This is a rough approximation for common ages
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
        _ => 27.4, // Below RMD age, use a safe default
    }
}

fn estimate_marginal_rate_on_rmd(
    req: &RothConversionStrategyRequest,
    income: &IncomeBreakdown,
) -> f64 {
    let baseline_req = FederalTaxRequest {
        filing_status: req.filing_status.clone(),
        tax_year: req.tax_year,
        income: income.clone(),
        adjustments: req.adjustments.clone(),
        deductions: req.deductions.clone(),
        tax_parameters: req.tax_parameters.clone(),
    };
    let result = run_federal_tax(&baseline_req);
    result.marginal_ordinary_rate
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn round4(v: f64) -> f64 {
    (v * 10000.0).round() / 10000.0
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::tax_request::*;

    fn single_2025_brackets() -> Vec<TaxBracket> {
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

    fn mfj_2025_brackets() -> Vec<TaxBracket> {
        vec![
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
                max: Some(394600.0),
                rate: 0.24,
            },
            TaxBracket {
                min: 394600.0,
                max: Some(501050.0),
                rate: 0.32,
            },
            TaxBracket {
                min: 501050.0,
                max: Some(751600.0),
                rate: 0.35,
            },
            TaxBracket {
                min: 751600.0,
                max: None,
                rate: 0.37,
            },
        ]
    }

    fn mfj_cg_brackets() -> Vec<TaxBracket> {
        vec![
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

    fn make_single_request(
        income: IncomeBreakdown,
        conversion_amount: Option<f64>,
        traditional_ira_balance: f64,
    ) -> RothConversionRequest {
        RothConversionRequest {
            filing_status: "single".to_string(),
            tax_year: 2025,
            income,
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
                ordinary_brackets: single_2025_brackets(),
                capital_gains_brackets: single_cg_brackets(),
                standard_deduction: 15000.0,
                capital_loss_limit: 3000.0,
                niit: NiitParams {
                    rate: 0.038,
                    threshold: 200000.0,
                },
                payroll: default_payroll(),
                salt: Some(default_salt()),
            },
            conversion_amount,
            traditional_ira_balance,
            nondeductible_basis: 0.0,
            total_traditional_ira_value: None,
            irmaa_brackets: None,
            gross_social_security_benefit: None,
            ss_taxation_params: None,
        }
    }

    // Test 1: Zero conversion → tax cost = 0
    #[test]
    fn test_zero_conversion() {
        let income = IncomeBreakdown {
            wages: 75000.0,
            ..Default::default()
        };
        let req = make_single_request(income, Some(0.0), 500000.0);
        let res = analyze_roth_conversion(&req);

        assert_eq!(res.conversion_amount, 0.0);
        assert_eq!(res.conversion_tax_cost, 0.0);
        assert_eq!(res.effective_rate_on_conversion, 0.0);
        assert_eq!(res.baseline.agi, res.with_conversion.agi);
        assert_eq!(
            res.baseline.total_income_tax,
            res.with_conversion.total_income_tax
        );
    }

    // Test 2: Bracket fill — $50K wages single, verify room in 22% bracket
    #[test]
    fn test_bracket_fill_22_pct() {
        let income = IncomeBreakdown {
            wages: 50000.0,
            ..Default::default()
        };
        let req = make_single_request(income, Some(20000.0), 500000.0);
        let res = analyze_roth_conversion(&req);

        // Taxable income baseline: 50000 - 15000 = 35000 (in 12% bracket)
        // With $20K conversion: 70000 - 15000 = 55000 (pushes into 22% bracket)
        assert_eq!(res.baseline.taxable_income, 35000.0);
        assert_eq!(res.with_conversion.taxable_income, 55000.0);
        assert!(res.conversion_tax_cost > 0.0);

        // Check bracket space shows room in 22% bracket
        let bracket_22 = res
            .bracket_space
            .iter()
            .find(|b| (b.rate - 0.22).abs() < 1e-10);
        assert!(bracket_22.is_some());
        let b22 = bracket_22.unwrap();
        assert!(b22.room_available > 0.0);
    }

    // Test 3: Conversion pushes to next bracket
    #[test]
    fn test_conversion_pushes_bracket() {
        // $90K wages → taxable = $75K → in 22% bracket
        // + $50K conversion → taxable = $125K → pushes into 24% bracket
        let income = IncomeBreakdown {
            wages: 90000.0,
            ..Default::default()
        };
        let req = make_single_request(income, Some(50000.0), 500000.0);
        let res = analyze_roth_conversion(&req);

        assert_eq!(res.baseline.marginal_ordinary_rate, 0.22);
        assert_eq!(res.with_conversion.marginal_ordinary_rate, 0.24);
        assert!(res.conversion_tax_cost > 0.0);
    }

    // Test 4: NIIT interaction — IRA distributions NOT NII, but raise MAGI
    #[test]
    fn test_niit_interaction() {
        let income = IncomeBreakdown {
            wages: 190000.0,
            taxable_interest: 5000.0,
            ..Default::default()
        };
        let req = make_single_request(income, Some(20000.0), 500000.0);
        let res = analyze_roth_conversion(&req);

        // Baseline AGI = 195000, below $200K NIIT threshold → NIIT = 0
        // With conversion AGI = 215000, above threshold
        // NII = 5000 (interest only — IRA distributions excluded per §408)
        // NIIT = 0.038 * min(5000, 215000 - 200000) = 0.038 * 5000 = 190
        assert_eq!(res.baseline.niit, 0.0);
        assert_eq!(res.with_conversion.niit, 190.0);
        assert_eq!(res.niit_increase, 190.0);
    }

    // Test 5: IRMAA cliff
    #[test]
    fn test_irmaa_cliff() {
        let income = IncomeBreakdown {
            wages: 108000.0,
            ..Default::default()
        };
        let mut req = make_single_request(income, Some(2000.0), 500000.0);
        req.irmaa_brackets = Some(IrmaaBrackets {
            base_premium_part_b: 202.90,
            tiers: vec![
                IrmaaTier {
                    magi_floor: 0.0,
                    magi_ceiling: Some(109000.0),
                    surcharge_part_b: 0.0,
                    surcharge_part_d: 0.0,
                },
                IrmaaTier {
                    magi_floor: 109000.0,
                    magi_ceiling: Some(137000.0),
                    surcharge_part_b: 81.20,
                    surcharge_part_d: 14.50,
                },
                IrmaaTier {
                    magi_floor: 137000.0,
                    magi_ceiling: Some(171000.0),
                    surcharge_part_b: 202.90,
                    surcharge_part_d: 37.50,
                },
            ],
            persons: 1,
        });

        let res = analyze_roth_conversion(&req);
        let irmaa = res.irmaa_impact.unwrap();

        // Baseline AGI = 108000, tier 0 (no surcharge)
        // With conversion AGI = 110000, tier 1 ($81.20 + $14.50 = $95.70/mo)
        assert_eq!(irmaa.baseline_tier_index, 0);
        assert_eq!(irmaa.conversion_tier_index, 1);
        assert_eq!(irmaa.baseline_annual_surcharge, 0.0);
        // 95.70 * 12 = 1148.40
        assert_eq!(irmaa.incremental_annual_surcharge, 1148.4);
    }

    // Test 6: Auto bracket-fill with no explicit amount
    #[test]
    fn test_auto_bracket_fill() {
        let income = IncomeBreakdown {
            wages: 50000.0,
            ..Default::default()
        };
        let req = make_single_request(income, None, 500000.0);
        let res = analyze_roth_conversion(&req);

        // Taxable = 35000, in 12% bracket (11925..48475)
        // Room in 12% bracket: 48475 - 35000 = 13475
        assert_eq!(res.conversion_amount, 13475.0);
        assert!(res.conversion_tax_cost > 0.0);
    }

    // Test 7: SS taxability — provisional income rises with conversion
    #[test]
    fn test_ss_taxability() {
        let income = IncomeBreakdown {
            taxable_pensions: 10000.0,
            ..Default::default()
        };
        let mut req = make_single_request(income, Some(20000.0), 500000.0);
        req.gross_social_security_benefit = Some(24000.0);
        req.ss_taxation_params = Some(SsTaxationParams {
            base_threshold: 25000.0,
            upper_threshold: 34000.0,
            max_taxable_pct_below_upper: 0.50,
            max_taxable_pct_above_upper: 0.85,
        });
        // Remove any SS from income since we'll compute it dynamically
        req.income.taxable_social_security = 0.0;

        let res = analyze_roth_conversion(&req);

        // Baseline: AGI_ex_ss = 10000, provisional = 10000 + 0 + 0.5*24000 = 22000 < 25000 → 0 taxable SS
        assert_eq!(res.baseline.taxable_social_security, 0.0);

        // With conversion: AGI_ex_ss = 10000 + 20000 = 30000, provisional = 30000 + 12000 = 42000
        // Between 34000 (upper) — wait, 42000 > 34000
        // tier1 = 0.5 * (34000 - 25000) = 4500
        // tier2 = 0.85 * (42000 - 34000) = 0.85 * 8000 = 6800
        // total = 11300
        // max = 0.85 * 24000 = 20400
        // taxable = min(20400, 11300) = 11300
        assert_eq!(res.with_conversion.taxable_social_security, 11300.0);
        assert_eq!(res.additional_ss_taxable, 11300.0);
    }

    // Test 8: MFJ with persons=2 for IRMAA
    #[test]
    fn test_mfj_irmaa_two_persons() {
        let income = IncomeBreakdown {
            wages: 216000.0,
            ..Default::default()
        };
        let req = RothConversionRequest {
            filing_status: "married_filing_jointly".to_string(),
            tax_year: 2025,
            income,
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
                ordinary_brackets: mfj_2025_brackets(),
                capital_gains_brackets: mfj_cg_brackets(),
                standard_deduction: 30000.0,
                capital_loss_limit: 3000.0,
                niit: NiitParams {
                    rate: 0.038,
                    threshold: 250000.0,
                },
                payroll: PayrollParams {
                    additional_medicare_threshold: 250000.0,
                    ..default_payroll()
                },
                salt: Some(default_salt()),
            },
            conversion_amount: Some(5000.0),
            traditional_ira_balance: 500000.0,
            nondeductible_basis: 0.0,
            total_traditional_ira_value: None,
            irmaa_brackets: Some(IrmaaBrackets {
                base_premium_part_b: 202.90,
                tiers: vec![
                    IrmaaTier {
                        magi_floor: 0.0,
                        magi_ceiling: Some(218000.0),
                        surcharge_part_b: 0.0,
                        surcharge_part_d: 0.0,
                    },
                    IrmaaTier {
                        magi_floor: 218000.0,
                        magi_ceiling: Some(274000.0),
                        surcharge_part_b: 81.20,
                        surcharge_part_d: 14.50,
                    },
                ],
                persons: 2,
            }),
            gross_social_security_benefit: None,
            ss_taxation_params: None,
        };

        let res = analyze_roth_conversion(&req);
        let irmaa = res.irmaa_impact.unwrap();

        // Baseline AGI = 216000, below 218K → tier 0
        // With conversion AGI = 221000, above 218K → tier 1
        // Surcharge = (81.20 + 14.50) * 12 * 2 = 95.70 * 24 = 2296.80
        assert_eq!(irmaa.baseline_tier_index, 0);
        assert_eq!(irmaa.conversion_tier_index, 1);
        assert_eq!(irmaa.persons, 2);
        assert_eq!(irmaa.incremental_annual_surcharge, 2296.8);
    }

    // Test 9: Pro-rata rule — $500K trad IRA with $50K basis
    #[test]
    fn test_pro_rata_rule() {
        let income = IncomeBreakdown {
            wages: 75000.0,
            ..Default::default()
        };
        let mut req = make_single_request(income, Some(100000.0), 500000.0);
        req.nondeductible_basis = 50000.0;

        let res = analyze_roth_conversion(&req);

        // basis_ratio = 50000 / 500000 = 0.10
        // taxable = 100000 * 0.90 = 90000
        // nontaxable = 100000 * 0.10 = 10000
        assert_eq!(res.pro_rata.basis_ratio, 0.10);
        assert_eq!(res.pro_rata.taxable_conversion, 90000.0);
        assert_eq!(res.pro_rata.nontaxable_conversion, 10000.0);
    }

    // Test 10: Pro-rata with SEP IRA
    #[test]
    fn test_pro_rata_with_sep() {
        let income = IncomeBreakdown {
            wages: 75000.0,
            ..Default::default()
        };
        let mut req = make_single_request(income, Some(100000.0), 500000.0);
        req.nondeductible_basis = 50000.0;
        req.total_traditional_ira_value = Some(700000.0); // 500K trad + 200K SEP

        let res = analyze_roth_conversion(&req);

        // basis_ratio = 50000 / 700000 ≈ 0.0714
        // taxable = 100000 * (1 - 0.0714) = 92857.14
        let expected_ratio = round4(50000.0 / 700000.0);
        assert_eq!(res.pro_rata.basis_ratio, expected_ratio);
        assert!((res.pro_rata.taxable_conversion - 92857.14).abs() < 0.01);
    }

    // Test 11: Two-year fixed conversion (multi-year strategy)
    #[test]
    fn test_two_year_fixed_conversion() {
        let income = IncomeBreakdown {
            wages: 60000.0,
            ..Default::default()
        };
        let req = RothConversionStrategyRequest {
            filing_status: "single".to_string(),
            tax_year: 2025,
            income,
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
                ordinary_brackets: single_2025_brackets(),
                capital_gains_brackets: single_cg_brackets(),
                standard_deduction: 15000.0,
                capital_loss_limit: 3000.0,
                niit: NiitParams {
                    rate: 0.038,
                    threshold: 200000.0,
                },
                payroll: default_payroll(),
                salt: Some(default_salt()),
            },
            traditional_ira_balance: 200000.0,
            roth_ira_balance: 50000.0,
            nondeductible_basis: 0.0,
            total_traditional_ira_value: None,
            owner_birth_date: "1962-01-01".to_string(),
            annual_growth_rate: 0.06,
            projection_years: 2,
            strategy: "fixed_amount".to_string(),
            target_bracket_rate: None,
            fixed_annual_conversion: Some(30000.0),
            income_events: vec![],
            irmaa_brackets: None,
            gross_social_security_benefit: None,
            ss_taxation_params: None,
            uniform_lifetime_table: None,
            rmd_start_age: Some(73),
        };

        let res = compute_roth_conversion_strategy(&req).unwrap();

        assert_eq!(res.annual_detail.len(), 2);
        assert_eq!(res.annual_detail[0].conversion_amount, 30000.0);
        assert_eq!(res.annual_detail[1].conversion_amount, 30000.0);

        // After 2 years of $30K conversions, trad should be reduced
        // Year 0: 200K - 30K = 170K, grow → 170K * 1.06 = 180200
        // Year 1: 180200 - 30K = 150200, grow → 150200 * 1.06 = 159212
        assert!(
            res.convert_scenario.final_trad_ira_balance
                < res.do_nothing_scenario.final_trad_ira_balance
        );
        assert!(
            res.convert_scenario.final_roth_ira_balance
                > res.do_nothing_scenario.final_roth_ira_balance
        );
    }

    // Test 12: RMD reduction from conversions
    #[test]
    fn test_rmd_reduction() {
        let income = IncomeBreakdown {
            taxable_pensions: 40000.0,
            ..Default::default()
        };
        let table = vec![
            UniformLifetimeEntry {
                age: 73,
                distribution_period: 26.5,
            },
            UniformLifetimeEntry {
                age: 74,
                distribution_period: 25.5,
            },
            UniformLifetimeEntry {
                age: 75,
                distribution_period: 24.6,
            },
        ];
        let req = RothConversionStrategyRequest {
            filing_status: "single".to_string(),
            tax_year: 2025,
            income,
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
                ordinary_brackets: single_2025_brackets(),
                capital_gains_brackets: single_cg_brackets(),
                standard_deduction: 15000.0,
                capital_loss_limit: 3000.0,
                niit: NiitParams {
                    rate: 0.038,
                    threshold: 200000.0,
                },
                payroll: default_payroll(),
                salt: Some(default_salt()),
            },
            traditional_ira_balance: 500000.0,
            roth_ira_balance: 100000.0,
            nondeductible_basis: 0.0,
            total_traditional_ira_value: None,
            owner_birth_date: "1952-01-01".to_string(),
            annual_growth_rate: 0.0, // 0% growth to simplify
            projection_years: 3,
            strategy: "fixed_amount".to_string(),
            target_bracket_rate: None,
            fixed_annual_conversion: Some(50000.0),
            income_events: vec![],
            irmaa_brackets: None,
            gross_social_security_benefit: None,
            ss_taxation_params: None,
            uniform_lifetime_table: Some(table),
            rmd_start_age: Some(73),
        };

        let res = compute_roth_conversion_strategy(&req).unwrap();

        // With conversions, trad balance is lower → RMDs are lower
        for row in &res.annual_detail {
            if row.rmd_convert > 0.0 || row.rmd_no_convert > 0.0 {
                assert!(
                    row.rmd_convert <= row.rmd_no_convert,
                    "Convert RMD ({}) should be <= no-convert RMD ({})",
                    row.rmd_convert,
                    row.rmd_no_convert
                );
            }
        }
        assert!(res.convert_scenario.total_rmds <= res.do_nothing_scenario.total_rmds);
    }

    // Test 13: Income events — wages stop, SS starts
    #[test]
    fn test_income_events() {
        let income = IncomeBreakdown {
            wages: 80000.0,
            ..Default::default()
        };
        let req = RothConversionStrategyRequest {
            filing_status: "single".to_string(),
            tax_year: 2025,
            income,
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
                ordinary_brackets: single_2025_brackets(),
                capital_gains_brackets: single_cg_brackets(),
                standard_deduction: 15000.0,
                capital_loss_limit: 3000.0,
                niit: NiitParams {
                    rate: 0.038,
                    threshold: 200000.0,
                },
                payroll: default_payroll(),
                salt: Some(default_salt()),
            },
            traditional_ira_balance: 300000.0,
            roth_ira_balance: 50000.0,
            nondeductible_basis: 0.0,
            total_traditional_ira_value: None,
            owner_birth_date: "1962-01-01".to_string(),
            annual_growth_rate: 0.0,
            projection_years: 4,
            strategy: "fixed_amount".to_string(),
            target_bracket_rate: None,
            fixed_annual_conversion: Some(20000.0),
            income_events: vec![
                IncomeEvent {
                    start_year: 2027,
                    end_year: None,
                    income_field: "wages".to_string(),
                    amount: 0.0,
                },
                IncomeEvent {
                    start_year: 2028,
                    end_year: None,
                    income_field: "taxable_social_security".to_string(),
                    amount: 20000.0,
                },
            ],
            irmaa_brackets: None,
            gross_social_security_benefit: None,
            ss_taxation_params: None,
            uniform_lifetime_table: None,
            rmd_start_age: Some(73),
        };

        let res = compute_roth_conversion_strategy(&req).unwrap();

        // Year 2025 (age 63): wages=80K, conversion=20K
        // Year 2026 (age 64): wages=80K, conversion=20K
        // Year 2027 (age 65): wages=0 (event), conversion=20K
        // Year 2028 (age 66): wages=0, SS=20K (event), conversion=20K
        assert_eq!(res.annual_detail.len(), 4);
        // Conversion tax cost should be lower in years without wages
        assert!(
            res.annual_detail[2].conversion_tax_cost < res.annual_detail[0].conversion_tax_cost
        );
    }

    // Test 14: Do-nothing baseline — zero conversions → scenarios match
    #[test]
    fn test_do_nothing_baseline() {
        let income = IncomeBreakdown {
            wages: 60000.0,
            ..Default::default()
        };
        let req = RothConversionStrategyRequest {
            filing_status: "single".to_string(),
            tax_year: 2025,
            income,
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
                ordinary_brackets: single_2025_brackets(),
                capital_gains_brackets: single_cg_brackets(),
                standard_deduction: 15000.0,
                capital_loss_limit: 3000.0,
                niit: NiitParams {
                    rate: 0.038,
                    threshold: 200000.0,
                },
                payroll: default_payroll(),
                salt: Some(default_salt()),
            },
            traditional_ira_balance: 200000.0,
            roth_ira_balance: 50000.0,
            nondeductible_basis: 0.0,
            total_traditional_ira_value: None,
            owner_birth_date: "1970-01-01".to_string(),
            annual_growth_rate: 0.06,
            projection_years: 3,
            strategy: "fixed_amount".to_string(),
            target_bracket_rate: None,
            fixed_annual_conversion: Some(0.0),
            income_events: vec![],
            irmaa_brackets: None,
            gross_social_security_benefit: None,
            ss_taxation_params: None,
            uniform_lifetime_table: None,
            rmd_start_age: Some(73),
        };

        let res = compute_roth_conversion_strategy(&req).unwrap();

        for row in &res.annual_detail {
            assert_eq!(row.conversion_amount, 0.0);
            assert_eq!(row.conversion_tax_cost, 0.0);
        }
        // Convert and no-convert trad balances should be equal
        assert!(
            (res.convert_scenario.final_trad_ira_balance
                - res.do_nothing_scenario.final_trad_ira_balance)
                .abs()
                < 0.01
        );
        assert_eq!(res.comparison.total_conversion_tax, 0.0);
    }

    // Test 15: SS taxability — IRS Pub 915 formula unit test
    #[test]
    fn test_compute_taxable_ss_below_base() {
        let params = SsTaxationParams {
            base_threshold: 25000.0,
            upper_threshold: 34000.0,
            max_taxable_pct_below_upper: 0.50,
            max_taxable_pct_above_upper: 0.85,
        };
        // provisional = 10000 + 0 + 0.5*20000 = 20000 < 25000
        let result = compute_taxable_ss(20000.0, 10000.0, 0.0, &params);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_compute_taxable_ss_between_thresholds() {
        let params = SsTaxationParams {
            base_threshold: 25000.0,
            upper_threshold: 34000.0,
            max_taxable_pct_below_upper: 0.50,
            max_taxable_pct_above_upper: 0.85,
        };
        // provisional = 20000 + 0 + 0.5*24000 = 32000
        // Between 25K and 34K
        // taxable = min(0.50 * 24000, 0.5 * (32000 - 25000)) = min(12000, 3500) = 3500
        let result = compute_taxable_ss(24000.0, 20000.0, 0.0, &params);
        assert_eq!(result, 3500.0);
    }

    #[test]
    fn test_compute_taxable_ss_above_upper() {
        let params = SsTaxationParams {
            base_threshold: 25000.0,
            upper_threshold: 34000.0,
            max_taxable_pct_below_upper: 0.50,
            max_taxable_pct_above_upper: 0.85,
        };
        // provisional = 40000 + 0 + 0.5*24000 = 52000
        // Above 34K
        // tier1 = 0.5 * (34000 - 25000) = 4500
        // tier2 = 0.85 * (52000 - 34000) = 0.85 * 18000 = 15300
        // total = 19800
        // max = 0.85 * 24000 = 20400
        // taxable = min(20400, 19800) = 19800
        let result = compute_taxable_ss(24000.0, 40000.0, 0.0, &params);
        assert_eq!(result, 19800.0);
    }

    #[test]
    fn test_compute_taxable_ss_mfj_thresholds() {
        let params = SsTaxationParams {
            base_threshold: 32000.0,
            upper_threshold: 44000.0,
            max_taxable_pct_below_upper: 0.50,
            max_taxable_pct_above_upper: 0.85,
        };
        // provisional = 30000 + 0 + 0.5*36000 = 48000
        // Above 44K
        // tier1 = 0.5 * (44000 - 32000) = 6000
        // tier2 = 0.85 * (48000 - 44000) = 3400
        // total = 9400
        // max = 0.85 * 36000 = 30600
        // taxable = min(30600, 9400) = 9400
        let result = compute_taxable_ss(36000.0, 30000.0, 0.0, &params);
        assert_eq!(result, 9400.0);
    }
}
