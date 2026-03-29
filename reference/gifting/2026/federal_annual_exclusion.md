---
category: gifting
year: 2026
key: federal_annual_exclusion
title: Federal Annual Exclusion
reviewed_artifact: gifting/2026/federal_annual_exclusion
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Annual Exclusion

## What This Is

Calendar-year 2026 federal gift tax annual exclusion amounts as published by the IRS in Rev. Proc. 2025-32. Contains two values: (1) the per-donee exclusion under IRC §2503(b), which is the maximum present-interest gift to any single recipient that is excluded from taxable gifts, and (2) the non-citizen spouse exclusion under IRC §2523(i), which is the higher annual exclusion that applies in lieu of the unlimited marital deduction when the donee spouse is not a U.S. citizen.

## Lookup Parameters

- calendar_year (2026) — both exclusion amounts are annual and specific to the calendar year

## Interpretation Notes

- per_donee_exclusion is a per-recipient, per-calendar-year dollar threshold. Gifts up to this amount to any single donee are excluded from taxable gifts under IRC §2503(b).
- non_citizen_spouse_exclusion is the per-calendar-year dollar threshold that replaces the per-donee exclusion for gifts to a spouse who is not a U.S. citizen, per IRC §2523(i)(2). It compensates for the unavailability of the unlimited marital deduction.
- Both values are nominal U.S. dollar amounts. No filing_status or lived_with_spouse_during_year parameter is needed to select the variant; the single default variant contains both values.

## Does Not Include

- The lifetime gift/estate tax unified credit (basic exclusion amount) under IRC §2010(c)
- The generation-skipping transfer (GST) tax exemption amount
- State-level gift tax exclusions or exemptions
- Annual exclusion amounts for prior or future calendar years

## Caveats

- The per-donee exclusion amount is not indexed every year; it increases only when cumulative inflation adjustments cross a $1,000 rounding threshold under IRC §2503(b)(2). The 2026 amount ($19,000) is unchanged from 2025.
- The non-citizen spouse exclusion under IRC §2523(i) is a separate inflation-adjusted amount and does not stack with the per-donee exclusion — it replaces it for gifts to a non-citizen spouse.
- These exclusions apply to gifts of present interests only; gifts of future interests do not qualify.
- Gift splitting under IRC §2513 allows a married couple to treat a gift by one spouse as made half by each, effectively doubling the per-donee exclusion, but requires a timely filed Form 709 election.

## Typical Uses

- Determining whether a gift triggers a Form 709 filing requirement
- Calculating the taxable portion of gifts exceeding the annual exclusion
- Planning spousal gifts when one spouse is not a U.S. citizen

## Machine Block

```json
{
  "schema_version": 1,
  "category": "gifting",
  "key": "federal_annual_exclusion",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_rp2025_32",
      "url": "https://www.irs.gov/irb/2025-45_IRB",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Rev. Proc. 2025-32 — Inflation Adjusted Items for Tax Year 2026",
      "published_at": "2025-10-09",
      "locator": "Section 4.42(1) and Section 4.42(2)",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_newsroom_2026",
      "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "IRS releases tax inflation adjustments for tax year 2026",
      "published_at": "2025-10-09",
      "locator": "Gift tax section",
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
          "non_citizen_spouse_exclusion": 194000,
          "per_donee_exclusion": 19000
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Rev. Proc. 2025-32 — Inflation Adjusted Items for Tax Year 2026 — https://www.irs.gov/irb/2025-45_IRB
- Internal Revenue Service — IRS releases tax inflation adjustments for tax year 2026 — https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
