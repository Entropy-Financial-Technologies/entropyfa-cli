# Monte Carlo Cash And Tax Realism Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Tighten the realism of `entropyfa compute projection` by reusing the cash-flow taxonomy, separating gross inflows and outflows from net funding need, making tax assumptions explicit by year, and distinguishing discretionary withdrawals, RMD withdrawals, and tax-payment withdrawals.

**Architecture:** Keep the current bucketed Monte Carlo and linear simulation architecture, but stop treating household cash flows as a single net monthly number for reporting and tax accumulation. Extend the request contract with taxonomy-aligned flow metadata, normalize legacy requests into the richer internal model, feed both linear and Monte Carlo through typed monthly flow events, and enrich period detail rows with gross cash movement and tax provenance instead of only aggregate balances and a single `annual_tax_paid` number.

**Tech Stack:** Rust, `serde`, existing simulation modules in `engine/src/compute/simulation`, embedded federal tax data, CLI JSON schema/docs, `cargo test`.

---

## File Structure

### Existing files to modify

- `engine/src/models/simulation_request.rs`
  - Extend `CashFlow` with taxonomy-aligned fields and destination-bucket support while preserving legacy request compatibility.
- `engine/src/models/simulation_response.rs`
  - Expand period detail rows with gross inflow/outflow, tax provenance, and split withdrawal maps.
- `engine/src/validation/mod.rs`
  - Validate new cash-flow fields, category values, and savings contribution rules.
- `engine/src/compute/simulation/normalized.rs`
  - Normalize legacy cash flows into typed flow events with inferred defaults.
- `engine/src/compute/simulation/path_simulator.rs`
  - Stop resolving to `Vec<f64>` only; add typed monthly flow expansion helpers.
- `engine/src/compute/simulation/linear.rs`
  - Use typed monthly flow accounting and emit richer period detail rows.
- `engine/src/compute/simulation/monte_carlo.rs`
  - Mirror the typed-flow and tax-provenance logic in stochastic paths.
- `engine/src/compute/simulation/tax.rs`
  - Accumulate external taxable inflows, preserve modeled-vs-authoritative metadata, and expose iterative settlement detail.
- `engine/src/compute/simulation/rmd.rs`
  - Keep existing RMD logic but surface its withdrawals as a distinct class in downstream detail rows.
- `cli/src/schema/simulation.rs`
  - Document the new flow fields, compatibility defaults, and richer output contract.
- `cli/tests/integration.rs`
  - Add end-to-end coverage for taxonomy-aligned flows, savings contributions, and tax provenance.
- `README.md`
  - Update the projection example so users see the richer cash-flow format.

### New files to create

- `engine/src/compute/simulation/flow_events.rs`
  - Typed monthly flow structs plus helpers to expand request-time flow definitions into per-month classified events.

### Existing files to read before touching code

- `docs/superpowers/specs/2026-03-18-bucketed-monte-carlo-design.md`
- `engine/src/models/simulation_request.rs`
- `engine/src/models/simulation_response.rs`
- `engine/src/compute/simulation/normalized.rs`
- `engine/src/compute/simulation/path_simulator.rs`
- `engine/src/compute/simulation/linear.rs`
- `engine/src/compute/simulation/monte_carlo.rs`
- `engine/src/compute/simulation/tax.rs`
- `engine/src/compute/simulation/rmd.rs`
- `cli/src/schema/simulation.rs`

## Implementation Notes

- Reuse the cash-flow report taxonomy rather than inventing a separate projection vocabulary.
- Keep request compatibility: old flows with only `amount` and `frequency` must still work.
- Use `direction: inflow | outflow` as the primary movement field.
- Treat `category: savings` as a true portfolio contribution only when `destination_bucket_id` is present; otherwise it remains a normal outflow.
- Do not add new high-level domain concepts like Social Security claiming or pension policies in this pass.
- Keep the current `realized_gain_ratio` heuristic for taxable buckets, but expose the realized-gain amount in period detail.
- Preserve the existing top-level Monte Carlo/linear response layout; this is a realism pass, not a response-family rewrite.
- Favor small commits after each task.

