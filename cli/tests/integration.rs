use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn entropyfa() -> Command {
    Command::new(env!("CARGO_BIN_EXE_entropyfa"))
}

fn entropyfa_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_entropyfa"))
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    path.push(format!("entropyfa-{label}-{nanos}"));
    fs::create_dir_all(&path).expect("should create temp dir");
    path
}

fn write_manifest(root: &Path, body: &str) {
    fs::create_dir_all(root).expect("should create reference root");
    fs::write(root.join("manifest.json"), body).expect("should write manifest");
}

fn install_test_binary(home_dir: &Path) -> PathBuf {
    let target = home_dir.join(".entropyfa/bin/entropyfa");
    fs::create_dir_all(target.parent().expect("binary should have parent"))
        .expect("should create binary directory");
    fs::copy(entropyfa_bin(), &target).expect("should copy test binary");
    let mut perms = fs::metadata(&target)
        .expect("copied binary should exist")
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&target, perms).expect("should make copied binary executable");
    target
}

fn write_managed_marker(root: &Path) {
    fs::create_dir_all(root).expect("should create reference root");
    fs::write(root.join(".entropyfa-managed"), b"managed").expect("should write marker");
}

fn write_install_metadata(binary_path: &Path, profile: &str, reference_root: Option<&Path>) {
    let metadata_path = binary_path
        .parent()
        .expect("binary path should have parent")
        .join("entropyfa.install.json");
    let reference_root_json = reference_root
        .map(|root| format!("\"{}\"", root.display()))
        .unwrap_or_else(|| "null".to_string());
    fs::write(
        metadata_path,
        format!("{{\"install_profile\":\"{profile}\",\"reference_root\":{reference_root_json}}}"),
    )
    .expect("should write install metadata");
}

fn run_ok(cmd: &mut Command) -> serde_json::Value {
    let output = cmd.output().expect("failed to execute process");
    assert_eq!(
        output.status.code(),
        Some(0),
        "expected exit code 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("non-utf8 stdout");
    serde_json::from_str(&stdout).expect("stdout is not valid JSON")
}

fn run_err(cmd: &mut Command) -> serde_json::Value {
    let output = cmd.output().expect("failed to execute process");
    assert_eq!(
        output.status.code(),
        Some(1),
        "expected exit code 1, got {:?}, stdout: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout)
    );
    let stdout = String::from_utf8(output.stdout).expect("non-utf8 stdout");
    serde_json::from_str(&stdout).expect("stdout is not valid JSON")
}

fn run_text(cmd: &mut Command) -> String {
    let output = cmd.output().expect("failed to execute process");
    assert_eq!(
        output.status.code(),
        Some(0),
        "expected exit code 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("non-utf8 stdout")
}

// ---------------------------------------------------------------------------
// 1. --version
// ---------------------------------------------------------------------------

#[test]
fn version_flag_shows_version() {
    let output = entropyfa()
        .arg("--version")
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("entropyfa"),
        "version output should contain 'entropyfa': {stdout}"
    );
}

// ---------------------------------------------------------------------------
// 2. --help
// ---------------------------------------------------------------------------

#[test]
fn help_flag_shows_usage() {
    let output = entropyfa()
        .arg("--help")
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("data"),
        "help should mention 'data' subcommand: {stdout}"
    );
    assert!(
        stdout.contains("compute"),
        "help should mention 'compute' subcommand: {stdout}"
    );
    assert!(
        stdout.contains("update"),
        "help should mention 'update' alias: {stdout}"
    );
}

#[test]
fn update_alias_help_works() {
    let output = entropyfa()
        .args(["update", "--help"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("Update entropyfa to the latest version"),
        "update alias help should describe the upgrade command: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// 2b. env → reference pack discovery and root resolution
// ---------------------------------------------------------------------------

#[test]
fn env_json_reports_missing_manifest_cleanly() {
    let reference_root = unique_temp_dir("missing-manifest");
    let home_dir = unique_temp_dir("missing-manifest-home");

    let v = run_ok(
        entropyfa()
            .args([
                "env",
                "--json",
                "--reference-root",
                &reference_root.display().to_string(),
            ])
            .env("HOME", &home_dir),
    );

    assert_eq!(v["ok"], true);
    assert_eq!(
        v["data"]["reference"]["resolved_root"],
        reference_root.display().to_string()
    );
    assert_eq!(v["data"]["reference"]["resolution_source"], "flag");
    assert_eq!(v["data"]["reference"]["packs_present"], false);
    assert!(v["data"]["reference"]["manifest"].is_null());
}

#[test]
fn env_json_reference_root_flag_takes_precedence_over_env_var() {
    let explicit_root = unique_temp_dir("explicit-root");
    let env_root = unique_temp_dir("env-root");
    let home_dir = unique_temp_dir("precedence-home");

    let v = run_ok(
        entropyfa()
            .args([
                "env",
                "--json",
                "--reference-root",
                &explicit_root.display().to_string(),
            ])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &env_root),
    );

    assert_eq!(
        v["data"]["reference"]["resolved_root"],
        explicit_root.display().to_string()
    );
    assert_eq!(v["data"]["reference"]["resolution_source"], "flag");
}

#[test]
fn env_json_reference_root_env_var_takes_precedence_over_default() {
    let env_root = unique_temp_dir("env-only-root");
    let home_dir = unique_temp_dir("env-only-home");

    let v = run_ok(
        entropyfa()
            .args(["env", "--json"])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &env_root),
    );

    assert_eq!(
        v["data"]["reference"]["resolved_root"],
        env_root.display().to_string()
    );
    assert_eq!(v["data"]["reference"]["resolution_source"], "env");
}

#[test]
fn env_json_reference_root_defaults_to_entropyfa_home_reference() {
    let home_dir = unique_temp_dir("default-home");
    let expected_root = home_dir.join(".entropyfa/reference");

    let v = run_ok(
        entropyfa()
            .args(["env", "--json"])
            .env("HOME", &home_dir)
            .env_remove("ENTROPYFA_REFERENCE_ROOT"),
    );

    assert_eq!(
        v["data"]["reference"]["resolved_root"],
        expected_root.display().to_string()
    );
    assert_eq!(v["data"]["reference"]["resolution_source"], "default");
}

#[test]
fn env_json_user_root_binary_without_managed_reference_root_is_binary_only() {
    let home_dir = unique_temp_dir("user-binary-only-home");
    let installed_binary = install_test_binary(&home_dir);

    let v = run_ok(
        Command::new(installed_binary)
            .args(["env", "--json"])
            .env("HOME", &home_dir)
            .env_remove("ENTROPYFA_REFERENCE_ROOT")
            .env_remove("ENTROPYFA_INSTALL_PROFILE"),
    );

    assert_eq!(v["data"]["install_profile"], "binary-only");
    assert_eq!(
        v["data"]["reference"]["resolved_root"],
        home_dir.join(".entropyfa/reference").display().to_string()
    );
}

#[test]
fn env_json_user_root_binary_with_managed_reference_root_is_full() {
    let home_dir = unique_temp_dir("user-full-home");
    let installed_binary = install_test_binary(&home_dir);
    let reference_root = home_dir.join(".entropyfa/reference");
    write_managed_marker(&reference_root);

    let v = run_ok(
        Command::new(installed_binary)
            .args(["env", "--json"])
            .env("HOME", &home_dir)
            .env_remove("ENTROPYFA_REFERENCE_ROOT")
            .env_remove("ENTROPYFA_INSTALL_PROFILE"),
    );

    assert_eq!(v["data"]["install_profile"], "full");
    assert_eq!(
        v["data"]["reference"]["resolved_root"],
        reference_root.display().to_string()
    );
}

#[test]
fn env_json_custom_full_install_uses_install_metadata_reference_root() {
    let home_dir = unique_temp_dir("custom-full-home");
    let install_root = unique_temp_dir("custom-full-install");
    let installed_binary = install_root.join("custom/bin/entropyfa");
    fs::create_dir_all(
        installed_binary
            .parent()
            .expect("custom binary should have parent"),
    )
    .expect("should create custom binary directory");
    fs::copy(entropyfa_bin(), &installed_binary).expect("should copy test binary");
    let mut perms = fs::metadata(&installed_binary)
        .expect("copied custom binary should exist")
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&installed_binary, perms).expect("should make copied binary executable");

    let custom_reference_root = install_root.join("custom/reference");
    write_install_metadata(&installed_binary, "full", Some(&custom_reference_root));

    let v = run_ok(
        Command::new(installed_binary)
            .args(["env", "--json"])
            .env("HOME", &home_dir)
            .env_remove("ENTROPYFA_REFERENCE_ROOT")
            .env_remove("ENTROPYFA_INSTALL_PROFILE"),
    );

    assert_eq!(v["data"]["install_profile"], "full");
    assert_eq!(
        v["data"]["reference"]["resolved_root"],
        custom_reference_root.display().to_string()
    );
    assert_eq!(
        v["data"]["reference"]["resolution_source"],
        "install-metadata"
    );
}

