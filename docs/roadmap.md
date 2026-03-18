# Product and Pipeline Roadmap

This doc is the maintainer view of where `entropyfa` should go if the goal is to become advisor-grade across the core planning surface and eventually surpass the breadth of leading advisor planning platforms.

This is not a promise that every item below is equally suitable for the current pipeline system. The important distinction is:

- some gaps are best solved by adding new yearly reference-data pipelines
- some gaps are bigger compute products that require multiple pipelines plus new request models and commands

## North Star

`entropyfa` should aim to be:

- broader than a tax calculator
- more trustworthy than agent-only research
- more composable than monolithic planning software
- more explicit about sources, assumptions, and dates than incumbent advisor tools

The goal is not to replicate any one incumbent product. `entropyfa` should differentiate on:

- verifiable reference data
- deterministic, scriptable compute primitives
- agent-native schemas and JSON contracts
- faster release cadence when laws and thresholds change

## Current Baseline

Today the shipped product is strong in a narrow slice:

- verified federal tax reference data
- verified RMD tables and rules
- verified IRMAA and Social Security taxation thresholds
- verified 417(e) mortality
- compute for federal tax, estate tax, RMDs, Roth conversions, pension comparison, projection, and goal solving

The repo also contains many empty domain shells that are not productized yet:

- social security
- healthcare
- withdrawal
- asset location
- insurance
- charitable
- gifting
- trust
- education
- debt
- business
- compensation
- real estate
- international
- divorce

That means the current module tree is wider than the actual shipped planning surface.

## Competitive Surface

Based on current official positioning from leading advisor planning platforms, the target is not just "more tax tables." The recurring platform themes are:

- goals-based planning
- retirement distribution and retirement income
- tax planning
- Social Security optimization
- healthcare cost planning
- insurance and risk management
- estate, gifting, trust, and UHNW planning
- detailed cash-flow planning
- client collaboration and interactive what-if analysis

This roadmap is an inference from their current public product pages, not a claim that every listed feature is implemented in exactly the same way.

## Product Strategy

The right way to build beyond current incumbent tools is:

1. Finish the highest-value household planning primitives first.
2. Use pipelines only for facts that are authoritative, reusable, and likely to change.
3. Build compute commands on top of those facts in narrow, composable units.
4. Avoid broad state-tax or UHNW modeling until the federal retirement-income core is complete.

In practice, that means the next phase should focus on the advisor questions that come up constantly:

- How much tax will this create?
- When should Social Security start?
- What will Medicare actually cost?
- How should withdrawals be sequenced?
- Should we convert to Roth?
- Is the pension lump sum attractive?
- What happens to the plan if we change retirement age, spending, or insurance coverage?

## Domain Map

### Tier 1: Retirement-Income Core

This is the highest-priority domain because it drives day-to-day advisor planning and already overlaps the current product.

Data pipelines:

- `insurance/medicare_base_premiums`
- `social_security/full_retirement_age_rules`
- `social_security/retirement_earnings_test_thresholds`
- `social_security/delayed_retirement_credit_rules`
- `social_security/early_retirement_reduction_rules`
- `tax/federal_qcd_limit`
- `retirement/contribution_limits`
- `tax/hsa_contribution_limits`

Compute products:

- Social Security claiming estimator / optimizer
- Medicare premium calculator
- tax-aware retirement income planner
- withdrawal sequencing engine
- contribution planning / catch-up planning

Why this tier matters:

- It closes the biggest gap between the current tax engine and full retirement advice.
- It creates direct reuse across Roth conversion, pension comparison, projection, and future withdrawal logic.
- It maps cleanly to the public positioning of leading advisor planning platforms around retirement distribution, Social Security optimization, health care cost planning, and tax planning.

### Tier 2: Estate, Gifting, and Trust

You already have a basic federal estate tax calculator, but that is well short of advisor-grade estate planning.

Data pipelines:

- `gifting/federal_annual_exclusion`
- `gifting/federal_gst_exemption`
- `trust/federal_income_tax_brackets`
- `trust/federal_net_investment_income_tax`
- `charitable/qcd_limit` if not folded into tax
- `charitable/agi_deduction_limits`

Compute products:

- gifting strategy calculator
- trust income tax calculator
- charitable giving planner
- portability / DSUE scenario tools
- fairness and legacy allocation analysis

Why this tier matters:

- It is a natural extension of what advisors already expect after retirement and tax planning.
- It pushes toward the "advanced estate strategies" and "trust strategies" positioning seen in incumbent platforms.

### Tier 3: Cash Flow, Debt, Education, and Real Estate

These are essential for holistic planning, but they are less pipeline-heavy and more compute/model-heavy.

Likely reference data:

- `education/college_cost_assumptions`
- `debt/federal_student_loan_parameters`
- `real_estate/mortgage_limits` if you decide to support tax-facing real estate scenarios

Compute products:

- debt payoff planner
- education funding planner
- home purchase / mortgage comparison
- richer household cash-flow engine

Why this tier matters:

- It improves breadth, especially for younger households and pre-retirees.
- It supports the "goals-based" and "cash-flow planning" story.

### Tier 4: Insurance and Healthcare Protection

This is not only retirement Medicare. It also includes risk management and long-term care.

Data pipelines:

- `insurance/medicare_base_premiums`
- `insurance/ltc_tax_qualified_premium_limits`
- `insurance/hsa_contribution_limits` if not kept under tax

Compute products:

- health care cost planner
- life insurance needs analysis
- disability income gap analysis
- long-term care funding analysis

Why this tier matters:

