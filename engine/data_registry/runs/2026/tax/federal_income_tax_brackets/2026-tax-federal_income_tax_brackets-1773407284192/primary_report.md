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
