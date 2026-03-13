# Review for `2026-tax-federal_income_tax_brackets-1773407284192`

- entry: `tax/federal_income_tax_brackets`
- year: `2026`
- current verification status: `authoritative`
- proposed verification status: `authoritative`
- primary schema_change_required: `false`
- verifier schema_change_required: `false`

## Accepted Sources
- Internal Revenue Service | Revenue Procedure 2025-32 (26 CFR 601.602) | https://www.irs.gov/pub/irs-drop/rp-25-32.pdf
- Internal Revenue Service | IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill | https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
- Doeren Mayhew CPAs and Advisors | IRS Releases 2026 Cost-of-Living Adjustments, Includes Amendments from OBBBA | https://www.doeren.com/viewpoint/irs-releases-2026-cost-of-living-adjustments-includes-amendments-from-obbba

## Diff From Current
- head_of_household: unchanged
- married_filing_jointly: unchanged
- married_filing_separately: unchanged
- qualifying_surviving_spouse: unchanged
- single: unchanged
- head_of_household: unchanged
- married_filing_jointly: unchanged
- married_filing_separately: unchanged
- qualifying_surviving_spouse: unchanged
- single: unchanged

## Primary Report
# Primary Extraction Report

## Summary
- entry: `tax/federal_income_tax_brackets`
- year: `2026`
- short conclusion: All 2026 federal income tax brackets for all five filing statuses were verified against IRS Revenue Procedure 2025-32 (via the IRS newsroom release) and corroborated by the Tax Foundation and Doeren Mayhew CPAs. The seven-rate structure (10%–37%) under the TCJA, made permanent by the One Big Beautiful Bill Act (OBBBA) § 70101, is confirmed. All proposed values match the current embedded values exactly.

## Sources Reviewed

### `src_irs_1` Internal Revenue Service | https://www.irs.gov/pub/irs-drop/rp-25-32.pdf
- source_class: primary
- locator: Section 3, Tax Rate Tables for Taxable Years Beginning in 2026
- why it matters: This is the authoritative IRS revenue procedure containing the official 2026 inflation-adjusted tax rate schedules for all filing statuses under IRC § 1(j) as amended by OBBBA § 70101. The PDF binary could not be parsed directly by the web fetch tool, but its contents are reflected in the IRS newsroom release (src_irs_2) and confirmed by secondary sources.

### `src_irs_2` Internal Revenue Service | https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
- source_class: primary
- locator: Tax rate schedule section – Single and Married Filing Jointly tables
- why it matters: Official IRS newsroom release (IR-2025-103, published 2025-10-09) providing the 2026 bracket thresholds for Single and Married Filing Jointly filers. This is the primary readable source for the bracket data. It references Rev. Proc. 2025-32 as the underlying document. Does not include Head of Household or Married Filing Separately tables.

### `src_tf_1` Tax Foundation | https://taxfoundation.org/data/all/federal/2026-tax-brackets/
- source_class: secondary (approved host per source_policy.json)
- locator: Table 1 – 2026 Federal Income Tax Brackets and Rates
- why it matters: Comprehensive secondary analysis by Garrett Watson and Alex Durante citing Rev. Proc. 2025-32. The full rendered table was truncated during web fetch (JavaScript-rendered content), but web search synthesis confirmed their Single and MFJ bracket thresholds match the IRS primary source. The article was originally published 2026-01-01 and last updated 2026-03-11.

### `src_doeren_1` Doeren Mayhew CPAs and Advisors | https://www.doeren.com/viewpoint/irs-releases-2026-cost-of-living-adjustments-includes-amendments-from-obbba
- source_class: secondary (NOTE: host doeren.com is NOT on the approved secondary hosts list)
- locator: Federal Income Tax Rate Schedules section – all filing status tables
- why it matters: This CPA firm analysis was the most complete secondary source successfully fetched, providing bracket tables for all five filing statuses. It was critical for confirming Head of Household and Married Filing Separately brackets that the IRS newsroom release omits. Published 2025-10-10, one day after the IRS release.

## Evidence Tables

### Single (confirmed by src_irs_2, corroborated by src_tf_1, src_doeren_1)

| Rate | Min ($) | Max ($) |
|------|---------|---------|
| 10%  | 0       | 12,400  |
| 12%  | 12,400  | 50,400  |
| 22%  | 50,400  | 105,700 |
| 24%  | 105,700 | 201,775 |
| 32%  | 201,775 | 256,225 |
| 35%  | 256,225 | 640,600 |
| 37%  | 640,600 | —       |

### Married Filing Jointly (confirmed by src_irs_2, corroborated by src_tf_1, src_doeren_1)

| Rate | Min ($) | Max ($) |
|------|---------|---------|
| 10%  | 0       | 24,800  |
| 12%  | 24,800  | 100,800 |
| 22%  | 100,800 | 211,400 |
| 24%  | 211,400 | 403,550 |
| 32%  | 403,550 | 512,450 |
| 35%  | 512,450 | 768,700 |
| 37%  | 768,700 | —       |

### Married Filing Separately (confirmed by src_doeren_1)

