# entropyfa

[![CI](https://github.com/Entropy-Financial-Technologies/entropyfa-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/Entropy-Financial-Technologies/entropyfa-cli/actions/workflows/ci.yml)

Verified financial reference data + deterministic computation engine. Fully offline, sub-millisecond, JSON-in/JSON-out.

## 30-Second Demo

```sh
# What data is available?
entropyfa data coverage

# Look up 2026 tax brackets
entropyfa data lookup --category tax --key brackets --filing-status single

# Compute federal tax
echo '{"filing_status":"single","income":{"wages":150000}}' | entropyfa compute federal-tax

# What does this command need?
entropyfa compute roth --schema

# Run a Roth conversion analysis
echo '{"filing_status":"married_filing_jointly","income":{"wages":200000},"conversion_amount":50000}' \
  | entropyfa compute roth

# Monte Carlo retirement projection
echo '{"portfolio_value":1000000,"annual_return":0.07,"annual_volatility":0.15,"annual_withdrawal":40000,"years":30,"simulations":10000}' \
  | entropyfa compute simulate
```

## Install

**Quick install** (macOS / Linux):

```sh
curl -fsSL https://raw.githubusercontent.com/Entropy-Financial-Technologies/entropyfa-cli/main/install.sh | sh
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
| `compute roth` | Roth conversion tax impact analysis |
| `compute roth-strategy` | Multi-year Roth conversion strategy |
| `compute pension` | Pension lump sum vs annuity comparison |
| `compute simulate` | Monte Carlo / linear projection |
| `compute solve` | Binary search goal solver |

Every compute command supports `--schema` to emit the expected JSON input schema.

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

## Architecture

```
stdin (JSON) --> entropyfa CLI --> entropyfa-engine --> stdout (JSON)
```

- **Zero network calls** -- all data is compiled into the binary
- **Sub-millisecond** -- pure computation, no I/O overhead
- **Deterministic** -- same input always produces the same output
- **Single binary** -- no runtime dependencies, no config files

The project is a Cargo workspace with two crates:

| Crate | Purpose |
|-------|---------|
| `engine` | Reference data + computation logic |
| `cli` | Clap-based CLI that reads JSON from stdin and writes JSON to stdout |

## Disclaimer

entropyfa is a computation tool, not financial advice. Tax laws are complex and change frequently. Always verify results against primary IRS sources and consult a qualified tax professional before making financial decisions. The authors and contributors accept no liability for decisions made based on this tool's output.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
