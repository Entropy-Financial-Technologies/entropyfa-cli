# RMD Schema-Guided Reference Pack Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor `compute rmd` into the first schema-guided calculator that loads shipped retirement reference packs locally, accepts client facts without inline `rmd_parameters`, and reports reference/override provenance in its response.

**Architecture:** Build this slice on top of the reference-pack installation foundation from `flexible-reference-pack-installation`, because `main` does not yet have `reference/` or reference-root resolution. Keep the RMD math in `entropyfa_engine`, add a thin CLI-side pack loader/assembler that resolves installed retirement packs, and update the request/response/schema contracts so the normal path is client-facts-plus-local-packs while explicit rule overrides remain available.

**Tech Stack:** Rust, Cargo, serde/serde_json, existing CLI assembler/command/schema modules, existing retirement RMD engine, markdown reference packs with structured metadata.

---

## File Structure

This plan assumes execution happens in a fresh worktree branched from `flexible-reference-pack-installation` or after that branch is merged.

**Create:**
- `reference/retirement/2026/uniform_lifetime_table.md`
- `reference/retirement/2026/joint_life_table.md`
- `reference/retirement/2026/single_life_expectancy_table.md`
- `reference/retirement/2026/distribution_rules.md`
- `cli/src/support/reference_packs.rs`

**Modify:**
- `reference/manifest.json`
- `reference/README.md`
- `cli/src/support/mod.rs`
- `cli/src/support/reference_paths.rs`
- `cli/src/assembler/retirement.rs`
- `cli/src/commands/retirement.rs`
- `cli/src/schema/retirement.rs`
- `cli/tests/integration.rs`
- `engine/src/models/retirement_rmd.rs`
- `engine/src/validation/mod.rs`
- `engine/src/compute/retirement/rmd.rs`
- `engine/tests/golden.rs`
- `README.md`

**Test Surfaces:**
- `cli/tests/integration.rs`
- `engine/tests/golden.rs`
- focused validation tests in `engine/src/validation/mod.rs`

## Implementation Notes

- Do not try to redesign every retirement calculator in this change.
- Keep `compute rmd-schedule` compatible with the same local pack loading approach if the delta is small; do not fully redesign its public schema in this pass.
- Keep the engine deterministic and auditable.
- The CLI should load installed packs using the resolved reference root; the engine should not learn filesystem path resolution.
- Use TDD for each contract change.
- Commit after each task.

### Task 1: Land Retirement Reference Packs

**Files:**
- Create: `reference/retirement/2026/uniform_lifetime_table.md`
- Create: `reference/retirement/2026/joint_life_table.md`
- Create: `reference/retirement/2026/single_life_expectancy_table.md`
- Create: `reference/retirement/2026/distribution_rules.md`
- Modify: `reference/manifest.json`
- Modify: `reference/README.md`
- Test: `cli/tests/integration.rs`

- [ ] **Step 1: Write the failing integration test for installed retirement packs**

Add a CLI integration assertion near the existing schema/discovery coverage that `entropyfa env --json` plus the installed reference root can expose retirement pack presence via the manifest path assumptions. Keep this narrow: assert that the manifest file includes retirement entries once the packs are added.

- [ ] **Step 2: Run the focused test to verify it fails**

Run: `cargo test -p entropyfa --test integration compute_rmd_schema -- --exact`

Expected: PASS on the old schema test, but no manifest-backed retirement pack assertions exist yet. Add the new assertion and confirm it fails because the files and manifest entries are missing.

- [ ] **Step 3: Write the minimal reference-pack files**

Create the retirement markdown packs using the reviewed 2026 RMD data already represented in `engine/src/data/retirement/rmd_rules.rs` and the table builders in the existing data module.

Each file must include:
- a rigid machine-readable block
- a short primer
- citations / review metadata

Seed content from current engine truth:
- `uniform_lifetime_table.md`: ages and distribution periods from `rmd_tables::uniform_lifetime()`
- `joint_life_table.md`: owner/spouse ages and periods from `rmd_tables::joint_life()`
- `single_life_expectancy_table.md`: ages and periods from `rmd_tables::single_life()`
- `distribution_rules.md`: start-age rules, account rules, beneficiary rules, ten-year rule, relief years, and pre-1987 403(b) rule from `rmd_rules::distribution_rules()`

- [ ] **Step 4: Register the packs in the reference manifest**

Update `reference/manifest.json` to include the new retirement pack entries and bundle metadata so installation/discovery can see them.

