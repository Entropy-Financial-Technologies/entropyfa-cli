# Projection Schema Contract Design

## Purpose

This spec defines a focused contract-hardening upgrade to:

```text
entropyfa compute projection --schema
```

The goal is not to change projection math. The goal is to make the command self-describing enough for an agent to discover:

- what inputs are accepted
- which fields are required vs optional
- how legacy inputs are normalized
- what output shape will be returned
- how linear detail differs from Monte Carlo detail

This is a schema and contract design only. It does not change simulation behavior.

## Current State

Today `compute projection --schema` already exposes useful agent-facing information:

- `command`
- `description`
- `when_to_use`
- `gather_from_user`
- `input_schema`
- `output_summary`

That is enough for basic discovery, but it still leaves important ambiguity:

- output shape is only summarized, not formally described
- normalization behavior is partly prose-only
- `request_echo` semantics are not explicit
- linear detail and Monte Carlo detail do not have fully documented row contracts
- nullable vs omitted vs empty-object behavior is not formalized

The current schema is therefore helpful, but not yet a reliable machine-readable source of truth for an agent.

## Product Goal

Make `compute projection --schema` the agent-grade source of truth for the projection command.

After this change, an agent should be able to:

- discover the request contract without trial-and-error
- discover the success and error envelopes without running sample calls first
- understand normalization/default behavior directly from schema output
- distinguish deterministic linear rows from aggregated Monte Carlo rows
- know which fields are optional, nullable, or omitted when empty

## Non-Goals

This design does not attempt to:

- redesign the projection command itself
- introduce the larger projection-platform rewrite
- replace the existing `input_schema`
- add new simulation features
- add a spreadsheet or export surface
- formalize every internal engine type in exhaustive JSON Schema syntax

This is intentionally a bounded schema-contract improvement.

## Approaches Considered

Three approaches were considered:

1. keep the current schema shape and improve prose
2. add machine-readable output and normalization sections beside the current input schema
3. build a fully versioned per-mode schema system with separate deep schemas for every output combination

Recommended approach: `2`

Why:

- it materially improves agent usability
- it keeps implementation bounded
- it preserves current human-readable guidance
- it avoids overengineering the command surface before the larger projection architecture is settled

## Recommended Contract Shape

`compute projection --schema` should keep the existing top-level success envelope but expand `data` to include four agent-first sections:

- `input_schema`
- `output_schema`
- `normalization_rules`
- `examples`

Existing descriptive sections should remain:

- `command`
- `description`
- `when_to_use`
- `gather_from_user`
- `output_summary`

Recommended top-level shape:

```json
{
  "ok": true,
  "data": {
    "command": "projection",
    "schema_version": "2.0",
    "description": "Run Monte Carlo and deterministic projection of portfolio balance over time",
    "when_to_use": "...",
    "modes": ["linear", "monte_carlo", "both"],
    "gather_from_user": {},
    "input_schema": {},
    "output_schema": {},
    "normalization_rules": {},
    "examples": {},
    "invariants": [],
    "output_summary": {}
  }
}
```

## Input Contract

The current `input_schema` is already directionally correct and should remain the main request definition.

It should continue to describe:

- legacy aggregate requests
- bucketed household requests
- typed `cash_flows`
- tax policy inputs
- RMD inputs
- detail controls

The main change is not the existence of `input_schema`. The main change is that its contract should be reinforced by explicit normalization rules and examples.

### Required Input Concepts

The schema should remain explicit that:

- `time_horizon_months` is always required
- a request must provide either:
  - `starting_balance` and `return_assumption`, or
  - `buckets`

### Optional Input Concepts

The schema should continue to expose:

- `mode`
- `num_simulations`
- `seed`
- `filing_status`
- `household`
- `spending_policy`
- `tax_policy`
- `rmd_policy`
- `cash_flows`
- `include_detail`
- `detail_granularity`
- `sample_paths`
- `path_indices`
- `custom_percentiles`

## Output Contract

The new `output_schema` should formally describe the actual response shape.

It should define:

- success envelope
- error envelope
- projection response object
- linear result object
- Monte Carlo result object
- deterministic detail row shape
- Monte Carlo detail row shape

### Success Envelope

```json
{
  "ok": true,
  "data": {
    "request_echo": {},
    "computation_time_ms": 0,
    "linear": {},
    "monte_carlo": {}
  }
}
```

### Error Envelope

```json
{
  "ok": false,
  "error": {
    "code": "string",
    "message": "string",
    "details": null
  }
}
```

### Projection Response

The top-level success `data` object should be documented as:

- `request_echo`
- `computation_time_ms`
- `linear`
- `monte_carlo`

#### `request_echo`

The schema must state explicitly:

- `request_echo` is the raw accepted request surface
- it is not the fully normalized internal engine state
- omitted legacy typed cash-flow fields may still appear as `null` here

This matters for agent interpretation.

### Linear Result

The `linear` section should be described with these fields:

- `final_balance`
- `time_series`
- `total_contributions`
- `total_withdrawals`
- `total_return_earned`
- `ending_balances_by_bucket`
- `annual_detail`

`linear` should be documented as:

- present when `mode` is `linear` or `both`
- omitted or `null` otherwise

### Monte Carlo Result

The `monte_carlo` section should be described with these fields:

