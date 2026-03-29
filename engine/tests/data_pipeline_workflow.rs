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
    fs::create_dir_all(engine_root.join("data_registry/pipelines/gifting")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/insurance")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/pension")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/retirement")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/social_security")).unwrap();
    fs::create_dir_all(engine_root.join("data_registry/pipelines/tax")).unwrap();
    fs::create_dir_all(engine_root.join("src/data/gifting")).unwrap();
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
        &actual_engine_root().join(
            "data_registry/pipelines/social_security/retirement_earnings_test_thresholds.json",
        ),
        &engine_root.join(
            "data_registry/pipelines/social_security/retirement_earnings_test_thresholds.json",
        ),
    );
    for prefix in &["afr", "section_7520"] {
        for month in &["01", "02", "03"] {
            let name = format!("{prefix}_2026_{month}.json");
            copy_file(
                &actual_engine_root().join(format!("data_registry/pipelines/rates/{name}")),
                &engine_root.join(format!("data_registry/pipelines/rates/{name}")),
            );
            copy_file(
                &actual_engine_root().join(format!("data_registry/2026/reviewed/rates/{name}")),
                &engine_root.join(format!("data_registry/2026/reviewed/rates/{name}")),
            );
        }
    }
    copy_file(
        &actual_engine_root().join("src/data/rates/afr.rs"),
        &engine_root.join("src/data/rates/afr.rs"),
    );
    copy_file(
        &actual_engine_root().join("src/data/rates/section_7520.rs"),
        &engine_root.join("src/data/rates/section_7520.rs"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/tax/hsa_contribution_limits.json"),
        &engine_root.join("data_registry/pipelines/tax/hsa_contribution_limits.json"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/2026/reviewed/tax/hsa_contribution_limits.json"),
        &engine_root.join("data_registry/2026/reviewed/tax/hsa_contribution_limits.json"),
    );
    copy_file(
        &actual_engine_root().join("src/data/tax/hsa.rs"),
        &engine_root.join("src/data/tax/hsa.rs"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/retirement/contribution_limits.json"),
        &engine_root.join("data_registry/pipelines/retirement/contribution_limits.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/2026/reviewed/retirement/contribution_limits.json"),
        &engine_root.join("data_registry/2026/reviewed/retirement/contribution_limits.json"),
    );
    copy_file(
        &actual_engine_root().join("src/data/retirement/contribution_limits.rs"),
        &engine_root.join("src/data/retirement/contribution_limits.rs"),
    );
    copy_file(
        &actual_engine_root().join("data_registry/pipelines/gifting/federal_annual_exclusion.json"),
        &engine_root.join("data_registry/pipelines/gifting/federal_annual_exclusion.json"),
    );
    copy_file(
        &actual_engine_root()
            .join("data_registry/2026/reviewed/gifting/federal_annual_exclusion.json"),
        &engine_root.join("data_registry/2026/reviewed/gifting/federal_annual_exclusion.json"),
    );
    copy_file(
        &actual_engine_root().join("src/data/gifting/annual_exclusion.rs"),
        &engine_root.join("src/data/gifting/annual_exclusion.rs"),
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
        &actual_engine_root().join("src/data/social_security/earnings_test.rs"),
        &engine_root.join("src/data/social_security/earnings_test.rs"),
    );
    copy_file(
        &actual_engine_root().join(
            "data_registry/2026/reviewed/social_security/retirement_earnings_test_thresholds.json",
        ),
        &engine_root.join(
            "data_registry/2026/reviewed/social_security/retirement_earnings_test_thresholds.json",
        ),
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

fn sample_reference_pack_primer() -> Value {
    json!({
        "what_this_is": "Annual federal IRMAA surcharge thresholds and the shared base Part B premium for the applicable filing-status variants.",
        "lookup_parameters": [
            "filing_status",
            "lived_with_spouse_during_year when married filing separately"
        ],
        "interpretation_notes": [
            "Each variant maps to one filing-status lookup path.",
            "Bracket minimums are inclusive and the final bracket is open ended."
        ],
        "does_not_include": [
            "Medicare Part D base premium changes outside the published IRMAA surcharge table",
            "Tax filing advice"
        ],
        "caveats": [
            "The published table must still be read alongside the married-filing-separately spouse-living rule."
        ],
        "typical_uses": [
            "Estimating Medicare premium surcharges in retirement projections"
        ]
    })
}

fn sample_primer_verdicts() -> Value {
    json!({
        "what_this_is": {
            "verdict": "confirm",
            "notes": ""
        },
        "lookup_parameters": {
            "verdict": "confirm",
            "notes": ""
        },
        "interpretation_notes": {
            "verdict": "confirm",
            "notes": ""
        },
        "does_not_include": {
            "verdict": "confirm",
            "notes": ""
        },
        "caveats": {
            "verdict": "confirm",
            "notes": ""
        },
        "typical_uses": {
            "verdict": "confirm",
            "notes": ""
        }
    })
}

fn primer_section_verdict_with_metadata(
    verdict: &str,
    notes: &str,
    issue_type: &str,
    auto_resolvable: bool,
    repair_guidance: &str,
) -> Value {
    json!({
        "verdict": verdict,
        "notes": notes,
        "issue_type": issue_type,
        "auto_resolvable": auto_resolvable,
        "repair_guidance": repair_guidance,
    })
}

fn field_verdict_with_metadata(
    field_path: &str,
    verdict: &str,
    notes: &str,
    issue_type: &str,
    auto_resolvable: bool,
    repair_guidance: &str,
) -> Value {
    json!({
        "field_path": field_path,
        "verdict": verdict,
        "corrected_value": Value::Null,
        "source_ids": ["src_cms_1"],
        "notes": notes,
        "issue_type": issue_type,
        "auto_resolvable": auto_resolvable,
        "repair_guidance": repair_guidance,
    })
}

fn overbroad_reference_pack_primer() -> Value {
    json!({
        "what_this_is": "Annual federal IRMAA surcharge thresholds, the shared base Part B premium, and broad retirement/tax planning notes for the applicable filing-status variants.",
        "lookup_parameters": [
            "filing_status",
            "lived_with_spouse_during_year when married filing separately",
            "taxable income",
        ],
        "interpretation_notes": [
            "Each variant maps to one filing-status lookup path.",
            "Bracket minimums are inclusive and the final bracket is open ended.",
            "Use this as a general retirement planning reference."
        ],
        "does_not_include": [
            "Medicare Part D base premium changes outside the published IRMAA surcharge table",
            "Tax filing advice",
            "Other retirement planning topics"
        ],
        "caveats": [
            "The published table must still be read alongside the married-filing-separately spouse-living rule.",
            "This primer is intentionally broader than the final consumer reference pack."
        ],
        "typical_uses": [
            "Estimating Medicare premium surcharges in retirement projections",
            "Broad household tax planning"
        ]
    })
}

#[derive(Clone, Copy)]
struct PrimerSectionVerdictSpec {
    section: &'static str,
    verdict: &'static str,
    notes: &'static str,
    issue_type: &'static str,
    auto_resolvable: bool,
    repair_guidance: &'static str,
}

impl PrimerSectionVerdictSpec {
    const fn new(
        section: &'static str,
        verdict: &'static str,
        notes: &'static str,
        issue_type: &'static str,
        auto_resolvable: bool,
        repair_guidance: &'static str,
    ) -> Self {
        Self {
            section,
            verdict,
            notes,
            issue_type,
            auto_resolvable,
            repair_guidance,
        }
    }
}

fn primer_section_verdict_from_spec(spec: PrimerSectionVerdictSpec) -> Value {
    primer_section_verdict_with_metadata(
        spec.verdict,
        spec.notes,
        spec.issue_type,
        spec.auto_resolvable,
        spec.repair_guidance,
    )
}

fn primer_verdicts_from_specs(specs: &[PrimerSectionVerdictSpec]) -> Value {
    let mut verdicts = serde_json::Map::new();
    for spec in specs {
        verdicts.insert(
            spec.section.to_string(),
            primer_section_verdict_from_spec(*spec),
        );
    }
    Value::Object(verdicts)
}

fn safe_primer_dispute_specs() -> Vec<PrimerSectionVerdictSpec> {
    vec![
        PrimerSectionVerdictSpec::new(
            "what_this_is",
            "confirm",
            "",
            "primer_scope_only",
            true,
            "Keep the Medicare-specific scope and remove broader planning language.",
        ),
        PrimerSectionVerdictSpec::new(
            "lookup_parameters",
            "confirm",
            "",
            "primer_scope_only",
            true,
            "Keep only filing-status lookup guidance.",
        ),
        PrimerSectionVerdictSpec::new(
            "interpretation_notes",
            "dispute",
            "Contains broad tax-planning language",
            "overbroad_primer",
            true,
            "Rewrite this section so it only explains the CMS IRMAA lookup.",
        ),
        PrimerSectionVerdictSpec::new(
            "does_not_include",
            "dispute",
            "Still mentions unrelated tax advice",
            "overbroad_primer",
            true,
            "Remove tax filing advice and unrelated planning guidance.",
        ),
        PrimerSectionVerdictSpec::new(
            "caveats",
            "dispute",
            "Overstates the reference pack scope",
            "overbroad_primer",
            true,
            "Limit caveats to source handling only.",
        ),
        PrimerSectionVerdictSpec::new(
            "typical_uses",
            "dispute",
            "Includes broad planning uses",
            "overbroad_primer",
            true,
            "Keep only Medicare premium estimation use cases.",
        ),
    ]
}

fn manual_required_value_dispute_primer_specs() -> Vec<PrimerSectionVerdictSpec> {
    vec![
        PrimerSectionVerdictSpec::new(
            "what_this_is",
            "confirm",
            "",
            "value_dispute_case",
            false,
            "Primer content is not the source of the blocker.",
        ),
        PrimerSectionVerdictSpec::new(
            "lookup_parameters",
            "confirm",
            "",
            "value_dispute_case",
            false,
            "Primer content is not the source of the blocker.",
        ),
        PrimerSectionVerdictSpec::new(
            "interpretation_notes",
            "confirm",
            "",
            "value_dispute_case",
            false,
            "Primer content is not the source of the blocker.",
        ),
        PrimerSectionVerdictSpec::new(
            "does_not_include",
            "confirm",
            "",
            "value_dispute_case",
            false,
            "Primer content is not the source of the blocker.",
        ),
        PrimerSectionVerdictSpec::new(
            "caveats",
            "confirm",
            "",
            "value_dispute_case",
            false,
            "Primer content is not the source of the blocker.",
        ),
        PrimerSectionVerdictSpec::new(
            "typical_uses",
            "confirm",
            "",
            "value_dispute_case",
            false,
            "Primer content is not the source of the blocker.",
        ),
    ]
}

fn write_fake_verifier_agent_with_payload(
    path: &Path,
    value_proposal: &Value,
    field_verdicts: Vec<Value>,
    primer_verdicts: Value,
    status_recommendation: &str,
    overall_verdict: &str,
    notes: &str,
    report_summary: &str,
) {
    let verifier_output = json!({
        "schema_version": 1,
        "run_id": "$ENTROPYFA_RUN_ID",
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
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
        "primer_verdicts": primer_verdicts,
        "status_recommendation": status_recommendation,
        "overall_verdict": overall_verdict,
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": notes,
        "value_proposal": value_proposal,
    });
    let verifier_output_json = serde_json::to_string_pretty(&verifier_output).unwrap();
    let script = format!(
        "#!/bin/sh\nset -eu\ncat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<'EOF'\n{verifier_output_json}\nEOF\nperl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\"\ncat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- {report_summary}\nEOF\necho verifier-complete\n"
    );
    write_executable_script(path, &script);
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
        "reference_pack_primer": sample_reference_pack_primer(),
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
        "primer_verdicts": sample_primer_verdicts(),
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
        "reference_pack_primer": sample_reference_pack_primer(),
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
        "primer_verdicts": sample_primer_verdicts(),
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
    write_fake_primary_agent_with_repair_mode(
        path,
        value_proposal,
        sample_reference_pack_primer(),
        None,
        None,
    );
}

fn write_fake_primary_agent_with_repair_mode(
    path: &Path,
    value_proposal: &Value,
    reference_pack_primer: Value,
    repair_output: Option<Value>,
    repair_report_summary: Option<&str>,
) {
    let primary_output =
        fake_primary_output_value("$ENTROPYFA_RUN_ID", value_proposal, reference_pack_primer);
    let primary_output_json = serde_json::to_string_pretty(&primary_output).unwrap();
    let repair_output_json = repair_output
        .as_ref()
        .map(|output| serde_json::to_string_pretty(output).unwrap())
        .unwrap_or_else(|| primary_output_json.clone());
    let repair_report_summary = repair_report_summary
        .map(|summary| summary.to_string())
        .unwrap_or_else(|| "Preserved the reviewed value proposal.".to_string());
    let script = format!(
        "#!/bin/sh\nset -eu\nif [ -n \"${{ENTROPYFA_REPAIR_OUTPUT_PATH:-}}\" ] || [ -n \"${{ENTROPYFA_REPAIR_REPORT_PATH:-}}\" ]; then\n  if [ -z \"${{ENTROPYFA_REPAIR_OUTPUT_PATH:-}}\" ] || [ -z \"${{ENTROPYFA_REPAIR_REPORT_PATH:-}}\" ]; then\n    echo repair mode requires both ENTROPYFA_REPAIR_OUTPUT_PATH and ENTROPYFA_REPAIR_REPORT_PATH >&2\n    exit 1\n  fi\n  output_path=\"$ENTROPYFA_REPAIR_OUTPUT_PATH\"\n  report_path=\"$ENTROPYFA_REPAIR_REPORT_PATH\"\n  cat > \"$output_path\" <<'EOF'\n{repair_output_json}\nEOF\n  perl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$output_path\"\n  cat > \"$report_path\" <<'EOF'\n# Repair Report\n\n## Summary\n- {repair_report_summary}\nEOF\n  echo repair-complete\nelse\n  cat > \"$ENTROPYFA_PRIMARY_OUTPUT_PATH\" <<'EOF'\n{primary_output_json}\nEOF\n  perl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$ENTROPYFA_PRIMARY_OUTPUT_PATH\"\n  cat > \"$ENTROPYFA_PRIMARY_REPORT_PATH\" <<'EOF'\n# Primary Extraction Report\n\n## Summary\n- extracted current IRMAA structure from CMS source\nEOF\n  echo primary-complete\nfi\n"
    );
    write_executable_script(path, &script);
}

fn fake_primary_output_value(
    run_id: &str,
    value_proposal: &Value,
    reference_pack_primer: Value,
) -> Value {
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
            "tool": "claude_code",
            "model": "claude-opus-4-6"
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
        "schema_change_required": false,
        "schema_change_notes": [],
        "reference_pack_primer": reference_pack_primer,
        "value_proposal": value_proposal,
        "field_evidence": field_evidence,
        "unresolved_issues": []
    });
    primary_output
}

fn write_fake_verifier_agent(path: &Path, value_proposal: &Value) {
    let field_verdicts = required_field_paths(value_proposal)
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
    let verifier_output = json!({
        "schema_version": 1,
        "run_id": "$ENTROPYFA_RUN_ID",
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
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
        "primer_verdicts": sample_primer_verdicts(),
        "status_recommendation": "authoritative",
        "overall_verdict": "pass",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": ""
    });
    let verifier_output_json = serde_json::to_string_pretty(&verifier_output).unwrap();
    let script = format!(
        "#!/bin/sh\nset -eu\ncat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<'EOF'\n{verifier_output_json}\nEOF\nperl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\"\ncat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\nEOF\necho verifier-complete\n"
    );
    write_executable_script(path, &script);
}

fn write_delayed_fake_verifier_agent(path: &Path, value_proposal: &Value) {
    let field_verdicts = required_field_paths(value_proposal)
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
    let verifier_output = json!({
        "schema_version": 1,
        "run_id": "$ENTROPYFA_RUN_ID",
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
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
        "primer_verdicts": sample_primer_verdicts(),
        "status_recommendation": "authoritative",
        "overall_verdict": "pass",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": ""
    });
    let verifier_output_json = serde_json::to_string_pretty(&verifier_output).unwrap();
    let script = format!(
        "#!/bin/sh\nset -eu\n(\n  sleep 1\n  cat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<'EOF'\n{verifier_output_json}\nEOF\n  perl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\"\n  cat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\nEOF\n) &\necho verifier-delayed\n"
    );
    write_executable_script(path, &script);
}

fn write_blocking_fake_verifier_agent(path: &Path, value_proposal: &Value) {
    let field_verdicts = required_field_paths(value_proposal)
        .into_iter()
        .enumerate()
        .map(|(index, field_path)| {
            if index == 0 {
                json!({
                    "field_path": field_path,
                    "verdict": "dispute",
                    "corrected_value": Value::Null,
                    "source_ids": ["src_cms_1"],
                    "notes": "Value does not match source"
                })
            } else {
                json!({
                    "field_path": field_path,
                    "verdict": "confirm",
                    "corrected_value": Value::Null,
                    "source_ids": ["src_cms_1"],
                    "notes": ""
                })
            }
        })
        .collect::<Vec<_>>();
    let verifier_output = json!({
        "schema_version": 1,
        "run_id": "$ENTROPYFA_RUN_ID",
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
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
        "primer_verdicts": sample_primer_verdicts(),
        "status_recommendation": "needs_human_attention",
        "overall_verdict": "needs_human_attention",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": "Primary proposal does not match source"
    });
    let verifier_output_json = serde_json::to_string_pretty(&verifier_output).unwrap();
    let script = format!(
        "#!/bin/sh\nset -eu\ncat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<'EOF'\n{verifier_output_json}\nEOF\nperl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\"\ncat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\n- disputed first field\nEOF\necho verifier-blocked\n"
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
fn run_agents_prepares_executes_and_auto_applies_on_clean_review() {
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
    assert!(outcome.review.approved);
    assert!(outcome.review.blocking_issues.is_empty());
    let applied = outcome
        .applied
        .as_ref()
        .expect("run-agents should auto-apply clean runs");
    assert!(applied.reviewed_artifact_path.exists());
    assert!(applied.reference_pack_path.exists());

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
    assert!(outcome.applied.is_some());
}

#[test]
fn run_agents_does_not_auto_apply_when_review_blocks() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");
    write_fake_primary_agent(&primary_bin, &value_proposal);
    write_blocking_fake_verifier_agent(&verifier_bin, &value_proposal);

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

    assert!(!outcome.review.approved);
    assert!(!outcome.review.blocking_issues.is_empty());
    assert!(outcome.applied.is_none());
}

#[test]
fn run_agents_auto_repairs_safe_primer_disputes_and_applies() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");

    let repaired_output = fake_primary_output_value(
        "$ENTROPYFA_RUN_ID",
        &value_proposal,
        sample_reference_pack_primer(),
    );
    write_fake_primary_agent_with_repair_mode(
        &primary_bin,
        &value_proposal,
        overbroad_reference_pack_primer(),
        Some(repaired_output),
        Some("Trimmed primer only; reviewed numeric values preserved."),
    );
    let field_verdicts = required_field_paths(&value_proposal)
        .into_iter()
        .map(|field_path| {
            field_verdict_with_metadata(
                &field_path,
                "confirm",
                "",
                "primer_scope_only",
                true,
                "Leave the reviewed numeric values untouched and trim only the primer prose.",
            )
        })
        .collect::<Vec<_>>();
    let primer_verdicts = primer_verdicts_from_specs(&safe_primer_dispute_specs());
    write_fake_verifier_agent_with_payload(
        &verifier_bin,
        &value_proposal,
        field_verdicts,
        primer_verdicts,
        "needs_human_attention",
        "needs_human_attention",
        "Primer sections are overbroad but safe to trim without changing the value proposal.",
        "schema_change_required: false",
    );

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

    assert!(
        outcome.prepared.run_dir.join("repair_output.json").exists(),
        "future repair path should write repair_output.json"
    );
    assert!(outcome.review.blocking_issues.is_empty());
    assert!(outcome.applied.is_some());
}

#[test]
fn run_agents_does_not_auto_repair_manual_required_value_dispute() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");

    write_fake_primary_agent(&primary_bin, &value_proposal);
    let mut field_verdicts = required_field_paths(&value_proposal)
        .into_iter()
        .map(|field_path| {
            field_verdict_with_metadata(
                &field_path,
                "confirm",
                "",
                "value_confirmed",
                true,
                "No repair needed for this field.",
            )
        })
        .collect::<Vec<_>>();
    field_verdicts[0] = field_verdict_with_metadata(
        &required_field_paths(&value_proposal)[0],
        "dispute",
        "Reviewed numeric value does not match source",
        "value_mismatch",
        false,
        "Escalate this field to manual review; do not auto-repair numeric values.",
    );
    let primer_verdicts = primer_verdicts_from_specs(&manual_required_value_dispute_primer_specs());
    write_fake_verifier_agent_with_payload(
        &verifier_bin,
        &value_proposal,
        field_verdicts,
        primer_verdicts,
        "needs_human_attention",
        "needs_human_attention",
        "A required numeric field is disputed and should remain manual-only.",
        "schema_change_required: false",
    );

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

    assert!(
        !outcome.prepared.run_dir.join("repair_output.json").exists(),
        "manual-required blocker should never produce repair_output.json"
    );
    assert!(outcome.applied.is_none());
    let primary_stdout = fs::read_to_string(&outcome.primary.stdout_log_path).unwrap();
    assert!(
        !primary_stdout.contains("repair-complete"),
        "manual-required blocker should not invoke repair mode"
    );
    let review_json: Value =
        serde_json::from_str(&fs::read_to_string(&outcome.review.review_path).unwrap()).unwrap();
    assert_eq!(review_json["auto_repair_eligible"], json!(false));
    assert_eq!(
        review_json["manual_required_blockers"][0]["issue_type"],
        json!("value_mismatch")
    );
}

#[test]
fn run_agents_rejects_repair_that_mutates_value_during_safe_repair() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");

    let mut unsafe_repair_output = fake_primary_output_value(
        "$ENTROPYFA_RUN_ID",
        &value_proposal,
        sample_reference_pack_primer(),
    );
    unsafe_repair_output["value_proposal"]["variants"][0]["value"]["base_part_b_premium"] =
        json!(999.0);
    write_fake_primary_agent_with_repair_mode(
        &primary_bin,
        &value_proposal,
        overbroad_reference_pack_primer(),
        Some(unsafe_repair_output),
        Some("Attempted repair mutated the reviewed numeric value and should be rejected."),
    );
    let field_verdicts = required_field_paths(&value_proposal)
        .into_iter()
        .map(|field_path| {
            field_verdict_with_metadata(
                &field_path,
                "confirm",
                "",
                "primer_scope_only",
                true,
                "Only trim primer prose; keep all reviewed values untouched.",
            )
        })
        .collect::<Vec<_>>();
    let primer_verdicts = primer_verdicts_from_specs(&safe_primer_dispute_specs());
    write_fake_verifier_agent_with_payload(
        &verifier_bin,
        &value_proposal,
        field_verdicts,
        primer_verdicts,
        "needs_human_attention",
        "needs_human_attention",
        "Primer-only repair should be blocked if the repair agent mutates numeric values.",
        "schema_change_required: false",
    );

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

    let review_json: Value =
        serde_json::from_str(&fs::read_to_string(&outcome.review.review_path).unwrap()).unwrap();
    assert_eq!(
        review_json["manual_required_blockers"][0]["issue_type"],
        json!("unsafe_repair_mutated_value")
    );
    assert!(
        outcome.prepared.run_dir.join("repair_output.json").exists(),
        "future repair path should write repair_output.json"
    );
    assert!(outcome.applied.is_none());
}

