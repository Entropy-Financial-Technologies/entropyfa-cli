# Reference Packs Federal Tax Slice Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first repo-reset vertical slice by introducing markdown tax reference packs, switching `compute federal-tax` to agent-supplied assumption bundles, and repointing the pipeline apply path to write markdown packs for tax entries.

**Architecture:** Keep the engine’s federal-tax math intact and move the seam up one layer. The agent will assemble `tax_parameters` explicitly, the CLI will validate and compute, and the pipeline will write reviewed markdown packs as the new agent-facing reference artifact while legacy JSON artifacts remain temporarily for non-migrated commands.

**Tech Stack:** Rust, `serde`, `serde_json`, `serde_yaml` (new), current CLI schema system, current `data-pipeline` workflow, markdown reference files under a new `reference/` tree, `cargo test`.

---

## Scope

This plan intentionally implements only the first slice from the approved design:

- tax reference packs in markdown
- `compute federal-tax` as the first agent-assembled calculator
- pipeline support to write those markdown packs
- docs/schema updates for the new flow

This plan does **not** fully remove `data lookup`, migrate non-tax calculators, or delete the old reviewed JSON registry yet.

## File Map

### Create

- `reference/tax/2025/federal_income_tax_brackets.md`
- `reference/tax/2026/federal_income_tax_brackets.md`
- `reference/tax/2026/federal_standard_deductions.md`
- `reference/tax/2026/federal_capital_gains_brackets.md`
- `reference/tax/2026/federal_capital_loss_limit.md`
- `reference/tax/2026/federal_net_investment_income_tax.md`
- `reference/tax/2026/federal_payroll_tax_parameters.md`
- `reference/tax/2026/federal_salt_deduction_parameters.md`
- `docs/reference-packs.md`
- `engine/src/reference_packs/mod.rs`
- `engine/src/reference_packs/markdown.rs`
- `engine/src/reference_packs/types.rs`
- `engine/src/reference_packs/tax.rs`
- `engine/tests/reference_packs.rs`

### Modify

- `engine/Cargo.toml`
- `engine/src/lib.rs`
- `engine/src/models/tax_response.rs`
- `engine/src/validation/mod.rs`
- `engine/src/bin/data-pipeline.rs`
- `engine/src/data_pipeline.rs`
- `engine/src/data_pipeline/workflow.rs`
- `cli/src/assembler/tax.rs`
- `cli/src/commands/data.rs`
- `cli/src/schema/tax.rs`
- `cli/tests/integration.rs`
- `README.md`
- `docs/README.md`
- `docs/data-pipeline.md`
- `docs/embedded-data.md`

## Design Constraints

- Do not redesign the federal-tax math in `engine/src/compute/tax/federal.rs`.
- Do not require runtime markdown lookup inside the calculator path. The calculator should consume explicit `tax_parameters`.
- Keep legacy JSON reviewed artifacts temporarily so non-migrated commands still work.
- Make markdown reference packs the new agent-facing canonical artifact for this slice.
- Keep the pipeline’s primary/verifier structured submissions JSON-based for now; only repoint the reviewed output artifact and docs in this slice.

## Task 1: Add Reference Pack Parsing And Validation Infrastructure

**Files:**
- Create: `engine/src/reference_packs/mod.rs`
- Create: `engine/src/reference_packs/markdown.rs`
- Create: `engine/src/reference_packs/types.rs`
- Create: `engine/src/reference_packs/tax.rs`
- Create: `engine/tests/reference_packs.rs`
- Modify: `engine/Cargo.toml`
- Modify: `engine/src/lib.rs`

- [ ] **Step 1: Write the failing reference-pack tests**

Create `engine/tests/reference_packs.rs` with focused tests:

```rust
#[test]
fn parses_tax_reference_pack_with_front_matter_and_body() {
    let pack = entropyfa_engine::reference_packs::load_markdown_pack(
        "reference/tax/2026/federal_income_tax_brackets.md"
    ).unwrap();

    assert_eq!(pack.category, "tax");
    assert_eq!(pack.key, "federal_income_tax_brackets");
    assert_eq!(pack.year, 2026);
    assert!(pack.body.contains("Federal Income Tax Brackets"));
}

#[test]
fn rejects_pack_missing_required_metadata() {
    let markdown = "---\ncategory: tax\n---\n# Broken\n";
    let err = entropyfa_engine::reference_packs::parse_markdown_pack(markdown).unwrap_err();
    assert!(err.to_string().contains("missing required field"));
}
```

