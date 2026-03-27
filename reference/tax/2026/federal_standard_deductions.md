---
category: tax
year: 2026
key: federal_standard_deductions
title: Federal Standard Deductions
reviewed_artifact: tax/2026/federal_standard_deductions
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Standard Deductions

## What This Is

Base standard deduction amounts for federal income tax, tax year 2026, by filing status. Published by the IRS in Revenue Procedure 2025-32 and reflecting inflation adjustments under IRC §63(c) as permanently extended by the One Big Beautiful Bill Act.

## Lookup Parameters

- filing_status: one of single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse

## Interpretation Notes

- Each variant is keyed by filing_status. The amount is a flat dollar value representing the maximum base standard deduction for that status.
- Married filing separately always equals the single filer amount.
- Qualifying surviving spouse always equals the married filing jointly amount.
- Amounts are annual and apply to tax year 2026 (returns filed in 2027).
- These amounts are set by IRC §63(c) as adjusted for inflation under IRC §1(f)(3) per Rev. Proc. 2025-32.

## Does Not Include

- Additional standard deduction for age 65 or older ($1,650 married/$2,050 unmarried for 2026)
- Additional standard deduction for blindness
- Enhanced senior standard deduction under OBBBA §102 (up to $4,000 single / $8,000 joint)
- Standard deduction for dependents (earned-income-based formula)
- State-level standard deductions

## Caveats

- These are base standard deduction amounts only. Additional standard deductions for age 65+ and blindness are separate parameters not included in this dataset.
- The One Big Beautiful Bill Act (signed July 4, 2025) permanently extended the TCJA's higher standard deduction. These amounts reflect that extension plus CY2026 inflation adjustments per Rev. Proc. 2025-32.
- Taxpayers claimed as dependents on another return may have a reduced standard deduction (limited to the greater of $1,350 or earned income plus $450 for 2026); that calculation is not captured here.
- The enhanced senior standard deduction (up to $4,000/$8,000 per OBBBA §102) is a separate line item and is not included in these base amounts.

## Typical Uses

- Determining whether a taxpayer benefits from itemizing vs. taking the standard deduction
- Estimating federal taxable income by subtracting the standard deduction from AGI
- Tax projection and withholding calculations for the upcoming filing year

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_standard_deductions",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_1",
      "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill",
      "published_at": "2025-10-09",
      "locator": "Standard deduction section listing amounts by filing status",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Revenue Procedure 2025-32",
      "published_at": "2025-10-09",
      "locator": "Section 3.01 — Standard Deduction",
      "counts_toward_status": true
    },
    {
      "source_id": "src_taxfoundation_1",
      "url": "https://taxfoundation.org/data/all/federal/2026-tax-brackets/",
      "host": "taxfoundation.org",
      "organization": "Tax Foundation",
      "source_class": "secondary",
      "title": "2026 Tax Brackets and Federal Income Tax Rates",
      "published_at": "2025-10-09",
      "locator": "Standard deduction table",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_1",
      "url": "https://www.kiplinger.com/taxes/standard-deduction-2026-amounts-are-here",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "A New Standard Deduction for 2026 is Here: What to Know Now",
      "published_at": "2025-10-09",
      "locator": "2026 standard deduction amounts table",
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
          "amount": 16100.0,
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
          "amount": 32200.0,
          "filing_status": "married_filing_jointly"
        }
      },
      {
        "label": "married_filing_separately",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "amount": 16100.0,
          "filing_status": "married_filing_separately"
        }
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "amount": 24150.0,
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
          "amount": 32200.0,
          "filing_status": "qualifying_surviving_spouse"
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill — https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
- Internal Revenue Service — Revenue Procedure 2025-32 — https://www.irs.gov/pub/irs-drop/rp-25-32.pdf
- Tax Foundation — 2026 Tax Brackets and Federal Income Tax Rates — https://taxfoundation.org/data/all/federal/2026-tax-brackets/
- Kiplinger — A New Standard Deduction for 2026 is Here: What to Know Now — https://www.kiplinger.com/taxes/standard-deduction-2026-amounts-are-here
