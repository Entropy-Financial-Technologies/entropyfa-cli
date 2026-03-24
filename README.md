<p align="center">
  <img src="assets/logo.png" alt="entropyFA" width="480">
</p>

<p align="center">
  <a href="https://github.com/Entropy-Financial-Technologies/entropyfa-cli/actions/workflows/ci.yml"><img src="https://github.com/Entropy-Financial-Technologies/entropyfa-cli/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/Entropy-Financial-Technologies/entropyfa-cli/releases/latest"><img src="https://img.shields.io/github/v/release/Entropy-Financial-Technologies/entropyfa-cli?label=release&color=B8860B" alt="Release"></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-2B3643" alt="License"></a>
</p>

<p align="center">Personal finance and wealth planning engine for AI agents.<br>Deterministic tax, retirement, and estate calculations plus the canonical reference-root bundle scaffolding and embedded federal data — local by default, sub-ms, JSON-in/JSON-out.</p>

**Why?** Financial planning agents need two things they can't do well on their own: (1) verified reference material — rates, limits, rules, tables, and yearly pack context that change annually and must come from official sources, not hallucinated, and (2) deterministic calculations — tax bracket stacking, actuarial math, Monte Carlo simulations. entropyfa ships the compute layer plus the canonical reference-root scaffolding and any reviewed packs included in the current release, so agents can read local reference material from disk when broader context is available.

**Current scope:** Full reviewed 2026 federal reference data, plus reviewed 2025 federal ordinary income tax brackets for `data lookup`. Federal tax and estate calculations, SALT-aware itemized deduction support, retirement/RMD rules, Roth conversion analysis, pension comparison, Monte Carlo projection, and goal solving all currently default to 2026. State tax/reference data is not shipped yet.

## 30-Second Demo

```sh
# What data is available?
entropyfa data coverage

# Look up 2026 federal income tax brackets
entropyfa data lookup --category tax --key federal_income_tax_brackets --filing-status single

# Look up 2025 federal income tax brackets
entropyfa data lookup --category tax --key federal_income_tax_brackets --year 2025 --filing-status single

# Compute federal tax
entropyfa compute federal-tax --json '{"filing_status":"single","income":{"wages":150000}}'

# What does this command need?
entropyfa compute roth-conversion --schema

# Run a Roth conversion analysis
entropyfa compute roth-conversion --json '{"filing_status":"married_filing_jointly","income":{"wages":200000},"conversion_amount":50000}'

# Monte Carlo retirement projection
entropyfa compute projection --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'

# Bucketed household projection
entropyfa compute projection --json '{"buckets":[{"id":"taxable","bucket_type":"taxable","starting_balance":600000,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"realized_gain_ratio":0.25},{"id":"ira","bucket_type":"tax_deferred","starting_balance":400000,"return_assumption":{"annual_mean":0.06,"annual_std_dev":0.10}}],"spending_policy":{"withdrawal_order":["taxable","ira"]},"time_horizon_months":360,"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'

# Add a terminal dashboard when you actually want the visual
entropyfa compute projection --visual --json '{"starting_balance":1000000,"time_horizon_months":360,"return_assumption":{"annual_mean":0.07,"annual_std_dev":0.15},"cash_flows":[{"amount":-4000,"frequency":"monthly"}]}'

```

### Monte Carlo Projection Dashboard

<p align="center">
  <img src="assets/monte-carlo-chart.png" alt="Monte Carlo projection chart" width="800">
</p>

See [docs/compute-visuals.md](docs/compute-visuals.md) for how the projection dashboard works, why it is opt-in, and how it relates to the JSON output.

`compute projection` accepts both legacy aggregate inputs and bucketed household inputs. Bucketed requests can also supply `spending_policy`, `tax_policy`, and `rmd_policy`. The terminal dashboard remains aggregate-only for now, so it does not render per-bucket charts yet.

Annual household federal tax uses embedded data when available, then modeled behavior after supported years. `tax_policy.mode` must be `none`, `embedded_federal`, or `modeled`; `embedded_federal` and `modeled` require `filing_status`.

