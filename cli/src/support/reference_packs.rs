use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use entropyfa_engine::models::retirement_rmd::{
    AgeDistributionPeriod, JointDistributionPeriod, RmdParameters,
};
use serde::de::DeserializeOwned;
use serde::Deserialize;

const RMD_CATEGORY: &str = "retirement";
const RMD_SCHEMA_VERSION: u32 = 1;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RmdReferenceBundle {
    pub year: u32,
    pub distribution_rules: RmdParameters,
    pub uniform_lifetime_table: Vec<AgeDistributionPeriod>,
    pub joint_life_table: Vec<JointDistributionPeriod>,
    pub single_life_table: Vec<AgeDistributionPeriod>,
}

pub fn load_rmd_reference_bundle(
    reference_root: &Path,
    year: u32,
) -> Result<RmdReferenceBundle, String> {
    let year_root = reference_root.join(RMD_CATEGORY).join(year.to_string());
    let distribution_rules_value: DistributionRulesValue = load_retirement_pack(
        &year_root,
        "distribution_rules.md",
        "distribution_rules",
        year,
    )?;
    let uniform_lifetime_table: Vec<AgeDistributionPeriod> = load_retirement_pack(
        &year_root,
        "uniform_lifetime_table.md",
        "uniform_lifetime_table",
        year,
    )?;
    let single_life_table: Vec<AgeDistributionPeriod> = load_retirement_pack(
        &year_root,
        "single_life_expectancy_table.md",
        "single_life_table",
        year,
    )?;
    let joint_life_table: Vec<JointDistributionPeriod> =
        load_retirement_pack(&year_root, "joint_life_table.md", "joint_life_table", year)?;
    let distribution_rules = distribution_rules_value.into_rmd_parameters(
        uniform_lifetime_table.clone(),
        joint_life_table.clone(),
        single_life_table.clone(),
    );

    Ok(RmdReferenceBundle {
        year,
        distribution_rules,
        uniform_lifetime_table,
        joint_life_table,
        single_life_table,
    })
}

#[derive(Debug, Deserialize)]
struct DistributionRulesValue {
    account_applicability: AccountApplicabilityValue,
    beneficiary_distribution: BeneficiaryDistributionValue,
    required_beginning: RequiredBeginningValue,
}

#[derive(Debug, Deserialize)]
struct AccountApplicabilityValue {
    designated_roth_owner_exemption_effective_year: Option<u32>,
    inherited_account_types: Vec<String>,
    owner_exempt_account_types: Vec<String>,
    owner_required_account_types: Vec<String>,
    pre_1987_403b: Pre1987Value,
    supports_pre_1987_403b_exclusion: bool,
}

#[derive(Debug, Deserialize)]
struct Pre1987Value {
    exclude_until_age: u32,
}

#[derive(Debug, Deserialize)]
struct BeneficiaryDistributionValue {
    beneficiary_categories: Vec<String>,
    eligible_designated_beneficiary_classes: Vec<String>,
    life_expectancy_method_by_class: HashMap<String, String>,
    minor_child_majority_age: u32,
    non_designated_beneficiary_rules: NonDesignatedBeneficiaryRulesValue,
    recognized_beneficiary_classes: Vec<String>,
    relief_years: Vec<u32>,
    spouse_delay_allowed: bool,
    ten_year_rule: TenYearRuleValue,
}

#[derive(Debug, Deserialize)]
struct NonDesignatedBeneficiaryRulesValue {
    when_owner_died_before_required_beginning_date: String,
    when_owner_died_on_or_after_required_beginning_date: String,
}

#[derive(Debug, Deserialize)]
struct TenYearRuleValue {
    annual_distributions_required_when_owner_died_on_or_after_rbd: bool,
    terminal_year: u32,
}

#[derive(Debug, Deserialize)]
struct RequiredBeginningValue {
    first_distribution_deadline: String,
    start_age_rules: Vec<entropyfa_engine::models::retirement_rmd::StartAgeRule>,
    still_working_exception: StillWorkingExceptionValue,
}

#[derive(Debug, Deserialize)]
struct StillWorkingExceptionValue {
    disallowed_for_five_percent_owners: bool,
    eligible_account_types: Vec<String>,
    eligible_plan_categories: Vec<String>,
}