- [ ] **Step 5: Update the reference root README**

Add one short section documenting the retirement pack layout and note that these packs are the canonical source for agent-readable RMD rules.

- [ ] **Step 6: Run the focused test to verify it passes**

Run: `cargo test -p entropyfa --test integration compute_rmd_schema -- --exact`

Expected: PASS once the manifest/file assumptions are satisfied.

- [ ] **Step 7: Commit**

```bash
git add reference/retirement/2026 reference/manifest.json reference/README.md cli/tests/integration.rs
git commit -m "feat: add retirement reference packs"
```

### Task 2: Add CLI Retirement Pack Loading

**Files:**
- Create: `cli/src/support/reference_packs.rs`
- Modify: `cli/src/support/mod.rs`
- Modify: `cli/src/support/reference_paths.rs`
- Modify: `cli/src/assembler/retirement.rs`
- Test: `cli/tests/integration.rs`

- [ ] **Step 1: Write the failing tests for local RMD pack resolution**

Add focused unit-style coverage where possible in CLI support, plus integration coverage that `compute rmd --json` can succeed without inline `rmd_parameters` when the reference root contains the installed retirement packs.

Also add a failing integration test for a missing-pack case that should return a structured `reference_pack_missing` error instead of `assembly_error` or a generic validation failure.

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
cargo test -p entropyfa --test integration compute_rmd_valid_without_inline_parameters -- --exact
cargo test -p entropyfa --test integration compute_rmd_missing_reference_pack -- --exact
```

Expected: FAIL because `assemble_rmd` still blindly deserializes `rmd_parameters` and the CLI has no pack loader.

- [ ] **Step 3: Implement the pack loader support module**

Create `cli/src/support/reference_packs.rs` with one clear responsibility: load and parse installed markdown reference packs into typed retirement structures suitable for request assembly.

Start with RMD-specific helpers, for example:
- `load_rmd_reference_bundle(reference_root: &Path, year: u32) -> Result<RmdReferenceBundle, String>`
- parsing helpers for the structured block in each retirement markdown file

Keep the loader narrow:
- no broad indexing
- no hidden fallback to embedded engine rules
- clear errors for missing file vs malformed file

- [ ] **Step 4: Wire the support module into CLI assembly**

Update `cli/src/support/mod.rs` to export the new helper.

Update `cli/src/assembler/retirement.rs`:
- change `assemble_rmd` to:
  - accept normal request JSON without `rmd_parameters`
  - resolve the reference root using the existing reference-path machinery from the installation branch
  - load the required retirement packs for `calculation_year`
  - build the `RmdParameters` struct for the engine
- keep an explicit override path if the input includes override rule payloads

Do the same for `assemble_rmd_schedule` if the delta is small and mechanical.

- [ ] **Step 5: Add precise missing-pack and invalid-pack errors**

Return structured assembly errors that can later be mapped in the command layer:
- `reference_pack_missing`
- `reference_pack_invalid`

At the assembly layer, make the string messages explicit enough for the command layer to classify.

- [ ] **Step 6: Run the focused tests to verify they pass**

Run:

```bash
cargo test -p entropyfa --test integration compute_rmd_valid_without_inline_parameters -- --exact
cargo test -p entropyfa --test integration compute_rmd_missing_reference_pack -- --exact
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add cli/src/support/mod.rs cli/src/support/reference_paths.rs cli/src/support/reference_packs.rs cli/src/assembler/retirement.rs cli/tests/integration.rs
git commit -m "feat: load RMD reference packs from install root"
```

### Task 3: Refactor the RMD Request and Validation Contracts

**Files:**
- Modify: `engine/src/models/retirement_rmd.rs`
- Modify: `engine/src/validation/mod.rs`
- Modify: `cli/src/assembler/retirement.rs`
- Test: `engine/src/validation/mod.rs`
- Test: `engine/tests/golden.rs`

- [ ] **Step 1: Write the failing engine-side tests for the lighter request**

Add validation tests showing:
- a normal owner-lifetime request without inline `rmd_parameters` should be valid once the CLI has assembled resolved rules into the request object
- malformed override payloads should be rejected cleanly before compute

Add a golden-style test that builds the request through the new assembly path and verifies the age-75 uniform-table example still computes the same numeric answer.

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
cargo test -p entropyfa_engine validate_retirement_rmd_request -- --nocapture
cargo test -p entropyfa_engine rmd_age_75_uniform_table_2026 -- --exact
```

