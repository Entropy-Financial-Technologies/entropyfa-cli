# Bucketed Monte Carlo Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade `entropyfa compute projection` from a pooled-balance Monte Carlo into a bucketed household simulator with withdrawal order, embedded federal tax handling, and optional RMD support while preserving the existing command surface.

**Architecture:** Keep the public `compute projection` command and evolve its request shape in a backward-compatible way. Add a normalized internal household simulation model, then route both linear and Monte Carlo execution through shared bucket-aware monthly path logic. Layer annual federal tax and RMD handling on top of that path engine, then enrich the existing response family with bucket-aware detail instead of introducing the shelved projection-platform envelope.

**Tech Stack:** Rust, `serde`, `rayon`, `rand`/`rand_chacha`, existing `entropyfa_engine::compute::tax::federal` logic, embedded retirement distribution data, CLI JSON schema/docs, `cargo test`.

---

## File Structure

### Existing files to modify

- `engine/src/models/simulation_request.rs`
  - Expand the public request contract to support household bucket inputs and richer cash-flow typing.
- `engine/src/models/simulation_response.rs`
  - Add bucket-aware outputs and annual tax / depletion diagnostics while preserving the current response family.
- `engine/src/validation/mod.rs`
  - Validate the new bucket, policy, and tax inputs.
- `engine/src/compute/simulation/linear.rs`
  - Move linear execution onto the shared bucket path engine.
- `engine/src/compute/simulation/monte_carlo.rs`
  - Move Monte Carlo execution onto the shared bucket path engine.
- `engine/src/compute/simulation/statistics.rs`
  - Compute bucket-aware Monte Carlo summary stats and richer annual detail.
- `engine/src/compute/simulation/mod.rs`
  - Export the new simulation helper modules.
- `cli/src/schema/simulation.rs`
  - Document the new request shape, compatibility behavior, and output additions.
- `cli/src/commands/simulation.rs`
  - Keep flag wiring intact while allowing the richer request/response contract.
- `cli/tests/integration.rs`
  - Add end-to-end CLI coverage for the new request shape and compatibility mode.
- `README.md`
  - Update product copy and examples for the upgraded projection command.
- `docs/compute-visuals.md`
  - Clarify what the visual dashboard does and does not reflect after bucket/tax upgrades.

### New files to create

- `engine/src/compute/simulation/normalized.rs`
  - Internal normalized request model plus legacy-request adapter.
- `engine/src/compute/simulation/buckets.rs`
  - Shared bucket state structs and monthly step logic.
- `engine/src/compute/simulation/withdrawals.rs`
  - Withdrawal-order and liquidation helpers.
- `engine/src/compute/simulation/tax.rs`
  - Annual household tax aggregation and embedded-federal tax execution helpers.
- `engine/src/compute/simulation/rmd.rs`
  - RMD eligibility and annual forced-distribution helpers for `tax_deferred` buckets.

### Existing files to read before touching code

- `engine/src/models/simulation_request.rs`
- `engine/src/models/simulation_response.rs`
- `engine/src/compute/simulation/path_simulator.rs`
- `engine/src/compute/simulation/linear.rs`
- `engine/src/compute/simulation/monte_carlo.rs`
- `engine/src/compute/simulation/statistics.rs`
- `engine/src/compute/tax/federal.rs`
- `engine/src/compute/retirement/rmd.rs`
- `cli/src/schema/simulation.rs`
- `docs/superpowers/specs/2026-03-18-bucketed-monte-carlo-design.md`

## Implementation Notes

- Preserve backward compatibility at the command level by treating the old `starting_balance` + `return_assumption` request as a one-bucket synthetic `taxable` portfolio during normalization.
- Do not try to add Medicare/IRMAA or state tax in this pass.
- Keep TDD discipline: test first, fail first, then implement the minimum.
- Favor small commits after each task.

## Task 1: Add the Normalized Household Simulation Contract

**Files:**
- Create: `engine/src/compute/simulation/normalized.rs`
- Modify: `engine/src/models/simulation_request.rs`
- Modify: `engine/src/compute/simulation/mod.rs`
- Modify: `engine/src/validation/mod.rs`
- Test: `engine/src/compute/simulation/normalized.rs`