impl DistributionRulesValue {
    fn into_rmd_parameters(
        self,
        uniform_lifetime_table: Vec<AgeDistributionPeriod>,
        joint_life_table: Vec<JointDistributionPeriod>,
        single_life_table: Vec<AgeDistributionPeriod>,
    ) -> RmdParameters {
        let DistributionRulesValue {
            account_applicability,
            beneficiary_distribution,
            required_beginning,
        } = self;

        let AccountApplicabilityValue {
            designated_roth_owner_exemption_effective_year,
            inherited_account_types,
            owner_exempt_account_types,
            owner_required_account_types,
            pre_1987_403b,
            supports_pre_1987_403b_exclusion,
        } = account_applicability;
        let BeneficiaryDistributionValue {
            beneficiary_categories,
            eligible_designated_beneficiary_classes,
            life_expectancy_method_by_class,
            minor_child_majority_age,
            non_designated_beneficiary_rules,
            recognized_beneficiary_classes,
            relief_years,
            spouse_delay_allowed,
            ten_year_rule,
        } = beneficiary_distribution;
        let RequiredBeginningValue {
            first_distribution_deadline,
            start_age_rules,
            still_working_exception,
        } = required_beginning;
        let NonDesignatedBeneficiaryRulesValue {
            when_owner_died_before_required_beginning_date,
            when_owner_died_on_or_after_required_beginning_date,
        } = non_designated_beneficiary_rules;
        let TenYearRuleValue {
            annual_distributions_required_when_owner_died_on_or_after_rbd,
            terminal_year,
        } = ten_year_rule;
        let StillWorkingExceptionValue {
            disallowed_for_five_percent_owners,
            eligible_account_types,
            eligible_plan_categories,
        } = still_working_exception;

        RmdParameters {
            uniform_lifetime_table,
            joint_life_table,
            single_life_table,
            required_beginning: entropyfa_engine::models::retirement_rmd::RequiredBeginningRules {
                start_age_rules,
                first_distribution_deadline,
                still_working_exception_plan_categories: eligible_plan_categories,
                still_working_exception_eligible_account_types: eligible_account_types,
                still_working_exception_disallowed_for_five_percent_owners:
                    disallowed_for_five_percent_owners,
            },
            account_rules: entropyfa_engine::models::retirement_rmd::AccountRules {
                owner_required_account_types,
                owner_exempt_account_types,
                inherited_account_types,
                supports_pre_1987_403b_exclusion,
                designated_roth_owner_exemption_effective_year,
            },
            beneficiary_rules: entropyfa_engine::models::retirement_rmd::BeneficiaryRules {
                beneficiary_categories,
                recognized_beneficiary_classes,
                eligible_designated_beneficiary_classes,
                life_expectancy_method_by_class,
                minor_child_majority_age,
                spouse_delay_allowed,
                non_designated_beneficiary_rules:
                    entropyfa_engine::models::retirement_rmd::NonDesignatedBeneficiaryRules {
                        when_owner_died_before_required_beginning_date,
                        when_owner_died_on_or_after_required_beginning_date,
                    },
            },
            ten_year_rule: entropyfa_engine::models::retirement_rmd::TenYearRule {
                terminal_year,
                annual_distributions_required_when_owner_died_on_or_after_rbd,
            },
            relief_years,
            pre_1987_403b_rules: entropyfa_engine::models::retirement_rmd::Pre1987Rules {
                exclude_until_age: pre_1987_403b.exclude_until_age,
            },
        }
    }
}

fn load_retirement_pack<T: DeserializeOwned>(
    year_root: &Path,
    file_name: &str,
    expected_key: &str,
    year: u32,
) -> Result<T, String> {
    let path = year_root.join(file_name);
    let contents = fs::read_to_string(&path).map_err(|err| match err.kind() {
        ErrorKind::NotFound => missing_pack_error(&path, year, expected_key),
        _ => invalid_pack_error(
            &path,
            &format!("failed to read retirement reference pack: {err}"),
        ),
    })?;

    let block = extract_machine_block(&contents).map_err(|err| invalid_pack_error(&path, &err))?;
    let document: PackDocument<T> =
        serde_json::from_str(block).map_err(|err| invalid_pack_error(&path, &err.to_string()))?;

    if document.schema_version != RMD_SCHEMA_VERSION {
        return Err(invalid_pack_error(
            &path,
            &format!(
                "expected schema_version {}, found {}",
                RMD_SCHEMA_VERSION, document.schema_version
            ),
        ));
    }
    if document.category != RMD_CATEGORY {
        return Err(invalid_pack_error(
            &path,
            &format!(
                "expected category `{RMD_CATEGORY}`, found `{}`",
                document.category
            ),
        ));
    }
    if document.key != expected_key {
        return Err(invalid_pack_error(
            &path,
            &format!("expected key `{expected_key}`, found `{}`", document.key),
        ));
    }
    if document.year != year {
        return Err(invalid_pack_error(
            &path,
            &format!("expected year {year}, found {}", document.year),
        ));
    }

    document
        .value
        .into_selected()
        .map_err(|err| invalid_pack_error(&path, &err))
}