#[test]
fn run_agents_auto_repairs_source_level_citation_locator_inexact() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");

    let mut primary_output = fake_primary_output_value(
        "$ENTROPYFA_RUN_ID",
        &value_proposal,
        sample_reference_pack_primer(),
    );
    primary_output["sources"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "source_id": "src_cms_2",
            "url": "https://www.cms.gov/newsroom/fact-sheets/example-irmaa-release-appendix",
            "host": "www.cms.gov",
            "organization": "CMS",
            "source_class": "primary",
            "title": "Example CMS IRMAA Release Appendix",
            "published_at": "2025-11-07",
            "locator": "Appendix",
            "notes": null
        }));

    let mut repaired_output = primary_output.clone();
    repaired_output["sources"][0]["locator"] = json!("Table 2, row 1");
    repaired_output["sources"][1]["locator"] = json!("Appendix A, row 1");
    let primary_output_json = serde_json::to_string_pretty(&primary_output).unwrap();
    let repair_output_json = serde_json::to_string_pretty(&repaired_output).unwrap();
    let script = format!(
        "#!/bin/sh\nset -eu\nif [ -n \"${{ENTROPYFA_REPAIR_OUTPUT_PATH:-}}\" ] || [ -n \"${{ENTROPYFA_REPAIR_REPORT_PATH:-}}\" ]; then\n  output_path=\"$ENTROPYFA_REPAIR_OUTPUT_PATH\"\n  report_path=\"$ENTROPYFA_REPAIR_REPORT_PATH\"\n  cat > \"$output_path\" <<'EOF'\n{repair_output_json}\nEOF\n  perl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$output_path\"\n  cat > \"$report_path\" <<'EOF'\n# Repair Report\n\n## Summary\n- Tightened the source locator only; reviewed values preserved.\nEOF\n  echo repair-complete\nelse\n  cat > \"$ENTROPYFA_PRIMARY_OUTPUT_PATH\" <<'EOF'\n{primary_output_json}\nEOF\n  perl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$ENTROPYFA_PRIMARY_OUTPUT_PATH\"\n  cat > \"$ENTROPYFA_PRIMARY_REPORT_PATH\" <<'EOF'\n# Primary Extraction Report\n\n## Summary\n- extracted current IRMAA structure from CMS source\nEOF\n  echo primary-complete\nfi\n"
    );
    write_executable_script(&primary_bin, &script);

    let field_verdicts = required_field_paths(&value_proposal)
        .into_iter()
        .map(|field_path| {
            field_verdict_with_metadata(&field_path, "confirm", "", "value_confirmed", false, "")
        })
        .collect::<Vec<_>>();
    let verifier_output = json!({
        "schema_version": 1,
        "run_id": "$ENTROPYFA_RUN_ID",
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
        },
        "source_verdicts": [
            {
                "source_id": "src_cms_1",
                "verdict": "accept",
                "counts_toward_status": true,
                "issue_type": "value_confirmed",
                "auto_resolvable": false,
                "reason": "Primary CMS source remains accepted.",
                "repair_guidance": ""
            },
            {
                "source_id": "src_cms_2",
                "verdict": "reject",
                "counts_toward_status": false,
                "reason": "Locator is not exact enough",
                "issue_type": "citation_locator_inexact",
                "auto_resolvable": true,
                "repair_guidance": "Tighten the locator to the exact table row that supports the value."
            }
        ],
        "field_verdicts": field_verdicts,
        "primer_verdicts": sample_primer_verdicts(),
        "status_recommendation": "authoritative",
        "overall_verdict": "needs_human_attention",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": "The value is fine; only the cited locator needs tightening."
    });
    let repair_verifier_output = json!({
        "schema_version": 1,
        "run_id": "$ENTROPYFA_RUN_ID",
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
        },
        "source_verdicts": [
            {
                "source_id": "src_cms_1",
                "verdict": "accept",
                "counts_toward_status": true,
                "issue_type": "value_confirmed",
                "auto_resolvable": false,
                "reason": "Primary CMS source remains accepted.",
                "repair_guidance": ""
            },
            {
                "source_id": "src_cms_2",
                "verdict": "accept",
                "counts_toward_status": false,
                "issue_type": "value_confirmed",
                "auto_resolvable": false,
                "reason": "Locator is now specific enough after repair.",
                "repair_guidance": ""
            }
        ],
        "field_verdicts": field_verdicts,
        "primer_verdicts": sample_primer_verdicts(),
        "status_recommendation": "authoritative",
        "overall_verdict": "pass",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": "The value is fine and the repaired locator is now exact enough."
    });
    let verifier_output_json = serde_json::to_string_pretty(&verifier_output).unwrap();
    let repair_verifier_output_json =
        serde_json::to_string_pretty(&repair_verifier_output).unwrap();
    let script = format!(
        "#!/bin/sh\nset -eu\nif [ \"$ENTROPYFA_AGENT_ROLE\" = \"repair_verifier\" ]; then\n  cat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<'EOF'\n{repair_verifier_output_json}\nEOF\n  cat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\n- repaired source locator now supports the value cleanly\nEOF\nelse\n  cat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<'EOF'\n{verifier_output_json}\nEOF\n  cat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\n- source locator needs tightening\nEOF\nfi\nperl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\"\necho verifier-complete\n"
    );
    write_executable_script(&verifier_bin, &script);

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

    assert!(
        outcome.prepared.run_dir.join("repair_output.json").exists(),
        "citation-only repair path should write repair_output.json"
    );
    assert!(outcome.review.blocking_issues.is_empty());
    assert!(outcome.applied.is_some());
}

