use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use entropyfa_engine::data_pipeline;
use serde_json::{json, Value};
use tempfile::TempDir;

fn actual_engine_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn setup_temp_engine_root() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let engine_root = temp_dir.path().join("engine");

    fs::create_dir_all(engine_root.join("data_registry/2025")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/2026")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/insurance")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/pension")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/retirement")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/social_security")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/tax")).unwrap();
    fs::create_dir_all(engine_root.join("src/data/insurance")).unwrap();
    fs::create_dir_all(engine_root.join("src/data/pension")).unwrap();
    fs::create_dir_all(engine_root.join("src/data/retirement")).unwrap();
    fs::create_dir_all(engine_root.join("src/data/social_security")).unwrap();
    fs::create_dir_all(engine_root.join("src/data/tax")).unwrap();

    copy_file(
        &actual_engine_root().join("data_registry/2025/metadata.json"),
        &engine_root.join("data_registry/2025/metadata.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/2026/metadata.json"),
        &engine_root.join("data_registry/2026/metadata.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/insurance/irmaa_brackets.json"),
        &engine_root.join("data_registry/pipelines/insurance/irmaa_brackets.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/insurance/medicare_base_premiums.json"),
        &engine_root.join("data_registry/pipelines/insurance/medicare_base_premiums.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/pension/mortality_417e.json"),
        &engine_root.join("data_registry/pipelines/pension/mortality_417e.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/tax/federal_income_tax_brackets.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_income_tax_brackets.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/tax/federal_standard_deductions.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_standard_deductions.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/tax/federal_capital_loss_limit.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_capital_loss_limit.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/pipelines/tax/federal_net_investment_income_tax.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_net_investment_income_tax.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/pipelines/tax/federal_payroll_tax_parameters.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_payroll_tax_parameters.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/tax/federal_estate_exemption.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_estate_exemption.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/pipelines/tax/federal_estate_applicable_credit.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_estate_applicable_credit.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/tax/federal_estate_brackets.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_estate_brackets.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/pipelines/tax/federal_capital_gains_brackets.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_capital_gains_brackets.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/tax/federal_qbi_deduction.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_qbi_deduction.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/pipelines/tax/federal_salt_deduction_parameters.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_salt_deduction_parameters.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/retirement/distribution_rules.json"),
        &engine_root.join("data_registry/pipelines/retirement/distribution_rules.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/pipelines/retirement/uniform_lifetime_table.json"),
        &engine_root.join("data_registry/pipelines/retirement/uniform_lifetime_table.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/retirement/single_life_table.json"),
        &engine_root.join("data_registry/pipelines/retirement/single_life_table.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/retirement/joint_life_table.json"),
        &engine_root.join("data_registry/pipelines/retirement/joint_life_table.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/pipelines/social_security/benefit_taxation_thresholds.json"),
        &engine_root
            .join("data_registry/pipelines/social_security/benefit_taxation_thresholds.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/pipelines/social_security/full_retirement_age_rules.json"),
        &engine_root.join("data_registry/pipelines/social_security/full_retirement_age_rules.json"),
    );
    copy_file(
        &actual_engine_root().join("src/data/tax/federal.rs"),
        &engine_root.join("src/data/tax/federal.rs"),
    );
    copy_file(
        &actual_engine_root().join("src/data/insurance/medicare.rs"),
        &engine_root.join("src/data/insurance/medicare.rs"),
    );
    copy_file(
        &actual_engine_root().join("src/data/social_security/taxation.rs"),
        &engine_root.join("src/data/social_security/taxation.rs"),
    );
    copy_file(
        &actual_engine_root().join("src/data/social_security/retirement_age.rs"),
        &engine_root.join("src/data/social_security/retirement_age.rs"),
    );
    copy_file(
        &actual_engine_root().join("src/data/tax/estate.rs"),
        &engine_root.join("src/data/tax/estate.rs"),
    );
    copy_file(
        &actual_engine_root().join("src/data/pension/mortality.rs"),
        &engine_root.join("src/data/pension/mortality.rs"),
    );
    copy_file(
        &actual_engine_root().join("src/data/retirement/rmd_rules.rs"),
        &engine_root.join("src/data/retirement/rmd_rules.rs"),
    );
    copy_file(
        &actual_engine_root().join("src/data/retirement/rmd_tables.rs"),
        &engine_root.join("src/data/retirement/rmd_tables.rs"),
    );

    (temp_dir, engine_root)
}

fn copy_file(source: &Path, target: &Path) {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::copy(source, target).unwrap();
}

fn reference_root_for(engine_root: &Path) -> PathBuf {
    engine_root.parent().unwrap().join("reference")
}

fn write_executable_script(path: &Path, contents: &str) {
    fs::write(path, contents).unwrap();
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }
}

fn load_value_proposal(run_dir: &Path) -> Value {
    let current_value: Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("current_value.json")).unwrap())
            .unwrap();
    current_value["value"].clone()
}

fn load_primary_template(run_dir: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(run_dir.join("primary_template.json")).unwrap())
        .unwrap()
}

fn load_template_field_paths(run_dir: &Path) -> Vec<String> {
    let verifier_template: Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("verifier_template.json")).unwrap())
            .unwrap();
    verifier_template["field_verdicts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|field| field["field_path"].as_str().unwrap().to_string())
        .collect()
}

fn required_field_paths(value_proposal: &Value) -> Vec<String> {
    value_proposal["variants"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|variant| {
            let label = variant["label"].as_str().unwrap();
            vec![
                format!("variants[{label}].value.base_part_b_premium"),
                format!("variants[{label}].value.brackets"),
            ]
        })
        .collect()
}

