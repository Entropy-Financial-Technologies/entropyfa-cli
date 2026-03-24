# Reference Packs And Compute Layer Design

## Purpose

This spec resets `entropyfa-cli` around a simpler agent-first architecture:

- reviewed yearly reference packs in markdown
- deterministic financial calculators
- agent-assembled inputs and assumptions

The current CLI has grown into both a lookup surface and a compute surface. This spec narrows it back down. The CLI should become the financial planning compute layer, not the main reference-data product.

## Core Decision

The new boundary is:

- the app owns client facts
- markdown reference packs own yearly tax/rule knowledge
- the agent assembles facts plus assumptions
- the CLI runs deterministic calculations

This means the repo is no longer optimized around `data lookup` as the primary product surface.

## Why Change

The current structure is useful, but it is carrying too many responsibilities at once:

- embedded reviewed data artifacts
- CLI lookup contracts
- CLI compute contracts
- docs that mostly explain CLI responses

That is more machinery than the agent actually needs.

The agent mostly needs:

1. to know what a calculator requires
2. to read trusted reference material
3. to gather client facts
4. to run deterministic math

That is better served by reference packs plus calculator contracts than by a CLI-centered lookup architecture.

## Goals

The redesign should achieve all of the following:

- make reviewed markdown reference packs the main human/agent knowledge surface
- keep deterministic calculators for tax, retirement, and projection math
- make the agent responsible for assembling assumptions and overrides
- make calculator schemas describe facts, assumptions, overrides, and outputs
- simplify the repo so the CLI is the compute layer rather than the encyclopedia
- allow explicit assumption overrides when the user or agent wants something other than the default reference pack assumptions

## Non-Goals

This redesign does not attempt to:

- preserve backward compatibility with the current CLI lookup surface
- keep `data lookup` as a first-class product surface if it no longer serves the architecture
- force every calculation into a large new projection platform rewrite
- let the agent do critical tax or retirement math freeform

## Recommended Architecture

The repo should be organized into four layers.

### 1. Reference Packs

Canonical yearly reference material should live in markdown packs under a new top-level directory such as:

```text
reference/
  tax/
    2026/
      federal_income_tax_brackets.md
      federal_standard_deductions.md
      federal_capital_gains_brackets.md
  insurance/
    2026/
      irmaa_brackets.md
      medicare_base_premiums.md
  retirement/
    2026/
      uniform_lifetime_table.md
      distribution_rules.md
```

Each pack should include:

- a strict machine-readable data block
- a short primer
- required parameters
- caveats and edge cases
- source citations
- review metadata

The pack should be usable by both:

- an agent reading the primer/context
- code extracting the structured data block

### 2. Deterministic Compute Layer

The CLI should remain for calculators such as:

- federal tax
- estate tax
- RMD
- Roth conversion
- pension comparison
- Monte Carlo projection

These calculators should accept explicit input contracts and return explicit output contracts.

They should not be responsible for broad reference-data discovery or silent assumption assembly.

### 3. Calculator Schemas

Each calculator should expose a schema that tells the agent:

- required client facts
- optional inputs
- supported overrides
- output shape
- recommended reference packs to read

The calculator schema should become the primary machine-readable contract.

The agent flow should be:

1. inspect calculator schema
2. gather client facts from the app
3. read the relevant markdown reference packs
4. decide which assumptions to pass
5. run the calculator
6. explain the result using echoed inputs and assumptions

### 4. Review Pipeline

The pipeline should be repointed so the reviewed output is the markdown reference pack itself.

That means the verification workflow should:

1. gather proposed values and citations
2. independently verify them
3. write or update the reviewed markdown pack
4. validate the structured data block
5. regenerate any small compute cache only if needed

The pipeline should stop thinking of JSON lookup artifacts as the main public output.

## Reference Pack Format

The markdown pack should combine structured data and primer text in one artifact.

It should not be freeform prose with numbers embedded unpredictably.

A reference pack should have a rigid front matter or fenced machine block, for example:

```md
---
category: tax
key: federal_income_tax_brackets
year: 2026
required_params:
  - filing_status
verification_status: authoritative
review_status: reviewed
sources:
  - authority: Internal Revenue Service
    title: Revenue Procedure 2025-32
    url: https://www.irs.gov/pub/irs-drop/rp-25-32.pdf
data:
  single:
    - { min: 0.0, max: 12400.0, rate: 0.10 }
    - { min: 12400.0, max: 50400.0, rate: 0.12 }
---

# Federal Income Tax Brackets

Use for ordinary income bracket math.
Requires `filing_status`.
Does not include payroll tax, NIIT, or capital gains stacking.

## Caveats

- Pair with standard deduction and capital gains brackets when computing full federal tax.
```

This gives one canonical artifact with:

- strict data for code
- primer text for the agent
- citations and caveats for auditability

## Responsibility Split

The clean responsibility split should be:

### App

Owns persistent client facts:

- household members
- ages
- filing status
- income
- assets and liabilities
- cash flows
- retirement dates

### Agent

Owns orchestration:

