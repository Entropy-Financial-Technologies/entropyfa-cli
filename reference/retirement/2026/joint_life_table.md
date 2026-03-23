---
category: retirement
year: 2026
key: joint_life_table
title: Joint Life Table
verification_status: authoritative
review_status: reviewed
source_registry: engine/data_registry/2026/reviewed/retirement/joint_life_table.json
---

# Joint Life Table

2026 Joint Life and Last Survivor Expectancy Table. Use this table when the distribution period depends on the owner and spouse ages.

## Machine Block

```json
{
  "schema_version": 1,
  "category": "retirement",
  "key": "joint_life_table",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_1",
      "url": "https://www.irs.gov/pub/irs-pdf/p590b.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Publication 590-B (2025), Distributions from Individual Retirement Arrangements (IRAs)",
      "published_at": "2026-01-21",
      "locator": "Appendix B, Table II (Joint Life and Last Survivor Expectancy), pages 52-66",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/publications/p590b",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Publication 590-B (2025), Distributions from Individual Retirement Arrangements (IRAs) - HTML version",
      "published_at": "2026-01-21",
      "locator": "Appendix B, Table II",
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
            "distribution_period": 72.0,
            "owner_age": 20,
            "spouse_age": 20
          },
          {
            "distribution_period": 71.5,
            "owner_age": 20,
            "spouse_age": 21
          },
          {
            "distribution_period": 71.0,
            "owner_age": 20,
            "spouse_age": 22
          },
          {
            "distribution_period": 70.6,
            "owner_age": 20,
            "spouse_age": 23
          },
          {
            "distribution_period": 70.2,
            "owner_age": 20,
            "spouse_age": 24
          },
          {
            "distribution_period": 69.8,
            "owner_age": 20,
            "spouse_age": 25
          },
          {
            "distribution_period": 69.5,
            "owner_age": 20,
            "spouse_age": 26
          },
          {
            "distribution_period": 69.1,
            "owner_age": 20,
            "spouse_age": 27
          },
          {
            "distribution_period": 68.8,
            "owner_age": 20,
            "spouse_age": 28
          },
          {
            "distribution_period": 68.5,
            "owner_age": 20,
            "spouse_age": 29
          },
          {
            "distribution_period": 71.5,
            "owner_age": 21,
            "spouse_age": 20
          },
          {
            "distribution_period": 71.0,
            "owner_age": 21,
            "spouse_age": 21
          },
          {
            "distribution_period": 70.5,
            "owner_age": 21,
            "spouse_age": 22
          },
          {
            "distribution_period": 70.0,
            "owner_age": 21,
            "spouse_age": 23
          },
          {
            "distribution_period": 69.6,
            "owner_age": 21,
            "spouse_age": 24
          },
          {
            "distribution_period": 69.2,
            "owner_age": 21,
            "spouse_age": 25
          },
          {
            "distribution_period": 68.8,
            "owner_age": 21,
            "spouse_age": 26
          },
          {
            "distribution_period": 68.5,
            "owner_age": 21,
            "spouse_age": 27
          },
          {
            "distribution_period": 68.1,
            "owner_age": 21,
            "spouse_age": 28
          },
          {
            "distribution_period": 67.8,
            "owner_age": 21,
            "spouse_age": 29
          },
          {
            "distribution_period": 71.0,
            "owner_age": 22,
            "spouse_age": 20
          },
          {
            "distribution_period": 70.5,
            "owner_age": 22,
            "spouse_age": 21
          },
          {
            "distribution_period": 70.0,
            "owner_age": 22,
            "spouse_age": 22
          },
          {
            "distribution_period": 69.5,
            "owner_age": 22,
            "spouse_age": 23
          },
          {
            "distribution_period": 69.0,
            "owner_age": 22,
            "spouse_age": 24
          },
          {
            "distribution_period": 68.6,
            "owner_age": 22,
            "spouse_age": 25
          },
          {
            "distribution_period": 68.2,
            "owner_age": 22,
            "spouse_age": 26
          },
          {
            "distribution_period": 67.8,
            "owner_age": 22,
            "spouse_age": 27
          },
          {
            "distribution_period": 67.5,
            "owner_age": 22,
            "spouse_age": 28
          },
          {
            "distribution_period": 67.1,
            "owner_age": 22,
            "spouse_age": 29
          },
          {
            "distribution_period": 70.6,
            "owner_age": 23,
            "spouse_age": 20
          },
          {
            "distribution_period": 70.0,
            "owner_age": 23,
            "spouse_age": 21
          },
          {
            "distribution_period": 69.5,
            "owner_age": 23,
            "spouse_age": 22
          },
          {
            "distribution_period": 69.0,
            "owner_age": 23,
            "spouse_age": 23
          },
          {
            "distribution_period": 68.5,
            "owner_age": 23,
            "spouse_age": 24
          },
          {
            "distribution_period": 68.0,
            "owner_age": 23,
            "spouse_age": 25
          },
          {
            "distribution_period": 67.6,
            "owner_age": 23,
            "spouse_age": 26
          },
          {
            "distribution_period": 67.2,
            "owner_age": 23,
            "spouse_age": 27
          },
          {
            "distribution_period": 66.8,
            "owner_age": 23,
            "spouse_age": 28
          },
          {
            "distribution_period": 66.5,
            "owner_age": 23,
            "spouse_age": 29
          },
          {
            "distribution_period": 70.2,
            "owner_age": 24,
            "spouse_age": 20
          },
          {
            "distribution_period": 69.6,
            "owner_age": 24,
            "spouse_age": 21
          },
          {
            "distribution_period": 69.0,
            "owner_age": 24,
            "spouse_age": 22
          },
          {
            "distribution_period": 68.5,
            "owner_age": 24,
            "spouse_age": 23
          },
          {
            "distribution_period": 68.0,
            "owner_age": 24,
            "spouse_age": 24
          },
          {
            "distribution_period": 67.5,
            "owner_age": 24,
            "spouse_age": 25
          },
          {
            "distribution_period": 67.1,
            "owner_age": 24,
            "spouse_age": 26
          },
          {
            "distribution_period": 66.6,
            "owner_age": 24,
            "spouse_age": 27
          },
          {
            "distribution_period": 66.2,
            "owner_age": 24,
            "spouse_age": 28
          },
          {
            "distribution_period": 65.8,
            "owner_age": 24,
            "spouse_age": 29
          },
          {
            "distribution_period": 69.8,
            "owner_age": 25,
            "spouse_age": 20
          },
          {
            "distribution_period": 69.2,
            "owner_age": 25,
            "spouse_age": 21
          },
          {
            "distribution_period": 68.6,
            "owner_age": 25,
            "spouse_age": 22
          },
          {
            "distribution_period": 68.0,
            "owner_age": 25,
            "spouse_age": 23
          },
          {
            "distribution_period": 67.5,
            "owner_age": 25,
            "spouse_age": 24
          },
          {
            "distribution_period": 67.0,
            "owner_age": 25,
            "spouse_age": 25
          },
          {
            "distribution_period": 66.5,
            "owner_age": 25,
            "spouse_age": 26
          },
          {
            "distribution_period": 66.1,
            "owner_age": 25,
            "spouse_age": 27
          },
          {
            "distribution_period": 65.6,
            "owner_age": 25,
            "spouse_age": 28
          },
          {
            "distribution_period": 65.2,
            "owner_age": 25,
            "spouse_age": 29
          },
          {
            "distribution_period": 69.5,
            "owner_age": 26,
            "spouse_age": 20
          },
          {
            "distribution_period": 68.8,
            "owner_age": 26,
            "spouse_age": 21
          },
          {
            "distribution_period": 68.2,
            "owner_age": 26,
            "spouse_age": 22
          },
          {
            "distribution_period": 67.6,
            "owner_age": 26,
            "spouse_age": 23
          },
          {
            "distribution_period": 67.1,
            "owner_age": 26,
            "spouse_age": 24
          },
          {
            "distribution_period": 66.5,
            "owner_age": 26,
            "spouse_age": 25
          },
          {
            "distribution_period": 66.0,
            "owner_age": 26,
            "spouse_age": 26
          },
          {
            "distribution_period": 65.5,
            "owner_age": 26,
            "spouse_age": 27
          },
          {
            "distribution_period": 65.1,
            "owner_age": 26,
            "spouse_age": 28
          },
          {
            "distribution_period": 64.6,
            "owner_age": 26,
            "spouse_age": 29
          },
          {
            "distribution_period": 69.1,
            "owner_age": 27,
            "spouse_age": 20
          },
          {
            "distribution_period": 68.5,
            "owner_age": 27,
            "spouse_age": 21
          },
          {
            "distribution_period": 67.8,
            "owner_age": 27,
            "spouse_age": 22
          },
          {
            "distribution_period": 67.2,
            "owner_age": 27,
            "spouse_age": 23
          },
          {
            "distribution_period": 66.6,
            "owner_age": 27,
            "spouse_age": 24
          },
          {
            "distribution_period": 66.1,
            "owner_age": 27,
            "spouse_age": 25
          },
          {
            "distribution_period": 65.5,
            "owner_age": 27,
            "spouse_age": 26
          },
          {
            "distribution_period": 65.0,
            "owner_age": 27,
            "spouse_age": 27
          },
          {
            "distribution_period": 64.5,
            "owner_age": 27,
            "spouse_age": 28
          },
          {
            "distribution_period": 64.1,
            "owner_age": 27,
            "spouse_age": 29
          },
          {
            "distribution_period": 68.8,
            "owner_age": 28,
            "spouse_age": 20
          },
          {
            "distribution_period": 68.1,
            "owner_age": 28,
            "spouse_age": 21
          },
          {
            "distribution_period": 67.5,
            "owner_age": 28,
            "spouse_age": 22
          },
          {
            "distribution_period": 66.8,
            "owner_age": 28,
            "spouse_age": 23
          },
          {
            "distribution_period": 66.2,
            "owner_age": 28,
            "spouse_age": 24
          },
          {
            "distribution_period": 65.6,
            "owner_age": 28,
            "spouse_age": 25
          },
          {
            "distribution_period": 65.1,
            "owner_age": 28,
            "spouse_age": 26
          },
          {
            "distribution_period": 64.5,
            "owner_age": 28,
            "spouse_age": 27
          },
          {
            "distribution_period": 64.0,
            "owner_age": 28,
            "spouse_age": 28
          },
          {
            "distribution_period": 63.5,
            "owner_age": 28,
            "spouse_age": 29
          },
          {
            "distribution_period": 68.5,
            "owner_age": 29,
            "spouse_age": 20
          },
          {
            "distribution_period": 67.8,
            "owner_age": 29,
            "spouse_age": 21
          },
          {
            "distribution_period": 67.1,
            "owner_age": 29,
            "spouse_age": 22
          },
          {
            "distribution_period": 66.5,
            "owner_age": 29,
            "spouse_age": 23
          },
          {
            "distribution_period": 65.8,
            "owner_age": 29,
            "spouse_age": 24
          },
          {
            "distribution_period": 65.2,
            "owner_age": 29,
            "spouse_age": 25
          },
          {
            "distribution_period": 64.6,
            "owner_age": 29,
            "spouse_age": 26
          },
          {
            "distribution_period": 64.1,
            "owner_age": 29,
            "spouse_age": 27
          },
          {
            "distribution_period": 63.5,
            "owner_age": 29,
            "spouse_age": 28
          },
          {
            "distribution_period": 63.0,
            "owner_age": 29,
            "spouse_age": 29
          },
          {
            "distribution_period": 68.3,
            "owner_age": 30,
            "spouse_age": 20
          },
          {
            "distribution_period": 67.5,
            "owner_age": 30,
            "spouse_age": 21
          },
          {
            "distribution_period": 66.8,
            "owner_age": 30,
            "spouse_age": 22
          },
          {
            "distribution_period": 66.2,
            "owner_age": 30,
            "spouse_age": 23
          },
          {
            "distribution_period": 65.5,
            "owner_age": 30,
            "spouse_age": 24
          },
          {
            "distribution_period": 64.9,
            "owner_age": 30,
            "spouse_age": 25
          },
          {
            "distribution_period": 64.2,
            "owner_age": 30,
            "spouse_age": 26
          },
          {
            "distribution_period": 63.7,
            "owner_age": 30,
            "spouse_age": 27
          },
          {
            "distribution_period": 63.1,
            "owner_age": 30,
            "spouse_age": 28
          },
          {
            "distribution_period": 62.6,
            "owner_age": 30,
            "spouse_age": 29
          },
          {
            "distribution_period": 62.0,
            "owner_age": 30,
            "spouse_age": 30
          },
          {
            "distribution_period": 61.6,
            "owner_age": 30,
            "spouse_age": 31
          },
          {
            "distribution_period": 61.1,
            "owner_age": 30,
            "spouse_age": 32
          },
          {
            "distribution_period": 60.7,
            "owner_age": 30,
            "spouse_age": 33
          },
          {
            "distribution_period": 60.3,
            "owner_age": 30,
            "spouse_age": 34
          },
          {
            "distribution_period": 59.9,
            "owner_age": 30,
            "spouse_age": 35
          },
          {
            "distribution_period": 59.5,
            "owner_age": 30,
            "spouse_age": 36
          },
          {
            "distribution_period": 59.2,
            "owner_age": 30,
            "spouse_age": 37
          },
          {
            "distribution_period": 58.9,
            "owner_age": 30,
            "spouse_age": 38
          },
          {
            "distribution_period": 58.6,
            "owner_age": 30,
            "spouse_age": 39
          },
          {
            "distribution_period": 68.0,
            "owner_age": 31,
            "spouse_age": 20
          },
          {
            "distribution_period": 67.3,
            "owner_age": 31,
            "spouse_age": 21
          },
          {
            "distribution_period": 66.6,
            "owner_age": 31,
            "spouse_age": 22
          },
          {
            "distribution_period": 65.8,
            "owner_age": 31,
            "spouse_age": 23
          },
          {
            "distribution_period": 65.2,
            "owner_age": 31,
            "spouse_age": 24
          },
          {
            "distribution_period": 64.5,
            "owner_age": 31,
            "spouse_age": 25
          },
          {
            "distribution_period": 63.9,
            "owner_age": 31,
            "spouse_age": 26
          },
          {
            "distribution_period": 63.2,
            "owner_age": 31,
            "spouse_age": 27
          },
          {
            "distribution_period": 62.7,
            "owner_age": 31,
            "spouse_age": 28
          },
          {
            "distribution_period": 62.1,
            "owner_age": 31,
            "spouse_age": 29
          },
          {
            "distribution_period": 61.6,
            "owner_age": 31,
            "spouse_age": 30
          },
          {
            "distribution_period": 61.1,
            "owner_age": 31,
            "spouse_age": 31
          },
          {
            "distribution_period": 60.6,
            "owner_age": 31,
            "spouse_age": 32
          },
          {
            "distribution_period": 60.1,
            "owner_age": 31,
            "spouse_age": 33
          },
          {
            "distribution_period": 59.7,
            "owner_age": 31,
            "spouse_age": 34
          },
          {
            "distribution_period": 59.3,
            "owner_age": 31,
            "spouse_age": 35
          },
          {
            "distribution_period": 58.9,
            "owner_age": 31,
            "spouse_age": 36
          },
          {
            "distribution_period": 58.6,
            "owner_age": 31,
            "spouse_age": 37
          },
          {
            "distribution_period": 58.2,
            "owner_age": 31,
            "spouse_age": 38
          },
          {
            "distribution_period": 57.9,
            "owner_age": 31,
            "spouse_age": 39
          },
          {
            "distribution_period": 67.8,
            "owner_age": 32,
            "spouse_age": 20
          },
          {
            "distribution_period": 67.0,
            "owner_age": 32,
            "spouse_age": 21
          },
          {
            "distribution_period": 66.3,
            "owner_age": 32,
            "spouse_age": 22
          },
          {
            "distribution_period": 65.6,
            "owner_age": 32,
            "spouse_age": 23
          },
          {
            "distribution_period": 64.9,
            "owner_age": 32,
            "spouse_age": 24
          },
          {
            "distribution_period": 64.2,
            "owner_age": 32,
            "spouse_age": 25
          },
          {
            "distribution_period": 63.5,
            "owner_age": 32,
            "spouse_age": 26
          },
          {
            "distribution_period": 62.9,
            "owner_age": 32,
            "spouse_age": 27
          },
          {
            "distribution_period": 62.3,
            "owner_age": 32,
            "spouse_age": 28
          },
          {
            "distribution_period": 61.7,
            "owner_age": 32,
            "spouse_age": 29
          },
          {
            "distribution_period": 61.1,
            "owner_age": 32,
            "spouse_age": 30
          },
          {
            "distribution_period": 60.6,
            "owner_age": 32,
            "spouse_age": 31
          },
          {
            "distribution_period": 60.1,
            "owner_age": 32,
            "spouse_age": 32
          },
          {
            "distribution_period": 59.6,
            "owner_age": 32,
            "spouse_age": 33
          },
          {
            "distribution_period": 59.1,
            "owner_age": 32,
            "spouse_age": 34
          },
          {
            "distribution_period": 58.7,
            "owner_age": 32,
            "spouse_age": 35
          },
          {
            "distribution_period": 58.3,
            "owner_age": 32,
            "spouse_age": 36
          },
          {
            "distribution_period": 57.9,
            "owner_age": 32,
            "spouse_age": 37
          },
          {
            "distribution_period": 57.6,
            "owner_age": 32,
            "spouse_age": 38
          },
          {
            "distribution_period": 57.2,
            "owner_age": 32,
            "spouse_age": 39
          },
          {
            "distribution_period": 67.6,
            "owner_age": 33,
            "spouse_age": 20
          },
          {
            "distribution_period": 66.8,
            "owner_age": 33,
            "spouse_age": 21
          },
          {
            "distribution_period": 66.0,
            "owner_age": 33,
            "spouse_age": 22
          },
          {
            "distribution_period": 65.3,
            "owner_age": 33,
            "spouse_age": 23
          },
          {
            "distribution_period": 64.6,
            "owner_age": 33,
            "spouse_age": 24
          },
          {
            "distribution_period": 63.9,
            "owner_age": 33,
            "spouse_age": 25
          },
          {
            "distribution_period": 63.2,
            "owner_age": 33,
            "spouse_age": 26
          },
          {
            "distribution_period": 62.5,
            "owner_age": 33,
            "spouse_age": 27
          },
          {
            "distribution_period": 61.9,
            "owner_age": 33,
            "spouse_age": 28
          },
          {
            "distribution_period": 61.3,
            "owner_age": 33,
            "spouse_age": 29
          },
          {
            "distribution_period": 60.7,
            "owner_age": 33,
            "spouse_age": 30
          },
          {
            "distribution_period": 60.1,
            "owner_age": 33,
            "spouse_age": 31
          },
          {
            "distribution_period": 59.6,
            "owner_age": 33,
            "spouse_age": 32
          },
          {
            "distribution_period": 59.1,
            "owner_age": 33,
            "spouse_age": 33
          },
          {
            "distribution_period": 58.6,
            "owner_age": 33,
            "spouse_age": 34
          },
          {
            "distribution_period": 58.1,
            "owner_age": 33,
            "spouse_age": 35
          },
          {
            "distribution_period": 57.7,
            "owner_age": 33,
            "spouse_age": 36
          },
          {
            "distribution_period": 57.3,
            "owner_age": 33,
            "spouse_age": 37
          },
          {
            "distribution_period": 56.9,
            "owner_age": 33,
            "spouse_age": 38
          },
          {
            "distribution_period": 56.6,
            "owner_age": 33,
            "spouse_age": 39
          },
          {
            "distribution_period": 67.4,
            "owner_age": 34,
            "spouse_age": 20
          },
          {
            "distribution_period": 66.6,
            "owner_age": 34,
            "spouse_age": 21
          },
          {
            "distribution_period": 65.8,
            "owner_age": 34,
            "spouse_age": 22
          },
          {
            "distribution_period": 65.1,
            "owner_age": 34,
            "spouse_age": 23
          },
          {
            "distribution_period": 64.3,
            "owner_age": 34,
            "spouse_age": 24
          },
          {
            "distribution_period": 63.6,
            "owner_age": 34,
            "spouse_age": 25
          },
          {
            "distribution_period": 62.9,
            "owner_age": 34,
            "spouse_age": 26
          },
          {
            "distribution_period": 62.2,
            "owner_age": 34,
            "spouse_age": 27
          },
          {
            "distribution_period": 61.5,
            "owner_age": 34,
            "spouse_age": 28
          },
          {
            "distribution_period": 60.9,
            "owner_age": 34,
            "spouse_age": 29
          },
          {
            "distribution_period": 60.3,
            "owner_age": 34,
            "spouse_age": 30
          },
          {
            "distribution_period": 59.7,
            "owner_age": 34,
            "spouse_age": 31
          },
          {
            "distribution_period": 59.1,
            "owner_age": 34,
            "spouse_age": 32
          },
          {
            "distribution_period": 58.6,
            "owner_age": 34,
            "spouse_age": 33
          },
          {
            "distribution_period": 58.1,
            "owner_age": 34,
            "spouse_age": 34
          },
          {
            "distribution_period": 57.6,
            "owner_age": 34,
            "spouse_age": 35
          },
          {
            "distribution_period": 57.2,
            "owner_age": 34,
            "spouse_age": 36
          },
          {
            "distribution_period": 56.7,
            "owner_age": 34,
            "spouse_age": 37
          },
          {
            "distribution_period": 56.3,
            "owner_age": 34,
            "spouse_age": 38
          },
          {
            "distribution_period": 55.9,
            "owner_age": 34,
            "spouse_age": 39
          },
          {
            "distribution_period": 67.2,
            "owner_age": 35,
            "spouse_age": 20
          },
          {
            "distribution_period": 66.4,
            "owner_age": 35,
            "spouse_age": 21
          },
          {
            "distribution_period": 65.6,
            "owner_age": 35,
            "spouse_age": 22
          },
          {
            "distribution_period": 64.8,
            "owner_age": 35,
            "spouse_age": 23
          },
          {
            "distribution_period": 64.1,
            "owner_age": 35,
            "spouse_age": 24
          },
          {
            "distribution_period": 63.3,
            "owner_age": 35,
            "spouse_age": 25
          },
          {
            "distribution_period": 62.6,
            "owner_age": 35,
            "spouse_age": 26
          },
          {
            "distribution_period": 61.9,
            "owner_age": 35,
            "spouse_age": 27
          },
          {
            "distribution_period": 61.2,
            "owner_age": 35,
            "spouse_age": 28
          },
          {
            "distribution_period": 60.5,
            "owner_age": 35,
            "spouse_age": 29
          },
          {
            "distribution_period": 59.9,
            "owner_age": 35,
            "spouse_age": 30
          },
          {
            "distribution_period": 59.3,
            "owner_age": 35,
            "spouse_age": 31
          },
          {
            "distribution_period": 58.7,
            "owner_age": 35,
            "spouse_age": 32
          },
          {
            "distribution_period": 58.1,
            "owner_age": 35,
            "spouse_age": 33
          },
          {
            "distribution_period": 57.6,
            "owner_age": 35,
            "spouse_age": 34
          },
          {
            "distribution_period": 57.1,
            "owner_age": 35,
            "spouse_age": 35
          },
          {
            "distribution_period": 56.6,
            "owner_age": 35,
            "spouse_age": 36
          },
          {
            "distribution_period": 56.2,
            "owner_age": 35,
            "spouse_age": 37
          },
          {
            "distribution_period": 55.7,
            "owner_age": 35,
            "spouse_age": 38
          },
          {
            "distribution_period": 55.3,
            "owner_age": 35,
            "spouse_age": 39
          },
          {
            "distribution_period": 67.1,
            "owner_age": 36,
            "spouse_age": 20
          },
          {
            "distribution_period": 66.2,
            "owner_age": 36,
            "spouse_age": 21
          },
          {
            "distribution_period": 65.4,
            "owner_age": 36,
            "spouse_age": 22
          },
          {
            "distribution_period": 64.6,
            "owner_age": 36,
            "spouse_age": 23
          },
          {
            "distribution_period": 63.8,
            "owner_age": 36,
            "spouse_age": 24
          },
          {
            "distribution_period": 63.1,
            "owner_age": 36,
            "spouse_age": 25
          },
          {
            "distribution_period": 62.3,
            "owner_age": 36,
            "spouse_age": 26
          },
          {
            "distribution_period": 61.6,
            "owner_age": 36,
            "spouse_age": 27
          },
          {
            "distribution_period": 60.9,
            "owner_age": 36,
            "spouse_age": 28
          },
          {
            "distribution_period": 60.2,
            "owner_age": 36,
            "spouse_age": 29
          },
          {
            "distribution_period": 59.5,
            "owner_age": 36,
            "spouse_age": 30
          },
          {
            "distribution_period": 58.9,
            "owner_age": 36,
            "spouse_age": 31
          },
          {
            "distribution_period": 58.3,
            "owner_age": 36,
            "spouse_age": 32
          },
          {
            "distribution_period": 57.7,
            "owner_age": 36,
            "spouse_age": 33
          },
          {
            "distribution_period": 57.2,
            "owner_age": 36,
            "spouse_age": 34
          },
          {
            "distribution_period": 56.6,
            "owner_age": 36,
            "spouse_age": 35
          },
          {
            "distribution_period": 56.1,
            "owner_age": 36,
            "spouse_age": 36
          },
          {
            "distribution_period": 55.6,
            "owner_age": 36,
            "spouse_age": 37
          },
          {
            "distribution_period": 55.2,
            "owner_age": 36,
            "spouse_age": 38
          },
          {
            "distribution_period": 54.7,
            "owner_age": 36,
            "spouse_age": 39
          },
          {
            "distribution_period": 66.9,
            "owner_age": 37,
            "spouse_age": 20
          },
          {
            "distribution_period": 66.1,
            "owner_age": 37,
            "spouse_age": 21
          },
          {
            "distribution_period": 65.2,
            "owner_age": 37,
            "spouse_age": 22
          },
          {
            "distribution_period": 64.4,
            "owner_age": 37,
            "spouse_age": 23
          },
          {
            "distribution_period": 63.6,
            "owner_age": 37,
            "spouse_age": 24
          },
          {
            "distribution_period": 62.8,
            "owner_age": 37,
            "spouse_age": 25
          },
          {
            "distribution_period": 62.1,
            "owner_age": 37,
            "spouse_age": 26
          },
          {
            "distribution_period": 61.3,
            "owner_age": 37,
            "spouse_age": 27
          },
          {
            "distribution_period": 60.6,
            "owner_age": 37,
            "spouse_age": 28
          },
          {
            "distribution_period": 59.9,
            "owner_age": 37,
            "spouse_age": 29
          },
          {
            "distribution_period": 59.2,
            "owner_age": 37,
            "spouse_age": 30
          },
          {
            "distribution_period": 58.6,
            "owner_age": 37,
            "spouse_age": 31
          },
          {
            "distribution_period": 57.9,
            "owner_age": 37,
            "spouse_age": 32
          },
          {
            "distribution_period": 57.3,
            "owner_age": 37,
            "spouse_age": 33
          },
          {
            "distribution_period": 56.7,
            "owner_age": 37,
            "spouse_age": 34
          },
          {
            "distribution_period": 56.2,
            "owner_age": 37,
            "spouse_age": 35
          },
          {
            "distribution_period": 55.6,
            "owner_age": 37,
            "spouse_age": 36
          },
          {
            "distribution_period": 55.1,
            "owner_age": 37,
            "spouse_age": 37
          },
          {
            "distribution_period": 54.6,
            "owner_age": 37,
            "spouse_age": 38
          },
          {
            "distribution_period": 54.2,
            "owner_age": 37,
            "spouse_age": 39
          },
          {
            "distribution_period": 66.8,
            "owner_age": 38,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.9,
            "owner_age": 38,
            "spouse_age": 21
          },
          {
            "distribution_period": 65.1,
            "owner_age": 38,
            "spouse_age": 22
          },
          {
            "distribution_period": 64.2,
            "owner_age": 38,
            "spouse_age": 23
          },
          {
            "distribution_period": 63.4,
            "owner_age": 38,
            "spouse_age": 24
          },
          {
            "distribution_period": 62.6,
            "owner_age": 38,
            "spouse_age": 25
          },
          {
            "distribution_period": 61.9,
            "owner_age": 38,
            "spouse_age": 26
          },
          {
            "distribution_period": 61.1,
            "owner_age": 38,
            "spouse_age": 27
          },
          {
            "distribution_period": 60.3,
            "owner_age": 38,
            "spouse_age": 28
          },
          {
            "distribution_period": 59.6,
            "owner_age": 38,
            "spouse_age": 29
          },
          {
            "distribution_period": 58.9,
            "owner_age": 38,
            "spouse_age": 30
          },
          {
            "distribution_period": 58.2,
            "owner_age": 38,
            "spouse_age": 31
          },
          {
            "distribution_period": 57.6,
            "owner_age": 38,
            "spouse_age": 32
          },
          {
            "distribution_period": 56.9,
            "owner_age": 38,
            "spouse_age": 33
          },
          {
            "distribution_period": 56.3,
            "owner_age": 38,
            "spouse_age": 34
          },
          {
            "distribution_period": 55.7,
            "owner_age": 38,
            "spouse_age": 35
          },
          {
            "distribution_period": 55.2,
            "owner_age": 38,
            "spouse_age": 36
          },
          {
            "distribution_period": 54.6,
            "owner_age": 38,
            "spouse_age": 37
          },
          {
            "distribution_period": 54.1,
            "owner_age": 38,
            "spouse_age": 38
          },
          {
            "distribution_period": 53.6,
            "owner_age": 38,
            "spouse_age": 39
          },
          {
            "distribution_period": 66.6,
            "owner_age": 39,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.8,
            "owner_age": 39,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.9,
            "owner_age": 39,
            "spouse_age": 22
          },
          {
            "distribution_period": 64.1,
            "owner_age": 39,
            "spouse_age": 23
          },
          {
            "distribution_period": 63.3,
            "owner_age": 39,
            "spouse_age": 24
          },
          {
            "distribution_period": 62.4,
            "owner_age": 39,
            "spouse_age": 25
          },
          {
            "distribution_period": 61.6,
            "owner_age": 39,
            "spouse_age": 26
          },
          {
            "distribution_period": 60.9,
            "owner_age": 39,
            "spouse_age": 27
          },
          {
            "distribution_period": 60.1,
            "owner_age": 39,
            "spouse_age": 28
          },
          {
            "distribution_period": 59.4,
            "owner_age": 39,
            "spouse_age": 29
          },
          {
            "distribution_period": 58.6,
            "owner_age": 39,
            "spouse_age": 30
          },
          {
            "distribution_period": 57.9,
            "owner_age": 39,
            "spouse_age": 31
          },
          {
            "distribution_period": 57.2,
            "owner_age": 39,
            "spouse_age": 32
          },
          {
            "distribution_period": 56.6,
            "owner_age": 39,
            "spouse_age": 33
          },
          {
            "distribution_period": 55.9,
            "owner_age": 39,
            "spouse_age": 34
          },
          {
            "distribution_period": 55.3,
            "owner_age": 39,
            "spouse_age": 35
          },
          {
            "distribution_period": 54.7,
            "owner_age": 39,
            "spouse_age": 36
          },
          {
            "distribution_period": 54.2,
            "owner_age": 39,
            "spouse_age": 37
          },
          {
            "distribution_period": 53.6,
            "owner_age": 39,
            "spouse_age": 38
          },
          {
            "distribution_period": 53.1,
            "owner_age": 39,
            "spouse_age": 39
          },
          {
            "distribution_period": 66.5,
            "owner_age": 40,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.6,
            "owner_age": 40,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.8,
            "owner_age": 40,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.9,
            "owner_age": 40,
            "spouse_age": 23
          },
          {
            "distribution_period": 63.1,
            "owner_age": 40,
            "spouse_age": 24
          },
          {
            "distribution_period": 62.3,
            "owner_age": 40,
            "spouse_age": 25
          },
          {
            "distribution_period": 61.5,
            "owner_age": 40,
            "spouse_age": 26
          },
          {
            "distribution_period": 60.7,
            "owner_age": 40,
            "spouse_age": 27
          },
          {
            "distribution_period": 59.9,
            "owner_age": 40,
            "spouse_age": 28
          },
          {
            "distribution_period": 59.1,
            "owner_age": 40,
            "spouse_age": 29
          },
          {
            "distribution_period": 58.4,
            "owner_age": 40,
            "spouse_age": 30
          },
          {
            "distribution_period": 57.6,
            "owner_age": 40,
            "spouse_age": 31
          },
          {
            "distribution_period": 56.9,
            "owner_age": 40,
            "spouse_age": 32
          },
          {
            "distribution_period": 56.3,
            "owner_age": 40,
            "spouse_age": 33
          },
          {
            "distribution_period": 55.6,
            "owner_age": 40,
            "spouse_age": 34
          },
          {
            "distribution_period": 55.0,
            "owner_age": 40,
            "spouse_age": 35
          },
          {
            "distribution_period": 54.3,
            "owner_age": 40,
            "spouse_age": 36
          },
          {
            "distribution_period": 53.8,
            "owner_age": 40,
            "spouse_age": 37
          },
          {
            "distribution_period": 53.2,
            "owner_age": 40,
            "spouse_age": 38
          },
          {
            "distribution_period": 52.7,
            "owner_age": 40,
            "spouse_age": 39
          },
          {
            "distribution_period": 52.2,
            "owner_age": 40,
            "spouse_age": 40
          },
          {
            "distribution_period": 51.7,
            "owner_age": 40,
            "spouse_age": 41
          },
          {
            "distribution_period": 51.2,
            "owner_age": 40,
            "spouse_age": 42
          },
          {
            "distribution_period": 50.8,
            "owner_age": 40,
            "spouse_age": 43
          },
          {
            "distribution_period": 50.4,
            "owner_age": 40,
            "spouse_age": 44
          },
          {
            "distribution_period": 50.0,
            "owner_age": 40,
            "spouse_age": 45
          },
          {
            "distribution_period": 49.7,
            "owner_age": 40,
            "spouse_age": 46
          },
          {
            "distribution_period": 49.3,
            "owner_age": 40,
            "spouse_age": 47
          },
          {
            "distribution_period": 49.0,
            "owner_age": 40,
            "spouse_age": 48
          },
          {
            "distribution_period": 48.8,
            "owner_age": 40,
            "spouse_age": 49
          },
          {
            "distribution_period": 66.4,
            "owner_age": 41,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.5,
            "owner_age": 41,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.6,
            "owner_age": 41,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.8,
            "owner_age": 41,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.9,
            "owner_age": 41,
            "spouse_age": 24
          },
          {
            "distribution_period": 62.1,
            "owner_age": 41,
            "spouse_age": 25
          },
          {
            "distribution_period": 61.3,
            "owner_age": 41,
            "spouse_age": 26
          },
          {
            "distribution_period": 60.5,
            "owner_age": 41,
            "spouse_age": 27
          },
          {
            "distribution_period": 59.7,
            "owner_age": 41,
            "spouse_age": 28
          },
          {
            "distribution_period": 58.9,
            "owner_age": 41,
            "spouse_age": 29
          },
          {
            "distribution_period": 58.1,
            "owner_age": 41,
            "spouse_age": 30
          },
          {
            "distribution_period": 57.4,
            "owner_age": 41,
            "spouse_age": 31
          },
          {
            "distribution_period": 56.7,
            "owner_age": 41,
            "spouse_age": 32
          },
          {
            "distribution_period": 56.0,
            "owner_age": 41,
            "spouse_age": 33
          },
          {
            "distribution_period": 55.3,
            "owner_age": 41,
            "spouse_age": 34
          },
          {
            "distribution_period": 54.6,
            "owner_age": 41,
            "spouse_age": 35
          },
          {
            "distribution_period": 54.0,
            "owner_age": 41,
            "spouse_age": 36
          },
          {
            "distribution_period": 53.4,
            "owner_age": 41,
            "spouse_age": 37
          },
          {
            "distribution_period": 52.8,
            "owner_age": 41,
            "spouse_age": 38
          },
          {
            "distribution_period": 52.2,
            "owner_age": 41,
            "spouse_age": 39
          },
          {
            "distribution_period": 51.7,
            "owner_age": 41,
            "spouse_age": 40
          },
          {
            "distribution_period": 51.2,
            "owner_age": 41,
            "spouse_age": 41
          },
          {
            "distribution_period": 50.7,
            "owner_age": 41,
            "spouse_age": 42
          },
          {
            "distribution_period": 50.2,
            "owner_age": 41,
            "spouse_age": 43
          },
          {
            "distribution_period": 49.8,
            "owner_age": 41,
            "spouse_age": 44
          },
          {
            "distribution_period": 49.4,
            "owner_age": 41,
            "spouse_age": 45
          },
          {
            "distribution_period": 49.0,
            "owner_age": 41,
            "spouse_age": 46
          },
          {
            "distribution_period": 48.7,
            "owner_age": 41,
            "spouse_age": 47
          },
          {
            "distribution_period": 48.4,
            "owner_age": 41,
            "spouse_age": 48
          },
          {
            "distribution_period": 48.1,
            "owner_age": 41,
            "spouse_age": 49
          },
          {
            "distribution_period": 66.3,
            "owner_age": 42,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.4,
            "owner_age": 42,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.5,
            "owner_age": 42,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.6,
            "owner_age": 42,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.8,
            "owner_age": 42,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.9,
            "owner_age": 42,
            "spouse_age": 25
          },
          {
            "distribution_period": 61.1,
            "owner_age": 42,
            "spouse_age": 26
          },
          {
            "distribution_period": 60.3,
            "owner_age": 42,
            "spouse_age": 27
          },
          {
            "distribution_period": 59.5,
            "owner_age": 42,
            "spouse_age": 28
          },
          {
            "distribution_period": 58.7,
            "owner_age": 42,
            "spouse_age": 29
          },
          {
            "distribution_period": 57.9,
            "owner_age": 42,
            "spouse_age": 30
          },
          {
            "distribution_period": 57.1,
            "owner_age": 42,
            "spouse_age": 31
          },
          {
            "distribution_period": 56.4,
            "owner_age": 42,
            "spouse_age": 32
          },
          {
            "distribution_period": 55.7,
            "owner_age": 42,
            "spouse_age": 33
          },
          {
            "distribution_period": 55.0,
            "owner_age": 42,
            "spouse_age": 34
          },
          {
            "distribution_period": 54.3,
            "owner_age": 42,
            "spouse_age": 35
          },
          {
            "distribution_period": 53.6,
            "owner_age": 42,
            "spouse_age": 36
          },
          {
            "distribution_period": 53.0,
            "owner_age": 42,
            "spouse_age": 37
          },
          {
            "distribution_period": 52.4,
            "owner_age": 42,
            "spouse_age": 38
          },
          {
            "distribution_period": 51.8,
            "owner_age": 42,
            "spouse_age": 39
          },
          {
            "distribution_period": 51.2,
            "owner_age": 42,
            "spouse_age": 40
          },
          {
            "distribution_period": 50.7,
            "owner_age": 42,
            "spouse_age": 41
          },
          {
            "distribution_period": 50.2,
            "owner_age": 42,
            "spouse_age": 42
          },
          {
            "distribution_period": 49.7,
            "owner_age": 42,
            "spouse_age": 43
          },
          {
            "distribution_period": 49.2,
            "owner_age": 42,
            "spouse_age": 44
          },
          {
            "distribution_period": 48.8,
            "owner_age": 42,
            "spouse_age": 45
          },
          {
            "distribution_period": 48.4,
            "owner_age": 42,
            "spouse_age": 46
          },
          {
            "distribution_period": 48.0,
            "owner_age": 42,
            "spouse_age": 47
          },
          {
            "distribution_period": 47.7,
            "owner_age": 42,
            "spouse_age": 48
          },
          {
            "distribution_period": 47.4,
            "owner_age": 42,
            "spouse_age": 49
          },
          {
            "distribution_period": 66.2,
            "owner_age": 43,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.3,
            "owner_age": 43,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.4,
            "owner_age": 43,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.5,
            "owner_age": 43,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.7,
            "owner_age": 43,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.8,
            "owner_age": 43,
            "spouse_age": 25
          },
          {
            "distribution_period": 61.0,
            "owner_age": 43,
            "spouse_age": 26
          },
          {
            "distribution_period": 60.1,
            "owner_age": 43,
            "spouse_age": 27
          },
          {
            "distribution_period": 59.3,
            "owner_age": 43,
            "spouse_age": 28
          },
          {
            "distribution_period": 58.5,
            "owner_age": 43,
            "spouse_age": 29
          },
          {
            "distribution_period": 57.7,
            "owner_age": 43,
            "spouse_age": 30
          },
          {
            "distribution_period": 56.9,
            "owner_age": 43,
            "spouse_age": 31
          },
          {
            "distribution_period": 56.2,
            "owner_age": 43,
            "spouse_age": 32
          },
          {
            "distribution_period": 55.4,
            "owner_age": 43,
            "spouse_age": 33
          },
          {
            "distribution_period": 54.7,
            "owner_age": 43,
            "spouse_age": 34
          },
          {
            "distribution_period": 54.0,
            "owner_age": 43,
            "spouse_age": 35
          },
          {
            "distribution_period": 53.3,
            "owner_age": 43,
            "spouse_age": 36
          },
          {
            "distribution_period": 52.6,
            "owner_age": 43,
            "spouse_age": 37
          },
          {
            "distribution_period": 52.0,
            "owner_age": 43,
            "spouse_age": 38
          },
          {
            "distribution_period": 51.4,
            "owner_age": 43,
            "spouse_age": 39
          },
          {
            "distribution_period": 50.8,
            "owner_age": 43,
            "spouse_age": 40
          },
          {
            "distribution_period": 50.2,
            "owner_age": 43,
            "spouse_age": 41
          },
          {
            "distribution_period": 49.7,
            "owner_age": 43,
            "spouse_age": 42
          },
          {
            "distribution_period": 49.2,
            "owner_age": 43,
            "spouse_age": 43
          },
          {
            "distribution_period": 48.7,
            "owner_age": 43,
            "spouse_age": 44
          },
          {
            "distribution_period": 48.3,
            "owner_age": 43,
            "spouse_age": 45
          },
          {
            "distribution_period": 47.8,
            "owner_age": 43,
            "spouse_age": 46
          },
          {
            "distribution_period": 47.4,
            "owner_age": 43,
            "spouse_age": 47
          },
          {
            "distribution_period": 47.1,
            "owner_age": 43,
            "spouse_age": 48
          },
          {
            "distribution_period": 46.7,
            "owner_age": 43,
            "spouse_age": 49
          },
          {
            "distribution_period": 66.1,
            "owner_age": 44,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.2,
            "owner_age": 44,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.3,
            "owner_age": 44,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.4,
            "owner_age": 44,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.5,
            "owner_age": 44,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.7,
            "owner_age": 44,
            "spouse_age": 25
          },
          {
            "distribution_period": 60.8,
            "owner_age": 44,
            "spouse_age": 26
          },
          {
            "distribution_period": 60.0,
            "owner_age": 44,
            "spouse_age": 27
          },
          {
            "distribution_period": 59.1,
            "owner_age": 44,
            "spouse_age": 28
          },
          {
            "distribution_period": 58.3,
            "owner_age": 44,
            "spouse_age": 29
          },
          {
            "distribution_period": 57.5,
            "owner_age": 44,
            "spouse_age": 30
          },
          {
            "distribution_period": 56.7,
            "owner_age": 44,
            "spouse_age": 31
          },
          {
            "distribution_period": 55.9,
            "owner_age": 44,
            "spouse_age": 32
          },
          {
            "distribution_period": 55.2,
            "owner_age": 44,
            "spouse_age": 33
          },
          {
            "distribution_period": 54.4,
            "owner_age": 44,
            "spouse_age": 34
          },
          {
            "distribution_period": 53.7,
            "owner_age": 44,
            "spouse_age": 35
          },
          {
            "distribution_period": 53.0,
            "owner_age": 44,
            "spouse_age": 36
          },
          {
            "distribution_period": 52.3,
            "owner_age": 44,
            "spouse_age": 37
          },
          {
            "distribution_period": 51.6,
            "owner_age": 44,
            "spouse_age": 38
          },
          {
            "distribution_period": 51.0,
            "owner_age": 44,
            "spouse_age": 39
          },
          {
            "distribution_period": 50.4,
            "owner_age": 44,
            "spouse_age": 40
          },
          {
            "distribution_period": 49.8,
            "owner_age": 44,
            "spouse_age": 41
          },
          {
            "distribution_period": 49.2,
            "owner_age": 44,
            "spouse_age": 42
          },
          {
            "distribution_period": 48.7,
            "owner_age": 44,
            "spouse_age": 43
          },
          {
            "distribution_period": 48.2,
            "owner_age": 44,
            "spouse_age": 44
          },
          {
            "distribution_period": 47.7,
            "owner_age": 44,
            "spouse_age": 45
          },
          {
            "distribution_period": 47.3,
            "owner_age": 44,
            "spouse_age": 46
          },
          {
            "distribution_period": 46.8,
            "owner_age": 44,
            "spouse_age": 47
          },
          {
            "distribution_period": 46.4,
            "owner_age": 44,
            "spouse_age": 48
          },
          {
            "distribution_period": 46.1,
            "owner_age": 44,
            "spouse_age": 49
          },
          {
            "distribution_period": 66.0,
            "owner_age": 45,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.1,
            "owner_age": 45,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.2,
            "owner_age": 45,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.3,
            "owner_age": 45,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.4,
            "owner_age": 45,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.5,
            "owner_age": 45,
            "spouse_age": 25
          },
          {
            "distribution_period": 60.7,
            "owner_age": 45,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.8,
            "owner_age": 45,
            "spouse_age": 27
          },
          {
            "distribution_period": 59.0,
            "owner_age": 45,
            "spouse_age": 28
          },
          {
            "distribution_period": 58.1,
            "owner_age": 45,
            "spouse_age": 29
          },
          {
            "distribution_period": 57.3,
            "owner_age": 45,
            "spouse_age": 30
          },
          {
            "distribution_period": 56.5,
            "owner_age": 45,
            "spouse_age": 31
          },
          {
            "distribution_period": 55.7,
            "owner_age": 45,
            "spouse_age": 32
          },
          {
            "distribution_period": 54.9,
            "owner_age": 45,
            "spouse_age": 33
          },
          {
            "distribution_period": 54.2,
            "owner_age": 45,
            "spouse_age": 34
          },
          {
            "distribution_period": 53.4,
            "owner_age": 45,
            "spouse_age": 35
          },
          {
            "distribution_period": 52.7,
            "owner_age": 45,
            "spouse_age": 36
          },
          {
            "distribution_period": 52.0,
            "owner_age": 45,
            "spouse_age": 37
          },
          {
            "distribution_period": 51.3,
            "owner_age": 45,
            "spouse_age": 38
          },
          {
            "distribution_period": 50.7,
            "owner_age": 45,
            "spouse_age": 39
          },
          {
            "distribution_period": 50.0,
            "owner_age": 45,
            "spouse_age": 40
          },
          {
            "distribution_period": 49.4,
            "owner_age": 45,
            "spouse_age": 41
          },
          {
            "distribution_period": 48.8,
            "owner_age": 45,
            "spouse_age": 42
          },
          {
            "distribution_period": 48.3,
            "owner_age": 45,
            "spouse_age": 43
          },
          {
            "distribution_period": 47.7,
            "owner_age": 45,
            "spouse_age": 44
          },
          {
            "distribution_period": 47.2,
            "owner_age": 45,
            "spouse_age": 45
          },
          {
            "distribution_period": 46.7,
            "owner_age": 45,
            "spouse_age": 46
          },
          {
            "distribution_period": 46.3,
            "owner_age": 45,
            "spouse_age": 47
          },
          {
            "distribution_period": 45.9,
            "owner_age": 45,
            "spouse_age": 48
          },
          {
            "distribution_period": 45.5,
            "owner_age": 45,
            "spouse_age": 49
          },
          {
            "distribution_period": 65.9,
            "owner_age": 46,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.0,
            "owner_age": 46,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.1,
            "owner_age": 46,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.2,
            "owner_age": 46,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.3,
            "owner_age": 46,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.4,
            "owner_age": 46,
            "spouse_age": 25
          },
          {
            "distribution_period": 60.6,
            "owner_age": 46,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.7,
            "owner_age": 46,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.8,
            "owner_age": 46,
            "spouse_age": 28
          },
          {
            "distribution_period": 58.0,
            "owner_age": 46,
            "spouse_age": 29
          },
          {
            "distribution_period": 57.2,
            "owner_age": 46,
            "spouse_age": 30
          },
          {
            "distribution_period": 56.3,
            "owner_age": 46,
            "spouse_age": 31
          },
          {
            "distribution_period": 55.5,
            "owner_age": 46,
            "spouse_age": 32
          },
          {
            "distribution_period": 54.7,
            "owner_age": 46,
            "spouse_age": 33
          },
          {
            "distribution_period": 54.0,
            "owner_age": 46,
            "spouse_age": 34
          },
          {
            "distribution_period": 53.2,
            "owner_age": 46,
            "spouse_age": 35
          },
          {
            "distribution_period": 52.4,
            "owner_age": 46,
            "spouse_age": 36
          },
          {
            "distribution_period": 51.7,
            "owner_age": 46,
            "spouse_age": 37
          },
          {
            "distribution_period": 51.0,
            "owner_age": 46,
            "spouse_age": 38
          },
          {
            "distribution_period": 50.3,
            "owner_age": 46,
            "spouse_age": 39
          },
          {
            "distribution_period": 49.7,
            "owner_age": 46,
            "spouse_age": 40
          },
          {
            "distribution_period": 49.0,
            "owner_age": 46,
            "spouse_age": 41
          },
          {
            "distribution_period": 48.4,
            "owner_age": 46,
            "spouse_age": 42
          },
          {
            "distribution_period": 47.8,
            "owner_age": 46,
            "spouse_age": 43
          },
          {
            "distribution_period": 47.3,
            "owner_age": 46,
            "spouse_age": 44
          },
          {
            "distribution_period": 46.7,
            "owner_age": 46,
            "spouse_age": 45
          },
          {
            "distribution_period": 46.2,
            "owner_age": 46,
            "spouse_age": 46
          },
          {
            "distribution_period": 45.7,
            "owner_age": 46,
            "spouse_age": 47
          },
          {
            "distribution_period": 45.3,
            "owner_age": 46,
            "spouse_age": 48
          },
          {
            "distribution_period": 44.9,
            "owner_age": 46,
            "spouse_age": 49
          },
          {
            "distribution_period": 65.9,
            "owner_age": 47,
            "spouse_age": 20
          },
          {
            "distribution_period": 65.0,
            "owner_age": 47,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.0,
            "owner_age": 47,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.1,
            "owner_age": 47,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.2,
            "owner_age": 47,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.3,
            "owner_age": 47,
            "spouse_age": 25
          },
          {
            "distribution_period": 60.5,
            "owner_age": 47,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.6,
            "owner_age": 47,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.7,
            "owner_age": 47,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.9,
            "owner_age": 47,
            "spouse_age": 29
          },
          {
            "distribution_period": 57.0,
            "owner_age": 47,
            "spouse_age": 30
          },
          {
            "distribution_period": 56.2,
            "owner_age": 47,
            "spouse_age": 31
          },
          {
            "distribution_period": 55.4,
            "owner_age": 47,
            "spouse_age": 32
          },
          {
            "distribution_period": 54.5,
            "owner_age": 47,
            "spouse_age": 33
          },
          {
            "distribution_period": 53.7,
            "owner_age": 47,
            "spouse_age": 34
          },
          {
            "distribution_period": 53.0,
            "owner_age": 47,
            "spouse_age": 35
          },
          {
            "distribution_period": 52.2,
            "owner_age": 47,
            "spouse_age": 36
          },
          {
            "distribution_period": 51.5,
            "owner_age": 47,
            "spouse_age": 37
          },
          {
            "distribution_period": 50.7,
            "owner_age": 47,
            "spouse_age": 38
          },
          {
            "distribution_period": 50.0,
            "owner_age": 47,
            "spouse_age": 39
          },
          {
            "distribution_period": 49.3,
            "owner_age": 47,
            "spouse_age": 40
          },
          {
            "distribution_period": 48.7,
            "owner_age": 47,
            "spouse_age": 41
          },
          {
            "distribution_period": 48.0,
            "owner_age": 47,
            "spouse_age": 42
          },
          {
            "distribution_period": 47.4,
            "owner_age": 47,
            "spouse_age": 43
          },
          {
            "distribution_period": 46.8,
            "owner_age": 47,
            "spouse_age": 44
          },
          {
            "distribution_period": 46.3,
            "owner_age": 47,
            "spouse_age": 45
          },
          {
            "distribution_period": 45.7,
            "owner_age": 47,
            "spouse_age": 46
          },
          {
            "distribution_period": 45.2,
            "owner_age": 47,
            "spouse_age": 47
          },
          {
            "distribution_period": 44.8,
            "owner_age": 47,
            "spouse_age": 48
          },
          {
            "distribution_period": 44.3,
            "owner_age": 47,
            "spouse_age": 49
          },
          {
            "distribution_period": 65.8,
            "owner_age": 48,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.9,
            "owner_age": 48,
            "spouse_age": 21
          },
          {
            "distribution_period": 64.0,
            "owner_age": 48,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.0,
            "owner_age": 48,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.1,
            "owner_age": 48,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.2,
            "owner_age": 48,
            "spouse_age": 25
          },
          {
            "distribution_period": 60.3,
            "owner_age": 48,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.5,
            "owner_age": 48,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.6,
            "owner_age": 48,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.7,
            "owner_age": 48,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.9,
            "owner_age": 48,
            "spouse_age": 30
          },
          {
            "distribution_period": 56.0,
            "owner_age": 48,
            "spouse_age": 31
          },
          {
            "distribution_period": 55.2,
            "owner_age": 48,
            "spouse_age": 32
          },
          {
            "distribution_period": 54.4,
            "owner_age": 48,
            "spouse_age": 33
          },
          {
            "distribution_period": 53.6,
            "owner_age": 48,
            "spouse_age": 34
          },
          {
            "distribution_period": 52.8,
            "owner_age": 48,
            "spouse_age": 35
          },
          {
            "distribution_period": 52.0,
            "owner_age": 48,
            "spouse_age": 36
          },
          {
            "distribution_period": 51.2,
            "owner_age": 48,
            "spouse_age": 37
          },
          {
            "distribution_period": 50.5,
            "owner_age": 48,
            "spouse_age": 38
          },
          {
            "distribution_period": 49.7,
            "owner_age": 48,
            "spouse_age": 39
          },
          {
            "distribution_period": 49.0,
            "owner_age": 48,
            "spouse_age": 40
          },
          {
            "distribution_period": 48.4,
            "owner_age": 48,
            "spouse_age": 41
          },
          {
            "distribution_period": 47.7,
            "owner_age": 48,
            "spouse_age": 42
          },
          {
            "distribution_period": 47.1,
            "owner_age": 48,
            "spouse_age": 43
          },
          {
            "distribution_period": 46.4,
            "owner_age": 48,
            "spouse_age": 44
          },
          {
            "distribution_period": 45.9,
            "owner_age": 48,
            "spouse_age": 45
          },
          {
            "distribution_period": 45.3,
            "owner_age": 48,
            "spouse_age": 46
          },
          {
            "distribution_period": 44.8,
            "owner_age": 48,
            "spouse_age": 47
          },
          {
            "distribution_period": 44.3,
            "owner_age": 48,
            "spouse_age": 48
          },
          {
            "distribution_period": 43.8,
            "owner_age": 48,
            "spouse_age": 49
          },
          {
            "distribution_period": 65.7,
            "owner_age": 49,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.8,
            "owner_age": 49,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.9,
            "owner_age": 49,
            "spouse_age": 22
          },
          {
            "distribution_period": 63.0,
            "owner_age": 49,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.1,
            "owner_age": 49,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.2,
            "owner_age": 49,
            "spouse_age": 25
          },
          {
            "distribution_period": 60.3,
            "owner_age": 49,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.4,
            "owner_age": 49,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.5,
            "owner_age": 49,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.6,
            "owner_age": 49,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.7,
            "owner_age": 49,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.9,
            "owner_age": 49,
            "spouse_age": 31
          },
          {
            "distribution_period": 55.0,
            "owner_age": 49,
            "spouse_age": 32
          },
          {
            "distribution_period": 54.2,
            "owner_age": 49,
            "spouse_age": 33
          },
          {
            "distribution_period": 53.4,
            "owner_age": 49,
            "spouse_age": 34
          },
          {
            "distribution_period": 52.6,
            "owner_age": 49,
            "spouse_age": 35
          },
          {
            "distribution_period": 51.8,
            "owner_age": 49,
            "spouse_age": 36
          },
          {
            "distribution_period": 51.0,
            "owner_age": 49,
            "spouse_age": 37
          },
          {
            "distribution_period": 50.2,
            "owner_age": 49,
            "spouse_age": 38
          },
          {
            "distribution_period": 49.5,
            "owner_age": 49,
            "spouse_age": 39
          },
          {
            "distribution_period": 48.8,
            "owner_age": 49,
            "spouse_age": 40
          },
          {
            "distribution_period": 48.1,
            "owner_age": 49,
            "spouse_age": 41
          },
          {
            "distribution_period": 47.4,
            "owner_age": 49,
            "spouse_age": 42
          },
          {
            "distribution_period": 46.7,
            "owner_age": 49,
            "spouse_age": 43
          },
          {
            "distribution_period": 46.1,
            "owner_age": 49,
            "spouse_age": 44
          },
          {
            "distribution_period": 45.5,
            "owner_age": 49,
            "spouse_age": 45
          },
          {
            "distribution_period": 44.9,
            "owner_age": 49,
            "spouse_age": 46
          },
          {
            "distribution_period": 44.3,
            "owner_age": 49,
            "spouse_age": 47
          },
          {
            "distribution_period": 43.8,
            "owner_age": 49,
            "spouse_age": 48
          },
          {
            "distribution_period": 43.3,
            "owner_age": 49,
            "spouse_age": 49
          },
          {
            "distribution_period": 65.7,
            "owner_age": 50,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.8,
            "owner_age": 50,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.8,
            "owner_age": 50,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.9,
            "owner_age": 50,
            "spouse_age": 23
          },
          {
            "distribution_period": 62.0,
            "owner_age": 50,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.1,
            "owner_age": 50,
            "spouse_age": 25
          },
          {
            "distribution_period": 60.2,
            "owner_age": 50,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.3,
            "owner_age": 50,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.4,
            "owner_age": 50,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.5,
            "owner_age": 50,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.6,
            "owner_age": 50,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.8,
            "owner_age": 50,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.9,
            "owner_age": 50,
            "spouse_age": 32
          },
          {
            "distribution_period": 54.1,
            "owner_age": 50,
            "spouse_age": 33
          },
          {
            "distribution_period": 53.2,
            "owner_age": 50,
            "spouse_age": 34
          },
          {
            "distribution_period": 52.4,
            "owner_age": 50,
            "spouse_age": 35
          },
          {
            "distribution_period": 51.6,
            "owner_age": 50,
            "spouse_age": 36
          },
          {
            "distribution_period": 50.8,
            "owner_age": 50,
            "spouse_age": 37
          },
          {
            "distribution_period": 50.0,
            "owner_age": 50,
            "spouse_age": 38
          },
          {
            "distribution_period": 49.2,
            "owner_age": 50,
            "spouse_age": 39
          },
          {
            "distribution_period": 48.5,
            "owner_age": 50,
            "spouse_age": 40
          },
          {
            "distribution_period": 47.8,
            "owner_age": 50,
            "spouse_age": 41
          },
          {
            "distribution_period": 47.1,
            "owner_age": 50,
            "spouse_age": 42
          },
          {
            "distribution_period": 46.4,
            "owner_age": 50,
            "spouse_age": 43
          },
          {
            "distribution_period": 45.7,
            "owner_age": 50,
            "spouse_age": 44
          },
          {
            "distribution_period": 45.1,
            "owner_age": 50,
            "spouse_age": 45
          },
          {
            "distribution_period": 44.5,
            "owner_age": 50,
            "spouse_age": 46
          },
          {
            "distribution_period": 43.9,
            "owner_age": 50,
            "spouse_age": 47
          },
          {
            "distribution_period": 43.3,
            "owner_age": 50,
            "spouse_age": 48
          },
          {
            "distribution_period": 42.8,
            "owner_age": 50,
            "spouse_age": 49
          },
          {
            "distribution_period": 42.3,
            "owner_age": 50,
            "spouse_age": 50
          },
          {
            "distribution_period": 41.8,
            "owner_age": 50,
            "spouse_age": 51
          },
          {
            "distribution_period": 41.4,
            "owner_age": 50,
            "spouse_age": 52
          },
          {
            "distribution_period": 40.9,
            "owner_age": 50,
            "spouse_age": 53
          },
          {
            "distribution_period": 40.6,
            "owner_age": 50,
            "spouse_age": 54
          },
          {
            "distribution_period": 40.2,
            "owner_age": 50,
            "spouse_age": 55
          },
          {
            "distribution_period": 39.8,
            "owner_age": 50,
            "spouse_age": 56
          },
          {
            "distribution_period": 39.5,
            "owner_age": 50,
            "spouse_age": 57
          },
          {
            "distribution_period": 39.2,
            "owner_age": 50,
            "spouse_age": 58
          },
          {
            "distribution_period": 39.0,
            "owner_age": 50,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.6,
            "owner_age": 51,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.7,
            "owner_age": 51,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.8,
            "owner_age": 51,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.8,
            "owner_age": 51,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.9,
            "owner_age": 51,
            "spouse_age": 24
          },
          {
            "distribution_period": 61.0,
            "owner_age": 51,
            "spouse_age": 25
          },
          {
            "distribution_period": 60.1,
            "owner_age": 51,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.2,
            "owner_age": 51,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.3,
            "owner_age": 51,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.4,
            "owner_age": 51,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.5,
            "owner_age": 51,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.6,
            "owner_age": 51,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.8,
            "owner_age": 51,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.9,
            "owner_age": 51,
            "spouse_age": 33
          },
          {
            "distribution_period": 53.1,
            "owner_age": 51,
            "spouse_age": 34
          },
          {
            "distribution_period": 52.2,
            "owner_age": 51,
            "spouse_age": 35
          },
          {
            "distribution_period": 51.4,
            "owner_age": 51,
            "spouse_age": 36
          },
          {
            "distribution_period": 50.6,
            "owner_age": 51,
            "spouse_age": 37
          },
          {
            "distribution_period": 49.8,
            "owner_age": 51,
            "spouse_age": 38
          },
          {
            "distribution_period": 49.0,
            "owner_age": 51,
            "spouse_age": 39
          },
          {
            "distribution_period": 48.3,
            "owner_age": 51,
            "spouse_age": 40
          },
          {
            "distribution_period": 47.5,
            "owner_age": 51,
            "spouse_age": 41
          },
          {
            "distribution_period": 46.8,
            "owner_age": 51,
            "spouse_age": 42
          },
          {
            "distribution_period": 46.1,
            "owner_age": 51,
            "spouse_age": 43
          },
          {
            "distribution_period": 45.4,
            "owner_age": 51,
            "spouse_age": 44
          },
          {
            "distribution_period": 44.7,
            "owner_age": 51,
            "spouse_age": 45
          },
          {
            "distribution_period": 44.1,
            "owner_age": 51,
            "spouse_age": 46
          },
          {
            "distribution_period": 43.5,
            "owner_age": 51,
            "spouse_age": 47
          },
          {
            "distribution_period": 42.9,
            "owner_age": 51,
            "spouse_age": 48
          },
          {
            "distribution_period": 42.3,
            "owner_age": 51,
            "spouse_age": 49
          },
          {
            "distribution_period": 41.8,
            "owner_age": 51,
            "spouse_age": 50
          },
          {
            "distribution_period": 41.3,
            "owner_age": 51,
            "spouse_age": 51
          },
          {
            "distribution_period": 40.8,
            "owner_age": 51,
            "spouse_age": 52
          },
          {
            "distribution_period": 40.4,
            "owner_age": 51,
            "spouse_age": 53
          },
          {
            "distribution_period": 40.0,
            "owner_age": 51,
            "spouse_age": 54
          },
          {
            "distribution_period": 39.6,
            "owner_age": 51,
            "spouse_age": 55
          },
          {
            "distribution_period": 39.2,
            "owner_age": 51,
            "spouse_age": 56
          },
          {
            "distribution_period": 38.9,
            "owner_age": 51,
            "spouse_age": 57
          },
          {
            "distribution_period": 38.6,
            "owner_age": 51,
            "spouse_age": 58
          },
          {
            "distribution_period": 38.3,
            "owner_age": 51,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.6,
            "owner_age": 52,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.7,
            "owner_age": 52,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.7,
            "owner_age": 52,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.8,
            "owner_age": 52,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.9,
            "owner_age": 52,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.9,
            "owner_age": 52,
            "spouse_age": 25
          },
          {
            "distribution_period": 60.0,
            "owner_age": 52,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.1,
            "owner_age": 52,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.2,
            "owner_age": 52,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.3,
            "owner_age": 52,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.4,
            "owner_age": 52,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.5,
            "owner_age": 52,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.7,
            "owner_age": 52,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.8,
            "owner_age": 52,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.9,
            "owner_age": 52,
            "spouse_age": 34
          },
          {
            "distribution_period": 52.1,
            "owner_age": 52,
            "spouse_age": 35
          },
          {
            "distribution_period": 51.3,
            "owner_age": 52,
            "spouse_age": 36
          },
          {
            "distribution_period": 50.4,
            "owner_age": 52,
            "spouse_age": 37
          },
          {
            "distribution_period": 49.6,
            "owner_age": 52,
            "spouse_age": 38
          },
          {
            "distribution_period": 48.8,
            "owner_age": 52,
            "spouse_age": 39
          },
          {
            "distribution_period": 48.0,
            "owner_age": 52,
            "spouse_age": 40
          },
          {
            "distribution_period": 47.3,
            "owner_age": 52,
            "spouse_age": 41
          },
          {
            "distribution_period": 46.5,
            "owner_age": 52,
            "spouse_age": 42
          },
          {
            "distribution_period": 45.8,
            "owner_age": 52,
            "spouse_age": 43
          },
          {
            "distribution_period": 45.1,
            "owner_age": 52,
            "spouse_age": 44
          },
          {
            "distribution_period": 44.4,
            "owner_age": 52,
            "spouse_age": 45
          },
          {
            "distribution_period": 43.8,
            "owner_age": 52,
            "spouse_age": 46
          },
          {
            "distribution_period": 43.1,
            "owner_age": 52,
            "spouse_age": 47
          },
          {
            "distribution_period": 42.5,
            "owner_age": 52,
            "spouse_age": 48
          },
          {
            "distribution_period": 41.9,
            "owner_age": 52,
            "spouse_age": 49
          },
          {
            "distribution_period": 41.4,
            "owner_age": 52,
            "spouse_age": 50
          },
          {
            "distribution_period": 40.8,
            "owner_age": 52,
            "spouse_age": 51
          },
          {
            "distribution_period": 40.3,
            "owner_age": 52,
            "spouse_age": 52
          },
          {
            "distribution_period": 39.9,
            "owner_age": 52,
            "spouse_age": 53
          },
          {
            "distribution_period": 39.4,
            "owner_age": 52,
            "spouse_age": 54
          },
          {
            "distribution_period": 39.0,
            "owner_age": 52,
            "spouse_age": 55
          },
          {
            "distribution_period": 38.6,
            "owner_age": 52,
            "spouse_age": 56
          },
          {
            "distribution_period": 38.2,
            "owner_age": 52,
            "spouse_age": 57
          },
          {
            "distribution_period": 37.9,
            "owner_age": 52,
            "spouse_age": 58
          },
          {
            "distribution_period": 37.6,
            "owner_age": 52,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.5,
            "owner_age": 53,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.6,
            "owner_age": 53,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.7,
            "owner_age": 53,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.7,
            "owner_age": 53,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.8,
            "owner_age": 53,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.9,
            "owner_age": 53,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.9,
            "owner_age": 53,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.0,
            "owner_age": 53,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.1,
            "owner_age": 53,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.2,
            "owner_age": 53,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.3,
            "owner_age": 53,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.4,
            "owner_age": 53,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.6,
            "owner_age": 53,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.7,
            "owner_age": 53,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.8,
            "owner_age": 53,
            "spouse_age": 34
          },
          {
            "distribution_period": 52.0,
            "owner_age": 53,
            "spouse_age": 35
          },
          {
            "distribution_period": 51.1,
            "owner_age": 53,
            "spouse_age": 36
          },
          {
            "distribution_period": 50.3,
            "owner_age": 53,
            "spouse_age": 37
          },
          {
            "distribution_period": 49.5,
            "owner_age": 53,
            "spouse_age": 38
          },
          {
            "distribution_period": 48.6,
            "owner_age": 53,
            "spouse_age": 39
          },
          {
            "distribution_period": 47.8,
            "owner_age": 53,
            "spouse_age": 40
          },
          {
            "distribution_period": 47.1,
            "owner_age": 53,
            "spouse_age": 41
          },
          {
            "distribution_period": 46.3,
            "owner_age": 53,
            "spouse_age": 42
          },
          {
            "distribution_period": 45.6,
            "owner_age": 53,
            "spouse_age": 43
          },
          {
            "distribution_period": 44.8,
            "owner_age": 53,
            "spouse_age": 44
          },
          {
            "distribution_period": 44.1,
            "owner_age": 53,
            "spouse_age": 45
          },
          {
            "distribution_period": 43.4,
            "owner_age": 53,
            "spouse_age": 46
          },
          {
            "distribution_period": 42.8,
            "owner_age": 53,
            "spouse_age": 47
          },
          {
            "distribution_period": 42.1,
            "owner_age": 53,
            "spouse_age": 48
          },
          {
            "distribution_period": 41.5,
            "owner_age": 53,
            "spouse_age": 49
          },
          {
            "distribution_period": 40.9,
            "owner_age": 53,
            "spouse_age": 50
          },
          {
            "distribution_period": 40.4,
            "owner_age": 53,
            "spouse_age": 51
          },
          {
            "distribution_period": 39.9,
            "owner_age": 53,
            "spouse_age": 52
          },
          {
            "distribution_period": 39.4,
            "owner_age": 53,
            "spouse_age": 53
          },
          {
            "distribution_period": 38.9,
            "owner_age": 53,
            "spouse_age": 54
          },
          {
            "distribution_period": 38.4,
            "owner_age": 53,
            "spouse_age": 55
          },
          {
            "distribution_period": 38.0,
            "owner_age": 53,
            "spouse_age": 56
          },
          {
            "distribution_period": 37.6,
            "owner_age": 53,
            "spouse_age": 57
          },
          {
            "distribution_period": 37.3,
            "owner_age": 53,
            "spouse_age": 58
          },
          {
            "distribution_period": 36.9,
            "owner_age": 53,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.5,
            "owner_age": 54,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.6,
            "owner_age": 54,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.6,
            "owner_age": 54,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.7,
            "owner_age": 54,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.7,
            "owner_age": 54,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.8,
            "owner_age": 54,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.9,
            "owner_age": 54,
            "spouse_age": 26
          },
          {
            "distribution_period": 59.0,
            "owner_age": 54,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.0,
            "owner_age": 54,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.1,
            "owner_age": 54,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.2,
            "owner_age": 54,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.3,
            "owner_age": 54,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.5,
            "owner_age": 54,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.6,
            "owner_age": 54,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.7,
            "owner_age": 54,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.8,
            "owner_age": 54,
            "spouse_age": 35
          },
          {
            "distribution_period": 51.0,
            "owner_age": 54,
            "spouse_age": 36
          },
          {
            "distribution_period": 50.1,
            "owner_age": 54,
            "spouse_age": 37
          },
          {
            "distribution_period": 49.3,
            "owner_age": 54,
            "spouse_age": 38
          },
          {
            "distribution_period": 48.5,
            "owner_age": 54,
            "spouse_age": 39
          },
          {
            "distribution_period": 47.7,
            "owner_age": 54,
            "spouse_age": 40
          },
          {
            "distribution_period": 46.9,
            "owner_age": 54,
            "spouse_age": 41
          },
          {
            "distribution_period": 46.1,
            "owner_age": 54,
            "spouse_age": 42
          },
          {
            "distribution_period": 45.3,
            "owner_age": 54,
            "spouse_age": 43
          },
          {
            "distribution_period": 44.6,
            "owner_age": 54,
            "spouse_age": 44
          },
          {
            "distribution_period": 43.8,
            "owner_age": 54,
            "spouse_age": 45
          },
          {
            "distribution_period": 43.1,
            "owner_age": 54,
            "spouse_age": 46
          },
          {
            "distribution_period": 42.5,
            "owner_age": 54,
            "spouse_age": 47
          },
          {
            "distribution_period": 41.8,
            "owner_age": 54,
            "spouse_age": 48
          },
          {
            "distribution_period": 41.2,
            "owner_age": 54,
            "spouse_age": 49
          },
          {
            "distribution_period": 40.6,
            "owner_age": 54,
            "spouse_age": 50
          },
          {
            "distribution_period": 40.0,
            "owner_age": 54,
            "spouse_age": 51
          },
          {
            "distribution_period": 39.4,
            "owner_age": 54,
            "spouse_age": 52
          },
          {
            "distribution_period": 38.9,
            "owner_age": 54,
            "spouse_age": 53
          },
          {
            "distribution_period": 38.4,
            "owner_age": 54,
            "spouse_age": 54
          },
          {
            "distribution_period": 37.9,
            "owner_age": 54,
            "spouse_age": 55
          },
          {
            "distribution_period": 37.5,
            "owner_age": 54,
            "spouse_age": 56
          },
          {
            "distribution_period": 37.1,
            "owner_age": 54,
            "spouse_age": 57
          },
          {
            "distribution_period": 36.7,
            "owner_age": 54,
            "spouse_age": 58
          },
          {
            "distribution_period": 36.3,
            "owner_age": 54,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.5,
            "owner_age": 55,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.5,
            "owner_age": 55,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.6,
            "owner_age": 55,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.6,
            "owner_age": 55,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.7,
            "owner_age": 55,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.8,
            "owner_age": 55,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.8,
            "owner_age": 55,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.9,
            "owner_age": 55,
            "spouse_age": 27
          },
          {
            "distribution_period": 58.0,
            "owner_age": 55,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.1,
            "owner_age": 55,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.2,
            "owner_age": 55,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.3,
            "owner_age": 55,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.4,
            "owner_age": 55,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.5,
            "owner_age": 55,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.6,
            "owner_age": 55,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.7,
            "owner_age": 55,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.9,
            "owner_age": 55,
            "spouse_age": 36
          },
          {
            "distribution_period": 50.0,
            "owner_age": 55,
            "spouse_age": 37
          },
          {
            "distribution_period": 49.1,
            "owner_age": 55,
            "spouse_age": 38
          },
          {
            "distribution_period": 48.3,
            "owner_age": 55,
            "spouse_age": 39
          },
          {
            "distribution_period": 47.5,
            "owner_age": 55,
            "spouse_age": 40
          },
          {
            "distribution_period": 46.7,
            "owner_age": 55,
            "spouse_age": 41
          },
          {
            "distribution_period": 45.9,
            "owner_age": 55,
            "spouse_age": 42
          },
          {
            "distribution_period": 45.1,
            "owner_age": 55,
            "spouse_age": 43
          },
          {
            "distribution_period": 44.3,
            "owner_age": 55,
            "spouse_age": 44
          },
          {
            "distribution_period": 43.6,
            "owner_age": 55,
            "spouse_age": 45
          },
          {
            "distribution_period": 42.9,
            "owner_age": 55,
            "spouse_age": 46
          },
          {
            "distribution_period": 42.2,
            "owner_age": 55,
            "spouse_age": 47
          },
          {
            "distribution_period": 41.5,
            "owner_age": 55,
            "spouse_age": 48
          },
          {
            "distribution_period": 40.8,
            "owner_age": 55,
            "spouse_age": 49
          },
          {
            "distribution_period": 40.2,
            "owner_age": 55,
            "spouse_age": 50
          },
          {
            "distribution_period": 39.6,
            "owner_age": 55,
            "spouse_age": 51
          },
          {
            "distribution_period": 39.0,
            "owner_age": 55,
            "spouse_age": 52
          },
          {
            "distribution_period": 38.4,
            "owner_age": 55,
            "spouse_age": 53
          },
          {
            "distribution_period": 37.9,
            "owner_age": 55,
            "spouse_age": 54
          },
          {
            "distribution_period": 37.4,
            "owner_age": 55,
            "spouse_age": 55
          },
          {
            "distribution_period": 36.9,
            "owner_age": 55,
            "spouse_age": 56
          },
          {
            "distribution_period": 36.5,
            "owner_age": 55,
            "spouse_age": 57
          },
          {
            "distribution_period": 36.1,
            "owner_age": 55,
            "spouse_age": 58
          },
          {
            "distribution_period": 35.7,
            "owner_age": 55,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.4,
            "owner_age": 56,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.5,
            "owner_age": 56,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.5,
            "owner_age": 56,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.6,
            "owner_age": 56,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.6,
            "owner_age": 56,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.7,
            "owner_age": 56,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.8,
            "owner_age": 56,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.8,
            "owner_age": 56,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.9,
            "owner_age": 56,
            "spouse_age": 28
          },
          {
            "distribution_period": 57.0,
            "owner_age": 56,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.1,
            "owner_age": 56,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.2,
            "owner_age": 56,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.3,
            "owner_age": 56,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.4,
            "owner_age": 56,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.5,
            "owner_age": 56,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.6,
            "owner_age": 56,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.7,
            "owner_age": 56,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.9,
            "owner_age": 56,
            "spouse_age": 37
          },
          {
            "distribution_period": 49.0,
            "owner_age": 56,
            "spouse_age": 38
          },
          {
            "distribution_period": 48.2,
            "owner_age": 56,
            "spouse_age": 39
          },
          {
            "distribution_period": 47.3,
            "owner_age": 56,
            "spouse_age": 40
          },
          {
            "distribution_period": 46.5,
            "owner_age": 56,
            "spouse_age": 41
          },
          {
            "distribution_period": 45.7,
            "owner_age": 56,
            "spouse_age": 42
          },
          {
            "distribution_period": 44.9,
            "owner_age": 56,
            "spouse_age": 43
          },
          {
            "distribution_period": 44.1,
            "owner_age": 56,
            "spouse_age": 44
          },
          {
            "distribution_period": 43.4,
            "owner_age": 56,
            "spouse_age": 45
          },
          {
            "distribution_period": 42.6,
            "owner_age": 56,
            "spouse_age": 46
          },
          {
            "distribution_period": 41.9,
            "owner_age": 56,
            "spouse_age": 47
          },
          {
            "distribution_period": 41.2,
            "owner_age": 56,
            "spouse_age": 48
          },
          {
            "distribution_period": 40.5,
            "owner_age": 56,
            "spouse_age": 49
          },
          {
            "distribution_period": 39.8,
            "owner_age": 56,
            "spouse_age": 50
          },
          {
            "distribution_period": 39.2,
            "owner_age": 56,
            "spouse_age": 51
          },
          {
            "distribution_period": 38.6,
            "owner_age": 56,
            "spouse_age": 52
          },
          {
            "distribution_period": 38.0,
            "owner_age": 56,
            "spouse_age": 53
          },
          {
            "distribution_period": 37.5,
            "owner_age": 56,
            "spouse_age": 54
          },
          {
            "distribution_period": 36.9,
            "owner_age": 56,
            "spouse_age": 55
          },
          {
            "distribution_period": 36.5,
            "owner_age": 56,
            "spouse_age": 56
          },
          {
            "distribution_period": 36.0,
            "owner_age": 56,
            "spouse_age": 57
          },
          {
            "distribution_period": 35.5,
            "owner_age": 56,
            "spouse_age": 58
          },
          {
            "distribution_period": 35.1,
            "owner_age": 56,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.4,
            "owner_age": 57,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.5,
            "owner_age": 57,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.5,
            "owner_age": 57,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.5,
            "owner_age": 57,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.6,
            "owner_age": 57,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.7,
            "owner_age": 57,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.7,
            "owner_age": 57,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.8,
            "owner_age": 57,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.9,
            "owner_age": 57,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.9,
            "owner_age": 57,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.0,
            "owner_age": 57,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.1,
            "owner_age": 57,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.2,
            "owner_age": 57,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.3,
            "owner_age": 57,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.4,
            "owner_age": 57,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.5,
            "owner_age": 57,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.6,
            "owner_age": 57,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.8,
            "owner_age": 57,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.9,
            "owner_age": 57,
            "spouse_age": 38
          },
          {
            "distribution_period": 48.0,
            "owner_age": 57,
            "spouse_age": 39
          },
          {
            "distribution_period": 47.2,
            "owner_age": 57,
            "spouse_age": 40
          },
          {
            "distribution_period": 46.3,
            "owner_age": 57,
            "spouse_age": 41
          },
          {
            "distribution_period": 45.5,
            "owner_age": 57,
            "spouse_age": 42
          },
          {
            "distribution_period": 44.7,
            "owner_age": 57,
            "spouse_age": 43
          },
          {
            "distribution_period": 43.9,
            "owner_age": 57,
            "spouse_age": 44
          },
          {
            "distribution_period": 43.1,
            "owner_age": 57,
            "spouse_age": 45
          },
          {
            "distribution_period": 42.4,
            "owner_age": 57,
            "spouse_age": 46
          },
          {
            "distribution_period": 41.6,
            "owner_age": 57,
            "spouse_age": 47
          },
          {
            "distribution_period": 40.9,
            "owner_age": 57,
            "spouse_age": 48
          },
          {
            "distribution_period": 40.2,
            "owner_age": 57,
            "spouse_age": 49
          },
          {
            "distribution_period": 39.5,
            "owner_age": 57,
            "spouse_age": 50
          },
          {
            "distribution_period": 38.9,
            "owner_age": 57,
            "spouse_age": 51
          },
          {
            "distribution_period": 38.2,
            "owner_age": 57,
            "spouse_age": 52
          },
          {
            "distribution_period": 37.6,
            "owner_age": 57,
            "spouse_age": 53
          },
          {
            "distribution_period": 37.1,
            "owner_age": 57,
            "spouse_age": 54
          },
          {
            "distribution_period": 36.5,
            "owner_age": 57,
            "spouse_age": 55
          },
          {
            "distribution_period": 36.0,
            "owner_age": 57,
            "spouse_age": 56
          },
          {
            "distribution_period": 35.5,
            "owner_age": 57,
            "spouse_age": 57
          },
          {
            "distribution_period": 35.0,
            "owner_age": 57,
            "spouse_age": 58
          },
          {
            "distribution_period": 34.6,
            "owner_age": 57,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.4,
            "owner_age": 58,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.4,
            "owner_age": 58,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.5,
            "owner_age": 58,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.5,
            "owner_age": 58,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.6,
            "owner_age": 58,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.6,
            "owner_age": 58,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.7,
            "owner_age": 58,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.7,
            "owner_age": 58,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.8,
            "owner_age": 58,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.9,
            "owner_age": 58,
            "spouse_age": 29
          },
          {
            "distribution_period": 56.0,
            "owner_age": 58,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.0,
            "owner_age": 58,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.1,
            "owner_age": 58,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.2,
            "owner_age": 58,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.3,
            "owner_age": 58,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.4,
            "owner_age": 58,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.5,
            "owner_age": 58,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.7,
            "owner_age": 58,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.8,
            "owner_age": 58,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.9,
            "owner_age": 58,
            "spouse_age": 39
          },
          {
            "distribution_period": 47.1,
            "owner_age": 58,
            "spouse_age": 40
          },
          {
            "distribution_period": 46.2,
            "owner_age": 58,
            "spouse_age": 41
          },
          {
            "distribution_period": 45.4,
            "owner_age": 58,
            "spouse_age": 42
          },
          {
            "distribution_period": 44.5,
            "owner_age": 58,
            "spouse_age": 43
          },
          {
            "distribution_period": 43.7,
            "owner_age": 58,
            "spouse_age": 44
          },
          {
            "distribution_period": 42.9,
            "owner_age": 58,
            "spouse_age": 45
          },
          {
            "distribution_period": 42.2,
            "owner_age": 58,
            "spouse_age": 46
          },
          {
            "distribution_period": 41.4,
            "owner_age": 58,
            "spouse_age": 47
          },
          {
            "distribution_period": 40.7,
            "owner_age": 58,
            "spouse_age": 48
          },
          {
            "distribution_period": 39.9,
            "owner_age": 58,
            "spouse_age": 49
          },
          {
            "distribution_period": 39.2,
            "owner_age": 58,
            "spouse_age": 50
          },
          {
            "distribution_period": 38.6,
            "owner_age": 58,
            "spouse_age": 51
          },
          {
            "distribution_period": 37.9,
            "owner_age": 58,
            "spouse_age": 52
          },
          {
            "distribution_period": 37.3,
            "owner_age": 58,
            "spouse_age": 53
          },
          {
            "distribution_period": 36.7,
            "owner_age": 58,
            "spouse_age": 54
          },
          {
            "distribution_period": 36.1,
            "owner_age": 58,
            "spouse_age": 55
          },
          {
            "distribution_period": 35.5,
            "owner_age": 58,
            "spouse_age": 56
          },
          {
            "distribution_period": 35.0,
            "owner_age": 58,
            "spouse_age": 57
          },
          {
            "distribution_period": 34.5,
            "owner_age": 58,
            "spouse_age": 58
          },
          {
            "distribution_period": 34.1,
            "owner_age": 58,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.4,
            "owner_age": 59,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.4,
            "owner_age": 59,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.4,
            "owner_age": 59,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.5,
            "owner_age": 59,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.5,
            "owner_age": 59,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.6,
            "owner_age": 59,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.6,
            "owner_age": 59,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.7,
            "owner_age": 59,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.8,
            "owner_age": 59,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.8,
            "owner_age": 59,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.9,
            "owner_age": 59,
            "spouse_age": 30
          },
          {
            "distribution_period": 55.0,
            "owner_age": 59,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.1,
            "owner_age": 59,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.2,
            "owner_age": 59,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.2,
            "owner_age": 59,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.3,
            "owner_age": 59,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.5,
            "owner_age": 59,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.6,
            "owner_age": 59,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.7,
            "owner_age": 59,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.8,
            "owner_age": 59,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.9,
            "owner_age": 59,
            "spouse_age": 40
          },
          {
            "distribution_period": 46.1,
            "owner_age": 59,
            "spouse_age": 41
          },
          {
            "distribution_period": 45.2,
            "owner_age": 59,
            "spouse_age": 42
          },
          {
            "distribution_period": 44.4,
            "owner_age": 59,
            "spouse_age": 43
          },
          {
            "distribution_period": 43.6,
            "owner_age": 59,
            "spouse_age": 44
          },
          {
            "distribution_period": 42.8,
            "owner_age": 59,
            "spouse_age": 45
          },
          {
            "distribution_period": 42.0,
            "owner_age": 59,
            "spouse_age": 46
          },
          {
            "distribution_period": 41.2,
            "owner_age": 59,
            "spouse_age": 47
          },
          {
            "distribution_period": 40.4,
            "owner_age": 59,
            "spouse_age": 48
          },
          {
            "distribution_period": 39.7,
            "owner_age": 59,
            "spouse_age": 49
          },
          {
            "distribution_period": 39.0,
            "owner_age": 59,
            "spouse_age": 50
          },
          {
            "distribution_period": 38.3,
            "owner_age": 59,
            "spouse_age": 51
          },
          {
            "distribution_period": 37.6,
            "owner_age": 59,
            "spouse_age": 52
          },
          {
            "distribution_period": 36.9,
            "owner_age": 59,
            "spouse_age": 53
          },
          {
            "distribution_period": 36.3,
            "owner_age": 59,
            "spouse_age": 54
          },
          {
            "distribution_period": 35.7,
            "owner_age": 59,
            "spouse_age": 55
          },
          {
            "distribution_period": 35.1,
            "owner_age": 59,
            "spouse_age": 56
          },
          {
            "distribution_period": 34.6,
            "owner_age": 59,
            "spouse_age": 57
          },
          {
            "distribution_period": 34.1,
            "owner_age": 59,
            "spouse_age": 58
          },
          {
            "distribution_period": 33.6,
            "owner_age": 59,
            "spouse_age": 59
          },
          {
            "distribution_period": 65.3,
            "owner_age": 60,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.4,
            "owner_age": 60,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.4,
            "owner_age": 60,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.4,
            "owner_age": 60,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.5,
            "owner_age": 60,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.5,
            "owner_age": 60,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.6,
            "owner_age": 60,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.7,
            "owner_age": 60,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.7,
            "owner_age": 60,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.8,
            "owner_age": 60,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.9,
            "owner_age": 60,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.9,
            "owner_age": 60,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.0,
            "owner_age": 60,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.1,
            "owner_age": 60,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.2,
            "owner_age": 60,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.3,
            "owner_age": 60,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.4,
            "owner_age": 60,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.5,
            "owner_age": 60,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.6,
            "owner_age": 60,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.7,
            "owner_age": 60,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.8,
            "owner_age": 60,
            "spouse_age": 40
          },
          {
            "distribution_period": 46.0,
            "owner_age": 60,
            "spouse_age": 41
          },
          {
            "distribution_period": 45.1,
            "owner_age": 60,
            "spouse_age": 42
          },
          {
            "distribution_period": 44.3,
            "owner_age": 60,
            "spouse_age": 43
          },
          {
            "distribution_period": 43.4,
            "owner_age": 60,
            "spouse_age": 44
          },
          {
            "distribution_period": 42.6,
            "owner_age": 60,
            "spouse_age": 45
          },
          {
            "distribution_period": 41.8,
            "owner_age": 60,
            "spouse_age": 46
          },
          {
            "distribution_period": 41.0,
            "owner_age": 60,
            "spouse_age": 47
          },
          {
            "distribution_period": 40.2,
            "owner_age": 60,
            "spouse_age": 48
          },
          {
            "distribution_period": 39.5,
            "owner_age": 60,
            "spouse_age": 49
          },
          {
            "distribution_period": 38.7,
            "owner_age": 60,
            "spouse_age": 50
          },
          {
            "distribution_period": 38.0,
            "owner_age": 60,
            "spouse_age": 51
          },
          {
            "distribution_period": 37.3,
            "owner_age": 60,
            "spouse_age": 52
          },
          {
            "distribution_period": 36.6,
            "owner_age": 60,
            "spouse_age": 53
          },
          {
            "distribution_period": 36.0,
            "owner_age": 60,
            "spouse_age": 54
          },
          {
            "distribution_period": 35.3,
            "owner_age": 60,
            "spouse_age": 55
          },
          {
            "distribution_period": 34.8,
            "owner_age": 60,
            "spouse_age": 56
          },
          {
            "distribution_period": 34.2,
            "owner_age": 60,
            "spouse_age": 57
          },
          {
            "distribution_period": 33.6,
            "owner_age": 60,
            "spouse_age": 58
          },
          {
            "distribution_period": 33.1,
            "owner_age": 60,
            "spouse_age": 59
          },
          {
            "distribution_period": 32.6,
            "owner_age": 60,
            "spouse_age": 60
          },
          {
            "distribution_period": 32.2,
            "owner_age": 60,
            "spouse_age": 61
          },
          {
            "distribution_period": 31.7,
            "owner_age": 60,
            "spouse_age": 62
          },
          {
            "distribution_period": 31.3,
            "owner_age": 60,
            "spouse_age": 63
          },
          {
            "distribution_period": 31.0,
            "owner_age": 60,
            "spouse_age": 64
          },
          {
            "distribution_period": 30.6,
            "owner_age": 60,
            "spouse_age": 65
          },
          {
            "distribution_period": 30.3,
            "owner_age": 60,
            "spouse_age": 66
          },
          {
            "distribution_period": 30.0,
            "owner_age": 60,
            "spouse_age": 67
          },
          {
            "distribution_period": 29.7,
            "owner_age": 60,
            "spouse_age": 68
          },
          {
            "distribution_period": 29.4,
            "owner_age": 60,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.3,
            "owner_age": 61,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.3,
            "owner_age": 61,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.4,
            "owner_age": 61,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.4,
            "owner_age": 61,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.5,
            "owner_age": 61,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.5,
            "owner_age": 61,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.6,
            "owner_age": 61,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.6,
            "owner_age": 61,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.7,
            "owner_age": 61,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.7,
            "owner_age": 61,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.8,
            "owner_age": 61,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.9,
            "owner_age": 61,
            "spouse_age": 31
          },
          {
            "distribution_period": 54.0,
            "owner_age": 61,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.0,
            "owner_age": 61,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.1,
            "owner_age": 61,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.2,
            "owner_age": 61,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.3,
            "owner_age": 61,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.4,
            "owner_age": 61,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.5,
            "owner_age": 61,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.6,
            "owner_age": 61,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.7,
            "owner_age": 61,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.8,
            "owner_age": 61,
            "spouse_age": 41
          },
          {
            "distribution_period": 45.0,
            "owner_age": 61,
            "spouse_age": 42
          },
          {
            "distribution_period": 44.1,
            "owner_age": 61,
            "spouse_age": 43
          },
          {
            "distribution_period": 43.3,
            "owner_age": 61,
            "spouse_age": 44
          },
          {
            "distribution_period": 42.4,
            "owner_age": 61,
            "spouse_age": 45
          },
          {
            "distribution_period": 41.6,
            "owner_age": 61,
            "spouse_age": 46
          },
          {
            "distribution_period": 40.8,
            "owner_age": 61,
            "spouse_age": 47
          },
          {
            "distribution_period": 40.0,
            "owner_age": 61,
            "spouse_age": 48
          },
          {
            "distribution_period": 39.2,
            "owner_age": 61,
            "spouse_age": 49
          },
          {
            "distribution_period": 38.5,
            "owner_age": 61,
            "spouse_age": 50
          },
          {
            "distribution_period": 37.7,
            "owner_age": 61,
            "spouse_age": 51
          },
          {
            "distribution_period": 37.0,
            "owner_age": 61,
            "spouse_age": 52
          },
          {
            "distribution_period": 36.3,
            "owner_age": 61,
            "spouse_age": 53
          },
          {
            "distribution_period": 35.7,
            "owner_age": 61,
            "spouse_age": 54
          },
          {
            "distribution_period": 35.0,
            "owner_age": 61,
            "spouse_age": 55
          },
          {
            "distribution_period": 34.4,
            "owner_age": 61,
            "spouse_age": 56
          },
          {
            "distribution_period": 33.8,
            "owner_age": 61,
            "spouse_age": 57
          },
          {
            "distribution_period": 33.2,
            "owner_age": 61,
            "spouse_age": 58
          },
          {
            "distribution_period": 32.7,
            "owner_age": 61,
            "spouse_age": 59
          },
          {
            "distribution_period": 32.2,
            "owner_age": 61,
            "spouse_age": 60
          },
          {
            "distribution_period": 31.7,
            "owner_age": 61,
            "spouse_age": 61
          },
          {
            "distribution_period": 31.2,
            "owner_age": 61,
            "spouse_age": 62
          },
          {
            "distribution_period": 30.8,
            "owner_age": 61,
            "spouse_age": 63
          },
          {
            "distribution_period": 30.4,
            "owner_age": 61,
            "spouse_age": 64
          },
          {
            "distribution_period": 30.0,
            "owner_age": 61,
            "spouse_age": 65
          },
          {
            "distribution_period": 29.7,
            "owner_age": 61,
            "spouse_age": 66
          },
          {
            "distribution_period": 29.4,
            "owner_age": 61,
            "spouse_age": 67
          },
          {
            "distribution_period": 29.1,
            "owner_age": 61,
            "spouse_age": 68
          },
          {
            "distribution_period": 28.8,
            "owner_age": 61,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.3,
            "owner_age": 62,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.3,
            "owner_age": 62,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.4,
            "owner_age": 62,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.4,
            "owner_age": 62,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.4,
            "owner_age": 62,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.5,
            "owner_age": 62,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.5,
            "owner_age": 62,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.6,
            "owner_age": 62,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.6,
            "owner_age": 62,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.7,
            "owner_age": 62,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.8,
            "owner_age": 62,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.8,
            "owner_age": 62,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.9,
            "owner_age": 62,
            "spouse_age": 32
          },
          {
            "distribution_period": 53.0,
            "owner_age": 62,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.1,
            "owner_age": 62,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.1,
            "owner_age": 62,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.2,
            "owner_age": 62,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.3,
            "owner_age": 62,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.4,
            "owner_age": 62,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.5,
            "owner_age": 62,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.6,
            "owner_age": 62,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.7,
            "owner_age": 62,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.9,
            "owner_age": 62,
            "spouse_age": 42
          },
          {
            "distribution_period": 44.0,
            "owner_age": 62,
            "spouse_age": 43
          },
          {
            "distribution_period": 43.1,
            "owner_age": 62,
            "spouse_age": 44
          },
          {
            "distribution_period": 42.3,
            "owner_age": 62,
            "spouse_age": 45
          },
          {
            "distribution_period": 41.5,
            "owner_age": 62,
            "spouse_age": 46
          },
          {
            "distribution_period": 40.6,
            "owner_age": 62,
            "spouse_age": 47
          },
          {
            "distribution_period": 39.8,
            "owner_age": 62,
            "spouse_age": 48
          },
          {
            "distribution_period": 39.0,
            "owner_age": 62,
            "spouse_age": 49
          },
          {
            "distribution_period": 38.3,
            "owner_age": 62,
            "spouse_age": 50
          },
          {
            "distribution_period": 37.5,
            "owner_age": 62,
            "spouse_age": 51
          },
          {
            "distribution_period": 36.8,
            "owner_age": 62,
            "spouse_age": 52
          },
          {
            "distribution_period": 36.1,
            "owner_age": 62,
            "spouse_age": 53
          },
          {
            "distribution_period": 35.4,
            "owner_age": 62,
            "spouse_age": 54
          },
          {
            "distribution_period": 34.7,
            "owner_age": 62,
            "spouse_age": 55
          },
          {
            "distribution_period": 34.1,
            "owner_age": 62,
            "spouse_age": 56
          },
          {
            "distribution_period": 33.4,
            "owner_age": 62,
            "spouse_age": 57
          },
          {
            "distribution_period": 32.8,
            "owner_age": 62,
            "spouse_age": 58
          },
          {
            "distribution_period": 32.3,
            "owner_age": 62,
            "spouse_age": 59
          },
          {
            "distribution_period": 31.7,
            "owner_age": 62,
            "spouse_age": 60
          },
          {
            "distribution_period": 31.2,
            "owner_age": 62,
            "spouse_age": 61
          },
          {
            "distribution_period": 30.8,
            "owner_age": 62,
            "spouse_age": 62
          },
          {
            "distribution_period": 30.3,
            "owner_age": 62,
            "spouse_age": 63
          },
          {
            "distribution_period": 29.9,
            "owner_age": 62,
            "spouse_age": 64
          },
          {
            "distribution_period": 29.5,
            "owner_age": 62,
            "spouse_age": 65
          },
          {
            "distribution_period": 29.1,
            "owner_age": 62,
            "spouse_age": 66
          },
          {
            "distribution_period": 28.7,
            "owner_age": 62,
            "spouse_age": 67
          },
          {
            "distribution_period": 28.4,
            "owner_age": 62,
            "spouse_age": 68
          },
          {
            "distribution_period": 28.1,
            "owner_age": 62,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.3,
            "owner_age": 63,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.3,
            "owner_age": 63,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.3,
            "owner_age": 63,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.4,
            "owner_age": 63,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.4,
            "owner_age": 63,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.5,
            "owner_age": 63,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.5,
            "owner_age": 63,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.6,
            "owner_age": 63,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.6,
            "owner_age": 63,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.7,
            "owner_age": 63,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.7,
            "owner_age": 63,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.8,
            "owner_age": 63,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.9,
            "owner_age": 63,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.9,
            "owner_age": 63,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.0,
            "owner_age": 63,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.1,
            "owner_age": 63,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.2,
            "owner_age": 63,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.3,
            "owner_age": 63,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.3,
            "owner_age": 63,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.4,
            "owner_age": 63,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.5,
            "owner_age": 63,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.7,
            "owner_age": 63,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.8,
            "owner_age": 63,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.9,
            "owner_age": 63,
            "spouse_age": 43
          },
          {
            "distribution_period": 43.0,
            "owner_age": 63,
            "spouse_age": 44
          },
          {
            "distribution_period": 42.2,
            "owner_age": 63,
            "spouse_age": 45
          },
          {
            "distribution_period": 41.3,
            "owner_age": 63,
            "spouse_age": 46
          },
          {
            "distribution_period": 40.5,
            "owner_age": 63,
            "spouse_age": 47
          },
          {
            "distribution_period": 39.7,
            "owner_age": 63,
            "spouse_age": 48
          },
          {
            "distribution_period": 38.9,
            "owner_age": 63,
            "spouse_age": 49
          },
          {
            "distribution_period": 38.1,
            "owner_age": 63,
            "spouse_age": 50
          },
          {
            "distribution_period": 37.3,
            "owner_age": 63,
            "spouse_age": 51
          },
          {
            "distribution_period": 36.6,
            "owner_age": 63,
            "spouse_age": 52
          },
          {
            "distribution_period": 35.8,
            "owner_age": 63,
            "spouse_age": 53
          },
          {
            "distribution_period": 35.1,
            "owner_age": 63,
            "spouse_age": 54
          },
          {
            "distribution_period": 34.4,
            "owner_age": 63,
            "spouse_age": 55
          },
          {
            "distribution_period": 33.8,
            "owner_age": 63,
            "spouse_age": 56
          },
          {
            "distribution_period": 33.1,
            "owner_age": 63,
            "spouse_age": 57
          },
          {
            "distribution_period": 32.5,
            "owner_age": 63,
            "spouse_age": 58
          },
          {
            "distribution_period": 31.9,
            "owner_age": 63,
            "spouse_age": 59
          },
          {
            "distribution_period": 31.3,
            "owner_age": 63,
            "spouse_age": 60
          },
          {
            "distribution_period": 30.8,
            "owner_age": 63,
            "spouse_age": 61
          },
          {
            "distribution_period": 30.3,
            "owner_age": 63,
            "spouse_age": 62
          },
          {
            "distribution_period": 29.8,
            "owner_age": 63,
            "spouse_age": 63
          },
          {
            "distribution_period": 29.4,
            "owner_age": 63,
            "spouse_age": 64
          },
          {
            "distribution_period": 28.9,
            "owner_age": 63,
            "spouse_age": 65
          },
          {
            "distribution_period": 28.5,
            "owner_age": 63,
            "spouse_age": 66
          },
          {
            "distribution_period": 28.2,
            "owner_age": 63,
            "spouse_age": 67
          },
          {
            "distribution_period": 27.8,
            "owner_age": 63,
            "spouse_age": 68
          },
          {
            "distribution_period": 27.5,
            "owner_age": 63,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.2,
            "owner_age": 64,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.3,
            "owner_age": 64,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.3,
            "owner_age": 64,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.3,
            "owner_age": 64,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.4,
            "owner_age": 64,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.4,
            "owner_age": 64,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.5,
            "owner_age": 64,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.5,
            "owner_age": 64,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.6,
            "owner_age": 64,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.6,
            "owner_age": 64,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.7,
            "owner_age": 64,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.8,
            "owner_age": 64,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.8,
            "owner_age": 64,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.9,
            "owner_age": 64,
            "spouse_age": 33
          },
          {
            "distribution_period": 52.0,
            "owner_age": 64,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.0,
            "owner_age": 64,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.1,
            "owner_age": 64,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.2,
            "owner_age": 64,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.3,
            "owner_age": 64,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.4,
            "owner_age": 64,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.5,
            "owner_age": 64,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.6,
            "owner_age": 64,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.7,
            "owner_age": 64,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.8,
            "owner_age": 64,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.9,
            "owner_age": 64,
            "spouse_age": 44
          },
          {
            "distribution_period": 42.1,
            "owner_age": 64,
            "spouse_age": 45
          },
          {
            "distribution_period": 41.2,
            "owner_age": 64,
            "spouse_age": 46
          },
          {
            "distribution_period": 40.4,
            "owner_age": 64,
            "spouse_age": 47
          },
          {
            "distribution_period": 39.5,
            "owner_age": 64,
            "spouse_age": 48
          },
          {
            "distribution_period": 38.7,
            "owner_age": 64,
            "spouse_age": 49
          },
          {
            "distribution_period": 37.9,
            "owner_age": 64,
            "spouse_age": 50
          },
          {
            "distribution_period": 37.1,
            "owner_age": 64,
            "spouse_age": 51
          },
          {
            "distribution_period": 36.3,
            "owner_age": 64,
            "spouse_age": 52
          },
          {
            "distribution_period": 35.6,
            "owner_age": 64,
            "spouse_age": 53
          },
          {
            "distribution_period": 34.9,
            "owner_age": 64,
            "spouse_age": 54
          },
          {
            "distribution_period": 34.2,
            "owner_age": 64,
            "spouse_age": 55
          },
          {
            "distribution_period": 33.5,
            "owner_age": 64,
            "spouse_age": 56
          },
          {
            "distribution_period": 32.8,
            "owner_age": 64,
            "spouse_age": 57
          },
          {
            "distribution_period": 32.2,
            "owner_age": 64,
            "spouse_age": 58
          },
          {
            "distribution_period": 31.5,
            "owner_age": 64,
            "spouse_age": 59
          },
          {
            "distribution_period": 31.0,
            "owner_age": 64,
            "spouse_age": 60
          },
          {
            "distribution_period": 30.4,
            "owner_age": 64,
            "spouse_age": 61
          },
          {
            "distribution_period": 29.9,
            "owner_age": 64,
            "spouse_age": 62
          },
          {
            "distribution_period": 29.4,
            "owner_age": 64,
            "spouse_age": 63
          },
          {
            "distribution_period": 28.9,
            "owner_age": 64,
            "spouse_age": 64
          },
          {
            "distribution_period": 28.4,
            "owner_age": 64,
            "spouse_age": 65
          },
          {
            "distribution_period": 28.0,
            "owner_age": 64,
            "spouse_age": 66
          },
          {
            "distribution_period": 27.6,
            "owner_age": 64,
            "spouse_age": 67
          },
          {
            "distribution_period": 27.2,
            "owner_age": 64,
            "spouse_age": 68
          },
          {
            "distribution_period": 26.9,
            "owner_age": 64,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.2,
            "owner_age": 65,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.3,
            "owner_age": 65,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.3,
            "owner_age": 65,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.3,
            "owner_age": 65,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.4,
            "owner_age": 65,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.4,
            "owner_age": 65,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.5,
            "owner_age": 65,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.5,
            "owner_age": 65,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.5,
            "owner_age": 65,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.6,
            "owner_age": 65,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.7,
            "owner_age": 65,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.7,
            "owner_age": 65,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.8,
            "owner_age": 65,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.8,
            "owner_age": 65,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.9,
            "owner_age": 65,
            "spouse_age": 34
          },
          {
            "distribution_period": 51.0,
            "owner_age": 65,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.1,
            "owner_age": 65,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.1,
            "owner_age": 65,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.2,
            "owner_age": 65,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.3,
            "owner_age": 65,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.4,
            "owner_age": 65,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.5,
            "owner_age": 65,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.6,
            "owner_age": 65,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.7,
            "owner_age": 65,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.8,
            "owner_age": 65,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.9,
            "owner_age": 65,
            "spouse_age": 45
          },
          {
            "distribution_period": 41.1,
            "owner_age": 65,
            "spouse_age": 46
          },
          {
            "distribution_period": 40.2,
            "owner_age": 65,
            "spouse_age": 47
          },
          {
            "distribution_period": 39.4,
            "owner_age": 65,
            "spouse_age": 48
          },
          {
            "distribution_period": 38.6,
            "owner_age": 65,
            "spouse_age": 49
          },
          {
            "distribution_period": 37.7,
            "owner_age": 65,
            "spouse_age": 50
          },
          {
            "distribution_period": 36.9,
            "owner_age": 65,
            "spouse_age": 51
          },
          {
            "distribution_period": 36.2,
            "owner_age": 65,
            "spouse_age": 52
          },
          {
            "distribution_period": 35.4,
            "owner_age": 65,
            "spouse_age": 53
          },
          {
            "distribution_period": 34.6,
            "owner_age": 65,
            "spouse_age": 54
          },
          {
            "distribution_period": 33.9,
            "owner_age": 65,
            "spouse_age": 55
          },
          {
            "distribution_period": 33.2,
            "owner_age": 65,
            "spouse_age": 56
          },
          {
            "distribution_period": 32.5,
            "owner_age": 65,
            "spouse_age": 57
          },
          {
            "distribution_period": 31.9,
            "owner_age": 65,
            "spouse_age": 58
          },
          {
            "distribution_period": 31.2,
            "owner_age": 65,
            "spouse_age": 59
          },
          {
            "distribution_period": 30.6,
            "owner_age": 65,
            "spouse_age": 60
          },
          {
            "distribution_period": 30.0,
            "owner_age": 65,
            "spouse_age": 61
          },
          {
            "distribution_period": 29.5,
            "owner_age": 65,
            "spouse_age": 62
          },
          {
            "distribution_period": 28.9,
            "owner_age": 65,
            "spouse_age": 63
          },
          {
            "distribution_period": 28.4,
            "owner_age": 65,
            "spouse_age": 64
          },
          {
            "distribution_period": 28.0,
            "owner_age": 65,
            "spouse_age": 65
          },
          {
            "distribution_period": 27.5,
            "owner_age": 65,
            "spouse_age": 66
          },
          {
            "distribution_period": 27.1,
            "owner_age": 65,
            "spouse_age": 67
          },
          {
            "distribution_period": 26.7,
            "owner_age": 65,
            "spouse_age": 68
          },
          {
            "distribution_period": 26.3,
            "owner_age": 65,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.2,
            "owner_age": 66,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.2,
            "owner_age": 66,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.3,
            "owner_age": 66,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.3,
            "owner_age": 66,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.3,
            "owner_age": 66,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.4,
            "owner_age": 66,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.4,
            "owner_age": 66,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.5,
            "owner_age": 66,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.5,
            "owner_age": 66,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.6,
            "owner_age": 66,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.6,
            "owner_age": 66,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.7,
            "owner_age": 66,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.7,
            "owner_age": 66,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.8,
            "owner_age": 66,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.9,
            "owner_age": 66,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.9,
            "owner_age": 66,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.0,
            "owner_age": 66,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.1,
            "owner_age": 66,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.2,
            "owner_age": 66,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.2,
            "owner_age": 66,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.3,
            "owner_age": 66,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.4,
            "owner_age": 66,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.5,
            "owner_age": 66,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.6,
            "owner_age": 66,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.7,
            "owner_age": 66,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.8,
            "owner_age": 66,
            "spouse_age": 45
          },
          {
            "distribution_period": 41.0,
            "owner_age": 66,
            "spouse_age": 46
          },
          {
            "distribution_period": 40.1,
            "owner_age": 66,
            "spouse_age": 47
          },
          {
            "distribution_period": 39.3,
            "owner_age": 66,
            "spouse_age": 48
          },
          {
            "distribution_period": 38.4,
            "owner_age": 66,
            "spouse_age": 49
          },
          {
            "distribution_period": 37.6,
            "owner_age": 66,
            "spouse_age": 50
          },
          {
            "distribution_period": 36.8,
            "owner_age": 66,
            "spouse_age": 51
          },
          {
            "distribution_period": 36.0,
            "owner_age": 66,
            "spouse_age": 52
          },
          {
            "distribution_period": 35.2,
            "owner_age": 66,
            "spouse_age": 53
          },
          {
            "distribution_period": 34.4,
            "owner_age": 66,
            "spouse_age": 54
          },
          {
            "distribution_period": 33.7,
            "owner_age": 66,
            "spouse_age": 55
          },
          {
            "distribution_period": 33.0,
            "owner_age": 66,
            "spouse_age": 56
          },
          {
            "distribution_period": 32.3,
            "owner_age": 66,
            "spouse_age": 57
          },
          {
            "distribution_period": 31.6,
            "owner_age": 66,
            "spouse_age": 58
          },
          {
            "distribution_period": 30.9,
            "owner_age": 66,
            "spouse_age": 59
          },
          {
            "distribution_period": 30.3,
            "owner_age": 66,
            "spouse_age": 60
          },
          {
            "distribution_period": 29.7,
            "owner_age": 66,
            "spouse_age": 61
          },
          {
            "distribution_period": 29.1,
            "owner_age": 66,
            "spouse_age": 62
          },
          {
            "distribution_period": 28.5,
            "owner_age": 66,
            "spouse_age": 63
          },
          {
            "distribution_period": 28.0,
            "owner_age": 66,
            "spouse_age": 64
          },
          {
            "distribution_period": 27.5,
            "owner_age": 66,
            "spouse_age": 65
          },
          {
            "distribution_period": 27.0,
            "owner_age": 66,
            "spouse_age": 66
          },
          {
            "distribution_period": 26.6,
            "owner_age": 66,
            "spouse_age": 67
          },
          {
            "distribution_period": 26.2,
            "owner_age": 66,
            "spouse_age": 68
          },
          {
            "distribution_period": 25.8,
            "owner_age": 66,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.2,
            "owner_age": 67,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.2,
            "owner_age": 67,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.3,
            "owner_age": 67,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.3,
            "owner_age": 67,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.3,
            "owner_age": 67,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.4,
            "owner_age": 67,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.4,
            "owner_age": 67,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.5,
            "owner_age": 67,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.5,
            "owner_age": 67,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.5,
            "owner_age": 67,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.6,
            "owner_age": 67,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.7,
            "owner_age": 67,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.7,
            "owner_age": 67,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.8,
            "owner_age": 67,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.8,
            "owner_age": 67,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.9,
            "owner_age": 67,
            "spouse_age": 35
          },
          {
            "distribution_period": 50.0,
            "owner_age": 67,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.0,
            "owner_age": 67,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.1,
            "owner_age": 67,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.2,
            "owner_age": 67,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.3,
            "owner_age": 67,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.4,
            "owner_age": 67,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.4,
            "owner_age": 67,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.5,
            "owner_age": 67,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.6,
            "owner_age": 67,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.8,
            "owner_age": 67,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.9,
            "owner_age": 67,
            "spouse_age": 46
          },
          {
            "distribution_period": 40.0,
            "owner_age": 67,
            "spouse_age": 47
          },
          {
            "distribution_period": 39.1,
            "owner_age": 67,
            "spouse_age": 48
          },
          {
            "distribution_period": 38.3,
            "owner_age": 67,
            "spouse_age": 49
          },
          {
            "distribution_period": 37.5,
            "owner_age": 67,
            "spouse_age": 50
          },
          {
            "distribution_period": 36.6,
            "owner_age": 67,
            "spouse_age": 51
          },
          {
            "distribution_period": 35.8,
            "owner_age": 67,
            "spouse_age": 52
          },
          {
            "distribution_period": 35.0,
            "owner_age": 67,
            "spouse_age": 53
          },
          {
            "distribution_period": 34.2,
            "owner_age": 67,
            "spouse_age": 54
          },
          {
            "distribution_period": 33.5,
            "owner_age": 67,
            "spouse_age": 55
          },
          {
            "distribution_period": 32.7,
            "owner_age": 67,
            "spouse_age": 56
          },
          {
            "distribution_period": 32.0,
            "owner_age": 67,
            "spouse_age": 57
          },
          {
            "distribution_period": 31.3,
            "owner_age": 67,
            "spouse_age": 58
          },
          {
            "distribution_period": 30.6,
            "owner_age": 67,
            "spouse_age": 59
          },
          {
            "distribution_period": 30.0,
            "owner_age": 67,
            "spouse_age": 60
          },
          {
            "distribution_period": 29.4,
            "owner_age": 67,
            "spouse_age": 61
          },
          {
            "distribution_period": 28.7,
            "owner_age": 67,
            "spouse_age": 62
          },
          {
            "distribution_period": 28.2,
            "owner_age": 67,
            "spouse_age": 63
          },
          {
            "distribution_period": 27.6,
            "owner_age": 67,
            "spouse_age": 64
          },
          {
            "distribution_period": 27.1,
            "owner_age": 67,
            "spouse_age": 65
          },
          {
            "distribution_period": 26.6,
            "owner_age": 67,
            "spouse_age": 66
          },
          {
            "distribution_period": 26.1,
            "owner_age": 67,
            "spouse_age": 67
          },
          {
            "distribution_period": 25.7,
            "owner_age": 67,
            "spouse_age": 68
          },
          {
            "distribution_period": 25.3,
            "owner_age": 67,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.2,
            "owner_age": 68,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.2,
            "owner_age": 68,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.2,
            "owner_age": 68,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.3,
            "owner_age": 68,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.3,
            "owner_age": 68,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.3,
            "owner_age": 68,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.4,
            "owner_age": 68,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.4,
            "owner_age": 68,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.5,
            "owner_age": 68,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.5,
            "owner_age": 68,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.6,
            "owner_age": 68,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.6,
            "owner_age": 68,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.7,
            "owner_age": 68,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.7,
            "owner_age": 68,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.8,
            "owner_age": 68,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.9,
            "owner_age": 68,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.9,
            "owner_age": 68,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.0,
            "owner_age": 68,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.1,
            "owner_age": 68,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.1,
            "owner_age": 68,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.2,
            "owner_age": 68,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.3,
            "owner_age": 68,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.4,
            "owner_age": 68,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.5,
            "owner_age": 68,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.6,
            "owner_age": 68,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.7,
            "owner_age": 68,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.8,
            "owner_age": 68,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.9,
            "owner_age": 68,
            "spouse_age": 47
          },
          {
            "distribution_period": 39.0,
            "owner_age": 68,
            "spouse_age": 48
          },
          {
            "distribution_period": 38.2,
            "owner_age": 68,
            "spouse_age": 49
          },
          {
            "distribution_period": 37.3,
            "owner_age": 68,
            "spouse_age": 50
          },
          {
            "distribution_period": 36.5,
            "owner_age": 68,
            "spouse_age": 51
          },
          {
            "distribution_period": 35.7,
            "owner_age": 68,
            "spouse_age": 52
          },
          {
            "distribution_period": 34.9,
            "owner_age": 68,
            "spouse_age": 53
          },
          {
            "distribution_period": 34.1,
            "owner_age": 68,
            "spouse_age": 54
          },
          {
            "distribution_period": 33.3,
            "owner_age": 68,
            "spouse_age": 55
          },
          {
            "distribution_period": 32.5,
            "owner_age": 68,
            "spouse_age": 56
          },
          {
            "distribution_period": 31.8,
            "owner_age": 68,
            "spouse_age": 57
          },
          {
            "distribution_period": 31.1,
            "owner_age": 68,
            "spouse_age": 58
          },
          {
            "distribution_period": 30.4,
            "owner_age": 68,
            "spouse_age": 59
          },
          {
            "distribution_period": 29.7,
            "owner_age": 68,
            "spouse_age": 60
          },
          {
            "distribution_period": 29.1,
            "owner_age": 68,
            "spouse_age": 61
          },
          {
            "distribution_period": 28.4,
            "owner_age": 68,
            "spouse_age": 62
          },
          {
            "distribution_period": 27.8,
            "owner_age": 68,
            "spouse_age": 63
          },
          {
            "distribution_period": 27.2,
            "owner_age": 68,
            "spouse_age": 64
          },
          {
            "distribution_period": 26.7,
            "owner_age": 68,
            "spouse_age": 65
          },
          {
            "distribution_period": 26.2,
            "owner_age": 68,
            "spouse_age": 66
          },
          {
            "distribution_period": 25.7,
            "owner_age": 68,
            "spouse_age": 67
          },
          {
            "distribution_period": 25.2,
            "owner_age": 68,
            "spouse_age": 68
          },
          {
            "distribution_period": 24.8,
            "owner_age": 68,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.2,
            "owner_age": 69,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.2,
            "owner_age": 69,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.2,
            "owner_age": 69,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.3,
            "owner_age": 69,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.3,
            "owner_age": 69,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.3,
            "owner_age": 69,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.4,
            "owner_age": 69,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.4,
            "owner_age": 69,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.5,
            "owner_age": 69,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.5,
            "owner_age": 69,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.6,
            "owner_age": 69,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.6,
            "owner_age": 69,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.7,
            "owner_age": 69,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.7,
            "owner_age": 69,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.8,
            "owner_age": 69,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.8,
            "owner_age": 69,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.9,
            "owner_age": 69,
            "spouse_age": 36
          },
          {
            "distribution_period": 49.0,
            "owner_age": 69,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.0,
            "owner_age": 69,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.1,
            "owner_age": 69,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.2,
            "owner_age": 69,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.2,
            "owner_age": 69,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.3,
            "owner_age": 69,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.4,
            "owner_age": 69,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.5,
            "owner_age": 69,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.6,
            "owner_age": 69,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.7,
            "owner_age": 69,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.8,
            "owner_age": 69,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.9,
            "owner_age": 69,
            "spouse_age": 48
          },
          {
            "distribution_period": 38.1,
            "owner_age": 69,
            "spouse_age": 49
          },
          {
            "distribution_period": 37.2,
            "owner_age": 69,
            "spouse_age": 50
          },
          {
            "distribution_period": 36.4,
            "owner_age": 69,
            "spouse_age": 51
          },
          {
            "distribution_period": 35.5,
            "owner_age": 69,
            "spouse_age": 52
          },
          {
            "distribution_period": 34.7,
            "owner_age": 69,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.9,
            "owner_age": 69,
            "spouse_age": 54
          },
          {
            "distribution_period": 33.1,
            "owner_age": 69,
            "spouse_age": 55
          },
          {
            "distribution_period": 32.3,
            "owner_age": 69,
            "spouse_age": 56
          },
          {
            "distribution_period": 31.6,
            "owner_age": 69,
            "spouse_age": 57
          },
          {
            "distribution_period": 30.9,
            "owner_age": 69,
            "spouse_age": 58
          },
          {
            "distribution_period": 30.1,
            "owner_age": 69,
            "spouse_age": 59
          },
          {
            "distribution_period": 29.4,
            "owner_age": 69,
            "spouse_age": 60
          },
          {
            "distribution_period": 28.8,
            "owner_age": 69,
            "spouse_age": 61
          },
          {
            "distribution_period": 28.1,
            "owner_age": 69,
            "spouse_age": 62
          },
          {
            "distribution_period": 27.5,
            "owner_age": 69,
            "spouse_age": 63
          },
          {
            "distribution_period": 26.9,
            "owner_age": 69,
            "spouse_age": 64
          },
          {
            "distribution_period": 26.3,
            "owner_age": 69,
            "spouse_age": 65
          },
          {
            "distribution_period": 25.8,
            "owner_age": 69,
            "spouse_age": 66
          },
          {
            "distribution_period": 25.3,
            "owner_age": 69,
            "spouse_age": 67
          },
          {
            "distribution_period": 24.8,
            "owner_age": 69,
            "spouse_age": 68
          },
          {
            "distribution_period": 24.3,
            "owner_age": 69,
            "spouse_age": 69
          },
          {
            "distribution_period": 65.2,
            "owner_age": 70,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.2,
            "owner_age": 70,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.2,
            "owner_age": 70,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 70,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.3,
            "owner_age": 70,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.3,
            "owner_age": 70,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.4,
            "owner_age": 70,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.4,
            "owner_age": 70,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.4,
            "owner_age": 70,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.5,
            "owner_age": 70,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.5,
            "owner_age": 70,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.6,
            "owner_age": 70,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.6,
            "owner_age": 70,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.7,
            "owner_age": 70,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.7,
            "owner_age": 70,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.8,
            "owner_age": 70,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.9,
            "owner_age": 70,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.9,
            "owner_age": 70,
            "spouse_age": 37
          },
          {
            "distribution_period": 48.0,
            "owner_age": 70,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.0,
            "owner_age": 70,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.1,
            "owner_age": 70,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.2,
            "owner_age": 70,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.3,
            "owner_age": 70,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.3,
            "owner_age": 70,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.4,
            "owner_age": 70,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.5,
            "owner_age": 70,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.6,
            "owner_age": 70,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.7,
            "owner_age": 70,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.8,
            "owner_age": 70,
            "spouse_age": 48
          },
          {
            "distribution_period": 38.0,
            "owner_age": 70,
            "spouse_age": 49
          },
          {
            "distribution_period": 37.1,
            "owner_age": 70,
            "spouse_age": 50
          },
          {
            "distribution_period": 36.2,
            "owner_age": 70,
            "spouse_age": 51
          },
          {
            "distribution_period": 35.4,
            "owner_age": 70,
            "spouse_age": 52
          },
          {
            "distribution_period": 34.6,
            "owner_age": 70,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.8,
            "owner_age": 70,
            "spouse_age": 54
          },
          {
            "distribution_period": 33.0,
            "owner_age": 70,
            "spouse_age": 55
          },
          {
            "distribution_period": 32.2,
            "owner_age": 70,
            "spouse_age": 56
          },
          {
            "distribution_period": 31.4,
            "owner_age": 70,
            "spouse_age": 57
          },
          {
            "distribution_period": 30.7,
            "owner_age": 70,
            "spouse_age": 58
          },
          {
            "distribution_period": 29.9,
            "owner_age": 70,
            "spouse_age": 59
          },
          {
            "distribution_period": 29.2,
            "owner_age": 70,
            "spouse_age": 60
          },
          {
            "distribution_period": 28.5,
            "owner_age": 70,
            "spouse_age": 61
          },
          {
            "distribution_period": 27.9,
            "owner_age": 70,
            "spouse_age": 62
          },
          {
            "distribution_period": 27.2,
            "owner_age": 70,
            "spouse_age": 63
          },
          {
            "distribution_period": 26.6,
            "owner_age": 70,
            "spouse_age": 64
          },
          {
            "distribution_period": 26.0,
            "owner_age": 70,
            "spouse_age": 65
          },
          {
            "distribution_period": 25.4,
            "owner_age": 70,
            "spouse_age": 66
          },
          {
            "distribution_period": 24.9,
            "owner_age": 70,
            "spouse_age": 67
          },
          {
            "distribution_period": 24.3,
            "owner_age": 70,
            "spouse_age": 68
          },
          {
            "distribution_period": 23.9,
            "owner_age": 70,
            "spouse_age": 69
          },
          {
            "distribution_period": 23.4,
            "owner_age": 70,
            "spouse_age": 70
          },
          {
            "distribution_period": 22.9,
            "owner_age": 70,
            "spouse_age": 71
          },
          {
            "distribution_period": 22.5,
            "owner_age": 70,
            "spouse_age": 72
          },
          {
            "distribution_period": 22.2,
            "owner_age": 70,
            "spouse_age": 73
          },
          {
            "distribution_period": 21.8,
            "owner_age": 70,
            "spouse_age": 74
          },
          {
            "distribution_period": 21.5,
            "owner_age": 70,
            "spouse_age": 75
          },
          {
            "distribution_period": 21.2,
            "owner_age": 70,
            "spouse_age": 76
          },
          {
            "distribution_period": 20.9,
            "owner_age": 70,
            "spouse_age": 77
          },
          {
            "distribution_period": 20.6,
            "owner_age": 70,
            "spouse_age": 78
          },
          {
            "distribution_period": 20.4,
            "owner_age": 70,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 71,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.2,
            "owner_age": 71,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.2,
            "owner_age": 71,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 71,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.3,
            "owner_age": 71,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.3,
            "owner_age": 71,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.3,
            "owner_age": 71,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.4,
            "owner_age": 71,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.4,
            "owner_age": 71,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.5,
            "owner_age": 71,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.5,
            "owner_age": 71,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.6,
            "owner_age": 71,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.6,
            "owner_age": 71,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.7,
            "owner_age": 71,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.7,
            "owner_age": 71,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.8,
            "owner_age": 71,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.8,
            "owner_age": 71,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.9,
            "owner_age": 71,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.9,
            "owner_age": 71,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.0,
            "owner_age": 71,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.1,
            "owner_age": 71,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.1,
            "owner_age": 71,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.2,
            "owner_age": 71,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.3,
            "owner_age": 71,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.4,
            "owner_age": 71,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.5,
            "owner_age": 71,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.6,
            "owner_age": 71,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.7,
            "owner_age": 71,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.8,
            "owner_age": 71,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.9,
            "owner_age": 71,
            "spouse_age": 49
          },
          {
            "distribution_period": 37.0,
            "owner_age": 71,
            "spouse_age": 50
          },
          {
            "distribution_period": 36.1,
            "owner_age": 71,
            "spouse_age": 51
          },
          {
            "distribution_period": 35.3,
            "owner_age": 71,
            "spouse_age": 52
          },
          {
            "distribution_period": 34.5,
            "owner_age": 71,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.6,
            "owner_age": 71,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.8,
            "owner_age": 71,
            "spouse_age": 55
          },
          {
            "distribution_period": 32.0,
            "owner_age": 71,
            "spouse_age": 56
          },
          {
            "distribution_period": 31.2,
            "owner_age": 71,
            "spouse_age": 57
          },
          {
            "distribution_period": 30.5,
            "owner_age": 71,
            "spouse_age": 58
          },
          {
            "distribution_period": 29.7,
            "owner_age": 71,
            "spouse_age": 59
          },
          {
            "distribution_period": 29.0,
            "owner_age": 71,
            "spouse_age": 60
          },
          {
            "distribution_period": 28.3,
            "owner_age": 71,
            "spouse_age": 61
          },
          {
            "distribution_period": 27.6,
            "owner_age": 71,
            "spouse_age": 62
          },
          {
            "distribution_period": 26.9,
            "owner_age": 71,
            "spouse_age": 63
          },
          {
            "distribution_period": 26.3,
            "owner_age": 71,
            "spouse_age": 64
          },
          {
            "distribution_period": 25.7,
            "owner_age": 71,
            "spouse_age": 65
          },
          {
            "distribution_period": 25.1,
            "owner_age": 71,
            "spouse_age": 66
          },
          {
            "distribution_period": 24.5,
            "owner_age": 71,
            "spouse_age": 67
          },
          {
            "distribution_period": 24.0,
            "owner_age": 71,
            "spouse_age": 68
          },
          {
            "distribution_period": 23.4,
            "owner_age": 71,
            "spouse_age": 69
          },
          {
            "distribution_period": 22.9,
            "owner_age": 71,
            "spouse_age": 70
          },
          {
            "distribution_period": 22.5,
            "owner_age": 71,
            "spouse_age": 71
          },
          {
            "distribution_period": 22.0,
            "owner_age": 71,
            "spouse_age": 72
          },
          {
            "distribution_period": 21.6,
            "owner_age": 71,
            "spouse_age": 73
          },
          {
            "distribution_period": 21.3,
            "owner_age": 71,
            "spouse_age": 74
          },
          {
            "distribution_period": 20.9,
            "owner_age": 71,
            "spouse_age": 75
          },
          {
            "distribution_period": 20.6,
            "owner_age": 71,
            "spouse_age": 76
          },
          {
            "distribution_period": 20.3,
            "owner_age": 71,
            "spouse_age": 77
          },
          {
            "distribution_period": 20.0,
            "owner_age": 71,
            "spouse_age": 78
          },
          {
            "distribution_period": 19.8,
            "owner_age": 71,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 72,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.2,
            "owner_age": 72,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.2,
            "owner_age": 72,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 72,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.3,
            "owner_age": 72,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.3,
            "owner_age": 72,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.3,
            "owner_age": 72,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.4,
            "owner_age": 72,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.4,
            "owner_age": 72,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.5,
            "owner_age": 72,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.5,
            "owner_age": 72,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.5,
            "owner_age": 72,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.6,
            "owner_age": 72,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.6,
            "owner_age": 72,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.7,
            "owner_age": 72,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.8,
            "owner_age": 72,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.8,
            "owner_age": 72,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.9,
            "owner_age": 72,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.9,
            "owner_age": 72,
            "spouse_age": 38
          },
          {
            "distribution_period": 47.0,
            "owner_age": 72,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.0,
            "owner_age": 72,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.1,
            "owner_age": 72,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.2,
            "owner_age": 72,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.2,
            "owner_age": 72,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.3,
            "owner_age": 72,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.4,
            "owner_age": 72,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.5,
            "owner_age": 72,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.6,
            "owner_age": 72,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.7,
            "owner_age": 72,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.8,
            "owner_age": 72,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.9,
            "owner_age": 72,
            "spouse_age": 50
          },
          {
            "distribution_period": 36.0,
            "owner_age": 72,
            "spouse_age": 51
          },
          {
            "distribution_period": 35.2,
            "owner_age": 72,
            "spouse_age": 52
          },
          {
            "distribution_period": 34.3,
            "owner_age": 72,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.5,
            "owner_age": 72,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.7,
            "owner_age": 72,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.9,
            "owner_age": 72,
            "spouse_age": 56
          },
          {
            "distribution_period": 31.1,
            "owner_age": 72,
            "spouse_age": 57
          },
          {
            "distribution_period": 30.3,
            "owner_age": 72,
            "spouse_age": 58
          },
          {
            "distribution_period": 29.5,
            "owner_age": 72,
            "spouse_age": 59
          },
          {
            "distribution_period": 28.8,
            "owner_age": 72,
            "spouse_age": 60
          },
          {
            "distribution_period": 28.1,
            "owner_age": 72,
            "spouse_age": 61
          },
          {
            "distribution_period": 27.4,
            "owner_age": 72,
            "spouse_age": 62
          },
          {
            "distribution_period": 26.7,
            "owner_age": 72,
            "spouse_age": 63
          },
          {
            "distribution_period": 26.0,
            "owner_age": 72,
            "spouse_age": 64
          },
          {
            "distribution_period": 25.4,
            "owner_age": 72,
            "spouse_age": 65
          },
          {
            "distribution_period": 24.8,
            "owner_age": 72,
            "spouse_age": 66
          },
          {
            "distribution_period": 24.2,
            "owner_age": 72,
            "spouse_age": 67
          },
          {
            "distribution_period": 23.6,
            "owner_age": 72,
            "spouse_age": 68
          },
          {
            "distribution_period": 23.1,
            "owner_age": 72,
            "spouse_age": 69
          },
          {
            "distribution_period": 22.5,
            "owner_age": 72,
            "spouse_age": 70
          },
          {
            "distribution_period": 22.0,
            "owner_age": 72,
            "spouse_age": 71
          },
          {
            "distribution_period": 21.6,
            "owner_age": 72,
            "spouse_age": 72
          },
          {
            "distribution_period": 21.1,
            "owner_age": 72,
            "spouse_age": 73
          },
          {
            "distribution_period": 20.7,
            "owner_age": 72,
            "spouse_age": 74
          },
          {
            "distribution_period": 20.4,
            "owner_age": 72,
            "spouse_age": 75
          },
          {
            "distribution_period": 20.0,
            "owner_age": 72,
            "spouse_age": 76
          },
          {
            "distribution_period": 19.7,
            "owner_age": 72,
            "spouse_age": 77
          },
          {
            "distribution_period": 19.4,
            "owner_age": 72,
            "spouse_age": 78
          },
          {
            "distribution_period": 19.2,
            "owner_age": 72,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 73,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.2,
            "owner_age": 73,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.2,
            "owner_age": 73,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 73,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 73,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.3,
            "owner_age": 73,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.3,
            "owner_age": 73,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.4,
            "owner_age": 73,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.4,
            "owner_age": 73,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.4,
            "owner_age": 73,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.5,
            "owner_age": 73,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.5,
            "owner_age": 73,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.6,
            "owner_age": 73,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.6,
            "owner_age": 73,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.7,
            "owner_age": 73,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.7,
            "owner_age": 73,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.8,
            "owner_age": 73,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.8,
            "owner_age": 73,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.9,
            "owner_age": 73,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.9,
            "owner_age": 73,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.0,
            "owner_age": 73,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.1,
            "owner_age": 73,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.1,
            "owner_age": 73,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.2,
            "owner_age": 73,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.3,
            "owner_age": 73,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.4,
            "owner_age": 73,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.4,
            "owner_age": 73,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.5,
            "owner_age": 73,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.6,
            "owner_age": 73,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.7,
            "owner_age": 73,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.8,
            "owner_age": 73,
            "spouse_age": 50
          },
          {
            "distribution_period": 36.0,
            "owner_age": 73,
            "spouse_age": 51
          },
          {
            "distribution_period": 35.1,
            "owner_age": 73,
            "spouse_age": 52
          },
          {
            "distribution_period": 34.2,
            "owner_age": 73,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.4,
            "owner_age": 73,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.6,
            "owner_age": 73,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.7,
            "owner_age": 73,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.9,
            "owner_age": 73,
            "spouse_age": 57
          },
          {
            "distribution_period": 30.1,
            "owner_age": 73,
            "spouse_age": 58
          },
          {
            "distribution_period": 29.4,
            "owner_age": 73,
            "spouse_age": 59
          },
          {
            "distribution_period": 28.6,
            "owner_age": 73,
            "spouse_age": 60
          },
          {
            "distribution_period": 27.9,
            "owner_age": 73,
            "spouse_age": 61
          },
          {
            "distribution_period": 27.2,
            "owner_age": 73,
            "spouse_age": 62
          },
          {
            "distribution_period": 26.5,
            "owner_age": 73,
            "spouse_age": 63
          },
          {
            "distribution_period": 25.8,
            "owner_age": 73,
            "spouse_age": 64
          },
          {
            "distribution_period": 25.1,
            "owner_age": 73,
            "spouse_age": 65
          },
          {
            "distribution_period": 24.5,
            "owner_age": 73,
            "spouse_age": 66
          },
          {
            "distribution_period": 23.9,
            "owner_age": 73,
            "spouse_age": 67
          },
          {
            "distribution_period": 23.3,
            "owner_age": 73,
            "spouse_age": 68
          },
          {
            "distribution_period": 22.7,
            "owner_age": 73,
            "spouse_age": 69
          },
          {
            "distribution_period": 22.2,
            "owner_age": 73,
            "spouse_age": 70
          },
          {
            "distribution_period": 21.6,
            "owner_age": 73,
            "spouse_age": 71
          },
          {
            "distribution_period": 21.1,
            "owner_age": 73,
            "spouse_age": 72
          },
          {
            "distribution_period": 20.7,
            "owner_age": 73,
            "spouse_age": 73
          },
          {
            "distribution_period": 20.3,
            "owner_age": 73,
            "spouse_age": 74
          },
          {
            "distribution_period": 19.9,
            "owner_age": 73,
            "spouse_age": 75
          },
          {
            "distribution_period": 19.5,
            "owner_age": 73,
            "spouse_age": 76
          },
          {
            "distribution_period": 19.1,
            "owner_age": 73,
            "spouse_age": 77
          },
          {
            "distribution_period": 18.8,
            "owner_age": 73,
            "spouse_age": 78
          },
          {
            "distribution_period": 18.6,
            "owner_age": 73,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 74,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 74,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.2,
            "owner_age": 74,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 74,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 74,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.3,
            "owner_age": 74,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.3,
            "owner_age": 74,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 74,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.4,
            "owner_age": 74,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.4,
            "owner_age": 74,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.5,
            "owner_age": 74,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.5,
            "owner_age": 74,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.6,
            "owner_age": 74,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.6,
            "owner_age": 74,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.7,
            "owner_age": 74,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.7,
            "owner_age": 74,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.8,
            "owner_age": 74,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.8,
            "owner_age": 74,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.9,
            "owner_age": 74,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.9,
            "owner_age": 74,
            "spouse_age": 39
          },
          {
            "distribution_period": 46.0,
            "owner_age": 74,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.0,
            "owner_age": 74,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.1,
            "owner_age": 74,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.2,
            "owner_age": 74,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.2,
            "owner_age": 74,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.3,
            "owner_age": 74,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.4,
            "owner_age": 74,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.5,
            "owner_age": 74,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.6,
            "owner_age": 74,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.7,
            "owner_age": 74,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.8,
            "owner_age": 74,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.9,
            "owner_age": 74,
            "spouse_age": 51
          },
          {
            "distribution_period": 35.0,
            "owner_age": 74,
            "spouse_age": 52
          },
          {
            "distribution_period": 34.1,
            "owner_age": 74,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.3,
            "owner_age": 74,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.4,
            "owner_age": 74,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.6,
            "owner_age": 74,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.8,
            "owner_age": 74,
            "spouse_age": 57
          },
          {
            "distribution_period": 30.0,
            "owner_age": 74,
            "spouse_age": 58
          },
          {
            "distribution_period": 29.2,
            "owner_age": 74,
            "spouse_age": 59
          },
          {
            "distribution_period": 28.4,
            "owner_age": 74,
            "spouse_age": 60
          },
          {
            "distribution_period": 27.7,
            "owner_age": 74,
            "spouse_age": 61
          },
          {
            "distribution_period": 27.0,
            "owner_age": 74,
            "spouse_age": 62
          },
          {
            "distribution_period": 26.2,
            "owner_age": 74,
            "spouse_age": 63
          },
          {
            "distribution_period": 25.5,
            "owner_age": 74,
            "spouse_age": 64
          },
          {
            "distribution_period": 24.9,
            "owner_age": 74,
            "spouse_age": 65
          },
          {
            "distribution_period": 24.2,
            "owner_age": 74,
            "spouse_age": 66
          },
          {
            "distribution_period": 23.6,
            "owner_age": 74,
            "spouse_age": 67
          },
          {
            "distribution_period": 23.0,
            "owner_age": 74,
            "spouse_age": 68
          },
          {
            "distribution_period": 22.4,
            "owner_age": 74,
            "spouse_age": 69
          },
          {
            "distribution_period": 21.8,
            "owner_age": 74,
            "spouse_age": 70
          },
          {
            "distribution_period": 21.3,
            "owner_age": 74,
            "spouse_age": 71
          },
          {
            "distribution_period": 20.7,
            "owner_age": 74,
            "spouse_age": 72
          },
          {
            "distribution_period": 20.3,
            "owner_age": 74,
            "spouse_age": 73
          },
          {
            "distribution_period": 19.8,
            "owner_age": 74,
            "spouse_age": 74
          },
          {
            "distribution_period": 19.4,
            "owner_age": 74,
            "spouse_age": 75
          },
          {
            "distribution_period": 19.0,
            "owner_age": 74,
            "spouse_age": 76
          },
          {
            "distribution_period": 18.6,
            "owner_age": 74,
            "spouse_age": 77
          },
          {
            "distribution_period": 18.3,
            "owner_age": 74,
            "spouse_age": 78
          },
          {
            "distribution_period": 18.0,
            "owner_age": 74,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 75,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 75,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.2,
            "owner_age": 75,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 75,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 75,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.3,
            "owner_age": 75,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.3,
            "owner_age": 75,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 75,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.4,
            "owner_age": 75,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.4,
            "owner_age": 75,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.5,
            "owner_age": 75,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.5,
            "owner_age": 75,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 75,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.6,
            "owner_age": 75,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.6,
            "owner_age": 75,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.7,
            "owner_age": 75,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.7,
            "owner_age": 75,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.8,
            "owner_age": 75,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.8,
            "owner_age": 75,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.9,
            "owner_age": 75,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.9,
            "owner_age": 75,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.0,
            "owner_age": 75,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.1,
            "owner_age": 75,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.1,
            "owner_age": 75,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.2,
            "owner_age": 75,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.3,
            "owner_age": 75,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.3,
            "owner_age": 75,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.4,
            "owner_age": 75,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.5,
            "owner_age": 75,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.6,
            "owner_age": 75,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.7,
            "owner_age": 75,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.8,
            "owner_age": 75,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.9,
            "owner_age": 75,
            "spouse_age": 52
          },
          {
            "distribution_period": 34.1,
            "owner_age": 75,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.2,
            "owner_age": 75,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.4,
            "owner_age": 75,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.5,
            "owner_age": 75,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.7,
            "owner_age": 75,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.9,
            "owner_age": 75,
            "spouse_age": 58
          },
          {
            "distribution_period": 29.1,
            "owner_age": 75,
            "spouse_age": 59
          },
          {
            "distribution_period": 28.3,
            "owner_age": 75,
            "spouse_age": 60
          },
          {
            "distribution_period": 27.5,
            "owner_age": 75,
            "spouse_age": 61
          },
          {
            "distribution_period": 26.8,
            "owner_age": 75,
            "spouse_age": 62
          },
          {
            "distribution_period": 26.1,
            "owner_age": 75,
            "spouse_age": 63
          },
          {
            "distribution_period": 25.3,
            "owner_age": 75,
            "spouse_age": 64
          },
          {
            "distribution_period": 24.6,
            "owner_age": 75,
            "spouse_age": 65
          },
          {
            "distribution_period": 24.0,
            "owner_age": 75,
            "spouse_age": 66
          },
          {
            "distribution_period": 23.3,
            "owner_age": 75,
            "spouse_age": 67
          },
          {
            "distribution_period": 22.7,
            "owner_age": 75,
            "spouse_age": 68
          },
          {
            "distribution_period": 22.1,
            "owner_age": 75,
            "spouse_age": 69
          },
          {
            "distribution_period": 21.5,
            "owner_age": 75,
            "spouse_age": 70
          },
          {
            "distribution_period": 20.9,
            "owner_age": 75,
            "spouse_age": 71
          },
          {
            "distribution_period": 20.4,
            "owner_age": 75,
            "spouse_age": 72
          },
          {
            "distribution_period": 19.9,
            "owner_age": 75,
            "spouse_age": 73
          },
          {
            "distribution_period": 19.4,
            "owner_age": 75,
            "spouse_age": 74
          },
          {
            "distribution_period": 18.9,
            "owner_age": 75,
            "spouse_age": 75
          },
          {
            "distribution_period": 18.5,
            "owner_age": 75,
            "spouse_age": 76
          },
          {
            "distribution_period": 18.1,
            "owner_age": 75,
            "spouse_age": 77
          },
          {
            "distribution_period": 17.8,
            "owner_age": 75,
            "spouse_age": 78
          },
          {
            "distribution_period": 17.4,
            "owner_age": 75,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 76,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 76,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.2,
            "owner_age": 76,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 76,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 76,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 76,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.3,
            "owner_age": 76,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 76,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.4,
            "owner_age": 76,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.4,
            "owner_age": 76,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 76,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.5,
            "owner_age": 76,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 76,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.6,
            "owner_age": 76,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.6,
            "owner_age": 76,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.7,
            "owner_age": 76,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.7,
            "owner_age": 76,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.8,
            "owner_age": 76,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.8,
            "owner_age": 76,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.9,
            "owner_age": 76,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.9,
            "owner_age": 76,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.0,
            "owner_age": 76,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.0,
            "owner_age": 76,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.1,
            "owner_age": 76,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.2,
            "owner_age": 76,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.2,
            "owner_age": 76,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.3,
            "owner_age": 76,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.4,
            "owner_age": 76,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.5,
            "owner_age": 76,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.5,
            "owner_age": 76,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.6,
            "owner_age": 76,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.7,
            "owner_age": 76,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.9,
            "owner_age": 76,
            "spouse_age": 52
          },
          {
            "distribution_period": 34.0,
            "owner_age": 76,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.1,
            "owner_age": 76,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.3,
            "owner_age": 76,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.4,
            "owner_age": 76,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.6,
            "owner_age": 76,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.8,
            "owner_age": 76,
            "spouse_age": 58
          },
          {
            "distribution_period": 29.0,
            "owner_age": 76,
            "spouse_age": 59
          },
          {
            "distribution_period": 28.2,
            "owner_age": 76,
            "spouse_age": 60
          },
          {
            "distribution_period": 27.4,
            "owner_age": 76,
            "spouse_age": 61
          },
          {
            "distribution_period": 26.6,
            "owner_age": 76,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.9,
            "owner_age": 76,
            "spouse_age": 63
          },
          {
            "distribution_period": 25.2,
            "owner_age": 76,
            "spouse_age": 64
          },
          {
            "distribution_period": 24.4,
            "owner_age": 76,
            "spouse_age": 65
          },
          {
            "distribution_period": 23.7,
            "owner_age": 76,
            "spouse_age": 66
          },
          {
            "distribution_period": 23.1,
            "owner_age": 76,
            "spouse_age": 67
          },
          {
            "distribution_period": 22.4,
            "owner_age": 76,
            "spouse_age": 68
          },
          {
            "distribution_period": 21.8,
            "owner_age": 76,
            "spouse_age": 69
          },
          {
            "distribution_period": 21.2,
            "owner_age": 76,
            "spouse_age": 70
          },
          {
            "distribution_period": 20.6,
            "owner_age": 76,
            "spouse_age": 71
          },
          {
            "distribution_period": 20.0,
            "owner_age": 76,
            "spouse_age": 72
          },
          {
            "distribution_period": 19.5,
            "owner_age": 76,
            "spouse_age": 73
          },
          {
            "distribution_period": 19.0,
            "owner_age": 76,
            "spouse_age": 74
          },
          {
            "distribution_period": 18.5,
            "owner_age": 76,
            "spouse_age": 75
          },
          {
            "distribution_period": 18.1,
            "owner_age": 76,
            "spouse_age": 76
          },
          {
            "distribution_period": 17.7,
            "owner_age": 76,
            "spouse_age": 77
          },
          {
            "distribution_period": 17.3,
            "owner_age": 76,
            "spouse_age": 78
          },
          {
            "distribution_period": 16.9,
            "owner_age": 76,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 77,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 77,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 77,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 77,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 77,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 77,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.3,
            "owner_age": 77,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 77,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 77,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.4,
            "owner_age": 77,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 77,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.5,
            "owner_age": 77,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 77,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.6,
            "owner_age": 77,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.6,
            "owner_age": 77,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.7,
            "owner_age": 77,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.7,
            "owner_age": 77,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.8,
            "owner_age": 77,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.8,
            "owner_age": 77,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.9,
            "owner_age": 77,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.9,
            "owner_age": 77,
            "spouse_age": 40
          },
          {
            "distribution_period": 45.0,
            "owner_age": 77,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.0,
            "owner_age": 77,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.1,
            "owner_age": 77,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.1,
            "owner_age": 77,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.2,
            "owner_age": 77,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.3,
            "owner_age": 77,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.3,
            "owner_age": 77,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.4,
            "owner_age": 77,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.5,
            "owner_age": 77,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.6,
            "owner_age": 77,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.7,
            "owner_age": 77,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.8,
            "owner_age": 77,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.9,
            "owner_age": 77,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.0,
            "owner_age": 77,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.2,
            "owner_age": 77,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.3,
            "owner_age": 77,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.5,
            "owner_age": 77,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.7,
            "owner_age": 77,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.8,
            "owner_age": 77,
            "spouse_age": 59
          },
          {
            "distribution_period": 28.0,
            "owner_age": 77,
            "spouse_age": 60
          },
          {
            "distribution_period": 27.3,
            "owner_age": 77,
            "spouse_age": 61
          },
          {
            "distribution_period": 26.5,
            "owner_age": 77,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.7,
            "owner_age": 77,
            "spouse_age": 63
          },
          {
            "distribution_period": 25.0,
            "owner_age": 77,
            "spouse_age": 64
          },
          {
            "distribution_period": 24.3,
            "owner_age": 77,
            "spouse_age": 65
          },
          {
            "distribution_period": 23.5,
            "owner_age": 77,
            "spouse_age": 66
          },
          {
            "distribution_period": 22.9,
            "owner_age": 77,
            "spouse_age": 67
          },
          {
            "distribution_period": 22.2,
            "owner_age": 77,
            "spouse_age": 68
          },
          {
            "distribution_period": 21.5,
            "owner_age": 77,
            "spouse_age": 69
          },
          {
            "distribution_period": 20.9,
            "owner_age": 77,
            "spouse_age": 70
          },
          {
            "distribution_period": 20.3,
            "owner_age": 77,
            "spouse_age": 71
          },
          {
            "distribution_period": 19.7,
            "owner_age": 77,
            "spouse_age": 72
          },
          {
            "distribution_period": 19.1,
            "owner_age": 77,
            "spouse_age": 73
          },
          {
            "distribution_period": 18.6,
            "owner_age": 77,
            "spouse_age": 74
          },
          {
            "distribution_period": 18.1,
            "owner_age": 77,
            "spouse_age": 75
          },
          {
            "distribution_period": 17.7,
            "owner_age": 77,
            "spouse_age": 76
          },
          {
            "distribution_period": 17.2,
            "owner_age": 77,
            "spouse_age": 77
          },
          {
            "distribution_period": 16.8,
            "owner_age": 77,
            "spouse_age": 78
          },
          {
            "distribution_period": 16.4,
            "owner_age": 77,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 78,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 78,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 78,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 78,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 78,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 78,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.3,
            "owner_age": 78,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 78,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 78,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.4,
            "owner_age": 78,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 78,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.5,
            "owner_age": 78,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 78,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.6,
            "owner_age": 78,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.6,
            "owner_age": 78,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 78,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.7,
            "owner_age": 78,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 78,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.8,
            "owner_age": 78,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.8,
            "owner_age": 78,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.9,
            "owner_age": 78,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.9,
            "owner_age": 78,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.0,
            "owner_age": 78,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.0,
            "owner_age": 78,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.1,
            "owner_age": 78,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.2,
            "owner_age": 78,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.2,
            "owner_age": 78,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.3,
            "owner_age": 78,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.4,
            "owner_age": 78,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.5,
            "owner_age": 78,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.5,
            "owner_age": 78,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.6,
            "owner_age": 78,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.7,
            "owner_age": 78,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.9,
            "owner_age": 78,
            "spouse_age": 53
          },
          {
            "distribution_period": 33.0,
            "owner_age": 78,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.1,
            "owner_age": 78,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.2,
            "owner_age": 78,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.4,
            "owner_age": 78,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.6,
            "owner_age": 78,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.7,
            "owner_age": 78,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.9,
            "owner_age": 78,
            "spouse_age": 60
          },
          {
            "distribution_period": 27.1,
            "owner_age": 78,
            "spouse_age": 61
          },
          {
            "distribution_period": 26.4,
            "owner_age": 78,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.6,
            "owner_age": 78,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.8,
            "owner_age": 78,
            "spouse_age": 64
          },
          {
            "distribution_period": 24.1,
            "owner_age": 78,
            "spouse_age": 65
          },
          {
            "distribution_period": 23.4,
            "owner_age": 78,
            "spouse_age": 66
          },
          {
            "distribution_period": 22.7,
            "owner_age": 78,
            "spouse_age": 67
          },
          {
            "distribution_period": 22.0,
            "owner_age": 78,
            "spouse_age": 68
          },
          {
            "distribution_period": 21.3,
            "owner_age": 78,
            "spouse_age": 69
          },
          {
            "distribution_period": 20.6,
            "owner_age": 78,
            "spouse_age": 70
          },
          {
            "distribution_period": 20.0,
            "owner_age": 78,
            "spouse_age": 71
          },
          {
            "distribution_period": 19.4,
            "owner_age": 78,
            "spouse_age": 72
          },
          {
            "distribution_period": 18.8,
            "owner_age": 78,
            "spouse_age": 73
          },
          {
            "distribution_period": 18.3,
            "owner_age": 78,
            "spouse_age": 74
          },
          {
            "distribution_period": 17.8,
            "owner_age": 78,
            "spouse_age": 75
          },
          {
            "distribution_period": 17.3,
            "owner_age": 78,
            "spouse_age": 76
          },
          {
            "distribution_period": 16.8,
            "owner_age": 78,
            "spouse_age": 77
          },
          {
            "distribution_period": 16.4,
            "owner_age": 78,
            "spouse_age": 78
          },
          {
            "distribution_period": 16.0,
            "owner_age": 78,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 79,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 79,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 79,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.2,
            "owner_age": 79,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 79,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 79,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.3,
            "owner_age": 79,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 79,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 79,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.4,
            "owner_age": 79,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 79,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.5,
            "owner_age": 79,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 79,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 79,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.6,
            "owner_age": 79,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 79,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.7,
            "owner_age": 79,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 79,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.8,
            "owner_age": 79,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.8,
            "owner_age": 79,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.9,
            "owner_age": 79,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.9,
            "owner_age": 79,
            "spouse_age": 41
          },
          {
            "distribution_period": 44.0,
            "owner_age": 79,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.0,
            "owner_age": 79,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.1,
            "owner_age": 79,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.1,
            "owner_age": 79,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.2,
            "owner_age": 79,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.3,
            "owner_age": 79,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.3,
            "owner_age": 79,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.4,
            "owner_age": 79,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.5,
            "owner_age": 79,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.6,
            "owner_age": 79,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.7,
            "owner_age": 79,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.8,
            "owner_age": 79,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.9,
            "owner_age": 79,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.0,
            "owner_age": 79,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.2,
            "owner_age": 79,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.3,
            "owner_age": 79,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.5,
            "owner_age": 79,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.7,
            "owner_age": 79,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.8,
            "owner_age": 79,
            "spouse_age": 60
          },
          {
            "distribution_period": 27.0,
            "owner_age": 79,
            "spouse_age": 61
          },
          {
            "distribution_period": 26.2,
            "owner_age": 79,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.5,
            "owner_age": 79,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.7,
            "owner_age": 79,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.9,
            "owner_age": 79,
            "spouse_age": 65
          },
          {
            "distribution_period": 23.2,
            "owner_age": 79,
            "spouse_age": 66
          },
          {
            "distribution_period": 22.5,
            "owner_age": 79,
            "spouse_age": 67
          },
          {
            "distribution_period": 21.8,
            "owner_age": 79,
            "spouse_age": 68
          },
          {
            "distribution_period": 21.1,
            "owner_age": 79,
            "spouse_age": 69
          },
          {
            "distribution_period": 20.4,
            "owner_age": 79,
            "spouse_age": 70
          },
          {
            "distribution_period": 19.8,
            "owner_age": 79,
            "spouse_age": 71
          },
          {
            "distribution_period": 19.2,
            "owner_age": 79,
            "spouse_age": 72
          },
          {
            "distribution_period": 18.6,
            "owner_age": 79,
            "spouse_age": 73
          },
          {
            "distribution_period": 18.0,
            "owner_age": 79,
            "spouse_age": 74
          },
          {
            "distribution_period": 17.4,
            "owner_age": 79,
            "spouse_age": 75
          },
          {
            "distribution_period": 16.9,
            "owner_age": 79,
            "spouse_age": 76
          },
          {
            "distribution_period": 16.4,
            "owner_age": 79,
            "spouse_age": 77
          },
          {
            "distribution_period": 16.0,
            "owner_age": 79,
            "spouse_age": 78
          },
          {
            "distribution_period": 15.6,
            "owner_age": 79,
            "spouse_age": 79
          },
          {
            "distribution_period": 65.1,
            "owner_age": 80,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 80,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 80,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 80,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 80,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 80,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 80,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 80,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 80,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.4,
            "owner_age": 80,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 80,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 80,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 80,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 80,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.6,
            "owner_age": 80,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 80,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.7,
            "owner_age": 80,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 80,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.8,
            "owner_age": 80,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.8,
            "owner_age": 80,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.9,
            "owner_age": 80,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.9,
            "owner_age": 80,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 80,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.0,
            "owner_age": 80,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.1,
            "owner_age": 80,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.1,
            "owner_age": 80,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.2,
            "owner_age": 80,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.2,
            "owner_age": 80,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.3,
            "owner_age": 80,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.4,
            "owner_age": 80,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.5,
            "owner_age": 80,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.5,
            "owner_age": 80,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.6,
            "owner_age": 80,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.7,
            "owner_age": 80,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.9,
            "owner_age": 80,
            "spouse_age": 54
          },
          {
            "distribution_period": 32.0,
            "owner_age": 80,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.1,
            "owner_age": 80,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.3,
            "owner_age": 80,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.4,
            "owner_age": 80,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.6,
            "owner_age": 80,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.8,
            "owner_age": 80,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.9,
            "owner_age": 80,
            "spouse_age": 61
          },
          {
            "distribution_period": 26.1,
            "owner_age": 80,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.3,
            "owner_age": 80,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.6,
            "owner_age": 80,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.8,
            "owner_age": 80,
            "spouse_age": 65
          },
          {
            "distribution_period": 23.1,
            "owner_age": 80,
            "spouse_age": 66
          },
          {
            "distribution_period": 22.3,
            "owner_age": 80,
            "spouse_age": 67
          },
          {
            "distribution_period": 21.6,
            "owner_age": 80,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.9,
            "owner_age": 80,
            "spouse_age": 69
          },
          {
            "distribution_period": 20.2,
            "owner_age": 80,
            "spouse_age": 70
          },
          {
            "distribution_period": 19.6,
            "owner_age": 80,
            "spouse_age": 71
          },
          {
            "distribution_period": 18.9,
            "owner_age": 80,
            "spouse_age": 72
          },
          {
            "distribution_period": 18.3,
            "owner_age": 80,
            "spouse_age": 73
          },
          {
            "distribution_period": 17.7,
            "owner_age": 80,
            "spouse_age": 74
          },
          {
            "distribution_period": 17.1,
            "owner_age": 80,
            "spouse_age": 75
          },
          {
            "distribution_period": 16.6,
            "owner_age": 80,
            "spouse_age": 76
          },
          {
            "distribution_period": 16.1,
            "owner_age": 80,
            "spouse_age": 77
          },
          {
            "distribution_period": 15.6,
            "owner_age": 80,
            "spouse_age": 78
          },
          {
            "distribution_period": 15.2,
            "owner_age": 80,
            "spouse_age": 79
          },
          {
            "distribution_period": 14.7,
            "owner_age": 80,
            "spouse_age": 80
          },
          {
            "distribution_period": 14.4,
            "owner_age": 80,
            "spouse_age": 81
          },
          {
            "distribution_period": 14.0,
            "owner_age": 80,
            "spouse_age": 82
          },
          {
            "distribution_period": 13.7,
            "owner_age": 80,
            "spouse_age": 83
          },
          {
            "distribution_period": 13.4,
            "owner_age": 80,
            "spouse_age": 84
          },
          {
            "distribution_period": 13.1,
            "owner_age": 80,
            "spouse_age": 85
          },
          {
            "distribution_period": 12.9,
            "owner_age": 80,
            "spouse_age": 86
          },
          {
            "distribution_period": 12.7,
            "owner_age": 80,
            "spouse_age": 87
          },
          {
            "distribution_period": 12.5,
            "owner_age": 80,
            "spouse_age": 88
          },
          {
            "distribution_period": 12.3,
            "owner_age": 80,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.1,
            "owner_age": 81,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 81,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 81,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 81,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 81,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 81,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 81,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 81,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 81,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.4,
            "owner_age": 81,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 81,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 81,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 81,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 81,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.6,
            "owner_age": 81,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 81,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.7,
            "owner_age": 81,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 81,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 81,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.8,
            "owner_age": 81,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 81,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.9,
            "owner_age": 81,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 81,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.0,
            "owner_age": 81,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.0,
            "owner_age": 81,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.1,
            "owner_age": 81,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.1,
            "owner_age": 81,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.2,
            "owner_age": 81,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.3,
            "owner_age": 81,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.3,
            "owner_age": 81,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.4,
            "owner_age": 81,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.5,
            "owner_age": 81,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.6,
            "owner_age": 81,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.7,
            "owner_age": 81,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.8,
            "owner_age": 81,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.9,
            "owner_age": 81,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.1,
            "owner_age": 81,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.2,
            "owner_age": 81,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.3,
            "owner_age": 81,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.5,
            "owner_age": 81,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.7,
            "owner_age": 81,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.9,
            "owner_age": 81,
            "spouse_age": 61
          },
          {
            "distribution_period": 26.0,
            "owner_age": 81,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.2,
            "owner_age": 81,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.5,
            "owner_age": 81,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.7,
            "owner_age": 81,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.9,
            "owner_age": 81,
            "spouse_age": 66
          },
          {
            "distribution_period": 22.2,
            "owner_age": 81,
            "spouse_age": 67
          },
          {
            "distribution_period": 21.5,
            "owner_age": 81,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.7,
            "owner_age": 81,
            "spouse_age": 69
          },
          {
            "distribution_period": 20.0,
            "owner_age": 81,
            "spouse_age": 70
          },
          {
            "distribution_period": 19.4,
            "owner_age": 81,
            "spouse_age": 71
          },
          {
            "distribution_period": 18.7,
            "owner_age": 81,
            "spouse_age": 72
          },
          {
            "distribution_period": 18.1,
            "owner_age": 81,
            "spouse_age": 73
          },
          {
            "distribution_period": 17.4,
            "owner_age": 81,
            "spouse_age": 74
          },
          {
            "distribution_period": 16.9,
            "owner_age": 81,
            "spouse_age": 75
          },
          {
            "distribution_period": 16.3,
            "owner_age": 81,
            "spouse_age": 76
          },
          {
            "distribution_period": 15.8,
            "owner_age": 81,
            "spouse_age": 77
          },
          {
            "distribution_period": 15.3,
            "owner_age": 81,
            "spouse_age": 78
          },
          {
            "distribution_period": 14.8,
            "owner_age": 81,
            "spouse_age": 79
          },
          {
            "distribution_period": 14.4,
            "owner_age": 81,
            "spouse_age": 80
          },
          {
            "distribution_period": 14.0,
            "owner_age": 81,
            "spouse_age": 81
          },
          {
            "distribution_period": 13.6,
            "owner_age": 81,
            "spouse_age": 82
          },
          {
            "distribution_period": 13.2,
            "owner_age": 81,
            "spouse_age": 83
          },
          {
            "distribution_period": 12.9,
            "owner_age": 81,
            "spouse_age": 84
          },
          {
            "distribution_period": 12.6,
            "owner_age": 81,
            "spouse_age": 85
          },
          {
            "distribution_period": 12.4,
            "owner_age": 81,
            "spouse_age": 86
          },
          {
            "distribution_period": 12.2,
            "owner_age": 81,
            "spouse_age": 87
          },
          {
            "distribution_period": 12.0,
            "owner_age": 81,
            "spouse_age": 88
          },
          {
            "distribution_period": 11.8,
            "owner_age": 81,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.1,
            "owner_age": 82,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 82,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 82,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 82,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 82,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 82,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 82,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 82,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 82,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 82,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 82,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 82,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 82,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 82,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.6,
            "owner_age": 82,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 82,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.7,
            "owner_age": 82,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 82,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 82,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.8,
            "owner_age": 82,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 82,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.9,
            "owner_age": 82,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 82,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.0,
            "owner_age": 82,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.0,
            "owner_age": 82,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.1,
            "owner_age": 82,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.1,
            "owner_age": 82,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.2,
            "owner_age": 82,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.3,
            "owner_age": 82,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.3,
            "owner_age": 82,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.4,
            "owner_age": 82,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.5,
            "owner_age": 82,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.6,
            "owner_age": 82,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.7,
            "owner_age": 82,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.8,
            "owner_age": 82,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.9,
            "owner_age": 82,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.0,
            "owner_age": 82,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.1,
            "owner_age": 82,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.3,
            "owner_age": 82,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.4,
            "owner_age": 82,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.6,
            "owner_age": 82,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.8,
            "owner_age": 82,
            "spouse_age": 61
          },
          {
            "distribution_period": 26.0,
            "owner_age": 82,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.2,
            "owner_age": 82,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.4,
            "owner_age": 82,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.6,
            "owner_age": 82,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.8,
            "owner_age": 82,
            "spouse_age": 66
          },
          {
            "distribution_period": 22.1,
            "owner_age": 82,
            "spouse_age": 67
          },
          {
            "distribution_period": 21.3,
            "owner_age": 82,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.6,
            "owner_age": 82,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.9,
            "owner_age": 82,
            "spouse_age": 70
          },
          {
            "distribution_period": 19.2,
            "owner_age": 82,
            "spouse_age": 71
          },
          {
            "distribution_period": 18.5,
            "owner_age": 82,
            "spouse_age": 72
          },
          {
            "distribution_period": 17.9,
            "owner_age": 82,
            "spouse_age": 73
          },
          {
            "distribution_period": 17.2,
            "owner_age": 82,
            "spouse_age": 74
          },
          {
            "distribution_period": 16.6,
            "owner_age": 82,
            "spouse_age": 75
          },
          {
            "distribution_period": 16.0,
            "owner_age": 82,
            "spouse_age": 76
          },
          {
            "distribution_period": 15.5,
            "owner_age": 82,
            "spouse_age": 77
          },
          {
            "distribution_period": 15.0,
            "owner_age": 82,
            "spouse_age": 78
          },
          {
            "distribution_period": 14.5,
            "owner_age": 82,
            "spouse_age": 79
          },
          {
            "distribution_period": 14.0,
            "owner_age": 82,
            "spouse_age": 80
          },
          {
            "distribution_period": 13.6,
            "owner_age": 82,
            "spouse_age": 81
          },
          {
            "distribution_period": 13.2,
            "owner_age": 82,
            "spouse_age": 82
          },
          {
            "distribution_period": 12.8,
            "owner_age": 82,
            "spouse_age": 83
          },
          {
            "distribution_period": 12.5,
            "owner_age": 82,
            "spouse_age": 84
          },
          {
            "distribution_period": 12.2,
            "owner_age": 82,
            "spouse_age": 85
          },
          {
            "distribution_period": 11.9,
            "owner_age": 82,
            "spouse_age": 86
          },
          {
            "distribution_period": 11.7,
            "owner_age": 82,
            "spouse_age": 87
          },
          {
            "distribution_period": 11.5,
            "owner_age": 82,
            "spouse_age": 88
          },
          {
            "distribution_period": 11.3,
            "owner_age": 82,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.1,
            "owner_age": 83,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 83,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 83,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 83,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 83,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 83,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 83,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 83,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 83,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 83,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 83,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 83,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 83,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 83,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.6,
            "owner_age": 83,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 83,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 83,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 83,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 83,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.8,
            "owner_age": 83,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 83,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.9,
            "owner_age": 83,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 83,
            "spouse_age": 42
          },
          {
            "distribution_period": 43.0,
            "owner_age": 83,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.0,
            "owner_age": 83,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.1,
            "owner_age": 83,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.1,
            "owner_age": 83,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.2,
            "owner_age": 83,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.2,
            "owner_age": 83,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.3,
            "owner_age": 83,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.4,
            "owner_age": 83,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.4,
            "owner_age": 83,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.5,
            "owner_age": 83,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.6,
            "owner_age": 83,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.7,
            "owner_age": 83,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.8,
            "owner_age": 83,
            "spouse_age": 55
          },
          {
            "distribution_period": 31.0,
            "owner_age": 83,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.1,
            "owner_age": 83,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.2,
            "owner_age": 83,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.4,
            "owner_age": 83,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.5,
            "owner_age": 83,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.7,
            "owner_age": 83,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.9,
            "owner_age": 83,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.1,
            "owner_age": 83,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.3,
            "owner_age": 83,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.5,
            "owner_age": 83,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.7,
            "owner_age": 83,
            "spouse_age": 66
          },
          {
            "distribution_period": 22.0,
            "owner_age": 83,
            "spouse_age": 67
          },
          {
            "distribution_period": 21.2,
            "owner_age": 83,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.5,
            "owner_age": 83,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.7,
            "owner_age": 83,
            "spouse_age": 70
          },
          {
            "distribution_period": 19.0,
            "owner_age": 83,
            "spouse_age": 71
          },
          {
            "distribution_period": 18.3,
            "owner_age": 83,
            "spouse_age": 72
          },
          {
            "distribution_period": 17.7,
            "owner_age": 83,
            "spouse_age": 73
          },
          {
            "distribution_period": 17.0,
            "owner_age": 83,
            "spouse_age": 74
          },
          {
            "distribution_period": 16.4,
            "owner_age": 83,
            "spouse_age": 75
          },
          {
            "distribution_period": 15.8,
            "owner_age": 83,
            "spouse_age": 76
          },
          {
            "distribution_period": 15.2,
            "owner_age": 83,
            "spouse_age": 77
          },
          {
            "distribution_period": 14.7,
            "owner_age": 83,
            "spouse_age": 78
          },
          {
            "distribution_period": 14.2,
            "owner_age": 83,
            "spouse_age": 79
          },
          {
            "distribution_period": 13.7,
            "owner_age": 83,
            "spouse_age": 80
          },
          {
            "distribution_period": 13.2,
            "owner_age": 83,
            "spouse_age": 81
          },
          {
            "distribution_period": 12.8,
            "owner_age": 83,
            "spouse_age": 82
          },
          {
            "distribution_period": 12.4,
            "owner_age": 83,
            "spouse_age": 83
          },
          {
            "distribution_period": 12.1,
            "owner_age": 83,
            "spouse_age": 84
          },
          {
            "distribution_period": 11.8,
            "owner_age": 83,
            "spouse_age": 85
          },
          {
            "distribution_period": 11.5,
            "owner_age": 83,
            "spouse_age": 86
          },
          {
            "distribution_period": 11.2,
            "owner_age": 83,
            "spouse_age": 87
          },
          {
            "distribution_period": 11.0,
            "owner_age": 83,
            "spouse_age": 88
          },
          {
            "distribution_period": 10.8,
            "owner_age": 83,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.1,
            "owner_age": 84,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 84,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 84,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 84,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 84,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 84,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 84,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 84,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 84,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 84,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 84,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 84,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 84,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 84,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 84,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 84,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 84,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 84,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 84,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.8,
            "owner_age": 84,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 84,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.9,
            "owner_age": 84,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 84,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 84,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.0,
            "owner_age": 84,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 84,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.1,
            "owner_age": 84,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.2,
            "owner_age": 84,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.2,
            "owner_age": 84,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.3,
            "owner_age": 84,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.3,
            "owner_age": 84,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.4,
            "owner_age": 84,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.5,
            "owner_age": 84,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.6,
            "owner_age": 84,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.7,
            "owner_age": 84,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.8,
            "owner_age": 84,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.9,
            "owner_age": 84,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.0,
            "owner_age": 84,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.2,
            "owner_age": 84,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.3,
            "owner_age": 84,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.5,
            "owner_age": 84,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.7,
            "owner_age": 84,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.8,
            "owner_age": 84,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.0,
            "owner_age": 84,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.2,
            "owner_age": 84,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.4,
            "owner_age": 84,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.6,
            "owner_age": 84,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.9,
            "owner_age": 84,
            "spouse_age": 67
          },
          {
            "distribution_period": 21.1,
            "owner_age": 84,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.4,
            "owner_age": 84,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.6,
            "owner_age": 84,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.9,
            "owner_age": 84,
            "spouse_age": 71
          },
          {
            "distribution_period": 18.2,
            "owner_age": 84,
            "spouse_age": 72
          },
          {
            "distribution_period": 17.5,
            "owner_age": 84,
            "spouse_age": 73
          },
          {
            "distribution_period": 16.8,
            "owner_age": 84,
            "spouse_age": 74
          },
          {
            "distribution_period": 16.2,
            "owner_age": 84,
            "spouse_age": 75
          },
          {
            "distribution_period": 15.6,
            "owner_age": 84,
            "spouse_age": 76
          },
          {
            "distribution_period": 15.0,
            "owner_age": 84,
            "spouse_age": 77
          },
          {
            "distribution_period": 14.4,
            "owner_age": 84,
            "spouse_age": 78
          },
          {
            "distribution_period": 13.9,
            "owner_age": 84,
            "spouse_age": 79
          },
          {
            "distribution_period": 13.4,
            "owner_age": 84,
            "spouse_age": 80
          },
          {
            "distribution_period": 12.9,
            "owner_age": 84,
            "spouse_age": 81
          },
          {
            "distribution_period": 12.5,
            "owner_age": 84,
            "spouse_age": 82
          },
          {
            "distribution_period": 12.1,
            "owner_age": 84,
            "spouse_age": 83
          },
          {
            "distribution_period": 11.7,
            "owner_age": 84,
            "spouse_age": 84
          },
          {
            "distribution_period": 11.4,
            "owner_age": 84,
            "spouse_age": 85
          },
          {
            "distribution_period": 11.1,
            "owner_age": 84,
            "spouse_age": 86
          },
          {
            "distribution_period": 10.8,
            "owner_age": 84,
            "spouse_age": 87
          },
          {
            "distribution_period": 10.5,
            "owner_age": 84,
            "spouse_age": 88
          },
          {
            "distribution_period": 10.3,
            "owner_age": 84,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.1,
            "owner_age": 85,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 85,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 85,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 85,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.2,
            "owner_age": 85,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 85,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 85,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.3,
            "owner_age": 85,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 85,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 85,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 85,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 85,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 85,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 85,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 85,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 85,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 85,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 85,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 85,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.8,
            "owner_age": 85,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 85,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 85,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 85,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 85,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.0,
            "owner_age": 85,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 85,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.1,
            "owner_age": 85,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 85,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.2,
            "owner_age": 85,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.3,
            "owner_age": 85,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.3,
            "owner_age": 85,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.4,
            "owner_age": 85,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.5,
            "owner_age": 85,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.6,
            "owner_age": 85,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.7,
            "owner_age": 85,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.8,
            "owner_age": 85,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.9,
            "owner_age": 85,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.0,
            "owner_age": 85,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.1,
            "owner_age": 85,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.3,
            "owner_age": 85,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.4,
            "owner_age": 85,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.6,
            "owner_age": 85,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.8,
            "owner_age": 85,
            "spouse_age": 62
          },
          {
            "distribution_period": 25.0,
            "owner_age": 85,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.1,
            "owner_age": 85,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.3,
            "owner_age": 85,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.6,
            "owner_age": 85,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.8,
            "owner_age": 85,
            "spouse_age": 67
          },
          {
            "distribution_period": 21.0,
            "owner_age": 85,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.3,
            "owner_age": 85,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.5,
            "owner_age": 85,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.8,
            "owner_age": 85,
            "spouse_age": 71
          },
          {
            "distribution_period": 18.1,
            "owner_age": 85,
            "spouse_age": 72
          },
          {
            "distribution_period": 17.4,
            "owner_age": 85,
            "spouse_age": 73
          },
          {
            "distribution_period": 16.7,
            "owner_age": 85,
            "spouse_age": 74
          },
          {
            "distribution_period": 16.0,
            "owner_age": 85,
            "spouse_age": 75
          },
          {
            "distribution_period": 15.4,
            "owner_age": 85,
            "spouse_age": 76
          },
          {
            "distribution_period": 14.8,
            "owner_age": 85,
            "spouse_age": 77
          },
          {
            "distribution_period": 14.2,
            "owner_age": 85,
            "spouse_age": 78
          },
          {
            "distribution_period": 13.6,
            "owner_age": 85,
            "spouse_age": 79
          },
          {
            "distribution_period": 13.1,
            "owner_age": 85,
            "spouse_age": 80
          },
          {
            "distribution_period": 12.6,
            "owner_age": 85,
            "spouse_age": 81
          },
          {
            "distribution_period": 12.2,
            "owner_age": 85,
            "spouse_age": 82
          },
          {
            "distribution_period": 11.8,
            "owner_age": 85,
            "spouse_age": 83
          },
          {
            "distribution_period": 11.4,
            "owner_age": 85,
            "spouse_age": 84
          },
          {
            "distribution_period": 11.0,
            "owner_age": 85,
            "spouse_age": 85
          },
          {
            "distribution_period": 10.7,
            "owner_age": 85,
            "spouse_age": 86
          },
          {
            "distribution_period": 10.4,
            "owner_age": 85,
            "spouse_age": 87
          },
          {
            "distribution_period": 10.1,
            "owner_age": 85,
            "spouse_age": 88
          },
          {
            "distribution_period": 9.9,
            "owner_age": 85,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.1,
            "owner_age": 86,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 86,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 86,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 86,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 86,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 86,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 86,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 86,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 86,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 86,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 86,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 86,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.5,
            "owner_age": 86,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 86,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 86,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 86,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 86,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 86,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 86,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 86,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 86,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 86,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 86,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 86,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.0,
            "owner_age": 86,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 86,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.1,
            "owner_age": 86,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 86,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.2,
            "owner_age": 86,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 86,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.3,
            "owner_age": 86,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.4,
            "owner_age": 86,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.5,
            "owner_age": 86,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.5,
            "owner_age": 86,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.6,
            "owner_age": 86,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.7,
            "owner_age": 86,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.9,
            "owner_age": 86,
            "spouse_age": 56
          },
          {
            "distribution_period": 30.0,
            "owner_age": 86,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.1,
            "owner_age": 86,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.2,
            "owner_age": 86,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.4,
            "owner_age": 86,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.6,
            "owner_age": 86,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.7,
            "owner_age": 86,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.9,
            "owner_age": 86,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.1,
            "owner_age": 86,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.3,
            "owner_age": 86,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.5,
            "owner_age": 86,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.7,
            "owner_age": 86,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.9,
            "owner_age": 86,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.2,
            "owner_age": 86,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.4,
            "owner_age": 86,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.7,
            "owner_age": 86,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.9,
            "owner_age": 86,
            "spouse_age": 72
          },
          {
            "distribution_period": 17.2,
            "owner_age": 86,
            "spouse_age": 73
          },
          {
            "distribution_period": 16.5,
            "owner_age": 86,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.9,
            "owner_age": 86,
            "spouse_age": 75
          },
          {
            "distribution_period": 15.2,
            "owner_age": 86,
            "spouse_age": 76
          },
          {
            "distribution_period": 14.6,
            "owner_age": 86,
            "spouse_age": 77
          },
          {
            "distribution_period": 14.0,
            "owner_age": 86,
            "spouse_age": 78
          },
          {
            "distribution_period": 13.4,
            "owner_age": 86,
            "spouse_age": 79
          },
          {
            "distribution_period": 12.9,
            "owner_age": 86,
            "spouse_age": 80
          },
          {
            "distribution_period": 12.4,
            "owner_age": 86,
            "spouse_age": 81
          },
          {
            "distribution_period": 11.9,
            "owner_age": 86,
            "spouse_age": 82
          },
          {
            "distribution_period": 11.5,
            "owner_age": 86,
            "spouse_age": 83
          },
          {
            "distribution_period": 11.1,
            "owner_age": 86,
            "spouse_age": 84
          },
          {
            "distribution_period": 10.7,
            "owner_age": 86,
            "spouse_age": 85
          },
          {
            "distribution_period": 10.4,
            "owner_age": 86,
            "spouse_age": 86
          },
          {
            "distribution_period": 10.0,
            "owner_age": 86,
            "spouse_age": 87
          },
          {
            "distribution_period": 9.8,
            "owner_age": 86,
            "spouse_age": 88
          },
          {
            "distribution_period": 9.5,
            "owner_age": 86,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.0,
            "owner_age": 87,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 87,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 87,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 87,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 87,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 87,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 87,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 87,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 87,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 87,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 87,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 87,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 87,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 87,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 87,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 87,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 87,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 87,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 87,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 87,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 87,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 87,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 87,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 87,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.0,
            "owner_age": 87,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 87,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.1,
            "owner_age": 87,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 87,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.2,
            "owner_age": 87,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 87,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.3,
            "owner_age": 87,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.4,
            "owner_age": 87,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.4,
            "owner_age": 87,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.5,
            "owner_age": 87,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.6,
            "owner_age": 87,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.7,
            "owner_age": 87,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.8,
            "owner_age": 87,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.9,
            "owner_age": 87,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.1,
            "owner_age": 87,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.2,
            "owner_age": 87,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.4,
            "owner_age": 87,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.5,
            "owner_age": 87,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.7,
            "owner_age": 87,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.9,
            "owner_age": 87,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.0,
            "owner_age": 87,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.2,
            "owner_age": 87,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.4,
            "owner_age": 87,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.6,
            "owner_age": 87,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.9,
            "owner_age": 87,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.1,
            "owner_age": 87,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.3,
            "owner_age": 87,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.6,
            "owner_age": 87,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.8,
            "owner_age": 87,
            "spouse_age": 72
          },
          {
            "distribution_period": 17.1,
            "owner_age": 87,
            "spouse_age": 73
          },
          {
            "distribution_period": 16.4,
            "owner_age": 87,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.7,
            "owner_age": 87,
            "spouse_age": 75
          },
          {
            "distribution_period": 15.1,
            "owner_age": 87,
            "spouse_age": 76
          },
          {
            "distribution_period": 14.4,
            "owner_age": 87,
            "spouse_age": 77
          },
          {
            "distribution_period": 13.8,
            "owner_age": 87,
            "spouse_age": 78
          },
          {
            "distribution_period": 13.2,
            "owner_age": 87,
            "spouse_age": 79
          },
          {
            "distribution_period": 12.7,
            "owner_age": 87,
            "spouse_age": 80
          },
          {
            "distribution_period": 12.2,
            "owner_age": 87,
            "spouse_age": 81
          },
          {
            "distribution_period": 11.7,
            "owner_age": 87,
            "spouse_age": 82
          },
          {
            "distribution_period": 11.2,
            "owner_age": 87,
            "spouse_age": 83
          },
          {
            "distribution_period": 10.8,
            "owner_age": 87,
            "spouse_age": 84
          },
          {
            "distribution_period": 10.4,
            "owner_age": 87,
            "spouse_age": 85
          },
          {
            "distribution_period": 10.0,
            "owner_age": 87,
            "spouse_age": 86
          },
          {
            "distribution_period": 9.7,
            "owner_age": 87,
            "spouse_age": 87
          },
          {
            "distribution_period": 9.4,
            "owner_age": 87,
            "spouse_age": 88
          },
          {
            "distribution_period": 9.1,
            "owner_age": 87,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.0,
            "owner_age": 88,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 88,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 88,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 88,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 88,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 88,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 88,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 88,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 88,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 88,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 88,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 88,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 88,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 88,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 88,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 88,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 88,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 88,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 88,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 88,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 88,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 88,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 88,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 88,
            "spouse_age": 43
          },
          {
            "distribution_period": 42.0,
            "owner_age": 88,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 88,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 88,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 88,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.2,
            "owner_age": 88,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 88,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.3,
            "owner_age": 88,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 88,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.4,
            "owner_age": 88,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.5,
            "owner_age": 88,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.6,
            "owner_age": 88,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.7,
            "owner_age": 88,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.8,
            "owner_age": 88,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.9,
            "owner_age": 88,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.0,
            "owner_age": 88,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.2,
            "owner_age": 88,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.3,
            "owner_age": 88,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.5,
            "owner_age": 88,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.6,
            "owner_age": 88,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.8,
            "owner_age": 88,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.0,
            "owner_age": 88,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.2,
            "owner_age": 88,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.4,
            "owner_age": 88,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.6,
            "owner_age": 88,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.8,
            "owner_age": 88,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.0,
            "owner_age": 88,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.2,
            "owner_age": 88,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.5,
            "owner_age": 88,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.7,
            "owner_age": 88,
            "spouse_age": 72
          },
          {
            "distribution_period": 17.0,
            "owner_age": 88,
            "spouse_age": 73
          },
          {
            "distribution_period": 16.3,
            "owner_age": 88,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.6,
            "owner_age": 88,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.9,
            "owner_age": 88,
            "spouse_age": 76
          },
          {
            "distribution_period": 14.3,
            "owner_age": 88,
            "spouse_age": 77
          },
          {
            "distribution_period": 13.7,
            "owner_age": 88,
            "spouse_age": 78
          },
          {
            "distribution_period": 13.1,
            "owner_age": 88,
            "spouse_age": 79
          },
          {
            "distribution_period": 12.5,
            "owner_age": 88,
            "spouse_age": 80
          },
          {
            "distribution_period": 12.0,
            "owner_age": 88,
            "spouse_age": 81
          },
          {
            "distribution_period": 11.5,
            "owner_age": 88,
            "spouse_age": 82
          },
          {
            "distribution_period": 11.0,
            "owner_age": 88,
            "spouse_age": 83
          },
          {
            "distribution_period": 10.5,
            "owner_age": 88,
            "spouse_age": 84
          },
          {
            "distribution_period": 10.1,
            "owner_age": 88,
            "spouse_age": 85
          },
          {
            "distribution_period": 9.8,
            "owner_age": 88,
            "spouse_age": 86
          },
          {
            "distribution_period": 9.4,
            "owner_age": 88,
            "spouse_age": 87
          },
          {
            "distribution_period": 9.1,
            "owner_age": 88,
            "spouse_age": 88
          },
          {
            "distribution_period": 8.8,
            "owner_age": 88,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.0,
            "owner_age": 89,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 89,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 89,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 89,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 89,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 89,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 89,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 89,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 89,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 89,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 89,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 89,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 89,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 89,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 89,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 89,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 89,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.7,
            "owner_age": 89,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 89,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 89,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 89,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 89,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 89,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 89,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 89,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 89,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 89,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 89,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 89,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 89,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.3,
            "owner_age": 89,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 89,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.4,
            "owner_age": 89,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.5,
            "owner_age": 89,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.6,
            "owner_age": 89,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.7,
            "owner_age": 89,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.8,
            "owner_age": 89,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.9,
            "owner_age": 89,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.0,
            "owner_age": 89,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.2,
            "owner_age": 89,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.3,
            "owner_age": 89,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.4,
            "owner_age": 89,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.6,
            "owner_age": 89,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.8,
            "owner_age": 89,
            "spouse_age": 63
          },
          {
            "distribution_period": 24.0,
            "owner_age": 89,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.1,
            "owner_age": 89,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.3,
            "owner_age": 89,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.5,
            "owner_age": 89,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.7,
            "owner_age": 89,
            "spouse_age": 68
          },
          {
            "distribution_period": 20.0,
            "owner_age": 89,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.2,
            "owner_age": 89,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.4,
            "owner_age": 89,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.7,
            "owner_age": 89,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.9,
            "owner_age": 89,
            "spouse_age": 73
          },
          {
            "distribution_period": 16.2,
            "owner_age": 89,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.5,
            "owner_age": 89,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.8,
            "owner_age": 89,
            "spouse_age": 76
          },
          {
            "distribution_period": 14.2,
            "owner_age": 89,
            "spouse_age": 77
          },
          {
            "distribution_period": 13.5,
            "owner_age": 89,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.9,
            "owner_age": 89,
            "spouse_age": 79
          },
          {
            "distribution_period": 12.3,
            "owner_age": 89,
            "spouse_age": 80
          },
          {
            "distribution_period": 11.8,
            "owner_age": 89,
            "spouse_age": 81
          },
          {
            "distribution_period": 11.3,
            "owner_age": 89,
            "spouse_age": 82
          },
          {
            "distribution_period": 10.8,
            "owner_age": 89,
            "spouse_age": 83
          },
          {
            "distribution_period": 10.3,
            "owner_age": 89,
            "spouse_age": 84
          },
          {
            "distribution_period": 9.9,
            "owner_age": 89,
            "spouse_age": 85
          },
          {
            "distribution_period": 9.5,
            "owner_age": 89,
            "spouse_age": 86
          },
          {
            "distribution_period": 9.1,
            "owner_age": 89,
            "spouse_age": 87
          },
          {
            "distribution_period": 8.8,
            "owner_age": 89,
            "spouse_age": 88
          },
          {
            "distribution_period": 8.5,
            "owner_age": 89,
            "spouse_age": 89
          },
          {
            "distribution_period": 65.0,
            "owner_age": 90,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 90,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 90,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 90,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 90,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 90,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 90,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 90,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 90,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 90,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.4,
            "owner_age": 90,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 90,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 90,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 90,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 90,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 90,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 90,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 90,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 90,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 90,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 90,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 90,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 90,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 90,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 90,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 90,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 90,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 90,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 90,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 90,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.3,
            "owner_age": 90,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 90,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.4,
            "owner_age": 90,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.5,
            "owner_age": 90,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.6,
            "owner_age": 90,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.7,
            "owner_age": 90,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.8,
            "owner_age": 90,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.9,
            "owner_age": 90,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.0,
            "owner_age": 90,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.1,
            "owner_age": 90,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.3,
            "owner_age": 90,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.4,
            "owner_age": 90,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.6,
            "owner_age": 90,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.7,
            "owner_age": 90,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.9,
            "owner_age": 90,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.1,
            "owner_age": 90,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.3,
            "owner_age": 90,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.5,
            "owner_age": 90,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.7,
            "owner_age": 90,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.9,
            "owner_age": 90,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.1,
            "owner_age": 90,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.4,
            "owner_age": 90,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.6,
            "owner_age": 90,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.9,
            "owner_age": 90,
            "spouse_age": 73
          },
          {
            "distribution_period": 16.1,
            "owner_age": 90,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.4,
            "owner_age": 90,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.8,
            "owner_age": 90,
            "spouse_age": 76
          },
          {
            "distribution_period": 14.1,
            "owner_age": 90,
            "spouse_age": 77
          },
          {
            "distribution_period": 13.4,
            "owner_age": 90,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.8,
            "owner_age": 90,
            "spouse_age": 79
          },
          {
            "distribution_period": 12.2,
            "owner_age": 90,
            "spouse_age": 80
          },
          {
            "distribution_period": 11.6,
            "owner_age": 90,
            "spouse_age": 81
          },
          {
            "distribution_period": 11.1,
            "owner_age": 90,
            "spouse_age": 82
          },
          {
            "distribution_period": 10.6,
            "owner_age": 90,
            "spouse_age": 83
          },
          {
            "distribution_period": 10.1,
            "owner_age": 90,
            "spouse_age": 84
          },
          {
            "distribution_period": 9.7,
            "owner_age": 90,
            "spouse_age": 85
          },
          {
            "distribution_period": 9.3,
            "owner_age": 90,
            "spouse_age": 86
          },
          {
            "distribution_period": 8.9,
            "owner_age": 90,
            "spouse_age": 87
          },
          {
            "distribution_period": 8.6,
            "owner_age": 90,
            "spouse_age": 88
          },
          {
            "distribution_period": 8.3,
            "owner_age": 90,
            "spouse_age": 89
          },
          {
            "distribution_period": 8.0,
            "owner_age": 90,
            "spouse_age": 90
          },
          {
            "distribution_period": 7.7,
            "owner_age": 90,
            "spouse_age": 91
          },
          {
            "distribution_period": 7.5,
            "owner_age": 90,
            "spouse_age": 92
          },
          {
            "distribution_period": 7.3,
            "owner_age": 90,
            "spouse_age": 93
          },
          {
            "distribution_period": 7.1,
            "owner_age": 90,
            "spouse_age": 94
          },
          {
            "distribution_period": 6.9,
            "owner_age": 90,
            "spouse_age": 95
          },
          {
            "distribution_period": 6.8,
            "owner_age": 90,
            "spouse_age": 96
          },
          {
            "distribution_period": 6.7,
            "owner_age": 90,
            "spouse_age": 97
          },
          {
            "distribution_period": 6.6,
            "owner_age": 90,
            "spouse_age": 98
          },
          {
            "distribution_period": 6.5,
            "owner_age": 90,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 91,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 91,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 91,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 91,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 91,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 91,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 91,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 91,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 91,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 91,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 91,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 91,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 91,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 91,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 91,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 91,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 91,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 91,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 91,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 91,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 91,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 91,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.9,
            "owner_age": 91,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 91,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 91,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 91,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 91,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 91,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 91,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 91,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 91,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 91,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.4,
            "owner_age": 91,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.5,
            "owner_age": 91,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 91,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 91,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 91,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.9,
            "owner_age": 91,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.0,
            "owner_age": 91,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.1,
            "owner_age": 91,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.3,
            "owner_age": 91,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.4,
            "owner_age": 91,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.6,
            "owner_age": 91,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.7,
            "owner_age": 91,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.9,
            "owner_age": 91,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.1,
            "owner_age": 91,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.3,
            "owner_age": 91,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.5,
            "owner_age": 91,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.7,
            "owner_age": 91,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.9,
            "owner_age": 91,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.1,
            "owner_age": 91,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.3,
            "owner_age": 91,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.5,
            "owner_age": 91,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.8,
            "owner_age": 91,
            "spouse_age": 73
          },
          {
            "distribution_period": 16.1,
            "owner_age": 91,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.3,
            "owner_age": 91,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.6,
            "owner_age": 91,
            "spouse_age": 76
          },
          {
            "distribution_period": 14.0,
            "owner_age": 91,
            "spouse_age": 77
          },
          {
            "distribution_period": 13.3,
            "owner_age": 91,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.7,
            "owner_age": 91,
            "spouse_age": 79
          },
          {
            "distribution_period": 12.1,
            "owner_age": 91,
            "spouse_age": 80
          },
          {
            "distribution_period": 11.5,
            "owner_age": 91,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.9,
            "owner_age": 91,
            "spouse_age": 82
          },
          {
            "distribution_period": 10.4,
            "owner_age": 91,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.9,
            "owner_age": 91,
            "spouse_age": 84
          },
          {
            "distribution_period": 9.5,
            "owner_age": 91,
            "spouse_age": 85
          },
          {
            "distribution_period": 9.1,
            "owner_age": 91,
            "spouse_age": 86
          },
          {
            "distribution_period": 8.7,
            "owner_age": 91,
            "spouse_age": 87
          },
          {
            "distribution_period": 8.3,
            "owner_age": 91,
            "spouse_age": 88
          },
          {
            "distribution_period": 8.0,
            "owner_age": 91,
            "spouse_age": 89
          },
          {
            "distribution_period": 7.7,
            "owner_age": 91,
            "spouse_age": 90
          },
          {
            "distribution_period": 7.5,
            "owner_age": 91,
            "spouse_age": 91
          },
          {
            "distribution_period": 7.2,
            "owner_age": 91,
            "spouse_age": 92
          },
          {
            "distribution_period": 7.0,
            "owner_age": 91,
            "spouse_age": 93
          },
          {
            "distribution_period": 6.8,
            "owner_age": 91,
            "spouse_age": 94
          },
          {
            "distribution_period": 6.6,
            "owner_age": 91,
            "spouse_age": 95
          },
          {
            "distribution_period": 6.5,
            "owner_age": 91,
            "spouse_age": 96
          },
          {
            "distribution_period": 6.4,
            "owner_age": 91,
            "spouse_age": 97
          },
          {
            "distribution_period": 6.2,
            "owner_age": 91,
            "spouse_age": 98
          },
          {
            "distribution_period": 6.1,
            "owner_age": 91,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 92,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 92,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 92,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 92,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 92,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 92,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 92,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 92,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 92,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 92,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 92,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 92,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 92,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 92,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 92,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 92,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 92,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 92,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 92,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 92,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 92,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 92,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 92,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 92,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 92,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 92,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 92,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 92,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 92,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 92,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 92,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 92,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.4,
            "owner_age": 92,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.5,
            "owner_age": 92,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 92,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 92,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 92,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 92,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.0,
            "owner_age": 92,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.1,
            "owner_age": 92,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.2,
            "owner_age": 92,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.4,
            "owner_age": 92,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.5,
            "owner_age": 92,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.7,
            "owner_age": 92,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.9,
            "owner_age": 92,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.0,
            "owner_age": 92,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.2,
            "owner_age": 92,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.4,
            "owner_age": 92,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.6,
            "owner_age": 92,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.8,
            "owner_age": 92,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.0,
            "owner_age": 92,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.3,
            "owner_age": 92,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.5,
            "owner_age": 92,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.7,
            "owner_age": 92,
            "spouse_age": 73
          },
          {
            "distribution_period": 16.0,
            "owner_age": 92,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.3,
            "owner_age": 92,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.6,
            "owner_age": 92,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.9,
            "owner_age": 92,
            "spouse_age": 77
          },
          {
            "distribution_period": 13.2,
            "owner_age": 92,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.6,
            "owner_age": 92,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.9,
            "owner_age": 92,
            "spouse_age": 80
          },
          {
            "distribution_period": 11.4,
            "owner_age": 92,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.8,
            "owner_age": 92,
            "spouse_age": 82
          },
          {
            "distribution_period": 10.3,
            "owner_age": 92,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.8,
            "owner_age": 92,
            "spouse_age": 84
          },
          {
            "distribution_period": 9.3,
            "owner_age": 92,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.9,
            "owner_age": 92,
            "spouse_age": 86
          },
          {
            "distribution_period": 8.5,
            "owner_age": 92,
            "spouse_age": 87
          },
          {
            "distribution_period": 8.1,
            "owner_age": 92,
            "spouse_age": 88
          },
          {
            "distribution_period": 7.8,
            "owner_age": 92,
            "spouse_age": 89
          },
          {
            "distribution_period": 7.5,
            "owner_age": 92,
            "spouse_age": 90
          },
          {
            "distribution_period": 7.2,
            "owner_age": 92,
            "spouse_age": 91
          },
          {
            "distribution_period": 7.0,
            "owner_age": 92,
            "spouse_age": 92
          },
          {
            "distribution_period": 6.7,
            "owner_age": 92,
            "spouse_age": 93
          },
          {
            "distribution_period": 6.5,
            "owner_age": 92,
            "spouse_age": 94
          },
          {
            "distribution_period": 6.4,
            "owner_age": 92,
            "spouse_age": 95
          },
          {
            "distribution_period": 6.2,
            "owner_age": 92,
            "spouse_age": 96
          },
          {
            "distribution_period": 6.1,
            "owner_age": 92,
            "spouse_age": 97
          },
          {
            "distribution_period": 5.9,
            "owner_age": 92,
            "spouse_age": 98
          },
          {
            "distribution_period": 5.8,
            "owner_age": 92,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 93,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 93,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 93,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 93,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 93,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 93,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 93,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 93,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 93,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 93,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 93,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 93,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 93,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 93,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 93,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 93,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 93,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 93,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 93,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 93,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 93,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 93,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 93,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 93,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 93,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 93,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 93,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 93,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 93,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 93,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 93,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 93,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.4,
            "owner_age": 93,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 93,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 93,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 93,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 93,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 93,
            "spouse_age": 57
          },
          {
            "distribution_period": 29.0,
            "owner_age": 93,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.1,
            "owner_age": 93,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.2,
            "owner_age": 93,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.4,
            "owner_age": 93,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.5,
            "owner_age": 93,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.7,
            "owner_age": 93,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.8,
            "owner_age": 93,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.0,
            "owner_age": 93,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.2,
            "owner_age": 93,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.4,
            "owner_age": 93,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.6,
            "owner_age": 93,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.8,
            "owner_age": 93,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.0,
            "owner_age": 93,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.2,
            "owner_age": 93,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.4,
            "owner_age": 93,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.7,
            "owner_age": 93,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.9,
            "owner_age": 93,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.2,
            "owner_age": 93,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.5,
            "owner_age": 93,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.8,
            "owner_age": 93,
            "spouse_age": 77
          },
          {
            "distribution_period": 13.1,
            "owner_age": 93,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.5,
            "owner_age": 93,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.9,
            "owner_age": 93,
            "spouse_age": 80
          },
          {
            "distribution_period": 11.3,
            "owner_age": 93,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.7,
            "owner_age": 93,
            "spouse_age": 82
          },
          {
            "distribution_period": 10.1,
            "owner_age": 93,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.6,
            "owner_age": 93,
            "spouse_age": 84
          },
          {
            "distribution_period": 9.2,
            "owner_age": 93,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.7,
            "owner_age": 93,
            "spouse_age": 86
          },
          {
            "distribution_period": 8.3,
            "owner_age": 93,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.9,
            "owner_age": 93,
            "spouse_age": 88
          },
          {
            "distribution_period": 7.6,
            "owner_age": 93,
            "spouse_age": 89
          },
          {
            "distribution_period": 7.3,
            "owner_age": 93,
            "spouse_age": 90
          },
          {
            "distribution_period": 7.0,
            "owner_age": 93,
            "spouse_age": 91
          },
          {
            "distribution_period": 6.7,
            "owner_age": 93,
            "spouse_age": 92
          },
          {
            "distribution_period": 6.5,
            "owner_age": 93,
            "spouse_age": 93
          },
          {
            "distribution_period": 6.3,
            "owner_age": 93,
            "spouse_age": 94
          },
          {
            "distribution_period": 6.1,
            "owner_age": 93,
            "spouse_age": 95
          },
          {
            "distribution_period": 5.9,
            "owner_age": 93,
            "spouse_age": 96
          },
          {
            "distribution_period": 5.8,
            "owner_age": 93,
            "spouse_age": 97
          },
          {
            "distribution_period": 5.7,
            "owner_age": 93,
            "spouse_age": 98
          },
          {
            "distribution_period": 5.5,
            "owner_age": 93,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 94,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 94,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 94,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 94,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 94,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 94,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 94,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 94,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 94,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 94,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 94,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 94,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 94,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 94,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 94,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 94,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 94,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 94,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 94,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 94,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 94,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 94,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 94,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 94,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 94,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 94,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 94,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 94,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 94,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 94,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 94,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 94,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.4,
            "owner_age": 94,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 94,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 94,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 94,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 94,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 94,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 94,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.1,
            "owner_age": 94,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.2,
            "owner_age": 94,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 94,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.5,
            "owner_age": 94,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.7,
            "owner_age": 94,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.8,
            "owner_age": 94,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.0,
            "owner_age": 94,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.2,
            "owner_age": 94,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.4,
            "owner_age": 94,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.6,
            "owner_age": 94,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.8,
            "owner_age": 94,
            "spouse_age": 69
          },
          {
            "distribution_period": 19.0,
            "owner_age": 94,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.2,
            "owner_age": 94,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.4,
            "owner_age": 94,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.6,
            "owner_age": 94,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.9,
            "owner_age": 94,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.2,
            "owner_age": 94,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.4,
            "owner_age": 94,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.7,
            "owner_age": 94,
            "spouse_age": 77
          },
          {
            "distribution_period": 13.1,
            "owner_age": 94,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.4,
            "owner_age": 94,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.8,
            "owner_age": 94,
            "spouse_age": 80
          },
          {
            "distribution_period": 11.2,
            "owner_age": 94,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.6,
            "owner_age": 94,
            "spouse_age": 82
          },
          {
            "distribution_period": 10.0,
            "owner_age": 94,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.5,
            "owner_age": 94,
            "spouse_age": 84
          },
          {
            "distribution_period": 9.0,
            "owner_age": 94,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.6,
            "owner_age": 94,
            "spouse_age": 86
          },
          {
            "distribution_period": 8.2,
            "owner_age": 94,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.8,
            "owner_age": 94,
            "spouse_age": 88
          },
          {
            "distribution_period": 7.4,
            "owner_age": 94,
            "spouse_age": 89
          },
          {
            "distribution_period": 7.1,
            "owner_age": 94,
            "spouse_age": 90
          },
          {
            "distribution_period": 6.8,
            "owner_age": 94,
            "spouse_age": 91
          },
          {
            "distribution_period": 6.5,
            "owner_age": 94,
            "spouse_age": 92
          },
          {
            "distribution_period": 6.3,
            "owner_age": 94,
            "spouse_age": 93
          },
          {
            "distribution_period": 6.1,
            "owner_age": 94,
            "spouse_age": 94
          },
          {
            "distribution_period": 5.9,
            "owner_age": 94,
            "spouse_age": 95
          },
          {
            "distribution_period": 5.7,
            "owner_age": 94,
            "spouse_age": 96
          },
          {
            "distribution_period": 5.5,
            "owner_age": 94,
            "spouse_age": 97
          },
          {
            "distribution_period": 5.4,
            "owner_age": 94,
            "spouse_age": 98
          },
          {
            "distribution_period": 5.3,
            "owner_age": 94,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 95,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 95,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 95,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 95,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 95,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 95,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 95,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 95,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 95,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 95,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 95,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 95,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 95,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 95,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 95,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 95,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 95,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 95,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 95,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 95,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 95,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 95,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 95,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 95,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 95,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 95,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 95,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 95,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 95,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 95,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 95,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 95,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.4,
            "owner_age": 95,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 95,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 95,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 95,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 95,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 95,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 95,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.1,
            "owner_age": 95,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.2,
            "owner_age": 95,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 95,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.5,
            "owner_age": 95,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 95,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.8,
            "owner_age": 95,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.0,
            "owner_age": 95,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.2,
            "owner_age": 95,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.4,
            "owner_age": 95,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.6,
            "owner_age": 95,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.7,
            "owner_age": 95,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.9,
            "owner_age": 95,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.2,
            "owner_age": 95,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.4,
            "owner_age": 95,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.6,
            "owner_age": 95,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.9,
            "owner_age": 95,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.1,
            "owner_age": 95,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.4,
            "owner_age": 95,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.7,
            "owner_age": 95,
            "spouse_age": 77
          },
          {
            "distribution_period": 13.0,
            "owner_age": 95,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.3,
            "owner_age": 95,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.7,
            "owner_age": 95,
            "spouse_age": 80
          },
          {
            "distribution_period": 11.1,
            "owner_age": 95,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.5,
            "owner_age": 95,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.9,
            "owner_age": 95,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.4,
            "owner_age": 95,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.9,
            "owner_age": 95,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.5,
            "owner_age": 95,
            "spouse_age": 86
          },
          {
            "distribution_period": 8.0,
            "owner_age": 95,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.6,
            "owner_age": 95,
            "spouse_age": 88
          },
          {
            "distribution_period": 7.3,
            "owner_age": 95,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.9,
            "owner_age": 95,
            "spouse_age": 90
          },
          {
            "distribution_period": 6.6,
            "owner_age": 95,
            "spouse_age": 91
          },
          {
            "distribution_period": 6.4,
            "owner_age": 95,
            "spouse_age": 92
          },
          {
            "distribution_period": 6.1,
            "owner_age": 95,
            "spouse_age": 93
          },
          {
            "distribution_period": 5.9,
            "owner_age": 95,
            "spouse_age": 94
          },
          {
            "distribution_period": 5.7,
            "owner_age": 95,
            "spouse_age": 95
          },
          {
            "distribution_period": 5.5,
            "owner_age": 95,
            "spouse_age": 96
          },
          {
            "distribution_period": 5.3,
            "owner_age": 95,
            "spouse_age": 97
          },
          {
            "distribution_period": 5.2,
            "owner_age": 95,
            "spouse_age": 98
          },
          {
            "distribution_period": 5.0,
            "owner_age": 95,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 96,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 96,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 96,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 96,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 96,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 96,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 96,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 96,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 96,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 96,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 96,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 96,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 96,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 96,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 96,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 96,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 96,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 96,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 96,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 96,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 96,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 96,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 96,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 96,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 96,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 96,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 96,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 96,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 96,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 96,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 96,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 96,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 96,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 96,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 96,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 96,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 96,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 96,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 96,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 96,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.2,
            "owner_age": 96,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 96,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.5,
            "owner_age": 96,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 96,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.8,
            "owner_age": 96,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.0,
            "owner_age": 96,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.2,
            "owner_age": 96,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 96,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 96,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.7,
            "owner_age": 96,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.9,
            "owner_age": 96,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.1,
            "owner_age": 96,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.4,
            "owner_age": 96,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.6,
            "owner_age": 96,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.8,
            "owner_age": 96,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.1,
            "owner_age": 96,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.3,
            "owner_age": 96,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.6,
            "owner_age": 96,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.9,
            "owner_age": 96,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.3,
            "owner_age": 96,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.6,
            "owner_age": 96,
            "spouse_age": 80
          },
          {
            "distribution_period": 11.0,
            "owner_age": 96,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.4,
            "owner_age": 96,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.9,
            "owner_age": 96,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.3,
            "owner_age": 96,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.8,
            "owner_age": 96,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.4,
            "owner_age": 96,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.9,
            "owner_age": 96,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.5,
            "owner_age": 96,
            "spouse_age": 88
          },
          {
            "distribution_period": 7.1,
            "owner_age": 96,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.8,
            "owner_age": 96,
            "spouse_age": 90
          },
          {
            "distribution_period": 6.5,
            "owner_age": 96,
            "spouse_age": 91
          },
          {
            "distribution_period": 6.2,
            "owner_age": 96,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.9,
            "owner_age": 96,
            "spouse_age": 93
          },
          {
            "distribution_period": 5.7,
            "owner_age": 96,
            "spouse_age": 94
          },
          {
            "distribution_period": 5.5,
            "owner_age": 96,
            "spouse_age": 95
          },
          {
            "distribution_period": 5.3,
            "owner_age": 96,
            "spouse_age": 96
          },
          {
            "distribution_period": 5.1,
            "owner_age": 96,
            "spouse_age": 97
          },
          {
            "distribution_period": 5.0,
            "owner_age": 96,
            "spouse_age": 98
          },
          {
            "distribution_period": 4.8,
            "owner_age": 96,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 97,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 97,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 97,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 97,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 97,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 97,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 97,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 97,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 97,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 97,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 97,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 97,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 97,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 97,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 97,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 97,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 97,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 97,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 97,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 97,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 97,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 97,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 97,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 97,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 97,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 97,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 97,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 97,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 97,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 97,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 97,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 97,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 97,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 97,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 97,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 97,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 97,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 97,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 97,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 97,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.2,
            "owner_age": 97,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 97,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.5,
            "owner_age": 97,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 97,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.8,
            "owner_age": 97,
            "spouse_age": 64
          },
          {
            "distribution_period": 23.0,
            "owner_age": 97,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 97,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 97,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 97,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.7,
            "owner_age": 97,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.9,
            "owner_age": 97,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.1,
            "owner_age": 97,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.3,
            "owner_age": 97,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.6,
            "owner_age": 97,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.8,
            "owner_age": 97,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.0,
            "owner_age": 97,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.3,
            "owner_age": 97,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.6,
            "owner_age": 97,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.9,
            "owner_age": 97,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.2,
            "owner_age": 97,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.6,
            "owner_age": 97,
            "spouse_age": 80
          },
          {
            "distribution_period": 11.0,
            "owner_age": 97,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.4,
            "owner_age": 97,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.8,
            "owner_age": 97,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.2,
            "owner_age": 97,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.7,
            "owner_age": 97,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.3,
            "owner_age": 97,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.8,
            "owner_age": 97,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.4,
            "owner_age": 97,
            "spouse_age": 88
          },
          {
            "distribution_period": 7.0,
            "owner_age": 97,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.7,
            "owner_age": 97,
            "spouse_age": 90
          },
          {
            "distribution_period": 6.4,
            "owner_age": 97,
            "spouse_age": 91
          },
          {
            "distribution_period": 6.1,
            "owner_age": 97,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.8,
            "owner_age": 97,
            "spouse_age": 93
          },
          {
            "distribution_period": 5.5,
            "owner_age": 97,
            "spouse_age": 94
          },
          {
            "distribution_period": 5.3,
            "owner_age": 97,
            "spouse_age": 95
          },
          {
            "distribution_period": 5.1,
            "owner_age": 97,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.9,
            "owner_age": 97,
            "spouse_age": 97
          },
          {
            "distribution_period": 4.8,
            "owner_age": 97,
            "spouse_age": 98
          },
          {
            "distribution_period": 4.6,
            "owner_age": 97,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 98,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 98,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 98,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 98,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 98,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 98,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 98,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 98,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 98,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 98,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 98,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 98,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 98,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 98,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 98,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 98,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 98,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 98,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 98,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 98,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 98,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 98,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 98,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 98,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 98,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 98,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 98,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 98,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 98,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 98,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 98,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 98,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 98,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 98,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 98,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 98,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 98,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 98,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 98,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 98,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.2,
            "owner_age": 98,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 98,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.5,
            "owner_age": 98,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 98,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.8,
            "owner_age": 98,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 98,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 98,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 98,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 98,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.7,
            "owner_age": 98,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.9,
            "owner_age": 98,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.1,
            "owner_age": 98,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.3,
            "owner_age": 98,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 98,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.8,
            "owner_age": 98,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.0,
            "owner_age": 98,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.3,
            "owner_age": 98,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.6,
            "owner_age": 98,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.9,
            "owner_age": 98,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.2,
            "owner_age": 98,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.5,
            "owner_age": 98,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.9,
            "owner_age": 98,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.3,
            "owner_age": 98,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.7,
            "owner_age": 98,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.2,
            "owner_age": 98,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.7,
            "owner_age": 98,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.2,
            "owner_age": 98,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.7,
            "owner_age": 98,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.3,
            "owner_age": 98,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.9,
            "owner_age": 98,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.6,
            "owner_age": 98,
            "spouse_age": 90
          },
          {
            "distribution_period": 6.2,
            "owner_age": 98,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.9,
            "owner_age": 98,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.7,
            "owner_age": 98,
            "spouse_age": 93
          },
          {
            "distribution_period": 5.4,
            "owner_age": 98,
            "spouse_age": 94
          },
          {
            "distribution_period": 5.2,
            "owner_age": 98,
            "spouse_age": 95
          },
          {
            "distribution_period": 5.0,
            "owner_age": 98,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.8,
            "owner_age": 98,
            "spouse_age": 97
          },
          {
            "distribution_period": 4.6,
            "owner_age": 98,
            "spouse_age": 98
          },
          {
            "distribution_period": 4.5,
            "owner_age": 98,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 99,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 99,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 99,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 99,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 99,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 99,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 99,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 99,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 99,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 99,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 99,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 99,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 99,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 99,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 99,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 99,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 99,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 99,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 99,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 99,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 99,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 99,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 99,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 99,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 99,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 99,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 99,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.1,
            "owner_age": 99,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 99,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.2,
            "owner_age": 99,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 99,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 99,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 99,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 99,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 99,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 99,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 99,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 99,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 99,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 99,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.2,
            "owner_age": 99,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 99,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 99,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 99,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.8,
            "owner_age": 99,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 99,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 99,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 99,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 99,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.7,
            "owner_age": 99,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.9,
            "owner_age": 99,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.1,
            "owner_age": 99,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.3,
            "owner_age": 99,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 99,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 99,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.0,
            "owner_age": 99,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.3,
            "owner_age": 99,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.5,
            "owner_age": 99,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.8,
            "owner_age": 99,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.2,
            "owner_age": 99,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.5,
            "owner_age": 99,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.9,
            "owner_age": 99,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.2,
            "owner_age": 99,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.7,
            "owner_age": 99,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.1,
            "owner_age": 99,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.6,
            "owner_age": 99,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.1,
            "owner_age": 99,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.6,
            "owner_age": 99,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.2,
            "owner_age": 99,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.8,
            "owner_age": 99,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.5,
            "owner_age": 99,
            "spouse_age": 90
          },
          {
            "distribution_period": 6.1,
            "owner_age": 99,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.8,
            "owner_age": 99,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.5,
            "owner_age": 99,
            "spouse_age": 93
          },
          {
            "distribution_period": 5.3,
            "owner_age": 99,
            "spouse_age": 94
          },
          {
            "distribution_period": 5.0,
            "owner_age": 99,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.8,
            "owner_age": 99,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.6,
            "owner_age": 99,
            "spouse_age": 97
          },
          {
            "distribution_period": 4.5,
            "owner_age": 99,
            "spouse_age": 98
          },
          {
            "distribution_period": 4.3,
            "owner_age": 99,
            "spouse_age": 99
          },
          {
            "distribution_period": 65.0,
            "owner_age": 100,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 100,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 100,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 100,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 100,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 100,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 100,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 100,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 100,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 100,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 100,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 100,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 100,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 100,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 100,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 100,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 100,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 100,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 100,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 100,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 100,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 100,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 100,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 100,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 100,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 100,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 100,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 100,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 100,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 100,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 100,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 100,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 100,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 100,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 100,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 100,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 100,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 100,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 100,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 100,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 100,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 100,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 100,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 100,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.8,
            "owner_age": 100,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 100,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 100,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 100,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 100,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.7,
            "owner_age": 100,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.9,
            "owner_age": 100,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.1,
            "owner_age": 100,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.3,
            "owner_age": 100,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 100,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 100,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.0,
            "owner_age": 100,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 100,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.5,
            "owner_age": 100,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.8,
            "owner_age": 100,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.1,
            "owner_age": 100,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.5,
            "owner_age": 100,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.8,
            "owner_age": 100,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.2,
            "owner_age": 100,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.6,
            "owner_age": 100,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.1,
            "owner_age": 100,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.5,
            "owner_age": 100,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.0,
            "owner_age": 100,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.6,
            "owner_age": 100,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.2,
            "owner_age": 100,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.8,
            "owner_age": 100,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.4,
            "owner_age": 100,
            "spouse_age": 90
          },
          {
            "distribution_period": 6.0,
            "owner_age": 100,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.7,
            "owner_age": 100,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.4,
            "owner_age": 100,
            "spouse_age": 93
          },
          {
            "distribution_period": 5.2,
            "owner_age": 100,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.9,
            "owner_age": 100,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.7,
            "owner_age": 100,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.5,
            "owner_age": 100,
            "spouse_age": 97
          },
          {
            "distribution_period": 4.3,
            "owner_age": 100,
            "spouse_age": 98
          },
          {
            "distribution_period": 4.2,
            "owner_age": 100,
            "spouse_age": 99
          },
          {
            "distribution_period": 4.1,
            "owner_age": 100,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.9,
            "owner_age": 100,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.8,
            "owner_age": 100,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.7,
            "owner_age": 100,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.7,
            "owner_age": 100,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.6,
            "owner_age": 100,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.6,
            "owner_age": 100,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.6,
            "owner_age": 100,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.6,
            "owner_age": 100,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.6,
            "owner_age": 100,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 101,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 101,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 101,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 101,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 101,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 101,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 101,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 101,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 101,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 101,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 101,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 101,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 101,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 101,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 101,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 101,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 101,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 101,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 101,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 101,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 101,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 101,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 101,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 101,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 101,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 101,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 101,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 101,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 101,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 101,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 101,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 101,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 101,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 101,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 101,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 101,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 101,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 101,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 101,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 101,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 101,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 101,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 101,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 101,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.8,
            "owner_age": 101,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 101,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 101,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 101,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 101,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.7,
            "owner_age": 101,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.9,
            "owner_age": 101,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.1,
            "owner_age": 101,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.3,
            "owner_age": 101,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 101,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 101,
            "spouse_age": 74
          },
          {
            "distribution_period": 15.0,
            "owner_age": 101,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 101,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.5,
            "owner_age": 101,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.8,
            "owner_age": 101,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.1,
            "owner_age": 101,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.4,
            "owner_age": 101,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.8,
            "owner_age": 101,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.2,
            "owner_age": 101,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.6,
            "owner_age": 101,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.0,
            "owner_age": 101,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.5,
            "owner_age": 101,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.0,
            "owner_age": 101,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.5,
            "owner_age": 101,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.1,
            "owner_age": 101,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.7,
            "owner_age": 101,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.3,
            "owner_age": 101,
            "spouse_age": 90
          },
          {
            "distribution_period": 6.0,
            "owner_age": 101,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.6,
            "owner_age": 101,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.3,
            "owner_age": 101,
            "spouse_age": 93
          },
          {
            "distribution_period": 5.1,
            "owner_age": 101,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.8,
            "owner_age": 101,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.6,
            "owner_age": 101,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.4,
            "owner_age": 101,
            "spouse_age": 97
          },
          {
            "distribution_period": 4.2,
            "owner_age": 101,
            "spouse_age": 98
          },
          {
            "distribution_period": 4.1,
            "owner_age": 101,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.9,
            "owner_age": 101,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.8,
            "owner_age": 101,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.7,
            "owner_age": 101,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.6,
            "owner_age": 101,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.5,
            "owner_age": 101,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.5,
            "owner_age": 101,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.5,
            "owner_age": 101,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.4,
            "owner_age": 101,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.4,
            "owner_age": 101,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.4,
            "owner_age": 101,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 102,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 102,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 102,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 102,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 102,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 102,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 102,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 102,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 102,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 102,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 102,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 102,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 102,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 102,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 102,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.6,
            "owner_age": 102,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 102,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 102,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 102,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 102,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 102,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 102,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 102,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 102,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 102,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 102,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 102,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 102,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 102,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 102,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 102,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 102,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 102,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 102,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 102,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 102,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 102,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 102,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 102,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 102,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 102,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 102,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 102,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 102,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 102,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 102,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 102,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 102,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 102,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.7,
            "owner_age": 102,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 102,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 102,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.3,
            "owner_age": 102,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 102,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 102,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 102,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 102,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.5,
            "owner_age": 102,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.8,
            "owner_age": 102,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.1,
            "owner_age": 102,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.4,
            "owner_age": 102,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.8,
            "owner_age": 102,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 102,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.6,
            "owner_age": 102,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.0,
            "owner_age": 102,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.5,
            "owner_age": 102,
            "spouse_age": 85
          },
          {
            "distribution_period": 8.0,
            "owner_age": 102,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.5,
            "owner_age": 102,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.0,
            "owner_age": 102,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.6,
            "owner_age": 102,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.3,
            "owner_age": 102,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.9,
            "owner_age": 102,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.6,
            "owner_age": 102,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.3,
            "owner_age": 102,
            "spouse_age": 93
          },
          {
            "distribution_period": 5.0,
            "owner_age": 102,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.7,
            "owner_age": 102,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.5,
            "owner_age": 102,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.3,
            "owner_age": 102,
            "spouse_age": 97
          },
          {
            "distribution_period": 4.1,
            "owner_age": 102,
            "spouse_age": 98
          },
          {
            "distribution_period": 4.0,
            "owner_age": 102,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.8,
            "owner_age": 102,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.7,
            "owner_age": 102,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.6,
            "owner_age": 102,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.5,
            "owner_age": 102,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.4,
            "owner_age": 102,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.4,
            "owner_age": 102,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.3,
            "owner_age": 102,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.3,
            "owner_age": 102,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.3,
            "owner_age": 102,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.3,
            "owner_age": 102,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 103,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 103,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 103,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 103,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 103,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 103,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 103,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 103,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 103,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 103,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 103,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 103,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 103,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 103,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 103,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 103,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 103,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 103,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 103,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 103,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 103,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 103,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 103,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 103,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 103,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 103,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 103,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 103,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 103,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 103,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 103,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 103,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 103,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 103,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 103,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 103,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 103,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 103,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 103,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 103,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 103,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 103,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 103,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 103,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 103,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 103,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 103,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 103,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 103,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 103,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 103,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 103,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.3,
            "owner_age": 103,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 103,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 103,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 103,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 103,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.5,
            "owner_age": 103,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.8,
            "owner_age": 103,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.1,
            "owner_age": 103,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.4,
            "owner_age": 103,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 103,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 103,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 103,
            "spouse_age": 83
          },
          {
            "distribution_period": 9.0,
            "owner_age": 103,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.4,
            "owner_age": 103,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.9,
            "owner_age": 103,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.4,
            "owner_age": 103,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.0,
            "owner_age": 103,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.6,
            "owner_age": 103,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.2,
            "owner_age": 103,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.9,
            "owner_age": 103,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.5,
            "owner_age": 103,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.2,
            "owner_age": 103,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.9,
            "owner_age": 103,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.7,
            "owner_age": 103,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.5,
            "owner_age": 103,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.2,
            "owner_age": 103,
            "spouse_age": 97
          },
          {
            "distribution_period": 4.1,
            "owner_age": 103,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.9,
            "owner_age": 103,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.7,
            "owner_age": 103,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.6,
            "owner_age": 103,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.5,
            "owner_age": 103,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.4,
            "owner_age": 103,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.3,
            "owner_age": 103,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.3,
            "owner_age": 103,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.2,
            "owner_age": 103,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.2,
            "owner_age": 103,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.2,
            "owner_age": 103,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.2,
            "owner_age": 103,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 104,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 104,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 104,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 104,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 104,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 104,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 104,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 104,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 104,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 104,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 104,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 104,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 104,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 104,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 104,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 104,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 104,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 104,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 104,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 104,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.8,
            "owner_age": 104,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 104,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 104,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 104,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 104,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 104,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 104,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 104,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 104,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 104,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 104,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 104,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 104,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 104,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 104,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 104,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 104,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 104,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 104,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 104,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 104,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 104,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 104,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 104,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 104,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 104,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 104,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 104,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 104,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 104,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 104,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 104,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 104,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 104,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 104,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 104,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 104,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.5,
            "owner_age": 104,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 104,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 104,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.4,
            "owner_age": 104,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 104,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 104,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 104,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 104,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.4,
            "owner_age": 104,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.9,
            "owner_age": 104,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.4,
            "owner_age": 104,
            "spouse_age": 87
          },
          {
            "distribution_period": 7.0,
            "owner_age": 104,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.6,
            "owner_age": 104,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.2,
            "owner_age": 104,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.8,
            "owner_age": 104,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.5,
            "owner_age": 104,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.2,
            "owner_age": 104,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.9,
            "owner_age": 104,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.6,
            "owner_age": 104,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.4,
            "owner_age": 104,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.2,
            "owner_age": 104,
            "spouse_age": 97
          },
          {
            "distribution_period": 4.0,
            "owner_age": 104,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.8,
            "owner_age": 104,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.7,
            "owner_age": 104,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.5,
            "owner_age": 104,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.4,
            "owner_age": 104,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.3,
            "owner_age": 104,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.3,
            "owner_age": 104,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.2,
            "owner_age": 104,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.2,
            "owner_age": 104,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.2,
            "owner_age": 104,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.1,
            "owner_age": 104,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.1,
            "owner_age": 104,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 105,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 105,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 105,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 105,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 105,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 105,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 105,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 105,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 105,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 105,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 105,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 105,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 105,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 105,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 105,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 105,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 105,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 105,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 105,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 105,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 105,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 105,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 105,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 105,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 105,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 105,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 105,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 105,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 105,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 105,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 105,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 105,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 105,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 105,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 105,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 105,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 105,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 105,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 105,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 105,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 105,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 105,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 105,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 105,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 105,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 105,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 105,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 105,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 105,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 105,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 105,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 105,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 105,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 105,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 105,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 105,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 105,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 105,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 105,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 105,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.4,
            "owner_age": 105,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 105,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 105,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 105,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 105,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.4,
            "owner_age": 105,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.9,
            "owner_age": 105,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.4,
            "owner_age": 105,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 105,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.5,
            "owner_age": 105,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.1,
            "owner_age": 105,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.8,
            "owner_age": 105,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.4,
            "owner_age": 105,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.1,
            "owner_age": 105,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.9,
            "owner_age": 105,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.6,
            "owner_age": 105,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.4,
            "owner_age": 105,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.1,
            "owner_age": 105,
            "spouse_age": 97
          },
          {
            "distribution_period": 4.0,
            "owner_age": 105,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.8,
            "owner_age": 105,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.6,
            "owner_age": 105,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.5,
            "owner_age": 105,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.4,
            "owner_age": 105,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.3,
            "owner_age": 105,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.2,
            "owner_age": 105,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.1,
            "owner_age": 105,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.1,
            "owner_age": 105,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.1,
            "owner_age": 105,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.1,
            "owner_age": 105,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.1,
            "owner_age": 105,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 106,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 106,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 106,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 106,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 106,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 106,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 106,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 106,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 106,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 106,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 106,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 106,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 106,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 106,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 106,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 106,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 106,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 106,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 106,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 106,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 106,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 106,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 106,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 106,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 106,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 106,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 106,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 106,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 106,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 106,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 106,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 106,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 106,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 106,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 106,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 106,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 106,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 106,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 106,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 106,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 106,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 106,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 106,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 106,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 106,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 106,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 106,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 106,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 106,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 106,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 106,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 106,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 106,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 106,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 106,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 106,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 106,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 106,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 106,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 106,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.4,
            "owner_age": 106,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 106,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 106,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 106,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 106,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.4,
            "owner_age": 106,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.9,
            "owner_age": 106,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.4,
            "owner_age": 106,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 106,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.5,
            "owner_age": 106,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.1,
            "owner_age": 106,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.8,
            "owner_age": 106,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.4,
            "owner_age": 106,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.1,
            "owner_age": 106,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.8,
            "owner_age": 106,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.6,
            "owner_age": 106,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.3,
            "owner_age": 106,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.1,
            "owner_age": 106,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.9,
            "owner_age": 106,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.8,
            "owner_age": 106,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.6,
            "owner_age": 106,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.5,
            "owner_age": 106,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.3,
            "owner_age": 106,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.2,
            "owner_age": 106,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.2,
            "owner_age": 106,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.1,
            "owner_age": 106,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.1,
            "owner_age": 106,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.1,
            "owner_age": 106,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.0,
            "owner_age": 106,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.0,
            "owner_age": 106,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 107,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 107,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 107,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 107,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 107,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 107,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 107,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 107,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 107,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 107,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 107,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 107,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 107,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 107,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 107,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 107,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 107,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 107,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 107,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 107,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 107,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 107,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 107,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 107,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 107,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 107,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 107,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 107,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 107,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 107,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 107,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 107,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 107,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 107,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 107,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 107,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 107,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 107,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 107,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 107,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 107,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 107,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 107,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 107,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 107,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 107,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 107,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 107,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 107,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 107,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 107,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 107,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 107,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 107,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 107,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 107,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 107,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 107,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 107,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 107,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.4,
            "owner_age": 107,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 107,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 107,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 107,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 107,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.4,
            "owner_age": 107,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.9,
            "owner_age": 107,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.4,
            "owner_age": 107,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 107,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.5,
            "owner_age": 107,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.1,
            "owner_age": 107,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.8,
            "owner_age": 107,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.4,
            "owner_age": 107,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.1,
            "owner_age": 107,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.8,
            "owner_age": 107,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.6,
            "owner_age": 107,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.3,
            "owner_age": 107,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.1,
            "owner_age": 107,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.9,
            "owner_age": 107,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.7,
            "owner_age": 107,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.6,
            "owner_age": 107,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.4,
            "owner_age": 107,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.3,
            "owner_age": 107,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.2,
            "owner_age": 107,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.2,
            "owner_age": 107,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.1,
            "owner_age": 107,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.1,
            "owner_age": 107,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.0,
            "owner_age": 107,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.0,
            "owner_age": 107,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.0,
            "owner_age": 107,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 108,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 108,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 108,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 108,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 108,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 108,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 108,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 108,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 108,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 108,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 108,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 108,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 108,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 108,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 108,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 108,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 108,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 108,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 108,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 108,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 108,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 108,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 108,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 108,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 108,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 108,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 108,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 108,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 108,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 108,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 108,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 108,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 108,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 108,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 108,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 108,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 108,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 108,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 108,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 108,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 108,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 108,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 108,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 108,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 108,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 108,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 108,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 108,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.5,
            "owner_age": 108,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 108,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 108,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 108,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 108,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.5,
            "owner_age": 108,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 108,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 108,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 108,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 108,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 108,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 108,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.4,
            "owner_age": 108,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 108,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 108,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 108,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 108,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.4,
            "owner_age": 108,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.8,
            "owner_age": 108,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.4,
            "owner_age": 108,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 108,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.5,
            "owner_age": 108,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.1,
            "owner_age": 108,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.7,
            "owner_age": 108,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.4,
            "owner_age": 108,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.1,
            "owner_age": 108,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.8,
            "owner_age": 108,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.5,
            "owner_age": 108,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.3,
            "owner_age": 108,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.1,
            "owner_age": 108,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.9,
            "owner_age": 108,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.7,
            "owner_age": 108,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.6,
            "owner_age": 108,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.4,
            "owner_age": 108,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.3,
            "owner_age": 108,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.2,
            "owner_age": 108,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.1,
            "owner_age": 108,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.1,
            "owner_age": 108,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.0,
            "owner_age": 108,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.0,
            "owner_age": 108,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.0,
            "owner_age": 108,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.0,
            "owner_age": 108,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 109,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 109,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 109,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 109,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 109,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 109,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 109,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 109,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 109,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 109,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 109,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 109,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 109,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 109,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 109,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 109,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 109,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 109,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 109,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 109,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 109,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 109,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 109,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 109,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 109,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 109,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 109,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 109,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 109,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 109,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 109,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 109,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 109,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 109,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 109,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 109,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 109,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 109,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 109,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 109,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 109,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 109,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 109,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 109,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 109,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 109,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 109,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 109,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 109,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 109,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 109,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 109,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 109,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 109,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 109,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 109,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 109,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 109,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 109,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 109,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 109,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 109,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 109,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 109,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 109,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.4,
            "owner_age": 109,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.8,
            "owner_age": 109,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.4,
            "owner_age": 109,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 109,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.5,
            "owner_age": 109,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.1,
            "owner_age": 109,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.7,
            "owner_age": 109,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.4,
            "owner_age": 109,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.1,
            "owner_age": 109,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.8,
            "owner_age": 109,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.5,
            "owner_age": 109,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.3,
            "owner_age": 109,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.1,
            "owner_age": 109,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.9,
            "owner_age": 109,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.7,
            "owner_age": 109,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.6,
            "owner_age": 109,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.4,
            "owner_age": 109,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.3,
            "owner_age": 109,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.2,
            "owner_age": 109,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.1,
            "owner_age": 109,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.1,
            "owner_age": 109,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.0,
            "owner_age": 109,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.0,
            "owner_age": 109,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.0,
            "owner_age": 109,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.0,
            "owner_age": 109,
            "spouse_age": 109
          },
          {
            "distribution_period": 65.0,
            "owner_age": 110,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 110,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 110,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 110,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 110,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 110,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 110,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 110,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 110,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 110,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 110,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 110,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 110,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 110,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 110,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 110,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 110,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 110,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 110,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 110,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 110,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 110,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 110,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 110,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 110,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 110,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 110,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 110,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 110,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 110,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 110,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 110,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 110,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 110,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 110,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 110,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 110,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 110,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 110,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 110,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 110,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 110,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 110,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 110,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 110,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 110,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 110,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 110,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 110,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 110,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 110,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 110,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 110,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 110,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 110,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 110,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 110,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 110,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 110,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 110,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 110,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 110,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 110,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 110,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 110,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.3,
            "owner_age": 110,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.8,
            "owner_age": 110,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.4,
            "owner_age": 110,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 110,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.5,
            "owner_age": 110,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.1,
            "owner_age": 110,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.7,
            "owner_age": 110,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.4,
            "owner_age": 110,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.1,
            "owner_age": 110,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.8,
            "owner_age": 110,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.5,
            "owner_age": 110,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.3,
            "owner_age": 110,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.1,
            "owner_age": 110,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.9,
            "owner_age": 110,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.7,
            "owner_age": 110,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.5,
            "owner_age": 110,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.4,
            "owner_age": 110,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.3,
            "owner_age": 110,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.2,
            "owner_age": 110,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.1,
            "owner_age": 110,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.1,
            "owner_age": 110,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.0,
            "owner_age": 110,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.0,
            "owner_age": 110,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.0,
            "owner_age": 110,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.0,
            "owner_age": 110,
            "spouse_age": 109
          },
          {
            "distribution_period": 3.0,
            "owner_age": 110,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.9,
            "owner_age": 110,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.9,
            "owner_age": 110,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.9,
            "owner_age": 110,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.9,
            "owner_age": 110,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.8,
            "owner_age": 110,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.7,
            "owner_age": 110,
            "spouse_age": 116
          },
          {
            "distribution_period": 2.6,
            "owner_age": 110,
            "spouse_age": 117
          },
          {
            "distribution_period": 2.5,
            "owner_age": 110,
            "spouse_age": 118
          },
          {
            "distribution_period": 2.2,
            "owner_age": 110,
            "spouse_age": 119
          },
          {
            "distribution_period": 2.0,
            "owner_age": 110,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 111,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 111,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 111,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 111,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 111,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 111,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 111,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 111,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 111,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 111,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 111,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 111,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 111,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 111,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 111,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 111,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 111,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 111,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 111,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 111,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 111,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 111,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 111,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 111,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 111,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 111,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 111,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 111,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 111,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 111,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 111,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 111,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 111,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 111,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 111,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 111,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 111,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 111,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 111,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 111,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 111,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 111,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 111,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 111,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 111,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 111,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 111,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 111,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 111,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 111,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 111,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 111,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 111,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 111,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 111,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 111,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 111,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 111,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 111,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 111,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 111,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 111,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 111,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 111,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 111,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.3,
            "owner_age": 111,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.8,
            "owner_age": 111,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.3,
            "owner_age": 111,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 111,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.5,
            "owner_age": 111,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.1,
            "owner_age": 111,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.7,
            "owner_age": 111,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.4,
            "owner_age": 111,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.1,
            "owner_age": 111,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.8,
            "owner_age": 111,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.5,
            "owner_age": 111,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.3,
            "owner_age": 111,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.1,
            "owner_age": 111,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.9,
            "owner_age": 111,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.7,
            "owner_age": 111,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.5,
            "owner_age": 111,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.4,
            "owner_age": 111,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.3,
            "owner_age": 111,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.2,
            "owner_age": 111,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.1,
            "owner_age": 111,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.0,
            "owner_age": 111,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.0,
            "owner_age": 111,
            "spouse_age": 106
          },
          {
            "distribution_period": 3.0,
            "owner_age": 111,
            "spouse_age": 107
          },
          {
            "distribution_period": 3.0,
            "owner_age": 111,
            "spouse_age": 108
          },
          {
            "distribution_period": 3.0,
            "owner_age": 111,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.9,
            "owner_age": 111,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.9,
            "owner_age": 111,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.9,
            "owner_age": 111,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.9,
            "owner_age": 111,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.8,
            "owner_age": 111,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.8,
            "owner_age": 111,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.7,
            "owner_age": 111,
            "spouse_age": 116
          },
          {
            "distribution_period": 2.6,
            "owner_age": 111,
            "spouse_age": 117
          },
          {
            "distribution_period": 2.4,
            "owner_age": 111,
            "spouse_age": 118
          },
          {
            "distribution_period": 2.2,
            "owner_age": 111,
            "spouse_age": 119
          },
          {
            "distribution_period": 2.0,
            "owner_age": 111,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 112,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 112,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 112,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 112,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 112,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 112,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 112,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 112,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 112,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 112,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 112,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 112,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 112,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 112,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 112,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 112,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 112,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 112,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 112,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 112,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 112,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 112,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 112,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 112,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 112,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 112,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 112,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 112,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 112,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 112,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 112,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 112,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 112,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 112,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 112,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 112,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 112,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 112,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 112,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 112,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 112,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 112,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 112,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 112,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 112,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 112,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 112,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 112,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 112,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 112,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 112,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 112,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 112,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 112,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 112,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 112,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 112,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 112,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 112,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 112,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 112,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 112,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.1,
            "owner_age": 112,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.5,
            "owner_age": 112,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 112,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.3,
            "owner_age": 112,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.8,
            "owner_age": 112,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.3,
            "owner_age": 112,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 112,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.5,
            "owner_age": 112,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.1,
            "owner_age": 112,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.7,
            "owner_age": 112,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.4,
            "owner_age": 112,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.1,
            "owner_age": 112,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.8,
            "owner_age": 112,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.5,
            "owner_age": 112,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.3,
            "owner_age": 112,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.0,
            "owner_age": 112,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.8,
            "owner_age": 112,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.7,
            "owner_age": 112,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.5,
            "owner_age": 112,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.4,
            "owner_age": 112,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.2,
            "owner_age": 112,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.1,
            "owner_age": 112,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.1,
            "owner_age": 112,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.0,
            "owner_age": 112,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.0,
            "owner_age": 112,
            "spouse_age": 106
          },
          {
            "distribution_period": 2.9,
            "owner_age": 112,
            "spouse_age": 107
          },
          {
            "distribution_period": 2.9,
            "owner_age": 112,
            "spouse_age": 108
          },
          {
            "distribution_period": 2.9,
            "owner_age": 112,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.9,
            "owner_age": 112,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.9,
            "owner_age": 112,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.9,
            "owner_age": 112,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.9,
            "owner_age": 112,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.8,
            "owner_age": 112,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.8,
            "owner_age": 112,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.7,
            "owner_age": 112,
            "spouse_age": 116
          },
          {
            "distribution_period": 2.6,
            "owner_age": 112,
            "spouse_age": 117
          },
          {
            "distribution_period": 2.4,
            "owner_age": 112,
            "spouse_age": 118
          },
          {
            "distribution_period": 2.2,
            "owner_age": 112,
            "spouse_age": 119
          },
          {
            "distribution_period": 2.0,
            "owner_age": 112,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 113,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 113,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 113,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 113,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 113,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 113,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 113,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 113,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 113,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 113,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 113,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 113,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 113,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 113,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 113,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 113,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 113,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 113,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 113,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 113,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 113,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 113,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 113,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 113,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 113,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 113,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 113,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 113,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 113,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 113,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 113,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 113,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 113,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 113,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 113,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 113,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 113,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 113,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 113,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 113,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 113,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 113,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 113,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 113,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 113,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 113,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 113,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 113,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 113,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 113,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 113,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 113,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 113,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 113,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 113,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 113,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.2,
            "owner_age": 113,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 113,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 113,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 113,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 113,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 113,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.0,
            "owner_age": 113,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.4,
            "owner_age": 113,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 113,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.3,
            "owner_age": 113,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.8,
            "owner_age": 113,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.3,
            "owner_age": 113,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 113,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.4,
            "owner_age": 113,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.1,
            "owner_age": 113,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.7,
            "owner_age": 113,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.3,
            "owner_age": 113,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.0,
            "owner_age": 113,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.7,
            "owner_age": 113,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.5,
            "owner_age": 113,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.2,
            "owner_age": 113,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.0,
            "owner_age": 113,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.8,
            "owner_age": 113,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.6,
            "owner_age": 113,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.5,
            "owner_age": 113,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.4,
            "owner_age": 113,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.2,
            "owner_age": 113,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.1,
            "owner_age": 113,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.1,
            "owner_age": 113,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.0,
            "owner_age": 113,
            "spouse_age": 105
          },
          {
            "distribution_period": 3.0,
            "owner_age": 113,
            "spouse_age": 106
          },
          {
            "distribution_period": 2.9,
            "owner_age": 113,
            "spouse_age": 107
          },
          {
            "distribution_period": 2.9,
            "owner_age": 113,
            "spouse_age": 108
          },
          {
            "distribution_period": 2.9,
            "owner_age": 113,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.9,
            "owner_age": 113,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.9,
            "owner_age": 113,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.9,
            "owner_age": 113,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.8,
            "owner_age": 113,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.8,
            "owner_age": 113,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.8,
            "owner_age": 113,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.7,
            "owner_age": 113,
            "spouse_age": 116
          },
          {
            "distribution_period": 2.6,
            "owner_age": 113,
            "spouse_age": 117
          },
          {
            "distribution_period": 2.4,
            "owner_age": 113,
            "spouse_age": 118
          },
          {
            "distribution_period": 2.2,
            "owner_age": 113,
            "spouse_age": 119
          },
          {
            "distribution_period": 1.9,
            "owner_age": 113,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 114,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 114,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 114,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 114,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 114,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 114,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 114,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 114,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 114,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 114,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 114,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 114,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 114,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 114,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 114,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 114,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 114,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 114,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 114,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 114,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 114,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 114,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 114,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 114,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 114,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 114,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 114,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 114,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 114,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 114,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 114,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 114,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 114,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 114,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 114,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 114,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 114,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 114,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 114,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 114,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 114,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 114,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 114,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 114,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 114,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 114,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 114,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 114,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 114,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 114,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 114,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 114,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 114,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 114,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 114,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 114,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.1,
            "owner_age": 114,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 114,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 114,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 114,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 114,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 114,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.0,
            "owner_age": 114,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.4,
            "owner_age": 114,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.9,
            "owner_age": 114,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.3,
            "owner_age": 114,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.8,
            "owner_age": 114,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.3,
            "owner_age": 114,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.9,
            "owner_age": 114,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.4,
            "owner_age": 114,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.0,
            "owner_age": 114,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.7,
            "owner_age": 114,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.3,
            "owner_age": 114,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.0,
            "owner_age": 114,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.7,
            "owner_age": 114,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.4,
            "owner_age": 114,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.2,
            "owner_age": 114,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.0,
            "owner_age": 114,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.8,
            "owner_age": 114,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.6,
            "owner_age": 114,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.5,
            "owner_age": 114,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.3,
            "owner_age": 114,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.2,
            "owner_age": 114,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.1,
            "owner_age": 114,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.0,
            "owner_age": 114,
            "spouse_age": 104
          },
          {
            "distribution_period": 3.0,
            "owner_age": 114,
            "spouse_age": 105
          },
          {
            "distribution_period": 2.9,
            "owner_age": 114,
            "spouse_age": 106
          },
          {
            "distribution_period": 2.9,
            "owner_age": 114,
            "spouse_age": 107
          },
          {
            "distribution_period": 2.9,
            "owner_age": 114,
            "spouse_age": 108
          },
          {
            "distribution_period": 2.9,
            "owner_age": 114,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.9,
            "owner_age": 114,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.8,
            "owner_age": 114,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.8,
            "owner_age": 114,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.8,
            "owner_age": 114,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.8,
            "owner_age": 114,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.7,
            "owner_age": 114,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.6,
            "owner_age": 114,
            "spouse_age": 116
          },
          {
            "distribution_period": 2.5,
            "owner_age": 114,
            "spouse_age": 117
          },
          {
            "distribution_period": 2.4,
            "owner_age": 114,
            "spouse_age": 118
          },
          {
            "distribution_period": 2.1,
            "owner_age": 114,
            "spouse_age": 119
          },
          {
            "distribution_period": 1.9,
            "owner_age": 114,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 115,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 115,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 115,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 115,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 115,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 115,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 115,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 115,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 115,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 115,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 115,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 115,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 115,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 115,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 115,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 115,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 115,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 115,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 115,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 115,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 115,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 115,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 115,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 115,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 115,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 115,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 115,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 115,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 115,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 115,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 115,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 115,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 115,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 115,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 115,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 115,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 115,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 115,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 115,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 115,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 115,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 115,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 115,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 115,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 115,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 115,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 115,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 115,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 115,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 115,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 115,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 115,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 115,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 115,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.7,
            "owner_age": 115,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 115,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.1,
            "owner_age": 115,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 115,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 115,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 115,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 115,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.7,
            "owner_age": 115,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.0,
            "owner_age": 115,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.4,
            "owner_age": 115,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.8,
            "owner_age": 115,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.3,
            "owner_age": 115,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.8,
            "owner_age": 115,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.3,
            "owner_age": 115,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.8,
            "owner_age": 115,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.4,
            "owner_age": 115,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.0,
            "owner_age": 115,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.6,
            "owner_age": 115,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.3,
            "owner_age": 115,
            "spouse_age": 92
          },
          {
            "distribution_period": 5.0,
            "owner_age": 115,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.7,
            "owner_age": 115,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.4,
            "owner_age": 115,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.2,
            "owner_age": 115,
            "spouse_age": 96
          },
          {
            "distribution_period": 4.0,
            "owner_age": 115,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.8,
            "owner_age": 115,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.6,
            "owner_age": 115,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.4,
            "owner_age": 115,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.3,
            "owner_age": 115,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.2,
            "owner_age": 115,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.1,
            "owner_age": 115,
            "spouse_age": 103
          },
          {
            "distribution_period": 3.0,
            "owner_age": 115,
            "spouse_age": 104
          },
          {
            "distribution_period": 2.9,
            "owner_age": 115,
            "spouse_age": 105
          },
          {
            "distribution_period": 2.9,
            "owner_age": 115,
            "spouse_age": 106
          },
          {
            "distribution_period": 2.9,
            "owner_age": 115,
            "spouse_age": 107
          },
          {
            "distribution_period": 2.8,
            "owner_age": 115,
            "spouse_age": 108
          },
          {
            "distribution_period": 2.8,
            "owner_age": 115,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.8,
            "owner_age": 115,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.8,
            "owner_age": 115,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.8,
            "owner_age": 115,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.8,
            "owner_age": 115,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.7,
            "owner_age": 115,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.7,
            "owner_age": 115,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.6,
            "owner_age": 115,
            "spouse_age": 116
          },
          {
            "distribution_period": 2.5,
            "owner_age": 115,
            "spouse_age": 117
          },
          {
            "distribution_period": 2.3,
            "owner_age": 115,
            "spouse_age": 118
          },
          {
            "distribution_period": 2.1,
            "owner_age": 115,
            "spouse_age": 119
          },
          {
            "distribution_period": 1.8,
            "owner_age": 115,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 116,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 116,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 116,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 116,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 116,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 116,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 116,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 116,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 116,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 116,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 116,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 116,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 116,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 116,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 116,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 116,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 116,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 116,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 116,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 116,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 116,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 116,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 116,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 116,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 116,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 116,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 116,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 116,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 116,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 116,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 116,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 116,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 116,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 116,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 116,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 116,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 116,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 116,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 116,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 116,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 116,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 116,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 116,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 116,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 116,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 116,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 116,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.3,
            "owner_age": 116,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 116,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 116,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 116,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 116,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 116,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 116,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.6,
            "owner_age": 116,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 116,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.1,
            "owner_age": 116,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 116,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 116,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 116,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 116,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.6,
            "owner_age": 116,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.0,
            "owner_age": 116,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.4,
            "owner_age": 116,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.8,
            "owner_age": 116,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.3,
            "owner_age": 116,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.7,
            "owner_age": 116,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.3,
            "owner_age": 116,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.8,
            "owner_age": 116,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.4,
            "owner_age": 116,
            "spouse_age": 89
          },
          {
            "distribution_period": 6.0,
            "owner_age": 116,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.6,
            "owner_age": 116,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.2,
            "owner_age": 116,
            "spouse_age": 92
          },
          {
            "distribution_period": 4.9,
            "owner_age": 116,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.6,
            "owner_age": 116,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.4,
            "owner_age": 116,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.1,
            "owner_age": 116,
            "spouse_age": 96
          },
          {
            "distribution_period": 3.9,
            "owner_age": 116,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.7,
            "owner_age": 116,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.5,
            "owner_age": 116,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.3,
            "owner_age": 116,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.2,
            "owner_age": 116,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.1,
            "owner_age": 116,
            "spouse_age": 102
          },
          {
            "distribution_period": 3.0,
            "owner_age": 116,
            "spouse_age": 103
          },
          {
            "distribution_period": 2.9,
            "owner_age": 116,
            "spouse_age": 104
          },
          {
            "distribution_period": 2.8,
            "owner_age": 116,
            "spouse_age": 105
          },
          {
            "distribution_period": 2.8,
            "owner_age": 116,
            "spouse_age": 106
          },
          {
            "distribution_period": 2.8,
            "owner_age": 116,
            "spouse_age": 107
          },
          {
            "distribution_period": 2.8,
            "owner_age": 116,
            "spouse_age": 108
          },
          {
            "distribution_period": 2.8,
            "owner_age": 116,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.7,
            "owner_age": 116,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.7,
            "owner_age": 116,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.7,
            "owner_age": 116,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.7,
            "owner_age": 116,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.6,
            "owner_age": 116,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.6,
            "owner_age": 116,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.5,
            "owner_age": 116,
            "spouse_age": 116
          },
          {
            "distribution_period": 2.4,
            "owner_age": 116,
            "spouse_age": 117
          },
          {
            "distribution_period": 2.2,
            "owner_age": 116,
            "spouse_age": 118
          },
          {
            "distribution_period": 2.0,
            "owner_age": 116,
            "spouse_age": 119
          },
          {
            "distribution_period": 1.8,
            "owner_age": 116,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 117,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 117,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 117,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 117,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 117,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 117,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 117,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 117,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 117,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 117,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 117,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 117,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 117,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 117,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 117,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 117,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 117,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 117,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 117,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 117,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 117,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 117,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 117,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 117,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 117,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 117,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 117,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 117,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 117,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 117,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 117,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 117,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 117,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 117,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 117,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 117,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 117,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 117,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 117,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 117,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 117,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 117,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 117,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.6,
            "owner_age": 117,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 117,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 117,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 117,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.2,
            "owner_age": 117,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 117,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 117,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 117,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 117,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 117,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 117,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.6,
            "owner_age": 117,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 117,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.1,
            "owner_age": 117,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 117,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.7,
            "owner_age": 117,
            "spouse_age": 78
          },
          {
            "distribution_period": 12.0,
            "owner_age": 117,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 117,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.6,
            "owner_age": 117,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.0,
            "owner_age": 117,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.4,
            "owner_age": 117,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.8,
            "owner_age": 117,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.2,
            "owner_age": 117,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.7,
            "owner_age": 117,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.2,
            "owner_age": 117,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.8,
            "owner_age": 117,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.3,
            "owner_age": 117,
            "spouse_age": 89
          },
          {
            "distribution_period": 5.9,
            "owner_age": 117,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.5,
            "owner_age": 117,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.2,
            "owner_age": 117,
            "spouse_age": 92
          },
          {
            "distribution_period": 4.9,
            "owner_age": 117,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.6,
            "owner_age": 117,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.3,
            "owner_age": 117,
            "spouse_age": 95
          },
          {
            "distribution_period": 4.0,
            "owner_age": 117,
            "spouse_age": 96
          },
          {
            "distribution_period": 3.8,
            "owner_age": 117,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.6,
            "owner_age": 117,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.4,
            "owner_age": 117,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.3,
            "owner_age": 117,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.1,
            "owner_age": 117,
            "spouse_age": 101
          },
          {
            "distribution_period": 3.0,
            "owner_age": 117,
            "spouse_age": 102
          },
          {
            "distribution_period": 2.9,
            "owner_age": 117,
            "spouse_age": 103
          },
          {
            "distribution_period": 2.8,
            "owner_age": 117,
            "spouse_age": 104
          },
          {
            "distribution_period": 2.7,
            "owner_age": 117,
            "spouse_age": 105
          },
          {
            "distribution_period": 2.7,
            "owner_age": 117,
            "spouse_age": 106
          },
          {
            "distribution_period": 2.7,
            "owner_age": 117,
            "spouse_age": 107
          },
          {
            "distribution_period": 2.7,
            "owner_age": 117,
            "spouse_age": 108
          },
          {
            "distribution_period": 2.6,
            "owner_age": 117,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.6,
            "owner_age": 117,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.6,
            "owner_age": 117,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.6,
            "owner_age": 117,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.6,
            "owner_age": 117,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.5,
            "owner_age": 117,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.5,
            "owner_age": 117,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.4,
            "owner_age": 117,
            "spouse_age": 116
          },
          {
            "distribution_period": 2.3,
            "owner_age": 117,
            "spouse_age": 117
          },
          {
            "distribution_period": 2.1,
            "owner_age": 117,
            "spouse_age": 118
          },
          {
            "distribution_period": 1.9,
            "owner_age": 117,
            "spouse_age": 119
          },
          {
            "distribution_period": 1.6,
            "owner_age": 117,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 118,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 118,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 118,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 118,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 118,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 118,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 118,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 118,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 118,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 118,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 118,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 118,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 118,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 118,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 118,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 118,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 118,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 118,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 118,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 118,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 118,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 118,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 118,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 118,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 118,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 118,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 118,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 118,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 118,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 118,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 118,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 118,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 118,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 118,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 118,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 118,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 118,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 118,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 118,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 118,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 118,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.3,
            "owner_age": 118,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 118,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.5,
            "owner_age": 118,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 118,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 118,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 118,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.2,
            "owner_age": 118,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 118,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 118,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 118,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 118,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 118,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 118,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.6,
            "owner_age": 118,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.9,
            "owner_age": 118,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.1,
            "owner_age": 118,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 118,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.6,
            "owner_age": 118,
            "spouse_age": 78
          },
          {
            "distribution_period": 11.9,
            "owner_age": 118,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.3,
            "owner_age": 118,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.6,
            "owner_age": 118,
            "spouse_age": 81
          },
          {
            "distribution_period": 10.0,
            "owner_age": 118,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.3,
            "owner_age": 118,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.8,
            "owner_age": 118,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.2,
            "owner_age": 118,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.7,
            "owner_age": 118,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.2,
            "owner_age": 118,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.7,
            "owner_age": 118,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.3,
            "owner_age": 118,
            "spouse_age": 89
          },
          {
            "distribution_period": 5.8,
            "owner_age": 118,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.5,
            "owner_age": 118,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.1,
            "owner_age": 118,
            "spouse_age": 92
          },
          {
            "distribution_period": 4.8,
            "owner_age": 118,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.5,
            "owner_age": 118,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.2,
            "owner_age": 118,
            "spouse_age": 95
          },
          {
            "distribution_period": 3.9,
            "owner_age": 118,
            "spouse_age": 96
          },
          {
            "distribution_period": 3.7,
            "owner_age": 118,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.5,
            "owner_age": 118,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.3,
            "owner_age": 118,
            "spouse_age": 99
          },
          {
            "distribution_period": 3.1,
            "owner_age": 118,
            "spouse_age": 100
          },
          {
            "distribution_period": 3.0,
            "owner_age": 118,
            "spouse_age": 101
          },
          {
            "distribution_period": 2.8,
            "owner_age": 118,
            "spouse_age": 102
          },
          {
            "distribution_period": 2.7,
            "owner_age": 118,
            "spouse_age": 103
          },
          {
            "distribution_period": 2.6,
            "owner_age": 118,
            "spouse_age": 104
          },
          {
            "distribution_period": 2.6,
            "owner_age": 118,
            "spouse_age": 105
          },
          {
            "distribution_period": 2.5,
            "owner_age": 118,
            "spouse_age": 106
          },
          {
            "distribution_period": 2.5,
            "owner_age": 118,
            "spouse_age": 107
          },
          {
            "distribution_period": 2.5,
            "owner_age": 118,
            "spouse_age": 108
          },
          {
            "distribution_period": 2.5,
            "owner_age": 118,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.5,
            "owner_age": 118,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.4,
            "owner_age": 118,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.4,
            "owner_age": 118,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.4,
            "owner_age": 118,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.4,
            "owner_age": 118,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.3,
            "owner_age": 118,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.2,
            "owner_age": 118,
            "spouse_age": 116
          },
          {
            "distribution_period": 2.1,
            "owner_age": 118,
            "spouse_age": 117
          },
          {
            "distribution_period": 1.9,
            "owner_age": 118,
            "spouse_age": 118
          },
          {
            "distribution_period": 1.7,
            "owner_age": 118,
            "spouse_age": 119
          },
          {
            "distribution_period": 1.4,
            "owner_age": 118,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 119,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 119,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 119,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 119,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 119,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 119,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 119,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 119,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 119,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 119,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 119,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 119,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 119,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 119,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 119,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 119,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 119,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 119,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 119,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 119,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 119,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 119,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 119,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 119,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 119,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 119,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 119,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 119,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 119,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 119,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 119,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 119,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 119,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 119,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 119,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 119,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.7,
            "owner_age": 119,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 119,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 119,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 119,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 119,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.2,
            "owner_age": 119,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 119,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.5,
            "owner_age": 119,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 119,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 119,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.1,
            "owner_age": 119,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.2,
            "owner_age": 119,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 119,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 119,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 119,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 119,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 119,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 119,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.6,
            "owner_age": 119,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.8,
            "owner_age": 119,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.1,
            "owner_age": 119,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.4,
            "owner_age": 119,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.6,
            "owner_age": 119,
            "spouse_age": 78
          },
          {
            "distribution_period": 11.9,
            "owner_age": 119,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.2,
            "owner_age": 119,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.6,
            "owner_age": 119,
            "spouse_age": 81
          },
          {
            "distribution_period": 9.9,
            "owner_age": 119,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.3,
            "owner_age": 119,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.7,
            "owner_age": 119,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.2,
            "owner_age": 119,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.6,
            "owner_age": 119,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.1,
            "owner_age": 119,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.6,
            "owner_age": 119,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.2,
            "owner_age": 119,
            "spouse_age": 89
          },
          {
            "distribution_period": 5.8,
            "owner_age": 119,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.4,
            "owner_age": 119,
            "spouse_age": 91
          },
          {
            "distribution_period": 5.0,
            "owner_age": 119,
            "spouse_age": 92
          },
          {
            "distribution_period": 4.7,
            "owner_age": 119,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.4,
            "owner_age": 119,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.1,
            "owner_age": 119,
            "spouse_age": 95
          },
          {
            "distribution_period": 3.8,
            "owner_age": 119,
            "spouse_age": 96
          },
          {
            "distribution_period": 3.6,
            "owner_age": 119,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.3,
            "owner_age": 119,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.1,
            "owner_age": 119,
            "spouse_age": 99
          },
          {
            "distribution_period": 2.9,
            "owner_age": 119,
            "spouse_age": 100
          },
          {
            "distribution_period": 2.8,
            "owner_age": 119,
            "spouse_age": 101
          },
          {
            "distribution_period": 2.6,
            "owner_age": 119,
            "spouse_age": 102
          },
          {
            "distribution_period": 2.5,
            "owner_age": 119,
            "spouse_age": 103
          },
          {
            "distribution_period": 2.4,
            "owner_age": 119,
            "spouse_age": 104
          },
          {
            "distribution_period": 2.4,
            "owner_age": 119,
            "spouse_age": 105
          },
          {
            "distribution_period": 2.3,
            "owner_age": 119,
            "spouse_age": 106
          },
          {
            "distribution_period": 2.3,
            "owner_age": 119,
            "spouse_age": 107
          },
          {
            "distribution_period": 2.3,
            "owner_age": 119,
            "spouse_age": 108
          },
          {
            "distribution_period": 2.3,
            "owner_age": 119,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.2,
            "owner_age": 119,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.2,
            "owner_age": 119,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.2,
            "owner_age": 119,
            "spouse_age": 112
          },
          {
            "distribution_period": 2.2,
            "owner_age": 119,
            "spouse_age": 113
          },
          {
            "distribution_period": 2.1,
            "owner_age": 119,
            "spouse_age": 114
          },
          {
            "distribution_period": 2.1,
            "owner_age": 119,
            "spouse_age": 115
          },
          {
            "distribution_period": 2.0,
            "owner_age": 119,
            "spouse_age": 116
          },
          {
            "distribution_period": 1.9,
            "owner_age": 119,
            "spouse_age": 117
          },
          {
            "distribution_period": 1.7,
            "owner_age": 119,
            "spouse_age": 118
          },
          {
            "distribution_period": 1.3,
            "owner_age": 119,
            "spouse_age": 119
          },
          {
            "distribution_period": 1.1,
            "owner_age": 119,
            "spouse_age": 120
          },
          {
            "distribution_period": 65.0,
            "owner_age": 120,
            "spouse_age": 20
          },
          {
            "distribution_period": 64.1,
            "owner_age": 120,
            "spouse_age": 21
          },
          {
            "distribution_period": 63.1,
            "owner_age": 120,
            "spouse_age": 22
          },
          {
            "distribution_period": 62.1,
            "owner_age": 120,
            "spouse_age": 23
          },
          {
            "distribution_period": 61.1,
            "owner_age": 120,
            "spouse_age": 24
          },
          {
            "distribution_period": 60.2,
            "owner_age": 120,
            "spouse_age": 25
          },
          {
            "distribution_period": 59.2,
            "owner_age": 120,
            "spouse_age": 26
          },
          {
            "distribution_period": 58.2,
            "owner_age": 120,
            "spouse_age": 27
          },
          {
            "distribution_period": 57.3,
            "owner_age": 120,
            "spouse_age": 28
          },
          {
            "distribution_period": 56.3,
            "owner_age": 120,
            "spouse_age": 29
          },
          {
            "distribution_period": 55.3,
            "owner_age": 120,
            "spouse_age": 30
          },
          {
            "distribution_period": 54.4,
            "owner_age": 120,
            "spouse_age": 31
          },
          {
            "distribution_period": 53.4,
            "owner_age": 120,
            "spouse_age": 32
          },
          {
            "distribution_period": 52.5,
            "owner_age": 120,
            "spouse_age": 33
          },
          {
            "distribution_period": 51.5,
            "owner_age": 120,
            "spouse_age": 34
          },
          {
            "distribution_period": 50.5,
            "owner_age": 120,
            "spouse_age": 35
          },
          {
            "distribution_period": 49.6,
            "owner_age": 120,
            "spouse_age": 36
          },
          {
            "distribution_period": 48.6,
            "owner_age": 120,
            "spouse_age": 37
          },
          {
            "distribution_period": 47.7,
            "owner_age": 120,
            "spouse_age": 38
          },
          {
            "distribution_period": 46.7,
            "owner_age": 120,
            "spouse_age": 39
          },
          {
            "distribution_period": 45.7,
            "owner_age": 120,
            "spouse_age": 40
          },
          {
            "distribution_period": 44.8,
            "owner_age": 120,
            "spouse_age": 41
          },
          {
            "distribution_period": 43.8,
            "owner_age": 120,
            "spouse_age": 42
          },
          {
            "distribution_period": 42.9,
            "owner_age": 120,
            "spouse_age": 43
          },
          {
            "distribution_period": 41.9,
            "owner_age": 120,
            "spouse_age": 44
          },
          {
            "distribution_period": 41.0,
            "owner_age": 120,
            "spouse_age": 45
          },
          {
            "distribution_period": 40.0,
            "owner_age": 120,
            "spouse_age": 46
          },
          {
            "distribution_period": 39.0,
            "owner_age": 120,
            "spouse_age": 47
          },
          {
            "distribution_period": 38.1,
            "owner_age": 120,
            "spouse_age": 48
          },
          {
            "distribution_period": 37.1,
            "owner_age": 120,
            "spouse_age": 49
          },
          {
            "distribution_period": 36.2,
            "owner_age": 120,
            "spouse_age": 50
          },
          {
            "distribution_period": 35.3,
            "owner_age": 120,
            "spouse_age": 51
          },
          {
            "distribution_period": 34.3,
            "owner_age": 120,
            "spouse_age": 52
          },
          {
            "distribution_period": 33.4,
            "owner_age": 120,
            "spouse_age": 53
          },
          {
            "distribution_period": 32.5,
            "owner_age": 120,
            "spouse_age": 54
          },
          {
            "distribution_period": 31.6,
            "owner_age": 120,
            "spouse_age": 55
          },
          {
            "distribution_period": 30.6,
            "owner_age": 120,
            "spouse_age": 56
          },
          {
            "distribution_period": 29.8,
            "owner_age": 120,
            "spouse_age": 57
          },
          {
            "distribution_period": 28.9,
            "owner_age": 120,
            "spouse_age": 58
          },
          {
            "distribution_period": 28.0,
            "owner_age": 120,
            "spouse_age": 59
          },
          {
            "distribution_period": 27.1,
            "owner_age": 120,
            "spouse_age": 60
          },
          {
            "distribution_period": 26.2,
            "owner_age": 120,
            "spouse_age": 61
          },
          {
            "distribution_period": 25.4,
            "owner_age": 120,
            "spouse_age": 62
          },
          {
            "distribution_period": 24.5,
            "owner_age": 120,
            "spouse_age": 63
          },
          {
            "distribution_period": 23.7,
            "owner_age": 120,
            "spouse_age": 64
          },
          {
            "distribution_period": 22.9,
            "owner_age": 120,
            "spouse_age": 65
          },
          {
            "distribution_period": 22.0,
            "owner_age": 120,
            "spouse_age": 66
          },
          {
            "distribution_period": 21.2,
            "owner_age": 120,
            "spouse_age": 67
          },
          {
            "distribution_period": 20.4,
            "owner_age": 120,
            "spouse_age": 68
          },
          {
            "distribution_period": 19.6,
            "owner_age": 120,
            "spouse_age": 69
          },
          {
            "distribution_period": 18.8,
            "owner_age": 120,
            "spouse_age": 70
          },
          {
            "distribution_period": 18.0,
            "owner_age": 120,
            "spouse_age": 71
          },
          {
            "distribution_period": 17.2,
            "owner_age": 120,
            "spouse_age": 72
          },
          {
            "distribution_period": 16.4,
            "owner_age": 120,
            "spouse_age": 73
          },
          {
            "distribution_period": 15.6,
            "owner_age": 120,
            "spouse_age": 74
          },
          {
            "distribution_period": 14.8,
            "owner_age": 120,
            "spouse_age": 75
          },
          {
            "distribution_period": 14.1,
            "owner_age": 120,
            "spouse_age": 76
          },
          {
            "distribution_period": 13.3,
            "owner_age": 120,
            "spouse_age": 77
          },
          {
            "distribution_period": 12.6,
            "owner_age": 120,
            "spouse_age": 78
          },
          {
            "distribution_period": 11.9,
            "owner_age": 120,
            "spouse_age": 79
          },
          {
            "distribution_period": 11.2,
            "owner_age": 120,
            "spouse_age": 80
          },
          {
            "distribution_period": 10.5,
            "owner_age": 120,
            "spouse_age": 81
          },
          {
            "distribution_period": 9.9,
            "owner_age": 120,
            "spouse_age": 82
          },
          {
            "distribution_period": 9.3,
            "owner_age": 120,
            "spouse_age": 83
          },
          {
            "distribution_period": 8.7,
            "owner_age": 120,
            "spouse_age": 84
          },
          {
            "distribution_period": 8.1,
            "owner_age": 120,
            "spouse_age": 85
          },
          {
            "distribution_period": 7.6,
            "owner_age": 120,
            "spouse_age": 86
          },
          {
            "distribution_period": 7.1,
            "owner_age": 120,
            "spouse_age": 87
          },
          {
            "distribution_period": 6.6,
            "owner_age": 120,
            "spouse_age": 88
          },
          {
            "distribution_period": 6.1,
            "owner_age": 120,
            "spouse_age": 89
          },
          {
            "distribution_period": 5.7,
            "owner_age": 120,
            "spouse_age": 90
          },
          {
            "distribution_period": 5.3,
            "owner_age": 120,
            "spouse_age": 91
          },
          {
            "distribution_period": 4.9,
            "owner_age": 120,
            "spouse_age": 92
          },
          {
            "distribution_period": 4.6,
            "owner_age": 120,
            "spouse_age": 93
          },
          {
            "distribution_period": 4.3,
            "owner_age": 120,
            "spouse_age": 94
          },
          {
            "distribution_period": 4.0,
            "owner_age": 120,
            "spouse_age": 95
          },
          {
            "distribution_period": 3.7,
            "owner_age": 120,
            "spouse_age": 96
          },
          {
            "distribution_period": 3.4,
            "owner_age": 120,
            "spouse_age": 97
          },
          {
            "distribution_period": 3.2,
            "owner_age": 120,
            "spouse_age": 98
          },
          {
            "distribution_period": 3.0,
            "owner_age": 120,
            "spouse_age": 99
          },
          {
            "distribution_period": 2.8,
            "owner_age": 120,
            "spouse_age": 100
          },
          {
            "distribution_period": 2.6,
            "owner_age": 120,
            "spouse_age": 101
          },
          {
            "distribution_period": 2.5,
            "owner_age": 120,
            "spouse_age": 102
          },
          {
            "distribution_period": 2.3,
            "owner_age": 120,
            "spouse_age": 103
          },
          {
            "distribution_period": 2.2,
            "owner_age": 120,
            "spouse_age": 104
          },
          {
            "distribution_period": 2.1,
            "owner_age": 120,
            "spouse_age": 105
          },
          {
            "distribution_period": 2.1,
            "owner_age": 120,
            "spouse_age": 106
          },
          {
            "distribution_period": 2.1,
            "owner_age": 120,
            "spouse_age": 107
          },
          {
            "distribution_period": 2.0,
            "owner_age": 120,
            "spouse_age": 108
          },
          {
            "distribution_period": 2.0,
            "owner_age": 120,
            "spouse_age": 109
          },
          {
            "distribution_period": 2.0,
            "owner_age": 120,
            "spouse_age": 110
          },
          {
            "distribution_period": 2.0,
            "owner_age": 120,
            "spouse_age": 111
          },
          {
            "distribution_period": 2.0,
            "owner_age": 120,
            "spouse_age": 112
          },
          {
            "distribution_period": 1.9,
            "owner_age": 120,
            "spouse_age": 113
          },
          {
            "distribution_period": 1.9,
            "owner_age": 120,
            "spouse_age": 114
          },
          {
            "distribution_period": 1.8,
            "owner_age": 120,
            "spouse_age": 115
          },
          {
            "distribution_period": 1.8,
            "owner_age": 120,
            "spouse_age": 116
          },
          {
            "distribution_period": 1.6,
            "owner_age": 120,
            "spouse_age": 117
          },
          {
            "distribution_period": 1.4,
            "owner_age": 120,
            "spouse_age": 118
          },
          {
            "distribution_period": 1.1,
            "owner_age": 120,
            "spouse_age": 119
          },
          {
            "distribution_period": 1.0,
            "owner_age": 120,
            "spouse_age": 120
          }
        ]
      }
    ]
  }
}

```

## Review Notes

- Primary source is IRS Publication 590-B, Appendix B Table II.
- Canonical engine source: engine/src/data/retirement/rmd_tables.rs
