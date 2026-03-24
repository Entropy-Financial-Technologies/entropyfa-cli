# Run-Agents Auto-Repair Design

## Goal

Keep the one-command maintainer path fast without weakening trust.

`run-agents` should automatically repair and re-review runs when the verifier agrees the value is correct and the remaining issues are limited to safe artifact-quality defects such as primer wording or citation precision.

This is a follow-on to auto-apply:

- auto-apply handles clean runs
- auto-repair should handle safe-to-fix blocked runs
- human intervention should remain reserved for substantive disagreements

## Problem

The new primer-enforcement gate is doing the right thing by blocking overbroad or inaccurate guidance, but it currently stops the whole run even when:

- the value itself is correct
- accepted official sources are already sufficient
- the only defects are narrow wording or citation-quality issues

The estate exemption example is the model case:

- final value was correct
- status still reached `authoritative`
- review blocked only because the primer overstated scope and misstated portability filing language

That kind of run should not require a human.

## Scope

Change only the `run-agents` orchestration path and the machine-readable review/verifier contract needed to support bounded automatic repair.

Keep these commands manual and semantically unchanged:

- `prepare`
- `review`
- `apply`

Manual review remains the escape hatch.

## Design Principle

The system should auto-repair only when the disagreement is about how the reviewed result is described, not whether it is true.

So the trust boundary becomes:

- agent may repair wording, locators, and similar constrained artifacts
- verifier still decides whether the repaired run is acceptable
- humans only enter when the blocker concerns truth, source sufficiency, or schema fit

## Blocker Classes

The verifier/review path should classify every blocking issue into one of two buckets.

### Auto-Resolvable

These may be repaired automatically:

- primer scope overstatement
- primer wording too broad or imprecise
- missing required primer section
- blank required primer section
- caveat wording that is directionally right but factually imprecise
- conditional shorthand stated too strongly
- exact-source locator defect when the source itself is still accepted and sufficient
- similar artifact-shape issues that do not change the reviewed value

### Manual Required

These must stop and require human intervention:

- disputed field values
- uncertainty about the actual value
- required official source missing or rejected
- source-policy failure that affects status sufficiency
- contradictory primary sources
- `schema_change_required == true`
- shape mismatch between official source and current validator/generator
- any blocker that would require changing reviewed JSON values without clean verifier agreement

If any blocker is `manual_required`, `run-agents` must stop after review.

## Repair Loop

`run-agents` should evolve from:

1. prepare
2. primary
3. verifier
4. review
5. optionally auto-apply

to:

1. prepare
2. primary
3. verifier
4. review
5. if clean, auto-apply
6. if blocked only by auto-resolvable issues:
   - run a bounded repair pass
   - rerun verifier
   - rerun review
   - if now clean, auto-apply
7. if still blocked, stop

The loop should be tightly bounded:

- maximum 1 repair pass in the first slice
- optionally 2 later, but do not start there
- no open-ended iterative negotiation

## Repair Input Contract

The repair pass should not receive the entire run as a blank slate.

It should receive:

- the current reviewed value proposal
- accepted sources
- verifier disagreements
- exact disputed primer sections
- exact citation defects to repair
- explicit instruction that reviewed numeric values may not change unless the verifier flagged a value error

So the repair pass is not “do the entry again.”
It is “repair only these constrained defects.”

## Verifier Contract Changes

The verifier output should become more structured for blocker handling.

Each disputed item should expose:

- `issue_type`
- `auto_resolvable`
- `repair_guidance`

Examples:

- `issue_type: primer_scope_overstatement`
- `issue_type: primer_factual_imprecision`
- `issue_type: citation_locator_inexact`
- `issue_type: value_dispute`
- `issue_type: source_policy_failure`
- `issue_type: schema_mismatch`

And for primer sections:

- the verifier should continue giving section-level verdicts
- but also provide repair guidance specific to each disputed section

For example:

- tighten `what_this_is` so it describes the statutory basic exclusion amount, not the case-specific tax-free amount of an estate
- remove unconditional `$30,000,000` shorthand or mark it clearly as conditional portability shorthand
- correct portability caveat to say the estate representative files Form 706 for the decedent

## Repair Safety Rules

The repair pass may:

- rewrite primer prose
- add or tighten caveats
- narrow overbroad statements
- fix source locators
- remove unsupported shorthand

The repair pass may not:

- change reviewed numeric values unless the verifier explicitly classified the issue as a value defect
- change verification status upward on its own
- add new unofficial sources to satisfy an official-source requirement
- bypass schema-change blocks
- overwrite unresolved value disputes

## Review Outcome Changes

Review should surface whether a blocked run is eligible for auto-repair.

Recommended additions:

- `all_blockers_auto_resolvable: true|false`
- `auto_repair_eligible: true|false`
- `manual_required_blockers`
- `auto_resolvable_blockers`

That lets `run-agents` make one deterministic decision:

- all blockers auto-resolvable -> repair once
- otherwise stop

## CLI Output

`run-agents` should report which path happened:

- `auto_applied: true|false`
- `auto_repair_attempted: true|false`
- `auto_repair_succeeded: true|false`

When repair was attempted, print:

- repaired artifact paths
- re-review path
- final apply artifact paths if successful

If repair failed, keep the manual next-step guidance.

## Testing

Add coverage for:

- blocked run with only primer wording disputes triggers one repair pass
- repaired run is re-verified, re-reviewed, and auto-applied on success
- blocked run with value dispute does not attempt repair
- blocked run with source-policy failure does not attempt repair
- blocked run with schema change required does not attempt repair
- repair pass cannot mutate reviewed numeric values in the safe-repair path
- CLI output reports repair attempt and final disposition

## Non-Goals

- do not let repair silently bless disputed values
- do not replace the independent verifier
- do not remove primer enforcement
- do not make `review` itself auto-repair or auto-apply
- do not add an open-ended multi-agent debate loop

## First Slice Recommendation

Start narrowly.

First slice should support auto-repair only for:

- disputed required primer sections
- missing/blank required primer sections
- citation locator defects that do not affect accepted-source sufficiency

That is enough to cover the most common low-risk blocks, including the estate exemption example, without broadening the trust surface too quickly.