#[test]
fn env_json_install_metadata_beats_profile_hint_for_reference_root() {
    let home_dir = unique_temp_dir("custom-metadata-beats-profile-home");
    let install_root = unique_temp_dir("custom-metadata-beats-profile-install");
    let installed_binary = install_root.join("custom/bin/entropyfa");
    fs::create_dir_all(
        installed_binary
            .parent()
            .expect("custom binary should have parent"),
    )
    .expect("should create custom binary directory");
    fs::copy(entropyfa_bin(), &installed_binary).expect("should copy test binary");
    let mut perms = fs::metadata(&installed_binary)
        .expect("copied custom binary should exist")
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&installed_binary, perms).expect("should make copied binary executable");

    let custom_reference_root = install_root.join("custom/reference");
    write_install_metadata(&installed_binary, "full", Some(&custom_reference_root));

    let v = run_ok(
        Command::new(installed_binary)
            .args(["env", "--json"])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_INSTALL_PROFILE", "platform")
            .env_remove("ENTROPYFA_REFERENCE_ROOT"),
    );

    assert_eq!(v["data"]["install_profile"], "platform");
    assert_eq!(
        v["data"]["reference"]["resolved_root"],
        custom_reference_root.display().to_string()
    );
    assert_eq!(
        v["data"]["reference"]["resolution_source"],
        "install-metadata"
    );
}

#[test]
fn env_json_platform_install_profile_override_uses_platform_root() {
    let home_dir = unique_temp_dir("platform-override-home");

    let v = run_ok(
        entropyfa()
            .args(["env", "--json"])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_INSTALL_PROFILE", "platform")
            .env_remove("ENTROPYFA_REFERENCE_ROOT"),
    );

    assert_eq!(v["data"]["install_profile"], "platform");
    assert_eq!(v["data"]["reference"]["resolution_source"], "default");
    assert_eq!(
        v["data"]["reference"]["resolved_root"],
        "/opt/entropyfa/reference"
    );
}

#[test]
fn env_json_includes_manifest_metadata_when_available() {
    let reference_root = unique_temp_dir("manifest-present");
    let home_dir = unique_temp_dir("manifest-present-home");
    write_manifest(
        &reference_root,
        r#"{
  "bundle_version": "v2026.03.0",
  "generated_at": "2026-03-22T00:00:00Z",
  "categories": { "tax": ["2026"] },
  "pack_count": 7
}"#,
    );

    let v = run_ok(
        entropyfa()
            .args([
                "env",
                "--json",
                "--reference-root",
                &reference_root.display().to_string(),
            ])
            .env("HOME", &home_dir),
    );

    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["version"], env!("CARGO_PKG_VERSION"));
    assert!(v["data"]["binary_path"].as_str().is_some());
    assert_eq!(v["data"]["install_profile"], "binary-only");
    assert_eq!(v["data"]["reference"]["packs_present"], true);
    assert_eq!(
        v["data"]["reference"]["manifest"]["bundle_version"],
        "v2026.03.0"
    );
    assert_eq!(v["data"]["reference"]["manifest"]["pack_count"], 7);
    assert_eq!(
        v["data"]["reference"]["manifest"]["categories"]["tax"][0],
        "2026"
    );
}

#[test]
fn env_json_unreadable_manifest_does_not_report_packs_present() {
    let reference_root = unique_temp_dir("invalid-manifest");
    let home_dir = unique_temp_dir("invalid-manifest-home");
    write_manifest(&reference_root, "{not-json");

    let v = run_ok(
        entropyfa()
            .args([
                "env",
                "--json",
                "--reference-root",
                &reference_root.display().to_string(),
            ])
            .env("HOME", &home_dir),
    );

    assert_eq!(v["data"]["reference"]["packs_present"], false);
    assert!(v["data"]["reference"]["manifest"].is_null());
}

#[test]
fn env_json_zero_pack_manifest_does_not_report_packs_present() {
    let reference_root = unique_temp_dir("zero-pack-manifest");
    let home_dir = unique_temp_dir("zero-pack-manifest-home");
    write_manifest(
        &reference_root,
        r#"{
  "bundle_version": "dev",
  "generated_at": null,
  "categories": {},
  "pack_count": 0
}"#,
    );

    let v = run_ok(
        entropyfa()
            .args([
                "env",
                "--json",
                "--reference-root",
                &reference_root.display().to_string(),
            ])
            .env("HOME", &home_dir),
    );

    assert_eq!(v["data"]["reference"]["packs_present"], false);
    assert_eq!(v["data"]["reference"]["manifest"]["pack_count"], 0);
}

#[test]
fn env_plain_text_prints_human_summary() {
    let reference_root = unique_temp_dir("plain-summary");
    write_manifest(
        &reference_root,
        r#"{
  "bundle_version": "dev",
  "pack_count": 0
}"#,
    );

    let stdout = run_text(entropyfa().args([
        "env",
        "--reference-root",
        &reference_root.display().to_string(),
    ]));

    assert!(
        stdout.contains("Version:"),
        "plain env output should mention Version: {stdout}"
    );
    assert!(
        stdout.contains("Reference root:"),
        "plain env output should mention Reference root: {stdout}"
    );
    assert!(
        stdout.contains("packs present"),
        "plain env output should mention packs present: {stdout}"
    );
}

