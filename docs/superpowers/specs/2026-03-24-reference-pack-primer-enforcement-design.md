# Reference Pack Primer Enforcement Design

## Goal

Make generated markdown reference packs materially more useful to an agent by requiring a reviewed primer section above the machine block. The primer must be authored by the primary agent, independently checked by the verifier, and block `apply` when it is missing or disputed.

## Problem

The current generated pack shape is too thin:

- front matter
- one generic introductory sentence
- machine block
- sources

That is enough for raw extraction, but not enough for a model that needs to understand what a dataset is, how to interpret it, and where its boundaries are. The current pipeline also has no mechanism for the verifier to review that explanatory layer separately from the numeric payload.

## Non-Goals

- do not turn packs into workflow scripts for the agent
- do not encode calculator-specific orchestration into the pack
- do not remove the machine block or sources
- do not redesign pipeline status, install, or calculator contracts in this slice

## Primer Contract

The primer must stay data-centric rather than workflow-centric.

Required sections:

- `What This Is`
- `Lookup Parameters`
- `Interpretation Notes`
- `Does Not Include`
- `Caveats`

Optional:

- `Typical Uses`

The intended meaning of each section:

- `What This Is`
  - short factual description of the dataset and what values it contains
- `Lookup Parameters`
  - only the parameters needed to select the right value or variant from this dataset
- `Interpretation Notes`
  - how to read the values: units, thresholds, inclusivity, stacking behavior, age semantics, effective-year behavior, and similar dataset semantics
- `Does Not Include`
  - nearby concepts an agent might incorrectly assume are included
- `Caveats`
  - uncertainty, interim guidance, source-shape limitations, and edge-case warnings
- `Typical Uses`
  - optional examples of where the dataset is commonly relevant; explicitly non-exclusive

## Artifact Contract

The primary submission must include a structured `reference_pack_primer` object. The verifier submission must include section-by-section verdicts for the same primer.

Recommended shape:

```json
{
  "reference_pack_primer": {
    "what_this_is": "Short factual paragraph.",
    "lookup_parameters": [
      "filing_status"
    ],
    "interpretation_notes": [
      "Bracket minimums are inclusive and maximums are exclusive except the final open-ended bracket."
    ],
    "does_not_include": [
      "Capital gains rates",
      "Payroll tax"
    ],
    "caveats": [
      "Interim guidance may change before final revenue procedure publication."
    ],
    "typical_uses": [
      "Estimating ordinary federal income tax"
    ]
  }
}
```

The verifier contract should mirror the required sections through a separate verdict object rather than trying to overload numeric `field_verdicts`.

Recommended shape:

```json
{
  "primer_verdicts": {
    "what_this_is": {
      "verdict": "confirm",
      "notes": ""
    },
    "lookup_parameters": {
      "verdict": "confirm",
      "notes": ""
    },
    "interpretation_notes": {
      "verdict": "confirm",
      "notes": ""
    },
    "does_not_include": {
      "verdict": "confirm",
      "notes": ""
    },
    "caveats": {
      "verdict": "confirm",
      "notes": ""
    },
    "typical_uses": {
      "verdict": "confirm",
      "notes": ""
    }
  }
}
```

`typical_uses` is optional in the primary artifact. If omitted there, the verifier section may be omitted as well.

## Review Rules

`review` must add blocking issues when:

- any required primer section is missing
- any required primer section is present but empty after trimming
- verifier omits a verdict for a required primer section
- verifier marks any required primer section as `dispute` or `uncertain`

`review` should also surface primer disagreement clearly in `review.md`, not bury it among numeric field issues.

`apply` should remain gated by the existing rule:

- approved review
- no blocking issues

That means primer failures automatically block `apply` without a separate special case.

## Pack Rendering

Generated packs should keep:

- front matter
- machine block
- sources

But the top of the file should become:

1. title
2. primer sections
3. machine block
4. sources

The rendered primer should use stable section headings:

- `## What This Is`
- `## Lookup Parameters`
- `## Interpretation Notes`
- `## Does Not Include`
- `## Caveats`
- optional `## Typical Uses`

Lists should render as bullets. `what_this_is` should render as prose.

## Prompt and Template Changes

The primary prompt must explicitly instruct the agent to:

- fill every required primer section
- keep the primer focused on dataset semantics, not generic financial planning advice

The verifier prompt must explicitly instruct the verifier to:

- independently review the primer for factual accuracy and fit
- dispute sections that overstate the dataset, omit essential limits, or describe the wrong lookup parameters

Template JSON files should include a primer skeleton so the agents can copy it directly.

## Testing

Add focused workflow tests that prove:

- `review` blocks when the primary submission omits the primer
- `review` blocks when the verifier disputes a required primer section
- `apply` writes a pack containing the required primer headings and values when review passes

Existing happy-path helpers should emit valid primer data so the broader workflow suite still represents the new contract.
