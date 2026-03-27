---
category: tax
year: 2026
key: federal_capital_loss_limit
title: Federal Capital Loss Limit
reviewed_artifact: tax/2026/federal_capital_loss_limit
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Capital Loss Limit

## What This Is

The annual limit on net capital losses that an individual taxpayer may deduct against ordinary income, per IRC §1211(b), broken out by federal filing status for the 2026 tax year.

## Lookup Parameters

- filing_status — one of: single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse

## Interpretation Notes

- Each variant stores a single numeric 'limit' field representing the maximum net capital loss deductible against ordinary income per tax year for that filing status.
- The limit is applied after netting all short-term and long-term capital gains and losses; only the excess net loss is capped.
- All filing statuses except married_filing_separately share the same $3,000 limit; married_filing_separately is $1,500.

## Does Not Include

- Capital gains tax rates or bracket thresholds
- Net investment income tax (NIIT) thresholds
- Capital loss carryforward calculation rules or worksheets
- Section 1244 small business stock loss limits (up to $50,000/$100,000), which are treated as ordinary losses rather than capital losses

## Caveats

- The $3,000/$1,500 limit is statutory (IRC §1211(b)) and is NOT indexed for inflation; it has been unchanged since 1978.
- The limit applies only to net capital losses deducted against ordinary income; capital losses may offset unlimited capital gains before this cap applies.
- Married-filing-separately filers who lived with their spouse at any time during the year are subject to additional passive-activity and wash-sale interactions, but the capital loss deduction limit itself does not vary by cohabitation status.

## Typical Uses

- Determining the maximum capital loss offset against ordinary income in tax-year projections
- Calculating after-tax impact of tax-loss harvesting strategies

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_capital_loss_limit",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_topic409",
      "url": "https://www.irs.gov/taxtopics/tc409",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Topic No. 409, Capital Gains and Losses",
      "published_at": "2026-02-25",
      "locator": "Section: Limit on the deduction — \"the lesser of $3,000 ($1,500 if married filing separately) or your total net loss\"",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_pub550",
      "url": "https://www.irs.gov/publications/p550",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Publication 550 (2025), Investment Income and Expenses",
      "published_at": "2025-12-01",
      "locator": "Chapter 4 > Capital Losses > Limit on deduction",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_i1040sd",
      "url": "https://www.irs.gov/instructions/i1040sd",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Instructions for Schedule D (Form 1040) (2025)",
      "published_at": "2025-12-01",
      "locator": "Capital Loss Limit section — \"You can deduct capital losses up to the amount of your capital gains plus $3,000 ($1,500 if married filing separately)\"",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_capgains",
      "url": "https://www.kiplinger.com/taxes/capital-gains-tax/602224/capital-gains-tax-rates",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "Capital Gains Tax Rates 2025 and 2026: Updated Brackets, Rules and Comparison",
      "published_at": "2026-01-01",
      "locator": "Capital loss limit discussion within article",
      "counts_toward_status": true
    },
    {
      "source_id": "src_taxfoundation_brackets",
      "url": "https://taxfoundation.org/data/all/federal/2025-tax-brackets/",
      "host": "taxfoundation.org",
      "organization": "Tax Foundation",
      "source_class": "secondary",
      "title": "2025 Tax Brackets and Federal Income Tax Rates",
      "published_at": "2025-11-01",
      "locator": "Capital gains and losses section",
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
          "filing_status": "single",
          "limit": 3000.0
        }
      },
      {
        "label": "married_filing_jointly",
        "params": {
          "filing_status": "married_filing_jointly",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "filing_status": "married_filing_jointly",
          "limit": 3000.0
        }
      },
      {
        "label": "married_filing_separately",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "filing_status": "married_filing_separately",
          "limit": 1500.0
        }
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "filing_status": "head_of_household",
          "limit": 3000.0
        }
      },
      {
        "label": "qualifying_surviving_spouse",
        "params": {
          "filing_status": "qualifying_surviving_spouse",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "filing_status": "qualifying_surviving_spouse",
          "limit": 3000.0
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Topic No. 409, Capital Gains and Losses — https://www.irs.gov/taxtopics/tc409
- Internal Revenue Service — Publication 550 (2025), Investment Income and Expenses — https://www.irs.gov/publications/p550
- Internal Revenue Service — Instructions for Schedule D (Form 1040) (2025) — https://www.irs.gov/instructions/i1040sd
- Kiplinger — Capital Gains Tax Rates 2025 and 2026: Updated Brackets, Rules and Comparison — https://www.kiplinger.com/taxes/capital-gains-tax/602224/capital-gains-tax-rates
- Tax Foundation — 2025 Tax Brackets and Federal Income Tax Rates — https://taxfoundation.org/data/all/federal/2025-tax-brackets/
