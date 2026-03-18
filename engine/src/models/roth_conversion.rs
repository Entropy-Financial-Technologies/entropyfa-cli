use serde::{Deserialize, Serialize};

use super::tax_request::{Adjustments, DeductionConfig, IncomeBreakdown, TaxParameters};
use super::tax_response::BracketDetail;

// ---------------------------------------------------------------------------
// Single-year request/response
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RothConversionRequest {
    pub filing_status: String,
    #[serde(default = "default_tax_year")]
    pub tax_year: u32,
    pub income: IncomeBreakdown,
    #[serde(default)]
    pub adjustments: Adjustments,
    pub deductions: DeductionConfig,
    pub tax_parameters: TaxParameters,

    /// Explicit conversion amount; omit to auto-fill current bracket
    pub conversion_amount: Option<f64>,
    pub traditional_ira_balance: f64,

    /// Non-deductible basis in traditional IRA (Form 8606). Defaults to 0.
    #[serde(default)]
    pub nondeductible_basis: f64,
    /// Total value of all traditional/SEP/SIMPLE IRAs (for pro-rata rule).
    /// Defaults to traditional_ira_balance if not provided.
    pub total_traditional_ira_value: Option<f64>,

    /// IRMAA brackets for Medicare surcharge cliff analysis (optional).
    pub irmaa_brackets: Option<IrmaaBrackets>,

    /// Gross (pre-tax) Social Security benefit. If provided, taxable SS is
    /// dynamically recomputed using IRS Pub 915 provisional income formula.
    pub gross_social_security_benefit: Option<f64>,
    /// SS taxation thresholds (from social_security/benefit_taxation_thresholds pipeline).
    /// Required if gross_social_security_benefit is provided.
    pub ss_taxation_params: Option<SsTaxationParams>,
}

#[derive(Debug, Serialize)]
pub struct RothConversionResponse {
    pub baseline: TaxSummary,
    pub with_conversion: TaxSummary,

    pub conversion_amount: f64,
    pub conversion_tax_cost: f64,
    pub effective_rate_on_conversion: f64,
    pub marginal_rate_on_conversion: f64,

    pub pro_rata: ProRataDetail,
    pub bracket_space: Vec<BracketSpaceDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub irmaa_impact: Option<IrmaaImpact>,
    pub niit_increase: f64,
    pub additional_ss_taxable: f64,
}

#[derive(Debug, Serialize)]
pub struct TaxSummary {
    pub agi: f64,
    pub taxable_income: f64,
    pub ordinary_income_tax: f64,
    pub capital_gains_tax: f64,
    pub niit: f64,
    pub total_income_tax: f64,
    pub total_tax: f64,
    pub effective_rate: f64,
    pub marginal_ordinary_rate: f64,
    pub taxable_social_security: f64,
    pub ordinary_bracket_detail: Vec<BracketDetail>,
}

#[derive(Debug, Serialize)]
pub struct ProRataDetail {
    pub total_traditional_ira_value: f64,
    pub nondeductible_basis: f64,
    pub basis_ratio: f64,
    pub taxable_conversion: f64,
    pub nontaxable_conversion: f64,
}

#[derive(Debug, Serialize)]
pub struct BracketSpaceDetail {
    pub rate: f64,
    pub bracket_min: f64,
    pub bracket_max: Option<f64>,
    pub already_used: f64,
    pub room_available: f64,
    pub conversion_in_bracket: f64,
    pub tax_cost_in_bracket: f64,
    pub cumulative_conversion: f64,
    pub cumulative_tax_cost: f64,
}

// ---------------------------------------------------------------------------
// IRMAA
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrmaaBrackets {
    pub base_premium_part_b: f64,
    pub tiers: Vec<IrmaaTier>,
    /// Number of Medicare enrollees (1 or 2 for couples). Defaults to 1.
    #[serde(default = "default_persons")]
    pub persons: u32,
}

fn default_persons() -> u32 {
    1
}