#[test]
fn run_agents_auto_repairs_field_level_citation_locator_inexact() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");

    let mut repaired_output = fake_primary_output_value(
        "$ENTROPYFA_RUN_ID",
        &value_proposal,
        sample_reference_pack_primer(),
    );
    repaired_output["field_evidence"][0]["locator"] = json!("Table 2, row 1");
    write_fake_primary_agent_with_repair_mode(
        &primary_bin,
        &value_proposal,
        sample_reference_pack_primer(),
        Some(repaired_output),
        Some("Tightened the field-evidence locator only; reviewed values preserved."),
    );

    let required_paths = required_field_paths(&value_proposal);
    let mut field_verdicts = Vec::new();
    for (index, field_path) in required_paths.iter().enumerate() {
        if index == 0 {
            field_verdicts.push(field_verdict_with_metadata(
                field_path,
                "dispute",
                "The field evidence locator is not exact enough",
                "citation_locator_inexact",
                true,
                "Tighten the field-evidence locator without changing the reviewed value.",
            ));
        } else {
            field_verdicts.push(field_verdict_with_metadata(
                field_path,
                "confirm",
                "",
                "value_confirmed",
                false,
                "",
            ));
        }
    }
    let repair_field_verdicts = required_paths
        .iter()
        .map(|field_path| {
            field_verdict_with_metadata(field_path, "confirm", "", "value_confirmed", false, "")
        })
        .collect::<Vec<_>>();
    let verifier_output = json!({
        "schema_version": 1,
        "run_id": "$ENTROPYFA_RUN_ID",
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
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
        "primer_verdicts": sample_primer_verdicts(),
        "status_recommendation": "authoritative",
        "overall_verdict": "needs_human_attention",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": "The reviewed value is correct; one field-evidence locator needs tightening.",
        "value_proposal": value_proposal,
    });
    let repair_verifier_output = json!({
        "schema_version": 1,
        "run_id": "$ENTROPYFA_RUN_ID",
        "agent": {
            "tool": "codex",
            "model": "gpt-5.4"
        },
        "source_verdicts": [
            {
                "source_id": "src_cms_1",
                "verdict": "accept",
                "counts_toward_status": true,
                "reason": "Primary CMS source"
            }
        ],
        "field_verdicts": repair_field_verdicts,
        "primer_verdicts": sample_primer_verdicts(),
        "status_recommendation": "authoritative",
        "overall_verdict": "pass",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": "The reviewed value is correct and the field-evidence locator is now exact enough.",
        "value_proposal": value_proposal,
    });
    let verifier_output_json = serde_json::to_string_pretty(&verifier_output).unwrap();
    let repair_verifier_output_json =
        serde_json::to_string_pretty(&repair_verifier_output).unwrap();
    let script = format!(
        "#!/bin/sh\nset -eu\nif [ \"$ENTROPYFA_AGENT_ROLE\" = \"repair_verifier\" ]; then\n  cat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<'EOF'\n{repair_verifier_output_json}\nEOF\n  cat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\n- repaired field-evidence locator now supports the value cleanly\nEOF\nelse\n  cat > \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\" <<'EOF'\n{verifier_output_json}\nEOF\n  cat > \"$ENTROPYFA_VERIFIER_REPORT_PATH\" <<'EOF'\n# Verifier Review Report\n\n## Overall Assessment\n- schema_change_required: false\n- field-evidence locator needs tightening\nEOF\nfi\nperl -0pi -e 's/\\$ENTROPYFA_RUN_ID/$ENV{{ENTROPYFA_RUN_ID}}/g' \"$ENTROPYFA_VERIFIER_OUTPUT_PATH\"\necho verifier-complete\n"
    );
    write_executable_script(&verifier_bin, &script);

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

    assert!(
        outcome.prepared.run_dir.join("repair_output.json").exists(),
        "field-level citation repair path should write repair_output.json"
    );
    assert!(outcome.review.blocking_issues.is_empty());
    assert!(outcome.applied.is_some());
}

