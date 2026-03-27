---
category: tax
year: 2026
key: federal_net_investment_income_tax
title: Federal Net Investment Income Tax
reviewed_artifact: tax/2026/federal_net_investment_income_tax
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Net Investment Income Tax

## What This Is

Net Investment Income Tax (NIIT) rate and Modified Adjusted Gross Income (MAGI) thresholds by federal filing status, as imposed by IRC Section 1411. The NIIT is a 3.8% surtax on the lesser of net investment income or the excess of MAGI over the filing-status threshold.

## Lookup Parameters

- filing_status — one of: single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse

## Interpretation Notes

- rate is a decimal (0.038 = 3.8%)
- threshold is a MAGI dollar amount; the tax applies only when a taxpayer's MAGI exceeds this value
- Each variant is keyed by filing_status; select the variant matching the taxpayer's federal filing status
- The tax applies to individuals only; estates and trusts have a different, inflation-adjusted threshold not stored here

## Does Not Include

- The definition or computation of net investment income (NII) itself — e.g., which categories of income count
- Estate and trust NIIT thresholds (which are inflation-adjusted separately)
- The additional 0.9% Medicare surtax on earned income, which is a separate provision

## Caveats

- The MAGI thresholds are statutory (IRC §1411(b)) and are NOT indexed for inflation; the same dollar amounts have applied since tax year 2013.
- The NIIT is computed on the lesser of (a) net investment income or (b) MAGI minus the applicable threshold; this dataset stores only the threshold and rate, not the computation rule.
- For married-filing-separately filers who lived with their spouse at any time during the year, the threshold is $125,000; however the lived_with_spouse_during_year parameter is null in this dataset because the $125,000 statutory threshold applies regardless.

## Typical Uses

- Determining whether a taxpayer's MAGI is above the NIIT threshold for their filing status
- Estimating the additional 3.8% surtax liability on net investment income

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_net_investment_income_tax",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_1",
      "url": "https://www.irs.gov/individuals/net-investment-income-tax",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Net Investment Income Tax",
      "published_at": "2025-02-05",
      "locator": "Filing status threshold table and 3.8% rate statement on main NIIT page",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/newsroom/questions-and-answers-on-the-net-investment-income-tax",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Questions and Answers on the Net Investment Income Tax",
      "published_at": "2024-11-15",
      "locator": "Q&A #1 threshold table listing all five filing statuses and their dollar thresholds; explicit statement that thresholds are not indexed for inflation",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_1",
      "url": "https://www.kiplinger.com/taxes/what-is-net-investment-income-tax",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "Net Investment Income Tax (NIIT) 2025: What Is It and Who Pays?",
      "published_at": "2025-01-01",
      "locator": "Article body — NIIT rate and threshold discussion",
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
          "rate": 0.038,
          "threshold": 200000.0
        }
      },
      {
        "label": "married_filing_jointly",
        "params": {
          "filing_status": "married_filing_jointly",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "rate": 0.038,
          "threshold": 250000.0
        }
      },
      {
        "label": "married_filing_separately",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "rate": 0.038,
          "threshold": 125000.0
        }
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "rate": 0.038,
          "threshold": 200000.0
        }
      },
      {
        "label": "qualifying_surviving_spouse",
        "params": {
          "filing_status": "qualifying_surviving_spouse",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "rate": 0.038,
          "threshold": 250000.0
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Net Investment Income Tax — https://www.irs.gov/individuals/net-investment-income-tax
- Internal Revenue Service — Questions and Answers on the Net Investment Income Tax — https://www.irs.gov/newsroom/questions-and-answers-on-the-net-investment-income-tax
- Kiplinger — Net Investment Income Tax (NIIT) 2025: What Is It and Who Pays? — https://www.kiplinger.com/taxes/what-is-net-investment-income-tax