fn default_tax_year() -> u32 {
    2026
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IrmaaTier {
    pub magi_floor: f64,
    pub magi_ceiling: Option<f64>,
    pub surcharge_part_b: f64,
    pub surcharge_part_d: f64,
}

#[derive(Debug, Serialize)]
pub struct IrmaaImpact {
    pub baseline_magi: f64,
    pub conversion_magi: f64,
    pub baseline_tier_index: usize,
    pub conversion_tier_index: usize,
    pub baseline_annual_surcharge: f64,
    pub conversion_annual_surcharge: f64,
    pub incremental_annual_surcharge: f64,
    pub persons: u32,
}

// ---------------------------------------------------------------------------
// Social Security taxation parameters
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SsTaxationParams {
    pub base_threshold: f64,
    pub upper_threshold: f64,
    pub max_taxable_pct_below_upper: f64,
    pub max_taxable_pct_above_upper: f64,
}

// ---------------------------------------------------------------------------
// Multi-year strategy request/response
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RothConversionStrategyRequest {
    pub filing_status: String,
    #[serde(default = "default_tax_year")]
    pub tax_year: u32,
    pub income: IncomeBreakdown,
    #[serde(default)]
    pub adjustments: Adjustments,
    pub deductions: DeductionConfig,
    pub tax_parameters: TaxParameters,

    pub traditional_ira_balance: f64,
    #[serde(default)]
    pub roth_ira_balance: f64,
    #[serde(default)]
    pub nondeductible_basis: f64,
    pub total_traditional_ira_value: Option<f64>,

    pub owner_birth_date: String,
    pub annual_growth_rate: f64,
    pub projection_years: u32,

    pub strategy: String,
    pub target_bracket_rate: Option<f64>,
    pub fixed_annual_conversion: Option<f64>,

    #[serde(default)]
    pub income_events: Vec<IncomeEvent>,

    pub irmaa_brackets: Option<IrmaaBrackets>,
    pub gross_social_security_benefit: Option<f64>,
    pub ss_taxation_params: Option<SsTaxationParams>,

    /// Uniform lifetime table for RMD projections (optional).
    /// If provided, RMDs are projected for both scenarios.
    pub uniform_lifetime_table: Option<Vec<UniformLifetimeEntry>>,
    /// RMD start age (e.g. 73 or 75). Defaults to 73.
    pub rmd_start_age: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IncomeEvent {
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub income_field: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UniformLifetimeEntry {
    pub age: u32,
    pub distribution_period: f64,
}

#[derive(Debug, Serialize)]
pub struct RothConversionStrategyResponse {
    pub strategy: String,
    pub projection_years: u32,
    pub annual_growth_rate: f64,
    pub annual_detail: Vec<StrategyYearDetail>,
    pub convert_scenario: ScenarioSummary,
    pub do_nothing_scenario: ScenarioSummary,
    pub comparison: StrategyComparison,
}

#[derive(Debug, Serialize)]
pub struct StrategyYearDetail {
    pub year: u32,
    pub age: u32,
    pub conversion_amount: f64,
    pub taxable_conversion: f64,
    pub conversion_tax_cost: f64,
    pub effective_rate_on_conversion: f64,
    pub trad_ira_balance_convert: f64,
    pub roth_ira_balance_convert: f64,
    pub trad_ira_balance_no_convert: f64,
    pub rmd_convert: f64,
    pub rmd_no_convert: f64,
    pub rmd_tax_convert: f64,
    pub rmd_tax_no_convert: f64,
}

#[derive(Debug, Serialize)]
pub struct ScenarioSummary {
    pub final_trad_ira_balance: f64,
    pub final_roth_ira_balance: f64,
    pub total_taxes_paid: f64,
    pub total_rmds: f64,
}

#[derive(Debug, Serialize)]
pub struct StrategyComparison {
    pub total_conversion_tax: f64,
    pub total_rmd_tax_savings: f64,
    pub net_tax_savings: f64,
    pub breakeven_year: Option<u32>,
    pub convert_after_tax_total: f64,
    pub no_convert_after_tax_total: f64,
}
