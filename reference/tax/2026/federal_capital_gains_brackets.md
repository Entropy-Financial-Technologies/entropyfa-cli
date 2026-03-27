---
category: tax
year: 2026
key: federal_capital_gains_brackets
title: Federal Capital Gains Brackets
reviewed_artifact: tax/2026/federal_capital_gains_brackets
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Capital Gains Brackets

## What This Is

Federal long-term capital gains and qualified dividends tax rate brackets for tax year 2026, as published by the IRS in Revenue Procedure 2025-32 under §1(h) and §1(j)(5). Contains the taxable-income thresholds that determine whether the 0%, 15%, or 20% statutory rate applies, broken out by filing status.

## Lookup Parameters

- filing_status: one of single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse

## Interpretation Notes

- Each bracket has a min (inclusive) and max (exclusive for the boundary, i.e., max of 0% bracket = min of 15% bracket), with the final bracket having max = null (open-ended).
- The 'taxable income' used to determine the applicable rate is the taxpayer's total taxable income, not just the capital gain itself.
- The 0% rate applies when total taxable income (including the gain) does not exceed the 'Maximum Zero Rate Amount'.
- The 20% rate applies to the extent taxable income exceeds the 'Maximum 15 Percent Rate Amount'.
- Qualifying surviving spouse uses the same thresholds as married filing jointly.
- Rates of 0%, 15%, and 20% are the statutory long-term capital gains rates under §1(h).

## Does Not Include

- Short-term capital gains tax rates (taxed as ordinary income)
- Net Investment Income Tax (3.8% surtax under §1411)
- Unrecaptured §1250 gain rate (25%)
- Collectibles gain rate (28%)
- State capital gains taxes
- Estates and trusts capital gains brackets (separate thresholds: $3,300 / $16,250 for 2026)

## Caveats

- These brackets apply only to net long-term capital gains and qualified dividends taxed under §1(h); short-term capital gains are taxed as ordinary income.
- The 3.8% Net Investment Income Tax (NIIT) under §1411 may apply on top of these rates for higher-income taxpayers; it is not included in this dataset.
- Thresholds are based on taxable income (after deductions), not gross income or net capital gain amount alone.
- The 25% rate for unrecaptured §1250 gain and the 28% rate for collectibles gain are separate from this bracket structure and are not included.
- These thresholds are inflation-adjusted annually under §1(j)(5) as amended by the One, Big, Beautiful Bill Act; they are specific to tax year 2026.

## Typical Uses

- Determining the federal long-term capital gains tax rate tier based on a taxpayer's filing status and total taxable income
- Projecting after-tax proceeds from the sale of long-held assets
- Tax-loss harvesting and gain-realization planning in portfolio management

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_capital_gains_brackets",
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
      "title": "Revenue Procedure 2025-32: Inflation Adjusted Items for Taxable Years Beginning in 2025 and 2026",
      "published_at": "2025-10-08",
      "locator": "Section 4.03 — Maximum Capital Gains Rate (§1(h), §1(j)(5)), page 13, table of Maximum Zero Rate Amount and Maximum 15% Rate Amount by filing status",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_newsroom_1",
      "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill",
      "published_at": "2025-10-08",
      "locator": "Newsroom announcement referencing Rev. Proc. 2025-32 for all 2026 inflation adjustments",
      "counts_toward_status": false
    },
    {
      "source_id": "src_kiplinger_1",
      "url": "https://www.kiplinger.com/taxes/irs-updates-capital-gains-tax-thresholds",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "IRS Updates Capital Gains Tax Thresholds for 2026: Here's What's New",
      "published_at": "2025-10-09",
      "locator": "Article body: 2026 capital gains threshold table and filing-status breakdown",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_2",
      "url": "https://www.kiplinger.com/taxes/capital-gains-tax/602224/capital-gains-tax-rates",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "Capital Gains Tax Rates 2025 and 2026: Updated Brackets, Rules and Comparison",
      "published_at": "2025-10-09",
      "locator": "2026 capital gains rate comparison table",
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
        "value": [
          {
            "max": 49450.0,
            "min": 0.0,
            "rate": 0.0
          },
          {
            "max": 545500.0,
            "min": 49450.0,
            "rate": 0.15
          },
          {
            "max": null,
            "min": 545500.0,
            "rate": 0.2
          }
        ]
      },
      {
        "label": "married_filing_jointly",
        "params": {
          "filing_status": "married_filing_jointly",
          "lived_with_spouse_during_year": null
        },
        "value": [
          {
            "max": 98900.0,
            "min": 0.0,
            "rate": 0.0
          },
          {
            "max": 613700.0,
            "min": 98900.0,
            "rate": 0.15
          },
          {
            "max": null,
            "min": 613700.0,
            "rate": 0.2
          }
        ]
      },
      {
        "label": "married_filing_separately",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": null
        },
        "value": [
          {
            "max": 49450.0,
            "min": 0.0,
            "rate": 0.0
          },
          {
            "max": 306850.0,
            "min": 49450.0,
            "rate": 0.15
          },
          {
            "max": null,
            "min": 306850.0,
            "rate": 0.2
          }
        ]
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": [
          {
            "max": 66200.0,
            "min": 0.0,
            "rate": 0.0
          },
          {
            "max": 579600.0,
            "min": 66200.0,
            "rate": 0.15
          },
          {
            "max": null,
            "min": 579600.0,
            "rate": 0.2
          }
        ]
      },
      {
        "label": "qualifying_surviving_spouse",
        "params": {
          "filing_status": "qualifying_surviving_spouse",
          "lived_with_spouse_during_year": null
        },
        "value": [
          {
            "max": 98900.0,
            "min": 0.0,
            "rate": 0.0
          },
          {
            "max": 613700.0,
            "min": 98900.0,
            "rate": 0.15
          },
          {
            "max": null,
            "min": 613700.0,
            "rate": 0.2
          }
        ]
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Revenue Procedure 2025-32: Inflation Adjusted Items for Taxable Years Beginning in 2025 and 2026 — https://www.irs.gov/pub/irs-drop/rp-25-32.pdf
- Internal Revenue Service — IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill — https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
- Kiplinger — IRS Updates Capital Gains Tax Thresholds for 2026: Here's What's New — https://www.kiplinger.com/taxes/irs-updates-capital-gains-tax-thresholds
- Kiplinger — Capital Gains Tax Rates 2025 and 2026: Updated Brackets, Rules and Comparison — https://www.kiplinger.com/taxes/capital-gains-tax/602224/capital-gains-tax-rates
