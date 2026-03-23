---
category: retirement
year: 2026
key: uniform_lifetime_table
title: Uniform Lifetime Table
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Uniform Lifetime Table

2026 Uniform Lifetime Table for required minimum distributions. Use this table for owner-account distribution-period lookups under the uniform lifetime method.

## Machine Block

```json
{
  "schema_version": 1,
  "category": "retirement",
  "key": "uniform_lifetime_table",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_1",
      "url": "https://www.irs.gov/publications/p590b",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Publication 590-B (2025), Distributions from Individual Retirement Arrangements (IRAs)",
      "published_at": "2025",
      "locator": "Appendix B, Table III (Uniform Lifetime)",
      "counts_toward_status": false
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/pub/irs-pdf/p590b.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Publication 590-B (2025) PDF — Distributions from Individual Retirement Arrangements (IRAs)",
      "published_at": "2025",
      "locator": "Appendix B, Table III (Uniform Lifetime), pages 63-64",
      "counts_toward_status": true
    },
    {
      "source_id": "src_capitalgroup_1",
      "url": "https://www.capitalgroup.com/individual/service-and-support/rmd/how-to-calculate/irs-uniform-lifetime-table.html",
      "host": "www.capitalgroup.com",
      "organization": "Capital Group",
      "source_class": "secondary",
      "title": "IRS Uniform Lifetime Table",
      "published_at": "2025",
      "locator": "Full table on page, ages 72-120+",
      "counts_toward_status": false
    },
    {
      "source_id": "src_smartasset_1",
      "url": "https://smartasset.com/retirement/rmd-table",
      "host": "smartasset.com",
      "organization": "SmartAsset",
      "source_class": "secondary",
      "title": "IRA Required Minimum Distribution (RMD) Table for 2026",
      "published_at": "2025",
      "locator": "Uniform Lifetime Table section, ages 73-120+",
      "counts_toward_status": false
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
            "age": 72,
            "distribution_period": 27.4
          },
          {
            "age": 73,
            "distribution_period": 26.5
          },
          {
            "age": 74,
            "distribution_period": 25.5
          },
          {
            "age": 75,
            "distribution_period": 24.6
          },
          {
            "age": 76,
            "distribution_period": 23.7
          },
          {
            "age": 77,
            "distribution_period": 22.9
          },
          {
            "age": 78,
            "distribution_period": 22.0
          },
          {
            "age": 79,
            "distribution_period": 21.1
          },
          {
            "age": 80,
            "distribution_period": 20.2
          },
          {
            "age": 81,
            "distribution_period": 19.4
          },
          {
            "age": 82,
            "distribution_period": 18.5
          },
          {
            "age": 83,
            "distribution_period": 17.7
          },
          {
            "age": 84,
            "distribution_period": 16.8
          },
          {
            "age": 85,
            "distribution_period": 16.0
          },
          {
            "age": 86,
            "distribution_period": 15.2
          },
          {
            "age": 87,
            "distribution_period": 14.4
          },
          {
            "age": 88,
            "distribution_period": 13.7
          },
          {
            "age": 89,
            "distribution_period": 12.9
          },
          {
            "age": 90,
            "distribution_period": 12.2
          },
          {
            "age": 91,
            "distribution_period": 11.5
          },
          {
            "age": 92,
            "distribution_period": 10.8
          },
          {
            "age": 93,
            "distribution_period": 10.1
          },
          {
            "age": 94,
            "distribution_period": 9.5
          },
          {
            "age": 95,
            "distribution_period": 8.9
          },
          {
            "age": 96,
            "distribution_period": 8.4
          },
          {
            "age": 97,
            "distribution_period": 7.8
          },
          {
            "age": 98,
            "distribution_period": 7.3
          },
          {
            "age": 99,
            "distribution_period": 6.8
          },
          {
            "age": 100,
            "distribution_period": 6.4
          },
          {
            "age": 101,
            "distribution_period": 6.0
          },
          {
            "age": 102,
            "distribution_period": 5.6
          },
          {
            "age": 103,
            "distribution_period": 5.2
          },
          {
            "age": 104,
            "distribution_period": 4.9
          },
          {
            "age": 105,
            "distribution_period": 4.6
          },
          {
            "age": 106,
            "distribution_period": 4.3
          },
          {
            "age": 107,
            "distribution_period": 4.1
          },
          {
            "age": 108,
            "distribution_period": 3.9
          },
          {
            "age": 109,
            "distribution_period": 3.7
          },
          {
            "age": 110,
            "distribution_period": 3.5
          },
          {
            "age": 111,
            "distribution_period": 3.4
          },
          {
            "age": 112,
            "distribution_period": 3.3
          },
          {
            "age": 113,
            "distribution_period": 3.1
          },
          {
            "age": 114,
            "distribution_period": 3.0
          },
          {
            "age": 115,
            "distribution_period": 2.9
          },
          {
            "age": 116,
            "distribution_period": 2.8
          },
          {
            "age": 117,
            "distribution_period": 2.7
          },
          {
            "age": 118,
            "distribution_period": 2.5
          },
          {
            "age": 119,
            "distribution_period": 2.3
          },
          {
            "age": 120,
            "distribution_period": 2.0
          }
        ]
      }
    ]
  }
}

```

## Review Notes

- Primary source is IRS Publication 590-B, Appendix B Table III.
- Canonical engine source: engine/src/data/retirement/rmd_tables.rs