For bucketed runs, set `filing_status` when annual household tax matters, and set `household.birth_years` plus `household.retirement_month` when RMD behavior matters.

## Install

**Quick install** (macOS / Linux):

```sh
curl -fsSL https://get.entropyfa.com | sh
```

That default install behaves like `--profile full`:

- installs `entropyfa` to `~/.entropyfa/bin/entropyfa`
- installs the canonical reference root to `~/.entropyfa/reference/...` plus any reviewed packs included in the current release
- updates your shell profile if `~/.entropyfa/bin` is not already on `PATH`

Install profiles:

```sh
# Binary only
curl -fsSL https://get.entropyfa.com | sh -s -- --profile binary-only

# Full install (default)
curl -fsSL https://get.entropyfa.com | sh -s -- --profile full

# Platform/container-style install with explicit paths
curl -fsSL https://get.entropyfa.com | sh -s -- --profile platform \
  --install-dir /usr/local/bin \
  --reference-dir /opt/entropyfa/reference
```

- `binary-only` installs just the executable.
- `full` installs the executable plus the reference-root bundle and any reviewed packs included in the current release.
- `platform` installs the same full bundle but skips shell-profile edits and is intended for shared images or container-style layouts.

Existing `--system` still works and uses system defaults:

```sh
curl -fsSL https://get.entropyfa.com | sh -s -- --system
```

Reference-root resolution in the CLI is:

1. explicit `--reference-root`
2. `ENTROPYFA_REFERENCE_ROOT`
3. installer-written `entropyfa.install.json` metadata beside the active binary
4. runtime platform hint such as `ENTROPYFA_INSTALL_PROFILE=platform`, or platform auto-detection for binaries under `/opt/entropyfa/...` and managed `/usr/local/bin/entropyfa` installs
5. default local root at `~/.entropyfa/reference`

Default local installs use `~/.entropyfa/reference`. Explicit hints are the usual way to point at a non-default tree, while platform/container-style installs can also auto-detect `/opt/entropyfa/reference` when the binary lives under `/opt/entropyfa/...` or when a managed `/usr/local/bin/entropyfa` install is paired with `/opt/entropyfa/reference/.entropyfa-managed`.

Official installs also write `entropyfa.install.json` beside the binary, so custom `full` and `platform` layouts installed via `install.sh` can rediscover their reference root without extra env vars. Manual layouts outside the installer still need an explicit hint such as `ENTROPYFA_REFERENCE_ROOT=/path/to/reference`, `ENTROPYFA_INSTALL_PROFILE=platform`, or `--reference-root`.

To inspect the active binary path, version, and resolved reference metadata:

```sh
entropyfa env --json
```

Releases now publish three artifact types:

- `entropyfa-<target>.tar.gz` for binary-only installs
- `entropyfa-full-<target>.tar.gz` for the binary plus reference-root bundle
- `entropyfa-reference-packs-<tag>.tar.gz` for the reference-root bundle alone

Many compute commands can still run in standalone OSS installs without local packs when you pass explicit assumptions in the request JSON. Reference packs are for reviewed markdown context on disk and agent inspection, not a hard requirement for calculator execution.

**Cargo**:

```sh
cargo install --git https://github.com/Entropy-Financial-Technologies/entropyfa-cli.git entropyfa
```

This installs the binary only. It does not fetch the reference-root bundle or reviewed packs, so it is not feature-parity with the default full installer unless you install or point at a reference root separately.

**From source**:

```sh
git clone https://github.com/Entropy-Financial-Technologies/entropyfa-cli.git
cd entropyfa-cli
cargo build --release
mkdir -p ~/.entropyfa/bin
install -m 755 target/release/entropyfa ~/.entropyfa/bin/entropyfa
```

This also installs the binary only. To match the default full install, add the reference-root bundle or point the binary at an existing reference root after building.

## OpenClaw

The official OpenClaw skill is:

- package slug: `entropyfa`
- display name: `entropyFA Financial Planning`
- description: `Verified financial planning data and blazing-fast, deterministic calculators for Monte Carlo projection, goal solving, Roth conversions, RMDs, income tax, estate tax, and pension analysis.`