fn write_primary_output(
    run_dir: &Path,
    run_id: &str,
    value_proposal: &Value,
    schema_change_required: bool,
) {
    let field_evidence = required_field_paths(value_proposal)
        .into_iter()
        .map(|field_path| {
            json!({
                "field_path": field_path,
                "source_id": "src_cms_1",
                "locator": "Table 2"
            })
        })
        .collect::<Vec<_>>();
    let primary_output = json!({
        "schema_version": 1,
        "run_id": run_id,
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
        },
        "sources": [
            {
                "source_id": "src_cms_1",
                "url": "https://www.cms.gov/newsroom/fact-sheets/example-irmaa-release",
                "host": "www.cms.gov",
                "organization": "CMS",
                "source_class": "primary",
                "title": "Example CMS IRMAA Release",
                "published_at": "2025-11-07",
                "locator": "Table 2",
                "notes": null
            }
        ],
        "proposed_status": "authoritative",
        "schema_change_required": schema_change_required,
        "schema_change_notes": if schema_change_required {
            json!(["Official source introduces a schema mismatch"])
        } else {
            json!([])
        },
        "value_proposal": value_proposal,
        "field_evidence": field_evidence,
        "unresolved_issues": if schema_change_required {
            json!(["Current lookup contract does not fit the official source cleanly"])
        } else {
            json!([])
        }
    });
    fs::write(
        run_dir.join("primary_output.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&primary_output).unwrap()
        ),
    )
    .unwrap();
}

fn write_verifier_output(
    run_dir: &Path,
    run_id: &str,
    value_proposal: &Value,
    dispute_first_field: bool,
    schema_change_required: bool,
) {
    let mut field_verdicts = required_field_paths(value_proposal)
        .into_iter()
        .map(|field_path| {
            json!({
                "field_path": field_path,
                "verdict": "confirm",
                "corrected_value": Value::Null,
                "source_ids": ["src_cms_1"],
                "notes": ""
            })
        })
        .collect::<Vec<_>>();
    if dispute_first_field {
        field_verdicts[0]["verdict"] = json!("dispute");
        field_verdicts[0]["notes"] = json!("Value does not match source");
    }

    let verifier_output = json!({
        "schema_version": 1,
        "run_id": run_id,
        "agent": {
            "tool": "claude_code",
            "model": "claude-opus-4-6"
        },
        "source_verdicts": [
            {
                "source_id": "src_cms_1",
                "verdict": "accept",
                "counts_toward_status": true,
                "reason": "Primary CMS source"
            }
        ],
        "field_verdicts": field_verdicts,
        "status_recommendation": if dispute_first_field { "needs_human_attention" } else { "authoritative" },
        "overall_verdict": if dispute_first_field { "needs_human_attention" } else { "pass" },
        "schema_change_required": schema_change_required,
        "schema_change_notes": if schema_change_required {
            json!(["Verifier believes the official source requires a contract update"])
        } else {
            json!([])
        },
        "notes": ""
    });
    fs::write(
        run_dir.join("verifier_output.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&verifier_output).unwrap()
        ),
    )
    .unwrap();
}

fn write_reports(run_dir: &Path, schema_change_required: bool) {
    let primary_report = if schema_change_required {
        "# Primary Extraction Report\n\n## Summary\n- schema change observed\n"
    } else {
        "# Primary Extraction Report\n\n## Summary\n- extracted current IRMAA structure from CMS source\n"
    };
    let verifier_report = if schema_change_required {
        "# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: true\n"
    } else {
        "# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\n"
    };
    fs::write(run_dir.join("primary_report.md"), primary_report).unwrap();
    fs::write(run_dir.join("verifier_report.md"), verifier_report).unwrap();
}

fn write_generic_primary_output(
    run_dir: &Path,
    run_id: &str,
    value_proposal: &Value,
    field_paths: &[String],
    source_id: &str,
    source_url: &str,
    source_host: &str,
    organization: &str,
    title: &str,
    locator: &str,
) {
    let field_evidence = field_paths
        .iter()
        .map(|field_path| {
            json!({
                "field_path": field_path,
                "source_id": source_id,
                "locator": locator
            })
        })
        .collect::<Vec<_>>();
    let primary_output = json!({
        "schema_version": 1,
        "run_id": run_id,
        "agent": {
            "tool": "claude_code",
            "model": "claude-opus-4-6"
        },
        "sources": [
            {
                "source_id": source_id,
                "url": source_url,
                "host": source_host,
                "organization": organization,
                "source_class": "primary",
                "title": title,
                "published_at": "2025-11-18",
                "locator": locator,
                "notes": null
            }
        ],
        "proposed_status": "authoritative",
        "schema_change_required": false,
        "schema_change_notes": [],
        "value_proposal": value_proposal,
        "field_evidence": field_evidence,
        "unresolved_issues": []
    });
    fs::write(
        run_dir.join("primary_output.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&primary_output).unwrap()
        ),
    )
    .unwrap();
}

fn write_generic_verifier_output(
    run_dir: &Path,
    run_id: &str,
    field_paths: &[String],
    source_id: &str,
) {
    let field_verdicts = field_paths
        .iter()
        .map(|field_path| {
            json!({
                "field_path": field_path,
                "verdict": "confirm",
                "corrected_value": Value::Null,
                "source_ids": [source_id],
                "notes": ""
            })
        })
        .collect::<Vec<_>>();
    let verifier_output = json!({
        "schema_version": 1,
        "run_id": run_id,
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
        },
        "source_verdicts": [
            {
                "source_id": source_id,
                "verdict": "accept",
                "counts_toward_status": true,
                "reason": "Primary official source"
            }
        ],
        "field_verdicts": field_verdicts,
        "status_recommendation": "authoritative",
        "overall_verdict": "pass",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": ""
    });
    fs::write(
        run_dir.join("verifier_output.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&verifier_output).unwrap()
        ),
    )
    .unwrap();
}

