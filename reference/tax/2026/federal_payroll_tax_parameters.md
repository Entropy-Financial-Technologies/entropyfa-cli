---
category: tax
year: 2026
key: federal_payroll_tax_parameters
title: Federal Payroll Tax Parameters
reviewed_artifact: tax/2026/federal_payroll_tax_parameters
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Federal Payroll Tax Parameters

## What This Is

Federal payroll tax parameters for tax year 2026, including Social Security (OASDI) and Medicare (HI) rates, the Social Security wage base, self-employment equivalent rates, and Additional Medicare Tax rate and thresholds by filing status.

## Lookup Parameters

- filing_status — determines the additional_medicare_threshold; all other fields are identical across filing statuses

## Interpretation Notes

- social_security_rate is the employee-only OASDI rate (6.2%); self_employment_tax_rate is the combined rate (12.4%) paid by self-employed individuals.
- medicare_rate is the employee-only HI rate (1.45%); self_employment_medicare_rate is the combined rate (2.9%).
- additional_medicare_rate (0.9%) applies only to wages or self-employment income exceeding the additional_medicare_threshold; there is no employer match for this tax.
- additional_medicare_threshold varies by filing status: $200,000 for Single/HoH/QSS, $250,000 for MFJ, $125,000 for MFS.
- social_security_wage_base ($184,500 for 2026) caps the earnings subject to Social Security tax; Medicare has no wage base cap.
- All rates are expressed as decimals (e.g., 0.062 = 6.2%).

## Does Not Include

- Employer-side matching rates (the dataset stores employee-side and self-employed rates only)
- FUTA (Federal Unemployment Tax Act) rates or wage bases
- State-level payroll or unemployment tax parameters
- Railroad Retirement Tax Act (RRTA) rates, which have separate thresholds
- Net Investment Income Tax (NIIT) of 3.8%, which is a separate surtax on investment income

## Caveats

- The Additional Medicare Tax thresholds are statutory (set by ACA §9015) and are not indexed for inflation; they have remained unchanged since 2013.
- Social Security and Medicare rates shown are the employee-only share; employers pay a matching amount. Self-employment rates are the combined employee+employer equivalent.
- The social_security_wage_base is adjusted annually by SSA based on the national average wage index; it applies per worker, not per job, so workers with multiple employers may need to reconcile on their tax return.
- These parameters do not include the employer-side FUTA tax or state unemployment taxes.

## Typical Uses

- Computing FICA withholding on employee wages
- Computing self-employment tax liability on Schedule SE
- Determining the Additional Medicare Tax obligation on Form 8959
- Projecting total payroll tax burden across filing statuses for financial planning

## Machine Block

