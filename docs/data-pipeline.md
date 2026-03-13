# Data Pipeline

The embedded reference data is part of the product surface. The maintainer workflow now has two layers:

1. `prepare` / `review` / `apply` for agent-assisted source verification and code generation
2. `snapshot` / `validate` for post-update regression checking against the live embedded data

## Agent Workflow

Use this when you want Claude Code or Codex to help refresh an entry such as
`insurance/irmaa_brackets` or `tax/federal_income_tax_brackets`.

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

You should then inspect the generated `review.md` and, if it looks correct, run `apply` manually.

If you omit the explicit model flags, `run-agents` now defaults to Claude `claude-opus-4-6` for the primary pass and Codex `gpt-5.4` for the verifier pass.

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

Each run now includes:

- strict JSON templates for machine-readable output
- markdown report templates for human-readable evidence packets
- `current_value.json` for comparison only, not as truth

If either agent concludes the official source no longer fits the current lookup contract, it must set `schema_change_required: true`. Review will then block `apply` until the schema, validator, and generator are updated deliberately.

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
- entry shapes remain sane: contiguous brackets, monotonic mortality tables, required fields present, etc.
- metadata caveats are surfaced according to the verification policy above

## Current Caveats

The initial 2026 dataset still has a few non-authoritative areas:

- `insurance/irmaa_brackets` is marked `placeholder`
- `retirement/joint_life_table` is marked `derived` and `partial`
- `pension/mortality_417e` is marked `derived` and `partial`

That is deliberate until those entries are reviewed through the pipeline.
