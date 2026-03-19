# Bucketed Monte Carlo Design

## Purpose

This spec defines a focused upgrade to `entropyfa compute projection`.

The goal is not to turn the current Monte Carlo calculator into the full projection-platform rewrite. The goal is to make the existing simulator materially more realistic and advisor-useful with a bounded set of changes:

- multiple account buckets instead of one pooled balance
- real embedded federal tax logic instead of a flat tax drag assumption
- withdrawal order / liquidation preference
- retirement cash-flow handling such as RMDs
- support for both accumulation and decumulation in one model

This is intended as a practical product improvement that can ship before any larger projection-engine rewrite is resumed.

## Current State

Today the Monte Carlo path is still a simple pooled-balance simulator:

- one `starting_balance`
- one `return_assumption`
- one monthly path balance
- generic periodic cash flows
- no account-bucket depletion logic
- no household federal tax layer
- no RMD handling

That simplicity is useful, but it makes the tool weaker for real retirement planning and weaker than the level of realism advisors expect when comparing accumulation vs retirement-withdrawal paths.

## Product Goal

Upgrade the existing Monte Carlo tool into a lightweight household retirement simulator while keeping it operationally simple.

The upgraded simulator should answer questions like:

- what is the success rate if the household contributes before retirement and withdraws after retirement?
- what happens if spending is funded from taxable first, then IRA, then Roth?
- how much does federal tax drag reduce success?
- how do required minimum distributions affect depletion risk?

The simulator should remain:

- fast
- deterministic for a given seed
- inspectable in JSON
- simpler than the proposed future projection-engine rewrite

## Non-Goals

This design does not attempt to do the following:

- replace the future projection platform
- add a full ledger-style projection row model
- add position-level or tax-lot-level holdings
- add monthly tax filing realism or withholding reconciliation detail
- model full healthcare planning, Medicare, or IRMAA inside Monte Carlo v1
- model state tax
- model actual taxable lot harvesting

Those remain future opportunities. This spec is deliberately smaller.

## Recommended Approach

Three broad options were considered:

1. keep the pooled simulator and add a tax heuristic
2. upgrade to a bucketed household simulator with embedded tax logic
3. build a full mini projection engine inside Monte Carlo

Recommended option: `2`

Why:

- it materially improves realism
- it supports both accumulation and retirement-spending cases
- it introduces account depletion logic, which is the biggest current product gap
- it avoids reintroducing the projection-engine complexity that is currently being shelved

## Core Model

The new Monte Carlo path should model a household portfolio as a small set of account buckets rather than a single pooled portfolio.

### Bucket Types

V1 supports:

- `taxable`
- `tax_deferred`
- `tax_free`
- `cash` (optional)

Each bucket should carry:

- `id`
- `bucket_type`
- `starting_balance`
- `return_assumption`
- optional `withdrawal_priority`
- optional contribution or withdrawal targeting metadata
- optional taxable-bucket gain realization assumptions

The design intentionally stays at the bucket level rather than per account or per position.

## Supported Use Cases

The same bucketed model should handle both:

### Accumulation

- wages or other earned income support contributions
- portfolio grows stochastically
- contributions are directed into selected buckets

### Decumulation

- spending or retirement cash needs are modeled as household outflows
- external inflows such as Social Security or pension offset spending
- remaining cash need is funded from bucket withdrawals according to policy
- required distributions are forced when applicable

This avoids splitting the product into separate accumulation and retirement Monte Carlo tools.

## Input Shape

The current single-balance request should evolve into a household bucket request.

Recommended shape:

```json
{
  "mode": "both",
  "num_simulations": 10000,
  "seed": 1,
  "time_horizon_months": 360,
  "filing_status": "married_filing_jointly",
  "household": {
    "birth_years": [1965, 1967],
    "retirement_month": 24
  },
  "buckets": [
    {
      "id": "taxable",
      "bucket_type": "taxable",
      "starting_balance": 500000,
      "return_assumption": { "annual_mean": 0.06, "annual_std_dev": 0.14 },
      "realized_gain_ratio": 0.65,
      "withdrawal_priority": 2
    },
    {
      "id": "ira",
      "bucket_type": "tax_deferred",
      "starting_balance": 900000,
      "return_assumption": { "annual_mean": 0.06, "annual_std_dev": 0.14 },
      "withdrawal_priority": 3
    },
    {
      "id": "roth",
      "bucket_type": "tax_free",
      "starting_balance": 200000,
      "return_assumption": { "annual_mean": 0.06, "annual_std_dev": 0.14 },
      "withdrawal_priority": 4
    }
  ],
  "cash_flows": [
    {
      "type": "employment_income",
      "amount": 180000,
      "frequency": "annual",
      "start_month": 0,
      "end_month": 24,
      "tax_treatment": "ordinary_income"
    },
    {
      "type": "spending",
      "amount": -9000,
      "frequency": "monthly",
      "start_month": 0
    }
  ],
  "spending_policy": {
    "withdrawal_order": ["cash", "taxable", "tax_deferred", "tax_free"],
    "rebalance_tax_withholding_from": "cash"
  },
  "tax_policy": {
    "mode": "embedded_federal",
    "modeled_tax_inflation_rate": 0.025
  },
  "rmd_policy": {
    "enabled": true,
    "distribution_month": 12
  }
}
```

