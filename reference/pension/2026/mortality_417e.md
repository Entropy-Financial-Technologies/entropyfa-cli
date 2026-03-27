---
category: pension
year: 2026
key: mortality_417e
title: Mortality 417e
reviewed_artifact: pension/2026/mortality_417e
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Mortality 417e

## What This Is

The § 417(e)(3) applicable mortality table for 2026 plan years, published in IRS Notice 2025-40. Contains unisex qx (probability of death) values by attained age, derived from a 50/50 blend of male and female static combined mortality rates under § 1.430(h)(3)-1 per Rev. Rul. 2007-67.

## Lookup Parameters

- age (integer, 50-100 in this public entry)

## Interpretation Notes

- Each qx value is the probability of death within one year for a person of the given age, expressed as a decimal (e.g., 0.00090 = 0.09%).
- Age is the attained age at the start of the measurement interval.
- Values are ordered by strictly increasing age.
- The table is used to compute the minimum present value of lump-sum and other accelerated distributions from defined benefit pension plans under IRC § 417(e)(3).

## Does Not Include

- Sex-specific (male-only or female-only) mortality rates
- Generational mortality tables or mortality improvement projection scales
- Substitute plan-specific mortality tables under § 430(h)(3)(C)
- Disability-based mortality tables under § 430(h)(3)(D)
- Applicable interest rates (segment rates) used alongside this table for § 417(e) present-value calculations

## Caveats

- This table is updated annually; the 2026 table applies only to distributions with annuity starting dates during stability periods beginning in calendar year 2026.
- The unisex rates are a fixed 50/50 blend of male and female combined static mortality rates per Rev. Rul. 2007-67; they do not reflect plan-specific experience.
- The full IRS table covers ages 0-120, but only ages 50-100 are exposed in this public entry.

## Typical Uses

- Computing the minimum present value of a single-sum distribution from a defined benefit plan under § 417(e)(3)
- Determining the mortality component for converting an annuity benefit to an equivalent lump sum

## Machine Block

```json
{
  "schema_version": 1,
  "category": "pension",
  "key": "mortality_417e",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_1",
      "url": "https://www.irs.gov/pub/irs-drop/n-25-40.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service / Department of the Treasury",
      "source_class": "primary",
      "title": "Notice 2025-40: Updated Static Mortality Tables for Defined Benefit Pension Plans for 2026",
      "published_at": "2025-07-15",
      "locator": "Appendix, pages 4-6, column labeled 'Unisex'",
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
            "age": 50,
            "qx": 0.0009
          },
          {
            "age": 51,
            "qx": 0.001
          },
          {
            "age": 52,
            "qx": 0.00112
          },
          {
            "age": 53,
            "qx": 0.00125
          },
          {
            "age": 54,
            "qx": 0.00142
          },
          {
            "age": 55,
            "qx": 0.0017
          },
          {
            "age": 56,
            "qx": 0.00208
          },
          {
            "age": 57,
            "qx": 0.00242
          },
          {
            "age": 58,
            "qx": 0.00281
          },
          {
            "age": 59,
            "qx": 0.00322
          },
          {
            "age": 60,
            "qx": 0.00373
          },
          {
            "age": 61,
            "qx": 0.00427
          },
          {
            "age": 62,
            "qx": 0.00504
          },
          {
            "age": 63,
            "qx": 0.00582
          },
          {
            "age": 64,
            "qx": 0.00645
          },
          {
            "age": 65,
            "qx": 0.00729
          },
          {
            "age": 66,
            "qx": 0.00819
          },
          {
            "age": 67,
            "qx": 0.00906
          },
          {
            "age": 68,
            "qx": 0.01001
          },
          {
            "age": 69,
            "qx": 0.01108
          },
          {
            "age": 70,
            "qx": 0.01232
          },
          {
            "age": 71,
            "qx": 0.01374
          },
          {
            "age": 72,
            "qx": 0.01535
          },
          {
            "age": 73,
            "qx": 0.01717
          },
          {
            "age": 74,
            "qx": 0.01928
          },
          {
            "age": 75,
            "qx": 0.0217
          },
          {
            "age": 76,
            "qx": 0.02447
          },
          {
            "age": 77,
            "qx": 0.02761
          },
          {
            "age": 78,
            "qx": 0.0312
          },
          {
            "age": 79,
            "qx": 0.03528
          },
          {
            "age": 80,
            "qx": 0.04015
          },
          {
            "age": 81,
            "qx": 0.04512
          },
          {
            "age": 82,
            "qx": 0.0507
          },
          {
            "age": 83,
            "qx": 0.05697
          },
          {
            "age": 84,
            "qx": 0.06407
          },
          {
            "age": 85,
            "qx": 0.07216
          },
          {
            "age": 86,
            "qx": 0.08139
          },
          {
            "age": 87,
            "qx": 0.09182
          },
          {
            "age": 88,
            "qx": 0.10363
          },
          {
            "age": 89,
            "qx": 0.11672
          },
          {
            "age": 90,
            "qx": 0.13112
          },
          {
            "age": 91,
            "qx": 0.14617
          },
          {
            "age": 92,
            "qx": 0.1617
          },
          {
            "age": 93,
            "qx": 0.17767
          },
          {
            "age": 94,
            "qx": 0.19383
          },
          {
            "age": 95,
            "qx": 0.21015
          },
          {
            "age": 96,
            "qx": 0.22749
          },
          {
            "age": 97,
            "qx": 0.24529
          },
          {
            "age": 98,
            "qx": 0.26363
          },
          {
            "age": 99,
            "qx": 0.28252
          },
          {
            "age": 100,
            "qx": 0.30178
          }
        ]
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service / Department of the Treasury — Notice 2025-40: Updated Static Mortality Tables for Defined Benefit Pension Plans for 2026 — https://www.irs.gov/pub/irs-drop/n-25-40.pdf
