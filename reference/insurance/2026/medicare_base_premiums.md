---
category: insurance
year: 2026
key: medicare_base_premiums
title: Medicare Base Premiums
reviewed_artifact: insurance/2026/medicare_base_premiums
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Medicare Base Premiums

## What This Is

Annual standard Medicare Part B monthly premium, Part B annual deductible, and Part D national base beneficiary premium for calendar year 2026, as published by the Centers for Medicare & Medicaid Services (CMS).

## Lookup Parameters

- No lookup parameters are needed. This dataset contains a single 'default' variant applicable to all standard Medicare beneficiaries regardless of filing status.

## Interpretation Notes

- All values are for calendar year 2026 (January 1 – December 31, 2026).
- part_b_standard_monthly_premium is the per-month amount; multiply by 12 for annual cost.
- part_b_annual_deductible is the full-year amount that must be met before Part B coinsurance applies.
- part_d_base_beneficiary_premium is the national base beneficiary premium published annually by CMS as a statutory benchmark. It is not a plan premium and is not the Part D IRMAA surcharge table; CMS publishes Part D IRMAA surcharge amounts separately.
- This dataset has a single 'default' variant with no filing-status or household parameters because Medicare base premiums do not vary by tax filing status.

## Does Not Include

- IRMAA surcharge brackets or income-adjusted premium amounts for Part B or Part D
- Medicare Part A premiums or deductibles
- Medicare Part C (Medicare Advantage) plan premiums
- Actual Part D plan premiums charged by individual prescription drug plans or MA-PD plans
- Part B coinsurance rates (20% after deductible)
- Medicare Supplement (Medigap) premiums

## Caveats

- Part B premium shown is the standard premium only; beneficiaries with MAGI above specified thresholds pay additional IRMAA surcharges, which are tracked separately.
- Part D base beneficiary premium is the national base amount used for IRMAA calculations and does not represent what any individual plan charges; actual Part D plan premiums vary by plan and region.
- The Part D base beneficiary premium for 2026 is subject to the IRA premium stabilization provision capping annual increases at 6% through 2029.
- CMS publishes the national base beneficiary premium ($38.99) as a statutory benchmark. Separately, CMS may operate a voluntary Part D Premium Stabilization Demonstration that applies a uniform reduction in participating plan premium calculations; any such demonstration adjustment does not change the published national base beneficiary premium in this dataset.

## Typical Uses

- Estimating baseline Medicare costs in retirement cash-flow projections
- Computing Part B IRMAA-adjusted total premiums when combined with the separate IRMAA bracket dataset
- Using the Part D national base beneficiary premium as a reference input for late-enrollment-penalty calculations or as a benchmark in Medicare budgeting, without implying that plan-specific Part D totals can be derived from this value alone
- Comparing year-over-year Medicare cost changes for beneficiary budgeting

## Machine Block

```json
{
  "schema_version": 1,
  "category": "insurance",
  "key": "medicare_base_premiums",
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
      "locator": "Fact sheet — Part B premium and deductible sections",
      "counts_toward_status": true
    },
    {
      "source_id": "src_cms_2",
      "url": "https://www.cms.gov/newsroom/fact-sheets/2026-medicare-part-d-bid-information-and-part-d-premium-stabilization-demonstration-parameters",
      "host": "www.cms.gov",
      "organization": "Centers for Medicare & Medicaid Services",
      "source_class": "primary",
      "title": "2026 Medicare Part D Bid Information and Part D Premium Stabilization Demonstration Parameters",
      "published_at": "2025-07-28",
      "locator": "Fact sheet — base beneficiary premium announcement",
      "counts_toward_status": true
    },
    {
      "source_id": "src_cms_3",
      "url": "https://www.cms.gov/files/document/mm14279-medicare-deductible-coinsurance-premium-rates-cy-2026-update.pdf",
      "host": "www.cms.gov",
      "organization": "Centers for Medicare & Medicaid Services",
      "source_class": "primary",
      "title": "Medicare Deductible, Coinsurance & Premium Rates: CY 2026 Update (MM14279)",
      "published_at": "2025-11-20",
      "locator": "PDF — CY 2026 rates table",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kff_1",
      "url": "https://www.kff.org/quick-take/medicare-beneficiaries-are-not-insulated-from-affordability-challenges-as-part-b-premiums-rise-in-2026/",
      "host": "www.kff.org",
      "organization": "KFF (Kaiser Family Foundation)",
      "source_class": "secondary",
      "title": "Medicare Beneficiaries Are Not Insulated from Affordability Challenges As Part B Premiums Rise in 2026",
      "published_at": "2025-11-17",
      "locator": "Quote: 'Medicare Part B premiums will increase from $185.00 per month to $202.90 per month, a 10% increase.'",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_1",
      "url": "https://www.kiplinger.com/retirement/medicare/what-you-will-pay-for-medicare-in-2026",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "What You Will Pay for Medicare in 2026",
      "published_at": "2025-11-17",
      "locator": "Article body — Part B standard monthly premium ($202.90) and Part B annual deductible ($283)",
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
          "part_b_annual_deductible": 283.0,
          "part_b_standard_monthly_premium": 202.9,
          "part_d_base_beneficiary_premium": 38.99
        }
      }
    ]
  }
}
```

## Sources

- Centers for Medicare & Medicaid Services — 2026 Medicare Parts A & B Premiums and Deductibles — https://www.cms.gov/newsroom/fact-sheets/2026-medicare-parts-b-premiums-deductibles
- Centers for Medicare & Medicaid Services — 2026 Medicare Part D Bid Information and Part D Premium Stabilization Demonstration Parameters — https://www.cms.gov/newsroom/fact-sheets/2026-medicare-part-d-bid-information-and-part-d-premium-stabilization-demonstration-parameters
- Centers for Medicare & Medicaid Services — Medicare Deductible, Coinsurance & Premium Rates: CY 2026 Update (MM14279) — https://www.cms.gov/files/document/mm14279-medicare-deductible-coinsurance-premium-rates-cy-2026-update.pdf
- KFF (Kaiser Family Foundation) — Medicare Beneficiaries Are Not Insulated from Affordability Challenges As Part B Premiums Rise in 2026 — https://www.kff.org/quick-take/medicare-beneficiaries-are-not-insulated-from-affordability-challenges-as-part-b-premiums-rise-in-2026/
- Kiplinger — What You Will Pay for Medicare in 2026 — https://www.kiplinger.com/retirement/medicare/what-you-will-pay-for-medicare-in-2026