// ---------------------------------------------------------------------------
// 3. data coverage → ok:true with entries array
// ---------------------------------------------------------------------------

#[test]
fn data_coverage_returns_entries() {
    let v = run_ok(entropyfa().args(["data", "coverage"]));
    assert_eq!(v["ok"], true);
    let entries = v["data"]["entries"].as_array().expect("entries is array");
    assert!(
        !entries.is_empty(),
        "coverage should return at least one entry"
    );
    assert!(
        v["data"]["count"].as_u64().unwrap() > 0,
        "count should be > 0"
    );
}

// ---------------------------------------------------------------------------
// 4. data coverage --category tax → only tax entries
// ---------------------------------------------------------------------------

#[test]
fn data_coverage_filtered_by_category() {
    let v = run_ok(entropyfa().args(["data", "coverage", "--category", "tax"]));
    assert_eq!(v["ok"], true);
    let entries = v["data"]["entries"].as_array().expect("entries is array");
    assert!(!entries.is_empty(), "should have tax entries");
    for entry in entries {
        assert_eq!(
            entry["category"].as_str().unwrap(),
            "tax",
            "all entries should be category=tax, got: {entry}"
        );
    }
}

// ---------------------------------------------------------------------------
// 5. data lookup brackets → 7 brackets
// ---------------------------------------------------------------------------

#[test]
fn data_lookup_tax_brackets_single() {
    let v = run_ok(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "tax",
        "--key",
        "federal_income_tax_brackets",
        "--filing-status",
        "single",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["category"], "tax");
    assert_eq!(v["data"]["key"], "federal_income_tax_brackets");
    assert_eq!(v["data"]["year"], 2026);
    assert_eq!(v["data"]["verification_status"], "authoritative");
    assert_eq!(v["data"]["pipeline_reviewed"], true);
    let sources = v["data"]["sources"].as_array().expect("sources is array");
    assert!(!sources.is_empty(), "sources should not be empty");
    assert!(
        sources[0]["url"].is_string(),
        "reviewed source should include a URL"
    );
    let brackets = v["data"]["value"].as_array().expect("brackets is array");
    assert_eq!(
        brackets.len(),
        7,
        "single filing status should have 7 brackets, got {}",
        brackets.len()
    );
}

#[test]
fn data_lookup_medicare_base_premiums() {
    let v = run_ok(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "insurance",
        "--key",
        "medicare_base_premiums",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["category"], "insurance");
    assert_eq!(v["data"]["key"], "medicare_base_premiums");
    assert_eq!(v["data"]["verification_status"], "authoritative");
    assert_eq!(v["data"]["pipeline_reviewed"], true);
    let sources = v["data"]["sources"].as_array().expect("sources is array");
    assert!(!sources.is_empty(), "sources should not be empty");
    assert_eq!(v["data"]["value"]["part_b_standard_monthly_premium"], 202.9);
    assert_eq!(v["data"]["value"]["part_b_annual_deductible"], 283.0);
    assert_eq!(v["data"]["value"]["part_d_base_beneficiary_premium"], 38.99);
}

#[test]
fn data_lookup_full_retirement_age_rules() {
    let v = run_ok(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "social_security",
        "--key",
        "full_retirement_age_rules",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["category"], "social_security");
    assert_eq!(v["data"]["key"], "full_retirement_age_rules");
    assert_eq!(v["data"]["verification_status"], "authoritative");
    assert_eq!(v["data"]["pipeline_reviewed"], true);
    assert_eq!(
        v["data"]["value"]["benefit_scope"],
        "retirement_and_spousal"
    );
    assert_eq!(v["data"]["value"]["january_1_births_use_prior_year"], true);
    let rules = v["data"]["value"]["rules"]
        .as_array()
        .expect("rules is array");
    assert_eq!(rules.len(), 13);
    assert_eq!(rules[0]["birth_year_max"], 1937);
    assert_eq!(rules[12]["birth_year_min"], 1960);
    assert_eq!(rules[12]["full_retirement_age_years"], 67);
}

#[test]
fn data_lookup_irmaa_mfs_lived_apart() {
    let v = run_ok(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "insurance",
        "--key",
        "irmaa_brackets",
        "--filing-status",
        "married_filing_separately",
        "--lived-with-spouse-during-year",
        "false",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["category"], "insurance");
    assert_eq!(v["data"]["key"], "irmaa_brackets");
    assert_eq!(v["data"]["verification_status"], "authoritative");
    assert_eq!(v["data"]["pipeline_reviewed"], true);
    assert_eq!(
        v["data"]["value"]["filing_status"],
        "married_filing_separately"
    );
    assert_eq!(v["data"]["value"]["lived_with_spouse_during_year"], false);
    let sources = v["data"]["sources"].as_array().expect("sources is array");
    assert!(!sources.is_empty(), "sources should not be empty");
    assert!(
        sources[0]["url"].is_string(),
        "reviewed source should include a URL"
    );
    let brackets = v["data"]["value"]["brackets"]
        .as_array()
        .expect("brackets is array");
    assert_eq!(brackets.len(), 6);
}

#[test]
fn data_lookup_irmaa_mfs_requires_spouse_flag() {
    let v = run_err(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "insurance",
        "--key",
        "irmaa_brackets",
        "--filing-status",
        "married_filing_separately",
    ]));
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "lookup_error");
    assert!(v["error"]["message"]
        .as_str()
        .unwrap()
        .contains("lived_with_spouse_during_year"));
}

// ---------------------------------------------------------------------------
// 6. data lookup nonexistent category → ok:false
// ---------------------------------------------------------------------------

#[test]
fn data_lookup_unknown_category_returns_error() {
    let v = run_err(entropyfa().args([
        "data",
        "lookup",
        "--category",
        "nonexistent",
        "--key",
        "foo",
    ]));
    assert_eq!(v["ok"], false);
    assert!(
        v["error"]["code"].as_str().is_some(),
        "error should have a code"
    );
}