Install it into your current OpenClaw workspace with:

```sh
clawhub install entropyfa
```

See [docs/openclaw.md](docs/openclaw.md) for prerequisites, reference-root discovery, filesystem reads, example prompts, and trust guidance. The skill source lives in [integrations/openclaw/entropyfa](integrations/openclaw/entropyfa).

## Upgrade

```sh
entropyfa upgrade
```

This checks GitHub for the latest release and refreshes the right artifact for the detected install profile:

- binary-only installs update just the executable
- full/platform installs update the executable and the installed reference packs together

If your existing install lives in a system directory such as `/usr/local/bin` and is not writable, binary-only installs fall back to `~/.entropyfa/bin` instead of prompting for `sudo`. Full/platform installs fail closed in that situation so the binary and reference packs do not drift; rerun the installer or rebuild the platform image when you need to refresh a shared install. The CLI also checks for updates in the background — if a newer version is available, you'll see a reminder on stderr.

`entropyfa update` is supported as an alias for the same self-update flow.

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

All commands emit a JSON envelope to `stdout`. If `--result-hook-url` is set, entropyfa also POSTs the same envelope to your webhook endpoint as a best-effort side effect.

Tax-oriented compute flows accept either aggregate `deductions.itemized_amount` or detailed Schedule A-style inputs such as `deductions.state_local_income_or_sales_tax`, `deductions.real_property_tax`, `deductions.personal_property_tax`, and `deductions.other_itemized_deductions`.

### RMD Flow

`compute rmd` and `compute rmd-schedule` are the first schema-guided retirement calculators.

Normal agent flow:

1. Run `entropyfa compute rmd --schema`
2. Read the listed retirement reference packs under `reference/retirement/2026/`
3. Gather the required client facts such as `calculation_year`, `prior_year_end_balance`, `account_type`, and `owner_birth_date`
4. Run `entropyfa compute rmd --json '...'`

The normal path does not require inline `rmd_parameters`. The CLI loads the installed retirement reference packs and returns `references_used`, `assumptions_used`, and `overrides_used` in the response. Inline `rmd_parameters` remains available only as an explicit override path for nonstandard hypotheticals.

## Embedded Data Sources

Embedded reference data is compiled into the binary, and `data lookup` returns source URLs by default.

See [docs/embedded-data.md](docs/embedded-data.md) for every supported key, required params, and example lookup responses from the current reviewed dataset.

- **Tax brackets** -- federal income tax rates by filing status (Rev. Proc.)
- **Standard deductions** -- standard deduction amounts by filing status
- **SALT deduction parameters** -- federal Schedule A SALT cap, phaseout, and floor by filing status
- **Capital gains brackets** -- 0%/15%/20% thresholds by filing status
- **Estate tax** -- exemption amount and rate schedule
- **NIIT** -- net investment income tax thresholds
- **Payroll rates** -- Social Security wage base, Medicare rates, and Additional Medicare thresholds
- **QBI thresholds** -- qualified business income deduction phase-in ranges
- **RMD tables** -- Uniform Lifetime, Joint Life, Single Life Expectancy
- **IRMAA brackets** -- Medicare Part B/D income-related surcharges
- **Medicare base premiums** -- Medicare Part B standard premium, Part B deductible, and Part D base beneficiary premium
- **SS full retirement age rules** -- Social Security retirement and spousal FRA table by birth year
- **SS taxation thresholds** -- Social Security benefit taxation thresholds by filing status
- **417(e) mortality tables** -- Section 417(e) mortality rates for pension lump-sum work

## How Reference Data Is Verified

The embedded data is not hand-waved into the binary. Each yearly entry goes through a review pipeline before it becomes part of a release.

The current process is:

1. A primary agent pass gathers the official values, extracts the proposed payload, and cites exact source URLs and locators.
2. A separate verifier agent independently checks the same entry against the cited sources and flags disagreements, source-policy issues, or contract mismatches.
3. `run-agents` automatically reviews the run and auto-applies it only when the verifier outcome is clean, approved, and recommends applying the result.
4. Blocked or disputed runs stay at review, where maintainers can inspect the evidence packet and use manual `review` / `apply` commands as the escape hatch.
5. Approved runs are applied back into the reviewed artifact, markdown reference pack, generated Rust source, metadata, and snapshot.