## Taxonomy Contract

Use these request-side flow fields:

```rust
pub struct CashFlow {
    pub amount: f64,
    pub frequency: String,
    pub start_month: Option<u32>,
    pub end_month: Option<u32>,
    pub direction: Option<String>,          // inflow | outflow
    pub category: Option<String>,           // earned_income | portfolio_income | other_income | taxes | housing | living | savings | charitable | debt | insurance | education | transportation | other
    pub cash_character: Option<String>,     // cash | non_cash
    pub tax_treatment: Option<String>,      // ordinary_income | capital_gain | non_taxable | tax_free | ignore
    pub label: Option<String>,
    pub destination_bucket_id: Option<String>,
}
```

Legacy inference rules:

- Positive `amount` with no `direction` becomes `inflow`.
- Negative `amount` with no `direction` becomes `outflow`.
- Missing `category` becomes `other_income` for positive flows and `other` for negative flows.
- Missing `cash_character` becomes `cash`.
- Missing `tax_treatment` becomes `ignore`.

## Task 1: Extend The Public Request Contract For Taxonomy-Aligned Flows

**Files:**
- Modify: `engine/src/models/simulation_request.rs`
- Modify: `engine/src/validation/mod.rs`
- Modify: `engine/src/compute/simulation/normalized.rs`
- Test: `engine/src/validation/mod.rs`
- Test: `engine/src/compute/simulation/normalized.rs`

- [ ] **Step 1: Write failing validation tests for the new flow contract**