// ---------------------------------------------------------------------------
// 7. compute federal-tax --schema → schema JSON with gather_from_user
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_schema() {
    let v = run_ok(entropyfa().args(["compute", "federal-tax", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "schema should contain gather_from_user"
    );
    assert!(
        v["data"]["input_schema"].is_object(),
        "schema should contain input_schema"
    );
}

// ---------------------------------------------------------------------------
// 8. compute rmd --schema → schema JSON
// ---------------------------------------------------------------------------

#[test]
fn compute_rmd_schema() {
    let v = run_ok(entropyfa().args(["compute", "rmd", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "rmd schema should contain gather_from_user"
    );
    let required = v["data"]["input_schema"]["required"]
        .as_array()
        .expect("required is array");
    assert!(
        !required
            .iter()
            .any(|value| value.as_str() == Some("rmd_parameters")),
        "rmd schema should not require inline rmd_parameters anymore"
    );
    assert!(
        v["data"]["input_schema"]["properties"]["rmd_parameters"]["description"]
            .as_str()
            .unwrap()
            .contains("Optional override block"),
        "rmd schema should describe rmd_parameters as optional override"
    );
}

#[test]
fn compute_rmd_valid_without_inline_parameters() {
    let reference_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../reference");
    let home_dir = unique_temp_dir("compute-rmd-home");
    let input = r#"{
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira",
        "owner_birth_date": "1953-06-15"
    }"#;

    let v = run_ok(
        entropyfa()
            .args(["compute", "rmd", "--json", input])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["calculation_year"], 2026);
    assert!(v["data"]["rmd_required"].as_bool().is_some());
    assert!(v["data"]["rmd_amount"].as_f64().unwrap() > 0.0);
    assert!(
        v["data"]["citations"].as_array().is_some(),
        "response should include citations"
    );
}

#[test]
fn compute_rmd_object_parameters_bypass_installed_packs() {
    let reference_root = unique_temp_dir("compute-rmd-object-override-empty-root");
    let home_dir = unique_temp_dir("compute-rmd-object-override-home");
    let input = serde_json::json!({
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira",
        "owner_birth_date": "1953-06-15",
        "rmd_parameters": {
            "uniform_lifetime_table": [
                {"age": 73, "distribution_period": 26.5}
            ],
            "joint_life_table": [
                {"owner_age": 73, "spouse_age": 72, "distribution_period": 27.4}
            ],
            "single_life_table": [
                {"age": 73, "distribution_period": 26.5}
            ],
            "required_beginning": {
                "start_age_rules": [
                    {
                        "birth_year_min": 1951,
                        "birth_year_max": 1959,
                        "start_age": 73,
                        "guidance_status": null,
                        "notes": null
                    }
                ],
                "first_distribution_deadline": "april_1_following_year",
                "still_working_exception_plan_categories": [],
                "still_working_exception_eligible_account_types": [],
                "still_working_exception_disallowed_for_five_percent_owners": true
            },
            "account_rules": {
                "owner_required_account_types": ["traditional_ira"],
                "owner_exempt_account_types": ["roth_ira"],
                "inherited_account_types": ["inherited_ira"],
                "supports_pre_1987_403b_exclusion": true,
                "designated_roth_owner_exemption_effective_year": null
            },
            "beneficiary_rules": {
                "beneficiary_categories": [],
                "recognized_beneficiary_classes": [],
                "eligible_designated_beneficiary_classes": [],
                "life_expectancy_method_by_class": {},
                "minor_child_majority_age": 21,
                "spouse_delay_allowed": true,
                "non_designated_beneficiary_rules": {
                    "when_owner_died_before_required_beginning_date": "five_year_rule",
                    "when_owner_died_on_or_after_required_beginning_date": "owner_remaining_life_expectancy"
                }
            },
            "ten_year_rule": {
                "terminal_year": 10,
                "annual_distributions_required_when_owner_died_on_or_after_rbd": true
            },
            "relief_years": [],
            "pre_1987_403b_rules": {
                "exclude_until_age": 75
            }
        }
    });

    let v = run_ok(
        entropyfa()
            .args(["compute", "rmd", "--json", &input.to_string()])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["calculation_year"], 2026);
    assert!(v["data"]["rmd_amount"].as_f64().unwrap() > 0.0);
}

#[test]
fn compute_rmd_missing_reference_pack() {
    let reference_root = unique_temp_dir("compute-rmd-missing-pack");
    let home_dir = unique_temp_dir("compute-rmd-missing-pack-home");
    let input = r#"{
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira",
        "owner_birth_date": "1953-06-15"
    }"#;

    let v = run_err(
        entropyfa()
            .args(["compute", "rmd", "--json", input])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "reference_pack_missing");
    assert!(
        v["error"]["message"]
            .as_str()
            .unwrap()
            .contains("missing retirement reference pack"),
        "missing-pack error should be explicit: {}",
        v["error"]["message"]
    );
}

#[test]
fn compute_rmd_invalid_reference_pack() {
    let reference_root = unique_temp_dir("compute-rmd-invalid-pack");
    let home_dir = unique_temp_dir("compute-rmd-invalid-pack-home");
    let year_root = reference_root.join("retirement/2026");
    fs::create_dir_all(&year_root).expect("year root should be creatable");
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../reference/retirement/2026/uniform_lifetime_table.md"),
        year_root.join("uniform_lifetime_table.md"),
    )
    .expect("should copy pack");
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../reference/retirement/2026/single_life_expectancy_table.md"),
        year_root.join("single_life_expectancy_table.md"),
    )
    .expect("should copy pack");
    fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../reference/retirement/2026/joint_life_table.md"),
        year_root.join("joint_life_table.md"),
    )
    .expect("should copy pack");
    fs::write(
        year_root.join("distribution_rules.md"),
        "## Machine Block\n\n```json\n{ not-json\n```\n",
    )
    .expect("should write malformed pack");

    let input = r#"{
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira",
        "owner_birth_date": "1953-06-15"
    }"#;

    let v = run_err(
        entropyfa()
            .args(["compute", "rmd", "--json", input])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "reference_pack_invalid");
}

#[test]
fn compute_rmd_non_object_parameters_are_assembly_error() {
    let reference_root = unique_temp_dir("compute-rmd-bad-override");
    let home_dir = unique_temp_dir("compute-rmd-bad-override-home");
    let input = r#"{
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira",
        "owner_birth_date": "1953-06-15",
        "rmd_parameters": []
    }"#;

    let v = run_err(
        entropyfa()
            .args(["compute", "rmd", "--json", input])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "assembly_error");
    assert!(
        v["error"]["message"]
            .as_str()
            .unwrap()
            .contains("rmd_parameters must be null or an object"),
        "non-object override should fail fast: {}",
        v["error"]["message"]
    );
}

#[test]
fn compute_rmd_malformed_input_stays_assembly_error_with_missing_reference_pack() {
    let reference_root = unique_temp_dir("compute-rmd-malformed-pack");
    let home_dir = unique_temp_dir("compute-rmd-malformed-pack-home");

    let v = run_err(
        entropyfa()
            .args(["compute", "rmd", "--json", "{}"])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "assembly_error");
    assert!(
        v["error"]["message"]
            .as_str()
            .unwrap()
            .contains("missing required field"),
        "malformed input should fail before pack loading: {}",
        v["error"]["message"]
    );
}

#[test]
fn compute_rmd_missing_owner_birth_date_stays_assembly_error_with_missing_reference_pack() {
    let reference_root = unique_temp_dir("compute-rmd-missing-owner-bd-pack");
    let home_dir = unique_temp_dir("compute-rmd-missing-owner-bd-pack-home");
    let input = r#"{
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira"
    }"#;

    let v = run_err(
        entropyfa()
            .args(["compute", "rmd", "--json", input])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "assembly_error");
    assert!(
        v["error"]["message"]
            .as_str()
            .unwrap()
            .contains("owner_birth_date"),
        "missing owner_birth_date should fail before pack loading: {}",
        v["error"]["message"]
    );
}