fn write_fake_primary_agent(path: &Path, value_proposal: &Value) {
    let value_proposal_json = serde_json::to_string_pretty(value_proposal).unwrap();
    let field_evidence_json = required_field_paths(value_proposal)
        .into_iter()
        .map(|field_path| {
            format!(
                "    {{\"field_path\": \"{field_path}\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}}"
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");
    let script = format!(
        "#!/bin/sh\nset -eu\ncat > \"$ENTROPYFA_PRIMARY_OUTPUT_PATH\" <<EOF\n{{\n  \"schema_version\": 1,\n  \"run_id\": \"$ENTROPYFA_RUN_ID\",\n  \"agent\": {{\n    \"tool\": \"claude_code\",\n    \"model\": \"claude-opus-4-6\"\n  }},\n  \"sources\": [\n    {{\n      \"source_id\": \"src_cms_1\",\n      \"url\": \"https://www.cms.gov/newsroom/fact-sheets/example-irmaa-release\",\n      \"host\": \"www.cms.gov\",\n      \"organization\": \"CMS\",\n      \"source_class\": \"primary\",\n      \"title\": \"Example CMS IRMAA Release\",\n      \"published_at\": \"2025-11-07\",\n      \"locator\": \"Table 2\",\n      \"notes\": null\n    }}\n  ],\n  \"proposed_status\": \"authoritative\",\n  \"schema_change_required\": false,\n  \"schema_change_notes\": [],\n  \"value_proposal\": {value_proposal_json},\n  \"field_evidence\": [\n{field_evidence_json}\n  ],\n  \"unresolved_issues\": []\n}}\nEOF\ncat > \"$ENTROPYFA_PRIMARY_REPORT_PATH\" <<'EOF'\n# Primary Extraction Report\n\n## Summary\n- extracted current IRMAA structure from CMS source\nEOF\necho primary-complete\n"
    );
    write_executable_script(path, &script);
}

fn write_fake_verifier_agent(path: &Path, value_proposal: &Value) {
    let field_verdicts_json = required_field_paths(value_proposal)
        .into_iter()
        .map(|field_path| {
            format!(
                "    {{\"field_path\": \"{field_path}\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"}}"
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");
    let script = format!(
        "#!/bin/sh\nset -eu\ncat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<EOF\n{{\n  \"schema_version\": 1,\n  \"run_id\": \"$ENTROPYFA_RUN_ID\",\n  \"agent\": {{\n    \"tool\": \"codex\",\n    \"model\": \"gpt-5.4\"\n  }},\n  \"source_verdicts\": [\n    {{\n      \"source_id\": \"src_cms_1\",\n      \"verdict\": \"accept\",\n      \"counts_toward_status\": true,\n      \"reason\": \"Primary CMS source\"\n    }}\n  ],\n  \"field_verdicts\": [\n{field_verdicts_json}\n  ],\n  \"status_recommendation\": \"authoritative\",\n  \"overall_verdict\": \"pass\",\n  \"schema_change_required\": false,\n  \"schema_change_notes\": [],\n  \"notes\": \"\"\n}}\nEOF\ncat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\nEOF\necho verifier-complete\n"
    );
    write_executable_script(path, &script);
}

fn write_delayed_fake_verifier_agent(path: &Path, value_proposal: &Value) {
    let field_verdicts_json = required_field_paths(value_proposal)
        .into_iter()
        .map(|field_path| {
            format!(
                "    {{\"field_path\": \"{field_path}\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"}}"
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");
    let script = format!(
        "#!/bin/sh\nset -eu\n(\n  sleep 1\n  cat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<EOF\n{{\n  \"schema_version\": 1,\n  \"run_id\": \"$ENTROPYFA_RUN_ID\",\n  \"agent\": {{\n    \"tool\": \"codex\",\n    \"model\": \"gpt-5.4\"\n  }},\n  \"source_verdicts\": [\n    {{\n      \"source_id\": \"src_cms_1\",\n      \"verdict\": \"accept\",\n      \"counts_toward_status\": true,\n      \"reason\": \"Primary CMS source\"\n    }}\n  ],\n  \"field_verdicts\": [\n{field_verdicts_json}\n  ],\n  \"status_recommendation\": \"authoritative\",\n  \"overall_verdict\": \"pass\",\n  \"schema_change_required\": false,\n  \"schema_change_notes\": [],\n  \"notes\": \"\"\n}}\nEOF\n  cat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\nEOF\n) &\necho verifier-delayed\n"
    );
    write_executable_script(path, &script);
}

#[test]
fn prepare_review_apply_irmaa_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["base_part_b_premium"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["brackets"][0]["magi_min"],
        json!("<number>")
    );
    assert!(prepared.run_dir.join("primary_report_template.md").exists());
    assert!(prepared
        .run_dir
        .join("verifier_report_template.md")
        .exists());

    write_primary_output(&prepared.run_dir, &prepared.run_id, &value_proposal, false);
    write_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        false,
        false,
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "insurance/irmaa_brackets review blocked: {:?}",
        review.blocking_issues
    );
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert!(review.blocking_issues.is_empty());

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    assert!(apply.reviewed_artifact_path.exists());
    assert!(apply.generated_source_path.exists());
    assert!(apply.reference_pack_path.exists());
    assert!(apply.reference_manifest_path.exists());

    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("pub fn base_part_b_premium() -> f64"));
    assert!(generated_source.contains("FilingStatus::Single"));

    let reference_pack = fs::read_to_string(&apply.reference_pack_path).unwrap();
    assert!(reference_pack.contains("category: insurance"));
    assert!(reference_pack.contains("key: irmaa_brackets"));
    assert!(reference_pack.contains("\"category\": \"insurance\""));
    assert!(reference_pack.contains("\"key\": \"irmaa_brackets\""));

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&apply.reference_manifest_path).unwrap()).unwrap();
    assert_eq!(manifest["pack_count"], json!(1));
    assert_eq!(manifest["categories"]["insurance"], json!(["2026"]));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let irmaa_entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "insurance" && entry.key == "irmaa_brackets")
        .unwrap();
    assert_eq!(
        irmaa_entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(irmaa_entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_medicare_base_premiums_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "medicare_base_premiums")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["label"],
        json!("default")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]
            ["part_b_standard_monthly_premium"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["part_b_annual_deductible"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]
            ["part_d_base_beneficiary_premium"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_cms_1",
        "https://www.cms.gov/newsroom/fact-sheets/2026-medicare-parts-b-premiums-deductibles",
        "www.cms.gov",
        "Centers for Medicare & Medicaid Services",
        "2026 Medicare Parts A & B Premiums and Deductibles",
        "Medicare Part B Premium and Deductible section",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_cms_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "insurance/medicare_base_premiums review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("pub struct MedicareBasePremiums"));
    assert!(generated_source.contains("pub fn base_premiums() -> MedicareBasePremiums"));
    assert!(generated_source.contains("part_b_standard_monthly_premium: 202.9"));
    assert!(generated_source.contains("part_d_base_beneficiary_premium: 38.99"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "insurance" && entry.key == "medicare_base_premiums")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_full_retirement_age_rules_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared = data_pipeline::prepare_run_at(
        &engine_root,
        2026,
        "social_security",
        "full_retirement_age_rules",
    )
    .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["label"],
        json!("default")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["benefit_scope"],
        json!("<string>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]
            ["january_1_births_use_prior_year"],
        json!("<boolean>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["rules"][0]["birth_year_min"],
        json!("<number_or_null>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["rules"][0]
            ["full_retirement_age_years"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_ssa_1",
        "https://www.ssa.gov/pubs/EN-05-10035.pdf",
        "www.ssa.gov",
        "Social Security Administration",
        "Retirement Benefits (2026)",
        "Pages 5-6, Full retirement age chart",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_ssa_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "social_security/full_retirement_age_rules review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("pub struct FullRetirementAgeRules"));
    assert!(
        generated_source.contains("pub fn full_retirement_age_rules() -> FullRetirementAgeRules")
    );
    assert!(generated_source.contains("benefit_scope: \"retirement_and_spousal\""));
    assert!(generated_source
        .contains("assert_eq!(full_retirement_age_for_birth_year(1959), Some((66, 10)))"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| {
            entry.category == "social_security" && entry.key == "full_retirement_age_rules"
        })
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_tax_brackets_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_income_tax_brackets")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["min"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["rate"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
        "www.irs.gov",
        "IRS",
        "Revenue Procedure 2025-32",
        "2026 federal income tax rate tables",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_income_tax_brackets review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(
        generated_source.contains("Federal income tax brackets (2025-2026, reviewed artifacts)")
    );
    assert!(generated_source.contains("pub fn brackets(status: FilingStatus) -> Vec<TaxBracket>"));
    assert!(generated_source.contains(
        "pub fn brackets_for_year(year: u32, status: FilingStatus) -> Result<Vec<TaxBracket>, DataError>"
    ));
    assert!(generated_source.contains("pub fn standard_deductions(status: FilingStatus) -> f64"));
    assert!(generated_source.contains("FilingStatus::Single"));
    assert!(generated_source.contains("2025 => Ok(brackets_2025(status))"));
    assert!(generated_source.contains("640600.0"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_income_tax_brackets")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_tax_brackets_2025_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2025, "tax", "federal_income_tax_brackets")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/rp-24-40.pdf",
        "www.irs.gov",
        "IRS",
        "Revenue Procedure 2024-40",
        "2025 federal income tax rate tables",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_income_tax_brackets 2025 review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("2025 => Ok(brackets_2025(status))"));
    assert!(generated_source.contains("626350.0"));
    assert!(generated_source.contains("751600.0"));
    assert!(apply
        .metadata_path
        .to_string_lossy()
        .contains("data_registry/2025/metadata.json"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    assert_eq!(registry.year, 2025);
    assert_eq!(registry.entries.len(), 1);
}

#[test]
fn prepare_review_apply_capital_gains_brackets_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_capital_gains_brackets")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["min"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["rate"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
        "www.irs.gov",
        "IRS",
        "Revenue Procedure 2025-32",
        "2026 long-term capital gains rate tables",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_capital_gains_brackets review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("Capital gains brackets (2026, reviewed artifact)"));
    assert!(generated_source
        .contains("pub fn capital_gains_brackets(status: FilingStatus) -> Vec<TaxBracket>"));
    assert!(generated_source.contains("613700.0"));
    assert!(generated_source.contains("pub fn niit(status: FilingStatus) -> NiitParams"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_capital_gains_brackets")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_standard_deductions_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_standard_deductions")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["amount"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["filing_status"],
        json!("single")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
        "www.irs.gov",
        "IRS",
        "Revenue Procedure 2025-32",
        "2026 standard deduction amounts",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_standard_deductions review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("Standard deductions (2026, reviewed artifact)"));
    assert!(generated_source.contains("pub fn standard_deductions(status: FilingStatus) -> f64"));
    assert!(generated_source.contains("FilingStatus::Single => 16100.0"));
    assert!(generated_source.contains("FilingStatus::HeadOfHousehold => 24150.0"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_standard_deductions")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn review_tolerates_missing_verifier_agent_metadata() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_standard_deductions")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
        "www.irs.gov",
        "IRS",
        "Revenue Procedure 2025-32",
        "2026 standard deduction amounts",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let verifier_output_path = prepared.run_dir.join("verifier_output.json");
    let mut verifier_output: Value =
        serde_json::from_str(&fs::read_to_string(&verifier_output_path).unwrap()).unwrap();
    verifier_output
        .as_object_mut()
        .unwrap()
        .remove("agent")
        .unwrap();
    fs::write(
        &verifier_output_path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&verifier_output).unwrap()
        ),
    )
    .unwrap();

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(review.approved);
    assert!(review.blocking_issues.is_empty());
    assert!(review
        .warnings
        .iter()
        .any(|warning| warning.contains("verifier_output.json did not include agent metadata")));
}

#[test]
fn review_tolerates_structured_primary_unresolved_issues() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_standard_deductions")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
        "www.irs.gov",
        "IRS",
        "Revenue Procedure 2025-32",
        "2026 standard deduction amounts",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let primary_output_path = prepared.run_dir.join("primary_output.json");
    let mut primary_output: Value =
        serde_json::from_str(&fs::read_to_string(&primary_output_path).unwrap()).unwrap();
    primary_output["unresolved_issues"] = json!([
        {
            "issue": "Example low-severity nuance",
            "severity": "low"
        }
    ]);
    fs::write(
        &primary_output_path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&primary_output).unwrap()
        ),
    )
    .unwrap();

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        false,
        Some("tester".into()),
    )
    .unwrap();
    assert!(review.blocking_issues.is_empty());
}

#[test]
fn prepare_review_apply_capital_loss_limit_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_capital_loss_limit")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/publications/p550",
        "www.irs.gov",
        "IRS",
        "Publication 550",
        "Capital loss deduction limit",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_capital_loss_limit review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("Capital loss limit (2026, reviewed artifact)"));
    assert!(generated_source.contains("pub fn capital_loss_limit(status: FilingStatus) -> f64"));
    assert!(generated_source.contains("FilingStatus::MarriedFilingSeparately => 1500.0"));
    assert!(generated_source.contains("FilingStatus::Single => 3000.0"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_capital_loss_limit")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_niit_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared = data_pipeline::prepare_run_at(
        &engine_root,
        2026,
        "tax",
        "federal_net_investment_income_tax",
    )
    .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/newsroom/questions-and-answers-on-the-net-investment-income-tax",
        "www.irs.gov",
        "IRS",
        "Questions and Answers on the Net Investment Income Tax",
        "Thresholds by filing status",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_net_investment_income_tax review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("Net Investment Income Tax (2026, reviewed artifact)"));
    assert!(generated_source.contains("pub fn niit(status: FilingStatus) -> NiitParams"));
    assert!(generated_source.contains("FilingStatus::Single => (0.038, 200000.0)"));
    assert!(generated_source.contains("FilingStatus::MarriedFilingSeparately => (0.038, 125000.0)"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_net_investment_income_tax")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_payroll_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_payroll_tax_parameters")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/taxtopics/tc751",
        "www.irs.gov",
        "IRS",
        "Topic no. 751, Social Security and Medicare withholding rates",
        "2026 rates and wage base",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_payroll_tax_parameters review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("Payroll tax parameters (2026, reviewed artifact)"));
    assert!(generated_source.contains("pub fn payroll(status: FilingStatus) -> PayrollParams"));
    assert!(generated_source.contains("social_security_wage_base: 184500.0"));
    assert!(generated_source.contains("additional_medicare_threshold: 125000.0"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_payroll_tax_parameters")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_social_security_taxation_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared = data_pipeline::prepare_run_at(
        &engine_root,
        2026,
        "social_security",
        "benefit_taxation_thresholds",
    )
    .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/publications/p915",
        "www.irs.gov",
        "IRS",
        "Publication 915",
        "Base amount and adjusted base amount thresholds",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "social_security/benefit_taxation_thresholds review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source
        .contains("Social Security benefit taxation thresholds (2026, reviewed artifact)."));
    assert!(generated_source.contains(
        "pub fn thresholds(\n    status: FilingStatus,\n    lived_with_spouse_during_year: Option<bool>,\n) -> Result<SsTaxationThresholds, DataError>"
    ));
    assert!(
        generated_source.contains("FilingStatus::MarriedFilingJointly => Ok(SsTaxationThresholds")
    );
    assert!(generated_source
        .contains("FilingStatus::MarriedFilingSeparately => match lived_with_spouse_during_year"));
    assert!(generated_source.contains("base_amount: 32000.0"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| {
            entry.category == "social_security" && entry.key == "benefit_taxation_thresholds"
        })
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_qbi_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_qbi_deduction").unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
        "www.irs.gov",
        "IRS",
        "Revenue Procedure 2025-32",
        "Section 199A threshold and phase-in amounts",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_qbi_deduction review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source
        .contains("QBI Deduction parameters (Section 199A, 2026, reviewed artifact)"));
    assert!(generated_source
        .contains("pub fn qbi_deduction(status: FilingStatus) -> QbiDeductionParams"));
    assert!(generated_source.contains("minimum_qbi_deduction: 400.0"));
    assert!(generated_source.contains("minimum_qbi_amount: 1000.0"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_qbi_deduction")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_salt_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared = data_pipeline::prepare_run_at(
        &engine_root,
        2026,
        "tax",
        "federal_salt_deduction_parameters",
    )
    .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/forms-pubs/correction-to-state-and-local-income-tax-deduction-amount-in-the-2026-form-1040-es",
        "www.irs.gov",
        "IRS",
        "Correction to State and Local Income Tax Deduction Amount in the 2026 Form 1040-ES",
        "Corrected 2026 state and local income tax deduction amounts by filing status",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_salt_deduction_parameters review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("SALT deduction parameters (2026, reviewed artifact)"));
    assert!(generated_source
        .contains("pub fn salt_deduction_parameters(status: FilingStatus) -> SaltDeductionParams"));
    assert!(generated_source.contains("cap_amount: 40400.0"));
    assert!(generated_source.contains("floor_amount: 5000.0"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_salt_deduction_parameters")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_distribution_rules_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "retirement", "distribution_rules")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["label"],
        json!("default")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["beneficiary_distribution"]
            ["ten_year_rule"]["terminal_year"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/publications/p590b",
        "www.irs.gov",
        "IRS",
        "Publication 590-B",
        "Required minimum distributions",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "retirement/distribution_rules review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("pub fn distribution_rules() -> RmdParameters"));
    assert!(generated_source.contains("still_working_exception_plan_categories: vec!["));
    assert!(generated_source.contains("still_working_exception_eligible_account_types: vec!["));
    assert!(generated_source.contains("guidance_status: Some(\"interim_good_faith\".to_string())"));
    assert!(generated_source.contains("designated_roth_owner_exemption_effective_year: Some(2024)"));
    assert!(generated_source.contains("relief_years: vec![2021, 2022, 2023, 2024]"));
    assert!(generated_source.contains("beneficiary_categories: vec!["));
    assert!(generated_source.contains("recognized_beneficiary_classes: vec!["));
    assert!(generated_source.contains("eligible_designated_beneficiary_classes: vec!["));
    assert!(generated_source
        .contains("non_designated_beneficiary_rules: NonDesignatedBeneficiaryRules"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "retirement" && entry.key == "distribution_rules")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_uniform_lifetime_table_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "retirement", "uniform_lifetime_table")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["label"],
        json!("default")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["age"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["distribution_period"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/publications/p590b",
        "www.irs.gov",
        "IRS",
        "Publication 590-B",
        "Table III, Uniform Lifetime",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "retirement/uniform_lifetime_table review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source
        .contains("Uniform Lifetime Table (Table III) — IRS Pub 590-B (2026, reviewed artifact)"));
    assert!(generated_source.contains("pub fn uniform_lifetime() -> Vec<AgeDistributionPeriod>"));
    assert!(generated_source.contains("AgeDistributionPeriod {"));
    assert!(generated_source.contains("age: 72"));
    assert!(generated_source.contains("distribution_period: 27.4"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "retirement" && entry.key == "uniform_lifetime_table")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_single_life_table_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "retirement", "single_life_table")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["label"],
        json!("default")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["age"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["distribution_period"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/publications/p590b",
        "www.irs.gov",
        "IRS",
        "Publication 590-B",
        "Table I, Single Life Expectancy",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "retirement/single_life_table review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains(
        "Single Life Expectancy Table (Table I) — IRS Pub 590-B (2026, reviewed artifact)"
    ));
    assert!(generated_source.contains("pub fn single_life() -> Vec<AgeDistributionPeriod>"));
    assert!(generated_source.contains("AgeDistributionPeriod {"));
    assert!(generated_source.contains("age: 0"));
    assert!(generated_source.contains("distribution_period: 84.6"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "retirement" && entry.key == "single_life_table")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_joint_life_table_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "retirement", "joint_life_table")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["label"],
        json!("default")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["owner_age"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["spouse_age"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["distribution_period"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/publications/p590b",
        "www.irs.gov",
        "IRS",
        "Publication 590-B",
        "Table II, Joint Life and Last Survivor",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "retirement/joint_life_table review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains(
        "Joint Life and Last Survivor Table (Table II) — IRS Pub 590-B (2026, reviewed artifact)"
    ));
    assert!(generated_source.contains("pub fn joint_life() -> Vec<JointDistributionPeriod>"));
    assert!(generated_source.contains("JointDistributionPeriod {"));
    assert!(generated_source.contains("owner_age: 80"));
    assert!(generated_source.contains("spouse_age: 70"));
    assert!(generated_source.contains("distribution_period: 18.4"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "retirement" && entry.key == "joint_life_table")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_mortality_417e_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "pension", "mortality_417e").unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["label"],
        json!("default")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["age"],
        json!("<number>")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"][0]["qx"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/n-24-76.pdf",
        "www.irs.gov",
        "IRS",
        "Notice 2024-76",
        "Applicable mortality table",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "pension/mortality_417e review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("pub fn table_417e() -> Vec<MortalityEntry>"));
    assert!(generated_source.contains("MortalityEntry {"));
    assert!(generated_source.contains("age: 50"));
    assert!(generated_source.contains("qx: 0.00281"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "pension" && entry.key == "mortality_417e")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn verifier_prompt_explains_primary_vs_current_value_for_updates() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_payroll_tax_parameters")
            .unwrap();

    let verifier_prompt = fs::read_to_string(prepared.run_dir.join("verifier_prompt.md")).unwrap();
    assert!(verifier_prompt.contains(
        "Use `field_verdicts[]` to judge whether `primary_output.json` is supported by the cited or replacement sources"
    ));
    assert!(verifier_prompt.contains(
        "Do not use `dispute` merely because `current_value.json` differs from `primary_output.json`."
    ));
    assert!(verifier_prompt.contains(
        "If official sources support the primary proposal and the current embedded value is stale, use `confirm`"
    ));
}

#[test]
fn prepare_review_apply_estate_exemption_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_estate_exemption")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let primary_template = load_primary_template(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["label"],
        json!("default")
    );
    assert_eq!(
        primary_template["value_proposal"]["variants"][0]["value"]["exemption"],
        json!("<number>")
    );

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
        "www.irs.gov",
        "IRS",
        "Revenue Procedure 2025-32",
        "2026 estate and gift tax basic exclusion amount",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_estate_exemption review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source
        .contains("Basic exclusion amount (exemption) for 2026, reviewed artifact."));
    assert!(generated_source.contains("pub fn exemption() -> f64"));
    assert!(generated_source.contains("15_000_000.0") || generated_source.contains("15000000.0"));
    assert!(generated_source.contains("assert_eq!(exemption(), 15000000.0);"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_estate_exemption")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_estate_applicable_credit_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared = data_pipeline::prepare_run_at(
        &engine_root,
        2026,
        "tax",
        "federal_estate_applicable_credit",
    )
    .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/pub/irs-drop/rp-25-32.pdf",
        "www.irs.gov",
        "IRS",
        "Revenue Procedure 2025-32",
        "2026 estate tax corresponding credit amount",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_estate_applicable_credit review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("Applicable credit amount for 2026, reviewed artifact."));
    assert!(generated_source.contains("pub fn applicable_credit() -> f64"));
    assert!(generated_source.contains("5_945_800.0") || generated_source.contains("5945800.0"));
    assert!(generated_source.contains("assert_eq!(applicable_credit(), 5945800.0);"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_estate_applicable_credit")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn prepare_review_apply_estate_brackets_happy_path() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "tax", "federal_estate_brackets")
            .unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);
    let field_paths = load_template_field_paths(&prepared.run_dir);

    write_generic_primary_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        &field_paths,
        "src_irs_1",
        "https://www.irs.gov/instructions/i706",
        "www.irs.gov",
        "IRS",
        "Instructions for Form 706",
        "Part 2 - Tax Computation table",
    );
    write_generic_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &field_paths,
        "src_irs_1",
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(
        review.approved,
        "tax/federal_estate_brackets review blocked: {:?}",
        review.blocking_issues
    );
    assert!(review.blocking_issues.is_empty());
    assert_eq!(
        review.status_decision,
        data_pipeline::VerificationStatus::Authoritative
    );

    let apply = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("pub fn brackets() -> Vec<TaxBracket>"));
    assert!(generated_source.contains("rate: 0.18"));
    assert!(generated_source.contains("rate: 0.4") || generated_source.contains("rate: 0.40"));
    assert!(generated_source.contains("assert_eq!(b.len(), 12);"));

    let registry = data_pipeline::load_registry(&apply.metadata_path).unwrap();
    let entry = registry
        .entries
        .iter()
        .find(|entry| entry.category == "tax" && entry.key == "federal_estate_brackets")
        .unwrap();
    assert_eq!(
        entry.verification_status,
        data_pipeline::VerificationStatus::Authoritative
    );
    assert_eq!(entry.completeness, data_pipeline::Completeness::Full);
}

#[test]
fn review_run_blocks_verifier_dispute() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);

    write_primary_output(&prepared.run_dir, &prepared.run_id, &value_proposal, false);
    write_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        true,
        false,
    );
    write_reports(&prepared.run_dir, false);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();

    assert!(!review.approved);
    assert!(!review.blocking_issues.is_empty());
    assert!(review
        .blocking_issues
        .iter()
        .any(|issue| issue.contains("verifier marked")));
}

#[test]
fn review_run_blocks_schema_change_required() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);

    write_primary_output(&prepared.run_dir, &prepared.run_id, &value_proposal, true);
    write_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        false,
        true,
    );
    write_reports(&prepared.run_dir, true);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();

    assert!(!review.approved);
    assert!(review
        .blocking_issues
        .iter()
        .any(|issue| issue.contains("schema_change_required")));
}

