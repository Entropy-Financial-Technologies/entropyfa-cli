# Projection Platform Rewrite Design

## Purpose

This spec defines the target rewrite of `entropyfa`'s forward-looking planning surface into one shared projection platform.

The goal is not to keep adding calculator-shaped APIs. The goal is to make every forward-looking planner compile into the same projection engine, emit the same canonical run envelope, and support the same agent and human inspection model.

This spec covers:

- the engine boundary
- the public command contract
- the generic program model
- the canonical run envelope
- the first executable slice
- the migration of current planners onto the platform

## Product Direction

The platform should be:

- agent-first in operation
- human-inspectable in outputs
- generic rather than calculator-shaped
- explicit about provenance, assumptions, and modeled behavior

The engine should not be organized around fixed product names like `retirement_projection` or `pension_comparison`.

Instead:

- the public command is `projection run`
- the main executable object is a generic `program`
- optional `template` contracts provide reusable planner-specific presets
- the engine runs programs
- planner-specific summaries are derived views on top of the same run

## Locked Decisions

These decisions are locked for this design unless there is a strong reason to reopen them:

- this is a full-platform rewrite direction, not an adapter-first migration
- v1 is a clean break from current forward-looking command contracts
- all forward-looking planners return one canonical run envelope
- v1 compute grain is household/account only
- v1 output requirement is JSON only
- annual mode ships first
- monthly mode is designed for from day one, but not required in v1
- authoritative yearly data is used whenever available, with explicit modeled behavior after the last supported year
- templates are structured machine-readable contracts first, with markdown guidance layered on top

## Public Command Model

The public engine-native command is:

```text
entropyfa projection run
```

The request shape is centered on a generic `program`, not a calculator enum.

Recommended v1 shape:

```json
{
  "template": "roth_conversion_strategy",
  "program": {
    "facts": {},
    "policies": {},
    "actions": [],
    "goals": [],
    "assumptions": {},
    "variants": []
  },
  "output": {
    "detail_level": "summary",
    "include_tax_detail": true,
    "include_trace": true
  }
}
```

The truly engine-native form omits `template` entirely:

```json
{
  "program": {},
  "output": {}
}
```

The meaning of each layer is:

- `template`: optional reusable contract for agent convenience and planner presets
- `program`: the executable scenario definition
- `output`: inspection and verbosity controls

## Program Model

The `program` should be expressive enough to support today's planners and future ones without renaming the public surface.

### Facts

The facts layer represents the current state of the world.

Initial v1 facts should support:

- household
- people
- filing relationships
- accounts
- account balances
- summarized income sources
- summarized outflows
- liabilities when needed

Future facts can add:

- positions
- tax lots
- entity structures
- trust structures
- insurance products
- real estate detail

### Policies

Policies describe ongoing rules applied by the engine.

Examples:

- withdrawal order
- tax handling policy
- healthcare premium policy
- growth policy
- solver policy
- modeled-after-last-supported-year policy

### Actions

Actions are explicit scenario instructions.

Examples:

- transfer
- convert
- withdraw
- contribute
- elect pension option
- claim Social Security
- change retirement age

### Goals

Goals are optional. Not every run is goal-seeking.

Examples:

- preserve terminal balance threshold
- cover spending through a target age
- minimize lifetime tax
- compare variants against a success criterion

### Assumptions

Assumptions carry modeling inputs that are not direct facts.

Examples:

- return assumptions
- inflation assumptions
- cash-flow growth assumptions
- modeled future tax inflation
- modeled future healthcare inflation

### Variants

Variants allow a single run to compare alternate strategies using the same facts base.

Examples:

- pension annuity vs rollover vs cash-out
- convert vs do nothing
- claim at one age vs another

## Canonical Run Envelope

Every forward-looking planner should return the same top-level run shape.

Recommended structure:

```json
{
  "run": {},
  "scenarios": [],
  "periods": [],
  "rows": [],
  "snapshots": [],
  "detail": {
    "tax": [],
    "healthcare": [],
    "trace": []
  },
  "summary": {}
}
```

### Run

The run metadata should include:

- `run_id`
- command and template metadata
- request provenance
- supported authoritative years
- modeled-after-last-supported-year markers
- assumption provenance

### Scenarios

Scenarios define the base path and any variants in the run.

Fields should include:

- `scenario_id`
- `label`
- `kind`
- `is_baseline`
- template metadata when relevant

### Periods

One record per period per scenario.

Fields should include:

- `scenario_id`
- `period_grain`
- `period_index`
- `period_start`
- `period_end`
- `is_authoritative_period`
- `modeled_from_year`

### Rows

Rows are the normalized movement ledger.

V1 rows should support:

- `row_id`
- `scenario_id`
- `period_index`
- `scope_type`
- `scope_id`
- `row_kind`
- `component`
- `amount`
- `direction`
- `currency`
- `parent_row_id`
- `rollup_key`
- `calc_source`
- `policy_ref`
- `assumption_ref`
- `human_label`
- `agent_label`

