---
category: tax
year: 2026
key: federal_salt_deduction_parameters
title: Federal Salt Deduction Parameters
reviewed_artifact: tax/2026/federal_salt_deduction_parameters
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Salt Deduction Parameters

## What This Is

Federal state and local tax (SALT) deduction parameters for tax year 2026 under IRC §164(b)(6) as amended by the One, Big, Beautiful Bill Act. Contains per-filing-status cap amount, MAGI-based phaseout threshold, phaseout rate, and floor amount.

## Lookup Parameters

- filing_status — determines whether the taxpayer uses full or half (MFS) parameter values

## Interpretation Notes

- cap_amount is the maximum combined deduction for state/local income (or sales) taxes plus real estate and personal property taxes, before any MAGI-based reduction.
- phaseout_threshold is the MAGI level above which the cap begins to decrease. The reduction equals 30% of MAGI in excess of the threshold.
- phaseout_rate (0.3) is the statutory reduction rate applied to the excess MAGI above the threshold.
- floor_amount is the minimum deduction cap after phaseout; the cap cannot be reduced below this amount regardless of income.
- The statute defines only two tiers: married-filing-separately (half amounts) and all other statuses (full amounts). Single, MFJ, HoH, and QSS share identical parameters.
- All dollar amounts are per-return, not per-person (relevant for MFJ filers).

## Does Not Include

- The choice between deducting state/local income tax versus state/local sales tax (that election is caller input).
- State-level pass-through entity tax (PTET) workaround amounts, which bypass the SALT cap entirely.
- Foreign tax amounts, which are claimed separately under IRC §901.
- Any state-specific SALT cap conformity rules or additional state-level limitations.

## Caveats

- The increased SALT cap ($40,000 base, indexed 1% annually) applies only to tax years 2025–2029; it reverts to $10,000/$5,000 MFS in 2030 unless Congress acts.
- The phaseout uses modified AGI (MAGI), not AGI. MAGI adds back student loan interest, deductible IRA contributions, foreign earned income exclusion, and tax-exempt interest.
- The 2026 figures ($40,400 cap, $505,000 threshold) reflect the statutory 1% annual increase from the 2025 base amounts ($40,000 cap, $500,000 threshold).
- The IRS posted a correction to the 2026 Form 1040-ES on 2026-02-27, updating the SALT reminder to reflect the correct 2026 inflation-adjusted amounts.

## Typical Uses

- Computing the allowable SALT deduction on Schedule A, Line 5e, given filing status and MAGI.
- Estimating itemized deduction totals for tax projection and estimated payment calculations.

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_salt_deduction_parameters",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_1040es_correction",
      "url": "https://www.irs.gov/forms-pubs/correction-to-state-and-local-income-tax-deduction-amount-in-the-2026-form-1040-es",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Correction to state and local income tax deduction amount in the 2026 Form 1040-ES",
      "published_at": "2026-02-27",
      "locator": "Full page — correction notice with 2026 inflation-adjusted SALT cap, phaseout threshold, and floor amounts",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_scha_2025",
      "url": "https://www.irs.gov/instructions/i1040sca",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Instructions for Schedule A (Form 1040) (2025) — State and Local Tax Deduction Worksheet",
      "published_at": "2025-12-01",
      "locator": "State and Local Tax Deduction Worksheet, Line 6: multiply excess MAGI by 0.30",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_topic503",
      "url": "https://www.irs.gov/taxtopics/tc503",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Topic No. 503, Deductible Taxes",
      "published_at": "2025-08-01",
      "locator": "Section on overall SALT deduction limit: '$40,000 ($20,000 if married filing separately)' with MAGI limitation and $10,000 floor",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_revproc_2025_32",
      "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill",
      "published_at": "2025-10-01",
      "locator": "Newsroom announcement referencing Revenue Procedure 2025-32 for 2026 inflation adjustments",
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
          "cap_amount": 40400.0,
          "filing_status": "single",
          "floor_amount": 10000.0,
          "phaseout_rate": 0.3,
          "phaseout_threshold": 505000.0
        }
      },
      {
        "label": "married_filing_jointly",
        "params": {
          "filing_status": "married_filing_jointly",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "cap_amount": 40400.0,
          "filing_status": "married_filing_jointly",
          "floor_amount": 10000.0,
          "phaseout_rate": 0.3,
          "phaseout_threshold": 505000.0
        }
      },
      {
        "label": "married_filing_separately",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "cap_amount": 20200.0,
          "filing_status": "married_filing_separately",
          "floor_amount": 5000.0,
          "phaseout_rate": 0.3,
          "phaseout_threshold": 252500.0
        }
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "cap_amount": 40400.0,
          "filing_status": "head_of_household",
          "floor_amount": 10000.0,
          "phaseout_rate": 0.3,
          "phaseout_threshold": 505000.0
        }
      },
      {
        "label": "qualifying_surviving_spouse",
        "params": {
          "filing_status": "qualifying_surviving_spouse",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "cap_amount": 40400.0,
          "filing_status": "qualifying_surviving_spouse",
          "floor_amount": 10000.0,
          "phaseout_rate": 0.3,
          "phaseout_threshold": 505000.0
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Correction to state and local income tax deduction amount in the 2026 Form 1040-ES — https://www.irs.gov/forms-pubs/correction-to-state-and-local-income-tax-deduction-amount-in-the-2026-form-1040-es
- Internal Revenue Service — Instructions for Schedule A (Form 1040) (2025) — State and Local Tax Deduction Worksheet — https://www.irs.gov/instructions/i1040sca
- Internal Revenue Service — Topic No. 503, Deductible Taxes — https://www.irs.gov/taxtopics/tc503
- Internal Revenue Service — IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill — https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
