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

2026 Medicare Part B Income-Related Monthly Adjustment Amount (IRMAA) bracket tables, keyed by tax filing status and MAGI. Contains the standard Part B premium ($202.90/month) and the tiered monthly surcharges that apply when MAGI exceeds specified thresholds. Published annually by CMS; thresholds are inflation-indexed under 42 U.S.C. §1395r.

## Lookup Parameters

- filing_status: one of single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse
- lived_with_spouse_during_year: boolean (only relevant when filing_status = married_filing_separately; true = lived with spouse, false = lived apart all year; null for other statuses)
- magi: the beneficiary's Modified Adjusted Gross Income from the applicable tax year

## Interpretation Notes

- Each variant's brackets form a partition of all non-negative MAGI values. The first bracket covers MAGI from $0 through magi_max (inclusive). Interior brackets cover MAGI strictly greater than magi_min and less than or equal to magi_max. The final bracket covers MAGI strictly greater than magi_min with magi_max = null (no upper bound).
- monthly_surcharge is the Income-Related Monthly Adjustment Amount added on top of base_part_b_premium. A surcharge of $0.00 means the beneficiary pays only the standard premium.
- The total monthly Part B premium = base_part_b_premium + monthly_surcharge.
- All dollar amounts are monthly. Multiply by 12 for annual cost.
- Filing statuses single, head_of_household, qualifying_surviving_spouse, and married_filing_separately (lived apart all year) all share the same MAGI thresholds and surcharges. Married_filing_jointly has doubled thresholds. Married_filing_separately (lived with spouse) has only three tiers with much higher surcharges in the middle tier.

## Does Not Include

- Part D (prescription drug) IRMAA surcharges
- Part A premiums or deductibles
- Part B immunosuppressive-drug-only coverage IRMAA amounts
- Medicare Advantage or Medigap premium data
- Part B annual deductible ($283 for 2026)

## Caveats

- IRMAA determination uses MAGI from two years prior (2024 tax return for 2026 premiums). Life-changing events may allow a more recent year via SSA-44.
- The CMS fact sheet publishes the 5th tier upper bound as strict less-than (e.g., <$500,000) and the top tier as greater-than-or-equal (≥$500,000). Under the contract convention (interior brackets inclusive of magi_max), $500,000 exactly maps to tier 5; under CMS notation it maps to tier 6. The practical difference is one dollar of MAGI at exactly $500,000/$750,000.
- Part B immunosuppressive-drug-only coverage has its own slightly different IRMAA surcharge amounts; this dataset covers full Part B coverage only.
- IRMAA applies per person. Both spouses on Medicare each pay IRMAA based on the joint MAGI.

## Typical Uses

- Estimating total Medicare Part B cost for retirement income projections
- Roth conversion planning to evaluate IRMAA cliff effects
- Tax-bracket-aware withdrawal sequencing that accounts for Medicare surcharges

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
      "organization": "Centers for Medicare & Medicaid Services",
      "source_class": "primary",
      "title": "2026 Medicare Parts A & B Premiums and Deductibles",
      "published_at": "2025-11-08",
      "locator": "Part B IRMAA tables for Individual, Joint, and Married Filing Separately filers",
      "counts_toward_status": true
    },
    {
      "source_id": "src_ssa_1",
      "url": "https://secure.ssa.gov/poms.nsf/lnx/0601101020",
      "host": "secure.ssa.gov",
      "organization": "Social Security Administration",
      "source_class": "supporting_official",
      "title": "SSA POMS HI 01101.020 – IRMAA Sliding Scale Tables",
      "published_at": "2025-12-02",
      "locator": "Table B.1 (Single/HoH/QSS), Table B.2 (MFJ), Table B.3 (MFS)",
      "counts_toward_status": false
    },
    {
      "source_id": "src_ssa_2",
      "url": "https://secure.ssa.gov/poms.nsf/lnx/0601120060",
      "host": "secure.ssa.gov",
      "organization": "Social Security Administration",
      "source_class": "supporting_official",
      "title": "SSA POMS HI 01120.060 – Married, Filing Separately – Lived Apart All Year",
      "published_at": "2011-10-04",
      "locator": "Policy section: 'we will make any necessary adjustments to IRMAA by using the table in HI 01101.020B.1'",
      "counts_toward_status": false
    },
    {
      "source_id": "src_kff_1",
      "url": "https://www.kff.org/quick-take/medicare-beneficiaries-are-not-insulated-from-affordability-challenges-as-part-b-premiums-rise-in-2026/",
      "host": "www.kff.org",
      "organization": "KFF (Kaiser Family Foundation)",
      "source_class": "secondary",
      "title": "Medicare Beneficiaries Are Not Insulated from Affordability Challenges As Part B Premiums Rise in 2026",
      "published_at": "2025-11-08",
      "locator": "Paragraph: 'standard monthly premium…$202.90' and 'income above $109,000…phases up to $689.90 per month'",
      "counts_toward_status": false
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

- Centers for Medicare & Medicaid Services — 2026 Medicare Parts A & B Premiums and Deductibles — https://www.cms.gov/newsroom/fact-sheets/2026-medicare-parts-b-premiums-deductibles
- Social Security Administration — SSA POMS HI 01101.020 – IRMAA Sliding Scale Tables — https://secure.ssa.gov/poms.nsf/lnx/0601101020
- Social Security Administration — SSA POMS HI 01120.060 – Married, Filing Separately – Lived Apart All Year — https://secure.ssa.gov/poms.nsf/lnx/0601120060
- KFF (Kaiser Family Foundation) — Medicare Beneficiaries Are Not Insulated from Affordability Challenges As Part B Premiums Rise in 2026 — https://www.kff.org/quick-take/medicare-beneficiaries-are-not-insulated-from-affordability-challenges-as-part-b-premiums-rise-in-2026/
