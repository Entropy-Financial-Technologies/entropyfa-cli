---
category: social_security
year: 2026
key: retirement_earnings_test_thresholds
title: Retirement Earnings Test Thresholds
reviewed_artifact: social_security/2026/retirement_earnings_test_thresholds
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Retirement Earnings Test Thresholds

## What This Is

Annual and monthly exempt-earnings amounts and benefit-reduction rates for the Social Security retirement earnings test, which governs how much a beneficiary's retirement benefits are withheld when they have earned income before reaching full retirement age.

## Lookup Parameters

- No variant-selection parameters are needed; the earnings test thresholds are uniform across all filing statuses. The single 'default' variant applies to all beneficiaries subject to the test.

## Interpretation Notes

- Two age brackets exist: (1) 'under FRA' applies when the beneficiary is under full retirement age for the entire calendar year; (2) 'year of FRA' applies in the calendar year the beneficiary reaches FRA, counting only pre-FRA-month earnings.
- The monthly exempt amount equals the annual exempt amount divided by 12 and is used for the 'special first-year rule,' which allows a monthly earnings test in the first partial year of retirement.
- Reduction rates are expressed as a fraction of excess earnings withheld: 0.5 means $1 withheld per $2 over the limit; 0.333... means $1 withheld per $3 over the limit.
- All amounts are in nominal U.S. dollars for the stated calendar year.

## Does Not Include

- Post-FRA benefit recalculation (Adjustment of Reduction Factor / ARF) that restores previously withheld benefits.
- Substantial Gainful Activity (SGA) thresholds for disability benefits, which are separate from the retirement earnings test.
- The Social Security taxable maximum (wage base), which is a separate annual parameter.

## Caveats

- Reduction rates (0.5 for under-FRA; 1/3 for year-of-FRA) are statutory under Social Security Act §203 and do not change annually; only the exempt amounts are adjusted each year via the national average wage index.
- The year-of-FRA bracket counts only earnings in months before the month FRA is reached; starting with the FRA month, the earnings test no longer applies.
- Benefits withheld due to the earnings test are not permanently lost — SSA recalculates benefits at FRA via the Adjustment of Reduction Factor (ARF), but ARF is not included in this dataset.
- Self-employment income and certain deferred compensation may be counted differently; special rules apply to the first year of retirement (monthly test).

## Typical Uses

- Determining whether and by how much Social Security retirement benefits will be reduced for a beneficiary who has earned income before reaching full retirement age.
- Applying the monthly earnings test under the special first-year rule for newly retired beneficiaries mid-year.

## Machine Block

```json
{
  "schema_version": 1,
  "category": "social_security",
  "key": "retirement_earnings_test_thresholds",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_ssa_1",
      "url": "https://www.ssa.gov/oact/cola/rtea.html",
      "host": "www.ssa.gov",
      "organization": "Social Security Administration, Office of the Chief Actuary",
      "source_class": "primary",
      "title": "Exempt Amounts Under the Earnings Test",
      "published_at": "2025-10-24",
      "locator": "Table of annual and monthly exempt amounts, row for year 2026",
      "counts_toward_status": true
    },
    {
      "source_id": "src_ssa_2",
      "url": "https://www.ssa.gov/benefits/retirement/planner/whileworking.html",
      "host": "www.ssa.gov",
      "organization": "Social Security Administration",
      "source_class": "primary",
      "title": "Benefits Planner: Retirement | Receiving Benefits While Working",
      "published_at": "2025-10-24",
      "locator": "Earnings limits and reduction rates for 2026 described in main content sections",
      "counts_toward_status": true
    },
    {
      "source_id": "src_ssa_3",
      "url": "https://www.ssa.gov/news/en/cola/factsheets/2026.html",
      "host": "www.ssa.gov",
      "organization": "Social Security Administration",
      "source_class": "primary",
      "title": "2026 Cost-of-Living Adjustment (COLA) Fact Sheet",
      "published_at": "2025-10-24",
      "locator": "Earnings test section of the 2026 COLA fact sheet",
      "counts_toward_status": true
    },
    {
      "source_id": "src_aarp_1",
      "url": "https://www.aarp.org/social-security/retirement/working-in-retirement/",
      "host": "www.aarp.org",
      "organization": "AARP",
      "source_class": "secondary",
      "title": "You Can Work and Get Social Security. But It Might Cost You",
      "published_at": "2025-11-01",
      "locator": "Article body — 2026 earnings limits section",
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
          "under_fra_annual_exempt_amount": 24480.0,
          "under_fra_monthly_exempt_amount": 2040.0,
          "under_fra_reduction_rate": 0.5,
          "year_of_fra_annual_exempt_amount": 65160.0,
          "year_of_fra_monthly_exempt_amount": 5430.0,
          "year_of_fra_reduction_rate": 0.3333333333333333
        }
      }
    ]
  }
}
```

## Sources

- Social Security Administration, Office of the Chief Actuary — Exempt Amounts Under the Earnings Test — https://www.ssa.gov/oact/cola/rtea.html
- Social Security Administration — Benefits Planner: Retirement | Receiving Benefits While Working — https://www.ssa.gov/benefits/retirement/planner/whileworking.html
- Social Security Administration — 2026 Cost-of-Living Adjustment (COLA) Fact Sheet — https://www.ssa.gov/news/en/cola/factsheets/2026.html
- AARP — You Can Work and Get Social Security. But It Might Cost You — https://www.aarp.org/social-security/retirement/working-in-retirement/