```rust
#[test]
fn test_projection_accepts_taxonomy_aligned_cash_flow() {
    let mut req = valid_bucketed_request();
    req.cash_flows = vec![CashFlow {
        amount: 3_000.0,
        frequency: "monthly".into(),
        start_month: Some(0),
        end_month: None,
        direction: Some("inflow".into()),
        category: Some("other_income".into()),
        cash_character: Some("cash".into()),
        tax_treatment: Some("non_taxable".into()),
        label: Some("Social Security".into()),
        destination_bucket_id: None,
    }];

    let errors = validate_simulation_request(&req);
    assert!(errors.is_empty(), "expected no errors, got: {errors:?}");
}

#[test]
fn test_projection_rejects_invalid_cash_flow_direction_and_category() {
    let mut req = valid_bucketed_request();
    req.cash_flows = vec![CashFlow {
        amount: 1_000.0,
        frequency: "monthly".into(),
        start_month: Some(0),
        end_month: None,
        direction: Some("sideways".into()),
        category: Some("vacation".into()),
        cash_character: Some("cash".into()),
        tax_treatment: Some("ordinary_income".into()),
        label: None,
        destination_bucket_id: None,
    }];

    let errors = validate_simulation_request(&req);
    assert!(errors.iter().any(|e| e.contains("direction")));
    assert!(errors.iter().any(|e| e.contains("category")));
}

#[test]
fn test_projection_rejects_savings_flow_destination_that_does_not_exist() {
    let mut req = valid_bucketed_request();
    req.cash_flows = vec![CashFlow {
        amount: -500.0,
        frequency: "monthly".into(),
        start_month: Some(0),
        end_month: None,
        direction: Some("outflow".into()),
        category: Some("savings".into()),
        cash_character: Some("cash".into()),
        tax_treatment: Some("ignore".into()),
        label: Some("IRA Contribution".into()),
        destination_bucket_id: Some("missing".into()),
    }];

    let errors = validate_simulation_request(&req);
    assert!(errors.iter().any(|e| e.contains("destination_bucket_id")));
}
```

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
cargo test -p entropyfa-engine test_projection_accepts_taxonomy_aligned_cash_flow -- --exact
cargo test -p entropyfa-engine test_projection_rejects_invalid_cash_flow_direction_and_category -- --exact
cargo test -p entropyfa-engine test_projection_rejects_savings_flow_destination_that_does_not_exist -- --exact
```

Expected: FAIL because the current `CashFlow` contract does not have these fields or validations.

- [ ] **Step 3: Extend `CashFlow` in the public request model**

Add the new optional fields in `engine/src/models/simulation_request.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlow {
    pub amount: f64,
    pub frequency: String,
    pub start_month: Option<u32>,
    pub end_month: Option<u32>,
    pub direction: Option<String>,
    pub category: Option<String>,
    pub cash_character: Option<String>,
    pub tax_treatment: Option<String>,
    pub label: Option<String>,
    pub destination_bucket_id: Option<String>,
}
```

- [ ] **Step 4: Normalize legacy flows into typed defaults**

Update `engine/src/compute/simulation/normalized.rs` so `NormalizedCashFlow` includes:

```rust
pub struct NormalizedCashFlow {
    pub amount: f64,
    pub frequency: String,
    pub start_month: Option<u32>,
    pub end_month: Option<u32>,
    pub direction: String,
    pub category: String,
    pub cash_character: String,
    pub tax_treatment: String,
    pub label: Option<String>,
    pub destination_bucket_id: Option<String>,
}
```

Implement default inference rules exactly as documented above.

- [ ] **Step 5: Add validation for the new cash-flow fields**

Extend `validate_simulation_request_contract` in `engine/src/validation/mod.rs` to enforce:

- `direction` is one of `inflow`, `outflow` when present
- `category` is one of the allowed taxonomy values when present
- `cash_character` is `cash` or `non_cash` when present
- `tax_treatment` is one of `ordinary_income`, `capital_gain`, `non_taxable`, `tax_free`, `ignore` when present
- `destination_bucket_id` must reference a known bucket when present
- if `destination_bucket_id` is present, the flow must be `direction = outflow` and `category = savings`

- [ ] **Step 6: Run the focused tests again**

Run the three commands from Step 2.

Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add engine/src/models/simulation_request.rs \
        engine/src/validation/mod.rs \
        engine/src/compute/simulation/normalized.rs
git commit -m "feat: add taxonomy-aligned projection cash flows"
```

## Task 2: Build Typed Monthly Flow Expansion And Accounting Helpers

**Files:**
- Create: `engine/src/compute/simulation/flow_events.rs`
- Modify: `engine/src/compute/simulation/mod.rs`
- Modify: `engine/src/compute/simulation/path_simulator.rs`
- Modify: `engine/src/compute/simulation/tax.rs`
- Test: `engine/src/compute/simulation/flow_events.rs`
- Test: `engine/src/compute/simulation/tax.rs`

- [ ] **Step 1: Write failing tests for typed monthly flow expansion**

```rust
#[test]
fn test_expand_monthly_flows_preserves_gross_inflow_and_outflow_events() {
    let flows = vec![
        NormalizedCashFlow {
            amount: 3_000.0,
            frequency: "monthly".into(),
            start_month: Some(0),
            end_month: None,
            direction: "inflow".into(),
            category: "other_income".into(),
            cash_character: "cash".into(),
            tax_treatment: "non_taxable".into(),
            label: Some("Social Security".into()),
            destination_bucket_id: None,
        },
        NormalizedCashFlow {
            amount: -6_500.0,
            frequency: "monthly".into(),
            start_month: Some(0),
            end_month: None,
            direction: "outflow".into(),
            category: "living".into(),
            cash_character: "cash".into(),
            tax_treatment: "ignore".into(),
            label: Some("Core Spending".into()),
            destination_bucket_id: None,
        },
    ];

    let events = expand_monthly_flow_events(&flows, 1);
    assert_eq!(events[0].len(), 2);
    assert_eq!(events[0][0].direction, FlowDirection::Inflow);
    assert_eq!(events[0][1].direction, FlowDirection::Outflow);
}

#[test]
fn test_external_taxable_inflow_accumulates_ordinary_income() {
    let mut agg = AnnualTaxAccumulator::default();
    agg.record_external_flow(3_000.0, "ordinary_income");
    assert_eq!(agg.ordinary_income, 3_000.0);
}
```

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
cargo test -p entropyfa-engine test_expand_monthly_flows_preserves_gross_inflow_and_outflow_events -- --exact
cargo test -p entropyfa-engine test_external_taxable_inflow_accumulates_ordinary_income -- --exact
```

Expected: FAIL because the helper module and accumulator method do not exist.

- [ ] **Step 3: Add a typed monthly flow module**

Create `engine/src/compute/simulation/flow_events.rs` with:

```rust
pub enum FlowDirection {
    Inflow,
    Outflow,
}