fn extract_machine_block(contents: &str) -> Result<&str, String> {
    let machine_block_start = contents
        .find("## Machine Block")
        .ok_or_else(|| "missing `## Machine Block` section".to_string())?;
    let after_heading = &contents[machine_block_start..];
    let fence_start = after_heading
        .find("```json")
        .ok_or_else(|| "missing fenced JSON machine block".to_string())?;
    let after_fence = &after_heading[fence_start + "```json".len()..];
    let start = after_fence
        .find('\n')
        .map(|idx| idx + 1)
        .ok_or_else(|| "machine block JSON fence is missing a newline".to_string())?;
    let json_block = &after_fence[start..];
    let end = json_block
        .find("```")
        .ok_or_else(|| "missing closing machine block fence".to_string())?;

    Ok(json_block[..end].trim())
}

#[derive(Debug, Deserialize)]
struct PackDocument<T> {
    schema_version: u32,
    category: String,
    key: String,
    year: u32,
    value: PackValue<T>,
}

#[derive(Debug, Deserialize)]
struct PackValue<T> {
    variants: Vec<PackVariant<T>>,
}

#[derive(Debug, Deserialize)]
struct PackVariant<T> {
    label: String,
    #[allow(dead_code)]
    params: serde_json::Value,
    value: T,
}

impl<T> PackValue<T> {
    fn into_selected(self) -> Result<T, String> {
        let mut variants = self.variants;
        if variants.is_empty() {
            return Err("no variants found in retirement reference pack".to_string());
        }

        if let Some(index) = variants
            .iter()
            .position(|variant| variant.label == "default")
        {
            return Ok(variants.swap_remove(index).value);
        }

        Ok(variants.remove(0).value)
    }
}

fn missing_pack_error(path: &Path, year: u32, expected_key: &str) -> String {
    format!(
        "reference_pack_missing: missing retirement reference pack `{expected_key}` for {year} at {}",
        path.display()
    )
}

fn invalid_pack_error(path: &Path, detail: &str) -> String {
    format!(
        "reference_pack_invalid: invalid retirement reference pack at {}: {detail}",
        path.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(label: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let nonce = COUNTER.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "entropyfa-rmd-reference-packs-{label}-{}-{nonce}",
            timestamp
        ));
        fs::create_dir_all(&path).expect("temp test directory should be creatable");
        path
    }

    fn reference_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../reference")
    }

    fn copy_pack(src_root: &Path, dst_root: &Path, file_name: &str) {
        let src = src_root.join("retirement/2026").join(file_name);
        let dst_dir = dst_root.join("retirement/2026");
        fs::create_dir_all(&dst_dir).expect("destination directory should be creatable");
        fs::copy(&src, dst_dir.join(file_name)).expect("reference pack should copy");
    }

    #[test]
    fn extract_machine_block_returns_json_payload() {
        let contents = r#"
## Machine Block

```json
{ "answer": 42 }
```
"#;

        let block = extract_machine_block(contents).expect("machine block should parse");

        assert_eq!(block, r#"{ "answer": 42 }"#);
    }

    #[test]
    fn load_rmd_reference_bundle_reads_installed_packs() {
        let bundle = load_rmd_reference_bundle(&reference_root(), 2026)
            .expect("reference bundle should load");

        assert_eq!(bundle.year, 2026);
        assert_eq!(
            bundle
                .distribution_rules
                .required_beginning
                .first_distribution_deadline,
            "april_1_following_year"
        );
        assert_eq!(bundle.uniform_lifetime_table.first().unwrap().age, 72);
        assert_eq!(bundle.single_life_table.first().unwrap().age, 0);
        assert_eq!(bundle.joint_life_table.first().unwrap().owner_age, 20);
    }

    #[test]
    fn load_rmd_reference_bundle_reports_missing_file() {
        let temp_root = unique_temp_dir("missing-file");
        copy_pack(&reference_root(), &temp_root, "distribution_rules.md");

        let err = load_rmd_reference_bundle(&temp_root, 2026).unwrap_err();

        assert!(err.starts_with("reference_pack_missing"));
        assert!(err.contains("uniform_lifetime_table"));
    }

    #[test]
    fn load_rmd_reference_bundle_reports_invalid_json() {
        let temp_root = unique_temp_dir("invalid-json");
        for file_name in [
            "distribution_rules.md",
            "uniform_lifetime_table.md",
            "single_life_expectancy_table.md",
            "joint_life_table.md",
        ] {
            copy_pack(&reference_root(), &temp_root, file_name);
        }
        fs::write(
            temp_root
                .join("retirement/2026")
                .join("uniform_lifetime_table.md"),
            "## Machine Block\n\n```json\n{ not-json\n```\n",
        )
        .expect("invalid pack should be writable");

        let err = load_rmd_reference_bundle(&temp_root, 2026).unwrap_err();

        assert!(err.starts_with("reference_pack_invalid"));
        assert!(err.contains("uniform_lifetime_table.md"));
    }
}
