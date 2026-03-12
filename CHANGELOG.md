# Changelog

## [0.2.0] - 2026-03-12

### Breaking Changes
- **`--json` flag replaces stdin.** All compute commands now require `--json '<JSON>'` instead of piping JSON via stdin. `--schema` still works standalone.
- **5 commands renamed** for clarity:
  - `roth` → `roth-conversion`
  - `roth-strategy` → `roth-conversion-strategy`
  - `pension` → `pension-comparison`
  - `simulate` → `projection`
  - `solve` → `goal-solver`
- Unchanged: `federal-tax`, `estate-tax`, `rmd`, `rmd-schedule`

## [0.1.0] - 2026-03-12

### Added
- Initial release
- `data lookup` and `data coverage` commands for verified reference data
- Federal income tax computation (`compute federal-tax`)
- Estate tax computation (`compute estate-tax`)
- RMD calculation and projection (`compute rmd`, `compute rmd-schedule`)
- Roth conversion analysis (`compute roth`, `compute roth-strategy`)
- Pension lump sum vs annuity comparison (`compute pension`)
- Monte Carlo simulation (`compute simulate`)
- Binary search goal solver (`compute solve`)
- `--schema` flag for agent guidance on every compute command
- Embedded 2026 reference data: tax brackets, standard deductions, capital gains brackets, estate tax, NIIT, payroll rates, QBI thresholds, RMD tables, IRMAA brackets, SS taxation thresholds, 417(e) mortality tables
- Cross-platform binaries: macOS (Intel + Apple Silicon), Linux (x86_64 + ARM64)