pub struct MonthlyFlowEvent {
    pub amount: f64,
    pub direction: FlowDirection,
    pub category: String,
    pub cash_character: String,
    pub tax_treatment: String,
    pub label: Option<String>,
    pub destination_bucket_id: Option<String>,
}

pub fn expand_monthly_flow_events(
    flows: &[NormalizedCashFlow],
    total_months: u32,
) -> Vec<Vec<MonthlyFlowEvent>> {
    // Expand the existing frequency logic into typed monthly events.
}
```

- [ ] **Step 4: Extend the annual tax accumulator to accept external flows**

Add explicit external-flow recording helpers in `engine/src/compute/simulation/tax.rs`:

```rust
impl AnnualTaxAccumulator {
    pub fn record_external_flow(&mut self, amount: f64, tax_treatment: &str) {
        match tax_treatment {
            "ordinary_income" => self.ordinary_income += amount.max(0.0),
            "capital_gain" => self.realized_capital_gain += amount.max(0.0),
            _ => {}
        }
    }
}
```

- [ ] **Step 5: Keep the old net-cash-flow helper only as a derived convenience**

Refactor `engine/src/compute/simulation/path_simulator.rs` so it can still produce a net monthly vector if needed, but only from the typed event expansion. Do not leave two independent frequency-expansion implementations in the codebase.

- [ ] **Step 6: Run the focused tests again**

Run the commands from Step 2.

Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add engine/src/compute/simulation/flow_events.rs \
        engine/src/compute/simulation/mod.rs \
        engine/src/compute/simulation/path_simulator.rs \
        engine/src/compute/simulation/tax.rs
git commit -m "feat: add typed projection flow accounting"
```

## Task 3: Upgrade Linear Projection Detail To Show Realistic Cash And Tax Accounting

**Files:**
- Modify: `engine/src/models/simulation_response.rs`
- Modify: `engine/src/compute/simulation/linear.rs`
- Modify: `engine/src/compute/simulation/rmd.rs`
- Test: `engine/src/compute/simulation/linear.rs`

- [ ] **Step 1: Write failing linear-detail tests for gross flows and split withdrawals**