- [ ] **Step 2: Run the tests to verify they fail**

Run:

```bash
cargo test -p entropyfa-engine --test reference_packs
```

Expected: FAIL because `reference_packs` does not exist yet.

- [ ] **Step 3: Add the minimal parsing dependency**

Add to `engine/Cargo.toml`:

```toml
serde_yaml = "0.9"
```

- [ ] **Step 4: Implement the minimal reference-pack types and parser**

Create:

- `engine/src/reference_packs/types.rs` for the parsed metadata/body structs
- `engine/src/reference_packs/markdown.rs` for front-matter parsing
- `engine/src/reference_packs/mod.rs` to expose:
  - `parse_markdown_pack`
  - `load_markdown_pack`

Use one rigid format only for v1:

- YAML front matter
- markdown body after the closing `---`

- [ ] **Step 5: Export the module**

Add `pub mod reference_packs;` to `engine/src/lib.rs`.

- [ ] **Step 6: Re-run the reference-pack tests**

Run:

```bash
cargo test -p entropyfa-engine --test reference_packs
```

Expected: PASS for the parser tests, with file-loading tests still failing because the packs do not exist yet.

- [ ] **Step 7: Commit**

```bash
git add engine/Cargo.toml \
  engine/src/lib.rs \
  engine/src/reference_packs/mod.rs \
  engine/src/reference_packs/markdown.rs \
  engine/src/reference_packs/types.rs \
  engine/tests/reference_packs.rs
git commit -m "feat: add markdown reference pack parser"
```

## Task 2: Add Tax Reference Packs And Typed Extraction

**Files:**
- Create: `reference/tax/2025/federal_income_tax_brackets.md`
- Create: `reference/tax/2026/federal_income_tax_brackets.md`
- Create: `reference/tax/2026/federal_standard_deductions.md`
- Create: `reference/tax/2026/federal_capital_gains_brackets.md`
- Create: `reference/tax/2026/federal_capital_loss_limit.md`
- Create: `reference/tax/2026/federal_net_investment_income_tax.md`
- Create: `reference/tax/2026/federal_payroll_tax_parameters.md`
- Create: `reference/tax/2026/federal_salt_deduction_parameters.md`
- Create: `engine/src/reference_packs/tax.rs`
- Modify: `engine/tests/reference_packs.rs`

- [ ] **Step 1: Extend the failing tests to load typed tax parameters**

Add tests such as:

```rust
#[test]
fn loads_2026_single_tax_parameter_bundle_from_reference_packs() {
    let params = entropyfa_engine::reference_packs::tax::load_federal_tax_parameters(
        2026,
        "single"
    ).unwrap();

    assert_eq!(params.standard_deduction, 16100.0);
    assert_eq!(params.ordinary_brackets.len(), 7);
    assert_eq!(params.capital_gains_brackets.len(), 3);
}

#[test]
fn all_tax_reference_packs_parse_cleanly() {
    for path in [
        "reference/tax/2025/federal_income_tax_brackets.md",
        "reference/tax/2026/federal_income_tax_brackets.md",
        "reference/tax/2026/federal_standard_deductions.md",
        "reference/tax/2026/federal_capital_gains_brackets.md",
        "reference/tax/2026/federal_capital_loss_limit.md",
        "reference/tax/2026/federal_net_investment_income_tax.md",
        "reference/tax/2026/federal_payroll_tax_parameters.md",
        "reference/tax/2026/federal_salt_deduction_parameters.md",
    ] {
        entropyfa_engine::reference_packs::load_markdown_pack(path).unwrap();
    }
}
```

- [ ] **Step 2: Run the tests to verify they fail**

Run:

```bash
cargo test -p entropyfa-engine --test reference_packs
```

Expected: FAIL because the reference pack files and tax extraction helpers do not exist yet.

- [ ] **Step 3: Create the markdown tax reference packs**

Each markdown pack must include:

- YAML front matter with:
  - `category`
  - `key`
  - `year`
  - `required_params`
  - `verification_status`
  - `review_status`
  - `sources`
  - `data`
- short markdown primer below

Source the numeric values from the existing reviewed JSON artifacts under:

