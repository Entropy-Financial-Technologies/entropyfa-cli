# Data Pipeline

The embedded reference data is part of the product surface. The maintainer workflow has two layers:

1. `prepare` / `review` / `apply` for agent-assisted source verification and code generation
2. `snapshot` / `validate` for post-update regression checking against the live embedded data

Use:

- `status` to inspect the current registry and pipeline state
- `run-agents` when a pipeline definition already exists and you want the full verification flow
- `prepare` / `review` / `apply` when you want to inspect each stage manually

## Verification Model

This workflow is intentionally multi-step and multi-agent.

The default review path uses:

- a primary pass to gather values and produce the proposed payload plus source citations
- a separate verifier pass to independently check the proposal against the cited sources
- a human approval step before any reviewed data is applied to the repo

In the current default setup, `run-agents` uses Claude `claude-opus-4-6` for the primary pass and Codex `gpt-5.4` for the verifier pass. The goal is not model branding. The goal is to avoid single-pass source extraction becoming the final truth without an independent check.

This is also where source policy matters. A run can search broadly, but only accepted sources on policy-approved hosts count toward final status. For example, an IRS-backed entry should be decided by accepted `irs.gov` sources, with supporting or secondary sources used only as corroboration.

## Current Scope

The current workflow is built for yearly reference entries with a stable lookup contract.

Good fits:

- `insurance/irmaa_brackets`
- `tax/federal_income_tax_brackets`
- future yearly federal entries like `tax/federal_standard_deductions`

Current non-fits:

- state-jurisdiction data that needs `state_code` or similar parameters
- monthly or periodic series such as AFR tables
- entries whose official source shape does not match an existing validator/generator

Those are real model gaps, not just missing docs.

## Quick Status Check

Use this before deciding what to run next:

```sh
cargo run -p entropyfa-engine --bin data-pipeline -- status
```

Useful variants:

```sh
cargo run -p entropyfa-engine --bin data-pipeline -- status --plain
cargo run -p entropyfa-engine --bin data-pipeline -- validate
cargo run -p entropyfa-engine --bin data-pipeline -- validate --strict
```

`status` shows:

- which registry entries have pipeline definitions
- the latest run state for pipeline-backed entries
- whether a reviewed artifact exists
- which entries are still `placeholder` or `derived`

## Agent Workflow

Use this when you want Claude Code or Codex to help refresh an existing pipeline-backed entry such as `insurance/irmaa_brackets` or `tax/federal_income_tax_brackets`.

### One-command orchestration

If you want the repo to prepare the run, invoke both agents, and generate a review packet automatically:

```sh
cargo run -p entropyfa-engine --bin data-pipeline -- run-agents \
  --year 2026 \
  --category insurance \
  --key irmaa_brackets \
  --primary-provider claude \
  --primary-model claude-opus-4-6 \
  --verifier-provider codex \
  --verifier-model gpt-5.4
```

This will:

- create a new run folder
- invoke the primary and verifier agents non-interactively
- write `primary_output.json`, `primary_report.md`, `verifier_output.json`, and `verifier_report.md`
- capture agent stdout/stderr logs in the run folder
- run `review` automatically without approving it

You should then inspect `review.md` and only run `apply` manually after you are satisfied.

If you omit the explicit model flags, `run-agents` defaults to Claude `claude-opus-4-6` for the primary pass and Codex `gpt-5.4` for the verifier pass.

### Manual step-by-step workflow

1. Prepare a run:

   ```sh
   cargo run -p entropyfa-engine --bin data-pipeline -- prepare --year 2026 --category insurance --key irmaa_brackets
   ```

2. Open the generated `primary_prompt.md` in a fresh Claude Code or Codex session.
   The agent should search broadly, extract data, cite sources, and write both `primary_output.json` and `primary_report.md`.

3. Open the generated `verifier_prompt.md` in a separate fresh session.
   The verifier should independently review the sources and write both `verifier_output.json` and `verifier_report.md`.

4. Review and approve:

   ```sh
   cargo run -p entropyfa-engine --bin data-pipeline -- review --run <RUN_ID>
   ```

5. Apply the approved result:

   ```sh
   cargo run -p entropyfa-engine --bin data-pipeline -- apply --run <RUN_ID>
   ```

`apply` writes the canonical reviewed artifact, regenerates the target Rust data file, updates metadata, rebuilds the snapshot, reruns validation, and runs targeted tests for the entry.

Run folders live under `engine/data_registry/runs/<year>/<category>/<key>/<run-id>/`.

Each run includes:

- strict JSON templates for machine-readable output
- markdown report templates for human-readable evidence packets
- `current_value.json` for comparison only, not as truth

If either agent concludes the official source no longer fits the current lookup contract, it must set `schema_change_required: true`. Review will then block `apply` until the schema, validator, and generator are updated deliberately.

