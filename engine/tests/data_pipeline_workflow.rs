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

    fs::create_dir_all(engine_root.join("data_registry/2026")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/insurance")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/tax")).unwrap();
    fs::create_dir_all(engine_root.join("src/data/insurance")).unwrap();
    fs::create_dir_all(engine_root.join("src/data/tax")).unwrap();

    copy_file(
        &actual_engine_root().join("data_registry/2026/metadata.json"),
        &engine_root.join("data_registry/2026/metadata.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/insurance/irmaa_brackets.json"),
        &engine_root.join("data_registry/pipelines/insurance/irmaa_brackets.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/tax/federal_income_tax_brackets.json"),
        &engine_root.join("data_registry/pipelines/tax/federal_income_tax_brackets.json"),
    );
    copy_file(
        &actual_engine_root().join("src/data/tax/federal.rs"),
        &engine_root.join("src/data/tax/federal.rs"),
    );

    (temp_dir, engine_root)
}

fn copy_file(source: &Path, target: &Path) {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::copy(source, target).unwrap();
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
    let script = format!(
        "#!/bin/sh\nset -eu\ncat > \"$ENTROPYFA_PRIMARY_OUTPUT_PATH\" <<EOF\n{{\n  \"schema_version\": 1,\n  \"run_id\": \"$ENTROPYFA_RUN_ID\",\n  \"agent\": {{\n    \"tool\": \"claude_code\",\n    \"model\": \"claude-opus-4-6\"\n  }},\n  \"sources\": [\n    {{\n      \"source_id\": \"src_cms_1\",\n      \"url\": \"https://www.cms.gov/newsroom/fact-sheets/example-irmaa-release\",\n      \"host\": \"www.cms.gov\",\n      \"organization\": \"CMS\",\n      \"source_class\": \"primary\",\n      \"title\": \"Example CMS IRMAA Release\",\n      \"published_at\": \"2025-11-07\",\n      \"locator\": \"Table 2\",\n      \"notes\": null\n    }}\n  ],\n  \"proposed_status\": \"authoritative\",\n  \"schema_change_required\": false,\n  \"schema_change_notes\": [],\n  \"value_proposal\": {value_proposal_json},\n  \"field_evidence\": [\n    {{\"field_path\": \"variants[single].value.base_part_b_premium\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}},\n    {{\"field_path\": \"variants[single].value.brackets\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}},\n    {{\"field_path\": \"variants[married_filing_jointly].value.base_part_b_premium\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}},\n    {{\"field_path\": \"variants[married_filing_jointly].value.brackets\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}},\n    {{\"field_path\": \"variants[married_filing_separately].value.base_part_b_premium\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}},\n    {{\"field_path\": \"variants[married_filing_separately].value.brackets\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}},\n    {{\"field_path\": \"variants[head_of_household].value.base_part_b_premium\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}},\n    {{\"field_path\": \"variants[head_of_household].value.brackets\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}},\n    {{\"field_path\": \"variants[qualifying_surviving_spouse].value.base_part_b_premium\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}},\n    {{\"field_path\": \"variants[qualifying_surviving_spouse].value.brackets\", \"source_id\": \"src_cms_1\", \"locator\": \"Table 2\"}}\n  ],\n  \"unresolved_issues\": []\n}}\nEOF\ncat > \"$ENTROPYFA_PRIMARY_REPORT_PATH\" <<'EOF'\n# Primary Extraction Report\n\n## Summary\n- extracted current IRMAA structure from CMS source\nEOF\necho primary-complete\n"
    );
    write_executable_script(path, &script);
}

