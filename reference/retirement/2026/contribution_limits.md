---
category: retirement
year: 2026
key: contribution_limits
title: Contribution Limits
reviewed_artifact: retirement/2026/contribution_limits
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Contribution Limits

## What This Is

IRS cost-of-living adjusted dollar limits for retirement plan contributions, deferrals, catch-up contributions, and related thresholds for calendar year 2026, as published in IRS Notice 2025-67.

## Lookup Parameters

- No variant parameters are required; a single 'default' variant contains all 15 limits for calendar year 2026.

## Interpretation Notes

- All values are annual dollar limits for calendar year 2026.
- elective_deferral_401k is the employee elective deferral ceiling for 401(k) and 403(b) plans under IRC §402(g) and governmental 457(b) plans under IRC §457(e)(15); all three share the same dollar limit ($24,500 for 2026).
- catch_up amounts are additional amounts above the base deferral; the total permitted contribution is base + applicable catch-up.
- Age-based catch-up tiers are mutually exclusive for a given tax year: a participant qualifying for catch_up_60_to_63 uses that amount instead of catch_up_50_plus, not in addition to it.
- ira_contribution_limit is the combined ceiling across all traditional and Roth IRAs held by one individual.
- sep_minimum_compensation ($800) is the threshold below which an employer may exclude an employee from SEP participation under IRC §408(k)(2)(C).
- highly_compensated_threshold ($160,000) is the prior-year compensation test for HCE status under IRC §414(q); it applies to plan years beginning in 2026 based on 2025 compensation.
- key_employee_threshold ($235,000) is the officer compensation threshold under IRC §416(i)(1)(A)(i) for top-heavy testing.

## Does Not Include

- IRA deductibility AGI phase-out ranges (filing-status-dependent; tracked separately).
- Roth IRA income eligibility phase-out ranges.
- Saver's Credit income thresholds.
- SIMPLE plan enhanced deferral limit for employers with 25 or fewer employees ($18,100 for 2026).
- Employer matching or non-elective contribution formulas.
- State-level retirement mandate thresholds.

## Caveats

- catch_up_401k_60_to_63 and simple_catch_up_60_to_63 are SECURE 2.0 Act provisions (§109) effective for tax years beginning after 2024; they apply only to participants who attain ages 60-63 during the calendar year.
- For 2026 and later, catch-up contributions by participants earning more than $145,000 (indexed) in FICA wages from the employer in the prior year must be designated Roth under SECURE 2.0 §603.
- sep_maximum_contribution and annual_additions_limit_415c are both the IRC §415(c) limit ($72,000 for 2026); they are the same statutory cap but serve different plan contexts.
- The IRA catch-up amount ($1,100) is now indexed to inflation under SECURE 2.0 §108, effective for tax years beginning after 2023.

## Typical Uses

- Determining the maximum elective deferral a participant may make to a 401(k)/403(b)/457(b) plan in 2026.
- Computing the total permitted contribution (base + catch-up) by age tier.
- Setting the employer contribution ceiling for SEP and defined contribution plans under §415(c).
- Applying the annual compensation cap when calculating employer contributions.
- Identifying HCE and key-employee status for nondiscrimination and top-heavy testing.

## Machine Block

```json
{
  "schema_version": 1,
  "category": "retirement",
  "key": "contribution_limits",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_cola_1",
      "url": "https://www.irs.gov/retirement-plans/cola-increases-for-dollar-limitations-on-benefits-and-contributions",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "COLA Increases for Dollar Limitations on Benefits and Contributions",
      "published_at": "2025-11-01",
      "locator": "Multi-year COLA table with per-limit rows; 2026 column",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_newsroom_1",
      "url": "https://www.irs.gov/newsroom/401k-limit-increases-to-24500-for-2026-ira-limit-increases-to-7500",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "401(k) limit increases to $24,500 for 2026, IRA limit increases to $7,500",
      "published_at": "2025-11-01",
      "locator": "Newsroom press release with narrative descriptions of key 2026 limits",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_notice_1",
      "url": "https://www.irs.gov/pub/irs-drop/n-25-67.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Notice 2025-67: 2026 Amounts Relating to Retirement Plans and IRAs",
      "published_at": "2025-11-01",
      "locator": "Full text of IRS Notice 2025-67 (PDF)",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_1",
      "url": "https://www.kiplinger.com/retirement/sep-ira/sep-ira-limits",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "SEP IRA Contribution Limits for 2026",
      "published_at": "2025-11-01",
      "locator": "Article headline and body confirming $72,000 SEP maximum for 2026",
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
        "value": {
          "annual_additions_limit_415c": 72000.0,
          "annual_compensation_limit": 360000.0,
          "catch_up_401k_50_plus": 8000.0,
          "catch_up_401k_60_to_63": 11250.0,
          "defined_benefit_limit": 290000.0,
          "elective_deferral_401k": 24500.0,
          "highly_compensated_threshold": 160000.0,
          "ira_catch_up_50_plus": 1100.0,
          "ira_contribution_limit": 7500.0,
          "key_employee_threshold": 235000.0,
          "sep_maximum_contribution": 72000.0,
          "sep_minimum_compensation": 800.0,
          "simple_catch_up_50_plus": 4000.0,
          "simple_catch_up_60_to_63": 5250.0,
          "simple_elective_deferral": 17000.0
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — COLA Increases for Dollar Limitations on Benefits and Contributions — https://www.irs.gov/retirement-plans/cola-increases-for-dollar-limitations-on-benefits-and-contributions
- Internal Revenue Service — 401(k) limit increases to $24,500 for 2026, IRA limit increases to $7,500 — https://www.irs.gov/newsroom/401k-limit-increases-to-24500-for-2026-ira-limit-increases-to-7500
- Internal Revenue Service — Notice 2025-67: 2026 Amounts Relating to Retirement Plans and IRAs — https://www.irs.gov/pub/irs-drop/n-25-67.pdf
- Kiplinger — SEP IRA Contribution Limits for 2026 — https://www.kiplinger.com/retirement/sep-ira/sep-ira-limits