```json
{
  "schema_version": 1,
  "category": "tax",
  "key": "federal_payroll_tax_parameters",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_irs_1",
      "url": "https://www.irs.gov/taxtopics/tc751",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Topic No. 751, Social Security and Medicare Withholding Rates",
      "published_at": "2026",
      "locator": "Full page — 2026 Social Security rate 6.2%, Medicare rate 1.45%, wage base $184,500, Additional Medicare Tax 0.9%",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_2",
      "url": "https://www.irs.gov/taxtopics/tc560",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Topic No. 560, Additional Medicare Tax",
      "published_at": "2026",
      "locator": "Full page — Additional Medicare Tax rate 0.9%, thresholds by filing status: MFJ $250,000, MFS $125,000, all others $200,000",
      "counts_toward_status": true
    },
    {
      "source_id": "src_irs_3",
      "url": "https://www.irs.gov/businesses/small-businesses-self-employed/questions-and-answers-for-the-additional-medicare-tax",
      "host": "www.irs.gov",
      "organization": "Internal Revenue Service",
      "source_class": "primary",
      "title": "Questions and Answers for the Additional Medicare Tax",
      "published_at": "2026",
      "locator": "Filing status threshold table: Single $200,000, MFJ $250,000, MFS $125,000, HoH $200,000, Qualifying widow(er) $200,000",
      "counts_toward_status": true
    },
    {
      "source_id": "src_ssa_1",
      "url": "https://www.ssa.gov/news/en/press/releases/2025-10-24.html",
      "host": "www.ssa.gov",
      "organization": "Social Security Administration",
      "source_class": "primary",
      "title": "Social Security Announces 2.8 Percent Benefit Increase for 2026",
      "published_at": "2025-10-24",
      "locator": "Press release announcing 2026 COLA and confirming maximum taxable earnings of $184,500",
      "counts_toward_status": true
    },
    {
      "source_id": "src_kiplinger_1",
      "url": "https://www.kiplinger.com/taxes/social-security-tax-wage-base-jumps",
      "host": "www.kiplinger.com",
      "organization": "Kiplinger",
      "source_class": "secondary",
      "title": "Social Security Tax Limit for 2026: What the Higher Cap Means for Your Paycheck",
      "published_at": "2025",
      "locator": "Article confirming 2026 Social Security wage base of $184,500 and tax rate of 6.2%",
      "counts_toward_status": true
    },
    {
      "source_id": "src_taxfoundation_1",
      "url": "https://taxfoundation.org/research/all/federal/payroll-taxes-social-security-medicare/",
      "host": "taxfoundation.org",
      "organization": "Tax Foundation",
      "source_class": "secondary",
      "title": "Federal Payroll Taxes: Social Security & Medicare",
      "published_at": "2026",
      "locator": "Overview page confirming combined payroll tax rate of 15.3% (12.4% SS + 2.9% Medicare)",
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
        "value": {
          "additional_medicare_rate": 0.009,
          "additional_medicare_threshold": 200000.0,
          "medicare_rate": 0.0145,
          "self_employment_medicare_rate": 0.029,
          "self_employment_tax_rate": 0.124,
          "social_security_rate": 0.062,
          "social_security_wage_base": 184500.0
        }
      },
      {
        "label": "married_filing_jointly",
        "params": {
          "filing_status": "married_filing_jointly",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "additional_medicare_rate": 0.009,
          "additional_medicare_threshold": 250000.0,
          "medicare_rate": 0.0145,
          "self_employment_medicare_rate": 0.029,
          "self_employment_tax_rate": 0.124,
          "social_security_rate": 0.062,
          "social_security_wage_base": 184500.0
        }
      },
      {
        "label": "married_filing_separately",
        "params": {
          "filing_status": "married_filing_separately",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "additional_medicare_rate": 0.009,
          "additional_medicare_threshold": 125000.0,
          "medicare_rate": 0.0145,
          "self_employment_medicare_rate": 0.029,
          "self_employment_tax_rate": 0.124,
          "social_security_rate": 0.062,
          "social_security_wage_base": 184500.0
        }
      },
      {
        "label": "head_of_household",
        "params": {
          "filing_status": "head_of_household",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "additional_medicare_rate": 0.009,
          "additional_medicare_threshold": 200000.0,
          "medicare_rate": 0.0145,
          "self_employment_medicare_rate": 0.029,
          "self_employment_tax_rate": 0.124,
          "social_security_rate": 0.062,
          "social_security_wage_base": 184500.0
        }
      },
      {
        "label": "qualifying_surviving_spouse",
        "params": {
          "filing_status": "qualifying_surviving_spouse",
          "lived_with_spouse_during_year": null
        },
        "value": {
          "additional_medicare_rate": 0.009,
          "additional_medicare_threshold": 200000.0,
          "medicare_rate": 0.0145,
          "self_employment_medicare_rate": 0.029,
          "self_employment_tax_rate": 0.124,
          "social_security_rate": 0.062,
          "social_security_wage_base": 184500.0
        }
      }
    ]
  }
}
```

## Sources

- Internal Revenue Service — Topic No. 751, Social Security and Medicare Withholding Rates — https://www.irs.gov/taxtopics/tc751
- Internal Revenue Service — Topic No. 560, Additional Medicare Tax — https://www.irs.gov/taxtopics/tc560
- Internal Revenue Service — Questions and Answers for the Additional Medicare Tax — https://www.irs.gov/businesses/small-businesses-self-employed/questions-and-answers-for-the-additional-medicare-tax
- Social Security Administration — Social Security Announces 2.8 Percent Benefit Increase for 2026 — https://www.ssa.gov/news/en/press/releases/2025-10-24.html
- Kiplinger — Social Security Tax Limit for 2026: What the Higher Cap Means for Your Paycheck — https://www.kiplinger.com/taxes/social-security-tax-wage-base-jumps
- Tax Foundation — Federal Payroll Taxes: Social Security & Medicare — https://taxfoundation.org/research/all/federal/payroll-taxes-social-security-medicare/