fn write_fake_verifier_agent(path: &Path) {
    let script = "#!/bin/sh\nset -eu\ncat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<EOF\n{\n  \"schema_version\": 1,\n  \"run_id\": \"$ENTROPYFA_RUN_ID\",\n  \"agent\": {\n    \"tool\": \"codex\",\n    \"model\": \"gpt-5.4\"\n  },\n  \"source_verdicts\": [\n    {\n      \"source_id\": \"src_cms_1\",\n      \"verdict\": \"accept\",\n      \"counts_toward_status\": true,\n      \"reason\": \"Primary CMS source\"\n    }\n  ],\n  \"field_verdicts\": [\n    {\"field_path\": \"variants[single].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[single].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[married_filing_jointly].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[married_filing_jointly].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[married_filing_separately].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[married_filing_separately].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[head_of_household].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[head_of_household].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[qualifying_surviving_spouse].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[qualifying_surviving_spouse].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"}\n  ],\n  \"status_recommendation\": \"authoritative\",\n  \"overall_verdict\": \"pass\",\n  \"schema_change_required\": false,\n  \"schema_change_notes\": [],\n  \"notes\": \"\"\n}\nEOF\ncat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\nEOF\necho verifier-complete\n";
    write_executable_script(path, script);
}

fn write_delayed_fake_verifier_agent(path: &Path) {
    let script = "#!/bin/sh\nset -eu\n(\n  sleep 1\n  cat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<EOF\n{\n  \"schema_version\": 1,\n  \"run_id\": \"$ENTROPYFA_RUN_ID\",\n  \"agent\": {\n    \"tool\": \"codex\",\n    \"model\": \"gpt-5.4\"\n  },\n  \"source_verdicts\": [\n    {\n      \"source_id\": \"src_cms_1\",\n      \"verdict\": \"accept\",\n      \"counts_toward_status\": true,\n      \"reason\": \"Primary CMS source\"\n    }\n  ],\n  \"field_verdicts\": [\n    {\"field_path\": \"variants[single].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[single].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[married_filing_jointly].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[married_filing_jointly].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[married_filing_separately].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[married_filing_separately].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[head_of_household].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[head_of_household].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[qualifying_surviving_spouse].value.base_part_b_premium\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"},\n    {\"field_path\": \"variants[qualifying_surviving_spouse].value.brackets\", \"verdict\": \"confirm\", \"corrected_value\": null, \"source_ids\": [\"src_cms_1\"], \"notes\": \"\"}\n  ],\n  \"status_recommendation\": \"authoritative\",\n  \"overall_verdict\": \"pass\",\n  \"schema_change_required\": false,\n  \"schema_change_notes\": [],\n  \"notes\": \"\"\n}\nEOF\n  cat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\nEOF\n) &\necho verifier-delayed\n";
    write_executable_script(path, script);
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

    let generated_source = fs::read_to_string(&apply.generated_source_path).unwrap();
    assert!(generated_source.contains("pub fn base_part_b_premium() -> f64"));
    assert!(generated_source.contains("FilingStatus::Single"));

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
    assert!(generated_source.contains("Federal income tax brackets (2026, reviewed artifact)"));
    assert!(generated_source.contains("pub fn brackets(status: FilingStatus) -> Vec<TaxBracket>"));
    assert!(generated_source.contains("pub fn standard_deductions(status: FilingStatus) -> f64"));
    assert!(generated_source.contains("FilingStatus::Single"));
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
fn run_agents_prepares_executes_and_reviews_without_approval() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");
    write_fake_primary_agent(&primary_bin, &value_proposal);
    write_fake_verifier_agent(&verifier_bin);

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
    write_delayed_fake_verifier_agent(&verifier_bin);

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
    assert_eq!(report.registry_entries, 17);
    assert_eq!(report.pipeline_definitions, 2);

    let irmaa = report
        .entries
        .iter()
        .find(|entry| entry.category == "insurance" && entry.key == "irmaa_brackets")
        .unwrap();
    assert!(irmaa.pipeline_defined);
    assert!(irmaa.reviewed_artifact_exists);
    assert_eq!(
        irmaa.latest_run.as_ref().unwrap().status.to_string(),
        "applied"
    );

    let distribution_rules = report
        .entries
        .iter()
        .find(|entry| entry.category == "retirement" && entry.key == "distribution_rules")
        .unwrap();
    assert!(!distribution_rules.pipeline_defined);
    assert!(distribution_rules.latest_run.is_none());
}