- [ ] **Step 1: Write the failing normalization tests**

```rust
#[test]
fn test_normalize_legacy_request_into_single_taxable_bucket() {
    let req = SimulationRequest {
        mode: Some("both".into()),
        num_simulations: Some(100),
        seed: Some(1),
        starting_balance: Some(100_000.0),
        buckets: vec![],
        time_horizon_months: 12,
        return_assumption: Some(ReturnAssumption {
            annual_mean: 0.06,
            annual_std_dev: 0.10,
        }),
        cash_flows: vec![],
        filing_status: None,
        household: None,
        spending_policy: None,
        tax_policy: None,
        rmd_policy: None,
        include_detail: false,
        detail_granularity: "annual".into(),
        sample_paths: None,
        path_indices: None,
        custom_percentiles: None,
    };

    let normalized = normalize_request(&req).expect("legacy request should normalize");
    assert_eq!(normalized.buckets.len(), 1);
    assert_eq!(normalized.buckets[0].bucket_type, BucketType::Taxable);
}

#[test]
fn test_normalize_bucketed_request_requires_withdrawal_order_when_multiple_buckets() {
    let req = bucketed_request_without_withdrawal_order();
    let err = normalize_request(&req).unwrap_err();
    assert!(err.iter().any(|e| e.contains("withdrawal_order")));
}
```

- [ ] **Step 2: Run the new tests to verify they fail**

Run: `cargo test -p entropyfa-engine test_normalize_legacy_request_into_single_taxable_bucket test_normalize_bucketed_request_requires_withdrawal_order_when_multiple_buckets`

Expected: FAIL because `normalize_request`, `BucketType`, and the new request fields do not exist yet.

- [ ] **Step 3: Expand the public request model**

Add the minimal new request structs in `engine/src/models/simulation_request.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationBucket {
    pub id: String,
    pub bucket_type: String,
    pub starting_balance: f64,
    pub return_assumption: ReturnAssumption,
    pub realized_gain_ratio: Option<f64>,
    pub withdrawal_priority: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseholdConfig {
    pub birth_years: Option<Vec<u32>>,
    pub retirement_month: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingPolicy {
    pub withdrawal_order: Vec<String>,
    pub rebalance_tax_withholding_from: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxPolicy {
    pub mode: String,
    pub modeled_tax_inflation_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RmdPolicy {
    pub enabled: bool,
    pub distribution_month: Option<u32>,
}
```

Also make legacy fields optional:

```rust
pub starting_balance: Option<f64>,
pub return_assumption: Option<ReturnAssumption>,
#[serde(default)]
pub buckets: Vec<SimulationBucket>,
pub filing_status: Option<String>,
pub household: Option<HouseholdConfig>,
pub spending_policy: Option<SpendingPolicy>,
pub tax_policy: Option<TaxPolicy>,
pub rmd_policy: Option<RmdPolicy>,
```

- [ ] **Step 4: Add the internal normalized model and adapter**

Create `engine/src/compute/simulation/normalized.rs` with:

```rust
pub enum BucketType {
    Taxable,
    TaxDeferred,
    TaxFree,
    Cash,
}

pub struct NormalizedSimulationRequest {
    pub num_simulations: u32,
    pub seed: Option<u64>,
    pub time_horizon_months: u32,
    pub filing_status: Option<String>,
    pub buckets: Vec<NormalizedBucket>,
    pub cash_flows: Vec<NormalizedCashFlow>,
    pub spending_policy: NormalizedSpendingPolicy,
    pub tax_policy: NormalizedTaxPolicy,
    pub rmd_policy: Option<NormalizedRmdPolicy>,
    pub include_detail: bool,
    pub detail_granularity: String,
    pub sample_paths: Option<usize>,
    pub path_indices: Option<Vec<usize>>,
    pub custom_percentiles: Option<Vec<u32>>,
}

pub fn normalize_request(req: &SimulationRequest) -> Result<NormalizedSimulationRequest, Vec<String>> {
    // Legacy path: synthesize a single taxable bucket from starting_balance + return_assumption.
    // Bucketed path: validate and convert explicit buckets.
}
```

