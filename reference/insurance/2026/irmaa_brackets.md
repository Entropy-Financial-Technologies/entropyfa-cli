---
category: insurance
year: 2026
key: irmaa_brackets
title: Irmaa Brackets
reviewed_artifact: insurance/2026/irmaa_brackets
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Irmaa Brackets

## What This Is

2026 Medicare Part B Income-Related Monthly Adjustment Amount (IRMAA) bracket tables published by CMS and administered by SSA. Contains the standard Part B base premium and tiered monthly surcharges indexed by modified adjusted gross income (MAGI) and tax filing status. Six variant tables cover all IRS filing statuses, with special handling for married-filing-separately filers based on whether they lived with their spouse during the tax year.

## Lookup Parameters

- filing_status — one of: single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse
- lived_with_spouse_during_year — boolean, required only when filing_status is married_filing_separately; determines whether the compressed MFS table or the single-filer table applies
- magi — modified adjusted gross income from the relevant prior tax year, used to select the applicable bracket

## Interpretation Notes

- Each variant's brackets array is ordered by ascending MAGI. The first bracket (magi_min=0, surcharge=0) covers income at or below magi_max. Interior brackets cover income strictly greater than magi_min and at or equal to magi_max. The final bracket (magi_max=null) covers income strictly greater than magi_min with no upper bound.
- monthly_surcharge is the IRMAA add-on above the base Part B premium; total monthly premium = base_part_b_premium + monthly_surcharge.
- base_part_b_premium is the standard Part B premium set by CMS for the calendar year, identical across all variants.
- All dollar values are monthly amounts in nominal USD for the stated calendar year.
- Filing status categories map to IRS filing statuses. 'Qualifying surviving spouse' corresponds to the IRS filing status 'Qualifying Surviving Spouse (with Dependent Child).'
- Single, head_of_household, qualifying_surviving_spouse, and married_filing_separately_lived_apart all share identical bracket thresholds and surcharges (derived from SSA POMS Table B.1).
- married_filing_separately_lived_with_spouse uses a compressed two-surcharge-tier table that jumps directly from $0 surcharge to the second-highest surcharge tier.

## Does Not Include

- Part D IRMAA prescription drug premium surcharges (separate dataset)
- Medicare Part A premiums or IRMAA adjustments
- Medicare Advantage plan-specific premium reductions
- Late enrollment penalty surcharges
- State-level Medicare Savings Program (MSP) eligibility thresholds

## Caveats

- IRMAA uses MAGI from two years prior (e.g., 2024 MAGI for 2026 premiums); if a more recent return is unavailable, SSA may use three-year-old data.
- IRMAA thresholds are cliff-based, not marginal: exceeding a threshold by $1 triggers the full surcharge for that tier.
- Beneficiaries may request a new initial determination (SSA-44) if a life-changing event reduced their income below the threshold used.
- The boundary between the second-highest and highest tiers uses strict less-than / greater-than-or-equal in the SSA published tables (e.g., <$500,000 / ≥$500,000), which differs from the ≤ / > convention used in interior brackets. The schema maps these to magi_max=500000 / magi_min=500000 per the documented contract convention.
- Married Filing Separately filers who lived apart all year must affirmatively attest (under penalty of perjury) to receive single-filer table treatment; absent attestation, the compressed MFS table applies.

## Typical Uses

- Projecting total Medicare Part B cost for retirement income planning by combining base_part_b_premium with the applicable monthly_surcharge for a given MAGI level
- Identifying IRMAA cliff thresholds to evaluate Roth conversion or capital gains realization strategies
- Modeling the marginal cost impact of income changes near bracket boundaries

## Machine Block

