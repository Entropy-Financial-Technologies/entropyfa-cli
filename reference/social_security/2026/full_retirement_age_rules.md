---
category: social_security
year: 2026
key: full_retirement_age_rules
title: Full Retirement Age Rules
reviewed_artifact: social_security/2026/full_retirement_age_rules
bundle_version: dev
verification_status: authoritative
review_status: reviewed
---

# Full Retirement Age Rules

## What This Is

A lookup table mapping birth year to full retirement age (FRA) for Social Security retirement and spousal benefits. The FRA is 65 for those born in 1937 or earlier, increases in 2-month increments for birth years 1938–1942 (reaching 65 and 10 months), holds at 66 for birth years 1943–1954, increases again in 2-month increments for birth years 1955–1959 (reaching 66 and 10 months), and is 67 for those born in 1960 or later.

## Lookup Parameters

- Year of birth (integer). No filing_status or other parameters affect which variant to select — there is only the default variant.

## Interpretation Notes

- Each rule specifies a birth-year range (birth_year_min to birth_year_max) and the corresponding FRA as years + months. For example, 66 years and 2 months means FRA is reached 66 years and 2 months after the date of birth.
- The first rule has birth_year_min = null, meaning it is open on the lower bound (applies to all birth years up to and including birth_year_max).
- The last rule has birth_year_max = null, meaning it is open on the upper bound (applies to birth_year_min and all later birth years).
- Rules are contiguous: every birth year maps to exactly one rule with no gaps.
- full_retirement_age_months is the additional months beyond full_retirement_age_years. A value of 0 months means the FRA is exactly that many years.

## Does Not Include

- Survivor benefit full retirement age (which follows a different schedule, reaching 67 for those born in 1962 or later rather than 1960)
- Early retirement reduction factors or delayed retirement credits
- Disability benefit conversion age rules
- Benefit amount calculations or earnings test thresholds

## Caveats

- The January 1 birth date rule means a person born on January 1 of year Y is treated as if born in year Y-1 for FRA determination. This is because SSA considers a person to attain an age on the day before their birthday.
- These FRA values are set by statute (Social Security Amendments of 1983) and are not subject to annual COLA or inflation adjustments. They do not change from year to year.
- No legislative changes to FRA have been enacted since the 1983 amendments. If future legislation raises FRA, this dataset would need updating.

## Typical Uses

- Determining the age at which a worker can claim unreduced retirement benefits
- Determining the age at which a spouse can claim unreduced spousal benefits
- Calculating early retirement reduction factors (which depend on months before FRA)
- Calculating delayed retirement credits (which depend on months after FRA up to age 70)

## Machine Block

