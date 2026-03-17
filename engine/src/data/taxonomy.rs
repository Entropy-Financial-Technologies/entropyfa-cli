/// Metadata for a single data entry available via `lookup()`.
#[derive(Debug, Clone)]
pub struct CoverageEntry {
    pub category: String,
    pub key: String,
    pub years: Vec<u32>,
    pub params: Vec<String>,
    pub domain_path: String,
    pub tags: Vec<String>,
    pub related_topics: Vec<String>,
    pub description: String,
}

/// Build the full registry of available data entries.
pub fn all_entries() -> Vec<CoverageEntry> {
    vec![
        // --- tax/federal ---
        CoverageEntry {
            category: "tax".into(),
            key: "federal_income_tax_brackets".into(),
            years: vec![2026],
            params: vec!["filing_status".into()],
            domain_path: "tax::federal::brackets".into(),
            tags: vec!["income_tax".into(), "federal".into()],
            related_topics: vec![
                "federal_standard_deductions".into(),
                "federal_capital_gains_brackets".into(),
            ],
            description: "Federal income tax brackets by filing status".into(),
        },
        CoverageEntry {
            category: "tax".into(),
            key: "federal_standard_deductions".into(),
            years: vec![2026],
            params: vec!["filing_status".into()],
            domain_path: "tax::federal::standard_deductions".into(),
            tags: vec!["income_tax".into(), "federal".into(), "deductions".into()],
            related_topics: vec!["federal_income_tax_brackets".into()],
            description: "Federal standard deduction amounts by filing status".into(),
        },
        CoverageEntry {
            category: "tax".into(),
            key: "federal_salt_deduction_parameters".into(),
            years: vec![2026],
            params: vec!["filing_status".into()],
            domain_path: "tax::federal::salt_deduction_parameters".into(),
            tags: vec![
                "income_tax".into(),
                "federal".into(),
                "deductions".into(),
                "salt".into(),
                "itemized".into(),
            ],
            related_topics: vec![
                "federal_standard_deductions".into(),
                "federal_income_tax_brackets".into(),
            ],
            description:
                "Federal SALT itemized deduction cap, phaseout threshold, phaseout rate, and floor by filing status."
                    .into(),
        },
        CoverageEntry {
            category: "tax".into(),
            key: "federal_capital_gains_brackets".into(),
            years: vec![2026],
            params: vec!["filing_status".into()],
            domain_path: "tax::federal::capital_gains_brackets".into(),
            tags: vec!["capital_gains".into(), "federal".into()],
            related_topics: vec![
                "federal_income_tax_brackets".into(),
                "federal_net_investment_income_tax".into(),
            ],
            description: "Federal long-term capital gains tax brackets by filing status".into(),
        },
        CoverageEntry {
            category: "tax".into(),
            key: "federal_net_investment_income_tax".into(),
            years: vec![2026],
            params: vec!["filing_status".into()],
            domain_path: "tax::federal::niit".into(),
            tags: vec!["investment_tax".into(), "federal".into()],
            related_topics: vec!["federal_capital_gains_brackets".into()],
            description: "Net investment income tax rate and thresholds".into(),
        },
        CoverageEntry {
            category: "tax".into(),
            key: "federal_payroll_tax_parameters".into(),
            years: vec![2026],
            params: vec!["filing_status".into()],
            domain_path: "tax::federal::payroll".into(),
            tags: vec!["payroll".into(), "fica".into(), "federal".into()],
            related_topics: vec!["federal_net_investment_income_tax".into()],
            description: "Federal Social Security, Medicare, and SE tax rates and thresholds"
                .into(),
        },
        CoverageEntry {
            category: "tax".into(),
            key: "federal_capital_loss_limit".into(),
            years: vec![2026],
            params: vec!["filing_status".into()],
            domain_path: "tax::federal::capital_loss_limit".into(),
            tags: vec!["capital_gains".into(), "federal".into()],
            related_topics: vec!["federal_capital_gains_brackets".into()],
            description: "Federal annual capital loss deduction limit".into(),
        },
        CoverageEntry {
            category: "tax".into(),
            key: "federal_qbi_deduction".into(),
            years: vec![2026],
            params: vec!["filing_status".into()],
            domain_path: "tax::federal::qbi_deduction".into(),
            tags: vec![
                "income_tax".into(),
                "federal".into(),
                "qbi".into(),
                "section_199a".into(),
            ],
            related_topics: vec![
                "federal_income_tax_brackets".into(),
                "federal_standard_deductions".into(),
            ],
            description: "Federal Qualified Business Income (Section 199A) deduction parameters"
                .into(),
        },
        // --- tax/estate ---
        CoverageEntry {
            category: "tax".into(),
            key: "federal_estate_exemption".into(),
            years: vec![2026],
            params: vec![],
            domain_path: "tax::estate::exemption".into(),
            tags: vec!["estate_tax".into(), "federal".into()],
            related_topics: vec!["federal_estate_brackets".into()],
            description: "Federal estate tax exemption amount".into(),
        },
        CoverageEntry {
            category: "tax".into(),
            key: "federal_estate_brackets".into(),
            years: vec![2026],
            params: vec![],
            domain_path: "tax::estate::brackets".into(),
            tags: vec!["estate_tax".into(), "federal".into()],
            related_topics: vec!["federal_estate_exemption".into()],
            description: "Federal estate tax brackets (graduated rates)".into(),
        },
        CoverageEntry {
            category: "tax".into(),
            key: "federal_estate_applicable_credit".into(),
            years: vec![2026],
            params: vec![],
            domain_path: "tax::estate::applicable_credit".into(),
            tags: vec!["estate_tax".into(), "federal".into()],
            related_topics: vec!["federal_estate_exemption".into()],
            description: "Applicable credit amount against estate tax".into(),
        },
        // --- retirement/rmd ---
        CoverageEntry {
            category: "retirement".into(),
            key: "uniform_lifetime_table".into(),
            years: vec![2026],
            params: vec![],
            domain_path: "retirement::rmd_tables::uniform_lifetime".into(),
            tags: vec!["rmd".into(), "retirement".into()],
            related_topics: vec!["single_life_table".into(), "distribution_rules".into()],
            description: "IRS Uniform Lifetime Table (Table III) for RMD calculations".into(),
        },
        CoverageEntry {
            category: "retirement".into(),
            key: "single_life_table".into(),
            years: vec![2026],
            params: vec![],
            domain_path: "retirement::rmd_tables::single_life".into(),
            tags: vec!["rmd".into(), "retirement".into()],
            related_topics: vec!["uniform_lifetime_table".into()],
            description: "IRS Single Life Expectancy Table (Table I) for inherited IRAs".into(),
        },
        CoverageEntry {
            category: "retirement".into(),
            key: "joint_life_table".into(),
            years: vec![2026],
            params: vec![],
            domain_path: "retirement::rmd_tables::joint_life".into(),
            tags: vec!["rmd".into(), "retirement".into()],
            related_topics: vec!["uniform_lifetime_table".into()],
            description: "IRS Joint Life and Last Survivor Table (Table II)".into(),
        },
        CoverageEntry {
            category: "retirement".into(),
            key: "distribution_rules".into(),
            years: vec![2026],
            params: vec![],
            domain_path: "retirement::rmd_rules::distribution_rules".into(),
            tags: vec!["rmd".into(), "retirement".into(), "secure_act".into()],
            related_topics: vec!["uniform_lifetime_table".into()],
            description:
                "RMD distribution rules grouped into required beginning dates, account applicability, and beneficiary distribution rules."
                    .into(),
        },
        // --- social_security ---
        CoverageEntry {
            category: "social_security".into(),
            key: "benefit_taxation_thresholds".into(),
            years: vec![2026],
            params: vec![
                "filing_status".into(),
                "lived_with_spouse_during_year".into(),
            ],
            domain_path: "social_security::taxation::thresholds".into(),
            tags: vec!["social_security".into(), "taxation".into()],
            related_topics: vec!["federal_income_tax_brackets".into()],
            description:
                "Social Security benefit taxation thresholds by filing status; married_filing_separately also requires whether the taxpayer lived with a spouse during the year."
                    .into(),
        },
        // --- insurance/irmaa ---
        CoverageEntry {
            category: "insurance".into(),
            key: "irmaa_brackets".into(),
            years: vec![2026],
            params: vec![
                "filing_status".into(),
                "lived_with_spouse_during_year".into(),
            ],
            domain_path: "insurance::irmaa::brackets".into(),
            tags: vec!["medicare".into(), "irmaa".into(), "insurance".into()],
            related_topics: vec!["federal_income_tax_brackets".into()],
            description:
                "Medicare IRMAA surcharge brackets by filing status; married_filing_separately also requires whether the taxpayer lived with a spouse during the year. Brackets use an inclusive upper bound, with the final tier open-ended."
                    .into(),
        },
        // --- pension ---
        CoverageEntry {
            category: "pension".into(),
            key: "mortality_417e".into(),
            years: vec![2026],
            params: vec![],
            domain_path: "pension::mortality::table_417e".into(),
            tags: vec!["pension".into(), "mortality".into(), "417e".into()],
            related_topics: vec![],
            description: "Section 417(e) mortality table for pension lump-sum calculations".into(),
        },
    ]
}