- [ ] **Step 5: Add validation for the new contract**

Extend `validate_simulation_request` in `engine/src/validation/mod.rs` to enforce:

- legacy request needs both `starting_balance` and `return_assumption`
- bucketed request needs `buckets.len() > 0`
- no negative `starting_balance`
- allowed `bucket_type` values only
- `realized_gain_ratio` in `[0.0, 1.0]`
- `modeled_tax_inflation_rate >= 0.0`
- `distribution_month` in `1..=12`

- [ ] **Step 6: Run the focused tests**

Run: `cargo test -p entropyfa-engine test_normalize_legacy_request_into_single_taxable_bucket test_normalize_bucketed_request_requires_withdrawal_order_when_multiple_buckets`

Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add engine/src/models/simulation_request.rs \
        engine/src/compute/simulation/normalized.rs \
        engine/src/compute/simulation/mod.rs \
        engine/src/validation/mod.rs
git commit -m "feat: normalize bucketed simulation requests"
```

## Task 2: Build Shared Bucket State and Withdrawal Logic

**Files:**
- Create: `engine/src/compute/simulation/buckets.rs`
- Create: `engine/src/compute/simulation/withdrawals.rs`
- Modify: `engine/src/compute/simulation/mod.rs`
- Test: `engine/src/compute/simulation/buckets.rs`
- Test: `engine/src/compute/simulation/withdrawals.rs`

- [ ] **Step 1: Write the failing bucket-state tests**

```rust
#[test]
fn test_apply_monthly_returns_per_bucket() {
    let mut state = HouseholdBucketState::from_balances(vec![
        ("taxable", 100_000.0),
        ("ira", 200_000.0),
    ]);
    let returns = vec![0.01, 0.02];
    apply_monthly_returns(&mut state, &returns);
    assert_eq!(state.balance("taxable"), 101_000.0);
    assert_eq!(state.balance("ira"), 204_000.0);
}