#[test]
fn manual_review_uses_repair_artifacts_after_blocked_repair() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");

    let mut unsafe_repair_output = fake_primary_output_value(
        "$ENTROPYFA_RUN_ID",
        &value_proposal,
        sample_reference_pack_primer(),
    );
    unsafe_repair_output["value_proposal"]["variants"][0]["value"]["base_part_b_premium"] =
        json!(999.0);
    write_fake_primary_agent_with_repair_mode(
        &primary_bin,
        &value_proposal,
        overbroad_reference_pack_primer(),
        Some(unsafe_repair_output),
        Some("Attempted repair mutated the reviewed numeric value and should be rejected."),
    );
    let field_verdicts = required_field_paths(&value_proposal)
        .into_iter()
        .map(|field_path| {
            field_verdict_with_metadata(
                &field_path,
                "confirm",
                "",
                "primer_scope_only",
                true,
                "Only trim primer prose; keep all reviewed values untouched.",
            )
        })
        .collect::<Vec<_>>();
    let primer_verdicts = primer_verdicts_from_specs(&safe_primer_dispute_specs());
    write_fake_verifier_agent_with_payload(
        &verifier_bin,
        &value_proposal,
        field_verdicts,
        primer_verdicts,
        "needs_human_attention",
        "needs_human_attention",
        "Primer-only repair should be blocked if the repair agent mutates numeric values.",
        "schema_change_required: false",
    );

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

    let repaired_output_path = outcome.prepared.run_dir.join("repair_output.json");
    let repaired_output = fake_primary_output_value(
        &outcome.prepared.run_id,
        &value_proposal,
        sample_reference_pack_primer(),
    );
    fs::write(
        &repaired_output_path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&repaired_output).unwrap()
        ),
    )
    .unwrap();
    fs::write(
        outcome.prepared.run_dir.join("repair_report.md"),
        "# Repair Report\n\n## Summary\n- repaired manually for rereview\n",
    )
    .unwrap();

    let manual_review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &outcome.prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();

    assert!(
        manual_review.blocking_issues.is_empty(),
        "manual review should reread repair_output.json when it exists"
    );
    assert!(manual_review.approved);

    let review_json: Value =
        serde_json::from_str(&fs::read_to_string(&manual_review.review_path).unwrap()).unwrap();
    assert_eq!(
        review_json["reference_pack_primer"]["what_this_is"],
        sample_reference_pack_primer()["what_this_is"].clone()
    );
}