```rust
#[test]
fn test_linear_detail_reports_gross_flows_instead_of_only_net_withdrawal() {
    let req = realistic_bucketed_request();
    let result = run_linear(&req);
    let detail = result.annual_detail.expect("detail should be present");

    assert_eq!(detail[0].gross_inflows, 3_000.0);
    assert_eq!(detail[0].gross_outflows, 6_500.0);
    assert_eq!(detail[0].withdrawals, 3_500.0);
}

#[test]
fn test_linear_detail_splits_rmd_and_tax_payment_withdrawals() {
    let req = rmd_enabled_bucketed_request_with_tax();
    let result = run_linear(&req);
    let detail = result.annual_detail.expect("detail should be present");

    assert!(detail[11].rmd_withdrawals_by_bucket["ira"] > 0.0);
    assert!(detail[11].tax_payment_withdrawals_by_bucket["taxable"] > 0.0);
}

#[test]
fn test_linear_detail_reports_tax_year_and_data_mode() {
    let req = rmd_enabled_bucketed_request_with_tax();
    let result = run_linear(&req);
    let detail = result.annual_detail.expect("detail should be present");

    assert_eq!(detail[11].tax_year, Some(2026));
    assert_eq!(detail[11].tax_data_mode.as_deref(), Some("authoritative"));
}
```

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
cargo test -p entropyfa-engine test_linear_detail_reports_gross_flows_instead_of_only_net_withdrawal -- --exact
cargo test -p entropyfa-engine test_linear_detail_splits_rmd_and_tax_payment_withdrawals -- --exact
cargo test -p entropyfa-engine test_linear_detail_reports_tax_year_and_data_mode -- --exact
```

Expected: FAIL because the current `PeriodDetail` shape does not include these fields.

- [ ] **Step 3: Expand `PeriodDetail` with realistic accounting fields**

Add the following fields in `engine/src/models/simulation_response.rs`:

```rust
pub struct PeriodDetail {
    // existing fields...
    pub gross_inflows: f64,
    pub gross_outflows: f64,
    pub net_external_cash_flow: f64,
    pub taxable_ordinary_income: f64,
    pub taxable_capital_gains: f64,
    pub tax_year: Option<u32>,
    pub tax_data_mode: Option<String>,
    pub modeled_from_year: Option<u32>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub discretionary_withdrawals_by_bucket: BTreeMap<String, f64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub rmd_withdrawals_by_bucket: BTreeMap<String, f64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub tax_payment_withdrawals_by_bucket: BTreeMap<String, f64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub contributions_by_bucket: BTreeMap<String, f64>,
}
```

Keep `bucket_withdrawals` for backward compatibility, but populate it as the combined total of the three withdrawal maps.

- [ ] **Step 4: Thread typed flow accounting through `run_linear`**

Update `engine/src/compute/simulation/linear.rs` so each month:

- processes expanded monthly flow events instead of only a net cash amount
- records gross inflows and gross outflows separately
- routes `category = savings` + `destination_bucket_id` into `contributions_by_bucket`
- records external taxable flows into `AnnualTaxAccumulator`
- tracks three withdrawal maps:
  - discretionary funding withdrawals
  - RMD withdrawals
  - tax-payment withdrawals

- [ ] **Step 5: Return tax provenance from settlement**

Extend `AnnualTaxSettlement` in `engine/src/compute/simulation/tax.rs` to expose:

```rust
pub struct AnnualTaxResult {
    pub total_tax: f64,
    pub taxable_ordinary_income: f64,
    pub realized_capital_gain: f64,
    pub data_mode: String,
    pub authoritative_year: u32,
}
```

Use those fields when building year-end `PeriodDetail`.

- [ ] **Step 6: Run the focused tests again**

Run the commands from Step 2.

Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add engine/src/models/simulation_response.rs \
        engine/src/compute/simulation/linear.rs \
        engine/src/compute/simulation/rmd.rs \
        engine/src/compute/simulation/tax.rs
git commit -m "feat: add realistic linear cash and tax detail"
```

## Task 4: Upgrade Monte Carlo Detail And Summary To Mirror The Realistic Accounting Model

**Files:**
- Modify: `engine/src/compute/simulation/monte_carlo.rs`
- Modify: `engine/src/models/simulation_response.rs`
- Modify: `engine/src/compute/simulation/statistics.rs`
- Test: `engine/src/compute/simulation/monte_carlo.rs`

- [ ] **Step 1: Write failing Monte Carlo tests for tax provenance and gross flow reporting**