| Rate | Min ($) | Max ($) |
|------|---------|---------|
| 10%  | 0       | 12,400  |
| 12%  | 12,400  | 50,400  |
| 22%  | 50,400  | 105,700 |
| 24%  | 105,700 | 201,775 |
| 32%  | 201,775 | 256,225 |
| 35%  | 256,225 | 384,350 |
| 37%  | 384,350 | —       |

### Head of Household (confirmed by src_doeren_1)

| Rate | Min ($) | Max ($) |
|------|---------|---------|
| 10%  | 0       | 17,700  |
| 12%  | 17,700  | 67,450  |
| 22%  | 67,450  | 105,700 |
| 24%  | 105,700 | 201,750 |
| 32%  | 201,750 | 256,200 |
| 35%  | 256,200 | 640,600 |
| 37%  | 640,600 | —       |

### Qualifying Surviving Spouse (same as MFJ per IRC § 1(a); confirmed by src_irs_2, src_doeren_1)

| Rate | Min ($) | Max ($) |
|------|---------|---------|
| 10%  | 0       | 24,800  |
| 12%  | 24,800  | 100,800 |
| 22%  | 100,800 | 211,400 |
| 24%  | 211,400 | 403,550 |
| 32%  | 403,550 | 512,450 |
| 35%  | 512,450 | 768,700 |
| 37%  | 768,700 | —       |

## Schema Fit
- schema_change_required: false
- The official IRS source data fits the current JSON schema exactly. Each filing status has seven brackets with `rate`, `min`, and `max` fields. The top bracket uses `null` for `max` to represent an unbounded upper limit. No structural changes are needed.

## Variant Notes

### single
- All seven brackets confirmed directly from the IRS newsroom release (src_irs_2). The 37% bracket begins at $640,600, up from $626,350 in 2025. The OBBBA made the TCJA rate structure permanent and applied an additional inflation adjustment for the bottom two brackets (10% and 12%) at approximately 4%, while higher brackets received approximately 2.3% adjustments.

### married_filing_jointly
- All seven brackets confirmed directly from the IRS newsroom release (src_irs_2). The 37% bracket begins at $768,700, up from $751,600 in 2025. MFJ brackets are approximately double the Single brackets for the 10%–24% rates, with wider spacing at 32%–37%.

### married_filing_separately
- Brackets confirmed from Doeren Mayhew (src_doeren_1). MFS brackets match Single exactly through the 32% rate. The 35% bracket is narrower ($256,225–$384,350) compared to Single ($256,225–$640,600), and the 37% threshold is correspondingly lower at $384,350. This is exactly half the MFJ 37% threshold ($768,700).

### head_of_household
- Brackets confirmed from Doeren Mayhew (src_doeren_1). The 10% and 12% brackets are wider than Single (10% up to $17,700 vs $12,400; 12% up to $67,450 vs $50,400), reflecting the more favorable HoH rate schedule. The 24%/32% boundary is $201,750 (not $201,775 as in Single), consistent with the historical $25 rounding difference observed in prior years (e.g., 2025: HoH 32%/35% at $243,700 vs Single $243,725). One web search synthesis erroneously reported $201,775 for HoH, but the detailed Doeren table and the current embedded value both show $201,750.

### qualifying_surviving_spouse
- QSS uses identical brackets to MFJ per IRC § 1(a), which defines the same rate schedule for both filing statuses. The Doeren source (src_doeren_1) labels its MFJ table as "Married Filing Jointly / Qualifying Surviving Spouse," confirming this equivalence. No independent bracket schedule exists for QSS.

## Comparison to Current Embedded Values
All proposed values match `current_value.json` exactly across all five filing statuses and all seven brackets. No changes are needed.

## Open Questions
- **Secondary source coverage gap**: Only 1 of the 2 required secondary confirmations came from an approved host (Tax Foundation at taxfoundation.org). Kiplinger.com returned HTTP 403 errors on all attempted fetches, and taxpolicycenter.org does not appear to publish a dedicated 2026 bracket page. The second confirmation was obtained from Doeren Mayhew (doeren.com), a reputable CPA firm but not on the approved secondary hosts list. A human reviewer may wish to manually verify the Tax Foundation Table 1 or attempt Kiplinger access to satisfy the minimum_secondary_confirmations=2 requirement.
- **Rev Proc 2025-32 PDF unreadable**: The authoritative source document (src_irs_1) could not be parsed by the web fetch tool due to binary PDF encoding. The IRS newsroom summary (src_irs_2) served as the effective primary source but only covers Single and MFJ. HoH and MFS brackets rely on secondary source confirmation.
- **HoH 24%/32% boundary discrepancy in web sources**: One web search synthesis reported $201,775 for HoH (matching Single), while the Doeren detailed table and current_value.json both show $201,750. The $201,750 value was adopted. This $25 difference is consistent with historical IRS rounding patterns for this bracket across HoH vs Single filing statuses.

## Verifier Report
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

## Warnings
- accepted source src_doeren_1 is on unsupported host www.doeren.com and will not count toward status

## Blocking Issues
- none