#[test]
fn review_run_suggests_contract_changes_for_irmaa_schema_gaps() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);

    write_primary_output(&prepared.run_dir, &prepared.run_id, &value_proposal, false);

    let mut verifier_output: Value = serde_json::from_str(
        &fs::read_to_string(prepared.run_dir.join("verifier_template.json")).unwrap(),
    )
    .unwrap();
    verifier_output["schema_version"] = json!(1);
    verifier_output["run_id"] = json!(prepared.run_id.clone());
    verifier_output["agent"] = json!({
        "tool": "codex",
        "model": "gpt-5.4"
    });
    verifier_output["source_verdicts"] = json!([
        {
            "source_id": "src_cms_1",
            "verdict": "accept",
            "counts_toward_status": true,
            "reason": "Primary CMS source"
        }
    ]);
    let field_verdicts = required_field_paths(&value_proposal)
        .into_iter()
        .map(|field_path| {
            let (verdict, notes) = if field_path
                == "variants[married_filing_separately_lived_with_spouse].value.brackets"
            {
                (
                    "uncertain",
                    "The official rule depends on whether the taxpayer lived with spouse during the year, which the prior married filing separately contract could not represent cleanly.",
                )
            } else if field_path.ends_with(".value.brackets")
                && (field_path.contains("single") || field_path.contains("married_filing_jointly"))
            {
                (
                    "confirm",
                    "The numeric thresholds match CMS but the current schema does not encode exact boundary semantics.",
                )
            } else {
                ("confirm", "")
            };
            json!({
                "field_path": field_path,
                "verdict": verdict,
                "corrected_value": Value::Null,
                "source_ids": ["src_cms_1"],
                "notes": notes,
            })
        })
        .collect::<Vec<_>>();
    verifier_output["field_verdicts"] = Value::Array(field_verdicts);
    verifier_output["status_recommendation"] = json!("needs_human_attention");
    verifier_output["overall_verdict"] = json!("needs_human_attention");
    verifier_output["schema_change_required"] = json!(true);
    verifier_output["schema_change_notes"] = json!([
        "The official married-filing-separately rule is conditional on whether the taxpayer lived with spouse during the year.",
        "The current schema stores magi_min and magi_max but does not encode exact <=, >, <, and >= boundary semantics."
    ]);
    verifier_output["notes"] = json!("Human review is required because the contract does not fully represent the official CMS/SSA rule.");
    fs::write(
        prepared.run_dir.join("verifier_output.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&verifier_output).unwrap()
        ),
    )
    .unwrap();
    write_reports(&prepared.run_dir, true);

    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();

    assert!(!review.approved);

    let review_json: Value =
        serde_json::from_str(&fs::read_to_string(prepared.run_dir.join("review.json")).unwrap())
            .unwrap();
    assert_eq!(
        review_json["recommended_action"],
        json!("update_contract_then_rerun_pipeline")
    );
    let suggested = review_json["suggested_contract_changes"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>();
    assert!(suggested
        .iter()
        .any(|item| item.contains("lived_with_spouse_during_year")));
    assert!(suggested
        .iter()
        .any(|item| item.contains("magi_min") || item.contains("inclusive")));
}

