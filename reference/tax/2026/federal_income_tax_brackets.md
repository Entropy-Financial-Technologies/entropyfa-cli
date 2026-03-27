---
category: tax
year: 2026
key: federal_income_tax_brackets
title: Federal Income Tax Brackets
reviewed_artifact: tax/2026/federal_income_tax_brackets
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Income Tax Brackets

## What This Is

Federal ordinary income tax rate schedules for tax year 2026, containing the seven marginal bracket boundaries and rates for each of the five filing statuses, as published in IRS Revenue Procedure 2025-32 and reflecting the permanent TCJA rate structure enacted by the One Big Beautiful Bill Act.

## Lookup Parameters

- filing_status: one of single, married_filing_jointly, married_filing_separately, head_of_household, qualifying_surviving_spouse

## Interpretation Notes

- Each bracket object contains min (inclusive), max (exclusive upper bound or null for the top bracket), and rate (decimal, e.g. 0.10 = 10%).
- Brackets are marginal: only the income within each range is taxed at that rate, not the taxpayer's entire income.
- The qualifying_surviving_spouse variant uses the same thresholds as married_filing_jointly per IRC §1(a).
- The married_filing_separately variant mirrors single thresholds through the 32% bracket but has a lower 35%→37% boundary ($384,350 vs. $640,600).
- Head of household has wider 10% and 12% brackets than single but converges to near-identical thresholds from the 22% bracket onward, with minor rounding differences at the 24%→32% and 32%→35% boundaries.

## Does Not Include

- Long-term capital gains tax brackets (separate dataset)
- Net Investment Income Tax (NIIT) of 3.8%
- Alternative Minimum Tax (AMT) rate structure and exemptions
- Standard deduction amounts
- Effective or average tax rate calculations
- State or local income tax brackets
- Self-employment tax rates

## Caveats

- These brackets apply to ordinary taxable income only; long-term capital gains and qualified dividends are taxed under a separate rate schedule.
- The One Big Beautiful Bill Act (OBBBA), signed July 2025, made permanent the TCJA individual rate structure and added an enhanced inflation adjustment for the bottom two brackets (10% and 12%) using a 4% adjustment vs. 2.3% for higher brackets.
- Bracket thresholds are rounded to the nearest $25 (single/MFS/HoH) or $50 (MFJ/QSS) per IRC §1(j) rounding rules, which causes minor inter-status asymmetries (e.g., HoH 24%→32% boundary at $201,750 vs. single at $201,775).

## Typical Uses

- Computing federal income tax liability on ordinary taxable income
- Projecting marginal rates for income planning and Roth conversion analysis
- Tax-bracket-aware withdrawal sequencing in retirement income planning

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_income_tax_brackets",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_newsroom_1",
      "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill",
      "published_at": "2025-10-01",
      "locator": "Tax rate schedule table showing single and MFJ bracket thresholds",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_revproc_1",
      "url": "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Revenue Procedure 2025-32",
      "published_at": "2025-10-01",
      "locator": "Section 4.01, Tables 1–4: Tax Rate Tables for taxable years beginning in 2026",
      "counts_toward_status": true
    },
    {
      "source_id": "src_taxfoundation_1",
      "url": "https://taxfoundation.org/data/all/federal/2026-tax-brackets/",
      "host": "taxfoundation.org",
      "organization": "Tax Foundation",
      "source_class": "secondary",
      "title": "2026 Tax Brackets and Federal Income Tax Rates",
      "published_at": "2025-10-01",
      "locator": "Table 1: 2026 Federal Income Tax Brackets and Rates",
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
        "value": [
          {
            "max": 12400.0,
            "min": 0.0,
            "rate": 0.1
          },
          {
            "max": 50400.0,
            "min": 12400.0,
            "rate": 0.12
          },
          {
            "max": 105700.0,
            "min": 50400.0,
            "rate": 0.22
          },
          {
            "max": 201775.0,
            "min": 105700.0,
            "rate": 0.24
          },
          {
            "max": 256225.0,
            "min": 201775.0,
            "rate": 0.32
          },
          {
            "max": 640600.0,
            "min": 256225.0,
            "rate": 0.35
          },
          {
            "max": null,
            "min": 640600.0,
            "rate": 0.37
          }
        ]
      },
      {
        "label": "married_filing_jointly",
        "params": {
          "filing_status": "married_filing_jointly",
          "lived_with_spouse_during_year": null
        },
        "value": [
          {
            "max": 24800.0,
            "min": 0.0,
            "rate": 0.1
          },
          {
            "max": 100800.0,
            "min": 24800.0,
            "rate": 0.12
          },
          {
            "max": 211400.0,
            "min": 100800.0,
            "rate": 0.22
          },
          {
            "max": 403550.0,
            "min": 211400.0,
            "rate": 0.24
          },
          {
            "max": 512450.0,
            "min": 403550.0,
            "rate": 0.32
          },
          {
            "max": 768700.0,
            "min": 512450.0,
            "rate": 0.35
          },
          {
            "max": null,
            "min": 768700.0,
            "rate": 0.37
          }
        ]
      },
      {
        "label": "married_filing_separately",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": null
        },
        "value": [
          {
            "max": 12400.0,
            "min": 0.0,
            "rate": 0.1
          },
          {
            "max": 50400.0,
            "min": 12400.0,
            "rate": 0.12
          },
          {
            "max": 105700.0,
            "min": 50400.0,
            "rate": 0.22
          },
          {
            "max": 201775.0,
            "min": 105700.0,
            "rate": 0.24
          },
          {
            "max": 256225.0,
            "min": 201775.0,
            "rate": 0.32
          },
          {
            "max": 384350.0,
            "min": 256225.0,
            "rate": 0.35
          },
          {
            "max": null,
            "min": 384350.0,
            "rate": 0.37
          }
        ]
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": [
          {
            "max": 17700.0,
            "min": 0.0,
            "rate": 0.1
          },
          {
            "max": 67450.0,
            "min": 17700.0,
            "rate": 0.12
          },
          {
            "max": 105700.0,
            "min": 67450.0,
            "rate": 0.22
          },
          {
            "max": 201750.0,
            "min": 105700.0,
            "rate": 0.24
          },
          {
            "max": 256200.0,
            "min": 201750.0,
            "rate": 0.32
          },
          {
            "max": 640600.0,
            "min": 256200.0,
            "rate": 0.35
          },
          {
            "max": null,
            "min": 640600.0,
            "rate": 0.37
          }
        ]
      },
      {
        "label": "qualifying_surviving_spouse",
        "params": {
          "filing_status": "qualifying_surviving_spouse",
          "lived_with_spouse_during_year": null
        },
        "value": [
          {
            "max": 24800.0,
            "min": 0.0,
            "rate": 0.1
          },
          {
            "max": 100800.0,
            "min": 24800.0,
            "rate": 0.12
          },
          {
            "max": 211400.0,
            "min": 100800.0,
            "rate": 0.22
          },
          {
            "max": 403550.0,
            "min": 211400.0,
            "rate": 0.24
          },
          {
            "max": 512450.0,
            "min": 403550.0,
            "rate": 0.32
          },
          {
            "max": 768700.0,
            "min": 512450.0,
            "rate": 0.35
          },
          {
            "max": null,
            "min": 768700.0,
            "rate": 0.37
          }
        ]
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill — https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill
- Internal Revenue Service — Revenue Procedure 2025-32 — https://www.irs.gov/pub/irs-drop/rp-25-32.pdf
- Tax Foundation — 2026 Tax Brackets and Federal Income Tax Rates — https://taxfoundation.org/data/all/federal/2026-tax-brackets/
