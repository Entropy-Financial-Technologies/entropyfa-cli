# Run-Agents Auto-Apply Design

## Goal

Make the normal maintainer path one command: `run-agents` should automatically apply a run when the independent review passes cleanly.

## Scope

Change only the orchestration behavior of `run-agents`.

Keep these commands manual and semantically unchanged:

- `prepare`
- `review`
- `apply`

That preserves the current debugging and inspection escape hatch.

## Desired Behavior

`run-agents` should continue to do:

1. prepare the run
2. execute the primary agent
3. execute the verifier agent
4. generate the review outcome

Then:

- if review has no blocking issues, is approved, and the recommended action is `apply_approved_result`, automatically run `apply`
- if review has blocking issues, stop after review and do not apply
- if review recommends any non-apply action, stop after review and do not apply

This makes the happy path fully automatic while preserving the current failure path for human inspection.

## Approval Model

`run-agents` should not prompt for human approval in the success case.

Instead, the orchestration path should review with automatic approval only when the review is clean enough to apply. That keeps the command non-interactive and aligned with the user’s stated preference.

Manual `review --run ...` should keep its current approval semantics.

## Output Contract

`run-agents` CLI output should explicitly report whether auto-apply happened.

Recommended additions:

- `auto_applied: true|false`
- when true, print the same apply artifact paths currently shown by `apply`
- when false, keep printing the review paths and the next-step guidance

The outcome type in the workflow layer should carry optional apply results so the CLI can report them cleanly.

## Safety Boundaries

Auto-apply should require all of these:

- `review.approved == true`
- `review.blocking_issues.is_empty()`
- `review.recommended_action == apply_approved_result`

Any other condition must leave the run at the review stage.

This is intentionally conservative.

## Testing

Add coverage for:

- `run-agents` auto-applies on a clean happy path
- `run-agents` does not auto-apply when review has blocking issues
- `run-agents` output includes `auto_applied: true` and the apply artifact paths on success
- existing delayed-output and failed-review paths still stop at review

## Non-Goals

- do not change `review` to auto-apply
- do not remove the standalone `apply` command
- do not change the underlying review policy or primer enforcement rules