#[test]
fn run_agents_preserves_initial_verifier_and_review_artifacts_before_rereview() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let bootstrap =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&bootstrap.run_dir);

    let primary_bin = engine_root.parent().unwrap().join("fake-claude");
    let verifier_bin = engine_root.parent().unwrap().join("fake-codex");

    let repaired_output = fake_primary_output_value(
        "$ENTROPYFA_RUN_ID",
        &value_proposal,
        sample_reference_pack_primer(),
    );
    write_fake_primary_agent_with_repair_mode(
        &primary_bin,
        &value_proposal,
        overbroad_reference_pack_primer(),
        Some(repaired_output),
        Some("Trimmed primer only; reviewed numeric values preserved."),
    );
    let field_verdicts = required_field_paths(&value_proposal)
        .into_iter()
        .map(|field_path| {
            field_verdict_with_metadata(
                &field_path,
                "confirm",
                "",
                "primer_scope_only",
                true,
                "Leave the reviewed numeric values untouched and trim only the primer prose.",
            )
        })
        .collect::<Vec<_>>();
    let primer_verdicts = primer_verdicts_from_specs(&safe_primer_dispute_specs());
    write_fake_verifier_agent_with_payload(
        &verifier_bin,
        &value_proposal,
        field_verdicts,
        primer_verdicts,
        "needs_human_attention",
        "needs_human_attention",
        "Primer sections are overbroad but safe to trim without changing the value proposal.",
        "schema_change_required: false",
    );

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

    let initial_verifier_output = outcome
        .prepared
        .run_dir
        .join("initial_verifier_output.json");
    let initial_verifier_report = outcome.prepared.run_dir.join("initial_verifier_report.md");
    let initial_review_json = outcome.prepared.run_dir.join("initial_review.json");
    let initial_review_md = outcome.prepared.run_dir.join("initial_review.md");

    assert!(initial_verifier_output.exists());
    assert!(initial_verifier_report.exists());
    assert!(initial_review_json.exists());
    assert!(initial_review_md.exists());

    let initial_review: Value =
        serde_json::from_str(&fs::read_to_string(&initial_review_json).unwrap()).unwrap();
    assert_eq!(initial_review["approved"], json!(false));
    assert_eq!(initial_review["auto_repair_eligible"], json!(true));
    assert!(!initial_review["blocking_issues"]
        .as_array()
        .unwrap()
        .is_empty());
    assert!(outcome.review.blocking_issues.is_empty());
}

