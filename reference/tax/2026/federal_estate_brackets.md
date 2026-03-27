---
category: tax
year: 2026
key: federal_estate_brackets
title: Federal Estate Brackets
reviewed_artifact: tax/2026/federal_estate_brackets
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Estate Brackets

## What This Is

The federal estate (and gift) tax unified rate schedule as published in IRS Form 706 Instructions Table A, containing 12 progressive marginal rate brackets from 18% on the first $10,000 to 40% on amounts over $1,000,000 of cumulative taxable transfers.

## Lookup Parameters

- No variant parameters are needed; the rate schedule is universal (filing_status and lived_with_spouse_during_year are both null).

## Interpretation Notes

- Each bracket defines a marginal rate applied to the portion of the taxable amount falling between min (inclusive) and max (exclusive), with max = null for the final open-ended tier at 40%.
- The 'taxable amount' is the total of adjusted taxable gifts plus the taxable estate before application of the unified credit.
- Rates are expressed as decimals (0.18 = 18%).
- The bracket structure is progressive: the first $10,000 is taxed at 18%, the next $10,000 at 20%, and so on up to 40% on amounts over $1,000,000.

## Does Not Include

- The basic exclusion amount (unified credit) — stored separately as federal_estate_applicable_credit
- State-level estate or inheritance taxes
- Generation-skipping transfer (GST) tax rates
- Gift tax annual exclusion amounts
- Portability elections or deceased spousal unused exclusion (DSUE) amounts

## Caveats

- The rate schedule is statutory under IRC §2001(c) and does not change annually; only the basic exclusion amount is inflation-adjusted.
- The table applies to the cumulative total of taxable transfers (lifetime gifts plus estate), not solely the estate at death.
- The unified credit effectively zeroes out tax on the first $15,000,000 of cumulative transfers for decedents dying in 2026, so most of these brackets have no practical tax impact for estates below that threshold.

## Typical Uses

- Computing the tentative estate tax on the sum of the taxable estate and adjusted taxable gifts before subtracting the unified credit.
- Determining the marginal rate applicable to a given transfer amount tier.

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_estate_brackets",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_1",
      "url": "https://www.irs.gov/instructions/i706",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Instructions for Form 706 (Rev. September 2025)",
      "published_at": "2025-09",
      "locator": "Table A — Unified Rate Schedule",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "IRS releases tax inflation adjustments for tax year 2026",
      "published_at": "2025-10",
      "locator": "Estate tax basic exclusion amount section",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_1",
      "url": "https://www.kiplinger.com/taxes/new-estate-tax-exemption-amount",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "2026 Estate Tax Exemption Amount: What You Need to Know",
      "published_at": "2025",
      "locator": "Estate tax rate range discussion",
      "counts_toward_status": true
    },
    {
      "source_id": "src_taxfoundation_1",
      "url": "https://taxfoundation.org/data/all/federal/2026-tax-brackets/",
      "host": "taxfoundation.org",
      "organization": "Tax Foundation",
      "source_class": "secondary",
      "title": "2026 Tax Brackets and Federal Income Tax Rates",
      "published_at": "2025",
      "locator": "Estate tax exemption section",
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
        "value": [
          {
            "max": 10000.0,
            "min": 0.0,
            "rate": 0.18
          },
          {
            "max": 20000.0,
            "min": 10000.0,
            "rate": 0.2
          },
          {
            "max": 40000.0,
            "min": 20000.0,
            "rate": 0.22
          },
          {
            "max": 60000.0,
            "min": 40000.0,
            "rate": 0.24
          },
          {
            "max": 80000.0,
            "min": 60000.0,
            "rate": 0.26
          },
          {
            "max": 100000.0,
            "min": 80000.0,
            "rate": 0.28
          },
          {
            "max": 150000.0,
            "min": 100000.0,
            "rate": 0.3
          },
          {
            "max": 250000.0,
            "min": 150000.0,
            "rate": 0.32
          },
          {
            "max": 500000.0,
            "min": 250000.0,
            "rate": 0.34
          },
          {
            "max": 750000.0,
            "min": 500000.0,
            "rate": 0.37
          },
          {
            "max": 1000000.0,
            "min": 750000.0,
            "rate": 0.39
          },
          {
            "max": null,
            "min": 1000000.0,
            "rate": 0.4
          }
        ]
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Instructions for Form 706 (Rev. September 2025) — https://www.irs.gov/instructions/i706
- Internal Revenue Service — IRS releases tax inflation adjustments for tax year 2026 — https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
- Kiplinger — 2026 Estate Tax Exemption Amount: What You Need to Know — https://www.kiplinger.com/taxes/new-estate-tax-exemption-amount
- Tax Foundation — 2026 Tax Brackets and Federal Income Tax Rates — https://taxfoundation.org/data/all/federal/2026-tax-brackets/