- `engine/data_registry/2025/reviewed/tax/`
- `engine/data_registry/2026/reviewed/tax/`

- [ ] **Step 4: Implement typed tax extraction helpers**

Create `engine/src/reference_packs/tax.rs` with helpers that turn the markdown `data` block into existing tax-request model types:

- `TaxBracket`
- `NiitParams`
- `PayrollParams`
- `SaltDeductionParams`
- a helper that assembles a `TaxParameters` bundle for one year + filing status

Keep the extraction narrow to the datasets used by `federal-tax`.

- [ ] **Step 5: Re-run the reference-pack tests**

Run:

```bash
cargo test -p entropyfa-engine --test reference_packs
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add reference/tax \
  engine/src/reference_packs/tax.rs \
  engine/tests/reference_packs.rs
git commit -m "feat: add markdown tax reference packs"
```

## Task 3: Switch `compute federal-tax` To Explicit Assumption Bundles

**Files:**
- Modify: `cli/src/assembler/tax.rs`
- Modify: `cli/src/schema/tax.rs`
- Modify: `engine/src/models/tax_response.rs`
- Modify: `engine/src/validation/mod.rs`
- Modify: `cli/tests/integration.rs`

- [ ] **Step 1: Write the failing CLI integration tests**

Add tests to `cli/tests/integration.rs`:

```rust
#[test]
fn compute_federal_tax_schema_exposes_required_inputs_supported_overrides_and_reference_packs() {
    let v = run_ok(entropyfa().args(["compute", "federal-tax", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(v["data"]["required_inputs"].is_array());
    assert!(v["data"]["supported_overrides"].is_array());
    assert!(v["data"]["recommended_reference_packs"].is_array());
}

#[test]
fn compute_federal_tax_requires_explicit_tax_parameters() {
    let v = run_err(entropyfa().args([
        "compute",
        "federal-tax",
        "--json",
        "{\"filing_status\":\"single\",\"income\":{\"wages\":150000}}",
    ]));
    assert_eq!(v["error"]["code"], "assembly_error");
    assert!(v["error"]["message"].as_str().unwrap().contains("tax_parameters"));
}

#[test]
fn compute_federal_tax_accepts_explicit_tax_parameters_bundle() {
    let v = run_ok(entropyfa().args([
        "compute",
        "federal-tax",
        "--json",
        include_str!(\"../fixtures/federal_tax_explicit_params.json\").trim(),
    ]));
    assert_eq!(v["ok"], true);
    assert!(v["data"]["assumptions_used"].is_object());
}
```

Use an inline JSON string if you prefer to avoid a fixture file.

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
cargo test -p entropyfa --test integration compute_federal_tax_schema_exposes_required_inputs_supported_overrides_and_reference_packs -- --exact
cargo test -p entropyfa --test integration compute_federal_tax_requires_explicit_tax_parameters -- --exact
cargo test -p entropyfa --test integration compute_federal_tax_accepts_explicit_tax_parameters_bundle -- --exact
```

Expected: FAIL because the current schema is still lookup-oriented and the assembler still auto-loads tax parameters.

- [ ] **Step 3: Remove hidden tax-parameter auto-loading from the assembler**

In `cli/src/assembler/tax.rs`:

- stop calling `build_tax_params` from `assemble_federal_tax`
- deserialize `tax_parameters` from input explicitly
- keep default parsing for `income`, `adjustments`, and `deductions`
- return a clear `assembly_error` when `tax_parameters` is missing

- [ ] **Step 4: Update the federal-tax schema to the new contract style**

In `cli/src/schema/tax.rs`, replace the current lookup-oriented shape with a compute-contract shape that includes:

- `required_inputs`
- `optional_inputs`
- `supported_overrides`
- `output_contract`
- `recommended_reference_packs`

Keep the top-level `command`, `description`, `when_to_use`, `example`, and `related_commands`.

The schema should explicitly say:

- the agent is expected to supply `tax_parameters`
- recommended reference packs are under `reference/tax/<year>/...`
- `tax_year` is informational for run context, not the hidden source of assumptions

- [ ] **Step 5: Echo the assumption bundle used in the response**

Extend `FederalTaxResponse` with a small `assumptions_used` block that includes:

- `tax_year`
- `filing_status`
- `tax_parameters`

Populate it in `engine/src/compute/tax/federal.rs` without changing the tax math.

- [ ] **Step 6: Add or tighten validation**

In `engine/src/validation/mod.rs`, ensure malformed explicit `tax_parameters` still produce clear validation errors rather than cryptic compute failures.

- [ ] **Step 7: Run the focused tests**

Run:

```bash
cargo test -p entropyfa --test integration compute_federal_tax_schema_exposes_required_inputs_supported_overrides_and_reference_packs -- --exact
cargo test -p entropyfa --test integration compute_federal_tax_requires_explicit_tax_parameters -- --exact
cargo test -p entropyfa --test integration compute_federal_tax_accepts_explicit_tax_parameters_bundle -- --exact
```

Expected: PASS.

- [ ] **Step 8: Run the full federal-tax integration slice**

Run:

```bash
cargo test -p entropyfa --test integration compute_federal_tax_
```

Expected: PASS for all `compute_federal_tax_*` tests.

- [ ] **Step 9: Commit**

```bash
git add cli/src/assembler/tax.rs \
  cli/src/schema/tax.rs \
  cli/tests/integration.rs \
  engine/src/models/tax_response.rs \
  engine/src/validation/mod.rs
