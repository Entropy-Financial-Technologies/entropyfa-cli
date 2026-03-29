---
category: tax
year: 2026
key: hsa_contribution_limits
title: Hsa Contribution Limits
reviewed_artifact: tax/2026/hsa_contribution_limits
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Hsa Contribution Limits

## What This Is

Calendar year 2026 HSA contribution limits (self-only, family, and age-55+ catch-up) and the HDHP qualification thresholds (minimum annual deductible and maximum annual out-of-pocket) as published by the IRS under IRC §223.

## Lookup Parameters

- coverage_tier: 'self_only' or 'family' — determines which contribution limit and HDHP threshold set applies

## Interpretation Notes

- All values are in nominal USD for calendar year 2026.
- hsa_contribution_self_only and hsa_contribution_family are the maximum annual HSA contribution (employee + employer combined) for each coverage tier.
- hsa_catch_up_55_plus is an additional amount on top of the base limit, available to account holders who are age 55 or older by end of the tax year and not enrolled in Medicare.
- hdhp_min_deductible values are the minimum annual deductible an HDHP must carry for the plan to qualify as HSA-eligible.
- hdhp_max_out_of_pocket values are the maximum annual out-of-pocket expenses (deductibles, copayments, coinsurance; excluding premiums) the plan may impose.
- The 'family' tier applies when the HDHP covers at least one other individual besides the account holder.

## Does Not Include

- Employer contribution amounts or employer-specific plan design details
- FSA (Flexible Spending Account) contribution limits, which are governed by a separate IRC section
- Archer MSA deductible ranges and out-of-pocket limits, which have different thresholds
- State-level HSA tax treatment variations (e.g., California and New Jersey do not conform to federal HSA tax treatment)

## Caveats

- The catch-up contribution ($1,000) is statutory under IRC §223(b)(3)(B) and is not indexed for inflation; it does not change year-to-year.
- These limits apply to calendar year 2026 contributions. Pro-rata rules apply for individuals who are HSA-eligible for only part of the year (last-month rule may override).
- The OBBBA (One Big Beautiful Bill Act) expanded HSA-eligible coverage types but did not change these dollar amounts for 2026.
- HDHP out-of-pocket maximums include deductibles, copayments, and coinsurance but exclude premiums.
- An individual enrolled in Medicare Part A or Part B is no longer HSA-eligible, even if still covered by an HDHP.

## Typical Uses

- Determining maximum allowable HSA contributions for a given coverage tier and age bracket
- Validating whether a health plan's deductible and out-of-pocket maximum qualify it as an HDHP for HSA eligibility

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "hsa_contribution_limits",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_1",
      "url": "https://www.irs.gov/publications/p969",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Publication 969 (2025), Health Savings Accounts and Other Tax-Favored Health Plans",
      "published_at": "2025",
      "locator": "2026 contribution limits table and HDHP definition table",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/pub/irs-drop/rp-25-19.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Revenue Procedure 2025-19",
      "published_at": "2025",
      "locator": "Section 3, inflation-adjusted amounts for calendar year 2026",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_1",
      "url": "https://www.kiplinger.com/taxes/irs-unveils-new-hsa-limits",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "2026 HSA Contribution Limits Are Set: What to Know Now",
      "published_at": "2025",
      "locator": "Article body, 2026 HSA limits summary table",
      "counts_toward_status": true
    }
  ],
  "value": {
    "variants": [
      {
        "label": "default",
        "params": {
          "filing_status": null,
          "lived_with_spouse_during_year": null
        },
        "value": {
          "hdhp_max_out_of_pocket_family": 17000,
          "hdhp_max_out_of_pocket_self_only": 8500,
          "hdhp_min_deductible_family": 3400,
          "hdhp_min_deductible_self_only": 1700,
          "hsa_catch_up_55_plus": 1000,
          "hsa_contribution_family": 8750,
          "hsa_contribution_self_only": 4400
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Publication 969 (2025), Health Savings Accounts and Other Tax-Favored Health Plans — https://www.irs.gov/publications/p969
- Internal Revenue Service — Revenue Procedure 2025-19 — https://www.irs.gov/pub/irs-drop/rp-25-19.pdf
- Kiplinger — 2026 HSA Contribution Limits Are Set: What to Know Now — https://www.kiplinger.com/taxes/irs-unveils-new-hsa-limits
