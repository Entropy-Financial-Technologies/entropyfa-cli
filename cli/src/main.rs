use clap::{Parser, Subcommand};

mod assembler;
mod chart;
mod commands;
mod input;
mod output;
mod schema;
mod version;
mod webhook;

#[derive(Parser)]
#[command(
    name = "entropyfa",
    version = concat!(env!("CARGO_PKG_VERSION"), " (data: 0.1.0)"),
    about = "Verified financial reference data + deterministic computation engine",
    long_about = "entropyfa embeds IRS-sourced reference data (tax brackets, RMD tables,\n\
        mortality tables, IRMAA brackets) and runs deterministic financial\n\
        calculations locally by default. No API keys, no config files.",
    after_help = "EXAMPLES:\n\
        entropyfa data coverage\n\
        entropyfa data lookup --category tax --key brackets --filing-status single\n\
        entropyfa compute federal-tax --json '{\"filing_status\":\"single\",\"income\":{\"wages\":100000}}'\n\
        entropyfa compute federal-tax --schema"
)]
struct Cli {
    /// POST the final JSON envelope to a webhook endpoint
    #[arg(long = "result-hook-url", alias = "result_hook_url", global = true)]
    result_hook_url: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Query verified reference data (rates, limits, rules, tables)
    Data {
        #[command(subcommand)]
        action: DataAction,
    },
    /// Run deterministic financial calculations
    Compute {
        #[command(subcommand)]
        action: ComputeAction,
    },
    /// Update entropyfa to the latest version
    Upgrade,
}

#[derive(Subcommand)]
enum DataAction {
    /// Look up specific reference data
    Lookup {
        #[arg(long)]
        category: String,
        #[arg(long)]
        key: String,
        #[arg(long, default_value = "2026")]
        year: u32,
        #[arg(long)]
        filing_status: Option<String>,
    },
    /// Discover available reference data
    Coverage {
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        year: Option<u32>,
    },
}

#[derive(Subcommand)]
enum ComputeAction {
    /// Federal income + payroll taxes
    FederalTax {
        /// Print input schema instead of computing
        #[arg(long)]
        schema: bool,
        /// JSON input payload
        #[arg(long)]
        json: Option<String>,
    },
    /// Federal estate tax (Form 706)
    EstateTax {
        /// Print input schema instead of computing
        #[arg(long)]
        schema: bool,
        /// JSON input payload
        #[arg(long)]
        json: Option<String>,
    },
    /// Required minimum distribution
    Rmd {
        /// Print input schema instead of computing
        #[arg(long)]
        schema: bool,
        /// JSON input payload
        #[arg(long)]
        json: Option<String>,
    },
    /// Multi-year RMD projection
    RmdSchedule {
        /// Print input schema instead of computing
        #[arg(long)]
        schema: bool,
        /// JSON input payload
        #[arg(long)]
        json: Option<String>,
    },
    /// Roth conversion tax impact
    RothConversion {
        /// Print input schema instead of computing
        #[arg(long)]
        schema: bool,
        /// JSON input payload
        #[arg(long)]
        json: Option<String>,
    },
    /// Multi-year Roth conversion strategy
    RothConversionStrategy {
        /// Print input schema instead of computing
        #[arg(long)]
        schema: bool,
        /// JSON input payload
        #[arg(long)]
        json: Option<String>,
    },
    /// Pension lump sum vs annuity comparison
    PensionComparison {
        /// Print input schema instead of computing
        #[arg(long)]
        schema: bool,
        /// JSON input payload
        #[arg(long)]
        json: Option<String>,
    },
    /// Monte Carlo / linear projection
    Projection {
        /// Print input schema instead of computing
        #[arg(long)]
        schema: bool,
        /// JSON input payload
        #[arg(long)]
        json: Option<String>,
        /// Deprecated: projection results render a terminal dashboard automatically
        #[arg(long, hide = true)]
        chart: bool,
        /// Include period-by-period detail breakdown
        #[arg(long)]
        detail: bool,
        /// Detail granularity: 'annual' (default) or 'monthly'
        #[arg(long)]
        detail_granularity: Option<String>,
        /// Return N evenly-spaced sample simulation paths
        #[arg(long)]
        sample_paths: Option<usize>,
        /// Return specific simulation paths by index (comma-separated)
        #[arg(long, value_delimiter = ',')]
        path_indices: Option<Vec<usize>>,
        /// Custom percentile time series (comma-separated, 0-100)
        #[arg(long, value_delimiter = ',')]
        percentiles: Option<Vec<u32>>,
    },
    /// Binary search goal solver
    GoalSolver {
        /// Print input schema instead of computing
        #[arg(long)]
        schema: bool,
        /// JSON input payload
        #[arg(long)]
        json: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let Cli {
        result_hook_url,
        command,
    } = cli;

    output::set_result_hook_url(result_hook_url);
    version::check_and_warn();

    match command {
        Command::Data { action } => match action {
            DataAction::Lookup {
                category,
                key,
                year,
                filing_status,
            } => commands::data::run_lookup(&category, &key, year, filing_status.as_deref()),
            DataAction::Coverage { category, year } => {
                commands::data::run_coverage(category.as_deref(), year)
            }
        },
        Command::Compute { action } => match action {
            ComputeAction::FederalTax { schema, json } => {
                commands::tax::run_federal_tax(schema, json)
            }
            ComputeAction::EstateTax { schema, json } => {
                commands::tax::run_estate_tax(schema, json)
            }
            ComputeAction::Rmd { schema, json } => commands::retirement::run_rmd(schema, json),
            ComputeAction::RmdSchedule { schema, json } => {
                commands::retirement::run_rmd_schedule(schema, json)
            }
            ComputeAction::RothConversion { schema, json } => {
                commands::retirement::run_roth(schema, json)
            }
            ComputeAction::RothConversionStrategy { schema, json } => {
                commands::retirement::run_roth_strategy(schema, json)
            }
            ComputeAction::PensionComparison { schema, json } => {
                commands::pension::run_pension(schema, json)
            }
            ComputeAction::Projection {
                schema,
                json,
                chart,
                detail,
                detail_granularity,
                sample_paths,
                path_indices,
                percentiles,
            } => commands::simulation::run_simulate(
                schema,
                json,
                chart,
                detail,
                detail_granularity,
                sample_paths,
                path_indices,
                percentiles,
            ),
            ComputeAction::GoalSolver { schema, json } => {
                commands::simulation::run_solve(schema, json)
            }
        },
        Command::Upgrade => version::run_upgrade(),
    }
}
