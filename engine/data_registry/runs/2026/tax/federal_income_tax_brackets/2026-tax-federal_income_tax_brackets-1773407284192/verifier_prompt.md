# Verifier Agent

Task: independently verify `tax/federal_income_tax_brackets` for year `2026`.

Read these files first:
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/source_policy.json`
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/current_value.json`
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/primary_output.json`
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/verifier_template.json`
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/verifier_report_template.md`

Write exactly two files:
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/verifier_output.json`
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/verifier_report.md`

Instructions:
1. Independently inspect the cited sources and any replacement official sources you find.
2. Start from `verifier_template.json`. Copy its structure exactly into `verifier_output.json` and preserve every key name.
3. Start from `verifier_report_template.md`. Preserve the headings, but fill it with freeform verification notes, disagreements, and caveats for a human reviewer.
4. Do not invent aliases or alternate shapes.
5. If the source material does not fit the current JSON schema cleanly, set `schema_change_required` to `true`, explain the mismatch in `schema_change_notes[]`, explain it again in `verifier_report.md`, and do not invent new JSON keys.
6. In `source_verdicts[]`, use this exact object shape:
`{"source_id","verdict","counts_toward_status","reason"}`.
7. `source_verdicts[].source_id` must match the exact `source_id` values from `primary_output.json`. Do not replace ids with URLs.
8. In `field_verdicts[]`, use this exact object shape:
`{"field_path","verdict","corrected_value","source_ids","notes"}`.
9. `field_path` values must match the exact required field paths from the template.
10. Every id in `field_verdicts[].source_ids` must match a `source_id` from `primary_output.json`.
11. Confirm, dispute, or mark uncertain each required field group in `field_verdicts[]`.
12. Recommend `authoritative`, `corroborated`, or `needs_human_attention`.
13. If anything is unresolved or inconsistent, set `overall_verdict` accordingly.

Required enums and literals:
- `source_verdicts[].verdict`: `accept` or `reject`
- `field_verdicts[].verdict`: `confirm`, `dispute`, or `uncertain`
- `status_recommendation`: `authoritative`, `corroborated`, or `needs_human_attention`
- `overall_verdict`: `pass`, `needs_human_attention`, or `reject`

Do not edit any Rust source, metadata, snapshot, or other repo files.
Do not write anything except `verifier_output.json` and `verifier_report.md`.

Pipeline details:
- pipeline: `federal_income_tax_brackets`
- required primary hosts: `*.irs.gov`
- allowed secondary hosts: `taxfoundation.org, www.taxfoundation.org, taxpolicycenter.org, www.taxpolicycenter.org, kiplinger.com, www.kiplinger.com`
