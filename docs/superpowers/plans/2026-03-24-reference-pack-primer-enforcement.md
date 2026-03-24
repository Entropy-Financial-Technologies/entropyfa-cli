# Reference Pack Primer Enforcement Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enforce rich, verifier-reviewed primer sections in generated markdown reference packs so `review` and `apply` block when the primer is missing or disputed.

**Architecture:** Extend the pipeline artifact contract with a structured primary primer and verifier primer verdicts, validate those sections during review, and render the approved primer into generated markdown packs above the machine block. Keep the existing numeric extraction flow intact and additive.

**Tech Stack:** Rust, Cargo, serde/serde_json, existing data-pipeline workflow and tests, markdown reference packs.

---

### Task 1: Add failing workflow tests for primer enforcement

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/pipeline-status-md-migration/engine/tests/data_pipeline_workflow.rs`

- [ ] **Step 1: Add test helpers for valid primer payloads**

Add helper builders for a valid primary `reference_pack_primer` object and matching verifier `primer_verdicts`.

- [ ] **Step 2: Write failing review test for missing primer**

Add a test that writes otherwise-valid primary/verifier artifacts but omits the primary primer, then asserts `review` is not approved and includes a blocking issue for the missing primer.

- [ ] **Step 3: Run the focused test to verify it fails**

Run: `cargo test -p entropyfa-engine --test data_pipeline_workflow review_blocks_when_reference_pack_primer_is_missing -- --exact`

Expected: FAIL because review currently ignores primer presence.

- [ ] **Step 4: Write failing review test for disputed primer section**

Add a test that writes a valid primer but marks one required verifier primer verdict as `dispute`, then asserts `review` blocks with the disputed section called out.

- [ ] **Step 5: Run the focused test to verify it fails**

Run: `cargo test -p entropyfa-engine --test data_pipeline_workflow review_blocks_when_verifier_disputes_required_primer_section -- --exact`

Expected: FAIL because review currently ignores primer verdicts.

- [ ] **Step 6: Write failing apply/render test for primer headings**

Add a test that drives a full approve/apply flow and asserts the generated markdown pack contains `## What This Is`, `## Lookup Parameters`, `## Interpretation Notes`, `## Does Not Include`, and `## Caveats`.

- [ ] **Step 7: Run the focused test to verify it fails**

Run: `cargo test -p entropyfa-engine --test data_pipeline_workflow apply_writes_reference_pack_with_required_primer_sections -- --exact`

Expected: FAIL because the current renderer only emits a generic intro.

### Task 2: Extend the pipeline artifact contract with primer data

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/pipeline-status-md-migration/engine/src/data_pipeline/workflow.rs`

- [ ] **Step 1: Add primer structs and enums**

Add focused types for:
- primary `ReferencePackPrimer`
- verifier `PrimerSectionVerdict`
- verifier `PrimerVerdicts`

Keep required sections explicit and `typical_uses` optional.

- [ ] **Step 2: Thread primer fields into submission/review structs**

Extend:
- `PrimarySubmission`
- `VerifierSubmission`
- `ReviewDecision`
- `ReviewedArtifact` only if needed for pack rendering

Use `#[serde(default)]` where it preserves deserialization of older artifacts, but rely on review validation to enforce the new contract.

- [ ] **Step 3: Update primary and verifier templates**

Add primer skeletons to:
- `build_primary_template`
- `build_verifier_template`

- [ ] **Step 4: Update prompts and report templates**

Revise:
- `render_primary_prompt`
- `render_verifier_prompt`

So agents are explicitly told the primer is required, data-centric, and independently reviewed.

### Task 3: Enforce primer review rules

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/pipeline-status-md-migration/engine/src/data_pipeline/workflow.rs`

- [ ] **Step 1: Add primer validation helpers**

Implement small helpers to validate:
- required primary primer sections exist and are non-empty
- verifier verdicts exist for each required section
- any `dispute` or `uncertain` verdict becomes a blocking issue

- [ ] **Step 2: Wire primer validation into review**

Extend `review_run_internal` so primer issues are added to `blocking_issues` alongside numeric validation issues.

- [ ] **Step 3: Preserve approved primer in the review artifact**

Store the approved primary primer in `ReviewDecision` so `apply` can render the exact reviewed text.

- [ ] **Step 4: Re-run the focused review tests**

Run:
- `cargo test -p entropyfa-engine --test data_pipeline_workflow review_blocks_when_reference_pack_primer_is_missing -- --exact`
- `cargo test -p entropyfa-engine --test data_pipeline_workflow review_blocks_when_verifier_disputes_required_primer_section -- --exact`

Expected: PASS

### Task 4: Render primer sections into markdown packs

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/pipeline-status-md-migration/engine/src/data_pipeline/workflow.rs`

- [ ] **Step 1: Update pack rendering input path**

Make `apply_run_at` pass the approved primer into pack rendering rather than relying on a generic intro string.

- [ ] **Step 2: Update `render_reference_pack_markdown`**

Render the primer sections above the machine block using stable headings and bullet formatting.

- [ ] **Step 3: Re-run the focused apply/render test**

Run: `cargo test -p entropyfa-engine --test data_pipeline_workflow apply_writes_reference_pack_with_required_primer_sections -- --exact`

Expected: PASS

### Task 5: Update helper outputs and pipeline docs

**Files:**
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/pipeline-status-md-migration/engine/tests/data_pipeline_workflow.rs`
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/pipeline-status-md-migration/docs/data-pipeline.md`
- Modify: `/Users/dan/dev/entropyfa-cli/.worktrees/pipeline-status-md-migration/reference/README.md`

- [ ] **Step 1: Update shared test helpers**

Make the common happy-path helper writers emit valid primer data and verifier primer verdicts so existing tests keep representing the current contract.

- [ ] **Step 2: Document the new primer requirement**

Update maintainer docs to say:
- primary agent writes the primer
- verifier independently reviews it
- review/apply block on missing or disputed required sections

- [ ] **Step 3: Document the generated pack structure**

Update reference-root docs to describe the richer primer sections now generated above the machine block.

### Task 6: Verify the full workflow

**Files:**
- Modify: none

- [ ] **Step 1: Run formatting**

Run: `cargo fmt --all -- --check`

- [ ] **Step 2: Run the workflow test suite**

Run: `cargo test -p entropyfa-engine --test data_pipeline_workflow`

- [ ] **Step 3: Run the full workspace tests**

Run: `cargo test --workspace`

- [ ] **Step 4: Smoke-check status output still works**

Run: `cargo run -p entropyfa-engine --bin data-pipeline -- status --plain`

- [ ] **Step 5: Commit**

```bash
git add docs/superpowers/specs/2026-03-24-reference-pack-primer-enforcement-design.md \
  docs/superpowers/plans/2026-03-24-reference-pack-primer-enforcement.md \
  engine/src/data_pipeline/workflow.rs \
  engine/tests/data_pipeline_workflow.rs \
  docs/data-pipeline.md \
  reference/README.md
git commit -m "feat: enforce reviewed primer sections in reference packs"
```