```rust
#[test]
fn test_mc_annual_detail_reports_gross_flow_metrics() {
    let req = realistic_bucketed_request();
    let result = run_monte_carlo(&req);
    let detail = result.annual_detail.expect("detail should be present");

    assert_eq!(detail[0].gross_inflows, 3_000.0);
    assert_eq!(detail[0].gross_outflows, 6_500.0);
    assert_eq!(detail[0].net_cash_flow, -3_500.0);
}

#[test]
fn test_mc_annual_detail_reports_tax_metadata_for_modeled_years() {
    let req = realistic_bucketed_request();
    let result = run_monte_carlo(&req);
    let detail = result.annual_detail.expect("detail should be present");

    assert_eq!(detail[11].tax_data_mode.as_deref(), Some("authoritative"));
    assert_eq!(detail[23].tax_data_mode.as_deref(), Some("modeled"));
}
```

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
cargo test -p entropyfa-engine test_mc_annual_detail_reports_gross_flow_metrics -- --exact
cargo test -p entropyfa-engine test_mc_annual_detail_reports_tax_metadata_for_modeled_years -- --exact
```

Expected: FAIL because `MonteCarloDetailRow` does not yet include the realism fields.

- [ ] **Step 3: Expand Monte Carlo annual detail with matching realism fields**

Add these fields in `engine/src/models/simulation_response.rs`:

```rust
pub struct MonteCarloDetailRow {
    // existing fields...
    pub gross_inflows: f64,
    pub gross_outflows: f64,
    pub taxable_ordinary_income: f64,
    pub taxable_capital_gains: f64,
    pub tax_year: Option<u32>,
    pub tax_data_mode: Option<String>,
    pub modeled_from_year: Option<u32>,
}
```

For Monte Carlo annual detail, report mean values across paths for the new scalar metrics. Do not try to add bucket-level withdrawal maps to the Monte Carlo annual detail in this pass.

- [ ] **Step 4: Thread typed flow accounting through Monte Carlo path simulation**

Update `engine/src/compute/simulation/monte_carlo.rs` so each simulated path:

- consumes typed monthly flow events
- accumulates gross inflows/outflows separately from net portfolio funding need
- records external taxable flows into the annual tax accumulator
- captures tax settlement metadata for the period detail rollup

- [ ] **Step 5: Run the focused tests again**

Run the commands from Step 2.

Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add engine/src/compute/simulation/monte_carlo.rs \
        engine/src/models/simulation_response.rs \
        engine/src/compute/simulation/statistics.rs
git commit -m "feat: add realistic monte carlo cash and tax detail"
```

## Task 5: Update CLI Schema, Examples, And Integration Coverage

**Files:**
- Modify: `cli/src/schema/simulation.rs`
- Modify: `cli/tests/integration.rs`
- Modify: `README.md`

- [ ] **Step 1: Write failing integration tests for the new request/response contract**

```rust
#[test]
fn compute_projection_schema_mentions_direction_category_and_tax_treatment() {
    let output = run_cli(&["compute", "projection", "--schema"]);
    let body = parse_success_json(&output.stdout);

    let cash_flows = &body["data"]["input_schema"]["properties"]["cash_flows"]["items"]["properties"];
    assert!(cash_flows.get("direction").is_some());
    assert!(cash_flows.get("category").is_some());
    assert!(cash_flows.get("tax_treatment").is_some());
    assert!(cash_flows.get("destination_bucket_id").is_some());
}

#[test]
fn compute_projection_bucketed_realistic_request_reports_tax_provenance() {
    let output = run_cli(&[
        "compute", "projection", "--json",
        "{\"mode\":\"linear\",\"time_horizon_months\":12,\"buckets\":[{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":50000,\"return_assumption\":{\"annual_mean\":0.0,\"annual_std_dev\":0.0}}],\"filing_status\":\"single\",\"spending_policy\":{\"withdrawal_order\":[\"ira\"],\"rebalance_tax_withholding_from\":\"ira\"},\"tax_policy\":{\"mode\":\"embedded_federal\",\"modeled_tax_inflation_rate\":0.025},\"cash_flows\":[{\"amount\":-4000,\"frequency\":\"monthly\",\"direction\":\"outflow\",\"category\":\"living\",\"cash_character\":\"cash\",\"tax_treatment\":\"ignore\"}],\"include_detail\":true,\"detail_granularity\":\"annual\"}"
    ]);
    let body = parse_success_json(&output.stdout);

    assert_eq!(body["data"]["linear"]["annual_detail"][0]["tax_year"], 2026);
    assert_eq!(body["data"]["linear"]["annual_detail"][0]["tax_data_mode"], "authoritative");
}
```

