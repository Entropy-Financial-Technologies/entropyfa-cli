# Pipeline Status Markdown Pack Migration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend the maintainer pipeline so `data-pipeline status` shows markdown-pack migration coverage and `apply` writes installable markdown reference packs plus updated manifest metadata.

**Architecture:** Keep the existing registry + reviewed JSON workflow as the compatibility base, then add a second canonical artifact path for markdown packs. Status becomes the migration dashboard by tracking both reviewed JSON and markdown-pack coverage per entry, while `apply` writes both artifacts and refreshes the repo-owned `reference/manifest.json`.

**Tech Stack:** Rust, Cargo, serde/serde_json, existing engine data-pipeline workflow, existing ratatui status dashboard, markdown reference packs.

---

### Task 1: Add failing workflow tests for markdown-pack coverage

**Files:**
- Modify: `engine/tests/data_pipeline_workflow.rs`

- [ ] Add a failing status-report test assertion for new report counts and per-entry markdown-pack fields.
- [ ] Add a failing apply test assertion that a non-retirement entry writes a markdown pack and updates manifest metadata.
- [ ] Run the focused workflow tests and confirm the new assertions fail for the expected reason.

### Task 2: Extend pipeline status/report models and renderers

**Files:**
- Modify: `engine/src/data_pipeline/workflow.rs`
- Modify: `engine/src/bin/data-pipeline.rs`

- [ ] Add report-level markdown-pack counts and per-entry pack coverage fields.
- [ ] Add a legacy-only / migration-gap signal derived from reviewed JSON without markdown pack.
- [ ] Update plain-text and TUI status rendering to show markdown-pack coverage and a legacy-only queue.
- [ ] Run focused tests and adjust output wording until the new assertions pass.

### Task 3: Write markdown packs and refresh manifest during apply

**Files:**
- Modify: `engine/src/data_pipeline/workflow.rs`
- Modify: `reference/manifest.json` if fixture expectations need to change

- [ ] Add a central mapping from reviewed entry key to reference-pack path/filename.
- [ ] Implement markdown-pack rendering from reviewed artifacts with rigid front matter + machine block + short primer/citation section.
- [ ] Update `apply_run_at` to write the markdown pack alongside reviewed JSON and generated Rust.
- [ ] Refresh the repo reference manifest from files on disk after apply.
- [ ] Run the focused workflow tests and confirm apply now produces packs and manifest changes.

### Task 4: Document the migration dashboard and verify the full repo

**Files:**
- Modify: `docs/data-pipeline.md`
- Modify: `reference/README.md`

- [ ] Update maintainer docs so `status` is described as the live migration dashboard for reviewed JSON vs markdown packs.
- [ ] Document that `apply` now writes markdown packs in addition to reviewed JSON for compatibility.
- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo test --workspace`.
- [ ] Run `cargo run -p entropyfa-engine --bin data-pipeline -- status --plain` and inspect the live output.
