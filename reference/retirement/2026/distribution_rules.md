---
category: retirement
year: 2026
key: distribution_rules
title: Distribution Rules
verification_status: authoritative
review_status: reviewed
source_registry: engine/data_registry/2026/reviewed/retirement/distribution_rules.json
---

# Distribution Rules

2026 RMD distribution rules covering required beginning age, account applicability, beneficiary routing, the ten-year rule, relief years, and the pre-1987 403(b) exclusion.

## Machine Block

```json
{
  "schema_version": 1,
  "category": "retirement",
  "key": "distribution_rules",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_pub590b",
      "url": "https://www.irs.gov/publications/p590b",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Publication 590-B (2025), Distributions from Individual Retirement Arrangements (IRAs)",
      "published_at": "2025",
      "locator": "Full publication; sections on required beginning date, beneficiary distributions, eligible designated beneficiaries, 10-year rule, 5-year rule, spouse provisions",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_rmd_topics",
      "url": "https://www.irs.gov/retirement-plans/plan-participant-employee/retirement-topics-required-minimum-distributions-rmds",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Retirement Topics - Required Minimum Distributions (RMDs)",
      "published_at": "2024",
      "locator": "Overview page listing account types, age 73 start age, still-working exception, Roth IRA and designated Roth account exemptions",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_rmd_faq",
      "url": "https://www.irs.gov/retirement-plans/retirement-plan-and-ira-required-minimum-distributions-faqs",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Retirement Plan and IRA Required Minimum Distributions FAQs",
      "published_at": "2024",
      "locator": "FAQ entries on account types, pre-1987 403(b) exclusion (age 75), designated Roth exemption",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_rmd_comparison",
      "url": "https://www.irs.gov/retirement-plans/rmd-comparison-chart-iras-vs-defined-contribution-plans",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "RMD Comparison Chart (IRAs vs. Defined Contribution Plans)",
      "published_at": "2024",
      "locator": "Comparison table: still-working exception for DC plans, 5% owner rule, Roth IRA and designated Roth exemption, April 1 deadline",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_td10001",
      "url": "https://www.irs.gov/irb/2024-33_IRB",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service / Department of the Treasury",
      "source_class": "primary",
      "title": "Internal Revenue Bulletin 2024-33: TD 10001 — Required Minimum Distributions",
      "published_at": "2024-07-19",
      "locator": "TD 10001; 89 FR 58886; §1.401(a)(9)-2 (applicable age), §1.401(a)(9)-4 (designated beneficiaries, EDB classes, minor child age 21), §1.401(a)(9)-5 (10-year rule annual distribution requirement), §402A(d) (designated Roth exemption)",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_notice_2024_35",
      "url": "https://www.irs.gov/pub/irs-drop/n-24-35.pdf",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Notice 2024-35: Certain Required Minimum Distributions for 2024",
      "published_at": "2024-04-16",
      "locator": "Section III: extends excise tax waiver to 2024 for 10-year rule annual RMDs; references Notice 2022-53 (2021–2022 relief) and Notice 2023-54 (2023 relief)",
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
          "account_applicability": {
            "designated_roth_owner_exemption_effective_year": 2024,
            "inherited_account_types": [
              "inherited_ira",
              "inherited_roth_ira",
              "inherited_401k",
              "inherited_roth_401k"
            ],
            "owner_exempt_account_types": [
              "roth_ira",
              "designated_roth_plan_account"
            ],
            "owner_required_account_types": [
              "traditional_ira",
              "sep_ira",
              "simple_ira",
              "401k",
              "403b",
              "457b"
            ],
            "pre_1987_403b": {
              "exclude_until_age": 75
            },
            "supports_pre_1987_403b_exclusion": true
          },
          "beneficiary_distribution": {
            "beneficiary_categories": [
              "eligible_designated_beneficiary",
              "designated_beneficiary",
              "non_designated_beneficiary"
            ],
            "eligible_designated_beneficiary_classes": [
              "spouse",
              "minor_child",
              "disabled",
              "chronically_ill",
              "not_more_than_10_years_younger"
            ],
            "life_expectancy_method_by_class": {
              "chronically_ill": "single_life_non_recalculated",
              "disabled": "single_life_non_recalculated",
              "minor_child": "single_life_non_recalculated",
              "not_more_than_10_years_younger": "single_life_non_recalculated",
              "other_designated_beneficiary": "ten_year_rule",
              "spouse": "single_life_recalculated"
            },
            "minor_child_majority_age": 21,
            "non_designated_beneficiary_rules": {
              "when_owner_died_before_required_beginning_date": "five_year_rule",
              "when_owner_died_on_or_after_required_beginning_date": "owner_remaining_life_expectancy"
            },
            "recognized_beneficiary_classes": [
              "spouse",
              "minor_child",
              "disabled",
              "chronically_ill",
              "not_more_than_10_years_younger",
              "other_designated_beneficiary",
              "non_designated_beneficiary"
            ],
            "relief_years": [
              2021,
              2022,
              2023,
              2024
            ],
            "spouse_delay_allowed": true,
            "ten_year_rule": {
              "annual_distributions_required_when_owner_died_on_or_after_rbd": true,
              "terminal_year": 10
            }
          },
          "required_beginning": {
            "first_distribution_deadline": "april_1_following_year",
            "start_age_rules": [
              {
                "birth_year_max": 1950,
                "birth_year_min": null,
                "guidance_status": "final",
                "notes": "Operational 2026 bucket for births in or before 1950. IRS legacy guidance distinguishes people born before July 1, 1949 (age 70½) from people born on or after July 1, 1949 but before January 1, 1951 (age 72); that older cutoff is carried in this note rather than split into a separate public rule row.",
                "start_age": 72
              },
              {
                "birth_year_max": 1958,
                "birth_year_min": 1951,
                "guidance_status": "final",
                "notes": null,
                "start_age": 73
              },
              {
                "birth_year_max": 1959,
                "birth_year_min": 1959,
                "guidance_status": "interim_good_faith",
                "notes": "IRS final regulations (TD 10001, 89 FR 58886) reserved the 1959 cohort; proposed regulations (89 FR 58644) specify applicable age 73. No final IRS resolution for that specific paragraph was found in the cited primary sources as of this extraction, so age 73 is carried here as the best supported proposed-rule reading with interim_good_faith status.",
                "start_age": 73
              },
              {
                "birth_year_max": null,
                "birth_year_min": 1960,
                "guidance_status": "final",
                "notes": null,
                "start_age": 75
              }
            ],
            "still_working_exception": {
              "disallowed_for_five_percent_owners": true,
              "eligible_account_types": [
                "401k",
                "403b"
              ],
              "eligible_plan_categories": [
                "401k",
                "403b",
                "profit_sharing",
                "other_defined_contribution_plan"
              ]
            }
          }
        }
      }
    ]
  }
}

```

## Review Notes

- Primary sources are IRS Publication 590-B, IRS RMD guidance pages, TD 10001, and Notice 2024-35.
- Canonical engine source: engine/src/data/retirement/rmd_rules.rs