#[test]
fn test_withdrawal_order_uses_taxable_before_tax_deferred() {
    let mut state = HouseholdBucketState::from_balances(vec![
        ("taxable", 25_000.0),
        ("ira", 50_000.0),
    ]);
    let result = fund_household_deficit(&mut state, 30_000.0, &["taxable", "ira"]);
    assert_eq!(result.withdrawn_from("taxable"), 25_000.0);
    assert_eq!(result.withdrawn_from("ira"), 5_000.0);
}
```

- [ ] **Step 2: Run the focused tests and verify they fail**

Run: `cargo test -p entropyfa-engine test_apply_monthly_returns_per_bucket test_withdrawal_order_uses_taxable_before_tax_deferred`

Expected: FAIL because `HouseholdBucketState` and `fund_household_deficit` do not exist.

- [ ] **Step 3: Implement bucket state**

Create `engine/src/compute/simulation/buckets.rs`:

```rust
#[derive(Debug, Clone)]
pub struct BucketState {
    pub id: String,
    pub bucket_type: BucketType,
    pub balance: f64,
    pub realized_gain_ratio: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct HouseholdBucketState {
    pub buckets: Vec<BucketState>,
    pub household_cash: f64,
}

pub fn apply_monthly_returns(state: &mut HouseholdBucketState, monthly_returns: &[f64]) {
    for (bucket, r) in state.buckets.iter_mut().zip(monthly_returns.iter()) {
        bucket.balance = (bucket.balance * (1.0 + r)).max(0.0);
    }
}
```

- [ ] **Step 4: Implement withdrawal-order helpers**

Create `engine/src/compute/simulation/withdrawals.rs`:

```rust
pub struct WithdrawalResult {
    pub funded: bool,
    pub remaining_shortfall: f64,
    pub withdrawals_by_bucket: BTreeMap<String, f64>,
}

pub fn fund_household_deficit(
    state: &mut HouseholdBucketState,
    amount_needed: f64,
    withdrawal_order: &[String],
) -> WithdrawalResult {
    // Walk bucket order, draining balances until amount_needed is funded or exhausted.
}
```

- [ ] **Step 5: Add depletion-focused tests**

```rust
#[test]
fn test_withdrawal_order_marks_unfunded_shortfall_when_all_buckets_exhausted() {
    let mut state = HouseholdBucketState::from_balances(vec![("taxable", 5_000.0)]);
    let result = fund_household_deficit(&mut state, 10_000.0, &["taxable"]);
    assert!(!result.funded);
    assert_eq!(result.remaining_shortfall, 5_000.0);
}
```

- [ ] **Step 6: Run the new unit tests**

Run: `cargo test -p entropyfa-engine test_apply_monthly_returns_per_bucket test_withdrawal_order_uses_taxable_before_tax_deferred test_withdrawal_order_marks_unfunded_shortfall_when_all_buckets_exhausted`

Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add engine/src/compute/simulation/buckets.rs \
        engine/src/compute/simulation/withdrawals.rs \
        engine/src/compute/simulation/mod.rs
git commit -m "feat: add bucket state and withdrawal order helpers"
```

## Task 3: Move Linear Projection onto the Bucket Engine

**Files:**
- Modify: `engine/src/compute/simulation/linear.rs`
- Modify: `engine/src/models/simulation_response.rs`
- Test: `engine/src/compute/simulation/linear.rs`

- [ ] **Step 1: Write the failing linear bucket tests**

```rust
#[test]
fn test_linear_bucketed_projection_reports_ending_balances_by_bucket() {
    let req = bucketed_linear_request();
    let result = run_linear(&req);
    let buckets = result.ending_balances_by_bucket.expect("bucket output should exist");
    assert!(buckets.get("taxable").unwrap() > &0.0);
    assert!(buckets.get("ira").unwrap() > &0.0);
}

#[test]
fn test_linear_withdrawal_order_depletes_taxable_before_ira() {
    let req = bucketed_linear_withdrawal_request();
    let result = run_linear(&req);
    let detail = result.annual_detail.expect("detail should exist");
    assert!(detail[0].bucket_withdrawals["taxable"] >= detail[0].bucket_withdrawals["ira"]);
}
```

- [ ] **Step 2: Run the targeted tests and verify they fail**

Run: `cargo test -p entropyfa-engine test_linear_bucketed_projection_reports_ending_balances_by_bucket test_linear_withdrawal_order_depletes_taxable_before_ira`

Expected: FAIL because the current linear runner only understands pooled balances.

- [ ] **Step 3: Extend the linear response structs minimally**

Modify `engine/src/models/simulation_response.rs`:

```rust
pub struct LinearResult {
    pub final_balance: f64,
    pub time_series: LinearTimeSeries,
    pub total_contributions: f64,
    pub total_withdrawals: f64,
    pub total_return_earned: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_balances_by_bucket: Option<BTreeMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_detail: Option<Vec<PeriodDetail>>,
}

pub struct PeriodDetail {
    // existing fields...
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub bucket_withdrawals: BTreeMap<String, f64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub ending_balances_by_bucket: BTreeMap<String, f64>,
}
```

- [ ] **Step 4: Rewrite `run_linear` to normalize the request and drive bucket state**

Pseudo-implementation sketch:

```rust
let normalized = normalize_request(req).expect("validated request should normalize");
let mut state = HouseholdBucketState::from_normalized(&normalized);

for month in 0..normalized.time_horizon_months {
    let monthly_returns = deterministic_monthly_returns_for_all_buckets(&normalized);
    apply_monthly_returns(&mut state, &monthly_returns);
    apply_scheduled_flows(&mut state, &normalized, month);
    fund_net_household_need(&mut state, &normalized.spending_policy);
    collect_period_detail(...);
}
```

- [ ] **Step 5: Run the focused linear tests**

Run: `cargo test -p entropyfa-engine test_linear_bucketed_projection_reports_ending_balances_by_bucket test_linear_withdrawal_order_depletes_taxable_before_ira`

Expected: PASS

- [ ] **Step 6: Run the existing linear test module**

Run: `cargo test -p entropyfa-engine linear::tests`

Expected: PASS, including old pooled-balance tests through compatibility normalization.

- [ ] **Step 7: Commit**

```bash
git add engine/src/compute/simulation/linear.rs \
        engine/src/models/simulation_response.rs
git commit -m "feat: move linear projection onto bucketed path engine"
```

## Task 4: Add Annual Household Federal Tax Handling

**Files:**
- Create: `engine/src/compute/simulation/tax.rs`
- Modify: `engine/src/compute/simulation/linear.rs`
- Modify: `engine/src/compute/simulation/monte_carlo.rs`
- Modify: `engine/src/models/simulation_response.rs`
- Test: `engine/src/compute/simulation/tax.rs`

- [ ] **Step 1: Write the failing annual-tax tests**

```rust
#[test]
fn test_tax_deferred_withdrawals_are_taxed_as_ordinary_income() {
    let mut agg = AnnualTaxAccumulator::default();
    agg.record_tax_deferred_withdrawal("ira", 40_000.0);
    let result = compute_annual_household_tax(&agg, "single", 2026, 0.025).unwrap();
    assert!(result.total_tax > 0.0);
}

#[test]
fn test_tax_free_withdrawals_do_not_increase_taxable_income() {
    let mut agg = AnnualTaxAccumulator::default();
    agg.record_tax_free_withdrawal("roth", 30_000.0);
    let result = compute_annual_household_tax(&agg, "single", 2026, 0.025).unwrap();
    assert_eq!(result.taxable_ordinary_income, 0.0);
}

#[test]
fn test_taxable_withdrawals_use_realized_gain_ratio() {
    let mut agg = AnnualTaxAccumulator::default();
    agg.record_taxable_withdrawal("taxable", 10_000.0, 0.60);
    let result = compute_annual_household_tax(&agg, "single", 2026, 0.025).unwrap();
    assert!(result.realized_capital_gain >= 5_999.0);
}
```

- [ ] **Step 2: Run the tax tests and verify they fail**

Run: `cargo test -p entropyfa-engine test_tax_deferred_withdrawals_are_taxed_as_ordinary_income test_tax_free_withdrawals_do_not_increase_taxable_income test_taxable_withdrawals_use_realized_gain_ratio`

Expected: FAIL because `AnnualTaxAccumulator` and `compute_annual_household_tax` do not exist.

- [ ] **Step 3: Implement the annual tax accumulator and executor**

Create `engine/src/compute/simulation/tax.rs`:

```rust
#[derive(Default, Debug, Clone)]
pub struct AnnualTaxAccumulator {
    pub ordinary_income: f64,
    pub realized_capital_gain: f64,
}

pub struct AnnualTaxResult {
    pub total_tax: f64,
    pub taxable_ordinary_income: f64,
    pub realized_capital_gain: f64,
    pub data_mode: String,
    pub authoritative_year: u32,
}

pub fn compute_annual_household_tax(
    agg: &AnnualTaxAccumulator,
    filing_status: &str,
    tax_year: u32,
    modeled_tax_inflation_rate: f64,
) -> Result<AnnualTaxResult, String> {
    // Call the existing embedded federal tax helpers.
}
```

- [ ] **Step 4: Charge annual tax from the configured source**

In both linear and Monte Carlo path execution:

- accumulate tax inputs during the year
- call `compute_annual_household_tax` at year-end
- withdraw the tax payment from the configured source bucket or `cash`
- record the payment in annual detail output

- [ ] **Step 5: Extend detail output for tax reporting**

Add response fields such as:

```rust
pub struct MonteCarloDetailRow {
    // existing percentile fields...
    pub annual_tax_paid: f64,
}

pub struct PeriodDetail {
    // existing fields...
    pub annual_tax_paid: f64,
}
```

- [ ] **Step 6: Run the tax-focused tests**

Run: `cargo test -p entropyfa-engine test_tax_deferred_withdrawals_are_taxed_as_ordinary_income test_tax_free_withdrawals_do_not_increase_taxable_income test_taxable_withdrawals_use_realized_gain_ratio`

Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add engine/src/compute/simulation/tax.rs \
        engine/src/compute/simulation/linear.rs \
        engine/src/compute/simulation/monte_carlo.rs \
        engine/src/models/simulation_response.rs
git commit -m "feat: add annual household tax handling to simulation"
```

## Task 5: Add RMD Support for Tax-Deferred Buckets

**Files:**
- Create: `engine/src/compute/simulation/rmd.rs`
- Modify: `engine/src/compute/simulation/linear.rs`
- Modify: `engine/src/compute/simulation/monte_carlo.rs`
- Test: `engine/src/compute/simulation/rmd.rs`

- [ ] **Step 1: Write the failing RMD tests**

```rust
#[test]
fn test_rmd_is_generated_for_tax_deferred_bucket_when_enabled() {
    let req = rmd_enabled_bucketed_request();
    let schedule = compute_household_rmd_for_year(&req, 2026).expect("RMD should compute");
    assert!(schedule.total_rmd > 0.0);
}

#[test]
fn test_rmd_forces_tax_deferred_withdrawal_before_normal_spending_order() {
    let req = rmd_enabled_bucketed_request();
    let result = run_linear(&req);
    let detail = result.annual_detail.expect("detail should exist");
    assert!(detail[0].bucket_withdrawals["ira"] > 0.0);
}
```

- [ ] **Step 2: Run the RMD tests and verify they fail**

Run: `cargo test -p entropyfa-engine test_rmd_is_generated_for_tax_deferred_bucket_when_enabled test_rmd_forces_tax_deferred_withdrawal_before_normal_spending_order`

Expected: FAIL because the simulation path does not yet compute RMDs.

- [ ] **Step 3: Implement the RMD helper using existing retirement data**

Create `engine/src/compute/simulation/rmd.rs`:

```rust
pub struct AnnualRmdResult {
    pub total_rmd: f64,
    pub withdrawals_by_bucket: BTreeMap<String, f64>,
}

pub fn compute_household_rmd_for_year(
    req: &NormalizedSimulationRequest,
    tax_year: u32,
    state: &HouseholdBucketState,
) -> Result<AnnualRmdResult, String> {
    // Reuse existing retirement distribution rule data and divisor logic.
}
```

- [ ] **Step 4: Integrate RMD into monthly execution**

At the configured distribution month:

- compute the annual RMD
- withdraw from eligible `tax_deferred` buckets before discretionary withdrawal logic
- record the withdrawal for tax aggregation and detail output

- [ ] **Step 5: Run the focused RMD tests**

Run: `cargo test -p entropyfa-engine test_rmd_is_generated_for_tax_deferred_bucket_when_enabled test_rmd_forces_tax_deferred_withdrawal_before_normal_spending_order`

Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add engine/src/compute/simulation/rmd.rs \
        engine/src/compute/simulation/linear.rs \
        engine/src/compute/simulation/monte_carlo.rs
git commit -m "feat: add rmd support to bucketed simulation"
```

## Task 6: Move Monte Carlo Execution onto the Bucket Engine

**Files:**
- Modify: `engine/src/compute/simulation/monte_carlo.rs`
- Modify: `engine/src/compute/simulation/statistics.rs`
- Modify: `engine/src/models/simulation_response.rs`
- Test: `engine/src/compute/simulation/monte_carlo.rs`
- Test: `engine/src/compute/simulation/statistics.rs`

- [ ] **Step 1: Write the failing Monte Carlo bucket tests**

```rust
#[test]
fn test_zero_volatility_bucketed_monte_carlo_matches_linear() {
    let req = zero_vol_bucketed_request();
    let mc = run_monte_carlo(&req);
    let linear = run_linear(&req);
    assert!((mc.mean - linear.final_balance).abs() < 1.0);
}

#[test]
fn test_bucketed_mc_reports_terminal_balances_by_bucket() {
    let req = bucketed_mc_request();
    let result = run_monte_carlo(&req);
    let bucket_p50 = result.bucket_terminal_percentiles.as_ref().unwrap();
    assert!(bucket_p50["taxable"].p50 >= 0.0);
}

#[test]
fn test_bucketed_mc_failure_tracks_depletion_months() {
    let req = stressed_bucketed_mc_request();
    let result = run_monte_carlo(&req);
    assert!(!result.paths_depleted_by_month.is_empty());
}
```

- [ ] **Step 2: Run the MC tests and verify they fail**

Run: `cargo test -p entropyfa-engine test_zero_volatility_bucketed_monte_carlo_matches_linear test_bucketed_mc_reports_terminal_balances_by_bucket test_bucketed_mc_failure_tracks_depletion_months`

Expected: FAIL because Monte Carlo still simulates one pooled path.

- [ ] **Step 3: Replace pooled path generation with bucketed path generation**

In `engine/src/compute/simulation/monte_carlo.rs`:

- normalize the request first
- generate per-bucket monthly return series
- simulate each path month-by-month using shared bucket logic
- store:
  - terminal total balance
  - terminal balances by bucket
  - annual detail rows
  - failure month if depleted

- [ ] **Step 4: Extend Monte Carlo statistics for bucket-aware summaries**

In `engine/src/compute/simulation/statistics.rs`, add:

```rust
pub struct BucketTerminalPercentiles {
    pub p10: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
}
```

and compute:

- bucket terminal percentiles
- bucket depletion counts
- annual median tax paid if detail requested

- [ ] **Step 5: Run the focused Monte Carlo tests**

Run: `cargo test -p entropyfa-engine test_zero_volatility_bucketed_monte_carlo_matches_linear test_bucketed_mc_reports_terminal_balances_by_bucket test_bucketed_mc_failure_tracks_depletion_months`

Expected: PASS

- [ ] **Step 6: Run the existing Monte Carlo test module**

Run: `cargo test -p entropyfa-engine monte_carlo::tests statistics::tests`

Expected: PASS, including legacy pooled-balance behavior through normalization.

- [ ] **Step 7: Commit**

```bash
git add engine/src/compute/simulation/monte_carlo.rs \
        engine/src/compute/simulation/statistics.rs \
        engine/src/models/simulation_response.rs
git commit -m "feat: move monte carlo projection onto bucketed path engine"
```

## Task 7: Update CLI Schema, Integration Tests, and Docs

**Files:**
- Modify: `cli/src/schema/simulation.rs`
- Modify: `cli/src/commands/simulation.rs`
- Modify: `cli/tests/integration.rs`
- Modify: `README.md`
- Modify: `docs/compute-visuals.md`

- [ ] **Step 1: Write the failing CLI integration tests**

```rust
#[test]
fn compute_projection_bucketed_request_returns_bucket_aware_output() {
    let output = run_entropyfa(&[
        "compute",
        "projection",
        "--json",
        r#"{
          "filing_status":"single",
          "time_horizon_months":24,
          "buckets":[
            {"id":"taxable","bucket_type":"taxable","starting_balance":100000,"return_assumption":{"annual_mean":0.05,"annual_std_dev":0.10}},
            {"id":"ira","bucket_type":"tax_deferred","starting_balance":200000,"return_assumption":{"annual_mean":0.05,"annual_std_dev":0.10}}
          ],
          "spending_policy":{"withdrawal_order":["taxable","ira"]},
          "tax_policy":{"mode":"embedded_federal","modeled_tax_inflation_rate":0.025}
        }"#
    ]);
    assert!(output.status.success());
    assert!(stdout_contains(&output, "\"bucket_terminal_percentiles\""));
}

#[test]
fn compute_projection_legacy_request_still_works() {
    let output = run_entropyfa(&[
        "compute",
        "projection",
        "--json",
        r#"{"starting_balance":100000,"time_horizon_months":12,"return_assumption":{"annual_mean":0.05,"annual_std_dev":0.10}}"#
    ]);
    assert!(output.status.success());
}
```

- [ ] **Step 2: Run the CLI integration tests and verify they fail**

Run: `cargo test -p entropyfa --test integration compute_projection_bucketed_request_returns_bucket_aware_output compute_projection_legacy_request_still_works`

Expected: FAIL because schema and response surfaces are not updated yet.

- [ ] **Step 3: Update schema and command docs**

In `cli/src/schema/simulation.rs`:

- describe `buckets`, `spending_policy`, `tax_policy`, and `rmd_policy`
- mark legacy `starting_balance` + `return_assumption` as still supported but compatibility-only
- document that tax is annual household federal tax using embedded data when available

In `cli/src/commands/simulation.rs`:

- keep existing flag overrides intact
- do not add new flags in this pass

- [ ] **Step 4: Update human-facing docs**

Modify `README.md` and `docs/compute-visuals.md` to show:

- one new bucketed JSON example
- one note on legacy compatibility
- one note that the terminal dashboard is still aggregate and does not yet render per-bucket charts

- [ ] **Step 5: Run the focused CLI integration tests**

Run: `cargo test -p entropyfa --test integration compute_projection_bucketed_request_returns_bucket_aware_output compute_projection_legacy_request_still_works`

Expected: PASS

- [ ] **Step 6: Run the full simulation-related test slice**

Run: `cargo test --workspace simulation`

Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add cli/src/schema/simulation.rs \
        cli/src/commands/simulation.rs \
        cli/tests/integration.rs \
        README.md \
        docs/compute-visuals.md
git commit -m "docs: expose bucketed monte carlo projection"
```

## Task 8: Final Verification and Smoke Runs

**Files:**
- No intended code changes; verification only

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all -- --check`

Expected: PASS

- [ ] **Step 2: Run the full workspace tests**

Run: `cargo test --workspace`

Expected: PASS

- [ ] **Step 3: Run a bucketed projection smoke test**

Run:

```bash
cargo run -p entropyfa -- compute projection --json '{
  "filing_status":"single",
  "time_horizon_months":24,
  "buckets":[
    {"id":"taxable","bucket_type":"taxable","starting_balance":100000,"return_assumption":{"annual_mean":0.05,"annual_std_dev":0.10},"realized_gain_ratio":0.60},
    {"id":"ira","bucket_type":"tax_deferred","starting_balance":200000,"return_assumption":{"annual_mean":0.05,"annual_std_dev":0.10}},
    {"id":"roth","bucket_type":"tax_free","starting_balance":50000,"return_assumption":{"annual_mean":0.05,"annual_std_dev":0.10}}
  ],
  "cash_flows":[
    {"type":"spending","amount":-4000,"frequency":"monthly","start_month":0}
  ],
  "spending_policy":{"withdrawal_order":["taxable","ira","roth"]},
  "tax_policy":{"mode":"embedded_federal","modeled_tax_inflation_rate":0.025},
  "mode":"both",
  "num_simulations":1000,
  "seed":1
}'
```

Expected: `ok: true` with bucket-aware outputs such as `ending_balances_by_bucket` and `bucket_terminal_percentiles`.

- [ ] **Step 4: Run a legacy-compatibility smoke test**

Run:

```bash
cargo run -p entropyfa -- compute projection --json '{
  "starting_balance":100000,
  "time_horizon_months":12,
  "return_assumption":{"annual_mean":0.05,"annual_std_dev":0.10},
  "mode":"both",
  "num_simulations":100,
  "seed":1
}'
```

Expected: `ok: true` and old-style request still accepted.

- [ ] **Step 5: Commit any final verification-only fixes**

```bash
git add -A
git commit -m "test: finalize bucketed monte carlo verification"
```

## Done Criteria

The implementation is complete when all of the following are true:

- `compute projection` accepts the new bucketed request shape
- legacy pooled requests still work through normalization
- linear and Monte Carlo both run on shared bucket-aware monthly logic
- withdrawal order materially affects depletion behavior
- annual household federal tax changes outcomes
- RMDs are supported for `tax_deferred` buckets
- CLI schema/docs reflect the new model clearly
- `cargo fmt --all -- --check` passes
- `cargo test --workspace` passes