git commit -m "feat: switch federal tax to explicit assumptions"
```

## Task 4: Repoint Pipeline Apply To Write Markdown Tax Reference Packs

**Files:**
- Modify: `engine/src/data_pipeline.rs`
- Modify: `engine/src/data_pipeline/workflow.rs`
- Modify: `engine/src/bin/data-pipeline.rs`
- Modify: `engine/tests/data_pipeline_workflow.rs`

- [ ] **Step 1: Write the failing pipeline workflow test**

Add a workflow test that applies a reviewed tax run and asserts:

- the markdown reference pack is written under `reference/tax/<year>/<key>.md`
- the markdown front matter contains the approved value and accepted sources
- the existing reviewed JSON artifact is still present for now

Example assertion shape:

```rust
#[test]
fn apply_run_writes_markdown_reference_pack_for_tax_entry() {
    let outcome = apply_run_at(&engine_root, &run_id).unwrap();
    let pack_path = workspace_root.join("reference/tax/2026/federal_income_tax_brackets.md");
    let contents = std::fs::read_to_string(pack_path).unwrap();
    assert!(contents.contains("category: tax"));
    assert!(contents.contains("key: federal_income_tax_brackets"));
    assert!(contents.contains("Revenue Procedure"));
}
```

- [ ] **Step 2: Run the focused workflow test and verify it fails**

Run:

```bash
cargo test -p entropyfa-engine apply_run_writes_markdown_reference_pack_for_tax_entry -- --exact
```

Expected: FAIL because apply currently writes only reviewed JSON + generated Rust outputs.

- [ ] **Step 3: Implement markdown-pack rendering for tax entries**

In `engine/src/data_pipeline/workflow.rs`:

- add a small renderer that turns `ReviewedArtifact` into the new markdown pack format
- write that pack under `reference/tax/<year>/<key>.md` during `apply`
- keep JSON reviewed artifact writes intact for this slice

In `engine/src/bin/data-pipeline.rs`, update user-facing output so `apply` mentions the markdown reference-pack path.

- [ ] **Step 4: Re-run the focused workflow test**

Run:

```bash
cargo test -p entropyfa-engine apply_run_writes_markdown_reference_pack_for_tax_entry -- --exact
```

Expected: PASS.

- [ ] **Step 5: Run the data-pipeline workflow suite**

Run:

```bash
cargo test -p entropyfa-engine --test data_pipeline_workflow
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add engine/src/data_pipeline.rs \
  engine/src/data_pipeline/workflow.rs \
  engine/src/bin/data-pipeline.rs \
  engine/tests/data_pipeline_workflow.rs