#[test]
fn compute_rmd_null_parameters_falls_back_to_installed_packs() {
    let reference_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../reference");
    let home_dir = unique_temp_dir("compute-rmd-null-params-home");
    let input = r#"{
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira",
        "owner_birth_date": "1953-06-15",
        "rmd_parameters": null
    }"#;

    let v = run_ok(
        entropyfa()
            .args(["compute", "rmd", "--json", input])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["calculation_year"], 2026);
}

#[test]
fn compute_rmd_schedule_null_parameters_falls_back_to_installed_packs() {
    let reference_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../reference");
    let home_dir = unique_temp_dir("compute-rmd-schedule-null-params-home");
    let input = r#"{
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira",
        "owner_birth_date": "1953-06-15",
        "annual_growth_rate": 0.05,
        "rmd_parameters": null
    }"#;

    let v = run_ok(
        entropyfa()
            .args(["compute", "rmd-schedule", "--json", input])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["rows"].as_array().is_some(),
        "schedule response should include rows"
    );
}

#[test]
fn compute_rmd_schedule_object_parameters_bypass_installed_packs() {
    let reference_root = unique_temp_dir("compute-rmd-schedule-object-override-empty-root");
    let home_dir = unique_temp_dir("compute-rmd-schedule-object-override-home");
    let input = serde_json::json!({
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira",
        "owner_birth_date": "1953-06-15",
        "annual_growth_rate": 0.05,
        "max_years": 1,
        "rmd_parameters": {
            "uniform_lifetime_table": [
                {"age": 73, "distribution_period": 26.5}
            ],
            "joint_life_table": [
                {"owner_age": 73, "spouse_age": 72, "distribution_period": 27.4}
            ],
            "single_life_table": [
                {"age": 73, "distribution_period": 26.5}
            ],
            "required_beginning": {
                "start_age_rules": [
                    {
                        "birth_year_min": 1951,
                        "birth_year_max": 1959,
                        "start_age": 73,
                        "guidance_status": null,
                        "notes": null
                    }
                ],
                "first_distribution_deadline": "april_1_following_year",
                "still_working_exception_plan_categories": [],
                "still_working_exception_eligible_account_types": [],
                "still_working_exception_disallowed_for_five_percent_owners": true
            },
            "account_rules": {
                "owner_required_account_types": ["traditional_ira"],
                "owner_exempt_account_types": ["roth_ira"],
                "inherited_account_types": ["inherited_ira"],
                "supports_pre_1987_403b_exclusion": true,
                "designated_roth_owner_exemption_effective_year": null
            },
            "beneficiary_rules": {
                "beneficiary_categories": [],
                "recognized_beneficiary_classes": [],
                "eligible_designated_beneficiary_classes": [],
                "life_expectancy_method_by_class": {},
                "minor_child_majority_age": 21,
                "spouse_delay_allowed": true,
                "non_designated_beneficiary_rules": {
                    "when_owner_died_before_required_beginning_date": "five_year_rule",
                    "when_owner_died_on_or_after_required_beginning_date": "owner_remaining_life_expectancy"
                }
            },
            "ten_year_rule": {
                "terminal_year": 10,
                "annual_distributions_required_when_owner_died_on_or_after_rbd": true
            },
            "relief_years": [],
            "pre_1987_403b_rules": {
                "exclude_until_age": 75
            }
        }
    });

    let v = run_ok(
        entropyfa()
            .args(["compute", "rmd-schedule", "--json", &input.to_string()])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["annual_growth_rate"], 0.05);
    assert_eq!(v["data"]["rows"].as_array().unwrap().len(), 1);
}

#[test]
fn compute_rmd_schedule_missing_annual_growth_rate_stays_assembly_error_with_missing_reference_pack(
) {
    let reference_root = unique_temp_dir("compute-rmd-schedule-missing-growth-pack");
    let home_dir = unique_temp_dir("compute-rmd-schedule-missing-growth-pack-home");
    let input = r#"{
        "calculation_year": 2026,
        "prior_year_end_balance": 500000,
        "account_type": "traditional_ira",
        "owner_birth_date": "1953-06-15"
    }"#;

    let v = run_err(
        entropyfa()
            .args(["compute", "rmd-schedule", "--json", input])
            .env("HOME", &home_dir)
            .env("ENTROPYFA_REFERENCE_ROOT", &reference_root),
    );

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "assembly_error");
    assert!(
        v["error"]["message"]
            .as_str()
            .unwrap()
            .contains("annual_growth_rate"),
        "missing annual_growth_rate should fail before pack loading: {}",
        v["error"]["message"]
    );
}

#[test]
fn env_json_reports_retirement_manifest_and_pack_files() {
    let reference_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../reference");
    let home_dir = unique_temp_dir("retirement-manifest-home");

    let env = run_ok(
        entropyfa()
            .args([
                "env",
                "--json",
                "--reference-root",
                &reference_root.display().to_string(),
            ])
            .env("HOME", &home_dir),
    );
    assert_eq!(env["ok"], true);
    assert_eq!(
        env["data"]["reference"]["manifest"]["bundle_version"],
        "dev"
    );
    assert_eq!(env["data"]["reference"]["manifest"]["pack_count"], 4);
    assert_eq!(env["data"]["reference"]["packs_present"], true);
    let retirement = env["data"]["reference"]["manifest"]["categories"]["retirement"]
        .as_array()
        .expect("retirement category should be an array");
    assert!(
        retirement.iter().any(|year| year == "2026"),
        "retirement category should include 2026"
    );

    for file_name in [
        "distribution_rules.md",
        "uniform_lifetime_table.md",
        "single_life_expectancy_table.md",
        "joint_life_table.md",
    ] {
        let pack_path = reference_root.join("retirement/2026").join(file_name);
        assert!(
            pack_path.is_file(),
            "expected retirement pack file to exist: {}",
            pack_path.display()
        );
    }
}

// ---------------------------------------------------------------------------
// 9. compute federal-tax with --json → ok:true
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_valid_input() {
    let input = r#"{"filing_status":"single","income":{"wages":100000}}"#;
    let v = run_ok(entropyfa().args(["compute", "federal-tax", "--json", input]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["total_tax"].as_f64().unwrap() > 0.0,
        "total_tax should be positive for $100k wages"
    );
    assert!(
        v["data"]["effective_rate"].as_f64().is_some(),
        "response should include effective_rate"
    );
}

// ---------------------------------------------------------------------------
// 10. compute federal-tax without --json → ok:false + exit code 1
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_missing_json_flag() {
    let v = run_err(entropyfa().args(["compute", "federal-tax"]));
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "missing_json");
}

// ---------------------------------------------------------------------------
// 11. compute federal-tax with invalid --json → ok:false + exit code 1
// ---------------------------------------------------------------------------

#[test]
fn compute_federal_tax_invalid_json() {
    let v = run_err(entropyfa().args(["compute", "federal-tax", "--json", "not-json"]));
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "invalid_json");
}