## Required vs Optional Inputs

### Required

- `time_horizon_months`
- `filing_status`
- `buckets`
- `spending_policy`
- `tax_policy`

### Usually Required

- `household.birth_years` whenever age-based rules like RMDs are enabled
- `cash_flows` for most realistic runs

### Optional

- `mode`
- `num_simulations`
- `seed`
- `rmd_policy`
- visualization and detail controls
- custom percentile controls

## Tax Logic

This upgrade should use the existing embedded federal tax logic rather than a simple effective tax assumption.

### Supported Tax Treatment

V1 should model:

- earned income and pensions as ordinary income
- `tax_deferred` withdrawals as ordinary income
- `tax_free` withdrawals as non-taxable
- `taxable` withdrawals as partly basis and partly gain using a configurable `realized_gain_ratio`

### Timing

Paths still run monthly, but household tax should be computed annually from accumulated year-to-date aggregates.

That means:

- income and withdrawals are tracked during the year
- tax is computed at year-end
- tax is then paid as an outflow from the configured source bucket or household cash bucket

This is not full monthly withholding realism, but it is a strong and understandable approximation for Monte Carlo v1.

### Supported-Year Policy

When reviewed embedded tax data exists for a year:

- use it directly

When the path extends past the last supported year:

- explicitly model thresholds and parameters forward using `modeled_tax_inflation_rate`

The output should state where authoritative support stopped and modeled tax handling began.

## Withdrawal and Liquidation Logic

This is the most important functional enhancement in the design.

The simulator should no longer assume a generic pooled outflow from one balance. Instead:

- household cash need is computed from inflows and outflows
- if net household cash is negative, the deficit is funded from buckets in the configured withdrawal order
- depletion moves bucket by bucket
- failed funding after all eligible buckets are exhausted marks the path as failed

This is the primary mechanism that makes the simulator materially more realistic.

## RMD Handling

V1 should support required minimum distributions for `tax_deferred` buckets.

Behavior:

- use the reviewed retirement distribution rules and life-table data already shipped in the repo
- compute annual RMD when the household reaches applicable ages
- force the RMD withdrawal from eligible `tax_deferred` buckets in the configured distribution month
- treat the forced withdrawal as ordinary income for annual tax
- make the resulting cash available to cover spending, taxes, or remain in the `cash` bucket

This should be automatic when `rmd_policy.enabled` is true and the required age inputs are present.

## Monthly Path Execution

Each simulation path should follow a stable order of operations per month:

1. apply simulated return to each bucket
2. apply scheduled bucket-specific contributions and external inflows
3. apply scheduled household outflows
4. if the month is the configured RMD month, generate forced `tax_deferred` withdrawals first
5. if household cash is negative, fund the deficit using the withdrawal order
6. accumulate annual tax aggregates
7. at year-end, compute household federal tax and pay it from the configured source
8. if all eligible buckets are exhausted and required outflows cannot be funded, mark the path as failed

This keeps the simulator simple while still introducing retirement-aware behavior.

## Output Enhancements

The Monte Carlo outputs can stay in the current family, but they should become bucket-aware.

Recommended additions:

- ending balances by bucket for percentile summaries
- depletion diagnostics by bucket
- annual tax totals in detail output
- annual required-distribution totals in detail output
- metadata indicating authoritative vs modeled tax years

The command does not need to adopt the full future projection-engine envelope. It can stay compatible with the current Monte Carlo response family while becoming richer.

## Success Definition

Default success should remain intuitive:

- success = the household can fund required outflows through the full horizon
- failure = required outflows cannot be funded once all eligible buckets are exhausted

Additional diagnostics should include:

- first failure month
- depleted bucket order
- median ending balance by bucket

## Product Positioning

This design should make `compute projection` stronger than a simple mean/vol toy without forcing a whole-platform rewrite.

It improves advisor usefulness by introducing:

- actual household tax treatment
- realistic liquidation sequencing
- retirement distribution behavior
- both accumulation and decumulation support in one tool

That is a meaningful product step even if it remains simpler than a full ledger-based planning engine.

## Risks and Simplifications

Key simplifications that should be stated clearly in docs and responses:

- bucket-level modeling, not account-lot realism
- taxable gain realization uses an assumption, not actual lots
- annual tax settlement, not fully realistic monthly withholding
- no Medicare or IRMAA modeling in Monte Carlo v1
- no state tax

These are acceptable because the purpose of this design is bounded realism, not full planning-engine fidelity.

## Recommended Sequence

1. extend the request model from `starting_balance` to `buckets`
2. introduce bucket-level path simulation
3. add household withdrawal-order logic
4. add annual household federal tax handling using embedded tax data
5. add RMD handling for `tax_deferred` buckets
6. enrich response detail and documentation

## Success Criteria

This upgrade is successful if:

- the command can model both accumulation and decumulation with one request shape
- withdrawals deplete configured buckets in the expected order
- annual federal tax materially affects success and ending balances
- RMDs appear automatically when enabled
- the output is still fast, understandable, and inspectable
- the result is substantially more realistic than the current pooled simulator without becoming the shelved full projection rewrite