In the current maintainer workflow, `run-agents` defaults to Claude `claude-opus-4-6` for the primary pass and Codex `gpt-5.4` for the verifier pass, but the important design choice is the independent two-pass review, not any one model name.

Accepted sources are controlled by per-entry source policy. For entries with a clear federal publisher, authoritative status requires an accepted official source on an approved host such as `irs.gov`, `cms.gov`, or `ssa.gov`. Supporting and secondary sources can corroborate the result, but they do not silently replace the primary official source requirement.

The lookup output reflects that provenance directly:

- `verification_status` shows the current trust status
- `pipeline_reviewed` shows whether the current embedded value came through the verification pipeline
- `sources` returns the source URLs and metadata used for the reviewed entry

## Documentation

The docs are split by audience:

- **User docs** — [docs/embedded-data.md](docs/embedded-data.md) for the embedded reference-data surface and example `data lookup` responses
- **User docs** — [docs/compute-visuals.md](docs/compute-visuals.md) for terminal dashboard behavior and chart-like compute output
- **User docs** — [docs/openclaw.md](docs/openclaw.md) for the official OpenClaw skill and install/usage guidance
- **Maintainer docs** — [docs/data-pipeline.md](docs/data-pipeline.md) for the contributor workflow that verifies, reviews, and applies yearly data updates
- **Maintainer docs** — [docs/roadmap.md](docs/roadmap.md) for the domain, product, and pipeline roadmap beyond the current federal tax/RMD surface
- **Docs index** — [docs/README.md](docs/README.md) for a simple entry point inside the `docs/` folder

## Feedback

For product feedback, bug reports, or questions:

- email `dan@entropyfa.com`
- message [@entropyfa](https://x.com/entropyfa) on X

## Designed for Agents

entropyfa is designed as a tool for AI agents doing financial planning:

- **`--schema` on every command** -- agents read the schema to know what inputs to gather from the user, why to use a command, and what related commands exist
- **`entropyfa env --json`** -- agents discover the installed binary path and resolved reference root before reading packs or running commands
- **Reference root on disk** -- agents can read any installed markdown files directly from the resolved reference root instead of treating the CLI as the only reference-data surface
- **JSON-in/JSON-out** -- structured I/O that agents parse natively
- **Human output on stderr** -- dashboards, warnings, and upgrade notices stay off the machine-readable stdout channel
- **Deterministic** -- same input always produces the same output, so agents can reason about results
- **No configuration** -- install and go, no API keys, no config files, and no outbound calls unless you opt into `upgrade` / `update` or `--result-hook-url`

Works with OpenClaw, Claude tool use, OpenAI function calling, LangChain, or plain shell exec.

## Architecture

```
--json '<JSON>' / flags --> entropyfa CLI --> entropyfa-engine --> stdout (JSON envelope)
                                         \
                                          --> stderr (dashboard / warnings)
                                         \
                                          --> optional webhook POST
```

- **Local by default** -- the compute layer runs locally, the installed reference root lives on disk, and embedded data still covers the CLI surfaces that ship inside the binary; outbound calls are opt-in
- **Sub-millisecond** -- pure computation, no I/O overhead
- **Single binary** -- no runtime dependencies
- **Monthly releases** -- updated when IRS publishes new data

The project is a Cargo workspace with two crates:

| Crate | Purpose |
|-------|---------|
| `engine` | Embedded IRS reference data where relevant + computation logic (usable as a Rust library) |
| `cli` | CLI that accepts JSON via `--json`, resolves the installed reference root, assembles compute requests, writes JSON to stdout, and can optionally POST result envelopes |

## Disclaimer

entropyfa is a computation tool, not financial advice. Tax laws are complex and change frequently. Always verify results against primary IRS sources and consult a qualified tax professional before making financial decisions. The authors and contributors accept no liability for decisions made based on this tool's output.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