// ---------------------------------------------------------------------------
// 12. compute estate-tax with --json → ok:true
// ---------------------------------------------------------------------------

#[test]
fn compute_estate_tax_valid_input() {
    let input = r#"{"gross_estate":20000000,"deductions":{"marital":0,"charitable":0,"debts_and_expenses":100000}}"#;
    let v = run_ok(entropyfa().args(["compute", "estate-tax", "--json", input]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["net_estate_tax"].as_f64().is_some(),
        "response should include net_estate_tax"
    );
}

// ---------------------------------------------------------------------------
// 13. Each --schema command works (renamed commands)
// ---------------------------------------------------------------------------

#[test]
fn compute_estate_tax_schema() {
    let v = run_ok(entropyfa().args(["compute", "estate-tax", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "estate-tax schema should contain gather_from_user"
    );
}

#[test]
fn compute_pension_comparison_schema() {
    let v = run_ok(entropyfa().args(["compute", "pension-comparison", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "pension-comparison schema should contain gather_from_user"
    );
}

#[test]
fn compute_projection_schema() {
    let v = run_ok(entropyfa().args(["compute", "projection", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "projection schema should contain gather_from_user"
    );
    let when_to_use = v["data"]["when_to_use"]
        .as_str()
        .expect("when_to_use should be a string");
    assert!(
        when_to_use.contains("--visual"),
        "projection schema should mention --visual: {when_to_use}"
    );
}

#[test]
fn compute_projection_schema_mentions_bucketed_and_legacy_requests() {
    let v = run_ok(entropyfa().args(["compute", "projection", "--schema"]));
    assert_eq!(v["ok"], true);

    let when_to_use = v["data"]["when_to_use"]
        .as_str()
        .expect("when_to_use should be a string");
    assert!(
        when_to_use.contains("bucket"),
        "projection schema should mention bucketed requests: {when_to_use}"
    );
    assert!(
        when_to_use.contains("legacy"),
        "projection schema should mention legacy compatibility: {when_to_use}"
    );

    let properties = v["data"]["input_schema"]["properties"]
        .as_object()
        .expect("input_schema.properties should be an object");
    for key in [
        "buckets",
        "filing_status",
        "household",
        "spending_policy",
        "tax_policy",
        "rmd_policy",
        "starting_balance",
        "return_assumption",
    ] {
        assert!(
            properties.contains_key(key),
            "projection schema should expose {key}"
        );
    }

    let tax_policy = v["data"]["input_schema"]["properties"]["tax_policy"]
        .as_object()
        .expect("tax_policy should be an object");
    let tax_policy_description = tax_policy
        .get("description")
        .and_then(|value| value.as_str())
        .expect("tax_policy description should be a string");
    assert!(
        tax_policy_description.contains("embedded data"),
        "tax_policy should mention embedded data: {tax_policy_description}"
    );
    assert!(
        tax_policy_description.contains("modeled"),
        "tax_policy should mention modeled behavior after supported years: {tax_policy_description}"
    );
    let tax_policy_mode = tax_policy
        .get("properties")
        .and_then(|value| value.as_object())
        .and_then(|properties| properties.get("mode"))
        .and_then(|value| value.as_object())
        .expect("tax_policy.mode should be an object");
    let tax_policy_mode_description = tax_policy_mode
        .get("description")
        .and_then(|value| value.as_str())
        .expect("tax_policy.mode description should be a string");
    assert!(
        tax_policy_mode_description.contains("none")
            && tax_policy_mode_description.contains("embedded_federal")
            && tax_policy_mode_description.contains("modeled"),
        "tax_policy.mode should document the allowed modes: {tax_policy_mode_description}"
    );
    let tax_policy_mode_enum = tax_policy_mode
        .get("enum")
        .and_then(|value| value.as_array())
        .expect("tax_policy.mode enum should be an array");
    assert_eq!(
        tax_policy_mode_enum,
        &vec![
            serde_json::json!("none"),
            serde_json::json!("embedded_federal"),
            serde_json::json!("modeled"),
        ]
    );

    let filing_status = v["data"]["input_schema"]["properties"]["filing_status"]
        .as_object()
        .expect("filing_status should be an object");
    let filing_status_description = filing_status
        .get("description")
        .and_then(|value| value.as_str())
        .expect("filing_status description should be a string");
    assert!(
        filing_status_description.contains("tax"),
        "filing_status should explain why it matters: {filing_status_description}"
    );

    let household = v["data"]["input_schema"]["properties"]["household"]
        .as_object()
        .expect("household should be an object");
    let household_description = household
        .get("description")
        .and_then(|value| value.as_str())
        .expect("household description should be a string");
    assert!(
        household_description.contains("RMD"),
        "household should explain when it matters: {household_description}"
    );

    let terminal_dashboard = v["data"]["output_summary"]["terminal_dashboard"]
        .as_str()
        .expect("terminal_dashboard summary should be a string");
    assert!(
        terminal_dashboard.contains("aggregate"),
        "projection schema should note that the terminal dashboard stays aggregate-only: {terminal_dashboard}"
    );
}

#[test]
fn compute_projection_schema_includes_output_schema_and_preserves_existing_keys() {
    let v = run_ok(entropyfa().args(["compute", "projection", "--schema"]));
    assert_eq!(v["ok"], true);

    let data = v["data"]
        .as_object()
        .expect("schema data should be an object");
    for key in [
        "command",
        "description",
        "when_to_use",
        "gather_from_user",
        "input_schema",
        "output_summary",
        "example",
        "related_commands",
        "output_schema",
        "normalization_rules",
        "invariants",
    ] {
        assert!(
            data.contains_key(key),
            "projection schema should preserve/add top-level key {key}"
        );
    }

    let output_schema = data
        .get("output_schema")
        .and_then(|value| value.as_object())
        .expect("output_schema should be an object");
    for key in [
        "success_envelope",
        "error_envelope",
        "projection_response",
        "linear_result",
        "monte_carlo_result",
        "period_detail",
        "monte_carlo_detail_row",
    ] {
        assert!(
            output_schema.contains_key(key),
            "output_schema should expose {key}"
        );
    }

    let projection_response = output_schema["projection_response"]
        .as_object()
        .expect("projection_response should be an object");
    let request_echo = projection_response["request_echo"]
        .as_str()
        .expect("projection_response.request_echo should be a string");
    assert!(
        request_echo.contains("assembled")
            && request_echo.contains("CLI")
            && request_echo.contains("not raw JSON"),
        "request_echo should document assembled-request semantics: {request_echo}"
    );

    let linear_result = output_schema["linear_result"]
        .as_object()
        .expect("linear_result should be an object");
    for key in [
        "final_balance",
        "time_series",
        "total_contributions",
        "total_withdrawals",
        "total_return_earned",
        "ending_balances_by_bucket",
        "annual_detail",
    ] {
        assert!(
            linear_result.contains_key(key),
            "linear_result should expose {key}"
        );
    }

    let monte_carlo_result = output_schema["monte_carlo_result"]
        .as_object()
        .expect("monte_carlo_result should be an object");
    for key in [
        "num_simulations",
        "percentiles",
        "mean",
        "std_dev",
        "min",
        "max",
        "success_rate",
        "paths_depleted_by_month",
        "survival_by_year",
        "balance_histogram",
        "time_series",
        "annual_detail",
        "sample_paths",
        "custom_percentile_series",
        "bucket_terminal_percentiles",
        "bucket_depletion_counts",
    ] {
        assert!(
            monte_carlo_result.contains_key(key),
            "monte_carlo_result should expose {key}"
        );
    }
}

#[test]
fn compute_projection_schema_documents_detail_row_shapes_and_omission_semantics() {
    let v = run_ok(entropyfa().args(["compute", "projection", "--schema"]));
    assert_eq!(v["ok"], true);

    let output_schema = v["data"]["output_schema"]
        .as_object()
        .expect("output_schema should be an object");

    let period_detail = output_schema["period_detail"]
        .as_object()
        .expect("period_detail should be an object");
    for key in [
        "period",
        "year",
        "beginning_balance",
        "contributions",
        "withdrawals",
        "investment_return",
        "ending_balance",
        "cumulative_contributions",
        "cumulative_withdrawals",
        "cumulative_return",
        "annual_tax_paid",
        "bucket_withdrawals",
        "ending_balances_by_bucket",
    ] {
        assert!(
            period_detail.contains_key(key),
            "period_detail should expose {key}"
        );
    }

    let monte_carlo_detail_row = output_schema["monte_carlo_detail_row"]
        .as_object()
        .expect("monte_carlo_detail_row should be an object");
    for key in [
        "period",
        "year",
        "balance_p10",
        "balance_p25",
        "balance_p50",
        "balance_p75",
        "balance_p90",
        "net_cash_flow",
        "cumulative_cash_flow",
        "annual_tax_paid",
        "survival_rate",
    ] {
        assert!(
            monte_carlo_detail_row.contains_key(key),
            "monte_carlo_detail_row should expose {key}"
        );
    }

    let net_cash_flow = monte_carlo_detail_row["net_cash_flow"]
        .as_str()
        .expect("monte_carlo_detail_row.net_cash_flow should be a string");
    assert!(
        net_cash_flow.contains("configured cash flows")
            && net_cash_flow.contains("not a percentile"),
        "monte_carlo_detail_row.net_cash_flow should distinguish deterministic flow from percentile data: {net_cash_flow}"
    );

    let invariants = v["data"]["invariants"]
        .as_array()
        .expect("invariants should be an array");
    let invariant_text: Vec<&str> = invariants
        .iter()
        .filter_map(|value| value.as_str())
        .collect();
    assert!(
        invariant_text
            .iter()
            .any(|value| value.contains("linear") && value.contains("omitted")),
        "invariants should document linear omission semantics: {:?}",
        invariant_text
    );
    assert!(
        invariant_text
            .iter()
            .any(|value| value.contains("monte_carlo") && value.contains("omitted")),
        "invariants should document monte_carlo omission semantics: {:?}",
        invariant_text
    );
    assert!(
        invariant_text
            .iter()
            .any(|value| value.contains("detail arrays") && value.contains("omitted")),
        "invariants should document detail omission semantics: {:?}",
        invariant_text
    );
}

#[test]
fn compute_projection_schema_documents_normalization_rules_for_current_cli_contract() {
    let v = run_ok(entropyfa().args(["compute", "projection", "--schema"]));
    assert_eq!(v["ok"], true);

    let normalization_rules = v["data"]["normalization_rules"]
        .as_object()
        .expect("normalization_rules should be an object");
    for key in [
        "cli_defaults",
        "assembly_surface",
        "validation_constraints",
        "tax_rules",
        "detail_semantics",
    ] {
        assert!(
            normalization_rules.contains_key(key),
            "normalization_rules should expose {key}"
        );
    }

    let cli_defaults = normalization_rules["cli_defaults"]
        .as_array()
        .expect("cli_defaults should be an array");
    let cli_default_text: Vec<&str> = cli_defaults
        .iter()
        .filter_map(|value| value.as_str())
        .collect();
    assert!(
        cli_default_text
            .iter()
            .any(|value| value.contains("mode") && value.contains("both")),
        "cli_defaults should describe the mode default: {:?}",
        cli_default_text
    );
    assert!(
        cli_default_text
            .iter()
            .any(|value| value.contains("--visual") && value.contains("custom_percentiles")),
        "cli_defaults should describe visual percentile injection: {:?}",
        cli_default_text
    );

    let assembly_surface = normalization_rules["assembly_surface"]
        .as_array()
        .expect("assembly_surface should be an array");
    let assembly_text: Vec<&str> = assembly_surface
        .iter()
        .filter_map(|value| value.as_str())
        .collect();
    assert!(
        assembly_text
            .iter()
            .any(|value| value.contains("request_echo") && value.contains("assembled")),
        "assembly_surface should document request_echo semantics: {:?}",
        assembly_text
    );
    assert!(
        assembly_text
            .iter()
            .any(|value| value.contains("cash_flows") && value.contains("amount")),
        "assembly_surface should document the current cash_flows shape: {:?}",
        assembly_text
    );
    assert!(
        assembly_text.iter().any(|value| {
            value.contains("empty `buckets`")
                && value.contains("legacy")
                && value.contains("validation")
        }),
        "assembly_surface should document empty-buckets legacy behavior: {:?}",
        assembly_text
    );

    let validation_constraints = normalization_rules["validation_constraints"]
        .as_array()
        .expect("validation_constraints should be an array");
    let validation_text: Vec<&str> = validation_constraints
        .iter()
        .filter_map(|value| value.as_str())
        .collect();
    for required_snippet in [
        "bucket IDs must be unique",
        "withdrawal_order",
        "exactly once",
        "same-age",
    ] {
        assert!(
            validation_text
                .iter()
                .any(|value| value.contains(required_snippet)),
            "validation_constraints should mention {required_snippet}: {:?}",
            validation_text
        );
    }
}

#[test]
fn compute_projection_help_shows_visual_flag() {
    let output = entropyfa()
        .args(["compute", "projection", "--help"])
        .output()
        .expect("failed to execute");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("help stdout should be utf-8");
    assert!(
        stdout.contains("--visual"),
        "projection help should mention --visual: {stdout}"
    );
    assert!(
        !stdout.contains("--tui"),
        "projection help should not mention removed alias --tui: {stdout}"
    );
}

#[test]
fn compute_projection_does_not_auto_add_visual_percentiles() {
    let v = run_ok(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"starting_balance\":1000,\"time_horizon_months\":12,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.1},\"num_simulations\":10,\"seed\":1,\"mode\":\"monte_carlo\"}",
    ]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["request_echo"]["custom_percentiles"].is_null(),
        "custom_percentiles should stay null unless --visual or --percentiles is requested: {}",
        v
    );
}

#[test]
fn compute_projection_bucketed_request_returns_bucket_aware_output() {
    let v = run_ok(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"buckets\":[{\"id\":\"taxable\",\"bucket_type\":\"taxable\",\"starting_balance\":60000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.1}},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":40000,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.08}}],\"time_horizon_months\":12,\"spending_policy\":{\"withdrawal_order\":[\"taxable\",\"ira\"]},\"num_simulations\":32,\"seed\":7}",
    ]));

    assert_eq!(v["ok"], true);
    assert_eq!(
        v["data"]["request_echo"]["buckets"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(
        v["data"]["monte_carlo"]["bucket_terminal_percentiles"].is_object(),
        "bucketed projection should include Monte Carlo bucket summaries"
    );
    assert!(
        v["data"]["monte_carlo"]["bucket_depletion_counts"].is_object(),
        "bucketed projection should include bucket depletion counts"
    );
    assert!(
        v["data"]["linear"]["ending_balances_by_bucket"].is_object(),
        "bucketed projection should include linear bucket balances"
    );
    assert!(
        v["data"]["monte_carlo"]["bucket_terminal_percentiles"]["__household_cash__"].is_object(),
        "bucketed projection should include the synthetic household cash summary bucket"
    );
}

#[test]
fn compute_projection_legacy_request_still_works() {
    let v = run_ok(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"starting_balance\":1000,\"time_horizon_months\":12,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.1},\"num_simulations\":10,\"seed\":1,\"mode\":\"monte_carlo\"}",
    ]));

    assert_eq!(v["ok"], true);
    assert_eq!(
        v["data"]["request_echo"]["starting_balance"].as_f64(),
        Some(1000.0)
    );
    assert!(
        v["data"]["request_echo"]["buckets"]
            .as_array()
            .expect("buckets should be an array")
            .is_empty(),
        "legacy projection should not echo any buckets"
    );
    assert!(
        v["data"]["monte_carlo"]["bucket_terminal_percentiles"].is_null(),
        "legacy projection should not include bucket summaries"
    );
}

#[test]
fn compute_projection_rejects_reserved_household_cash_bucket_id() {
    let v = run_err(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"buckets\":[{\"id\":\"__household_cash__\",\"bucket_type\":\"taxable\",\"starting_balance\":60000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.1}},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":40000,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.08}}],\"time_horizon_months\":12,\"spending_policy\":{\"withdrawal_order\":[\"__household_cash__\",\"ira\"],\"rebalance_tax_withholding_from\":\"__household_cash__\"},\"num_simulations\":32,\"seed\":7}",
    ]));

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "validation_error");
    assert!(
        v["error"]["message"]
            .as_str()
            .expect("error message should be a string")
            .contains("__household_cash__"),
        "reserved bucket id should be rejected"
    );
}