```json
{
  "schema_version": 1,
  "category": "insurance",
  "key": "irmaa_brackets",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_cms_1",
      "url": "https://www.cms.gov/newsroom/fact-sheets/2026-medicare-parts-b-premiums-deductibles",
      "host": "www.cms.gov",
      "organization": "Centers for Medicare & Medicaid Services (CMS)",
      "source_class": "primary",
      "title": "2026 Medicare Parts A & B Premiums and Deductibles",
      "published_at": "2025-11",
      "locator": "Fact sheet — 2026 standard monthly Part B premium ($202.90) and IRMAA income-related premium tables by filing status",
      "counts_toward_status": true
    },
    {
      "source_id": "src_ssa_1",
      "url": "https://secure.ssa.gov/poms.nsf/lnx/0601101020",
      "host": "secure.ssa.gov",
      "organization": "Social Security Administration (SSA)",
      "source_class": "supporting_official",
      "title": "SSA POMS HI 01101.020 — IRMAA Sliding Scale Tables",
      "published_at": "2025-12-02",
      "locator": "Section B — Tables B.1 (Single/HoH/QSS), B.2 (MFJ), B.3 (MFS) with all MAGI thresholds and corresponding total Part B premiums for 2026",
      "counts_toward_status": true
    },
    {
      "source_id": "src_ssa_2",
      "url": "https://secure.ssa.gov/poms.nsf/lnx/0601120060",
      "host": "secure.ssa.gov",
      "organization": "Social Security Administration (SSA)",
      "source_class": "supporting_official",
      "title": "SSA POMS HI 01120.060 — Married, Filing Separately – Lived Apart All Year",
      "published_at": "2011-10-04",
      "locator": "Policy section — directs SSA to use Table HI 01101.020B.1 (single filer table) when MFS filer attests to living apart all year",
      "counts_toward_status": true
    },
    {
      "source_id": "src_ssa_3",
      "url": "https://secure.ssa.gov/poms.nsf/lnx/0601101031",
      "host": "secure.ssa.gov",
      "organization": "Social Security Administration (SSA)",
      "source_class": "supporting_official",
      "title": "SSA POMS HI 01101.031 — How IRMAA is Calculated and How IRMAA Affects the Total Medicare Premium",
      "published_at": "2025-12-02",
      "locator": "Full section — IRMAA percentage tiers (35%, 50%, 65%, 80%, 85%) and calculation formula for 2026",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kff_1",
      "url": "https://www.kff.org/quick-take/medicare-beneficiaries-are-not-insulated-from-affordability-challenges-as-part-b-premiums-rise-in-2026/",
      "host": "www.kff.org",
      "organization": "KFF (Kaiser Family Foundation)",
      "source_class": "secondary",
      "title": "Medicare Beneficiaries Are Not Insulated from Affordability Challenges As Part B Premiums Rise in 2026",
      "published_at": "2025-11",
      "locator": "Article body — confirms $202.90 standard premium and $689.90 maximum premium for incomes ≥ $500,000",
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
          "base_part_b_premium": 202.9,
          "brackets": [
            {
              "magi_max": 109000.0,
              "magi_min": 0.0,
              "monthly_surcharge": 0.0
            },
            {
              "magi_max": 137000.0,
              "magi_min": 109000.0,
              "monthly_surcharge": 81.2
            },
            {
              "magi_max": 171000.0,
              "magi_min": 137000.0,
              "monthly_surcharge": 202.9
            },
            {
              "magi_max": 205000.0,
              "magi_min": 171000.0,
              "monthly_surcharge": 324.6
            },
            {
              "magi_max": 500000.0,
              "magi_min": 205000.0,
              "monthly_surcharge": 446.3
            },
            {
              "magi_max": null,
              "magi_min": 500000.0,
              "monthly_surcharge": 487.0
            }
          ],
          "filing_status": "single"
        }
      },
      {
        "label": "married_filing_jointly",
        "params": {
          "filing_status": "married_filing_jointly",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "base_part_b_premium": 202.9,
          "brackets": [
            {
              "magi_max": 218000.0,
              "magi_min": 0.0,
              "monthly_surcharge": 0.0
            },
            {
              "magi_max": 274000.0,
              "magi_min": 218000.0,
              "monthly_surcharge": 81.2
            },
            {
              "magi_max": 342000.0,
              "magi_min": 274000.0,
              "monthly_surcharge": 202.9
            },
            {
              "magi_max": 410000.0,
              "magi_min": 342000.0,
              "monthly_surcharge": 324.6
            },
            {
              "magi_max": 750000.0,
              "magi_min": 410000.0,
              "monthly_surcharge": 446.3
            },
            {
              "magi_max": null,
              "magi_min": 750000.0,
              "monthly_surcharge": 487.0
            }
          ],
          "filing_status": "married_filing_jointly"
        }
      },
      {
        "label": "married_filing_separately_lived_with_spouse",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": true
        },
        "value": {
          "base_part_b_premium": 202.9,
          "brackets": [
            {
              "magi_max": 109000.0,
              "magi_min": 0.0,
              "monthly_surcharge": 0.0
            },
            {
              "magi_max": 391000.0,
              "magi_min": 109000.0,
              "monthly_surcharge": 446.3
            },
            {
              "magi_max": null,
              "magi_min": 391000.0,
              "monthly_surcharge": 487.0
            }
          ],
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": true
        }
      },
      {
        "label": "married_filing_separately_lived_apart",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": false
        },
        "value": {
          "base_part_b_premium": 202.9,
          "brackets": [
            {
              "magi_max": 109000.0,
              "magi_min": 0.0,
              "monthly_surcharge": 0.0
            },
            {
              "magi_max": 137000.0,
              "magi_min": 109000.0,
              "monthly_surcharge": 81.2
            },
            {
              "magi_max": 171000.0,
              "magi_min": 137000.0,
              "monthly_surcharge": 202.9
            },
            {
              "magi_max": 205000.0,
              "magi_min": 171000.0,
              "monthly_surcharge": 324.6
            },
            {
              "magi_max": 500000.0,
              "magi_min": 205000.0,
              "monthly_surcharge": 446.3
            },
            {
              "magi_max": null,
              "magi_min": 500000.0,
              "monthly_surcharge": 487.0
            }
          ],
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": false
        }
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "base_part_b_premium": 202.9,
          "brackets": [
            {
              "magi_max": 109000.0,
              "magi_min": 0.0,
              "monthly_surcharge": 0.0
            },
            {
              "magi_max": 137000.0,
              "magi_min": 109000.0,
              "monthly_surcharge": 81.2
            },
            {
              "magi_max": 171000.0,
              "magi_min": 137000.0,
              "monthly_surcharge": 202.9
            },
            {
              "magi_max": 205000.0,
              "magi_min": 171000.0,
              "monthly_surcharge": 324.6
            },
            {
              "magi_max": 500000.0,
              "magi_min": 205000.0,
              "monthly_surcharge": 446.3
            },
            {
              "magi_max": null,
              "magi_min": 500000.0,
              "monthly_surcharge": 487.0
            }
          ],
          "filing_status": "head_of_household"
        }
      },
      {
        "label": "qualifying_surviving_spouse",
        "params": {
          "filing_status": "qualifying_surviving_spouse",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "base_part_b_premium": 202.9,
          "brackets": [
            {
              "magi_max": 109000.0,
              "magi_min": 0.0,
              "monthly_surcharge": 0.0
            },
            {
              "magi_max": 137000.0,
              "magi_min": 109000.0,
              "monthly_surcharge": 81.2
            },
            {
              "magi_max": 171000.0,
              "magi_min": 137000.0,
              "monthly_surcharge": 202.9
            },
            {
              "magi_max": 205000.0,
              "magi_min": 171000.0,
              "monthly_surcharge": 324.6
            },
            {
              "magi_max": 500000.0,
              "magi_min": 205000.0,
              "monthly_surcharge": 446.3
            },
            {
              "magi_max": null,
              "magi_min": 500000.0,
              "monthly_surcharge": 487.0
            }
          ],
          "filing_status": "qualifying_surviving_spouse"
        }
      }
    ]
  }
}
```

## Sources

- Centers for Medicare & Medicaid Services (CMS) — 2026 Medicare Parts A & B Premiums and Deductibles — https://www.cms.gov/newsroom/fact-sheets/2026-medicare-parts-b-premiums-deductibles
- Social Security Administration (SSA) — SSA POMS HI 01101.020 — IRMAA Sliding Scale Tables — https://secure.ssa.gov/poms.nsf/lnx/0601101020
- Social Security Administration (SSA) — SSA POMS HI 01120.060 — Married, Filing Separately – Lived Apart All Year — https://secure.ssa.gov/poms.nsf/lnx/0601120060
- Social Security Administration (SSA) — SSA POMS HI 01101.031 — How IRMAA is Calculated and How IRMAA Affects the Total Medicare Premium — https://secure.ssa.gov/poms.nsf/lnx/0601101031
- KFF (Kaiser Family Foundation) — Medicare Beneficiaries Are Not Insulated from Affordability Challenges As Part B Premiums Rise in 2026 — https://www.kff.org/quick-take/medicare-beneficiaries-are-not-insulated-from-affordability-challenges-as-part-b-premiums-rise-in-2026/
