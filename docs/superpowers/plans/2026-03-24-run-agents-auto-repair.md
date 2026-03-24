# Run-Agents Auto-Repair Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `data-pipeline run-agents` automatically repair and re-review safe blocked runs, then auto-apply them when the repaired run becomes clean, without weakening value/source/schema review boundaries.

**Architecture:** Extend the verifier and review contracts so blocked issues are classified as `auto_resolvable` or `manual_required`, then add a bounded repair pass to `run_agents_at` that rewrites only safe artifacts such as primer prose or citation locators. Persist repair artifacts separately in the run directory, re-run verifier and review against the repaired submission, and auto-apply only if the repaired review passes cleanly.

**Tech Stack:** Rust, Cargo, existing data-pipeline workflow and bin CLI, existing fake-agent workflow tests, markdown maintainer docs.

---

## File Structure Map

**Core workflow and contract files**

- Modify: `/Users/dan/dev/entropyfa-cli/engine/src/data_pipeline/workflow.rs`
  - Add verifier/review contract fields for blocker classification and repair guidance
  - Add repair artifact paths, repair prompt/template generation, repaired-primary loading, and the bounded auto-repair loop in `run_agents_at`
- Modify: `/Users/dan/dev/entropyfa-cli/engine/src/bin/data-pipeline.rs`
  - Extend `run-agents` summary output with repair-attempt/result lines

**Tests**

- Modify: `/Users/dan/dev/entropyfa-cli/engine/tests/data_pipeline_workflow.rs`
  - Add red/green workflow coverage for auto-repair success, non-eligible blocked runs, and repair safety boundaries
- Modify: `/Users/dan/dev/entropyfa-cli/engine/src/bin/data-pipeline.rs`
  - Extend the existing unit test module for CLI summary rendering

**Docs**

- Modify: `/Users/dan/dev/entropyfa-cli/docs/data-pipeline.md`
  - Document clean-pass auto-apply plus safe blocked-run auto-repair
- Modify: `/Users/dan/dev/entropyfa-cli/README.md`
  - Update the maintainer workflow summary

**Do not commit**

- Do not include local maintainer-run outputs already present in the tree, such as:
  - `/Users/dan/dev/entropyfa-cli/engine/data_registry/2026/metadata.json`
  - `/Users/dan/dev/entropyfa-cli/engine/data_registry/2026/reviewed/...`
  - `/Users/dan/dev/entropyfa-cli/reference/...`
- Use temp-engine-root tests for workflow coverage instead of real repo run directories.

---

### Task 1: Add failing tests for repair eligibility and repair loop behavior

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/engine/tests/data_pipeline_workflow.rs`

- [ ] **Step 1: Add a failing review-classification fixture shape**

Extend the fake verifier payload helpers so a verifier can emit section- or field-level metadata describing:
- `issue_type`
- `auto_resolvable`
- `repair_guidance`

Start with primer-only disputes and one manual-required value dispute.

- [ ] **Step 2: Add a failing happy-path auto-repair test**

Add a workflow test where:
- primary produces a correct value plus an overbroad primer
- verifier confirms the value but marks required primer sections as disputed and `auto_resolvable: true`
- a fake repair agent rewrites only the primer
- rerun verifier/review becomes clean
- `run_agents_at` returns `applied.is_some()`

- [ ] **Step 3: Add a failing non-eligible blocked-run test**

Add a workflow test where:
- verifier disputes a required value field and marks it `auto_resolvable: false`
- `run_agents_at` stops at review
- no repair pass runs
- `applied.is_none()`

- [ ] **Step 4: Add a failing repair-safety test**

Add a workflow test where a fake repair agent tries to mutate the reviewed numeric value while only primer repairs were allowed.

Expected behavior:
- repair path is rejected
- run remains blocked

- [ ] **Step 5: Run the focused workflow tests to verify RED**

Run:

```bash
cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_auto_repairs_safe_primer_disputes_and_applies -- --exact
cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_does_not_auto_repair_manual_required_value_dispute -- --exact
cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_rejects_repair_that_mutates_value_during_safe_repair -- --exact
```

Expected:
- compile failures or test failures because the repair contract and loop do not exist yet

### Task 2: Extend verifier and review contracts for repair classification

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/engine/src/data_pipeline/workflow.rs`
- Modify: `/Users/dan/dev/entropyfa-cli/engine/tests/data_pipeline_workflow.rs`

