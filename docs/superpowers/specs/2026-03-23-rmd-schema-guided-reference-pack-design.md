# RMD Schema-Guided Reference Pack Design

## Purpose

This spec defines the first concrete calculator refactor for the new agent-first model:

- the agent inspects the calculator schema
- the schema tells the agent what client facts are required
- the schema tells the agent which reference packs to read
- the agent combines client facts, reference knowledge, and any scenario assumptions
- the calculator performs deterministic math and reports provenance

`compute rmd` is the right first slice because it has a clear boundary:

- client-specific facts are narrow and well understood
- statutory rules are important and should not be improvised
- the current contract is too heavy because it requires the caller to pass the full `rmd_parameters` object

This refactor should make `compute rmd` the prototype for how future calculators work.

## Problem

The current `compute rmd` contract is structurally correct but poor for agent orchestration.

Today:

- the schema says the caller must provide `rmd_parameters`
- runtime validation requires the full RMD tables and rules object
- the built-in rule data exists in the repo, but the calculator contract does not guide the agent to it cleanly
- the happy-path example is misleading because it omits `rmd_parameters` even though runtime currently requires it

That creates the wrong ergonomics:

- too much low-value plumbing for the agent
- poor discoverability of the real rule sources
- schema and usability are out of sync

## Goals

This refactor should:

- make `compute rmd --schema` the authoritative guide for agent assembly
- stop requiring the caller to inline the full RMD rule tables on normal runs
- make the reference-pack requirements explicit and readable
- keep client facts and scenario choices agent-supplied
- allow explicit overrides for nonstandard hypotheticals
- make the response report which reference packs and assumptions were used

## Non-Goals

This refactor does not attempt to:

- redesign every calculator at once
- make the agent perform freeform retirement math
- remove the possibility of explicit rule overrides
- solve multi-account household withdrawal planning
- redesign `rmd-schedule` in the same change, beyond keeping it compatible with the same rule-loading model where practical

## Core Decision

`compute rmd` should use a schema-guided agent assembly model.

The clean split is:

- `entropyfa` ships and versions the statutory rule packs
- the agent reads those packs directly from disk
- the agent supplies client facts
- the agent supplies optional assumptions and overrides
- the calculator validates, loads, computes, and reports provenance

This is intentionally not one of the two extremes:

- not caller-supplies-everything
- not calculator-hides-everything

It is a guided middle path that is transparent and practical.

## Responsibility Split

### Reference Packs

Reference packs own:

- uniform lifetime table
- joint life and last survivor table
- single life expectancy table
- required beginning date rules
- account-type rules
- beneficiary rules
- ten-year rule behavior
- any review metadata and citations

These should ship as readable markdown artifacts under a stable directory such as:

```text
reference/
  retirement/
    2026/
      uniform_lifetime_table.md
      joint_life_table.md
      single_life_expectancy_table.md
      distribution_rules.md
```

### Agent

The agent owns:

- discovering the calculator contract from `--schema`
- gathering required client facts
- reading the listed reference packs
- deciding whether any optional overrides are needed
- assembling the request

### Calculator

The calculator owns:

- validating the assembled request
- resolving the referenced rule packs
- performing the deterministic RMD math
- returning the result plus provenance

## New Calculator Contract

The schema for `compute rmd` should stop treating the entire rule set as a required raw input blob.

Instead it should expose five first-class sections:

- `required_client_facts`
- `reference_requirements`
- `optional_assumptions`
- `optional_overrides`
- `output_schema`

The schema should remain returned through:

```sh
entropyfa compute rmd --schema
```

## Required Client Facts

The schema should declare the core facts the agent must provide:

- `calculation_year`
- `prior_year_end_balance`
- `account_type`
- `owner_birth_date`

Conditionally required facts should remain explicit:

- `spouse_birth_date`
- `spouse_is_sole_beneficiary`
- `beneficiary_birth_date`
- `beneficiary_class`
- `owner_is_alive`
- `owner_death_year`
- `is_still_working`
- `is_five_percent_owner`
- `pre_1987_403b_balance`

These are client or scenario facts, not reference-pack content.

## Reference Requirements

The schema should list the packs needed for a standard RMD run.

Proposed shape:

```json
{
  "reference_requirements": [
    {
      "id": "uniform_lifetime_table",
      "path": "reference/retirement/2026/uniform_lifetime_table.md",
      "required": true,
      "purpose": "Determine owner-lifetime divisor by attained age"
    },
    {
      "id": "joint_life_table",
      "path": "reference/retirement/2026/joint_life_table.md",
      "required": true,
      "purpose": "Determine divisor when spouse is sole beneficiary and substantially younger"
    },
    {
      "id": "single_life_expectancy_table",
      "path": "reference/retirement/2026/single_life_expectancy_table.md",
      "required": true,
      "purpose": "Determine inherited-account divisor where applicable"
    },
    {
      "id": "distribution_rules",
      "path": "reference/retirement/2026/distribution_rules.md",
      "required": true,
      "purpose": "Determine start age, account-specific rule path, and beneficiary handling"
    }
  ]
}
```