V1 executable scopes:

- `household`
- `account`

Schema-reserved future scopes:

- `person`
- `position`
- `tax_lot`

### Snapshots

Snapshots represent end-of-period state rather than movement.

V1 fields should include:

- `snapshot_id`
- `scenario_id`
- `period_index`
- `scope_type`
- `scope_id`
- `balance`
- `metadata`

Future snapshots can add:

- basis
- unrealized gain
- bucket balances
- lot-level state

### Detail Tables

Detail tables are normalized supporting views, not disconnected side outputs.

V1 detail tables should include:

- `tax`
- `trace`

`healthcare` should exist in the envelope even if some v1 runs do not populate it yet.

### Summary

Summary is always derived from the normalized run. It must not contain unique logic unavailable from rows, snapshots, and detail tables.

That allows:

- agent explanation
- human reconciliation
- future CSV export
- future spreadsheet views

## Execution Model

The engine is period-based from day one.

Supported grains:

- `year` in v1
- `month` later

The execution order for each period is:

1. open period snapshots
2. apply contractual inflows and outflows
3. apply explicit strategy actions and transfers
4. recognize taxable income and gains/losses
5. compute household tax and healthcare premiums
6. apply returns and valuation changes
7. write closing snapshots and rollups

This order is stable and shared across planners.

## First Executable Slice

The first executable slice should be intentionally simple but architecturally real.

It should be a linear portfolio projection program with:

- starting balances
- inflows
- outflows
- growth assumptions
- household federal tax
- authoritative yearly tax data when supported
- explicit modeled behavior after the last supported year
- canonical rows, snapshots, detail tables, and summary

This slice is not the full product. It is the first real proof that the engine contract and outputs are good enough to carry the rest of the planners.

It should remain simple in scope:

- no monthly execution yet
- no position or tax-lot execution yet
- no spreadsheet UI yet
- no planner-specific custom contracts yet

## Template Model

Templates are not markdown prompts pretending to be APIs.

Templates should be:

- structured contracts
- versioned
- machine-readable
- agent-friendly

Markdown can still exist, but only as:

- explanation
- usage guidance
- examples
- implementation notes

Examples of initial templates:

- linear portfolio
- Roth conversion strategy
- pension comparison

Each template should define:

- required facts
- supported actions
- supported policies
- expected derived summary sections

## Migration of Current Planners

The current forward-looking surface is small enough that v1 should move all of it onto the engine rather than preserving parallel bespoke loops.

### V1 Completion Rule

V1 does not count as complete until all of these are running on the engine:

- current `projection`
- current `roth-conversion-strategy`
- current `pension-comparison`

### Migration Phases

#### Phase A: Platform foundation

Build:

- generic `projection run`
- program model
- canonical run envelope
- annual engine kernel
- linear portfolio executable slice

#### Phase B: Migrate projection

Replace the existing projection command behavior with:

- a `program` or template compile step
- a shared engine run
- a derived summary section

#### Phase C: Migrate Roth conversion strategy

Convert Roth strategy into:

- a template or contract
- a scenario compiler
- a shared engine run with multiple variants or periods
- derived Roth-specific summaries from the run

#### Phase D: Migrate pension comparison

Convert pension comparison into:

- a template or contract
- multi-variant scenario compilation
- shared engine execution
- derived comparison summaries from the run

#### Phase E: Remove bespoke execution loops

After the migrated planners are stable:

- remove duplicate forward-looking execution code
- keep only scenario compilers and derived summary logic above the engine

## Data Provenance and Modeling Rules

The engine must be explicit about where its numbers come from.

For any period or detail computation:

- use authoritative embedded data if the year is supported
- if the run extends beyond the last supported year, switch to explicit modeled behavior
- record that switch in run metadata and relevant detail outputs

This rule should apply consistently across:

- tax data
- healthcare data
- future retirement policy data

The first executable slice should use real household federal tax treatment under this exact rule.

## V1 Non-Goals

The following are designed for, but not required in v1:

- monthly execution
- spreadsheet-style inspection UI
- CSV export
- position execution
- tax-lot execution
- broad state-tax modeling
- full planner library beyond the current forward-looking surface

## Testing Expectations for the Rewrite

The platform rewrite should be validated at four levels:

1. schema and contract tests for `program`, templates, and run envelope
2. engine tests for period execution order, row generation, snapshots, and provenance behavior
3. planner-template tests ensuring current planners compile into valid programs
4. end-to-end command tests for `projection run` across the first executable slice and migrated planners

The key invariant is:

- no summary number should exist that cannot be traced back to the canonical run

## Success Criteria

This design succeeds if:

- agents can call one public projection command across planning problems
- humans can inspect the same run without hidden planner-specific logic
- the first executable slice proves the run envelope on a real projection
- `projection`, `roth-conversion-strategy`, and `pension-comparison` all migrate onto the same engine
- future planners can be added as templates and compilers rather than new execution stacks
