use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use url::Url;

use super::{
    canonicalize, data_registry_root, engine_root, fnv1a64, load_registry, lookup_entry_variants,
    validate_value, write_registry, Completeness, PipelineError, RegistryDocument, RegistryEntry,
    SnapshotParams, SourceDocument, ValidationProfile, VerificationStatus,
};

const PIPELINE_DEFINITION_SCHEMA_VERSION: u32 = 1;
const RUN_SCHEMA_VERSION: u32 = 1;
const REVIEWED_ARTIFACT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDefinition {
    pub schema_version: u32,
    pub pipeline_name: String,
    pub category: String,
    pub key: String,
    pub year_strategy: YearStrategy,
    pub supported_years: Vec<u32>,
    pub validation_profile: ValidationProfile,
    pub generator_kind: GeneratorKind,
    pub target_source_path: String,
    pub expected_variants: Vec<ExpectedVariant>,
    pub required_primary_hosts: Vec<String>,
    pub allowed_supporting_hosts: Vec<String>,
    pub allowed_secondary_hosts: Vec<String>,
    pub minimum_secondary_confirmations: usize,
    pub require_exact_citation: bool,
    pub search_queries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum YearStrategy {
    Fixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorKind {
    IrmaaRust,
    TaxFederalBracketsRust,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExpectedVariant {
    pub label: String,
    pub params: SnapshotParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePolicyDocument {
    pub schema_version: u32,
    pub pipeline_name: String,
    pub category: String,
    pub key: String,
    pub year: u32,
    pub required_primary_hosts: Vec<String>,
    pub allowed_supporting_hosts: Vec<String>,
    pub allowed_secondary_hosts: Vec<String>,
    pub minimum_secondary_confirmations: usize,
    pub require_exact_citation: bool,
    pub search_queries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunManifest {
    pub schema_version: u32,
    pub run_id: String,
    pub pipeline_name: String,
    pub category: String,
    pub key: String,
    pub year: u32,
    pub status: RunStatus,
    pub expected_variants: Vec<ExpectedVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Prepared,
    Reviewed,
    Applied,
}

impl std::fmt::Display for RunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prepared => f.write_str("prepared"),
            Self::Reviewed => f.write_str("reviewed"),
            Self::Applied => f.write_str("applied"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentValueArtifact {
    pub schema_version: u32,
    pub category: String,
    pub key: String,
    pub year: u32,
    pub verification_status: VerificationStatus,
    pub completeness: Completeness,
    pub value: ValueProposal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDescriptor {
    pub tool: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    Primary,
    SupportingOfficial,
    Secondary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRecord {
    pub source_id: String,
    pub url: String,
    pub host: String,
    pub organization: String,
    pub source_class: SourceClass,
    pub title: String,
    pub published_at: Option<String>,
    pub locator: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueProposal {
    pub variants: Vec<ValueVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueVariant {
    pub label: String,
    pub params: SnapshotParams,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldEvidence {
    pub field_path: String,
    pub source_id: String,
    pub locator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimarySubmission {
    pub schema_version: u32,
    pub run_id: String,
    pub agent: AgentDescriptor,
    pub sources: Vec<SourceRecord>,
    pub proposed_status: VerificationStatus,
    #[serde(default)]
    pub schema_change_required: bool,
    #[serde(default)]
    pub schema_change_notes: Vec<String>,
    pub value_proposal: ValueProposal,
    pub field_evidence: Vec<FieldEvidence>,
    pub unresolved_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceVerdictDecision {
    Accept,
    Reject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceVerdict {
    pub source_id: String,
    pub verdict: SourceVerdictDecision,
    pub counts_toward_status: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FieldVerdictDecision {
    Confirm,
    Dispute,
    Uncertain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldVerdict {
    pub field_path: String,
    pub verdict: FieldVerdictDecision,
    pub corrected_value: Option<Value>,
    pub source_ids: Vec<String>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StatusRecommendation {
    Authoritative,
    Corroborated,
    NeedsHumanAttention,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OverallVerdict {
    Pass,
    NeedsHumanAttention,
    Reject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierSubmission {
    pub schema_version: u32,
    pub run_id: String,
    pub agent: AgentDescriptor,
    pub source_verdicts: Vec<SourceVerdict>,
    pub field_verdicts: Vec<FieldVerdict>,
    pub status_recommendation: StatusRecommendation,
    pub overall_verdict: OverallVerdict,
    #[serde(default)]
    pub schema_change_required: bool,
    #[serde(default)]
    pub schema_change_notes: Vec<String>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptedSource {
    pub source_id: String,
    pub url: String,
    pub host: String,
    pub organization: String,
    pub source_class: SourceClass,
    pub title: String,
    pub published_at: Option<String>,
    pub locator: Option<String>,
    pub counts_toward_status: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataPatch {
    pub verification_status: VerificationStatus,
    pub completeness: Completeness,
    pub source_documents: Vec<SourceDocument>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewDecision {
    pub schema_version: u32,
    pub run_id: String,
    pub approved: bool,
    pub approved_by: Option<String>,
    pub status_decision: VerificationStatus,
    pub blocking_issues: Vec<String>,
    pub warnings: Vec<String>,
    pub accepted_sources: Vec<AcceptedSource>,
    pub final_value: ValueProposal,
    pub metadata_patch: MetadataPatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewedArtifact {
    pub schema_version: u32,
    pub category: String,
    pub key: String,
    pub year: u32,
    pub verification_status: VerificationStatus,
    pub completeness: Completeness,
    pub accepted_sources: Vec<AcceptedSource>,
    pub value: ValueProposal,
}

#[derive(Debug, Clone)]
pub struct PreparedRun {
    pub run_id: String,
    pub run_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentProvider {
    Claude,
    Codex,
}

#[derive(Debug, Clone)]
pub struct AgentInvocationConfig {
    pub provider: AgentProvider,
    pub model: String,
    pub binary: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct RunAgentsConfig {
    pub year: u32,
    pub category: String,
    pub key: String,
    pub primary: AgentInvocationConfig,
    pub verifier: AgentInvocationConfig,
}

#[derive(Debug, Clone)]
pub struct AgentExecutionLog {
    pub provider: AgentProvider,
    pub model: String,
    pub stdout_log_path: PathBuf,
    pub stderr_log_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ReviewOutcome {
    pub run_id: String,
    pub run_dir: PathBuf,
    pub review_path: PathBuf,
    pub review_markdown_path: PathBuf,
    pub approved: bool,
    pub status_decision: VerificationStatus,
    pub warnings: Vec<String>,
    pub blocking_issues: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RunAgentsOutcome {
    pub prepared: PreparedRun,
    pub primary: AgentExecutionLog,
    pub verifier: AgentExecutionLog,
    pub review: ReviewOutcome,
}

#[derive(Debug, Clone)]
pub struct ApplyOutcome {
    pub run_id: String,
    pub year: u32,
    pub category: String,
    pub key: String,
    pub run_dir: PathBuf,
    pub reviewed_artifact_path: PathBuf,
    pub generated_source_path: PathBuf,
    pub metadata_path: PathBuf,
    pub snapshot_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PipelineStatusReport {
    pub year: u32,
    pub registry_entries: usize,
    pub pipeline_definitions: usize,
    pub reviewed_artifacts: usize,
    pub authoritative_entries: usize,
    pub corroborated_entries: usize,
    pub derived_entries: usize,
    pub placeholder_entries: usize,
    pub entries: Vec<PipelineStatusEntry>,
}

#[derive(Debug, Clone)]
pub struct PipelineStatusEntry {
    pub category: String,
    pub key: String,
    pub verification_status: VerificationStatus,
    pub completeness: Completeness,
    pub pipeline_defined: bool,
    pub reviewed_artifact_exists: bool,
    pub latest_run: Option<PipelineRunSummary>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PipelineRunSummary {
    pub run_id: String,
    pub status: RunStatus,
    pub approved: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PolicyMatchKind {
    Primary,
    Supporting,
    Secondary,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AgentRole {
    Primary,
    Verifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerminalStreamMode {
    Raw,
    ClaudeJson,
    CodexJson,
}

pub fn default_pipeline_definitions_dir() -> PathBuf {
    data_registry_root().join("pipelines")
}

pub fn default_pipeline_definition_path(category: &str, key: &str) -> PathBuf {
    default_pipeline_definitions_dir()
        .join(category)
        .join(format!("{key}.json"))
}

pub fn default_runs_root() -> PathBuf {
    data_registry_root().join("runs")
}

pub fn default_reviewed_root(year: u32) -> PathBuf {
    data_registry_root().join(year.to_string()).join("reviewed")
}

pub fn prepare_run(year: u32, category: &str, key: &str) -> Result<PreparedRun, PipelineError> {
    prepare_run_at(&engine_root(), year, category, key)
}

pub fn run_agents(config: &RunAgentsConfig) -> Result<RunAgentsOutcome, PipelineError> {
    run_agents_at(&engine_root(), config)
}

pub fn status_report(year: u32) -> Result<PipelineStatusReport, PipelineError> {
    status_report_at(&engine_root(), year)
}

pub fn prepare_run_at(
    engine_root: &Path,
    year: u32,
    category: &str,
    key: &str,
) -> Result<PreparedRun, PipelineError> {
    let definition = load_pipeline_definition_at(engine_root, category, key)?;
    ensure_year_supported(&definition, year)?;

    let expected_variants = ensure_expected_variants(category, key, &definition.expected_variants)?;
    let current_value = build_current_value(year, category, key, &expected_variants)?;
    let registry = load_registry(&metadata_path_for(engine_root, year))?;
    let current_entry = find_registry_entry(&registry, category, key)?;

    let run_id = generate_run_id(year, category, key);
    let run_dir = runs_root_for(engine_root)
        .join(year.to_string())
        .join(category)
        .join(key)
        .join(&run_id);
    fs::create_dir_all(&run_dir)?;

    let run_manifest = RunManifest {
        schema_version: RUN_SCHEMA_VERSION,
        run_id: run_id.clone(),
        pipeline_name: definition.pipeline_name.clone(),
        category: category.to_string(),
        key: key.to_string(),
        year,
        status: RunStatus::Prepared,
        expected_variants: expected_variants.clone(),
    };
    let current_artifact = CurrentValueArtifact {
        schema_version: REVIEWED_ARTIFACT_SCHEMA_VERSION,
        category: category.to_string(),
        key: key.to_string(),
        year,
        verification_status: current_entry.verification_status,
        completeness: current_entry.completeness,
        value: current_value.clone(),
    };
    let source_policy = build_source_policy(&definition, year);
    let primary_template = build_primary_template(&run_manifest, &definition);
    let verifier_template = build_verifier_template(&run_manifest, &definition);

    write_json(&run_dir.join("run.json"), &run_manifest)?;
    write_json(&run_dir.join("source_policy.json"), &source_policy)?;
    write_json(&run_dir.join("current_value.json"), &current_artifact)?;
    write_json_value(&run_dir.join("primary_template.json"), &primary_template)?;
    write_json_value(&run_dir.join("verifier_template.json"), &verifier_template)?;
    write_text(
        &run_dir.join("primary_report_template.md"),
        &render_primary_report_template(&run_manifest),
    )?;
    write_text(
        &run_dir.join("verifier_report_template.md"),
        &render_verifier_report_template(&run_manifest, &definition)?,
    )?;
    write_text(
        &run_dir.join("primary_prompt.md"),
        &render_primary_prompt(&run_dir, &run_manifest, &definition),
    )?;
    write_text(
        &run_dir.join("verifier_prompt.md"),
        &render_verifier_prompt(&run_dir, &run_manifest, &definition),
    )?;

    Ok(PreparedRun { run_id, run_dir })
}

pub fn run_agents_at(
    engine_root: &Path,
    config: &RunAgentsConfig,
) -> Result<RunAgentsOutcome, PipelineError> {
    let prepared = prepare_run_at(engine_root, config.year, &config.category, &config.key)?;
    let primary = execute_agent(
        engine_root,
        &prepared.run_dir,
        &config.primary,
        AgentRole::Primary,
    )?;
    let verifier = execute_agent(
        engine_root,
        &prepared.run_dir,
        &config.verifier,
        AgentRole::Verifier,
    )?;
    let review = review_run_with_approval_at(engine_root, &prepared.run_id, false, None)?;

    Ok(RunAgentsOutcome {
        prepared,
        primary,
        verifier,
        review,
    })
}

pub fn review_run(run_ref: &str) -> Result<ReviewOutcome, PipelineError> {
    review_run_at(&engine_root(), run_ref)
}

pub fn status_report_at(
    engine_root: &Path,
    year: u32,
) -> Result<PipelineStatusReport, PipelineError> {
    let registry = load_registry(&metadata_path_for(engine_root, year))?;
    let definitions = load_pipeline_definitions_for_year(engine_root, year)?;

    let mut authoritative_entries = 0;
    let mut corroborated_entries = 0;
    let mut derived_entries = 0;
    let mut placeholder_entries = 0;
    let mut reviewed_artifacts = 0;
    let mut entries = Vec::new();

    let mut sorted_entries = registry.entries.clone();
    sorted_entries.sort_by(|a, b| (&a.category, &a.key).cmp(&(&b.category, &b.key)));

    for entry in sorted_entries {
        match entry.verification_status {
            VerificationStatus::Authoritative => authoritative_entries += 1,
            VerificationStatus::Corroborated => corroborated_entries += 1,
            VerificationStatus::Derived => derived_entries += 1,
            VerificationStatus::Placeholder => placeholder_entries += 1,
        }

        let pipeline_defined = definitions
            .iter()
            .any(|definition| definition.category == entry.category && definition.key == entry.key);
        let reviewed_artifact_exists = reviewed_root_for(engine_root, year)
            .join(&entry.category)
            .join(format!("{}.json", entry.key))
            .exists();
        if reviewed_artifact_exists {
            reviewed_artifacts += 1;
        }
        let latest_run = latest_run_summary_for(engine_root, year, &entry.category, &entry.key)?;

        entries.push(PipelineStatusEntry {
            category: entry.category,
            key: entry.key,
            verification_status: entry.verification_status,
            completeness: entry.completeness,
            pipeline_defined,
            reviewed_artifact_exists,
            latest_run,
            notes: entry.notes,
        });
    }

    Ok(PipelineStatusReport {
        year,
        registry_entries: entries.len(),
        pipeline_definitions: definitions.len(),
        reviewed_artifacts,
        authoritative_entries,
        corroborated_entries,
        derived_entries,
        placeholder_entries,
        entries,
    })
}

pub fn review_run_at(engine_root: &Path, run_ref: &str) -> Result<ReviewOutcome, PipelineError> {
    let approver = default_approver();
    review_run_internal(engine_root, run_ref, None, approver)
}

pub fn review_run_with_approval(
    run_ref: &str,
    approved: bool,
    approved_by: Option<String>,
) -> Result<ReviewOutcome, PipelineError> {
    review_run_with_approval_at(&engine_root(), run_ref, approved, approved_by)
}

pub fn review_run_with_approval_at(
    engine_root: &Path,
    run_ref: &str,
    approved: bool,
    approved_by: Option<String>,
) -> Result<ReviewOutcome, PipelineError> {
    review_run_internal(engine_root, run_ref, Some(approved), approved_by)
}

pub fn apply_run(run_ref: &str) -> Result<ApplyOutcome, PipelineError> {
    apply_run_at(&engine_root(), run_ref)
}

pub fn apply_run_at(engine_root: &Path, run_ref: &str) -> Result<ApplyOutcome, PipelineError> {
    let run_dir = resolve_run_dir(engine_root, run_ref)?;
    let run_manifest: RunManifest = load_json(&run_dir.join("run.json"))?;
    let review: ReviewDecision = load_json(&run_dir.join("review.json"))?;
    let definition =
        load_pipeline_definition_at(engine_root, &run_manifest.category, &run_manifest.key)?;

    if !review.approved {
        return Err(PipelineError::new(format!(
            "review for run {} is not approved",
            run_manifest.run_id
        )));
    }
    if !review.blocking_issues.is_empty() {
        return Err(PipelineError::new(format!(
            "review for run {} still has blocking issues",
            run_manifest.run_id
        )));
    }

    let reviewed_artifact = ReviewedArtifact {
        schema_version: REVIEWED_ARTIFACT_SCHEMA_VERSION,
        category: run_manifest.category.clone(),
        key: run_manifest.key.clone(),
        year: run_manifest.year,
        verification_status: review.metadata_patch.verification_status,
        completeness: review.metadata_patch.completeness,
        accepted_sources: review.accepted_sources.clone(),
        value: review.final_value.clone(),
    };
    let reviewed_artifact_path = reviewed_root_for(engine_root, run_manifest.year)
        .join(&run_manifest.category)
        .join(format!("{}.json", run_manifest.key));
    write_json(&reviewed_artifact_path, &reviewed_artifact)?;

    let generated_source_path = engine_root.join(&definition.target_source_path);
    let generated_source = render_source(
        &engine_root.join(&definition.target_source_path),
        &definition,
        &reviewed_artifact,
    )?;
    write_text(&generated_source_path, &generated_source)?;

    let metadata_path = metadata_path_for(engine_root, run_manifest.year);
    update_registry_entry(
        &metadata_path,
        &run_manifest.category,
        &run_manifest.key,
        &review.metadata_patch,
    )?;

    update_run_status(&run_dir.join("run.json"), RunStatus::Applied)?;

    Ok(ApplyOutcome {
        run_id: run_manifest.run_id,
        year: run_manifest.year,
        category: run_manifest.category,
        key: run_manifest.key,
        run_dir,
        reviewed_artifact_path,
        generated_source_path,
        metadata_path,
        snapshot_path: snapshot_path_for(engine_root, run_manifest.year),
    })
}

fn review_run_internal(
    engine_root: &Path,
    run_ref: &str,
    approval_override: Option<bool>,
    approved_by: Option<String>,
) -> Result<ReviewOutcome, PipelineError> {
    let run_dir = resolve_run_dir(engine_root, run_ref)?;
    let run_manifest: RunManifest = load_json(&run_dir.join("run.json"))?;
    let definition =
        load_pipeline_definition_at(engine_root, &run_manifest.category, &run_manifest.key)?;
    let registry = load_registry(&metadata_path_for(engine_root, run_manifest.year))?;
    let current_entry = find_registry_entry(&registry, &run_manifest.category, &run_manifest.key)?;
    let current_artifact: CurrentValueArtifact = load_json(&run_dir.join("current_value.json"))?;
    let primary: PrimarySubmission = load_json(&run_dir.join("primary_output.json"))?;
    let verifier: VerifierSubmission = load_json(&run_dir.join("verifier_output.json"))?;

    if primary.run_id != run_manifest.run_id {
        return Err(PipelineError::new(format!(
            "primary_output.json run_id {} does not match run {}",
            primary.run_id, run_manifest.run_id
        )));
    }
    if verifier.run_id != run_manifest.run_id {
        return Err(PipelineError::new(format!(
            "verifier_output.json run_id {} does not match run {}",
            verifier.run_id, run_manifest.run_id
        )));
    }

    let mut blocking_issues = Vec::new();
    let mut warnings = Vec::new();
    let primary_report = load_required_report(
        &run_dir.join("primary_report.md"),
        "primary_report.md",
        &mut blocking_issues,
    );
    let verifier_report = load_required_report(
        &run_dir.join("verifier_report.md"),
        "verifier_report.md",
        &mut blocking_issues,
    );
    let schema_change_required = primary.schema_change_required || verifier.schema_change_required;

    let required_field_paths = required_field_paths(&definition, &run_manifest.expected_variants)?;
    blocking_issues.extend(validate_value_proposal(
        &definition,
        &run_manifest,
        &primary.value_proposal,
        schema_change_required,
    ));
    blocking_issues.extend(validate_field_evidence(
        &definition,
        &primary,
        &required_field_paths,
    ));
    blocking_issues.extend(validate_verifier_submission(
        &primary,
        &verifier,
        &required_field_paths,
    ));
    if primary.schema_change_required {
        blocking_issues.push(format!(
            "primary agent marked schema_change_required: {}",
            summarize_schema_change_notes(&primary.schema_change_notes, &primary.unresolved_issues)
        ));
    }
    if verifier.schema_change_required {
        let verifier_notes = if verifier.notes.trim().is_empty() {
            Vec::new()
        } else {
            vec![verifier.notes.clone()]
        };
        blocking_issues.push(format!(
            "verifier marked schema_change_required: {}",
            summarize_schema_change_notes(&verifier.schema_change_notes, &verifier_notes)
        ));
    }

    let accepted_sources =
        collect_accepted_sources(&definition, &primary, &verifier, &mut warnings)?;
    let status_decision = determine_status(
        &definition,
        &accepted_sources,
        &mut blocking_issues,
        current_entry.verification_status,
    );

    if verifier.status_recommendation != status_recommendation_for(status_decision) {
        warnings.push(format!(
            "verifier recommended {}, but review classified the run as {}",
            display_status_recommendation(&verifier.status_recommendation),
            status_decision
        ));
    }
    if primary.proposed_status != status_decision {
        warnings.push(format!(
            "primary agent proposed status {}, but review classified the run as {}",
            primary.proposed_status, status_decision
        ));
    }

    let metadata_patch = build_metadata_patch(status_decision, &accepted_sources);
    let review_markdown = render_review_markdown(
        &run_manifest,
        current_entry,
        &current_artifact.value,
        &primary.value_proposal,
        status_decision,
        &accepted_sources,
        &warnings,
        &blocking_issues,
        &primary_report,
        &verifier_report,
        &primary,
        &verifier,
    );

    let approved = if blocking_issues.is_empty() {
        match approval_override {
            Some(value) => value,
            None => prompt_for_approval(&run_manifest.run_id)?,
        }
    } else {
        false
    };
    let review_decision = ReviewDecision {
        schema_version: REVIEWED_ARTIFACT_SCHEMA_VERSION,
        run_id: run_manifest.run_id.clone(),
        approved,
        approved_by: approved.then_some(approved_by.unwrap_or_else(default_approver_name)),
        status_decision,
        blocking_issues: blocking_issues.clone(),
        warnings: warnings.clone(),
        accepted_sources: accepted_sources.clone(),
        final_value: primary.value_proposal.clone(),
        metadata_patch,
    };

    write_json(&run_dir.join("review.json"), &review_decision)?;
    write_text(&run_dir.join("review.md"), &review_markdown)?;
    update_run_status(&run_dir.join("run.json"), RunStatus::Reviewed)?;

    Ok(ReviewOutcome {
        run_id: run_manifest.run_id,
        run_dir: run_dir.clone(),
        review_path: run_dir.join("review.json"),
        review_markdown_path: run_dir.join("review.md"),
        approved,
        status_decision,
        warnings,
        blocking_issues,
    })
}

fn build_source_policy(definition: &PipelineDefinition, year: u32) -> SourcePolicyDocument {
    SourcePolicyDocument {
        schema_version: PIPELINE_DEFINITION_SCHEMA_VERSION,
        pipeline_name: definition.pipeline_name.clone(),
        category: definition.category.clone(),
        key: definition.key.clone(),
        year,
        required_primary_hosts: definition.required_primary_hosts.clone(),
        allowed_supporting_hosts: definition.allowed_supporting_hosts.clone(),
        allowed_secondary_hosts: definition.allowed_secondary_hosts.clone(),
        minimum_secondary_confirmations: definition.minimum_secondary_confirmations,
        require_exact_citation: definition.require_exact_citation,
        search_queries: definition.search_queries.clone(),
    }
}

fn execute_agent(
    engine_root: &Path,
    run_dir: &Path,
    config: &AgentInvocationConfig,
    role: AgentRole,
) -> Result<AgentExecutionLog, PipelineError> {
    let prompt_path = run_dir.join(match role {
        AgentRole::Primary => "primary_prompt.md",
        AgentRole::Verifier => "verifier_prompt.md",
    });
    let prompt = fs::read_to_string(&prompt_path)?;
    let stdout_log_path = run_dir.join(match role {
        AgentRole::Primary => "primary_agent.stdout.log",
        AgentRole::Verifier => "verifier_agent.stdout.log",
    });
    let stderr_log_path = run_dir.join(match role {
        AgentRole::Primary => "primary_agent.stderr.log",
        AgentRole::Verifier => "verifier_agent.stderr.log",
    });
    let workspace_root = workspace_root_for(engine_root)?;
    eprintln!(
        "[data-pipeline] running {} agent via {} ({})",
        role.as_str(),
        config.provider.as_str(),
        config.model
    );
    let mut command = match config.provider {
        AgentProvider::Claude => build_claude_command(
            &workspace_root,
            config.binary.as_deref(),
            &config.model,
            &prompt,
        ),
        AgentProvider::Codex => build_codex_command(
            &workspace_root,
            config.binary.as_deref(),
            &config.model,
            &prompt,
        ),
    };
    command.env("ENTROPYFA_RUN_DIR", run_dir);
    if let Some(run_id) = run_dir.file_name().and_then(|value| value.to_str()) {
        command.env("ENTROPYFA_RUN_ID", run_id);
    }
    command.env("ENTROPYFA_AGENT_ROLE", role.as_str());
    command.env("ENTROPYFA_AGENT_PROVIDER", config.provider.as_str());
    command.env(
        "ENTROPYFA_PRIMARY_OUTPUT_PATH",
        run_dir.join("primary_output.json"),
    );
    command.env(
        "ENTROPYFA_PRIMARY_REPORT_PATH",
        run_dir.join("primary_report.md"),
    );
    command.env(
        "ENTROPYFA_VERIFIER_OUTPUT_PATH",
        run_dir.join("verifier_output.json"),
    );
    command.env(
        "ENTROPYFA_VERIFIER_REPORT_PATH",
        run_dir.join("verifier_report.md"),
    );
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|error| {
        PipelineError::new(format!(
            "failed to launch {} agent via {}: {}",
            role.as_str(),
            config.provider.as_str(),
            error
        ))
    })?;
    let stdout = child.stdout.take().ok_or_else(|| {
        PipelineError::new(format!(
            "{} agent via {} did not provide stdout",
            role.as_str(),
            config.provider.as_str()
        ))
    })?;
    let stderr = child.stderr.take().ok_or_else(|| {
        PipelineError::new(format!(
            "{} agent via {} did not provide stderr",
            role.as_str(),
            config.provider.as_str()
        ))
    })?;
    let stdout_log_path_for_thread = stdout_log_path.clone();
    let stderr_log_path_for_thread = stderr_log_path.clone();
    let stdout_mode = match config.provider {
        AgentProvider::Claude => TerminalStreamMode::ClaudeJson,
        AgentProvider::Codex => TerminalStreamMode::CodexJson,
    };
    let stdout_thread = tee_stream(
        stdout,
        stdout_log_path_for_thread,
        false,
        stdout_mode,
        role,
        config.provider,
    );
    let stderr_thread = tee_stream(
        stderr,
        stderr_log_path_for_thread,
        true,
        TerminalStreamMode::Raw,
        role,
        config.provider,
    );

    let status = child.wait().map_err(|error| {
        PipelineError::new(format!(
            "{} agent via {} failed while waiting: {}",
            role.as_str(),
            config.provider.as_str(),
            error
        ))
    })?;
    join_tee_thread(stdout_thread, &stdout_log_path)?;
    join_tee_thread(stderr_thread, &stderr_log_path)?;
    eprintln!(
        "[data-pipeline] {} agent via {} completed with status {}",
        role.as_str(),
        config.provider.as_str(),
        status
    );

    if !status.success() {
        return Err(PipelineError::new(format!(
            "{} agent via {} failed with status {} (stdout: {}, stderr: {})",
            role.as_str(),
            config.provider.as_str(),
            status,
            stdout_log_path.display(),
            stderr_log_path.display()
        )));
    }

    wait_for_agent_outputs(run_dir, role);
    let missing = missing_agent_outputs(run_dir, role);
    if !missing.is_empty() {
        return Err(PipelineError::new(format!(
            "{} agent via {} did not produce required files: {}",
            role.as_str(),
            config.provider.as_str(),
            missing.join(", ")
        )));
    }

    Ok(AgentExecutionLog {
        provider: config.provider,
        model: config.model.clone(),
        stdout_log_path,
        stderr_log_path,
    })
}

fn tee_stream<R>(
    mut reader: R,
    log_path: PathBuf,
    is_stderr: bool,
    mode: TerminalStreamMode,
    role: AgentRole,
    provider: AgentProvider,
) -> thread::JoinHandle<Result<(), PipelineError>>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut log_file = File::create(&log_path)?;
        let mut buffer = [0_u8; 8192];
        let mut line_buffer = Vec::new();
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            log_file.write_all(&buffer[..bytes_read])?;
            line_buffer.extend_from_slice(&buffer[..bytes_read]);
            while let Some(newline_index) = line_buffer.iter().position(|byte| *byte == b'\n') {
                let line = line_buffer.drain(..=newline_index).collect::<Vec<_>>();
                emit_terminal_line(&line, is_stderr, mode, role, provider)?;
            }
        }
        if !line_buffer.is_empty() {
            emit_terminal_line(&line_buffer, is_stderr, mode, role, provider)?;
        }
        Ok(())
    })
}

fn emit_terminal_line(
    line: &[u8],
    is_stderr: bool,
    mode: TerminalStreamMode,
    role: AgentRole,
    provider: AgentProvider,
) -> Result<(), PipelineError> {
    let raw = String::from_utf8_lossy(line);
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    let rendered = match mode {
        TerminalStreamMode::Raw => trimmed.to_string(),
        TerminalStreamMode::ClaudeJson => {
            render_claude_terminal_line(trimmed).unwrap_or_else(|| trimmed.to_string())
        }
        TerminalStreamMode::CodexJson => {
            render_codex_terminal_line(trimmed).unwrap_or_else(|| trimmed.to_string())
        }
    };

    if rendered.is_empty() {
        return Ok(());
    }

    let mut stream: Box<dyn Write> = if is_stderr {
        Box::new(io::stderr())
    } else {
        Box::new(io::stdout())
    };
    writeln!(
        stream,
        "{}",
        if mode == TerminalStreamMode::Raw {
            rendered
        } else {
            format!("[{} {}] {}", role.as_str(), provider.as_str(), rendered)
        }
    )?;
    stream.flush()?;
    Ok(())
}

fn render_claude_terminal_line(line: &str) -> Option<String> {
    let value: Value = serde_json::from_str(line).ok()?;
    match value.get("type").and_then(Value::as_str) {
        Some("system") => {
            if value.get("subtype").and_then(Value::as_str) == Some("init") {
                Some("session initialized".to_string())
            } else {
                None
            }
        }
        Some("assistant") => summarize_claude_assistant_message(&value),
        Some("result") => Some("result received".to_string()),
        Some("error") => Some(format!("error: {}", truncate_for_terminal(line, 160))),
        Some("stream_event") | Some("user") => None,
        Some(other) => Some(format!("event: {other}")),
        None => Some(truncate_for_terminal(line, 160)),
    }
}

fn render_codex_terminal_line(line: &str) -> Option<String> {
    let value: Value = serde_json::from_str(line).ok()?;
    match value.get("type").and_then(Value::as_str) {
        Some("thread.started") => Some("session started".to_string()),
        Some("turn.started") | Some("turn.completed") => None,
        Some("item.started") => summarize_codex_item(value.get("item")?, true),
        Some("item.completed") => summarize_codex_item(value.get("item")?, false),
        Some("result") => Some("result received".to_string()),
        Some("error") => Some(format!("error: {}", truncate_for_terminal(line, 160))),
        Some(other) => Some(format!("event: {other}")),
        None => Some(truncate_for_terminal(line, 160)),
    }
}

fn summarize_codex_item(item: &Value, started: bool) -> Option<String> {
    match item.get("type").and_then(Value::as_str) {
        Some("agent_message") if !started => item
            .get("text")
            .and_then(Value::as_str)
            .map(|text| truncate_for_terminal(text, 160)),
        Some("command_execution") => summarize_codex_command(item, started),
        Some("web_search") if !started => summarize_codex_web_search(item),
        Some("file_change") if !started => summarize_codex_file_change(item),
        Some("reasoning") | Some("plan") => None,
        Some(other) if started => Some(format!("starting {other}")),
        Some(other) => Some(format!("completed {other}")),
        None => None,
    }
}

fn summarize_codex_command(item: &Value, started: bool) -> Option<String> {
    let command = item.get("command").and_then(Value::as_str)?;
    if started {
        return None;
    }

    let exit_code = item.get("exit_code").and_then(Value::as_i64);
    if exit_code == Some(0) {
        return None;
    }

    Some(format!(
        "shell failed{}: {}",
        exit_code
            .map(|code| format!(" ({code})"))
            .unwrap_or_default(),
        truncate_for_terminal(command, 140)
    ))
}

fn summarize_codex_web_search(item: &Value) -> Option<String> {
    let action = item.get("action")?;
    match action.get("type").and_then(Value::as_str) {
        Some("search") => action
            .get("query")
            .and_then(Value::as_str)
            .map(|query| format!("web search: {}", truncate_for_terminal(query, 120))),
        Some("open_page") => action
            .get("url")
            .or_else(|| item.get("query"))
            .and_then(Value::as_str)
            .map(|url| format!("open page: {}", truncate_for_terminal(url, 120))),
        Some("find_in_page") => {
            let pattern = action.get("pattern").and_then(Value::as_str).unwrap_or("");
            let url = action.get("url").and_then(Value::as_str).unwrap_or("");
            Some(format!(
                "find in page: {} @ {}",
                truncate_for_terminal(pattern, 40),
                truncate_for_terminal(url, 80)
            ))
        }
        Some(other) => Some(format!("web action: {other}")),
        None => None,
    }
}

fn summarize_codex_file_change(item: &Value) -> Option<String> {
    let changes = item.get("changes")?.as_array()?;
    let rendered = changes
        .iter()
        .filter_map(|change| {
            let path = change.get("path").and_then(Value::as_str)?;
            let kind = change
                .get("kind")
                .and_then(Value::as_str)
                .unwrap_or("change");
            Some(format!("{kind} {}", compact_path(path)))
        })
        .collect::<Vec<_>>();
    if rendered.is_empty() {
        None
    } else {
        Some(format!("files: {}", rendered.join(", ")))
    }
}

fn summarize_claude_assistant_message(value: &Value) -> Option<String> {
    let content = value.get("message")?.get("content")?.as_array()?;
    let parts = content
        .iter()
        .filter_map(|item| match item.get("type").and_then(Value::as_str) {
            Some("tool_use") => {
                let name = item.get("name").and_then(Value::as_str).unwrap_or("tool");
                let summary =
                    summarize_tool_use_input(name, item.get("input").unwrap_or(&Value::Null));
                Some(if summary.is_empty() {
                    name.to_string()
                } else {
                    format!("{name} {summary}")
                })
            }
            Some("text") => item
                .get("text")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|text| !text.is_empty())
                .map(|text| truncate_for_terminal(text, 160)),
            _ => None,
        })
        .collect::<Vec<_>>();

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" | "))
    }
}

fn summarize_tool_use_input(name: &str, input: &Value) -> String {
    match name {
        "Read" | "Write" | "Edit" | "NotebookEdit" => input
            .get("file_path")
            .or_else(|| input.get("path"))
            .and_then(Value::as_str)
            .map(compact_path)
            .unwrap_or_default(),
        "WebFetch" => input
            .get("url")
            .and_then(Value::as_str)
            .map(|url| truncate_for_terminal(url, 100))
            .unwrap_or_default(),
        "WebSearch" => input
            .get("query")
            .or_else(|| input.get("search_term"))
            .and_then(Value::as_str)
            .map(|query| format!("\"{}\"", truncate_for_terminal(query, 100)))
            .unwrap_or_default(),
        "Bash" => input
            .get("command")
            .or_else(|| input.get("cmd"))
            .and_then(Value::as_str)
            .map(|command| format!("`{}`", truncate_for_terminal(command, 100)))
            .unwrap_or_default(),
        _ => String::new(),
    }
}

fn compact_path(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(path)
        .to_string()
}

fn truncate_for_terminal(text: &str, max_chars: usize) -> String {
    let truncated = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        format!("{truncated}...")
    } else {
        truncated
    }
}

fn wait_for_agent_outputs(run_dir: &Path, role: AgentRole) {
    const OUTPUT_WAIT_ATTEMPTS: usize = 40;
    const OUTPUT_WAIT_INTERVAL: Duration = Duration::from_millis(100);

    for _ in 0..OUTPUT_WAIT_ATTEMPTS {
        if missing_agent_outputs(run_dir, role).is_empty() {
            return;
        }
        thread::sleep(OUTPUT_WAIT_INTERVAL);
    }
}

fn join_tee_thread(
    handle: thread::JoinHandle<Result<(), PipelineError>>,
    log_path: &Path,
) -> Result<(), PipelineError> {
    match handle.join() {
        Ok(result) => result,
        Err(_) => Err(PipelineError::new(format!(
            "streaming thread for {} panicked",
            log_path.display()
        ))),
    }
}

fn build_claude_command(
    workspace_root: &Path,
    binary: Option<&Path>,
    model: &str,
    prompt: &str,
) -> Command {
    let mut command = Command::new(binary.unwrap_or_else(|| Path::new("claude")));
    command
        .current_dir(workspace_root)
        .arg("-p")
        .arg("--model")
        .arg(model)
        .arg("--verbose")
        .arg("--output-format")
        .arg("stream-json")
        .arg("--include-partial-messages")
        .arg("--permission-mode")
        .arg("bypassPermissions")
        .arg(prompt);
    command
}

fn build_codex_command(
    workspace_root: &Path,
    binary: Option<&Path>,
    model: &str,
    prompt: &str,
) -> Command {
    let mut command = Command::new(binary.unwrap_or_else(|| Path::new("codex")));
    command
        .current_dir(workspace_root)
        .arg("exec")
        .arg("--full-auto")
        .arg("--json")
        .arg("-m")
        .arg(model)
        .arg("-C")
        .arg(workspace_root)
        .arg(prompt);
    command
}

fn workspace_root_for(engine_root: &Path) -> Result<PathBuf, PipelineError> {
    engine_root
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| PipelineError::new("engine root has no workspace parent"))
}

fn missing_agent_outputs(run_dir: &Path, role: AgentRole) -> Vec<String> {
    let required = match role {
        AgentRole::Primary => ["primary_output.json", "primary_report.md"].as_slice(),
        AgentRole::Verifier => ["verifier_output.json", "verifier_report.md"].as_slice(),
    };
    required
        .iter()
        .filter(|name| !run_dir.join(name).exists())
        .map(|name| (*name).to_string())
        .collect()
}

fn build_primary_template(run_manifest: &RunManifest, definition: &PipelineDefinition) -> Value {
    json!({
        "schema_version": REVIEWED_ARTIFACT_SCHEMA_VERSION,
        "run_id": run_manifest.run_id,
        "agent": {
            "tool": "<claude_code_or_codex>",
            "model": "<model_name>"
        },
        "sources": [],
        "proposed_status": "authoritative",
        "schema_change_required": false,
        "schema_change_notes": [],
        "value_proposal": build_value_proposal_skeleton(run_manifest, definition),
        "field_evidence": [],
        "unresolved_issues": []
    })
}

fn build_verifier_template(run_manifest: &RunManifest, definition: &PipelineDefinition) -> Value {
    let field_verdicts = required_field_paths(definition, &run_manifest.expected_variants)
        .unwrap_or_default()
        .into_iter()
        .map(|field_path| {
            json!({
                "field_path": field_path,
                "verdict": "confirm",
                "corrected_value": Value::Null,
                "source_ids": ["<source_id>"],
                "notes": ""
            })
        })
        .collect::<Vec<_>>();

    json!({
        "schema_version": REVIEWED_ARTIFACT_SCHEMA_VERSION,
        "run_id": run_manifest.run_id,
        "agent": {
            "tool": "<claude_code_or_codex>",
            "model": "<model_name>"
        },
        "source_verdicts": [],
        "field_verdicts": field_verdicts,
        "status_recommendation": "authoritative",
        "overall_verdict": "pass",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": ""
    })
}

fn build_value_proposal_skeleton(
    run_manifest: &RunManifest,
    definition: &PipelineDefinition,
) -> ValueProposal {
    let variants = run_manifest
        .expected_variants
        .iter()
        .map(|variant| ValueVariant {
            label: variant.label.clone(),
            params: variant.params.clone(),
            value: build_variant_value_skeleton(definition, variant),
        })
        .collect::<Vec<_>>();
    ValueProposal { variants }
}

fn build_variant_value_skeleton(
    definition: &PipelineDefinition,
    variant: &ExpectedVariant,
) -> Value {
    match definition.validation_profile {
        ValidationProfile::Brackets => json!([
            {
                "min": "<number>",
                "max": "<number_or_null>",
                "rate": "<number>"
            }
        ]),
        ValidationProfile::Irmaa => json!({
            "filing_status": variant
                .params
                .filing_status
                .clone()
                .unwrap_or_else(|| "<filing_status>".into()),
            "base_part_b_premium": "<number>",
            "brackets": [
                {
                    "magi_min": "<number>",
                    "magi_max": "<number_or_null>",
                    "monthly_surcharge": "<number>"
                }
            ]
        }),
        _ => Value::Null,
    }
}

fn render_primary_report_template(run_manifest: &RunManifest) -> String {
    format!(
        "# Primary Extraction Report\n\n\
## Summary\n\
- entry: `{}/{}`\n\
- year: `{}`\n\
- short conclusion:\n\n\
## Sources Reviewed\n\
- `[source_id]` Organization | URL\n\
  - source_class:\n\
  - locator:\n\
  - why it matters:\n\n\
## Schema Fit\n\
- schema_change_required: false\n\
- explain whether the official source fit the current JSON schema exactly:\n\n\
## Variant Notes\n\
{}\n\
## Open Questions\n\
- none\n",
        run_manifest.category,
        run_manifest.key,
        run_manifest.year,
        run_manifest
            .expected_variants
            .iter()
            .map(|variant| format!("### {}\n- notes:\n", variant.label))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn render_verifier_report_template(
    run_manifest: &RunManifest,
    definition: &PipelineDefinition,
) -> Result<String, PipelineError> {
    let field_paths = required_field_paths(definition, &run_manifest.expected_variants)?;
    Ok(format!(
        "# Verifier Review Report\n\n\
## Overall Assessment\n\
- entry: `{}/{}`\n\
- year: `{}`\n\
- overall_verdict: pass\n\
- status_recommendation: authoritative\n\
- schema_change_required: false\n\
- summary:\n\n\
## Source Review\n\
- `[source_id]` accept/reject | counts_toward_status: true/false\n\
  - reason:\n\n\
## Field Review\n\
{}\n\
## Disagreements Or Caveats\n\
- none\n",
        run_manifest.category,
        run_manifest.key,
        run_manifest.year,
        field_paths
            .into_iter()
            .map(|field_path| format!("- `{}`: confirm\n  - source_ids:\n  - notes:\n", field_path))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}

fn render_primary_prompt(
    run_dir: &Path,
    run_manifest: &RunManifest,
    definition: &PipelineDefinition,
) -> String {
    let field_path_examples = required_field_paths(definition, &run_manifest.expected_variants)
        .unwrap_or_default()
        .into_iter()
        .take(2)
        .collect::<Vec<_>>()
        .join(" and ");
    format!(
        "# Primary Extraction Agent\n\n\
Task: verify `{}/{}` for year `{}`.\n\n\
You may search broadly for official and credible corroborating sources, but only the hosts listed in \
`{}` count toward final review status.\n\n\
Read these files first:\n\
- `{}`\n\
- `{}`\n\
- `{}`\n\
- `{}`\n\n\
Write exactly two files:\n\
- `{}`\n\
- `{}`\n\n\
Instructions:\n\
1. Search broadly for candidate sources.\n\
2. Prefer official primary sources. Secondary sources are allowed only as corroborating evidence.\n\
3. Start from `primary_template.json`. Copy its structure exactly into `primary_output.json` and preserve every key name.\n\
4. Start from `primary_report_template.md`. Preserve the headings, but fill it with freeform evidence, tables, and narrative that help a human reviewer understand the source material.\n\
5. Do not invent aliases. Use `source_class`, not `type`. Use `published_at`, not `accessed`. Use `source_id`, not a URL in place of an id.\n\
6. Do not treat `current_value.json` as truth. It is only the previous embedded value for comparison.\n\
7. If the official source does not fit the current JSON schema cleanly, set `schema_change_required` to `true`, explain the mismatch in `schema_change_notes[]`, explain it again in `primary_report.md`, and do not invent new JSON keys.\n\
8. Populate `sources[]` with every source you relied on using this exact object shape:\n\
   `{{\"source_id\",\"url\",\"host\",\"organization\",\"source_class\",\"title\",\"published_at\",\"locator\",\"notes\"}}`.\n\
9. Choose stable source ids like `src_cms_1`, `src_ssa_1`, `src_kff_1`. They must be unique within the file.\n\
10. Update `value_proposal` with extracted values in the exact lookup shape already shown in the template.\n\
11. Populate `field_evidence[]` for every required field group using this exact object shape:\n\
   `{{\"field_path\",\"source_id\",\"locator\"}}`.\n\
12. `field_path` values must match the exact paths already implied by the template, for example `{}`.\n\
13. Every `field_evidence.source_id` must reference one of the ids you created in `sources[]`.\n\
14. Record any uncertainty in `unresolved_issues[]`.\n\
15. The task is incomplete until both output files exist on disk.\n\
16. If your environment does not expose a direct file-write tool, use shell commands to create the files at the exact paths above.\n\
17. After writing both files, run `ls -l` on each output path and do not stop until both commands succeed.\n\n\
Required enums and literals:\n\
- `proposed_status`: `authoritative`, `corroborated`, `derived`, or `placeholder`\n\
- `source_class`: `primary`, `supporting_official`, or `secondary`\n\n\
Do not edit any Rust source, metadata, snapshot, or other repo files.\n\
Do not write anything except `primary_output.json` and `primary_report.md`.\n\n\
Pipeline details:\n\
- pipeline: `{}`\n\
- expected variants: `{}`\n\
- search queries: `{}`\n",
        run_manifest.category,
        run_manifest.key,
        run_manifest.year,
        run_dir.join("source_policy.json").display(),
        run_dir.join("current_value.json").display(),
        run_dir.join("source_policy.json").display(),
        run_dir.join("primary_template.json").display(),
        run_dir.join("primary_report_template.md").display(),
        run_dir.join("primary_output.json").display(),
        run_dir.join("primary_report.md").display(),
        if field_path_examples.is_empty() {
            "`variants[...].value`".to_string()
        } else {
            field_path_examples
                .split(" and ")
                .map(|value| format!("`{value}`"))
                .collect::<Vec<_>>()
                .join(" and ")
        },
        definition.pipeline_name,
        run_manifest
            .expected_variants
            .iter()
            .map(|variant| variant.label.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        definition.search_queries.join(" | ")
    )
}

fn render_verifier_prompt(
    run_dir: &Path,
    run_manifest: &RunManifest,
    definition: &PipelineDefinition,
) -> String {
    format!(
        "# Verifier Agent\n\n\
Task: independently verify `{}/{}` for year `{}`.\n\n\
Read these files first:\n\
- `{}`\n\
- `{}`\n\
- `{}`\n\
- `{}`\n\
- `{}`\n\n\
Write exactly two files:\n\
- `{}`\n\
- `{}`\n\n\
Instructions:\n\
1. Independently inspect the cited sources and any replacement official sources you find.\n\
2. Start from `verifier_template.json`. Copy its structure exactly into `verifier_output.json` and preserve every key name.\n\
3. Start from `verifier_report_template.md`. Preserve the headings, but fill it with freeform verification notes, disagreements, and caveats for a human reviewer.\n\
4. Do not invent aliases or alternate shapes.\n\
5. If the source material does not fit the current JSON schema cleanly, set `schema_change_required` to `true`, explain the mismatch in `schema_change_notes[]`, explain it again in `verifier_report.md`, and do not invent new JSON keys.\n\
6. In `source_verdicts[]`, use this exact object shape:\n\
   `{{\"source_id\",\"verdict\",\"counts_toward_status\",\"reason\"}}`.\n\
7. `source_verdicts[].source_id` must match the exact `source_id` values from `primary_output.json`. Do not replace ids with URLs.\n\
8. In `field_verdicts[]`, use this exact object shape:\n\
   `{{\"field_path\",\"verdict\",\"corrected_value\",\"source_ids\",\"notes\"}}`.\n\
9. `field_path` values must match the exact required field paths from the template.\n\
10. Every id in `field_verdicts[].source_ids` must match a `source_id` from `primary_output.json`.\n\
11. Confirm, dispute, or mark uncertain each required field group in `field_verdicts[]`.\n\
12. Recommend `authoritative`, `corroborated`, or `needs_human_attention`.\n\
13. If anything is unresolved or inconsistent, set `overall_verdict` accordingly.\n\
14. The task is incomplete until both output files exist on disk.\n\
15. If your environment does not expose a direct file-write tool, use shell commands to create the files at the exact paths above.\n\
16. After writing both files, run `ls -l` on each output path and do not stop until both commands succeed.\n\n\
Required enums and literals:\n\
- `source_verdicts[].verdict`: `accept` or `reject`\n\
- `field_verdicts[].verdict`: `confirm`, `dispute`, or `uncertain`\n\
- `status_recommendation`: `authoritative`, `corroborated`, or `needs_human_attention`\n\
- `overall_verdict`: `pass`, `needs_human_attention`, or `reject`\n\n\
Do not edit any Rust source, metadata, snapshot, or other repo files.\n\
Do not write anything except `verifier_output.json` and `verifier_report.md`.\n\n\
Pipeline details:\n\
- pipeline: `{}`\n\
- required primary hosts: `{}`\n\
- allowed secondary hosts: `{}`\n",
        run_manifest.category,
        run_manifest.key,
        run_manifest.year,
        run_dir.join("source_policy.json").display(),
        run_dir.join("current_value.json").display(),
        run_dir.join("primary_output.json").display(),
        run_dir.join("verifier_template.json").display(),
        run_dir.join("verifier_report_template.md").display(),
        run_dir.join("verifier_output.json").display(),
        run_dir.join("verifier_report.md").display(),
        definition.pipeline_name,
        definition.required_primary_hosts.join(", "),
        definition.allowed_secondary_hosts.join(", ")
    )
}

fn validate_value_proposal(
    definition: &PipelineDefinition,
    run_manifest: &RunManifest,
    proposal: &ValueProposal,
    schema_change_required: bool,
) -> Vec<String> {
    let mut issues = Vec::new();
    let expected = run_manifest
        .expected_variants
        .iter()
        .map(|variant| (variant.label.as_str(), variant))
        .collect::<BTreeMap<_, _>>();
    let actual = proposal
        .variants
        .iter()
        .map(|variant| (variant.label.as_str(), variant))
        .collect::<BTreeMap<_, _>>();

    for label in expected.keys() {
        if !actual.contains_key(label) {
            issues.push(format!("value_proposal is missing variant {}", label));
        }
    }
    for label in actual.keys() {
        if !expected.contains_key(label) {
            issues.push(format!(
                "value_proposal includes unexpected variant {}",
                label
            ));
        }
    }

    for variant in &proposal.variants {
        let Some(expected_variant) = expected.get(variant.label.as_str()) else {
            continue;
        };
        if variant.params != expected_variant.params {
            issues.push(format!(
                "variant {} params do not match expected params",
                variant.label
            ));
        }
        if schema_change_required {
            continue;
        }
        issues.extend(validate_value(
            &format!("{}/{}", definition.category, definition.key),
            &variant.label,
            &definition.validation_profile,
            &variant.value,
        ));
        if let Some(expected_status) = &variant.params.filing_status {
            if let Some(value_obj) = variant.value.as_object() {
                match value_obj.get("filing_status").and_then(Value::as_str) {
                    Some(actual_status) if actual_status == expected_status => {}
                    Some(actual_status) => issues.push(format!(
                        "variant {} filing_status {} does not match expected {}",
                        variant.label, actual_status, expected_status
                    )),
                    None => issues.push(format!(
                        "variant {} is missing filing_status in value",
                        variant.label
                    )),
                }
            }
        }
    }

    issues
}

fn validate_field_evidence(
    definition: &PipelineDefinition,
    primary: &PrimarySubmission,
    required_field_paths: &[String],
) -> Vec<String> {
    let mut issues = Vec::new();
    let sources = primary
        .sources
        .iter()
        .map(|source| (source.source_id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let mut evidence_map: BTreeMap<&str, Vec<&FieldEvidence>> = BTreeMap::new();
    for evidence in &primary.field_evidence {
        evidence_map
            .entry(evidence.field_path.as_str())
            .or_default()
            .push(evidence);
        if !sources.contains_key(evidence.source_id.as_str()) {
            issues.push(format!(
                "field_evidence for {} references unknown source_id {}",
                evidence.field_path, evidence.source_id
            ));
        }
        if definition.require_exact_citation && evidence.locator.trim().is_empty() {
            issues.push(format!(
                "field_evidence for {} is missing a locator",
                evidence.field_path
            ));
        }
    }

    for field_path in required_field_paths {
        if !evidence_map.contains_key(field_path.as_str()) {
            issues.push(format!(
                "field_evidence is missing required field path {}",
                field_path
            ));
        }
    }

    issues
}

fn validate_verifier_submission(
    primary: &PrimarySubmission,
    verifier: &VerifierSubmission,
    required_field_paths: &[String],
) -> Vec<String> {
    let mut issues = Vec::new();
    let source_ids = primary
        .sources
        .iter()
        .map(|source| source.source_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut source_verdict_ids = BTreeSet::new();
    for verdict in &verifier.source_verdicts {
        if !source_ids.contains(verdict.source_id.as_str()) {
            issues.push(format!(
                "source_verdict references unknown source_id {}",
                verdict.source_id
            ));
        }
        if !source_verdict_ids.insert(verdict.source_id.as_str()) {
            issues.push(format!(
                "duplicate source_verdict for source_id {}",
                verdict.source_id
            ));
        }
    }

    let mut field_verdict_map = BTreeMap::new();
    for verdict in &verifier.field_verdicts {
        if field_verdict_map
            .insert(verdict.field_path.as_str(), verdict)
            .is_some()
        {
            issues.push(format!(
                "duplicate verifier field_verdict for {}",
                verdict.field_path
            ));
        }
        if verdict.source_ids.is_empty() {
            issues.push(format!(
                "field_verdict for {} must include at least one source_id",
                verdict.field_path
            ));
        }
        for source_id in &verdict.source_ids {
            if !source_ids.contains(source_id.as_str()) {
                issues.push(format!(
                    "field_verdict for {} references unknown source_id {}",
                    verdict.field_path, source_id
                ));
            }
        }
        if verdict.verdict != FieldVerdictDecision::Confirm {
            issues.push(format!(
                "verifier marked {} as {}",
                verdict.field_path,
                display_field_verdict(&verdict.verdict)
            ));
        }
    }

    for field_path in required_field_paths {
        if !field_verdict_map.contains_key(field_path.as_str()) {
            issues.push(format!(
                "verifier field_verdicts are missing required field path {}",
                field_path
            ));
        }
    }

    if verifier.overall_verdict != OverallVerdict::Pass {
        issues.push(format!(
            "verifier overall verdict is {}",
            display_overall_verdict(&verifier.overall_verdict)
        ));
    }

    issues
}

fn collect_accepted_sources(
    definition: &PipelineDefinition,
    primary: &PrimarySubmission,
    verifier: &VerifierSubmission,
    warnings: &mut Vec<String>,
) -> Result<Vec<AcceptedSource>, PipelineError> {
    let sources = primary
        .sources
        .iter()
        .map(|source| (source.source_id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let mut accepted = Vec::new();
    let mut seen_ids = BTreeSet::new();

    for verdict in &verifier.source_verdicts {
        if verdict.verdict != SourceVerdictDecision::Accept {
            continue;
        }
        let Some(source) = sources.get(verdict.source_id.as_str()) else {
            continue;
        };
        if !seen_ids.insert(source.source_id.as_str()) {
            continue;
        }

        let parsed = Url::parse(&source.url).map_err(|error| {
            PipelineError::new(format!(
                "source {} has invalid URL {}: {}",
                source.source_id, source.url, error
            ))
        })?;
        let Some(parsed_host) = parsed.host_str() else {
            return Err(PipelineError::new(format!(
                "source {} URL {} has no host",
                source.source_id, source.url
            )));
        };
        if !parsed_host.eq_ignore_ascii_case(&source.host) {
            return Err(PipelineError::new(format!(
                "source {} host {} does not match URL host {}",
                source.source_id, source.host, parsed_host
            )));
        }

        let policy_match = classify_host_match(definition, parsed_host);
        if policy_match == PolicyMatchKind::Unsupported {
            warnings.push(format!(
                "accepted source {} is on unsupported host {} and will not count toward status",
                source.source_id, parsed_host
            ));
        }
        if definition.require_exact_citation
            && source
                .locator
                .as_ref()
                .map(|locator| locator.trim().is_empty())
                .unwrap_or(true)
        {
            warnings.push(format!(
                "accepted source {} is missing a locator and will not count toward status",
                source.source_id
            ));
        }

        accepted.push(AcceptedSource {
            source_id: source.source_id.clone(),
            url: source.url.clone(),
            host: parsed_host.to_ascii_lowercase(),
            organization: source.organization.clone(),
            source_class: source.source_class.clone(),
            title: source.title.clone(),
            published_at: source.published_at.clone(),
            locator: source.locator.clone(),
            counts_toward_status: verdict.counts_toward_status
                && policy_match != PolicyMatchKind::Unsupported
                && (!definition.require_exact_citation
                    || source
                        .locator
                        .as_ref()
                        .map(|locator| !locator.trim().is_empty())
                        .unwrap_or(false)),
        });
    }

    Ok(accepted)
}

fn determine_status(
    definition: &PipelineDefinition,
    accepted_sources: &[AcceptedSource],
    blocking_issues: &mut Vec<String>,
    fallback_status: VerificationStatus,
) -> VerificationStatus {
    let mut secondary_organizations = BTreeSet::new();
    let mut has_primary = false;

    for source in accepted_sources {
        if !source.counts_toward_status {
            continue;
        }
        match classify_host_match(definition, &source.host) {
            PolicyMatchKind::Primary => has_primary = true,
            PolicyMatchKind::Secondary => {
                secondary_organizations.insert(source.organization.to_ascii_lowercase());
            }
            PolicyMatchKind::Supporting | PolicyMatchKind::Unsupported => {}
        }
    }

    if has_primary {
        VerificationStatus::Authoritative
    } else if secondary_organizations.len() >= definition.minimum_secondary_confirmations {
        VerificationStatus::Corroborated
    } else {
        blocking_issues.push(
            "accepted sources do not satisfy authoritative or corroborated status policy".into(),
        );
        fallback_status
    }
}

fn build_metadata_patch(
    status_decision: VerificationStatus,
    accepted_sources: &[AcceptedSource],
) -> MetadataPatch {
    let source_documents = accepted_sources
        .iter()
        .map(|source| SourceDocument {
            authority: source.organization.clone(),
            title: source.title.clone(),
            section: source.locator.clone(),
        })
        .collect::<Vec<_>>();
    let notes = match status_decision {
        VerificationStatus::Authoritative => None,
        VerificationStatus::Corroborated => {
            let organizations = accepted_sources
                .iter()
                .filter(|source| source.counts_toward_status)
                .map(|source| source.organization.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            Some(format!(
                "Reviewed via agent verification pipeline. No primary official source was accepted for this run; corroborated by {}.",
                organizations.join(", ")
            ))
        }
        VerificationStatus::Derived => {
            Some("Reviewed artifact remains derived and should not clear strict validation.".into())
        }
        VerificationStatus::Placeholder => Some(
            "Reviewed artifact remains placeholder and should not clear strict validation.".into(),
        ),
    };

    MetadataPatch {
        verification_status: status_decision,
        completeness: Completeness::Full,
        source_documents,
        notes,
    }
}

fn load_required_report(path: &Path, label: &str, blocking_issues: &mut Vec<String>) -> String {
    match fs::read_to_string(path) {
        Ok(content) if !content.trim().is_empty() => content,
        Ok(_) => {
            blocking_issues.push(format!("{label} is empty"));
            String::new()
        }
        Err(error) => {
            blocking_issues.push(format!("{label} is missing or unreadable: {}", error));
            String::new()
        }
    }
}

fn summarize_schema_change_notes(primary_notes: &[String], fallback_notes: &[String]) -> String {
    let combined = primary_notes
        .iter()
        .chain(fallback_notes.iter())
        .map(|note| note.trim())
        .filter(|note| !note.is_empty())
        .collect::<Vec<_>>();
    if combined.is_empty() {
        "no explanation provided".into()
    } else {
        combined.join(" | ")
    }
}

fn render_review_markdown(
    run_manifest: &RunManifest,
    current_entry: &RegistryEntry,
    current_value: &ValueProposal,
    proposed_value: &ValueProposal,
    status_decision: VerificationStatus,
    accepted_sources: &[AcceptedSource],
    warnings: &[String],
    blocking_issues: &[String],
    primary_report: &str,
    verifier_report: &str,
    primary: &PrimarySubmission,
    verifier: &VerifierSubmission,
) -> String {
    let mut lines = vec![
        format!("# Review for `{}`", run_manifest.run_id),
        String::new(),
        format!("- entry: `{}/{}`", run_manifest.category, run_manifest.key),
        format!("- year: `{}`", run_manifest.year),
        format!(
            "- current verification status: `{}`",
            current_entry.verification_status
        ),
        format!("- proposed verification status: `{}`", status_decision),
        format!(
            "- primary schema_change_required: `{}`",
            primary.schema_change_required
        ),
        format!(
            "- verifier schema_change_required: `{}`",
            verifier.schema_change_required
        ),
        String::new(),
        "## Accepted Sources".into(),
    ];
    if accepted_sources.is_empty() {
        lines.push("- none".into());
    } else {
        for source in accepted_sources {
            lines.push(format!(
                "- {} | {} | {}",
                source.organization, source.title, source.url
            ));
        }
    }
    lines.push(String::new());
    lines.push("## Diff From Current".into());
    for diff in variant_diffs(current_value, proposed_value) {
        lines.push(format!("- {}", diff));
    }
    lines.push(String::new());
    lines.push("## Primary Report".into());
    if primary_report.trim().is_empty() {
        lines.push("- missing".into());
    } else {
        lines.push(primary_report.trim().into());
    }
    lines.push(String::new());
    lines.push("## Verifier Report".into());
    if verifier_report.trim().is_empty() {
        lines.push("- missing".into());
    } else {
        lines.push(verifier_report.trim().into());
    }
    lines.push(String::new());
    lines.push("## Warnings".into());
    if warnings.is_empty() {
        lines.push("- none".into());
    } else {
        for warning in warnings {
            lines.push(format!("- {}", warning));
        }
    }
    lines.push(String::new());
    lines.push("## Blocking Issues".into());
    if blocking_issues.is_empty() {
        lines.push("- none".into());
    } else {
        for issue in blocking_issues {
            lines.push(format!("- {}", issue));
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

fn render_source(
    target_source_path: &Path,
    definition: &PipelineDefinition,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    match definition.generator_kind {
        GeneratorKind::IrmaaRust => render_irmaa_source(reviewed_artifact),
        GeneratorKind::TaxFederalBracketsRust => {
            render_tax_federal_brackets_source(target_source_path, reviewed_artifact)
        }
    }
}

fn render_irmaa_source(reviewed_artifact: &ReviewedArtifact) -> Result<String, PipelineError> {
    let mut variant_map = BTreeMap::new();
    let mut base_premium: Option<f64> = None;

    for variant in &reviewed_artifact.value.variants {
        let value_errors = validate_value(
            "insurance/irmaa_brackets",
            &variant.label,
            &ValidationProfile::Irmaa,
            &variant.value,
        );
        if !value_errors.is_empty() {
            return Err(PipelineError::new(format!(
                "reviewed IRMAA artifact has invalid variant {}: {}",
                variant.label,
                value_errors.join("; ")
            )));
        }
        let Some(obj) = variant.value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed IRMAA variant {} must be an object",
                variant.label
            )));
        };
        let Some(variant_base) = obj.get("base_part_b_premium").and_then(Value::as_f64) else {
            return Err(PipelineError::new(format!(
                "reviewed IRMAA variant {} is missing base_part_b_premium",
                variant.label
            )));
        };
        if let Some(current_base) = base_premium {
            if (current_base - variant_base).abs() > f64::EPSILON {
                return Err(PipelineError::new(
                    "all IRMAA variants must share the same base_part_b_premium",
                ));
            }
        } else {
            base_premium = Some(variant_base);
        }

        let Some(brackets) = obj.get("brackets").and_then(Value::as_array) else {
            return Err(PipelineError::new(format!(
                "reviewed IRMAA variant {} is missing brackets",
                variant.label
            )));
        };
        variant_map.insert(variant.label.clone(), brackets.clone());
    }

    let base_premium = base_premium.ok_or_else(|| {
        PipelineError::new("reviewed IRMAA artifact is missing base_part_b_premium")
    })?;

    let mut output = String::new();
    output.push_str("use crate::data::types::FilingStatus;\n\n");
    output.push_str("/// A single IRMAA bracket tier.\n");
    output.push_str("#[derive(Debug, Clone)]\n");
    output.push_str("pub struct IrmaaBracket {\n");
    output.push_str("    pub magi_min: f64,\n");
    output.push_str("    pub magi_max: Option<f64>,\n");
    output.push_str("    pub monthly_surcharge: f64,\n");
    output.push_str("}\n\n");
    output.push_str(&format!(
        "/// Generated from reviewed artifact for {} {}.\n",
        reviewed_artifact.category, reviewed_artifact.key
    ));
    output.push_str("pub fn brackets(status: FilingStatus) -> Vec<IrmaaBracket> {\n");
    output.push_str("    match status {\n");
    for label in [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ] {
        let Some(brackets) = variant_map.get(label) else {
            return Err(PipelineError::new(format!(
                "reviewed IRMAA artifact is missing variant {}",
                label
            )));
        };
        output.push_str(&format!("        {} => vec![\n", filing_status_arm(label)?));
        for bracket in brackets {
            let Some(obj) = bracket.as_object() else {
                return Err(PipelineError::new(format!(
                    "IRMAA bracket for {} is not an object",
                    label
                )));
            };
            let min = obj
                .get("magi_min")
                .and_then(Value::as_f64)
                .ok_or_else(|| PipelineError::new("IRMAA bracket missing magi_min"))?;
            let max = obj.get("magi_max").and_then(Value::as_f64);
            let surcharge = obj
                .get("monthly_surcharge")
                .and_then(Value::as_f64)
                .ok_or_else(|| PipelineError::new("IRMAA bracket missing monthly_surcharge"))?;
            output.push_str("            IrmaaBracket {\n");
            output.push_str(&format!("                magi_min: {},\n", format_f64(min)));
            match max {
                Some(value) => output.push_str(&format!(
                    "                magi_max: Some({}),\n",
                    format_f64(value)
                )),
                None => output.push_str("                magi_max: None,\n"),
            }
            output.push_str(&format!(
                "                monthly_surcharge: {},\n",
                format_f64(surcharge)
            ));
            output.push_str("            },\n");
        }
        output.push_str("        ],\n");
    }
    output.push_str("    }\n");
    output.push_str("}\n\n");
    output.push_str("pub fn base_part_b_premium() -> f64 {\n");
    output.push_str(&format!("    {}\n", format_f64(base_premium)));
    output.push_str("}\n\n");
    output.push_str("#[cfg(test)]\n");
    output.push_str("mod tests {\n");
    output.push_str("    use super::*;\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn single_irmaa_tiers() {\n");
    output.push_str("        let b = brackets(FilingStatus::Single);\n");
    output.push_str(&format!(
        "        assert_eq!(b.len(), {});\n",
        variant_map.get("single").map(Vec::len).unwrap_or(0)
    ));
    output.push_str("        assert_eq!(b[0].monthly_surcharge, 0.0);\n");
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn mfj_irmaa_tiers() {\n");
    output.push_str("        let b = brackets(FilingStatus::MarriedFilingJointly);\n");
    output.push_str(&format!(
        "        assert_eq!(b.len(), {});\n",
        variant_map
            .get("married_filing_jointly")
            .map(Vec::len)
            .unwrap_or(0)
    ));
    output.push_str("        assert!(b[0].magi_max.is_some());\n");
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn mfs_irmaa_only_three_tiers() {\n");
    output.push_str("        let b = brackets(FilingStatus::MarriedFilingSeparately);\n");
    output.push_str(&format!(
        "        assert_eq!(b.len(), {});\n",
        variant_map
            .get("married_filing_separately")
            .map(Vec::len)
            .unwrap_or(0)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn base_premium() {\n");
    output.push_str(&format!(
        "        assert_eq!(base_part_b_premium(), {});\n",
        format_f64(base_premium)
    ));
    output.push_str("    }\n");
    output.push_str("}\n");

    Ok(output)
}

fn render_tax_federal_brackets_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let mut variant_map = BTreeMap::new();

    for variant in &reviewed_artifact.value.variants {
        let value_errors = validate_value(
            "tax/federal_income_tax_brackets",
            &variant.label,
            &ValidationProfile::Brackets,
            &variant.value,
        );
        if !value_errors.is_empty() {
            return Err(PipelineError::new(format!(
                "reviewed tax brackets artifact has invalid variant {}: {}",
                variant.label,
                value_errors.join("; ")
            )));
        }
        let Some(brackets) = variant.value.as_array() else {
            return Err(PipelineError::new(format!(
                "reviewed tax brackets variant {} must be an array",
                variant.label
            )));
        };
        variant_map.insert(variant.label.clone(), brackets.clone());
    }

    let section = render_tax_federal_brackets_section(&variant_map)?;
    replace_source_section(
        &existing,
        "// ---------------------------------------------------------------------------\n// Federal income tax brackets",
        "// ---------------------------------------------------------------------------\n// Standard deductions",
        &section,
    )
}

fn render_tax_federal_brackets_section(
    variant_map: &BTreeMap<String, Vec<Value>>,
) -> Result<String, PipelineError> {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n\
// Federal income tax brackets (2026, reviewed artifact)\n\
// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn brackets(status: FilingStatus) -> Vec<TaxBracket> {\n");
    output.push_str("    match status {\n");
    for label in [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ] {
        let Some(brackets) = variant_map.get(label) else {
            return Err(PipelineError::new(format!(
                "reviewed tax brackets artifact is missing variant {}",
                label
            )));
        };
        output.push_str(&format!("        {} => vec![\n", filing_status_arm(label)?));
        for bracket in brackets {
            let Some(obj) = bracket.as_object() else {
                return Err(PipelineError::new(format!(
                    "tax bracket for {} is not an object",
                    label
                )));
            };
            let min = obj
                .get("min")
                .and_then(Value::as_f64)
                .ok_or_else(|| PipelineError::new("tax bracket missing min"))?;
            let max = obj.get("max").and_then(Value::as_f64);
            let rate = obj
                .get("rate")
                .and_then(Value::as_f64)
                .ok_or_else(|| PipelineError::new("tax bracket missing rate"))?;
            output.push_str("            TaxBracket {\n");
            output.push_str(&format!("                min: {},\n", format_f64(min)));
            match max {
                Some(value) => output.push_str(&format!(
                    "                max: Some({}),\n",
                    format_f64(value)
                )),
                None => output.push_str("                max: None,\n"),
            }
            output.push_str(&format!("                rate: {},\n", format_f64(rate)));
            output.push_str("            },\n");
        }
        output.push_str("        ],\n");
    }
    output.push_str("    }\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn replace_source_section(
    existing: &str,
    start_marker: &str,
    end_marker: &str,
    replacement: &str,
) -> Result<String, PipelineError> {
    let start = existing.find(start_marker).ok_or_else(|| {
        PipelineError::new(format!(
            "source file is missing start marker {}",
            start_marker
        ))
    })?;
    let end = existing.find(end_marker).ok_or_else(|| {
        PipelineError::new(format!("source file is missing end marker {}", end_marker))
    })?;
    if end <= start {
        return Err(PipelineError::new("source markers are out of order"));
    }

    let mut output = String::new();
    output.push_str(&existing[..start]);
    output.push_str(replacement);
    output.push_str(&existing[end..]);
    Ok(output)
}

fn update_registry_entry(
    metadata_path: &Path,
    category: &str,
    key: &str,
    metadata_patch: &MetadataPatch,
) -> Result<(), PipelineError> {
    let mut registry: RegistryDocument = load_registry(metadata_path)?;
    let entry = registry
        .entries
        .iter_mut()
        .find(|entry| entry.category == category && entry.key == key)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "metadata entry not found for {}/{} in {}",
                category,
                key,
                metadata_path.display()
            ))
        })?;

    entry.verification_status = metadata_patch.verification_status;
    entry.completeness = metadata_patch.completeness;
    entry.source_documents = metadata_patch.source_documents.clone();
    entry.notes = metadata_patch.notes.clone();
    write_registry(metadata_path, &registry)
}

fn update_run_status(run_manifest_path: &Path, status: RunStatus) -> Result<(), PipelineError> {
    let mut run_manifest: RunManifest = load_json(run_manifest_path)?;
    run_manifest.status = status;
    write_json(run_manifest_path, &run_manifest)
}

fn build_current_value(
    year: u32,
    category: &str,
    key: &str,
    expected_variants: &[ExpectedVariant],
) -> Result<ValueProposal, PipelineError> {
    let variants = lookup_entry_variants(category, key, year)?;
    let mut lookup_map = variants
        .into_iter()
        .map(|variant| (variant.label.clone(), variant))
        .collect::<BTreeMap<_, _>>();
    let mut output = Vec::new();

    for expected in expected_variants {
        let Some(variant) = lookup_map.remove(expected.label.as_str()) else {
            return Err(PipelineError::new(format!(
                "current lookup is missing expected variant {}",
                expected.label
            )));
        };
        output.push(ValueVariant {
            label: variant.label,
            params: variant.params,
            value: variant.value,
        });
    }

    Ok(ValueProposal { variants: output })
}

fn ensure_year_supported(definition: &PipelineDefinition, year: u32) -> Result<(), PipelineError> {
    if !definition.supported_years.contains(&year) {
        return Err(PipelineError::new(format!(
            "pipeline {} does not support year {}",
            definition.pipeline_name, year
        )));
    }
    Ok(())
}

fn ensure_expected_variants(
    category: &str,
    key: &str,
    configured: &[ExpectedVariant],
) -> Result<Vec<ExpectedVariant>, PipelineError> {
    let live_variants = lookup_entry_variants(category, key, 2026)?
        .into_iter()
        .map(|variant| ExpectedVariant {
            label: variant.label,
            params: variant.params,
        })
        .collect::<Vec<_>>();
    if configured != live_variants {
        return Err(PipelineError::new(format!(
            "pipeline definition expected_variants do not match live coverage for {}/{}",
            category, key
        )));
    }
    Ok(configured.to_vec())
}

fn required_field_paths(
    definition: &PipelineDefinition,
    variants: &[ExpectedVariant],
) -> Result<Vec<String>, PipelineError> {
    match definition.validation_profile {
        ValidationProfile::Brackets => Ok(variants
            .iter()
            .map(|variant| format!("variants[{}].value", variant.label))
            .collect()),
        ValidationProfile::Irmaa => {
            let mut paths = Vec::new();
            for variant in variants {
                paths.push(format!(
                    "variants[{}].value.base_part_b_premium",
                    variant.label
                ));
                paths.push(format!("variants[{}].value.brackets", variant.label));
            }
            Ok(paths)
        }
        _ => Err(PipelineError::new(format!(
            "prompt-pack workflow is not implemented for validation profile {:?}",
            definition.validation_profile
        ))),
    }
}

fn variant_diffs(current_value: &ValueProposal, proposed_value: &ValueProposal) -> Vec<String> {
    let current = current_value
        .variants
        .iter()
        .map(|variant| (variant.label.as_str(), variant))
        .collect::<BTreeMap<_, _>>();
    let proposed = proposed_value
        .variants
        .iter()
        .map(|variant| (variant.label.as_str(), variant))
        .collect::<BTreeMap<_, _>>();
    let mut diffs = Vec::new();

    for label in current.keys().chain(proposed.keys()) {
        let Some(current_variant) = current.get(label) else {
            diffs.push(format!("{}: added", label));
            continue;
        };
        let Some(proposed_variant) = proposed.get(label) else {
            diffs.push(format!("{}: removed", label));
            continue;
        };
        let current_checksum = checksum_value(&current_variant.value);
        let proposed_checksum = checksum_value(&proposed_variant.value);
        if current_checksum == proposed_checksum {
            diffs.push(format!("{}: unchanged", label));
        } else {
            diffs.push(format!(
                "{}: changed ({} -> {})",
                label, current_checksum, proposed_checksum
            ));
        }
    }

    diffs
}

fn checksum_value(value: &Value) -> String {
    let canonical = canonicalize(value);
    let bytes = serde_json::to_vec(&canonical).unwrap_or_default();
    fnv1a64(&bytes)
}

fn metadata_path_for(engine_root: &Path, year: u32) -> PathBuf {
    engine_root
        .join("data_registry")
        .join(year.to_string())
        .join("metadata.json")
}

fn snapshot_path_for(engine_root: &Path, year: u32) -> PathBuf {
    engine_root
        .join("data_registry")
        .join(year.to_string())
        .join("snapshot.json")
}

fn reviewed_root_for(engine_root: &Path, year: u32) -> PathBuf {
    engine_root
        .join("data_registry")
        .join(year.to_string())
        .join("reviewed")
}

fn runs_root_for(engine_root: &Path) -> PathBuf {
    engine_root.join("data_registry").join("runs")
}

fn pipeline_definition_path_for(engine_root: &Path, category: &str, key: &str) -> PathBuf {
    engine_root
        .join("data_registry")
        .join("pipelines")
        .join(category)
        .join(format!("{}.json", key))
}

fn load_pipeline_definition_at(
    engine_root: &Path,
    category: &str,
    key: &str,
) -> Result<PipelineDefinition, PipelineError> {
    let path = pipeline_definition_path_for(engine_root, category, key);
    let definition: PipelineDefinition = load_json(&path)?;
    if definition.schema_version != PIPELINE_DEFINITION_SCHEMA_VERSION {
        return Err(PipelineError::new(format!(
            "unsupported pipeline definition version {} in {}",
            definition.schema_version,
            path.display()
        )));
    }
    Ok(definition)
}

fn load_pipeline_definitions_for_year(
    engine_root: &Path,
    year: u32,
) -> Result<Vec<PipelineDefinition>, PipelineError> {
    let mut definitions = Vec::new();
    collect_pipeline_definitions(
        &engine_root.join("data_registry").join("pipelines"),
        &mut definitions,
    )?;
    definitions.retain(|definition| match definition.year_strategy {
        YearStrategy::Fixed => definition.supported_years.contains(&year),
    });
    definitions.sort_by(|a, b| (&a.category, &a.key).cmp(&(&b.category, &b.key)));
    Ok(definitions)
}

fn collect_pipeline_definitions(
    dir: &Path,
    definitions: &mut Vec<PipelineDefinition>,
) -> Result<(), PipelineError> {
    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_pipeline_definitions(&path, definitions)?;
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let definition: PipelineDefinition = load_json(&path)?;
        if definition.schema_version != PIPELINE_DEFINITION_SCHEMA_VERSION {
            return Err(PipelineError::new(format!(
                "unsupported pipeline definition version {} in {}",
                definition.schema_version,
                path.display()
            )));
        }
        definitions.push(definition);
    }

    Ok(())
}

fn latest_run_summary_for(
    engine_root: &Path,
    year: u32,
    category: &str,
    key: &str,
) -> Result<Option<PipelineRunSummary>, PipelineError> {
    let runs_dir = runs_root_for(engine_root)
        .join(year.to_string())
        .join(category)
        .join(key);
    if !runs_dir.exists() {
        return Ok(None);
    }

    let mut best: Option<(u128, PipelineRunSummary)> = None;
    for entry in fs::read_dir(runs_dir)? {
        let path = entry?.path();
        if !path.is_dir() || !path.join("run.json").exists() {
            continue;
        }

        let manifest: RunManifest = load_json(&path.join("run.json"))?;
        let approved = if path.join("review.json").exists() {
            Some(load_json::<ReviewDecision>(&path.join("review.json"))?.approved)
        } else {
            None
        };
        let sort_key = parse_run_timestamp_ms(&manifest.run_id)
            .or_else(|| modified_timestamp_ms(&path.join("run.json")))
            .unwrap_or_default();
        let created_at = format_timestamp_ms(sort_key);
        let summary = PipelineRunSummary {
            run_id: manifest.run_id,
            status: manifest.status,
            approved,
            created_at,
        };

        match &best {
            Some((current_key, _)) if *current_key >= sort_key => {}
            _ => best = Some((sort_key, summary)),
        }
    }

    Ok(best.map(|(_, summary)| summary))
}

fn parse_run_timestamp_ms(run_id: &str) -> Option<u128> {
    run_id.rsplit('-').next()?.parse::<u128>().ok()
}

fn modified_timestamp_ms(path: &Path) -> Option<u128> {
    path.metadata()
        .ok()?
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis())
}

fn format_timestamp_ms(timestamp_ms: u128) -> Option<String> {
    let nanos = (timestamp_ms as i128).checked_mul(1_000_000)?;
    let datetime = OffsetDateTime::from_unix_timestamp_nanos(nanos).ok()?;
    datetime.format(&Rfc3339).ok()
}

fn resolve_run_dir(engine_root: &Path, run_ref: &str) -> Result<PathBuf, PipelineError> {
    let candidate = PathBuf::from(run_ref);
    if candidate.join("run.json").exists() {
        return Ok(candidate);
    }

    find_run_dir(&runs_root_for(engine_root), run_ref)?.ok_or_else(|| {
        PipelineError::new(format!(
            "run {} was not found under {}",
            run_ref,
            runs_root_for(engine_root).display()
        ))
    })
}

fn find_run_dir(root: &Path, run_id: &str) -> Result<Option<PathBuf>, PipelineError> {
    if !root.exists() {
        return Ok(None);
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            if path.file_name().and_then(|value| value.to_str()) == Some(run_id)
                && path.join("run.json").exists()
            {
                return Ok(Some(path));
            }
            if let Some(found) = find_run_dir(&path, run_id)? {
                return Ok(Some(found));
            }
        }
    }
    Ok(None)
}

fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, PipelineError> {
    let contents = fs::read_to_string(path)?;
    let value = serde_json::from_str(&contents)?;
    Ok(value)
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), PipelineError> {
    let json = serde_json::to_string_pretty(value)?;
    write_text(path, &format!("{}\n", json))
}

fn write_json_value(path: &Path, value: &Value) -> Result<(), PipelineError> {
    let json = serde_json::to_string_pretty(value)?;
    write_text(path, &format!("{}\n", json))
}

fn write_text(path: &Path, contents: &str) -> Result<(), PipelineError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}

fn find_registry_entry<'a>(
    registry: &'a RegistryDocument,
    category: &str,
    key: &str,
) -> Result<&'a RegistryEntry, PipelineError> {
    registry
        .entries
        .iter()
        .find(|entry| entry.category == category && entry.key == key)
        .ok_or_else(|| {
            PipelineError::new(format!("metadata entry not found for {}/{}", category, key))
        })
}

fn classify_host_match(definition: &PipelineDefinition, host: &str) -> PolicyMatchKind {
    if matches_host_patterns(host, &definition.required_primary_hosts) {
        PolicyMatchKind::Primary
    } else if matches_host_patterns(host, &definition.allowed_secondary_hosts) {
        PolicyMatchKind::Secondary
    } else if matches_host_patterns(host, &definition.allowed_supporting_hosts) {
        PolicyMatchKind::Supporting
    } else {
        PolicyMatchKind::Unsupported
    }
}

fn matches_host_patterns(host: &str, patterns: &[String]) -> bool {
    patterns
        .iter()
        .any(|pattern| host_matches_pattern(host, pattern))
}

fn host_matches_pattern(host: &str, pattern: &str) -> bool {
    let host = host.to_ascii_lowercase();
    let pattern = pattern.to_ascii_lowercase();
    if let Some(suffix) = pattern.strip_prefix("*.") {
        host == suffix || host.ends_with(&format!(".{}", suffix))
    } else {
        host == pattern
    }
}

fn filing_status_arm(label: &str) -> Result<&'static str, PipelineError> {
    match label {
        "single" => Ok("FilingStatus::Single"),
        "married_filing_jointly" => Ok("FilingStatus::MarriedFilingJointly"),
        "married_filing_separately" => Ok("FilingStatus::MarriedFilingSeparately"),
        "head_of_household" => Ok("FilingStatus::HeadOfHousehold"),
        "qualifying_surviving_spouse" => Ok("FilingStatus::QualifyingSurvivingSpouse"),
        _ => Err(PipelineError::new(format!(
            "unsupported filing status label {}",
            label
        ))),
    }
}

fn format_f64(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.1}")
    } else {
        let mut formatted = format!("{value:.4}");
        while formatted.contains('.') && formatted.ends_with('0') {
            formatted.pop();
        }
        if formatted.ends_with('.') {
            formatted.push('0');
        }
        formatted
    }
}

impl AgentProvider {
    fn as_str(self) -> &'static str {
        match self {
            Self::Claude => "claude",
            Self::Codex => "codex",
        }
    }
}

impl AgentRole {
    fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Verifier => "verifier",
        }
    }
}

fn status_recommendation_for(status: VerificationStatus) -> StatusRecommendation {
    match status {
        VerificationStatus::Authoritative => StatusRecommendation::Authoritative,
        VerificationStatus::Corroborated => StatusRecommendation::Corroborated,
        VerificationStatus::Derived | VerificationStatus::Placeholder => {
            StatusRecommendation::NeedsHumanAttention
        }
    }
}

fn default_approver() -> Option<String> {
    env::var("USER").ok()
}

fn default_approver_name() -> String {
    default_approver().unwrap_or_else(|| "unknown".into())
}

fn prompt_for_approval(run_id: &str) -> Result<bool, PipelineError> {
    print!("Approve review for {}? [y/N] ", run_id);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(matches!(input.trim(), "y" | "Y" | "yes" | "YES"))
}

fn generate_run_id(year: u32, category: &str, key: &str) -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("{year}-{category}-{key}-{millis}")
}

fn display_field_verdict(verdict: &FieldVerdictDecision) -> &'static str {
    match verdict {
        FieldVerdictDecision::Confirm => "confirm",
        FieldVerdictDecision::Dispute => "dispute",
        FieldVerdictDecision::Uncertain => "uncertain",
    }
}

fn display_status_recommendation(recommendation: &StatusRecommendation) -> &'static str {
    match recommendation {
        StatusRecommendation::Authoritative => "authoritative",
        StatusRecommendation::Corroborated => "corroborated",
        StatusRecommendation::NeedsHumanAttention => "needs_human_attention",
    }
}

fn display_overall_verdict(verdict: &OverallVerdict) -> &'static str {
    match verdict {
        OverallVerdict::Pass => "pass",
        OverallVerdict::NeedsHumanAttention => "needs_human_attention",
        OverallVerdict::Reject => "reject",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_host_patterns_match_subdomains() {
        assert!(host_matches_pattern("www.cms.gov", "*.cms.gov"));
        assert!(host_matches_pattern("cms.gov", "*.cms.gov"));
        assert!(!host_matches_pattern("example.com", "*.cms.gov"));
    }

    #[test]
    fn status_recommendation_tracks_verification_status() {
        assert_eq!(
            status_recommendation_for(VerificationStatus::Authoritative),
            StatusRecommendation::Authoritative
        );
        assert_eq!(
            status_recommendation_for(VerificationStatus::Corroborated),
            StatusRecommendation::Corroborated
        );
    }
}