This is guidance for the agent. It does not require the agent to serialize the markdown packs back into the request on normal runs.

## Input Shape

The runtime request should move to a lighter normal form.

### Normal Request

Normal agent-driven calls should look like:

```json
{
  "calculation_year": 2026,
  "prior_year_end_balance": 500000,
  "account_type": "traditional_ira",
  "owner_birth_date": "1953-06-15"
}
```

### Optional Assumptions

The schema should expose optional calculator-level assumptions such as:

- `distribution_month`

These are not reference rules and not core client facts.

### Optional Overrides

The schema should allow an explicit override channel for nonstandard use cases.

Proposed examples:

- `override_uniform_lifetime_table`
- `override_joint_life_table`
- `override_single_life_expectancy_table`
- `override_distribution_rules`

The override path should be explicit and visibly different from the normal path so the result can report that a nonstandard rule source was used.

## Reference Resolution Model

For `compute rmd`, the calculator should resolve rule packs locally using the installed reference root.

Resolution order should be:

1. explicit override payload supplied in the request
2. local installed reference pack under the resolved reference root
3. fail with a clear error if neither exists

This preserves the main model:

- normal case: installed reviewed packs
- special case: explicit override by the agent

It does not require the agent to pass raw rule tables each run.

## Output Contract

The response should keep the existing result fields where they still make sense:

- `rmd_required`
- `rmd_amount`
- `distribution_period`
- `table_used`
- `rule_path`
- `decision_trace`

But it should add provenance sections:

- `references_used`
- `assumptions_used`
- `overrides_used`

Proposed response shape:

```json
{
  "ok": true,
  "data": {
    "result": {
      "rmd_required": true,
      "rmd_amount": 18867.92,
      "distribution_period": 26.5,
      "table_used": "uniform_lifetime_table",
      "rule_path": "owner_lifetime.standard_owner",
      "decision_trace": ["..."]
    },
    "references_used": [
      {
        "id": "uniform_lifetime_table",
        "path": "reference/retirement/2026/uniform_lifetime_table.md",
        "version": "2026.1"
      },
      {
        "id": "distribution_rules",
        "path": "reference/retirement/2026/distribution_rules.md",
        "version": "2026.1"
      }
    ],
    "assumptions_used": {
      "distribution_month": 12
    },
    "overrides_used": {}
  }
}
```

If an override was used, `overrides_used` should say so explicitly and `references_used` should not pretend the standard pack alone governed the result.

## Schema Semantics

The schema should make these rules explicit:

- reference packs are expected to exist on disk under the resolved reference root
- the agent is expected to read them directly
- the runtime request normally includes client facts, not the full rule corpus
- overrides are optional and explicit
- failure to resolve required packs should return a structured error

This means the schema becomes a guide for agent assembly, not just a raw validation summary.

## Error Handling

The calculator should fail clearly in these cases:

- required client facts missing
- required reference pack missing
- reference pack found but malformed
- override payload malformed
- conflicting beneficiary / account facts

These should return structured errors such as:

- `validation_error`
- `reference_pack_missing`
- `reference_pack_invalid`
- `override_invalid`

The error message should tell the agent exactly what was missing or invalid.

## Migration Strategy

This should be implemented in one calculator-first slice.

### Phase 1

Refactor `compute rmd`:

- new schema shape
- local reference-pack loading
- lighter normal request contract
- provenance in response

### Phase 2

Apply the same pattern to `compute rmd-schedule`.

### Phase 3

Use the lessons from `rmd` and `rmd-schedule` to refactor tax and other retirement calculators.

This avoids trying to redesign every calculator at once.

## Testing

The implementation should prove:

- `compute rmd --schema` exposes client facts, reference requirements, assumptions, overrides, and output schema clearly
- normal RMD runs succeed with installed reference packs and without inline `rmd_parameters`
- explicit override runs succeed and report overrides used
- missing-pack failures are structured and precise
- provenance accurately reflects the rule sources used

## Recommended User Experience

The intended agent workflow should be:

1. run `entropyfa compute rmd --schema`
2. read the listed reference packs from disk
3. gather client facts from the app
4. provide optional overrides only when needed
5. run `entropyfa compute rmd --json ...`
6. explain the result using `decision_trace`, `references_used`, and `overrides_used`

That is the model this first calculator should prove.