```json
{
  "schema_version": 1,
  "category": "social_security",
  "key": "full_retirement_age_rules",
  "year": 2026,
  "verification_status": "authoritative",
  "completeness": "full",
  "accepted_sources": [
    {
      "source_id": "src_ssa_1",
      "url": "https://www.ssa.gov/benefits/retirement/planner/ageincrease.html",
      "host": "www.ssa.gov",
      "organization": "Social Security Administration",
      "source_class": "primary",
      "title": "Benefits Planner: Retirement | Retirement Age Calculator",
      "published_at": null,
      "locator": "Full retirement age table by year of birth and January 1 birth date note",
      "counts_toward_status": true
    },
    {
      "source_id": "src_ssa_2",
      "url": "https://www.ssa.gov/oact/progdata/nra.html",
      "host": "www.ssa.gov",
      "organization": "Social Security Administration, Office of the Chief Actuary",
      "source_class": "primary",
      "title": "Normal retirement age (NRA)",
      "published_at": null,
      "locator": "NRA table by year of birth for retirement/spousal and survivor benefits",
      "counts_toward_status": true
    },
    {
      "source_id": "src_ssa_3",
      "url": "https://choosework.ssa.gov/Assets/cw/files/find-your-full-or-normal-retirement-age.pdf",
      "host": "choosework.ssa.gov",
      "organization": "Social Security Administration",
      "source_class": "primary",
      "title": "Find Your Full or Normal Retirement Age",
      "published_at": null,
      "locator": "PDF table of FRA by birth year (1943–1954 through 1960+) and January 1 birth date rule note",
      "counts_toward_status": true
    },
    {
      "source_id": "src_aarp_1",
      "url": "https://www.aarp.org/social-security/faq/full-retirement-age/",
      "host": "www.aarp.org",
      "organization": "AARP",
      "source_class": "secondary",
      "title": "What Is The Full Retirement Age For Social Security?",
      "published_at": null,
      "locator": "Article text confirming FRA of 66 and 10 months for 1959, and 67 for 1960+",
      "counts_toward_status": true
    },
    {
      "source_id": "src_nerdwallet_1",
      "url": "https://www.nerdwallet.com/retirement/learn/retirement-age",
      "host": "www.nerdwallet.com",
      "organization": "NerdWallet",
      "source_class": "secondary",
      "title": "Full Retirement Age for Social Security: Rules",
      "published_at": null,
      "locator": "Article text and table confirming FRA schedule by birth year",
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
          "benefit_scope": "retirement_and_spousal",
          "january_1_births_use_prior_year": true,
          "rules": [
            {
              "birth_year_max": 1937,
              "birth_year_min": null,
              "full_retirement_age_months": 0,
              "full_retirement_age_years": 65
            },
            {
              "birth_year_max": 1938,
              "birth_year_min": 1938,
              "full_retirement_age_months": 2,
              "full_retirement_age_years": 65
            },
            {
              "birth_year_max": 1939,
              "birth_year_min": 1939,
              "full_retirement_age_months": 4,
              "full_retirement_age_years": 65
            },
            {
              "birth_year_max": 1940,
              "birth_year_min": 1940,
              "full_retirement_age_months": 6,
              "full_retirement_age_years": 65
            },
            {
              "birth_year_max": 1941,
              "birth_year_min": 1941,
              "full_retirement_age_months": 8,
              "full_retirement_age_years": 65
            },
            {
              "birth_year_max": 1942,
              "birth_year_min": 1942,
              "full_retirement_age_months": 10,
              "full_retirement_age_years": 65
            },
            {
              "birth_year_max": 1954,
              "birth_year_min": 1943,
              "full_retirement_age_months": 0,
              "full_retirement_age_years": 66
            },
            {
              "birth_year_max": 1955,
              "birth_year_min": 1955,
              "full_retirement_age_months": 2,
              "full_retirement_age_years": 66
            },
            {
              "birth_year_max": 1956,
              "birth_year_min": 1956,
              "full_retirement_age_months": 4,
              "full_retirement_age_years": 66
            },
            {
              "birth_year_max": 1957,
              "birth_year_min": 1957,
              "full_retirement_age_months": 6,
              "full_retirement_age_years": 66
            },
            {
              "birth_year_max": 1958,
              "birth_year_min": 1958,
              "full_retirement_age_months": 8,
              "full_retirement_age_years": 66
            },
            {
              "birth_year_max": 1959,
              "birth_year_min": 1959,
              "full_retirement_age_months": 10,
              "full_retirement_age_years": 66
            },
            {
              "birth_year_max": null,
              "birth_year_min": 1960,
              "full_retirement_age_months": 0,
              "full_retirement_age_years": 67
            }
          ]
        }
      }
    ]
  }
}
```

## Sources

- Social Security Administration — Benefits Planner: Retirement | Retirement Age Calculator — https://www.ssa.gov/benefits/retirement/planner/ageincrease.html
- Social Security Administration, Office of the Chief Actuary — Normal retirement age (NRA) — https://www.ssa.gov/oact/progdata/nra.html
- Social Security Administration — Find Your Full or Normal Retirement Age — https://choosework.ssa.gov/Assets/cw/files/find-your-full-or-normal-retirement-age.pdf
- AARP — What Is The Full Retirement Age For Social Security? — https://www.aarp.org/social-security/faq/full-retirement-age/
- NerdWallet — Full Retirement Age for Social Security: Rules — https://www.nerdwallet.com/retirement/learn/retirement-age
