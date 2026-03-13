# Changelog

## [Unreleased]

## [0.1.0] - 2026-03-12

### Initial public release
- Compute commands use `--json '<JSON>'` as the canonical input interface
- Standardized command names: `roth-conversion`, `roth-conversion-strategy`, `pension-comparison`, `projection`, `goal-solver`
- `data lookup` and `data coverage` commands for verified reference data
- Federal income tax computation (`compute federal-tax`)
- Estate tax computation (`compute estate-tax`)
- RMD calculation and projection (`compute rmd`, `compute rmd-schedule`)
- Roth conversion analysis (`compute roth-conversion`, `compute roth-conversion-strategy`)
- Pension lump sum vs annuity comparison (`compute pension-comparison`)
- Monte Carlo projection (`compute projection`)
- Binary search goal solver (`compute goal-solver`)
- `--schema` flag for agent guidance on every compute command
- Terminal Monte Carlo dashboard rendered to stderr
- Optional `--result-hook-url` for posting result envelopes to a webhook
- Embedded 2026 reference data: tax brackets, standard deductions, capital gains brackets, estate tax, NIIT, payroll rates, QBI thresholds, RMD tables, IRMAA brackets, SS taxation thresholds, 417(e) mortality tables
- Cross-platform binaries: macOS (Intel + Apple Silicon), Linux (x86_64 + ARM64)
