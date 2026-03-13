# Verifier Review Report

## Overall Assessment
- entry: `tax/federal_income_tax_brackets`
- year: `2026`
- overall_verdict: pass
- status_recommendation: authoritative
- schema_change_required: false
- summary: The current value is correct. I independently verified all five filing-status schedules against Rev. Proc. 2025-32 using official IRS materials, including the cited PDF source and an official IRS Internal Revenue Bulletin mirror (`2025-45_IRB`, section `RP-2025-32`) that exposes the full tables as HTML text. That replacement official source removes the original run's PDF-parsing limitation. The only material disagreement I found is in `src_tf_1`, which appears to list the head-of-household 24% upper breakpoint as `$201,775`; the official IRS value is `$201,750`, which matches `current_value.json`.

## Source Review
- `src_irs_1` accept | counts_toward_status: true
- reason: Complete authoritative source. Rev. Proc. 2025-32 section 4.01 Tables 1-5 provides the full 2026 bracket schedules and matches the current value exactly.

- `src_irs_2` accept | counts_toward_status: false
- reason: Official IRS newsroom summary and accurate for the single and married-filing-jointly tables, but incomplete for full verification because it does not publish the full set of tables used by this schema.

- `src_tf_1` reject | counts_toward_status: false
- reason: The Tax Foundation 2026 table appears to disagree with the IRS on the head-of-household 24% breakpoint, showing `$201,775` where the IRS revenue procedure shows `$201,750`.

- `src_doeren_1` accept | counts_toward_status: false
- reason: Its tables match the IRS figures, but the host is outside the allowed secondary-host policy, so it cannot contribute to status. I also observed that the live page date appears to be `2025-11-04`, not `2025-10-10` as recorded in `primary_output.json`.

## Field Review
- `variants[single].value`: confirm
  - source_ids: `src_irs_1`
  - notes: Matches Rev. Proc. 2025-32 section 4.01 Table 1 exactly: 10% to `$12,400`, 12% to `$50,400`, 22% to `$105,700`, 24% to `$201,775`, 32% to `$256,225`, 35% to `$640,600`, then 37%.

- `variants[married_filing_jointly].value`: confirm
  - source_ids: `src_irs_1`
  - notes: Matches Rev. Proc. 2025-32 section 4.01 Table 1 exactly: 10% to `$24,800`, 12% to `$100,800`, 22% to `$211,400`, 24% to `$403,550`, 32% to `$512,450`, 35% to `$768,700`, then 37%.

- `variants[married_filing_separately].value`: confirm
  - source_ids: `src_irs_1`
  - notes: Matches Rev. Proc. 2025-32 section 4.01 Table 3 exactly: 10% to `$12,400`, 12% to `$50,400`, 22% to `$105,700`, 24% to `$201,775`, 32% to `$256,225`, 35% to `$384,350`, then 37%.

- `variants[head_of_household].value`: confirm
  - source_ids: `src_irs_1`
  - notes: Matches Rev. Proc. 2025-32 section 4.01 Table 4 exactly: 10% to `$17,700`, 12% to `$67,450`, 22% to `$105,700`, 24% to `$201,750`, 32% to `$256,200`, 35% to `$640,600`, then 37%. This is the field where `src_tf_1` appears to be wrong.

- `variants[qualifying_surviving_spouse].value`: confirm
  - source_ids: `src_irs_1`
  - notes: Confirmed from Rev. Proc. 2025-32 section 4.01 Table 1, which explicitly applies to married individuals filing joint returns and surviving spouses. The schedule is identical to married filing jointly and matches `current_value.json`.

## Disagreements Or Caveats
- `src_tf_1` appears materially inconsistent with the IRS for the head-of-household 24% breakpoint. I would not use it as corroboration for this entry without a corrected table.
- `src_irs_2` is reliable within its scope, but it is only a summary release. The complete verification should rest on `src_irs_1` or the official IRS bulletin mirror of the same revenue procedure.
- `src_doeren_1` matches the official values, but it is not on the approved secondary-host list and should remain non-counting.
- I did not find a second approved secondary source in the allowed-host set that added clean value over the IRS primary document. That is not blocking here because the accepted IRS primary source is sufficient for an `authoritative` recommendation under the policy semantics used by the pipeline.
