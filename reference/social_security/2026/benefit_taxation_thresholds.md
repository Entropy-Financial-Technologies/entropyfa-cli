---
category: social_security
year: 2026
key: benefit_taxation_thresholds
title: Benefit Taxation Thresholds
reviewed_artifact: social_security/2026/benefit_taxation_thresholds
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Benefit Taxation Thresholds

## What This Is

Federal income tax thresholds that determine the taxable percentage of Social Security benefits, organized by filing status. Defined by IRC §86 and published annually in IRS Publication 915. The dataset contains the base amount (lower threshold), upper amount (adjusted base amount / upper threshold), and the maximum taxable percentage of benefits for provisional income falling in each bracket.

## Lookup Parameters

- filing_status: one of single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse
- lived_with_spouse_during_year: boolean, only meaningful when filing_status is married_filing_separately

## Interpretation Notes

- Each variant is keyed by filing_status and (for MFS) lived_with_spouse_during_year boolean.
- base_amount is the lower provisional-income threshold: below it, $0 of benefits are taxable.
- upper_amount is the higher provisional-income threshold: between base_amount and upper_amount, up to max_taxable_pct_below_upper (50%) of benefits may be taxable.
- Above upper_amount, up to max_taxable_pct_above_upper (85%) of benefits may be taxable.
- All dollar amounts are annual and apply to the taxpayer's provisional income (also called combined income).
- MFS lived apart from spouse for the entire year uses the same thresholds as single/HoH/QSS.

## Does Not Include

- State-level taxation of Social Security benefits (varies by state)
- Railroad Retirement Tier I benefits (treated separately under equivalent provisions)
- The provisional income calculation itself (AGI + tax-exempt interest + 50% of SS benefits)
- Social Security payroll tax thresholds or wage base limits
- Net benefits after Medicare premium deductions

## Caveats

- These thresholds are set by IRC §86(c) and have not been indexed for inflation since enactment in 1993. They remain fixed at the same dollar amounts regardless of year.
- The 2026 edition of IRS Publication 915 has not yet been released as of March 2026. The 2025 edition (covering tax year 2025) is the most recent. Because these thresholds are statutory and have never changed, the 2025 Pub 915 is authoritative for the 2026 values.
- For married filing separately where the taxpayer lived with their spouse at any time during the year, both base_amount and upper_amount are $0, making up to 85% of benefits immediately taxable from the first dollar of provisional income.
- The max_taxable_pct values (0.50 and 0.85) are caps on the taxable fraction of total benefits, not marginal tax rates. The actual taxable amount is computed via IRS Worksheet 1 in Pub 915.

## Typical Uses

- Determining whether and what portion of a taxpayer's Social Security benefits are subject to federal income tax
- Computing the taxable Social Security benefit amount on Form 1040 line 6b

## Machine Block

```json
{
  "schema_version": 1,
  "category": "social_security",
  "key": "benefit_taxation_thresholds",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_pub915_1",
      "url": "https://www.irs.gov/publications/p915",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Publication 915 (2025), Social Security and Equivalent Railroad Retirement Benefits",
      "published_at": "2025",
      "locator": "Section 'Are Any of Your Benefits Taxable?', base amount and adjusted base amount tables; Worksheet 1",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_faq_1",
      "url": "https://www.irs.gov/faqs/social-security-income",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Social Security Income — Frequently Asked Questions",
      "published_at": "2025",
      "locator": "FAQ answer listing base amounts by filing status ($25,000 single, $32,000 MFJ, $0 MFS lived with spouse)",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_newsroom_1",
      "url": "https://www.irs.gov/newsroom/irs-reminds-taxpayers-their-social-security-benefits-may-be-taxable",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "IRS Reminds Taxpayers Their Social Security Benefits May Be Taxable",
      "published_at": "2022-02-09",
      "locator": "Threshold table listing $25,000–$34,000 (single) and $32,000–$44,000 (MFJ) with 50%/85% brackets",
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
          "base_amount": 25000.0,
          "filing_status": "single",
          "max_taxable_pct_above_upper": 0.85,
          "max_taxable_pct_below_upper": 0.5,
          "upper_amount": 34000.0
        }
      },
      {
        "label": "married_filing_jointly",
        "params": {
          "filing_status": "married_filing_jointly",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "base_amount": 32000.0,
          "filing_status": "married_filing_jointly",
          "max_taxable_pct_above_upper": 0.85,
          "max_taxable_pct_below_upper": 0.5,
          "upper_amount": 44000.0
        }
      },
      {
        "label": "married_filing_separately_lived_with_spouse",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": true
        },
        "value": {
          "base_amount": 0.0,
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": true,
          "max_taxable_pct_above_upper": 0.85,
          "max_taxable_pct_below_upper": 0.5,
          "upper_amount": 0.0
        }
      },
      {
        "label": "married_filing_separately_lived_apart",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": false
        },
        "value": {
          "base_amount": 25000.0,
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": false,
          "max_taxable_pct_above_upper": 0.85,
          "max_taxable_pct_below_upper": 0.5,
          "upper_amount": 34000.0
        }
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "base_amount": 25000.0,
          "filing_status": "head_of_household",
          "max_taxable_pct_above_upper": 0.85,
          "max_taxable_pct_below_upper": 0.5,
          "upper_amount": 34000.0
        }
      },
      {
        "label": "qualifying_surviving_spouse",
        "params": {
          "filing_status": "qualifying_surviving_spouse",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "base_amount": 25000.0,
          "filing_status": "qualifying_surviving_spouse",
          "max_taxable_pct_above_upper": 0.85,
          "max_taxable_pct_below_upper": 0.5,
          "upper_amount": 34000.0
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Publication 915 (2025), Social Security and Equivalent Railroad Retirement Benefits — https://www.irs.gov/publications/p915
- Internal Revenue Service — Social Security Income — Frequently Asked Questions — https://www.irs.gov/faqs/social-security-income
- Internal Revenue Service — IRS Reminds Taxpayers Their Social Security Benefits May Be Taxable — https://www.irs.gov/newsroom/irs-reminds-taxpayers-their-social-security-benefits-may-be-taxable
