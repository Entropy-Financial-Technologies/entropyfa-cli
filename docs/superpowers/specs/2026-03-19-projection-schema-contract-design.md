# Projection Schema Contract Design

## Purpose

This spec defines a focused contract-hardening upgrade to:

```text
entropyfa compute projection --schema
```

The goal is not to change projection math. The goal is to make the command self-describing enough for an agent to discover:

- what inputs are accepted
- which fields are required vs optional
- how CLI and assembly defaults affect the request
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
- `example`
- `related_commands`

That is enough for basic discovery, but it still leaves important ambiguity:

- output shape is only summarized, not formally described
- request assembly/default behavior is implicit in CLI code
- `request_echo` semantics are not explicit
- linear detail and Monte Carlo detail do not have fully documented row contracts
- nullable vs omitted vs empty-object behavior is not formalized

The current schema is therefore helpful, but not yet a reliable machine-readable source of truth for an agent.

## Product Goal

Make `compute projection --schema` the agent-grade source of truth for the current projection command.

After this change, an agent should be able to:

- discover the request contract without trial-and-error
- discover the success and error envelopes without running sample calls first
- understand CLI/default behavior directly from schema output
- distinguish deterministic linear rows from aggregated Monte Carlo rows
- know which fields are optional, nullable, or omitted when empty

## Non-Goals

This design does not attempt to:

- redesign the projection command itself
- introduce the larger projection-platform rewrite
- add new simulation features
- replace the current `input_schema`
- replace the current `example` or `related_commands` keys
- formalize every internal engine type in exhaustive JSON Schema syntax

This is intentionally a bounded schema-contract improvement.

## Approaches Considered

Three approaches were considered:

1. keep the current schema shape and improve prose only
2. add machine-readable output and normalization sections beside the current schema
3. build a fully versioned per-mode schema system with separate deep schemas for every output combination

Recommended approach: `2`

Why:

- it materially improves agent usability
- it keeps implementation bounded
- it preserves current human-readable guidance
- it avoids overengineering the command surface before the larger projection architecture is settled

## Recommended Contract Shape

`compute projection --schema` should keep the existing top-level success envelope and preserve the current top-level keys:

- `command`
- `description`
- `when_to_use`
- `gather_from_user`
- `input_schema`
- `output_summary`
- `example`
- `related_commands`

It should add these agent-first sections:

- `output_schema`
- `normalization_rules`
- `invariants`

The design should remain additive. It should not silently remove existing keys.

Recommended top-level shape:

```json
{
  "ok": true,
  "data": {
    "command": "projection",
    "description": "Run Monte Carlo and deterministic projection of portfolio balance over time",
    "when_to_use": "...",
    "gather_from_user": {},
    "input_schema": {},
    "output_schema": {},
    "normalization_rules": {},
    "invariants": [],
    "output_summary": {},
    "example": {},
    "related_commands": []
  }
}
```

## Input Contract

The current `input_schema` is already directionally correct and should remain the main request definition.

It should continue to describe:

- legacy aggregate requests
- bucketed household requests
- periodic `cash_flows`
- tax policy inputs
- RMD inputs
- detail controls

The main change is not the existence of `input_schema`. The main change is that its contract should be reinforced by explicit normalization rules and a formal output contract.

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

The new `output_schema` should formally describe the actual response shape returned on `main`.

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
    "message": "string"
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

- `request_echo` is the assembled `SimulationRequest`
- it reflects CLI defaults and CLI flag overrides before compute
- it is not the raw JSON payload
- it is not a separate internal engine-debug view

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
- `contributions`
- `withdrawals`
- `investment_return`
- `ending_balance`
- `cumulative_contributions`
- `cumulative_withdrawals`
- `cumulative_return`
- `annual_tax_paid`
- `bucket_withdrawals`
- `ending_balances_by_bucket`

The schema should state:

- linear detail rows represent one deterministic projection path
- they are the current public row-level detail surface for projection

### Monte Carlo Detail Row

The `output_schema` should define the aggregated Monte Carlo row shape, including:

- `period`
- `year`
- `balance_p10`
- `balance_p25`
- `balance_p50`
- `balance_p75`
- `balance_p90`
- `net_cash_flow`
- `cumulative_cash_flow`
- `annual_tax_paid`
- `survival_rate`

The schema must state explicitly:

- Monte Carlo detail rows are aggregated across simulation paths
- they are not one sampled path
- balance fields are percentile summaries
- `net_cash_flow` and `annual_tax_paid` are aggregated metrics for the period

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

The new `normalization_rules` section should turn today’s implicit command behavior into machine-readable guidance.

Recommended initial rules:

### CLI Defaults

- if `mode` is omitted, the CLI defaults it to `both`
- CLI flags override the same JSON fields when both are provided
- `--detail` sets `include_detail = true`
- `--detail-granularity` overrides JSON `detail_granularity`
- `--sample-paths`, `--path-indices`, and `--percentiles` override the same JSON fields
- when `--visual` is used with Monte Carlo output, the CLI injects percentile bands needed by the terminal dashboard

### Assembly Surface

- `request_echo` reflects the assembled request object after CLI overrides
- it is the public request model, not a normalized internal trace view
- current `cash_flows` support:
  - `amount`
  - `frequency`
  - `start_month`
  - `end_month`

### Tax Rules

- `embedded_federal` uses embedded data when the year is supported
- later years use modeled behavior inside compute
- the current public response on `main` does not expose authoritative-vs-modeled markers in detail rows

### Detail Semantics

- linear detail is deterministic
- Monte Carlo detail is aggregated
- `request_echo` is the assembled request object, not the original raw JSON text

## Examples

The current `example` field should be preserved.

This spec does not require replacing it with a plural `examples` field yet. The bounded goal is to improve the contract without forcing a broader schema redesign.

If future implementation wants to add more examples, that should be additive and should not remove the current `example` field.

## Invariants

The schema should expose a short list of non-negotiable invariants for agents:

- `time_horizon_months` is always required
- request must be legacy aggregate or bucketed household
- `request_echo` is the assembled request shape
- `linear` appears only for `linear` or `both`
- `monte_carlo` appears only for `monte_carlo` or `both`
- detail arrays appear only when `include_detail` is true
- Monte Carlo detail rows are aggregated summaries, not sampled paths
- `--visual` can mutate `custom_percentiles` for dashboard rendering

## Compatibility Strategy

This change should be additive.

It should:

- preserve current `input_schema`
- preserve current `output_summary`
- preserve current prose guidance
- preserve `example`
- preserve `related_commands`
- add formal sections without breaking existing consumers

That means current users of `--schema` should still work, while agents gain a much stronger contract.

## Implementation Outline

This spec implies a bounded implementation:

1. extend `cli/src/schema/simulation.rs`
2. add `output_schema`, `normalization_rules`, and `invariants`
3. preserve `example` and `related_commands`
4. reuse existing response model field names instead of inventing schema-only names
5. add integration coverage asserting the new sections exist and describe the current contract correctly

No engine math changes are required.

## Success Criteria

This design is successful if:

- an agent can discover the valid request shape without trial requests
- an agent can discover the success and error envelopes from `--schema`
- an agent can distinguish deterministic and Monte Carlo detail semantics from the schema alone
- CLI/default behavior is explicit instead of hidden in command code
- the new schema remains additive and does not break existing consumers
