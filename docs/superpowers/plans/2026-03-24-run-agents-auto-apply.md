# Run-Agents Auto-Apply Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `data-pipeline run-agents` automatically apply a run when review passes cleanly, while leaving `prepare`, `review`, and `apply` as manual escape hatches.

**Architecture:** Change workflow orchestration so `run_agents_at` performs an automatically approved review only for the one-command path, then conditionally calls `apply_run_at` when the review is approved, has no blocking issues, and recommends `apply_approved_result`. Expose the apply outcome through the workflow result and update CLI output to report whether auto-apply occurred.

**Tech Stack:** Rust, Cargo, existing data-pipeline workflow and bin CLI, existing fake-agent workflow tests.

---

### Task 1: Add failing tests for auto-apply workflow behavior

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/run-agents-auto-apply/engine/tests/data_pipeline_workflow.rs`

- [ ] **Step 1: Write failing happy-path test for auto-apply**

Update the current `run_agents` happy-path test so it expects:
- `review.approved == true`
- `applied.is_some()`
- the reference pack path exists

- [ ] **Step 2: Run the focused test to verify it fails**

Run: `cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_prepares_executes_and_auto_applies_on_clean_review -- --exact`

Expected: FAIL because `run_agents_at` currently stops at review and never applies.

- [ ] **Step 3: Write failing blocked-review test**

Add a fake verifier-agent variant that emits a dispute so review blocks, then assert:
- `review.approved == false`
- `applied.is_none()`

- [ ] **Step 4: Run the focused test to verify it fails**

Run: `cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_does_not_auto_apply_when_review_blocks -- --exact`

Expected: FAIL because `RunAgentsOutcome` does not yet expose apply state.

### Task 2: Add failing test for CLI run-agents summary output

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/run-agents-auto-apply/engine/src/bin/data-pipeline.rs`

- [ ] **Step 1: Extract a small summary formatter seam**

Before implementation, add a unit test around a new helper that should render:
- `auto_applied: true`
- `reference_pack: ...`
- `generated_source: ...`

for a synthetic auto-applied outcome.

- [ ] **Step 2: Run the focused test to verify it fails**

Run: `cargo test -p entropyfa-engine --bin data-pipeline auto_applied_summary_includes_apply_artifacts -- --exact`

Expected: FAIL because no such summary helper or output exists yet.

### Task 3: Implement workflow auto-apply

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/run-agents-auto-apply/engine/src/data_pipeline/workflow.rs`

- [ ] **Step 1: Extend `RunAgentsOutcome`**

Add:
- `pub applied: Option<ApplyOutcome>`

- [ ] **Step 2: Change run-agents review to auto-approve clean runs**

In `run_agents_at`, call `review_run_with_approval_at(..., true, None)` instead of the non-approved review path.

- [ ] **Step 3: Conditionally call `apply_run_at`**

Auto-apply only when:
- `review.approved`
- `review.blocking_issues.is_empty()`
- `review.recommended_action == ApplyApprovedResult`

- [ ] **Step 4: Re-run the focused workflow tests**

Run:
- `cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_prepares_executes_and_auto_applies_on_clean_review -- --exact`
- `cargo test -p entropyfa-engine --test data_pipeline_workflow run_agents_does_not_auto_apply_when_review_blocks -- --exact`

Expected: PASS

### Task 4: Implement CLI auto-apply reporting

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/run-agents-auto-apply/engine/src/bin/data-pipeline.rs`

- [ ] **Step 1: Add a summary formatter/helper**

Create a small helper that renders the `run-agents` summary lines so it can be unit-tested.

- [ ] **Step 2: Print `auto_applied: true|false`**

When `applied.is_some()`, print the same artifact paths shown by the standalone `apply` command.

- [ ] **Step 3: Keep next-step guidance only for non-applied runs**

If auto-apply happened, do not print manual `apply` next steps.

- [ ] **Step 4: Re-run the focused CLI test**

Run: `cargo test -p entropyfa-engine --bin data-pipeline auto_applied_summary_includes_apply_artifacts -- --exact`

Expected: PASS

### Task 5: Update docs

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/run-agents-auto-apply/docs/data-pipeline.md`
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/run-agents-auto-apply/README.md`

- [ ] **Step 1: Update `data-pipeline` docs**

Document that `run-agents` now auto-applies clean approved runs and only stops at review on failures or non-apply recommendations.

- [ ] **Step 2: Update README maintainer workflow wording**

Align the high-level maintainer flow with the new one-command happy path.

### Task 6: Verify the merged behavior

**Files:**
- Modify: none

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all -- --check`

- [ ] **Step 2: Run focused workflow tests**

Run: `cargo test -p entropyfa-engine --test data_pipeline_workflow`

- [ ] **Step 3: Run bin tests**

Run: `cargo test -p entropyfa-engine --bin data-pipeline`

- [ ] **Step 4: Run full workspace tests**

Run: `cargo test --workspace`

- [ ] **Step 5: Smoke-test the command**

Run:
`cargo run -p entropyfa-engine --bin data-pipeline -- run-agents --year 2026 --category insurance --key irmaa_brackets --primary-provider claude --primary-model claude-opus-4-6 --verifier-provider codex --verifier-model gpt-5.4`

Expected:
- review succeeds cleanly
- `auto_applied: true`
- apply artifact paths are printed

- [ ] **Step 6: Commit**

```bash
git add engine/src/data_pipeline/workflow.rs \
  engine/src/bin/data-pipeline.rs \
  engine/tests/data_pipeline_workflow.rs \
  docs/data-pipeline.md \
  README.md \
  docs/superpowers/plans/2026-03-24-run-agents-auto-apply.md
git commit -m "feat: auto-apply clean pipeline runs"
```