- [ ] **Step 1: Add structured repair metadata to verdict types**

Update the workflow-layer serde structs so disputed items can carry:
- `issue_type`
- `auto_resolvable`
- `repair_guidance`

At minimum:
- `PrimerSectionVerdict`
- `FieldVerdict`
- `SourceVerdict` if citation-source defects need the same classification path

Prefer explicit enums over freeform strings in Rust, with `snake_case` serde.

- [ ] **Step 2: Extend `ReviewDecision` / `ReviewOutcome`**

Add machine-readable review classification fields such as:
- `auto_repair_eligible`
- `all_blockers_auto_resolvable`
- `auto_resolvable_blockers`
- `manual_required_blockers`

Keep the existing human-readable `blocking_issues` array.

- [ ] **Step 3: Update verifier template generation**

Update `build_verifier_template(...)` so the JSON skeleton includes the new fields with clear defaults/placeholders.

- [ ] **Step 4: Update verifier prompt instructions**

Update `render_verifier_prompt(...)` so the verifier is required to:
- classify each disputed issue
- mark whether it is auto-resolvable
- give bounded repair guidance
- avoid marking value/source/schema issues as auto-resolvable

- [ ] **Step 5: Update review assembly logic**

Modify `review_run_with_approval_at(...)` internals so the review outcome computes:
- which blockers are safe
- whether every blocker is safe
- whether the run is eligible for one repair attempt

- [ ] **Step 6: Run focused tests to verify GREEN for contract changes**

Run the three focused tests from Task 1 again.

Expected:
- still failing on missing repair execution, but the review objects and parsed verifier payloads should now compile and expose the needed classification fields

### Task 3: Add a bounded repair artifact pipeline

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/engine/src/data_pipeline/workflow.rs`
- Modify: `/Users/dan/dev/entropyfa-cli/engine/tests/data_pipeline_workflow.rs`

- [ ] **Step 1: Add repair-specific run artifacts**

Define the artifact paths for:
- `repair_prompt.md`
- `repair_template.json`
- `repair_output.json`
- `repair_report.md`

Do not overwrite `primary_output.json`. Keep original and repaired artifacts side by side for auditability.

- [ ] **Step 2: Add a repair prompt/template renderer**

Create a bounded repair prompt that includes:
- current reviewed value proposal
- accepted sources
- exact disputed primer sections or citation defects
- explicit instruction that numeric values may not change in safe repair mode

The repair template should reuse the primary submission shape so repaired output can be re-reviewed without inventing a second result schema.

- [ ] **Step 3: Add repaired-primary loading rules**

Update review loading logic so:
- normal review uses `repair_output.json` when present and explicitly selected by the repair path
- otherwise it uses `primary_output.json`

Do not make manual `review --run ...` silently prefer arbitrary repaired artifacts without clear workflow ownership.

- [ ] **Step 4: Add repair validation guards**

Before accepting a repaired submission for re-review:
- verify that safe-repair mode did not mutate reviewed numeric values
- verify that only permitted sections changed
- reject the repair artifact if it exceeds the allowed scope

- [ ] **Step 5: Run focused workflow tests**

Run:

```bash
cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_auto_repairs_safe_primer_disputes_and_applies -- --exact
cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_rejects_repair_that_mutates_value_during_safe_repair -- --exact
```

Expected:
- tests still fail until `run_agents_at` actually invokes the repair loop

### Task 4: Implement the `run-agents` repair-and-rereview loop

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/engine/src/data_pipeline/workflow.rs`

- [ ] **Step 1: Extend `RunAgentsConfig` or local orchestration state only if needed**

Prefer not to change the public CLI arguments in the first slice. Reuse the existing primary/verifier agent configs for repair unless the implementation proves that a distinct repair provider is necessary.

Recommended first slice:
- use the primary provider/model for the repair pass
- keep the verifier independent

- [ ] **Step 2: Add one bounded repair attempt**