When that happens, `review.json` and `review.md` now include:

- `recommended_action`
- `suggested_contract_changes`

Use those as the maintainer handoff from source verification back into product and schema design.

## What Gets Persisted

Separate temporary run artifacts from the committed source of truth.

Ephemeral run artifacts:

- `engine/data_registry/runs/<year>/<category>/<key>/<run-id>/...`
- agent stdout/stderr logs
- prompt packs
- `primary_output.json`
- `primary_report.md`
- `verifier_output.json`
- `verifier_report.md`
- `review.json`
- `review.md`

Canonical committed artifacts:

- reviewed artifact:
  `engine/data_registry/<year>/reviewed/<category>/<key>.json`
- generated Rust source:
  whatever `target_source_path` points to
- registry metadata:
  `engine/data_registry/<year>/metadata.json`
- snapshot:
  `engine/data_registry/<year>/snapshot.json`

The reviewed artifact plus generated source are the committed truth. The run folder is the audit trail for how you got there.

Git policy:

- `engine/data_registry/runs/` is local-only and ignored by git
- commit reviewed artifacts, generated source, metadata, and snapshot
- do not commit raw agent stdout/stderr logs by default

## Maintainer Lifecycles

### 1. Refresh an existing yearly entry

Use this when the lookup key and pipeline definition already exist.

Examples:

- `insurance/irmaa_brackets`
- `tax/federal_income_tax_brackets`

Recommended flow:

1. Check current status.
2. Run `run-agents`.
3. Read `review.md`.
4. Approve with `review --run <RUN_ID>` if needed.
5. Apply the approved result.
6. Check `status` again.

Example:

```sh
cargo run -p entropyfa-engine --bin data-pipeline -- run-agents \
  --year 2026 \
  --category insurance \
  --key irmaa_brackets

cargo run -p entropyfa-engine --bin data-pipeline -- review --run <RUN_ID>
cargo run -p entropyfa-engine --bin data-pipeline -- apply --run <RUN_ID>
```

This is the correct workflow for things like:

- 2026 IRMAA when CMS publishes the final table
- 2027 federal income tax brackets after the IRS revenue procedure is published

### 2. Create a new yearly pipeline

Use this when the public data entry exists or will exist, but no pipeline definition exists yet.

Examples:

- `tax/federal_standard_deductions`
- `tax/federal_payroll_tax_parameters`

The full process is:

1. Define the public data contract.
2. Add or confirm lookup coverage in the engine.
3. Add or update registry metadata.
4. Add a pipeline definition.
5. Reuse an existing validator/generator if possible; otherwise add new ones.
6. Run the normal agent workflow.
7. Review and apply.

Concretely:

1. Add the entry to the public lookup surface if it does not already exist.
   This usually means the data lookup path in [mod.rs](/Users/dan/dev/entropyfa-cli/engine/src/data/mod.rs), taxonomy/coverage, and tests.

2. Add metadata in [metadata.json](/Users/dan/dev/entropyfa-cli/engine/data_registry/2026/metadata.json).

3. Add a pipeline definition under `engine/data_registry/pipelines/<category>/<key>.json`.

4. Choose a `validation_profile`.
   Today the implemented options are the Rust enum variants in [data_pipeline.rs](/Users/dan/dev/entropyfa-cli/engine/src/data_pipeline.rs).

5. Choose a `generator_kind`.
   Today the implemented options are the Rust enum variants in [workflow.rs](/Users/dan/dev/entropyfa-cli/engine/src/data_pipeline/workflow.rs).

6. Set `target_source_path` to the Rust file that should be regenerated on `apply`.

7. Set `expected_variants` to the exact variant/parameter combinations that the public lookup contract should expose.

8. Set source policy:
   - `required_primary_hosts`
   - `allowed_supporting_hosts`
   - `allowed_secondary_hosts`
   - `minimum_secondary_confirmations`
   - `contract_notes` when the public contract has a documented representation convention that should not be mistaken for a schema mismatch

9. Seed `search_queries` with the exact terms you want the primary and verifier agents to start from.

10. Run:

```sh
cargo run -p entropyfa-engine --bin data-pipeline -- run-agents \
  --year 2026 \
  --category <category> \
  --key <key>
```

Rule of thumb:

- if the new entry looks like an existing yearly bracket or table shape, add a pipeline now
- if it needs a truly new output shape, stop and add the validator and generator first

### 3. Update a recurring yearly entry for a new year

This is a special case of the prior two flows.

Example:

- federal income tax brackets in January 2027

Checklist:

1. Add the new supported year to the public data layer.
2. Add or roll forward the new-year metadata.
3. Add the pipeline definition for that year if needed.
4. Run the agents.
5. Review and apply.

