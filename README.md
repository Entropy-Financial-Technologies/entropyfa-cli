# entropyfa

[![CI](https://github.com/Entropy-Financial-Technologies/entropyfa-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/Entropy-Financial-Technologies/entropyfa-cli/actions/workflows/ci.yml)

A personal finance and financial planning engine built for AI agents. Gives any LLM access to verified IRS reference data and deterministic financial calculations — tax math, RMD computations, Roth conversion analysis, Monte Carlo simulations, pension valuations — all fully offline, sub-millisecond, JSON-in/JSON-out.

**Why?** Financial planning agents need two things they can't do well on their own: (1) verified reference data — rates, limits, rules, tables that change annually and must be IRS-sourced, not hallucinated, and (2) deterministic calculations — tax bracket stacking, actuarial math, Monte Carlo simulations. entropyfa bundles both into a single binary with zero configuration.

## 30-Second Demo

```sh
# What data is available?
entropyfa data coverage

# Look up 2026 tax brackets
entropyfa data lookup --category tax --key brackets --filing-status single

# Compute federal tax
entropyfa compute federal-tax --json '{"filing_status":"single","income":{"wages":150000}}'

# What does this command need?
entropyfa compute roth-conversion --schema

# Run a Roth conversion analysis
entropyfa compute roth-conversion --json '{"filing_status":"married_filing_jointly","income":{"wages":200000},"conversion_amount":50000}'

# Monte Carlo retirement projection
entropyfa compute projection --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'
```

## Install

**Quick install** (macOS / Linux):

```sh
curl -fsSL https://get.entropyfa.com | sh
```

**Cargo**:

```sh
cargo install --git https://github.com/Entropy-Financial-Technologies/entropyfa-cli.git entropyfa
```

**From source**:

```sh
git clone https://github.com/Entropy-Financial-Technologies/entropyfa-cli.git
cd entropyfa-cli
cargo build --release
cp target/release/entropyfa /usr/local/bin/
```

## Upgrade

```sh
entropyfa upgrade
```

This checks GitHub for the latest release, downloads the new binary for your platform, and replaces the current executable. The CLI also checks for updates in the background — if a newer version is available, you'll see a reminder on stderr.

## Commands

### Data

| Command | Description |
|---------|-------------|
| `data lookup` | Look up specific reference data by category, key, year, and filing status |
| `data coverage` | Discover all available reference data |

### Compute

| Command | Description |
|---------|-------------|
| `compute federal-tax` | Federal income + payroll taxes |
| `compute estate-tax` | Federal estate tax (Form 706) |
| `compute rmd` | Required minimum distribution for a single year |
| `compute rmd-schedule` | Multi-year RMD projection |
| `compute roth-conversion` | Roth conversion tax impact analysis |
| `compute roth-conversion-strategy` | Multi-year Roth conversion strategy |
| `compute pension-comparison` | Pension lump sum vs annuity comparison |
| `compute projection` | Monte Carlo / linear projection |
| `compute goal-solver` | Binary search goal solver |

Every compute command supports `--schema` to emit agent-oriented guidance: what inputs are needed, what to gather from the user, why to use this command, and related commands.

## Embedded Data Sources

All data is sourced from IRS publications and embedded at compile time:

- **Tax brackets** -- federal income tax rates by filing status (Rev. Proc.)
- **Standard deductions** -- by filing status, including age 65+ and blind additions
- **Capital gains brackets** -- 0%/15%/20% thresholds by filing status
- **Estate tax** -- exemption amount and rate schedule
- **NIIT** -- net investment income tax thresholds
- **Payroll rates** -- Social Security wage base, Medicare rates, Additional Medicare
- **QBI thresholds** -- qualified business income deduction phase-in ranges
- **RMD tables** -- Uniform Lifetime, Joint Life, Single Life Expectancy
- **IRMAA brackets** -- Medicare Part B/D income-related surcharges
- **SS taxation thresholds** -- Social Security benefit taxation brackets
- **417(e) mortality tables** -- pension lump sum mortality factors

## Designed for Agents

entropyfa is designed as a tool for AI agents doing financial planning:

- **`--schema` on every command** -- agents read the schema to know what inputs to gather from the user, why to use a command, and what related commands exist
- **`data coverage`** -- agents discover what reference data is available without hardcoding keys
- **JSON-in/JSON-out** -- structured I/O that agents parse natively
- **Deterministic** -- same input always produces the same output, so agents can reason about results
- **No configuration** -- install and go, no API keys, no config files, no network calls

Works with any agent framework — Claude tool use, OpenAI function calling, LangChain, or plain shell exec.

## Architecture

```
--json '<JSON>' --> entropyfa CLI --> entropyfa-engine --> stdout (JSON)
```

- **Zero network calls** -- all data is compiled into the binary
- **Sub-millisecond** -- pure computation, no I/O overhead
- **Single binary** -- no runtime dependencies
- **Monthly releases** -- updated when IRS publishes new data

The project is a Cargo workspace with two crates:

| Crate | Purpose |
|-------|---------|
| `engine` | Embedded IRS reference data + computation logic (usable as a Rust library) |
| `cli` | CLI that accepts JSON via `--json` flag, assembles compute requests with embedded data, and writes JSON to stdout |

## Disclaimer

entropyfa is a computation tool, not financial advice. Tax laws are complex and change frequently. Always verify results against primary IRS sources and consult a qualified tax professional before making financial decisions. The authors and contributors accept no liability for decisions made based on this tool's output.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