#[test]
fn review_blocks_when_reference_pack_primer_is_missing() {
    let (_temp_dir, engine_root) = setup_temp_engine_root();
    let prepared =
        data_pipeline::prepare_run_at(&engine_root, 2026, "insurance", "irmaa_brackets").unwrap();
    let value_proposal = load_value_proposal(&prepared.run_dir);

    write_primary_output(&prepared.run_dir, &prepared.run_id, &value_proposal, false);
    let primary_output_path = prepared.run_dir.join("primary_output.json");
    let mut primary_output: Value =
        serde_json::from_str(&fs::read_to_string(&primary_output_path).unwrap()).unwrap();
    primary_output
        .as_object_mut()
        .unwrap()
        .remove("reference_pack_primer");
    fs::write(
        &primary_output_path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&primary_output).unwrap()
        ),
    )
    .unwrap();

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

    assert!(!review.approved);
    assert!(review
        .blocking_issues
        .iter()
        .any(|issue| issue.contains("reference_pack_primer.what_this_is")));
}

#[test]
fn review_blocks_when_verifier_disputes_required_primer_section() {
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
    let verifier_output_path = prepared.run_dir.join("verifier_output.json");
    let mut verifier_output: Value =
        serde_json::from_str(&fs::read_to_string(&verifier_output_path).unwrap()).unwrap();
    verifier_output["primer_verdicts"]["what_this_is"]["verdict"] = json!("dispute");
    verifier_output["primer_verdicts"]["what_this_is"]["notes"] =
        json!("Primer overstates what the dataset contains");
    fs::write(
        &verifier_output_path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&verifier_output).unwrap()
        ),
    )
    .unwrap();
    write_reports(&prepared.run_dir, false);

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
        .any(|issue| issue.contains("primer section what_this_is")));
}

#[test]
fn apply_writes_reference_pack_with_required_primer_sections() {
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
    let review = data_pipeline::review_run_with_approval_at(
        &engine_root,
        &prepared.run_id,
        true,
        Some("tester".into()),
    )
    .unwrap();
    assert!(review.approved, "review should pass before apply");

    let applied = data_pipeline::apply_run_at(&engine_root, &prepared.run_id).unwrap();
    let pack = fs::read_to_string(applied.reference_pack_path).unwrap();

    assert!(pack.contains("## What This Is"));
    assert!(pack.contains("## Lookup Parameters"));
    assert!(pack.contains("## Interpretation Notes"));
    assert!(pack.contains("## Does Not Include"));
    assert!(pack.contains("## Caveats"));
    assert!(pack.contains("Annual federal IRMAA surcharge thresholds"));
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
    assert_eq!(report.registry_entries, 30);
    assert_eq!(report.pipeline_definitions, 30);
    assert_eq!(report.reviewed_artifacts, 12);
    assert_eq!(report.reference_packs, 1);
    assert_eq!(report.legacy_only_entries, 11);

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