Expected: FAIL until the model and validation contract are updated to the new assembly expectations.

- [ ] **Step 3: Update the request/response models**

Modify `engine/src/models/retirement_rmd.rs`:
- keep the engine request shape compatible with deterministic compute
- add provenance-friendly response fields such as:
  - `references_used`
  - `assumptions_used`
  - `overrides_used`

Do not push filesystem concepts into the engine request. The engine should still receive fully assembled rule data; the CLI remains responsible for locating files.

- [ ] **Step 4: Tighten validation around assembled rules and overrides**

Update `engine/src/validation/mod.rs` to validate:
- assembled `RmdParameters` remain structurally sound
- override payloads, if present, are coherent
- the current beneficiary/account-type constraints still hold

Avoid reintroducing “caller must provide full `rmd_parameters`” language in validation errors.

- [ ] **Step 5: Preserve numeric behavior**

Confirm the golden RMD computations still match the current reviewed values after the request/assembly refactor.

- [ ] **Step 6: Run the focused tests to verify they pass**

Run:

```bash
cargo test -p entropyfa_engine validate_retirement_rmd_request -- --nocapture
cargo test -p entropyfa_engine rmd_age_75_uniform_table_2026 -- --exact
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add engine/src/models/retirement_rmd.rs engine/src/validation/mod.rs cli/src/assembler/retirement.rs engine/tests/golden.rs
git commit -m "refactor: align RMD request contract with local reference packs"
```

### Task 4: Add Provenance to RMD Compute Responses

**Files:**
- Modify: `engine/src/compute/retirement/rmd.rs`
- Modify: `engine/src/models/retirement_rmd.rs`
- Test: `engine/tests/golden.rs`
- Test: `cli/tests/integration.rs`

- [ ] **Step 1: Write the failing tests for response provenance**

Add assertions that `compute rmd --json` returns:
- the standard numeric result
- `references_used`
- `assumptions_used`
- `overrides_used`

For the normal path, verify that `references_used` names the installed retirement packs and `overrides_used` is empty.

For an override-path test, verify that `overrides_used` is populated and does not falsely imply that only the installed standard packs governed the result.

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
cargo test -p entropyfa --test integration compute_rmd_response_reports_references_used -- --exact
cargo test -p entropyfa --test integration compute_rmd_response_reports_overrides_used -- --exact
```

Expected: FAIL because the current response only includes the bare result fields plus citations.

- [ ] **Step 3: Implement provenance-rich responses**

Update `engine/src/compute/retirement/rmd.rs` and the response model to return:
- existing numeric decision fields
- `references_used`
- `assumptions_used`
- `overrides_used`

Keep the provenance payload minimal and explicit.

- [ ] **Step 4: Preserve existing decision trace behavior**

Do not regress:
- `decision_trace`
- `table_used`
- `rule_path`
- current numeric rounding

- [ ] **Step 5: Run the focused tests to verify they pass**

Run:

```bash
cargo test -p entropyfa --test integration compute_rmd_response_reports_references_used -- --exact
cargo test -p entropyfa --test integration compute_rmd_response_reports_overrides_used -- --exact
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add engine/src/compute/retirement/rmd.rs engine/src/models/retirement_rmd.rs engine/tests/golden.rs cli/tests/integration.rs
git commit -m "feat: add RMD reference and override provenance"
```

### Task 5: Redesign `compute rmd --schema`

**Files:**
- Modify: `cli/src/schema/retirement.rs`
- Modify: `cli/tests/integration.rs`
- Test: `cli/tests/integration.rs`

- [ ] **Step 1: Write the failing schema tests**

Add integration assertions that `compute rmd --schema` now exposes:
- `required_client_facts`
- `reference_requirements`
- `optional_assumptions`
- `optional_overrides`
- `output_schema`

Also assert that:
- `rmd_parameters` is no longer listed as a required normal input
- the example command is runnable without inline `rmd_parameters`
- the schema lists the retirement reference pack paths explicitly

- [ ] **Step 2: Run the focused schema tests to verify they fail**

Run:

```bash
cargo test -p entropyfa --test integration compute_rmd_schema -- --exact
```

Expected: FAIL because the current schema still exposes `gather_from_user.required` with `rmd_parameters`.

- [ ] **Step 3: Rewrite the schema output**

Update `cli/src/schema/retirement.rs` so `rmd_schema()` presents the new contract:
- required client facts
- reference requirements with paths and purpose
- optional assumptions
- optional overrides
- output schema / summary aligned to the real response

Keep the schema honest about current constraints and supported scenarios.

- [ ] **Step 4: Keep `rmd-schedule` aligned where cheap**

If the schedule command is using the same loader and rule source, update its schema language so it no longer tells the agent to supply the entire rule corpus either. Keep this modest; avoid full redesign unless it is mechanically shared.

- [ ] **Step 5: Run the focused schema tests to verify they pass**

Run:

```bash
cargo test -p entropyfa --test integration compute_rmd_schema -- --exact
cargo test -p entropyfa --test integration compute_rmd_schedule_schema -- --exact
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add cli/src/schema/retirement.rs cli/tests/integration.rs
git commit -m "feat: expose schema-guided RMD contract"
```

### Task 6: Command-Layer Error Mapping and Docs

**Files:**
- Modify: `cli/src/commands/retirement.rs`
- Modify: `README.md`
- Modify: `cli/tests/integration.rs`
- Test: `cli/tests/integration.rs`

- [ ] **Step 1: Write the failing command/doc tests**

Add integration coverage that missing installed retirement packs yield a structured CLI error code:
- `reference_pack_missing`

Add one README example check by updating the documented `compute rmd` example and then using it in an integration-style smoke run if you already have a docs-smoke convention; otherwise keep the coverage in `cli/tests/integration.rs`.

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
cargo test -p entropyfa --test integration compute_rmd_missing_reference_pack -- --exact
```