- `num_simulations`
- `percentiles`
- `mean`
- `std_dev`
- `min`
- `max`
- `success_rate`
- `paths_depleted_by_month`
- `survival_by_year`
- `balance_histogram`
- `time_series`
- `annual_detail`
- `sample_paths`
- `custom_percentile_series`
- `bucket_terminal_percentiles`
- `bucket_depletion_counts`

`monte_carlo` should be documented as:

- present when `mode` is `monte_carlo` or `both`
- omitted or `null` otherwise

## Detail Row Contracts

The most important part of this design is making the detail-row semantics explicit.

### Linear Detail Row

The `output_schema` should define the deterministic row shape, including:

- `period`
- `year`
- `beginning_balance`
- `gross_inflows`
- `gross_outflows`
- `net_external_cash_flow`
- `contributions`
- `withdrawals`
- `investment_return`
- `annual_tax_paid`
- `taxable_ordinary_income`
- `taxable_capital_gains`
- `tax_year`
- `tax_data_mode`
- `modeled_from_year`
- `discretionary_withdrawals_by_bucket`
- `rmd_withdrawals_by_bucket`
- `tax_payment_withdrawals_by_bucket`
- `contributions_by_bucket`
- `bucket_withdrawals`
- `ending_balances_by_bucket`

The schema should state:

- linear detail rows represent one deterministic projection path
- they are the correct audit surface for row-by-row cash accounting

### Monte Carlo Detail Row

The `output_schema` should define the aggregated Monte Carlo row shape, including:

- `period`
- `year`
- `balance_p10`
- `balance_p25`
- `balance_p50`
- `balance_p75`
- `balance_p90`
- `gross_inflows`
- `gross_outflows`
- `net_cash_flow`
- `annual_tax_paid`
- `taxable_ordinary_income`
- `taxable_capital_gains`
- `tax_year`
- `tax_data_mode`
- `modeled_from_year`
- `survival_rate`

The schema must state explicitly:

- Monte Carlo detail rows are aggregated across simulation paths
- they are not one sampled path
- balance fields are percentile summaries
- tax fields are aggregated metrics for the period

This distinction is critical for agent correctness.

## Nullability And Omission Rules

The schema should document field behavior in one explicit place.

Recommended rules:

- `null` means the field exists conceptually but no value applies
- omitted means the field is suppressed when empty or not requested
- empty objects are omitted for bucket maps when nothing occurred
- detail arrays are omitted when `include_detail` is false
- `linear` is omitted or `null` when not requested by mode
- `monte_carlo` is omitted or `null` when not requested by mode

These rules should be written under `invariants` or a dedicated `field_behavior` section.

## Normalization Rules

The new `normalization_rules` section should turn today’s prose behavior into machine-readable guidance.

Recommended initial rules:

### Cash Flow Defaults

- if `direction` is omitted, infer from amount sign
- if `category` is omitted, infer:
  - positive -> `other_income`
  - negative -> `other`
- if `cash_character` is omitted, default to `cash`
- if `tax_treatment` is omitted, default to `ignore`

### Savings Routing

- `destination_bucket_id` only applies when:
  - `direction = outflow`
  - `category = savings`
  - `cash_character = cash`
- otherwise the request is invalid or the routing does not apply

### Tax Rules

- `embedded_federal` uses embedded data when the year is supported
- later years use modeled behavior
- `tax_data_mode` reports `authoritative` vs `modeled`
- `modeled_from_year` reports the last authoritative source year when applicable

### Detail Semantics

- linear detail is deterministic
- Monte Carlo detail is aggregated
- `request_echo` is raw request echo, not normalized output

## Examples

The new `examples` section should contain small executable payloads, not just prose.

Recommended examples:

- `minimal_legacy`
- `bucketed_household`
- `typed_cash_flows`
- `retirement_with_rmd`
- `monte_carlo_with_detail`

Each example should be:

- valid JSON
- intentionally small
- representative of one use case

## Invariants

The schema should expose a short list of non-negotiable invariants for agents:

- `time_horizon_months` is always required
- request must be legacy aggregate or bucketed household
- `request_echo` is raw request shape
- `linear` appears only for `linear` or `both`
- `monte_carlo` appears only for `monte_carlo` or `both`
- detail arrays appear only when `include_detail` is true
- Monte Carlo detail rows are aggregated summaries, not sampled paths
- `destination_bucket_id` routing applies only to cash savings outflows

## Compatibility Strategy

This change should be additive.

It should:

- preserve current `input_schema`
- preserve current `output_summary`
- preserve current prose guidance
- add formal sections without breaking existing consumers

That means current users of `--schema` should still work, while agents gain a much stronger contract.

## Implementation Outline

This spec implies a bounded implementation:

1. extend `cli/src/schema/simulation.rs`
2. add `schema_version`, `modes`, `output_schema`, `normalization_rules`, `examples`, and `invariants`
3. reuse existing response model field names instead of inventing schema-only names
4. add integration coverage asserting the new sections exist and describe the current contract correctly

No engine math changes are required.

## Success Criteria

This design is successful if:

- an agent can discover the valid request shape without trial requests
- an agent can discover the success and error envelopes from `--schema`
- an agent can distinguish deterministic and Monte Carlo detail semantics from the schema alone
- normalization behavior is explicit instead of hidden in prose
- the new schema remains additive and does not break existing consumers
