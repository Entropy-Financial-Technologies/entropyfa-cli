---
category: tax
year: 2026
key: federal_estate_applicable_credit
title: Federal Estate Applicable Credit
reviewed_artifact: tax/2026/federal_estate_applicable_credit
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Estate Applicable Credit

## What This Is

The federal estate tax applicable credit amount for estates of decedents dying in calendar year 2026. This is the dollar amount of tax credit available under IRC § 2010(c) that offsets estate tax liability, corresponding to a basic exclusion amount of $15,000,000 as established by the One, Big, Beautiful Bill Act (P.L. 119-21).

## Lookup Parameters

- No variant parameters are needed. The dataset contains a single default variant representing the per-decedent applicable credit for calendar year 2026.

## Interpretation Notes

- The applicable credit is stated in dollars and represents the maximum estate tax liability that can be offset before any tax is owed. It is the tentative tax (per § 2001(c) rate schedule) on the basic exclusion amount.
- A single value (no filing_status or other variant parameters) applies because the estate tax basic exclusion amount is per-decedent, not per-filing-status.
- To determine whether an estate owes tax: compute the tentative tax on the taxable estate plus adjusted taxable gifts, then subtract this applicable credit. If the result is zero or negative, no federal estate tax is due.

## Does Not Include

- State-level estate or inheritance tax credits or exemptions
- Gift tax applicable credit (uses the same basic exclusion amount but is tracked separately via lifetime gift tax returns on Form 709)
- Generation-skipping transfer (GST) tax exemption amount (set separately under § 2631)
- The 40% marginal estate tax rate or graduated rate table under § 2001(c) — only the final credit amount is stored

## Caveats

- The applicable credit amount is not directly published by the IRS for 2026; it is derived by computing the tentative tax under IRC § 2001(c) on the basic exclusion amount of $15,000,000. This derivation follows the same methodology the IRS uses in Form 706 instructions for prior years.
- The $15,000,000 basic exclusion amount for 2026 was set by the One, Big, Beautiful Bill (P.L. 119-21, enacted July 4, 2025) amending § 2010(c)(3), not by standard inflation adjustment. Future years will apply inflation adjustments to this new base.
- Portability of the deceased spousal unused exclusion amount (DSUE) under § 2010(c)(4) may allow a surviving spouse to use the unused portion of a predeceased spouse's exclusion, effectively doubling the available credit for a married couple.

## Typical Uses

- Determining whether a decedent's estate exceeds the federal estate tax threshold for 2026
- Computing net federal estate tax liability by subtracting the applicable credit from the tentative tax

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_estate_applicable_credit",
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
      "published_at": "2025-08-01",
      "locator": "Estate tax basic exclusion amount section — $15,000,000 for estates of decedents dying in 2026",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/businesses/small-businesses-self-employed/whats-new-estate-and-gift-tax",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "What's new — Estate and gift tax",
      "published_at": "2025-08-01",
      "locator": "Basic exclusion amount for year of death table — 2026: $15,000,000; One, Big, Beautiful Bill section",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_3",
      "url": "https://www.irs.gov/instructions/i706",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Instructions for Form 706 (09/2025)",
      "published_at": "2025-09-01",
      "locator": "Applicable credit amount table — 2025: basic exclusion $13,990,000, applicable credit $5,541,800",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_4",
      "url": "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Revenue Procedure 2025-32",
      "published_at": "2025-08-01",
      "locator": "Section 3.41 — Estate tax basic exclusion amount for 2026",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_1",
      "url": "https://www.kiplinger.com/taxes/new-estate-tax-exemption-amount",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "2026 Estate Tax Exemption Amount: What You Need to Know",
      "published_at": "2025-08-01",
      "locator": "Article body — 2026 estate tax exemption of $15,000,000",
      "counts_toward_status": true
    },
    {
      "source_id": "src_taxfoundation_1",
      "url": "https://taxfoundation.org/research/all/federal/one-big-beautiful-bill-act-tax-changes/",
      "host": "taxfoundation.org",
      "organization": "Tax Foundation",
      "source_class": "secondary",
      "title": "Big Beautiful Bill Explained: Tax Changes FAQ",
      "published_at": "2025-07-01",
      "locator": "Estate tax section — OBBB increases basic exclusion to $15,000,000 for 2026",
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
          "applicable_credit": 5945800.0
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill — https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
- Internal Revenue Service — What's new — Estate and gift tax — https://www.irs.gov/businesses/small-businesses-self-employed/whats-new-estate-and-gift-tax
- Internal Revenue Service — Instructions for Form 706 (09/2025) — https://www.irs.gov/instructions/i706
- Internal Revenue Service — Revenue Procedure 2025-32 — https://www.irs.gov/pub/irs-drop/rp-25-32.pdf
- Kiplinger — 2026 Estate Tax Exemption Amount: What You Need to Know — https://www.kiplinger.com/taxes/new-estate-tax-exemption-amount
- Tax Foundation — Big Beautiful Bill Explained: Tax Changes FAQ — https://taxfoundation.org/research/all/federal/one-big-beautiful-bill-act-tax-changes/