git commit -m "feat: write markdown tax reference packs from pipeline"
```

## Task 5: Rewrite Docs Around The First Slice

**Files:**
- Create: `docs/reference-packs.md`
- Modify: `README.md`
- Modify: `docs/README.md`
- Modify: `docs/data-pipeline.md`
- Modify: `docs/embedded-data.md`

- [ ] **Step 1: Write the failing doc-contract integration test**

Add one CLI integration smoke test asserting the new federal-tax schema mentions:

- `recommended_reference_packs`
- explicit `tax_parameters`

If you already covered that in Task 3, reuse the same test and do not duplicate coverage.

- [ ] **Step 2: Create `docs/reference-packs.md`**

Document:

- the markdown pack format
- how agents should use packs
- the first shipped tax pack set
- how packs relate to calculator schemas

- [ ] **Step 3: Update README**

Shift messaging from:

- lookup-first + compute

to:

- reference packs + deterministic compute

Keep `data lookup` documented as transitional for now, not central.

- [ ] **Step 4: Update `docs/data-pipeline.md`**

Document that for tax entries in this slice:

- `apply` writes markdown reference packs
- reviewed JSON still exists temporarily for non-migrated consumers

- [ ] **Step 5: Update `docs/embedded-data.md`**

Add a prominent note that the repo is moving toward markdown reference packs as the agent-facing surface, and that this doc is transitional while the full migration completes.

- [ ] **Step 6: Run docs-adjacent verification**

Run:

```bash
cargo test -p entropyfa --test integration compute_federal_tax_schema_exposes_required_inputs_supported_overrides_and_reference_packs -- --exact
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add README.md \
  docs/reference-packs.md \
  docs/README.md \
  docs/data-pipeline.md \
  docs/embedded-data.md
git commit -m "docs: document reference pack workflow"
```

## Task 6: Final Verification

**Files:**
- No new files; verify the complete slice

- [ ] **Step 1: Run formatting**

```bash
cargo fmt --all -- --check
```

Expected: PASS.

- [ ] **Step 2: Run engine reference-pack tests**

```bash
cargo test -p entropyfa-engine --test reference_packs
```

Expected: PASS.

- [ ] **Step 3: Run CLI integration tests**

```bash
cargo test -p entropyfa --test integration
```

Expected: PASS.

- [ ] **Step 4: Run data-pipeline workflow tests**

```bash
cargo test -p entropyfa-engine --test data_pipeline_workflow
```

Expected: PASS.

- [ ] **Step 5: Run full workspace verification**

```bash
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 6: Run live smoke checks**

Schema:

```bash
cargo run -p entropyfa -- compute federal-tax --schema
```

Explicit tax-parameter run:

```bash
cargo run -p entropyfa -- compute federal-tax --json '{
  "filing_status":"single",
  "tax_year":2026,
  "income":{"wages":150000},
  "tax_parameters":{
    "ordinary_brackets":[
      {"min":0.0,"max":12400.0,"rate":0.10},
      {"min":12400.0,"max":50400.0,"rate":0.12},
      {"min":50400.0,"max":105700.0,"rate":0.22},
      {"min":105700.0,"max":201775.0,"rate":0.24},
      {"min":201775.0,"max":256225.0,"rate":0.32},
      {"min":256225.0,"max":640600.0,"rate":0.35},
      {"min":640600.0,"max":null,"rate":0.37}
    ],
    "capital_gains_brackets":[
      {"min":0.0,"max":49450.0,"rate":0.0},
      {"min":49450.0,"max":545500.0,"rate":0.15},
      {"min":545500.0,"max":null,"rate":0.20}
    ],
    "standard_deduction":16100.0,
    "capital_loss_limit":3000.0,
    "niit":{"rate":0.038,"threshold":200000.0},
    "payroll":{
      "social_security_rate":0.062,
      "social_security_wage_base":176100.0,
      "self_employment_tax_rate":0.124,
      "medicare_rate":0.0145,
      "self_employment_medicare_rate":0.029,
      "additional_medicare_rate":0.009,
      "additional_medicare_threshold":200000.0
    },
    "salt":{
      "cap_amount":40400.0,
      "phaseout_threshold":505000.0,
      "phaseout_rate":0.3,
      "floor_amount":10000.0
    }
  }
}'
```

Pipeline smoke:

```bash
cargo run -p entropyfa-engine --bin data-pipeline -- status --plain
```

- [ ] **Step 7: Commit any final formatting/test-only adjustments**

```bash
git add -A
git commit -m "chore: finalize federal tax reference pack slice"
```

## Follow-On Plans

After this slice ships, create separate follow-on plans for:

1. migrating the remaining tax/estate/RMD calculators to explicit assumption bundles
2. repointing the rest of the pipeline outputs to markdown packs
3. deprecating or removing `data lookup`
4. migrating Monte Carlo / retirement calculators to the same contract style