#[test]
fn run_agents_prepares_executes_and_reviews_without_approval() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");
    write_fake_primary_agent(&primary_bin, &value_proposal);
    write_fake_verifier_agent(&verifier_bin, &value_proposal);

    let outcome = data_pipeline::run_agents_at(
        &engine_root,
        &data_pipeline::RunAgentsConfig {
            year: 2026,
            category: "insurance".into(),
            key: "irmaa_brackets".into(),
            primary: data_pipeline::AgentInvocationConfig {
                provider: data_pipeline::AgentProvider::Claude,
                model: "claude-opus-4-6".into(),
                binary: Some(primary_bin.clone()),
            },
            verifier: data_pipeline::AgentInvocationConfig {
                provider: data_pipeline::AgentProvider::Codex,
                model: "gpt-5.4".into(),
                binary: Some(verifier_bin.clone()),
            },
        },
    )
    .unwrap();

    assert!(outcome
        .prepared
        .run_dir
        .join("primary_output.json")
        .exists());
    assert!(outcome.prepared.run_dir.join("primary_report.md").exists());
    assert!(outcome
        .prepared
        .run_dir
        .join("verifier_output.json")
        .exists());
    assert!(outcome.prepared.run_dir.join("verifier_report.md").exists());
    assert!(outcome.primary.stdout_log_path.exists());
    assert!(outcome.verifier.stdout_log_path.exists());
    assert!(!outcome.review.approved);
    assert!(outcome.review.blocking_issues.is_empty());

    let stdout = fs::read_to_string(&outcome.primary.stdout_log_path).unwrap();
    assert!(stdout.contains("primary-complete"));
}