#[test]
fn compute_projection_rejects_invalid_filing_status_for_tax_enabled_bucketed_requests() {
    let v = run_err(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"buckets\":[{\"id\":\"taxable\",\"bucket_type\":\"taxable\",\"starting_balance\":60000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.1}},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":40000,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.08}}],\"time_horizon_months\":12,\"spending_policy\":{\"withdrawal_order\":[\"taxable\",\"ira\"]},\"tax_policy\":{\"mode\":\"modeled\"},\"filing_status\":\"invalid\",\"num_simulations\":32,\"seed\":7}",
    ]));

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "validation_error");
}

#[test]
fn compute_projection_rejects_missing_filing_status_for_tax_enabled_bucketed_requests() {
    let v = run_err(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"buckets\":[{\"id\":\"taxable\",\"bucket_type\":\"taxable\",\"starting_balance\":60000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.1}},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":40000,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.08}}],\"time_horizon_months\":12,\"spending_policy\":{\"withdrawal_order\":[\"taxable\",\"ira\"]},\"tax_policy\":{\"mode\":\"embedded_federal\"},\"num_simulations\":32,\"seed\":7}",
    ]));

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "validation_error");
}

#[test]
fn compute_projection_rejects_invalid_tax_policy_mode() {
    let v = run_err(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"buckets\":[{\"id\":\"taxable\",\"bucket_type\":\"taxable\",\"starting_balance\":60000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.1}},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":40000,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.08}}],\"time_horizon_months\":12,\"spending_policy\":{\"withdrawal_order\":[\"taxable\",\"ira\"]},\"tax_policy\":{\"mode\":\"bogus\"},\"filing_status\":\"single\",\"num_simulations\":32,\"seed\":7}",
    ]));

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "validation_error");
}