- read calculator schema
- determine missing client facts
- read relevant reference packs
- choose assumptions
- provide explicit overrides when needed

### Calculator

Owns deterministic math:

- validate contract
- run tax / retirement / projection calculations
- echo inputs used
- echo assumptions used

### Reference Packs

Own yearly domain knowledge:

- official values
- parameter requirements
- caveats
- review provenance

## Calculator Contract Style

Calculator schemas should no longer focus on hiding default behavior inside the CLI.

They should instead expose:

- `required_inputs`
- `optional_inputs`
- `supported_overrides`
- `output_contract`
- `recommended_reference_packs`

This makes the system more flexible without losing auditability.

Example:

```json
{
  "command": "federal-tax",
  "required_inputs": [
    "tax_year",
    "filing_status",
    "income"
  ],
  "optional_inputs": [
    "deductions",
    "credits",
    "payroll_inputs"
  ],
  "supported_overrides": [
    "tax_brackets",
    "standard_deduction",
    "capital_gains_brackets",
    "niit_thresholds"
  ],
  "recommended_reference_packs": [
    "reference/tax/2026/federal_income_tax_brackets.md",
    "reference/tax/2026/federal_standard_deductions.md",
    "reference/tax/2026/federal_capital_gains_brackets.md"
  ]
}
```

The calculator should not silently decide all assumptions. The agent should decide what to pass.

## Determinism And Flexibility

The design should remain deterministic where it matters:

- the calculator runs the same way for the same explicit inputs
- the final output echoes the resolved assumptions used

But the architecture should also allow flexibility:

- the agent can supply assumptions from the reference packs
- the agent can override them explicitly if the user wants an alternative case
- the agent can compare runs with different assumptions without changing the calculator itself

This is intentionally less “magic defaulting” than the current lookup-plus-compute model.

That is a feature, not a bug.

## What Gets Removed Or Demoted

The following should be reduced or removed from center stage:

- `data lookup` as the primary public product surface
- docs organized mainly around CLI response envelopes for reference data
- pipeline outputs whose main purpose is feeding CLI lookup
- calculator behavior that silently auto-loads too much reference context

What remains valuable:

- deterministic compute commands
- `--schema` for calculators
- yearly source verification
- machine-readable contracts for compute outputs

## Migration Plan

This should be treated as a deliberate reset, not a compatibility-preserving refactor.

### Phase 1: Define The New Canonical Artifact

Create the markdown reference-pack format and move a small core set of datasets first:

- `tax/federal_income_tax_brackets`
- `tax/federal_standard_deductions`
- `tax/federal_capital_gains_brackets`
- `insurance/irmaa_brackets`
- `insurance/medicare_base_premiums`
- `retirement/uniform_lifetime_table`
- `retirement/distribution_rules`

### Phase 2: Repoint The Pipeline

Change the review/apply workflow so the canonical reviewed output is the markdown pack.

Keep:

- primary pass
- verifier pass
- human approval
- validation of structured data

Change:

- reviewed target artifact
- maintainer docs
- generated outputs

### Phase 3: Narrow The CLI

Make the CLI primarily a compute layer:

- keep `compute ...`
- keep `compute ... --schema`
- de-emphasize or remove `data lookup`

### Phase 4: Simplify Calculator Contracts

Stop making calculators auto-resolve large assumption sets from embedded data.

Instead:

- agent reads schema
- agent reads reference packs
- agent passes assumptions explicitly
- calculator computes deterministically

### Phase 5: Rewrite Docs Around The New Model

Shift the docs from:

- “CLI lookup surface”

to:

- “reference packs”
- “calculator contracts”
- “pipeline for reviewed markdown packs”

## First Vertical Slice

The first slice should be:

- `federal-tax`
- a small set of tax reference packs
- a pipeline path that writes those packs
- an updated `compute federal-tax --schema` contract that references them

This is the smallest slice that proves:

- the markdown pack format
- the agent-assembled calculator model
- the pipeline redirect
- the simpler repo architecture

## Risks

The main risks are:

### 1. Too Little Structure In Markdown

If the pack format is too loose, code will end up scraping prose.

Mitigation:

- require rigid front matter or a rigid fenced machine block
- validate it strictly

### 2. Too Much Burden On The Agent

If calculators stop owning every default, the agent does more assembly work.

Mitigation:

- make schemas explicit
- provide recommended reference-pack paths
- echo resolved assumptions in outputs

### 3. Half-Migrated Repo Confusion

If both the old lookup model and the new reference-pack model linger too long, the repo will feel inconsistent.

Mitigation:

- choose a firm migration order
- rewrite docs aggressively
- treat compatibility as optional, not sacred

## Success Criteria

This redesign is successful if:

- the repo is easier to understand than the current lookup-plus-compute shape
- agents can reason from markdown reference packs directly
- calculators remain deterministic and auditable
- the pipeline writes reviewed markdown packs instead of lookup-first artifacts
- the CLI is clearly the compute layer rather than the main knowledge surface
- changing assumptions becomes an explicit agent action rather than hidden CLI behavior

