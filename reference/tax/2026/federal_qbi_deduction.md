---
category: tax
year: 2026
key: federal_qbi_deduction
title: Federal Qbi Deduction
reviewed_artifact: tax/2026/federal_qbi_deduction
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Qbi Deduction

## What This Is

Section 199A qualified business income (QBI) deduction parameters for tax year 2026, keyed by filing status. Contains the income threshold where W-2/UBIA limitations begin, the phase-in range endpoint where limitations fully apply, the deduction rate (20%), and the new OBBBA-enacted minimum deduction floor ($400 if QBI >= $1,000). Published in IRS Revenue Procedure 2025-32, Section 4.26.

## Lookup Parameters

- filing_status — one of: single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse

## Interpretation Notes

- Each variant stores the full parameter set: deduction_rate, threshold, phase_in_range_end, minimum_qbi_deduction, and minimum_qbi_amount.
- Below the threshold, the full 20% deduction applies without W-2/UBIA limitations.
- Between threshold and phase_in_range_end, W-2/UBIA limitations phase in linearly.
- Above phase_in_range_end, W-2/UBIA limitations apply fully and SSTB income is excluded entirely.
- The minimum_qbi_deduction ($400) applies only when aggregate QBI from all active qualified trades or businesses (material participation required) is at least minimum_qbi_amount ($1,000).
- All dollar amounts are for tax year 2026 as published in Rev. Proc. 2025-32, Section 4.26.

## Does Not Include

- W-2 wage and UBIA limitations that apply above the threshold (IRC §199A(b)(2))
- SSTB (specified service trade or business) classification rules (IRC §199A(d))
- Cooperative-related QBI adjustments (IRC §199A(b)(7))
- REIT dividend and PTP income deduction rules (IRC §199A(b)(1)(B))
- State-level QBI conformity or add-back rules

## Caveats

- The minimum deduction ($400) and minimum QBI eligibility ($1,000) were added by the OBBBA (2025) and are first effective for tax year 2026; they will be inflation-adjusted in $5 increments after 2026.
- The phase-in range widths ($75,000 single / $150,000 MFJ) are expanded from pre-OBBBA amounts ($50,000 / $100,000); these are also indexed for inflation after 2026.
- The deduction rate is 20%, not 23%; the House-proposed increase to 23% was not enacted in the final OBBBA.
- MFS threshold ($201,775) differs from Single/HOH/QSS ($201,750) due to independent rounding; MFS is not simply half of MFJ ($403,500 / 2 = $201,750).

## Typical Uses

- Determine whether a taxpayer's QBI deduction is subject to W-2 wage and UBIA limitations based on filing status and taxable income.
- Calculate the phase-in fraction for partially limited QBI deductions.
- Identify whether the minimum deduction floor applies.

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_qbi_deduction",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_revproc_1",
      "url": "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Revenue Procedure 2025-32 — Inflation Adjusted Items for Tax Year 2026",
      "published_at": "2025-10-09",
      "locator": "Section 4.26 — Qualified Business Income (§199A(e)(2), §199A(b)(3)(B), §199A(i))",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_newsroom_1",
      "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill",
      "published_at": "2025-10-09",
      "locator": "Newsroom announcement summarizing Rev. Proc. 2025-32 provisions",
      "counts_toward_status": false
    },
    {
      "source_id": "src_taxfoundation_1",
      "url": "https://taxfoundation.org/data/all/federal/2026-tax-brackets/",
      "host": "taxfoundation.org",
      "organization": "Tax Foundation",
      "source_class": "secondary",
      "title": "2026 Tax Brackets and Federal Income Tax Rates",
      "published_at": "2025-10-09",
      "locator": "Section 199A QBI deduction subsection",
      "counts_toward_status": true
    }
  ],
  "value": {
    "variants": [
      {
        "label": "single",
        "params": {
          "filing_status": "single",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "deduction_rate": 0.2,
          "minimum_qbi_amount": 1000.0,
          "minimum_qbi_deduction": 400.0,
          "phase_in_range_end": 276750.0,
          "threshold": 201750.0
        }
      },
      {
        "label": "married_filing_jointly",
        "params": {
          "filing_status": "married_filing_jointly",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "deduction_rate": 0.2,
          "minimum_qbi_amount": 1000.0,
          "minimum_qbi_deduction": 400.0,
          "phase_in_range_end": 553500.0,
          "threshold": 403500.0
        }
      },
      {
        "label": "married_filing_separately",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "deduction_rate": 0.2,
          "minimum_qbi_amount": 1000.0,
          "minimum_qbi_deduction": 400.0,
          "phase_in_range_end": 276775.0,
          "threshold": 201775.0
        }
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "deduction_rate": 0.2,
          "minimum_qbi_amount": 1000.0,
          "minimum_qbi_deduction": 400.0,
          "phase_in_range_end": 276750.0,
          "threshold": 201750.0
        }
      },
      {
        "label": "qualifying_surviving_spouse",
        "params": {
          "filing_status": "qualifying_surviving_spouse",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "deduction_rate": 0.2,
          "minimum_qbi_amount": 1000.0,
          "minimum_qbi_deduction": 400.0,
          "phase_in_range_end": 276750.0,
          "threshold": 201750.0
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Revenue Procedure 2025-32 — Inflation Adjusted Items for Tax Year 2026 — https://www.irs.gov/pub/irs-drop/rp-25-32.pdf
- Internal Revenue Service — IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill — https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
- Tax Foundation — 2026 Tax Brackets and Federal Income Tax Rates — https://taxfoundation.org/data/all/federal/2026-tax-brackets/