- Incumbents explicitly position around lifetime protection, healthcare costs, and risk management.
- This category differentiates `entropyfa` from generic tax calculators.

### Tier 5: Business Owner, Equity Comp, and Advanced Planning

This is where broader category leadership becomes real, but it should come after the retirement-income core is solid.

Data pipelines:

- `compensation/iso_amt_parameters`
- `compensation/qualified_small_business_stock_rules` if modeled as lookupable federal parameters
- `business/self_employment_retirement_plan_limits`

Compute products:

- stock option scenario analysis
- QSBS tax scenario analysis
- business succession / liquidity planning
- entity-aware household planning

Why this tier matters:

- It is high-value but narrower in audience.
- It should not block the much more common retirement-income workflows.

### Tier 6: State Tax and Jurisdictional Expansion

This is strategically important and potentially a moat, but it is also the biggest scope trap.

Do not treat this as "the next thing." It needs architecture work first:

- lookup params beyond `filing_status`
- `state_code` and possibly locality support
- multi-jurisdiction source policy
- release and validation strategy for incomplete state coverage

State tax should be its own program after the federal retirement-income core is much more complete.

## The Next Pipeline Definition

If the question is "what single pipeline definition should we build next?", the recommendation is:

- `insurance/medicare_base_premiums`

Why this should be next:

- It is a clean yearly CMS-backed contract.
- It pairs directly with the already-shipped `insurance/irmaa_brackets`.
- It upgrades current retirement outputs from surcharge-only modeling to actual Medicare premium modeling.
- It is reusable across pension comparison, retirement projection, and a future health care planner.
- It is much lower ambiguity than trying to start with a full Social Security optimization contract.

Suggested contract:

```json
{
  "part_b_standard_monthly_premium": 0.0,
  "part_b_annual_deductible": 0.0,
  "part_d_base_beneficiary_premium": 0.0
}
```

Why not start with Social Security claiming first:

- Social Security is the next major capability area, but not the best next lone pipeline.
- A real claiming engine needs multiple datasets and broader lookup parameters, not just one table.
- `insurance/medicare_base_premiums` is the highest-confidence, lowest-friction next definition that materially improves planning outputs immediately.

## The Next Major Capability Program

If the question is "what bigger planning program should we start next?", the answer is:

- Social Security and Medicare retirement-income planning

That program should likely include, in order:

1. `insurance/medicare_base_premiums`
2. `social_security/full_retirement_age_rules`
3. `social_security/retirement_earnings_test_thresholds`
4. `social_security/delayed_retirement_credit_rules`
5. `social_security/early_retirement_reduction_rules`
6. a new `compute social-security` command
7. integration into projection, Roth conversion, pension comparison, and future withdrawal logic

## Pipeline Prioritization Rules

When choosing the next reference-data entry, prioritize the ones that:

1. unlock or materially improve an existing compute command
2. are reused across multiple commands
3. come from a stable official yearly source
4. have a narrow, testable public contract
5. affect advisor recommendations frequently

That rubric strongly favors:

- Medicare base premiums
- Social Security claiming primitives
- QCD limits
- contribution limits
- trust tax brackets
- annual gift exclusion

It does not favor:

- broad state tax expansion yet
- exotic UHNW edge cases first
- data feeds that are mostly assumptions rather than authoritative rules

## Architectural Prerequisites

To support the roadmap cleanly, the pipeline system should evolve in parallel.

The current public lookup parameter model is too narrow for the full planning surface. It currently centers on:

- `filing_status`
- `lived_with_spouse_during_year`

The roadmap will require more parameter types, including:

- `state_code`
- `birth_year`
- `claim_age`
- `coverage_type`
- `beneficiary_class`
- `plan_type`
- `entity_type`

It will also require more than one reference-data shape:

- yearly scalar values
- yearly multi-field parameter bundles
- rule tables keyed by birth-year bands
- periodic series where a year alone is not enough

Without that work, the roadmap will bottleneck on contract design instead of source verification.

## Sequencing Recommendation

### Phase 1: Finish the Federal Retirement-Income Core

- add `insurance/medicare_base_premiums`
- backfill 2025 support for the remaining 2026-only tax entries that matter for compute
- add the first Social Security claiming rule pipelines
- add QCD and contribution-limit pipelines

### Phase 2: Turn Those Pipelines Into New Commands

- ship a `compute social-security` command
- ship a Medicare premium calculator or integrate it into retirement commands
- ship a first withdrawal sequencing / retirement income command

### Phase 3: Deepen Estate and Gifting

- annual gift exclusion
- trust tax brackets
- GST and charitable planning primitives

### Phase 4: Broaden Household Planning

- debt
- education
- real estate
- richer cash-flow modeling

### Phase 5: Open the Advanced Planning Frontier

- business owner planning
- equity compensation
- trust and entity planning
- eventually state tax

## Sources

Official product pages that informed the category-benchmark summary:

- eMoney planning overview: https://emoneyadvisor.com/why-emoney/planning/
- eMoney holistic planning case study: https://emoneyadvisor.com/resources/case-studies/delivering-holistic-comprehensive-financial-planning-with-emoney/
- eMoney advanced planning material: https://emoneyadvisor.com/wp-content/uploads/2020/07/Advisor-Education-Advanced-Planning-in-eMoney.pdf
- MoneyGuide platform overview: https://www.moneyguidepro.com/
- MoneyGuide platform details: https://www.moneyguidepro.com/ifa/home/platform
- MoneyGuide product/features page: https://www.moneyguidepro.com/ifa/home/products
- MoneyGuide FAQ surface area: https://www.moneyguidepro.com/ifa/home/faqs