Expected: FAIL because the command layer currently maps everything through generic assembly or validation errors.

- [ ] **Step 3: Add explicit command-level error mapping**

Update `cli/src/commands/retirement.rs` to classify assembly failures from the new loader:
- `reference_pack_missing`
- `reference_pack_invalid`
- fallback `assembly_error`

Do not overengineer a generic error taxonomy in this slice.

- [ ] **Step 4: Update public docs**

Update `README.md` to show the new RMD flow:
- ask schema
- read the listed retirement reference packs
- provide client facts
- run `compute rmd`

Be explicit that the normal path no longer requires inline `rmd_parameters`.

- [ ] **Step 5: Run the focused tests to verify they pass**

Run:

```bash
cargo test -p entropyfa --test integration compute_rmd_missing_reference_pack -- --exact
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add cli/src/commands/retirement.rs README.md cli/tests/integration.rs
git commit -m "feat: clarify RMD reference-pack errors and docs"
```

### Task 7: Full Verification

**Files:**
- Modify: none
- Test: `cli/tests/integration.rs`
- Test: `engine/tests/golden.rs`
- Test: `engine/src/validation/mod.rs`

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all -- --check`

Expected: PASS.

- [ ] **Step 2: Run focused RMD engine and CLI tests**

Run:

```bash
cargo test -p entropyfa --test integration compute_rmd_schema -- --exact
cargo test -p entropyfa --test integration compute_rmd_schedule_schema -- --exact
cargo test -p entropyfa --test integration compute_rmd_valid_without_inline_parameters -- --exact
cargo test -p entropyfa --test integration compute_rmd_missing_reference_pack -- --exact
cargo test -p entropyfa_engine rmd_age_75_uniform_table_2026 -- --exact
```

Expected: PASS.

- [ ] **Step 3: Run full workspace verification**

Run: `cargo test --workspace`

Expected: PASS.

- [ ] **Step 4: Run live smoke commands**

Run:

```bash
cargo run -p entropyfa -- compute rmd --schema
cargo run -p entropyfa -- compute rmd --json '{"calculation_year":2026,"prior_year_end_balance":500000,"account_type":"traditional_ira","owner_birth_date":"1953-06-15"}'
```

Expected:
- schema includes `required_client_facts`, `reference_requirements`, `optional_assumptions`, `optional_overrides`, and `output_schema`
- live command returns `ok: true` without inline `rmd_parameters`

- [ ] **Step 5: Commit the final verification-only checkpoint if needed**

```bash
git status --short
```

If verification exposed no new file edits, do not create a no-op commit.

## Handoff Notes

- Execute this plan in a worktree based on `flexible-reference-pack-installation`, not raw `main`.
- Reuse existing reference-root resolution instead of inventing a second path-discovery system.
- Keep the pack loader focused on retirement RMD packs for this slice.
- Do not let the engine silently fall back to hidden embedded rules for the normal path; that would undercut the whole agent-readable reference-pack model.