#[test]
fn run_agents_waits_for_delayed_verifier_outputs() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");
    write_fake_primary_agent(&primary_bin, &value_proposal);
    write_delayed_fake_verifier_agent(&verifier_bin, &value_proposal);

    let outcome = data_pipeline::run_agents_at(
        &engine_root,
        &data_pipeline::RunAgentsConfig {
            year: 2026,
            category: "insurance".into(),
            key: "irmaa_brackets".into(),
            primary: data_pipeline::AgentInvocationConfig {
                provider: data_pipeline::AgentProvider::Claude,
                model: "claude-opus-4-6".into(),
                binary: Some(primary_bin),
            },
            verifier: data_pipeline::AgentInvocationConfig {
                provider: data_pipeline::AgentProvider::Codex,
                model: "gpt-5.4".into(),
                binary: Some(verifier_bin),
            },
        },
    )
    .unwrap();

    assert!(outcome
        .prepared
        .run_dir
        .join("verifier_output.json")
        .exists());
    assert!(outcome.prepared.run_dir.join("verifier_report.md").exists());
    assert!(outcome.review.review_path.exists());
}

#[test]
fn status_report_summarizes_registry_and_pipeline_state() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    copy_file(
        &actual_engine_root()
            .join("data_registry/2026/reviewed/retirement/distribution_rules.json"),
        &engine_root.join("data_registry/2026/reviewed/retirement/distribution_rules.json"),
    );
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);

    write_primary_output(&prepared.run_dir, &prepared.run_id, &value_proposal, false);
    write_verifier_output(
        &prepared.run_dir,
        &prepared.run_id,
        &value_proposal,
        false,
        false,
    );
    write_reports(&prepared.run_dir, false);
    data_pipeline::review_run_with_approval_at(&engine_root, &prepared.run_id, true, None).unwrap();
    data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();

    let report = data_pipeline::status_report_at(&engine_root, 2026).unwrap();
    assert_eq!(report.registry_entries, 20);
    assert_eq!(report.pipeline_definitions, 20);
    assert_eq!(report.reviewed_artifacts, 2);
    assert_eq!(report.reference_packs, 1);
    assert_eq!(report.legacy_only_entries, 1);

    let irmaa = report
        .entries
        .iter()
        .find(|entry| entry.category == "insurance" && entry.key == "irmaa_brackets")
        .unwrap();
    assert!(irmaa.pipeline_defined);
    assert!(irmaa.reviewed_artifact_exists);
    assert!(irmaa.reference_pack_exists);
    assert_eq!(
        irmaa.latest_run.as_ref().unwrap().status.to_string(),
        "applied"
    );

    let distribution_rules = report
        .entries
        .iter()
        .find(|entry| entry.category == "retirement" && entry.key == "distribution_rules")
        .unwrap();
    assert!(distribution_rules.pipeline_defined);
    assert!(distribution_rules.reviewed_artifact_exists);
    assert!(!distribution_rules.reference_pack_exists);
    assert!(distribution_rules.latest_run.is_none());

    let expected_pack_path =
        reference_root_for(&engine_root).join("insurance/2026/irmaa_brackets.md");
    assert_eq!(&irmaa.reference_pack_path, &expected_pack_path);
}