- [ ] **Step 2: Run the focused integration tests to verify they fail**

Run:

```bash
cargo test -p entropyfa --test integration compute_projection_schema_mentions_direction_category_and_tax_treatment -- --exact
cargo test -p entropyfa --test integration compute_projection_bucketed_realistic_request_reports_tax_provenance -- --exact
```

Expected: FAIL because the schema and response do not yet expose these fields.

- [ ] **Step 3: Update the CLI schema and examples**

Update `cli/src/schema/simulation.rs` so it:

- documents the new cash-flow fields
- explains legacy inference behavior
- states that `category = savings` plus `destination_bucket_id` creates a contribution instead of a pure expense
- explains `tax_data_mode` as `authoritative` for 2026 and `modeled` after supported years

- [ ] **Step 4: Add one README example using the richer request**

Add a short `compute projection` example to `README.md` using:

- one inflow categorized as `other_income`
- one outflow categorized as `living`
- one savings outflow with `destination_bucket_id`

- [ ] **Step 5: Run the focused integration tests again**

Run the commands from Step 2.

Expected: PASS

- [ ] **Step 6: Run the full verification suite**

Run:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo run -p entropyfa -- compute projection --json "$(cat /tmp/entropyfa_projection_realistic.json)"
```

Expected:

- formatting check passes
- workspace tests pass
- the realistic projection command returns `ok: true`

- [ ] **Step 7: Commit**

```bash
git add cli/src/schema/simulation.rs \
        cli/tests/integration.rs \
        README.md
git commit -m "docs: clarify realistic projection cash and tax contract"
```

## Task 6: Final Reality Check Against The Audited Scenario

**Files:**
- No code changes expected unless a bug is found
- Test: `/tmp/entropyfa_projection_realistic.json`

- [ ] **Step 1: Run the audited realistic scenario again**

Run:

```bash
cargo run -p entropyfa -- compute projection --json "$(cat /tmp/entropyfa_projection_realistic.json)" > /tmp/entropyfa_projection_realistic_output.json
```

Expected: `ok: true`

- [ ] **Step 2: Verify the key realism invariants manually**

Check with `jq`:

```bash
jq '.data.linear.annual_detail[0] | {gross_inflows, gross_outflows, withdrawals, bucket_withdrawals, discretionary_withdrawals_by_bucket, contributions_by_bucket}' /tmp/entropyfa_projection_realistic_output.json
jq '.data.linear.annual_detail[11] | {tax_year, tax_data_mode, rmd_withdrawals_by_bucket, tax_payment_withdrawals_by_bucket, taxable_ordinary_income, taxable_capital_gains, annual_tax_paid}' /tmp/entropyfa_projection_realistic_output.json
jq '.data.linear.annual_detail[23] | {tax_year, tax_data_mode, modeled_from_year, rmd_withdrawals_by_bucket, tax_payment_withdrawals_by_bucket, taxable_ordinary_income, taxable_capital_gains, annual_tax_paid}' /tmp/entropyfa_projection_realistic_output.json
```

Expected:

- period 0 shows `gross_inflows = 3000` and `gross_outflows = 6500`
- period 11 shows `tax_year = 2026` and `tax_data_mode = "authoritative"`
- period 23 shows `tax_year = 2027` and `tax_data_mode = "modeled"`
- RMD and tax-payment withdrawals are separated from discretionary withdrawals

- [ ] **Step 3: Commit only if fixes were needed**

If no fixes were needed, do not create a no-op commit.