In `run_agents_at(...)`, after the first review:
- if review is clean, auto-apply as today
- else if `auto_repair_eligible == true`, run exactly one repair pass
- rerun verifier against the repaired submission
- rerun review
- auto-apply if the repaired review is now clean
- otherwise stop

- [ ] **Step 3: Record repair outcomes in `RunAgentsOutcome`**

Add optional repair outcome fields such as:
- `repair_attempted`
- `repair_succeeded`
- repair artifact paths and re-review paths, or a nested `RepairOutcome`

Keep the original `review` and `applied` fields intact.

- [ ] **Step 4: Re-run focused workflow tests**

Run the three focused workflow tests from Task 1.

Expected:
- all three PASS

### Task 5: Extend CLI summary output for auto-repair

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/engine/src/bin/data-pipeline.rs`

- [ ] **Step 1: Add a failing CLI-summary unit test for repair output**

Extend the existing summary-rendering test seam so a synthetic outcome with repair data expects:
- `auto_repair_attempted: true`
- `auto_repair_succeeded: true|false`
- repair artifact paths when present

- [ ] **Step 2: Run the focused CLI test to verify RED**

Run:

```bash
cargo test -p entropyfa-engine --bin data-pipeline tests::auto_repair_summary_includes_repair_artifacts -- --exact
```

Expected:
- FAIL because the summary helper does not render repair fields yet

- [ ] **Step 3: Update `render_run_agents_summary_lines(...)`**

Print:
- `auto_repair_attempted`
- `auto_repair_succeeded`
- `repair_prompt`
- `repair_output`
- `repair_report`
- re-review paths when useful

Only print manual next-step guidance when the final disposition is still blocked.

- [ ] **Step 4: Re-run the focused CLI test to verify GREEN**

Run:

```bash
cargo test -p entropyfa-engine --bin data-pipeline tests::auto_repair_summary_includes_repair_artifacts -- --exact
```

Expected:
- PASS

### Task 6: Update maintainer docs

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/docs/data-pipeline.md`
- Modify: `/Users/dan/dev/entropyfa-cli/README.md`

- [ ] **Step 1: Update the data-pipeline maintainer workflow**

Document the full `run-agents` path:
- clean review -> auto-apply
- safe blocked review -> auto-repair, re-review, maybe auto-apply
- manual-required blocker -> stop at review

- [ ] **Step 2: Document the trust boundary**

Be explicit that auto-repair does not resolve:
- disputed values
- missing official sources
- schema mismatch

- [ ] **Step 3: Update the README maintainer summary**

Keep the repo-level wording short and point maintainers to `docs/data-pipeline.md` for the full behavior.

### Task 7: Verify and commit

**Files:**
- Modify: none

- [ ] **Step 1: Run formatting**

Run:

```bash
cargo fmt --all -- --check
```

- [ ] **Step 2: Run focused workflow tests**

Run:

```bash
cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_auto_repairs_safe_primer_disputes_and_applies -- --exact
cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_does_not_auto_repair_manual_required_value_dispute -- --exact
cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_rejects_repair_that_mutates_value_during_safe_repair -- --exact
```

- [ ] **Step 3: Run full workflow and bin suites**

Run:

```bash
cargo test -p entropyfa-engine --test data_pipeline_workflow
cargo test -p entropyfa-engine --bin data-pipeline
```

- [ ] **Step 4: Run full workspace tests**

Run:

```bash
cargo test --workspace
```

- [ ] **Step 5: Smoke-test a safe blocked-run scenario**

Use a temp-engine-root or a scripted fake-agent workflow test harness, not a real repo entry, so the verification does not dirty committed reviewed artifacts or `reference/`.

Expected:
- first review blocks only on auto-resolvable primer issues
- repair pass runs once
- verifier/review rerun succeeds
- apply artifacts are produced

- [ ] **Step 6: Commit only code/tests/docs**

```bash
git add engine/src/data_pipeline/workflow.rs \
  engine/src/bin/data-pipeline.rs \
  engine/tests/data_pipeline_workflow.rs \
  docs/data-pipeline.md \
  README.md \
  docs/superpowers/plans/2026-03-24-run-agents-auto-repair.md
git commit -m "feat: auto-repair safe pipeline review blocks"
```
