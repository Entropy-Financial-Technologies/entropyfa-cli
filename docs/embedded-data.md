# Embedded Data Reference

This is the user-facing reference for `entropyfa data lookup`.

Examples below are drawn from the current reviewed 2026 embedded dataset. Output is pretty-printed for readability, and large arrays are abridged with `...`, but the field names and sample values come from the real CLI responses.

If you want the maintainer workflow that produces these reviewed artifacts, see [data-pipeline.md](data-pipeline.md).

## Response Envelope

Every lookup returns the same top-level shape:

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_income_tax_brackets",
    "year": 2026,
    "verification_status": "authoritative",
    "completeness": "full",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "counts_toward_status": true,
        "locator": "Section 3, Tax Rate Tables for Taxable Years Beginning in 2026",
        "published_at": "2025-10-09",
        "source_class": "primary",
        "title": "Revenue Procedure 2025-32 (26 CFR 601.602)",
        "url": "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf"
      }
    ],
    "value": []
  }
}
```

Field meanings:

- `verification_status`: current trust status for the embedded entry
- `completeness`: whether the entry is a full dataset or a subset
- `pipeline_reviewed`: whether the current embedded value came through the verification pipeline
- `source_origin`: where the provenance metadata came from, usually `reviewed_artifact`
- `sources`: source URLs and metadata returned by default
- `value`: the actual reference data payload for the requested key

## Endpoint Index

| Category | Key | Required Params | Typical Example |
| --- | --- | --- | --- |
| `tax` | `federal_income_tax_brackets` | `filing_status` | `entropyfa data lookup --category tax --key federal_income_tax_brackets --year 2026 --filing-status single` |
| `tax` | `federal_standard_deductions` | `filing_status` | `entropyfa data lookup --category tax --key federal_standard_deductions --year 2026 --filing-status single` |
| `tax` | `federal_salt_deduction_parameters` | `filing_status` | `entropyfa data lookup --category tax --key federal_salt_deduction_parameters --year 2026 --filing-status single` |
| `tax` | `federal_capital_gains_brackets` | `filing_status` | `entropyfa data lookup --category tax --key federal_capital_gains_brackets --year 2026 --filing-status single` |
| `tax` | `federal_net_investment_income_tax` | `filing_status` | `entropyfa data lookup --category tax --key federal_net_investment_income_tax --year 2026 --filing-status single` |
| `tax` | `federal_payroll_tax_parameters` | `filing_status` | `entropyfa data lookup --category tax --key federal_payroll_tax_parameters --year 2026 --filing-status single` |
| `tax` | `federal_capital_loss_limit` | `filing_status` | `entropyfa data lookup --category tax --key federal_capital_loss_limit --year 2026 --filing-status single` |
| `tax` | `federal_qbi_deduction` | `filing_status` | `entropyfa data lookup --category tax --key federal_qbi_deduction --year 2026 --filing-status single` |
| `tax` | `federal_estate_exemption` | none | `entropyfa data lookup --category tax --key federal_estate_exemption --year 2026` |
| `tax` | `federal_estate_brackets` | none | `entropyfa data lookup --category tax --key federal_estate_brackets --year 2026` |
| `tax` | `federal_estate_applicable_credit` | none | `entropyfa data lookup --category tax --key federal_estate_applicable_credit --year 2026` |
| `retirement` | `uniform_lifetime_table` | none | `entropyfa data lookup --category retirement --key uniform_lifetime_table --year 2026` |
| `retirement` | `single_life_table` | none | `entropyfa data lookup --category retirement --key single_life_table --year 2026` |
| `retirement` | `joint_life_table` | none | `entropyfa data lookup --category retirement --key joint_life_table --year 2026` |
| `retirement` | `distribution_rules` | none | `entropyfa data lookup --category retirement --key distribution_rules --year 2026` |
| `social_security` | `benefit_taxation_thresholds` | `filing_status`; `lived_with_spouse_during_year` for `married_filing_separately` | `entropyfa data lookup --category social_security --key benefit_taxation_thresholds --year 2026 --filing-status married_filing_separately --lived-with-spouse-during-year true` |
| `insurance` | `irmaa_brackets` | `filing_status`; `lived_with_spouse_during_year` for `married_filing_separately` | `entropyfa data lookup --category insurance --key irmaa_brackets --year 2026 --filing-status married_filing_separately --lived-with-spouse-during-year true` |
| `pension` | `mortality_417e` | none | `entropyfa data lookup --category pension --key mortality_417e --year 2026` |

## Tax

### `tax/federal_income_tax_brackets`

```sh
entropyfa data lookup --category tax --key federal_income_tax_brackets --year 2026 --filing-status single
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_income_tax_brackets",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Revenue Procedure 2025-32 (26 CFR 601.602)",
        "url": "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf"
      }
    ],
    "value": [
      { "min": 0.0, "max": 12400.0, "rate": 0.1 },
      { "min": 12400.0, "max": 50400.0, "rate": 0.12 },
      { "min": 50400.0, "max": 105700.0, "rate": 0.22 },
      { "min": 105700.0, "max": 201775.0, "rate": 0.24 },
      { "min": 201775.0, "max": 256225.0, "rate": 0.32 },
      { "min": 256225.0, "max": 640600.0, "rate": 0.35 },
      { "min": 640600.0, "max": null, "rate": 0.37 }
    ]
  }
}
```

### `tax/federal_standard_deductions`

```sh
entropyfa data lookup --category tax --key federal_standard_deductions --year 2026 --filing-status single
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_standard_deductions",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill",
        "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill"
      }
    ],
    "value": {
      "filing_status": "single",
      "amount": 16100.0
    }
  }
}
```

### `tax/federal_salt_deduction_parameters`

```sh
entropyfa data lookup --category tax --key federal_salt_deduction_parameters --year 2026 --filing-status single
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_salt_deduction_parameters",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "counts_toward_status": true,
        "locator": "Corrected 2026 state and local income tax deduction amounts by filing status",
        "published_at": "2026-03-16",
        "source_class": "primary",
        "title": "Correction to State and Local Income Tax Deduction Amount in the 2026 Form 1040-ES",
        "url": "https://www.irs.gov/forms-pubs/correction-to-state-and-local-income-tax-deduction-amount-in-the-2026-form-1040-es"
      },
      {
        "authority": "Internal Revenue Service",
        "counts_toward_status": true,
        "locator": "Line 5e worksheet: 30% reduction above modified AGI threshold, but not below the statutory floor",
        "published_at": "2025-12",
        "source_class": "primary",
        "title": "2025 Instructions for Schedule A (Form 1040)",
        "url": "https://www.irs.gov/instructions/i1040sca"
      }
    ],
    "value": {
      "cap_amount": 40400.0,
      "filing_status": "single",
      "floor_amount": 10000.0,
      "phaseout_rate": 0.3,
      "phaseout_threshold": 505000.0
    }
  }
}
```

### `tax/federal_capital_gains_brackets`

```sh
entropyfa data lookup --category tax --key federal_capital_gains_brackets --year 2026 --filing-status single
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_capital_gains_brackets",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Revenue Procedure 2025-32: Inflation Adjusted Items for Taxable Years Beginning in 2026",
        "url": "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf"
      }
    ],
    "value": [
      { "min": 0.0, "max": 49450.0, "rate": 0.0 },
      { "min": 49450.0, "max": 545500.0, "rate": 0.15 },
      { "min": 545500.0, "max": null, "rate": 0.2 }
    ]
  }
}
```

### `tax/federal_net_investment_income_tax`

```sh
entropyfa data lookup --category tax --key federal_net_investment_income_tax --year 2026 --filing-status single
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_net_investment_income_tax",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Topic No. 559, Net Investment Income Tax",
        "url": "https://www.irs.gov/taxtopics/tc559"
      }
    ],
    "value": {
      "rate": 0.038,
      "threshold": 200000.0
    }
  }
}
```

### `tax/federal_payroll_tax_parameters`

```sh
entropyfa data lookup --category tax --key federal_payroll_tax_parameters --year 2026 --filing-status single
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_payroll_tax_parameters",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Topic No. 751, Social Security and Medicare Withholding Rates",
        "url": "https://www.irs.gov/taxtopics/tc751"
      }
    ],
    "value": {
      "social_security_rate": 0.062,
      "social_security_wage_base": 184500.0,
      "medicare_rate": 0.0145,
      "self_employment_tax_rate": 0.124,
      "self_employment_medicare_rate": 0.029,
      "additional_medicare_rate": 0.009,
      "additional_medicare_threshold": 200000.0
    }
  }
}
```

### `tax/federal_capital_loss_limit`

```sh
entropyfa data lookup --category tax --key federal_capital_loss_limit --year 2026 --filing-status single
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_capital_loss_limit",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Topic No. 409, Capital Gains and Losses",
        "url": "https://www.irs.gov/taxtopics/tc409"
      }
    ],
    "value": {
      "filing_status": "single",
      "limit": 3000.0
    }
  }
}
```

### `tax/federal_qbi_deduction`

```sh
entropyfa data lookup --category tax --key federal_qbi_deduction --year 2026 --filing-status single
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_qbi_deduction",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Revenue Procedure 2025-32: 2026 Inflation-Adjusted Tax Items",
        "url": "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf"
      }
    ],
    "value": {
      "threshold": 201750.0,
      "phase_in_range_end": 276750.0,
      "deduction_rate": 0.2,
      "minimum_qbi_amount": 1000.0,
      "minimum_qbi_deduction": 400.0
    }
  }
}
```

### `tax/federal_estate_exemption`

```sh
entropyfa data lookup --category tax --key federal_estate_exemption --year 2026
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_estate_exemption",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill",
        "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill"
      }
    ],
    "value": {
      "exemption": 15000000.0
    }
  }
}
```

### `tax/federal_estate_brackets`

```sh
entropyfa data lookup --category tax --key federal_estate_brackets --year 2026
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_estate_brackets",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Instructions for Form 706 (Rev. September 2025) — Table A Unified Rate Schedule",
        "url": "https://www.irs.gov/instructions/i706"
      }
    ],
    "value": [
      { "min": 0.0, "max": 10000.0, "rate": 0.18 },
      { "min": 10000.0, "max": 20000.0, "rate": 0.2 },
      { "min": 20000.0, "max": 40000.0, "rate": 0.22 },
      { "min": 40000.0, "max": 60000.0, "rate": 0.24 },
      { "...": "..." },
      { "min": 750000.0, "max": 1000000.0, "rate": 0.39 },
      { "min": 1000000.0, "max": null, "rate": 0.4 }
    ]
  }
}
```

### `tax/federal_estate_applicable_credit`

```sh
entropyfa data lookup --category tax --key federal_estate_applicable_credit --year 2026
```

```json
{
  "ok": true,
  "data": {
    "category": "tax",
    "key": "federal_estate_applicable_credit",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "IRS releases tax inflation adjustments for tax year 2026, including amendments from the One, Big, Beautiful Bill",
        "url": "https://www.irs.gov/newsroom/irs-releases-tax-inflation-adjustments-for-tax-year-2026-including-amendments-from-the-one-big-beautiful-bill"
      }
    ],
    "value": {
      "applicable_credit": 5945800.0
    }
  }
}
```

## Retirement

### `retirement/uniform_lifetime_table`

```sh
entropyfa data lookup --category retirement --key uniform_lifetime_table --year 2026
```

```json
{
  "ok": true,
  "data": {
    "category": "retirement",
    "key": "uniform_lifetime_table",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Publication 590-B (2025), Distributions from Individual Retirement Arrangements (IRAs)",
        "url": "https://www.irs.gov/publications/p590b"
      }
    ],
    "value": [
      { "age": 72, "distribution_period": 27.4 },
      { "age": 73, "distribution_period": 26.5 },
      { "age": 74, "distribution_period": 25.5 },
      { "age": 75, "distribution_period": 24.6 },
      { "age": 76, "distribution_period": 23.7 },
      { "...": "..." },
      { "age": 119, "distribution_period": 2.3 },
      { "age": 120, "distribution_period": 2.0 }
    ]
  }
}
```

### `retirement/single_life_table`

```sh
entropyfa data lookup --category retirement --key single_life_table --year 2026
```

```json
{
  "ok": true,
  "data": {
    "category": "retirement",
    "key": "single_life_table",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Publication 590-B (2025), Distributions from Individual Retirement Arrangements (IRAs)",
        "url": "https://www.irs.gov/publications/p590b"
      }
    ],
    "value": [
      { "age": 0, "distribution_period": 84.6 },
      { "age": 1, "distribution_period": 83.7 },
      { "age": 2, "distribution_period": 82.7 },
      { "age": 3, "distribution_period": 81.7 },
      { "age": 4, "distribution_period": 80.7 },
      { "...": "..." },
      { "age": 114, "distribution_period": 1.9 },
      { "age": 115, "distribution_period": 1.9 }
    ]
  }
}
```

### `retirement/joint_life_table`

```sh
entropyfa data lookup --category retirement --key joint_life_table --year 2026
```

```json
{
  "ok": true,
  "data": {
    "category": "retirement",
    "key": "joint_life_table",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Publication 590-B (2025), Distributions from Individual Retirement Arrangements (IRAs)",
        "url": "https://www.irs.gov/pub/irs-pdf/p590b.pdf"
      }
    ],
    "value": [
      { "owner_age": 72, "spouse_age": 30, "distribution_period": 55.1 },
      { "owner_age": 72, "spouse_age": 40, "distribution_period": 45.5 },
      { "owner_age": 72, "spouse_age": 50, "distribution_period": 36.3 },
      { "owner_age": 72, "spouse_age": 55, "distribution_period": 32.0 },
      { "owner_age": 72, "spouse_age": 60, "distribution_period": 27.9 },
      { "...": "..." },
      { "owner_age": 95, "spouse_age": 90, "distribution_period": 6.7 },
      { "owner_age": 95, "spouse_age": 95, "distribution_period": 5.9 }
    ]
  }
}
```

### `retirement/distribution_rules`

```sh
entropyfa data lookup --category retirement --key distribution_rules --year 2026
```

```json
{
  "ok": true,
  "data": {
    "category": "retirement",
    "key": "distribution_rules",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Publication 590-B (2025), Distributions from Individual Retirement Arrangements (IRAs)",
        "url": "https://www.irs.gov/publications/p590b"
      }
    ],
    "value": {
      "required_beginning": {
        "first_distribution_deadline": "april_1_following_year",
        "start_age_rules": [
          {
            "birth_year_min": null,
            "birth_year_max": 1950,
            "start_age": 72,
            "guidance_status": "final",
            "notes": null
          },
          {
            "birth_year_min": 1951,
            "birth_year_max": 1958,
            "start_age": 73,
            "guidance_status": "final",
            "notes": null
          },
          {
            "birth_year_min": 1959,
            "birth_year_max": 1959,
            "start_age": 73,
            "guidance_status": "interim_good_faith",
            "notes": "IRS final regulations reserved the 1959 cohort; proposed regulations and Announcement 2026-7 continue to allow a reasonable good-faith interpretation treating 1959 births as age 73 until later final regulations apply."
          },
          {
            "birth_year_min": 1960,
            "birth_year_max": null,
            "start_age": 75,
            "guidance_status": "final",
            "notes": null
          }
        ],
        "still_working_exception": {
          "eligible_account_types": ["401k", "403b"],
          "eligible_plan_categories": ["401k", "403b", "profit_sharing", "other_defined_contribution_plan"],
          "disallowed_for_five_percent_owners": true
        }
      },
      "account_applicability": {
        "owner_required_account_types": ["traditional_ira", "sep_ira", "simple_ira", "401k", "403b", "457b"],
        "owner_exempt_account_types": ["roth_ira", "designated_roth_plan_account"],
        "inherited_account_types": ["inherited_ira", "inherited_roth_ira", "inherited_401k", "inherited_roth_401k"],
        "designated_roth_owner_exemption_effective_year": 2024,
        "supports_pre_1987_403b_exclusion": true,
        "pre_1987_403b": { "exclude_until_age": 75 }
      },
      "beneficiary_distribution": {
        "beneficiary_categories": ["eligible_designated_beneficiary", "designated_beneficiary", "non_designated_beneficiary"],
        "eligible_designated_beneficiary_classes": ["spouse", "minor_child", "disabled", "chronically_ill", "not_more_than_10_years_younger"],
        "minor_child_majority_age": 21,
        "spouse_delay_allowed": true,
        "ten_year_rule": {
          "terminal_year": 10,
          "annual_distributions_required_when_owner_died_on_or_after_rbd": true
        },
        "relief_years": [2021, 2022, 2023, 2024]
      }
    }
  }
}
```

## Social Security

### `social_security/benefit_taxation_thresholds`

This example uses the special `married_filing_separately` plus `--lived-with-spouse-during-year true` case so the extra flag is visible in the response.

```sh
entropyfa data lookup --category social_security --key benefit_taxation_thresholds --year 2026 --filing-status married_filing_separately --lived-with-spouse-during-year true
```

```json
{
  "ok": true,
  "data": {
    "category": "social_security",
    "key": "benefit_taxation_thresholds",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Publication 915 (2025), Social Security and Equivalent Railroad Retirement Benefits",
        "url": "https://www.irs.gov/publications/p915"
      }
    ],
    "value": {
      "filing_status": "married_filing_separately",
      "lived_with_spouse_during_year": true,
      "base_amount": 0.0,
      "upper_amount": 0.0,
      "max_taxable_pct_below_upper": 0.5,
      "max_taxable_pct_above_upper": 0.85
    }
  }
}
```

## Insurance

### `insurance/irmaa_brackets`

This example also uses the special married-filing-separately case so the spouse-living flag and the three-tier CMS table are visible.

```sh
entropyfa data lookup --category insurance --key irmaa_brackets --year 2026 --filing-status married_filing_separately --lived-with-spouse-during-year true
```

```json
{
  "ok": true,
  "data": {
    "category": "insurance",
    "key": "irmaa_brackets",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Centers for Medicare & Medicaid Services",
        "title": "2026 Medicare Parts A & B Premiums and Deductibles",
        "url": "https://www.cms.gov/newsroom/fact-sheets/2026-medicare-parts-b-premiums-deductibles"
      }
    ],
    "value": {
      "filing_status": "married_filing_separately",
      "lived_with_spouse_during_year": true,
      "base_part_b_premium": 202.9,
      "brackets": [
        { "magi_min": 0.0, "magi_max": 109000.0, "monthly_surcharge": 0.0 },
        { "magi_min": 109000.0, "magi_max": 391000.0, "monthly_surcharge": 446.3 },
        { "magi_min": 391000.0, "magi_max": null, "monthly_surcharge": 487.0 }
      ]
    }
  }
}
```

## Pension

### `pension/mortality_417e`

```sh
entropyfa data lookup --category pension --key mortality_417e --year 2026
```

```json
{
  "ok": true,
  "data": {
    "category": "pension",
    "key": "mortality_417e",
    "year": 2026,
    "verification_status": "authoritative",
    "pipeline_reviewed": true,
    "source_origin": "reviewed_artifact",
    "sources": [
      {
        "authority": "Internal Revenue Service",
        "title": "Notice 2025-40: Updated Static Mortality Tables for Defined Benefit Pension Plans for 2026",
        "url": "https://www.irs.gov/pub/irs-drop/n-25-40.pdf"
      }
    ],
    "value": [
      { "age": 50, "qx": 0.002813 },
      { "age": 51, "qx": 0.003053 },
      { "age": 52, "qx": 0.003319 },
      { "age": 53, "qx": 0.003618 },
      { "age": 54, "qx": 0.003955 },
      { "...": "..." },
      { "age": 99, "qx": 0.373 },
      { "age": 100, "qx": 0.4 }
    ]
  }
}
```
