use serde::Deserialize;

use super::roth_conversion::{SsTaxationParams, UniformLifetimeEntry};
use super::tax_request::{Adjustments, DeductionConfig, IncomeBreakdown, TaxParameters};

#[derive(Debug, Deserialize)]
pub struct PensionComparisonRequest {
    // Participant
    pub retiree_age: u32,
    pub retiree_gender: Option<String>,
    pub spouse_age: Option<u32>,
    pub spouse_gender: Option<String>,

    // Pension options
    pub selected_option_id: Option<String>,
    pub lump_sum_amount: Option<f64>,
    pub annuity_options: Vec<AnnuityOption>,

    // Investment assumptions (lump sum scenario)
    pub investment_return: ReturnAssumption,
    #[serde(default = "default_discount_rate")]
    pub discount_rate: f64,

    // Tax
    pub filing_status: String,
    #[allow(dead_code)]
    pub tax_year: u32,
    pub income: IncomeBreakdown,
    #[serde(default)]
    pub adjustments: Adjustments,
    pub deductions: DeductionConfig,
    pub tax_parameters: TaxParameters,

    // Social Security
    pub gross_social_security_benefit: Option<f64>,
    pub ss_taxation_params: Option<SsTaxationParams>,

    // Assumptions
    #[serde(default)]
    pub inflation_rate: f64,
    #[serde(default)]
    pub cola_rate: f64,

    // Overrides
    pub retiree_life_expectancy_override: Option<u32>,
    pub spouse_life_expectancy_override: Option<u32>,

    // Lump sum handling
    #[serde(default = "default_true")]
    pub rollover_to_ira: bool,
    #[serde(default)]
    pub after_tax_contribution_basis: f64,
    pub separation_age: Option<u32>,
    #[serde(default)]
    pub taxable_account_assumptions: TaxableAccountAssumptions,

    // Monte Carlo
    pub num_simulations: Option<u32>,
    pub seed: Option<u64>,

    // Reference data (assembled by Rails)
    pub mortality_table: MortalityTable,
    pub uniform_lifetime_table: Option<Vec<UniformLifetimeEntry>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnnuityOption {
    #[serde(default)]
    pub id: Option<String>,
    pub option_type: String,
    #[serde(default)]
    pub label: Option<String>,
    pub monthly_payment: f64,

    // Joint survivor fields
    pub survivor_pct: Option<f64>,
    pub survivor_monthly_payment: Option<f64>,
    pub popup_monthly_payment: Option<f64>,

    // Period certain fields
    pub period_certain_months: Option<u32>,

    // Level income (SS leveling) fields
    pub level_income_age: Option<u32>,
    pub level_income_reduced_payment: Option<f64>,

    // Per-option COLA (overrides global cola_rate)
    pub cola: Option<ColaConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColaConfig {
    pub rate: f64,
    #[serde(default)]
    pub start_delay_months: u32,
    pub cap_rate: Option<f64>,
    pub end_age: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MortalityTable {
    pub male_table: Option<Vec<MortalityEntry>>,
    pub female_table: Option<Vec<MortalityEntry>>,
    pub unisex_table: Vec<MortalityEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MortalityEntry {
    pub age: u32,
    pub qx: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReturnAssumption {
    #[serde(default)]
    pub annual_mean: f64,
    #[serde(default)]
    pub annual_std_dev: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TaxableAccountAssumptions {
    #[serde(default)]
    pub taxable_interest_yield: f64,
    #[serde(default)]
    pub ordinary_dividend_yield: f64,
    #[serde(default)]
    pub qualified_dividend_yield: f64,
    #[serde(default)]
    pub short_term_capital_gain_yield: f64,
    #[serde(default)]
    pub long_term_capital_gain_yield: f64,
}

fn default_discount_rate() -> f64 {
    0.05
}

fn default_true() -> bool {
    true
}
