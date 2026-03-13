# Primary Extraction Agent

Task: verify `tax/federal_income_tax_brackets` for year `2026`.

You may search broadly for official and credible corroborating sources, but only the hosts listed in `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/source_policy.json` count toward final review status.

Read these files first:
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/current_value.json`
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/source_policy.json`
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/primary_template.json`
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/primary_report_template.md`

Write exactly two files:
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/primary_output.json`
- `/Users/dan/dev/entropyfa-cli/engine/data_registry/runs/2026/tax/federal_income_tax_brackets/2026-tax-federal_income_tax_brackets-1773407284192/primary_report.md`

Instructions:
1. Search broadly for candidate sources.
2. Prefer official primary sources. Secondary sources are allowed only as corroborating evidence.
3. Start from `primary_template.json`. Copy its structure exactly into `primary_output.json` and preserve every key name.
4. Start from `primary_report_template.md`. Preserve the headings, but fill it with freeform evidence, tables, and narrative that help a human reviewer understand the source material.
5. Do not invent aliases. Use `source_class`, not `type`. Use `published_at`, not `accessed`. Use `source_id`, not a URL in place of an id.
6. Do not treat `current_value.json` as truth. It is only the previous embedded value for comparison.
7. If the official source does not fit the current JSON schema cleanly, set `schema_change_required` to `true`, explain the mismatch in `schema_change_notes[]`, explain it again in `primary_report.md`, and do not invent new JSON keys.
8. Populate `sources[]` with every source you relied on using this exact object shape:
`{"source_id","url","host","organization","source_class","title","published_at","locator","notes"}`.
9. Choose stable source ids like `src_cms_1`, `src_ssa_1`, `src_kff_1`. They must be unique within the file.
10. Update `value_proposal` with extracted values in the exact lookup shape already shown in the template.
11. Populate `field_evidence[]` for every required field group using this exact object shape:
`{"field_path","source_id","locator"}`.
12. `field_path` values must match the exact paths already implied by the template, for example ``variants[single].value` and `variants[married_filing_jointly].value``.
13. Every `field_evidence.source_id` must reference one of the ids you created in `sources[]`.
14. Record any uncertainty in `unresolved_issues[]`.

Required enums and literals:
- `proposed_status`: `authoritative`, `corroborated`, `derived`, or `placeholder`
- `source_class`: `primary`, `supporting_official`, or `secondary`

Do not edit any Rust source, metadata, snapshot, or other repo files.
Do not write anything except `primary_output.json` and `primary_report.md`.

Pipeline details:
- pipeline: `federal_income_tax_brackets`
- expected variants: `single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse`
- search queries: `Revenue Procedure 2025-32 2026 federal income tax brackets IRS | 2026 federal income tax rate tables IRS revenue procedure 2025-32 | 2026 ordinary income tax brackets IRS filing status`