If the source shape is unchanged, this should feel like a normal refresh, not a redesign.

## Designing a Pipeline Definition

Each pipeline definition currently includes:

- `category`
- `key`
- `year_strategy`
- `supported_years`
- `validation_profile`
- `generator_kind`
- `target_source_path`
- `expected_variants`
- source host policy fields
- `search_queries`

Use the existing examples as templates:

- [irmaa_brackets.json](/Users/dan/dev/entropyfa-cli/engine/data_registry/pipelines/insurance/irmaa_brackets.json)
- [federal_income_tax_brackets.json](/Users/dan/dev/entropyfa-cli/engine/data_registry/pipelines/tax/federal_income_tax_brackets.json)

Pick the closest existing pattern first. Do not invent a new shape in the prompt pack if the Rust validator/generator layer cannot consume it.

## Source Policy Guidance

The workflow allows broad search, but only accepted sources on policy-approved hosts count toward final status.

Use this model:

- `required_primary_hosts` for the decisive official source
- `allowed_supporting_hosts` for official but non-decisive support
- `allowed_secondary_hosts` for curated corroborating sources
- `contract_notes` for entry-specific representation guidance that agents should treat as part of the current contract

For example:

- IRS tables: `required_primary_hosts = ["*.irs.gov"]`
- CMS IRMAA tables: `required_primary_hosts = ["*.cms.gov"]`, `allowed_supporting_hosts = ["*.medicare.gov", "*.ssa.gov"]`

Good secondary sources can support review and can sometimes justify `corroborated`, but they should not silently replace the primary source requirement for entries that have a clear federal publisher.

Each source record should correspond to exactly one actual URL. If a run relies on two SSA POMS pages, model them as separate `sources[]` entries with separate locators.

## Verification Policy

The prompt-pack workflow allows broad search, but accepted evidence is constrained by the per-entry pipeline definition in `engine/data_registry/pipelines/`.

Status meanings:

- `authoritative`: accepted primary official source
- `corroborated`: no primary source accepted, but enough curated secondary sources matched
- `derived`: transformed or subset data from known sources
- `placeholder`: carry-forward or estimate

Strict validation behavior:

- `authoritative`: no status warning
- `corroborated`: warning only, including under `--strict`
- `derived`: warning normally, error under `--strict`
- `placeholder`: warning normally, error under `--strict`
- `partial`: warning normally, error under `--strict`

## Snapshot / Validation Workflow

After an update, or any time you want to regression-check the embedded data:

1. Regenerate the reviewed snapshot:

   ```sh
   cargo run -p entropyfa-engine --bin data-pipeline -- snapshot --output engine/data_registry/2026/snapshot.json
   ```

2. Validate the registry against the live embedded data:

   ```sh
   cargo run -p entropyfa-engine --bin data-pipeline -- validate
   ```

3. Use strict mode when you want the pipeline to fail on non-authoritative or partial entries:

   ```sh
   cargo run -p entropyfa-engine --bin data-pipeline -- validate --strict
   ```

What it checks:

- every `data coverage` entry is represented in registry metadata
- the generated snapshot covers the same entry set
- live `engine::data::lookup()` output still matches the checked-in snapshot
- entry shapes remain sane: contiguous brackets, monotonic mortality tables, required fields present, and so on
- metadata caveats are surfaced according to the verification policy above

## Unsupported Cases and Next Design Steps

### State data such as NJ income tax

This is not just a missing pipeline definition. The current workflow is still federal/year-centric.

What is missing:

- a jurisdiction-aware public data contract
- parameters such as `state_code`
- likely metadata fields such as jurisdiction level/code
- pipeline definitions that understand state-level authority sources

So the right path for NJ income tax is:

1. define the state-aware lookup contract first
2. extend metadata, taxonomy, and lookup params
3. then add the NJ pipeline

Do not force state data into the current federal-only naming model.

### Monthly series such as AFR rates

The current pipeline definition model uses `year_strategy` and `supported_years`. It does not have a month or period dimension.

So monthly AFR support needs:

- a periodic data model, not just a new pipeline definition
- likely `year_month` or equivalent parameters
- metadata and snapshot support for recurring monthly values
- generators and validators for that shape

Do not treat monthly AFR as just another yearly entry. It needs a different persistence model.

## Current Caveats

Use `status` as the live source of truth for current gaps:

```sh
cargo run -p entropyfa-engine --bin data-pipeline -- status --plain
```

As of this writing, the known non-authoritative areas are:

- `insurance/irmaa_brackets` is marked `placeholder`
- `retirement/joint_life_table` is marked `derived` and `partial`
- `pension/mortality_417e` is marked `derived` and `partial`

That is deliberate until those entries are reviewed through the pipeline.
