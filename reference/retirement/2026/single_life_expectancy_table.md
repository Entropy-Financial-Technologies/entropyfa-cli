---
category: retirement
year: 2026
key: single_life_expectancy_table
title: Single Life Expectancy Table
reviewed_artifact: retirement/2026/single_life_table
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Single Life Expectancy Table

2026 Single Life Expectancy Table. Use this table for recalculated and non-recalculated life expectancy methods that reference a single age column.

## Machine Block

```json
{
  "schema_version": 1,
  "category": "retirement",
  "key": "single_life_expectancy_table",
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
      "locator": "Appendix B, Table I (Single Life Expectancy)",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/pub/irs-pdf/p590b.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Publication 590-B (2025) PDF - Distributions from Individual Retirement Arrangements (IRAs)",
      "published_at": "2025",
      "locator": "Appendix B, Table I (Single Life Expectancy), pages near end of document",
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
            "age": 0,
            "distribution_period": 84.6
          },
          {
            "age": 1,
            "distribution_period": 83.7
          },
          {
            "age": 2,
            "distribution_period": 82.8
          },
          {
            "age": 3,
            "distribution_period": 81.8
          },
          {
            "age": 4,
            "distribution_period": 80.8
          },
          {
            "age": 5,
            "distribution_period": 79.8
          },
          {
            "age": 6,
            "distribution_period": 78.8
          },
          {
            "age": 7,
            "distribution_period": 77.9
          },
          {
            "age": 8,
            "distribution_period": 76.9
          },
          {
            "age": 9,
            "distribution_period": 75.9
          },
          {
            "age": 10,
            "distribution_period": 74.9
          },
          {
            "age": 11,
            "distribution_period": 73.9
          },
          {
            "age": 12,
            "distribution_period": 72.9
          },
          {
            "age": 13,
            "distribution_period": 71.9
          },
          {
            "age": 14,
            "distribution_period": 70.9
          },
          {
            "age": 15,
            "distribution_period": 69.9
          },
          {
            "age": 16,
            "distribution_period": 69.0
          },
          {
            "age": 17,
            "distribution_period": 68.0
          },
          {
            "age": 18,
            "distribution_period": 67.0
          },
          {
            "age": 19,
            "distribution_period": 66.0
          },
          {
            "age": 20,
            "distribution_period": 65.0
          },
          {
            "age": 21,
            "distribution_period": 64.1
          },
          {
            "age": 22,
            "distribution_period": 63.1
          },
          {
            "age": 23,
            "distribution_period": 62.1
          },
          {
            "age": 24,
            "distribution_period": 61.1
          },
          {
            "age": 25,
            "distribution_period": 60.2
          },
          {
            "age": 26,
            "distribution_period": 59.2
          },
          {
            "age": 27,
            "distribution_period": 58.2
          },
          {
            "age": 28,
            "distribution_period": 57.3
          },
          {
            "age": 29,
            "distribution_period": 56.3
          },
          {
            "age": 30,
            "distribution_period": 55.3
          },
          {
            "age": 31,
            "distribution_period": 54.4
          },
          {
            "age": 32,
            "distribution_period": 53.4
          },
          {
            "age": 33,
            "distribution_period": 52.5
          },
          {
            "age": 34,
            "distribution_period": 51.5
          },
          {
            "age": 35,
            "distribution_period": 50.5
          },
          {
            "age": 36,
            "distribution_period": 49.6
          },
          {
            "age": 37,
            "distribution_period": 48.6
          },
          {
            "age": 38,
            "distribution_period": 47.7
          },
          {
            "age": 39,
            "distribution_period": 46.7
          },
          {
            "age": 40,
            "distribution_period": 45.7
          },
          {
            "age": 41,
            "distribution_period": 44.8
          },
          {
            "age": 42,
            "distribution_period": 43.8
          },
          {
            "age": 43,
            "distribution_period": 42.9
          },
          {
            "age": 44,
            "distribution_period": 41.9
          },
          {
            "age": 45,
            "distribution_period": 41.0
          },
          {
            "age": 46,
            "distribution_period": 40.0
          },
          {
            "age": 47,
            "distribution_period": 39.0
          },
          {
            "age": 48,
            "distribution_period": 38.1
          },
          {
            "age": 49,
            "distribution_period": 37.1
          },
          {
            "age": 50,
            "distribution_period": 36.2
          },
          {
            "age": 51,
            "distribution_period": 35.3
          },
          {
            "age": 52,
            "distribution_period": 34.3
          },
          {
            "age": 53,
            "distribution_period": 33.4
          },
          {
            "age": 54,
            "distribution_period": 32.5
          },
          {
            "age": 55,
            "distribution_period": 31.6
          },
          {
            "age": 56,
            "distribution_period": 30.6
          },
          {
            "age": 57,
            "distribution_period": 29.8
          },
          {
            "age": 58,
            "distribution_period": 28.9
          },
          {
            "age": 59,
            "distribution_period": 28.0
          },
          {
            "age": 60,
            "distribution_period": 27.1
          },
          {
            "age": 61,
            "distribution_period": 26.2
          },
          {
            "age": 62,
            "distribution_period": 25.4
          },
          {
            "age": 63,
            "distribution_period": 24.5
          },
          {
            "age": 64,
            "distribution_period": 23.7
          },
          {
            "age": 65,
            "distribution_period": 22.9
          },
          {
            "age": 66,
            "distribution_period": 22.0
          },
          {
            "age": 67,
            "distribution_period": 21.2
          },
          {
            "age": 68,
            "distribution_period": 20.4
          },
          {
            "age": 69,
            "distribution_period": 19.6
          },
          {
            "age": 70,
            "distribution_period": 18.8
          },
          {
            "age": 71,
            "distribution_period": 18.0
          },
          {
            "age": 72,
            "distribution_period": 17.2
          },
          {
            "age": 73,
            "distribution_period": 16.4
          },
          {
            "age": 74,
            "distribution_period": 15.6
          },
          {
            "age": 75,
            "distribution_period": 14.8
          },
          {
            "age": 76,
            "distribution_period": 14.1
          },
          {
            "age": 77,
            "distribution_period": 13.3
          },
          {
            "age": 78,
            "distribution_period": 12.6
          },
          {
            "age": 79,
            "distribution_period": 11.9
          },
          {
            "age": 80,
            "distribution_period": 11.2
          },
          {
            "age": 81,
            "distribution_period": 10.5
          },
          {
            "age": 82,
            "distribution_period": 9.9
          },
          {
            "age": 83,
            "distribution_period": 9.3
          },
          {
            "age": 84,
            "distribution_period": 8.7
          },
          {
            "age": 85,
            "distribution_period": 8.1
          },
          {
            "age": 86,
            "distribution_period": 7.6
          },
          {
            "age": 87,
            "distribution_period": 7.1
          },
          {
            "age": 88,
            "distribution_period": 6.6
          },
          {
            "age": 89,
            "distribution_period": 6.1
          },
          {
            "age": 90,
            "distribution_period": 5.7
          },
          {
            "age": 91,
            "distribution_period": 5.3
          },
          {
            "age": 92,
            "distribution_period": 4.9
          },
          {
            "age": 93,
            "distribution_period": 4.6
          },
          {
            "age": 94,
            "distribution_period": 4.3
          },
          {
            "age": 95,
            "distribution_period": 4.0
          },
          {
            "age": 96,
            "distribution_period": 3.7
          },
          {
            "age": 97,
            "distribution_period": 3.4
          },
          {
            "age": 98,
            "distribution_period": 3.2
          },
          {
            "age": 99,
            "distribution_period": 3.0
          },
          {
            "age": 100,
            "distribution_period": 2.8
          },
          {
            "age": 101,
            "distribution_period": 2.6
          },
          {
            "age": 102,
            "distribution_period": 2.5
          },
          {
            "age": 103,
            "distribution_period": 2.3
          },
          {
            "age": 104,
            "distribution_period": 2.2
          },
          {
            "age": 105,
            "distribution_period": 2.1
          },
          {
            "age": 106,
            "distribution_period": 2.1
          },
          {
            "age": 107,
            "distribution_period": 2.1
          },
          {
            "age": 108,
            "distribution_period": 2.0
          },
          {
            "age": 109,
            "distribution_period": 2.0
          },
          {
            "age": 110,
            "distribution_period": 2.0
          },
          {
            "age": 111,
            "distribution_period": 2.0
          },
          {
            "age": 112,
            "distribution_period": 2.0
          },
          {
            "age": 113,
            "distribution_period": 1.9
          },
          {
            "age": 114,
            "distribution_period": 1.9
          },
          {
            "age": 115,
            "distribution_period": 1.8
          },
          {
            "age": 116,
            "distribution_period": 1.8
          },
          {
            "age": 117,
            "distribution_period": 1.6
          },
          {
            "age": 118,
            "distribution_period": 1.4
          },
          {
            "age": 119,
            "distribution_period": 1.1
          },
          {
            "age": 120,
            "distribution_period": 1.0
          }
        ]
      }
    ]
  }
}

```

## Review Notes

- Primary source is IRS Publication 590-B, Appendix B Table I.
- Canonical engine source: engine/src/data/retirement/rmd_tables.rs
