---
category: tax
year: 2026
key: federal_estate_exemption
title: Federal Estate Exemption
reviewed_artifact: tax/2026/federal_estate_exemption
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Estate Exemption

## What This Is

The 2026 federal estate tax basic exclusion amount (also called the estate tax exemption) under IRC § 2010(c)(3), as set by Revenue Procedure 2025-32 and the One, Big, Beautiful Bill Act (P.L. 119-21). This is the statutory per-decedent basic exclusion amount used in estate and gift tax calculations for deaths occurring in calendar year 2026; the actual tax-free amount for a specific estate depends on additional factors such as prior taxable gifts, DSUE, and applicable deductions.

## Lookup Parameters

- No variant parameters are required; the basic exclusion amount is a single flat figure that does not vary by filing status or other parameters

## Interpretation Notes

- This field stores the statutory basic exclusion amount under IRC § 2010(c)(3) for decedents dying in calendar year 2026; it is not a case-specific determination of whether a particular estate owes federal estate tax
- Whether a given estate owes tax depends on additional factors including adjusted taxable gifts, the deceased spousal unused exclusion (DSUE) amount, allowable deductions, and other unified transfer-tax mechanics
- For married couples, portability may allow the surviving spouse to use the deceased spouse's unused exclusion amount, but only if a timely Form 706 election is filed by the decedent's estate

## Does Not Include

- Gift tax annual exclusion amount (see federal_gift_tax_annual_exclusion)
- Generation-skipping transfer tax exemption (though currently set at the same amount)
- State estate or inheritance tax thresholds
- Estate tax rate schedule (40% top rate applies separately)

## Caveats

- The $15,000,000 basic exclusion amount was set by the One, Big, Beautiful Bill (P.L. 119-21) signed July 4, 2025, amending IRC § 2010(c)(3); future inflation adjustments apply to subsequent years
- This is a per-decedent amount; portability of a deceased spouse's unused exclusion requires a timely filed Form 706 election
- State-level estate taxes may apply at significantly lower thresholds and are not reflected in this dataset

## Typical Uses

- Benchmarking the federal estate-tax filing threshold; note that Form 706 filing is required when the gross estate plus adjusted taxable gifts exceeds this amount, and a timely filing may also be required solely to elect portability of the DSUE amount
- Calculating the unified credit against estate tax (the credit equivalent of the basic exclusion amount)

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_estate_exemption",
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
      "locator": "Paragraph: 'Estates of decedents who die during 2026 have a basic exclusion amount of $15,000,000, up from a total of $13,990,000 for estates of decedents who died in 2025.'",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/businesses/small-businesses-self-employed/whats-new-estate-and-gift-tax",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "What's new — Estate and gift tax",
      "published_at": "2025-10-09",
      "locator": "Section text: 'OBBB amends § 2010(c)(3) by increasing the basic exclusion amount to $15,000,000 for calendar year 2026'",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_3",
      "url": "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Revenue Procedure 2025-32",
      "published_at": "2025-10-09",
      "locator": "SECTION 2, paragraph .14, page 7: OBBBA amended IRC section 2010(c)(3) to increase the basic exclusion amount to $15,000,000 for calendar year 2026",
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
          "exemption": 15000000.0
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill — https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
- Internal Revenue Service — What's new — Estate and gift tax — https://www.irs.gov/businesses/small-businesses-self-employed/whats-new-estate-and-gift-tax
- Internal Revenue Service — Revenue Procedure 2025-32 — https://www.irs.gov/pub/irs-drop/rp-25-32.pdf
