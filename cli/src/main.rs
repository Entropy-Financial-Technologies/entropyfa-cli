use clap::{Parser, Subcommand};

mod assembler;
mod commands;
mod input;
mod output;
mod schema;

#[derive(Parser)]
#[command(
    name = "entropyfa",
    version = concat!(env!("CARGO_PKG_VERSION"), " (data: 0.1.0)"),
    about = "Verified financial reference data + deterministic computation engine",
    long_about = "entropyfa embeds IRS-sourced reference data (tax brackets, RMD tables,\n\
        mortality tables, IRMAA brackets) and runs deterministic financial\n\
        calculations locally. No network calls, no API keys, no config files.",
    after_help = "EXAMPLES:\n\
        entropyfa data coverage\n\
        entropyfa data lookup --category tax --key brackets --filing-status single\n\
        echo '{\"filing_status\":\"single\",\"income\":{\"wages\":100000}}' | entropyfa compute federal-tax\n\
        entropyfa compute federal-tax --schema"
)]
struct Cli {
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
        #[arg(long)]
        schema: bool,
    },
    /// Federal estate tax (Form 706)
    EstateTax {
        #[arg(long)]
        schema: bool,
    },
    /// Required minimum distribution
    Rmd {
        #[arg(long)]
        schema: bool,
    },
    /// Multi-year RMD projection
    RmdSchedule {
        #[arg(long)]
        schema: bool,
    },
    /// Roth conversion tax impact
    Roth {
        #[arg(long)]
        schema: bool,
    },
    /// Multi-year Roth conversion strategy
    RothStrategy {
        #[arg(long)]
        schema: bool,
    },
    /// Pension lump sum vs annuity comparison
    Pension {
        #[arg(long)]
        schema: bool,
    },
    /// Monte Carlo / linear projection
    Simulate {
        #[arg(long)]
        schema: bool,
    },
    /// Binary search goal solver
    Solve {
        #[arg(long)]
        schema: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
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
            ComputeAction::FederalTax { schema } => commands::tax::run_federal_tax(schema),
            ComputeAction::EstateTax { schema } => commands::tax::run_estate_tax(schema),
            ComputeAction::Rmd { schema } => commands::retirement::run_rmd(schema),
            ComputeAction::RmdSchedule { schema } => commands::retirement::run_rmd_schedule(schema),
            ComputeAction::Roth { schema } => commands::retirement::run_roth(schema),
            ComputeAction::RothStrategy { schema } => {
                commands::retirement::run_roth_strategy(schema)
            }
            ComputeAction::Pension { schema } => commands::pension::run_pension(schema),
            ComputeAction::Simulate { schema } => commands::simulation::run_simulate(schema),
            ComputeAction::Solve { schema } => commands::simulation::run_solve(schema),
        },
    }
}