#[test]
fn compute_projection_rejects_incomplete_withdrawal_order() {
    let v = run_err(entropyfa().args([
        "compute",
        "projection",
        "--json",
        "{\"buckets\":[{\"id\":\"taxable\",\"bucket_type\":\"taxable\",\"starting_balance\":60000,\"return_assumption\":{\"annual_mean\":0.06,\"annual_std_dev\":0.1}},{\"id\":\"ira\",\"bucket_type\":\"tax_deferred\",\"starting_balance\":40000,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.08}}],\"time_horizon_months\":12,\"spending_policy\":{\"withdrawal_order\":[\"taxable\"],\"rebalance_tax_withholding_from\":\"taxable\"},\"tax_policy\":{\"mode\":\"embedded_federal\"},\"filing_status\":\"single\",\"num_simulations\":32,\"seed\":7}",
    ]));

    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "validation_error");
}

#[test]
fn compute_projection_visual_adds_dashboard_percentiles() {
    let v = run_ok(entropyfa().args([
        "compute",
        "projection",
        "--visual",
        "--json",
        "{\"starting_balance\":1000,\"time_horizon_months\":12,\"return_assumption\":{\"annual_mean\":0.05,\"annual_std_dev\":0.1},\"num_simulations\":10,\"seed\":1,\"mode\":\"monte_carlo\"}",
    ]));
    assert_eq!(v["ok"], true);
    assert_eq!(
        v["data"]["request_echo"]["custom_percentiles"],
        serde_json::json!([20, 80])
    );
}

#[test]
fn compute_goal_solver_schema() {
    let v = run_ok(entropyfa().args(["compute", "goal-solver", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "goal-solver schema should contain gather_from_user"
    );
}

#[test]
fn compute_roth_conversion_schema() {
    let v = run_ok(entropyfa().args(["compute", "roth-conversion", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "roth-conversion schema should contain gather_from_user"
    );
}

#[test]
fn compute_roth_conversion_strategy_schema() {
    let v = run_ok(entropyfa().args(["compute", "roth-conversion-strategy", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "roth-conversion-strategy schema should contain gather_from_user"
    );
}

#[test]
fn compute_rmd_schedule_schema() {
    let v = run_ok(entropyfa().args(["compute", "rmd-schedule", "--schema"]));
    assert_eq!(v["ok"], true);
    assert!(
        v["data"]["gather_from_user"].is_object(),
        "rmd-schedule schema should contain gather_from_user"
    );
    let required = v["data"]["input_schema"]["required"]
        .as_array()
        .expect("required is array");
    assert!(
        !required
            .iter()
            .any(|value| value.as_str() == Some("rmd_parameters")),
        "rmd-schedule schema should not require inline rmd_parameters anymore"
    );
}
