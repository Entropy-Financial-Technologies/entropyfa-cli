use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::de::{DeserializeOwned, Error as DeError};
use serde::{Deserialize, Deserializer, Serialize};
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
    #[serde(default)]
    pub contract_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum YearStrategy {
    Fixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum GeneratorKind {
    IrmaaRust,
    InsuranceMedicareBasePremiumsRust,
    SocialSecurityFullRetirementAgeRust,
    TaxFederalBracketsRust,
    TaxFederalStandardDeductionsRust,
    TaxFederalCapitalGainsBracketsRust,
    TaxFederalCapitalLossLimitRust,
    TaxFederalNiitRust,
    TaxFederalPayrollRust,
    TaxFederalSaltDeductionRust,
    TaxFederalQbiRust,
    TaxFederalEstateExemptionRust,
    TaxFederalEstateApplicableCreditRust,
    TaxFederalEstateBracketsRust,
    SocialSecurityRetirementEarningsTestRust,
    SocialSecurityTaxationRust,
    RetirementDistributionRulesRust,
    RetirementUniformLifetimeRust,
    RetirementSingleLifeRust,
    RetirementJointLifeRust,
    #[serde(rename = "pension_mortality_417e_rust")]
    PensionMortality417eRust,
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
    #[serde(default)]
    pub contract_notes: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueProposal {
    pub variants: Vec<ValueVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReferencePackPrimer {
    #[serde(default)]
    pub what_this_is: String,
    #[serde(default)]
    pub lookup_parameters: Vec<String>,
    #[serde(default)]
    pub interpretation_notes: Vec<String>,
    #[serde(default)]
    pub does_not_include: Vec<String>,
    #[serde(default)]
    pub caveats: Vec<String>,
    #[serde(default)]
    pub typical_uses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimarySubmission {
    pub schema_version: u32,
    pub run_id: String,
    #[serde(default)]
    pub agent: Option<AgentDescriptor>,
    pub sources: Vec<SourceRecord>,
    pub proposed_status: VerificationStatus,
    #[serde(default)]
    pub schema_change_required: bool,
    #[serde(default)]
    pub schema_change_notes: Vec<String>,
    #[serde(default)]
    pub reference_pack_primer: Option<ReferencePackPrimer>,
    pub value_proposal: ValueProposal,
    pub field_evidence: Vec<FieldEvidence>,
    #[serde(default, deserialize_with = "deserialize_unresolved_issues")]
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
    #[serde(default)]
    pub issue_type: Option<ReviewIssueType>,
    #[serde(default)]
    pub auto_resolvable: Option<bool>,
    #[serde(default)]
    pub repair_guidance: String,
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
    #[serde(default)]
    pub issue_type: Option<ReviewIssueType>,
    #[serde(default)]
    pub auto_resolvable: Option<bool>,
    #[serde(default)]
    pub repair_guidance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimerSectionVerdict {
    pub verdict: FieldVerdictDecision,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub issue_type: Option<ReviewIssueType>,
    #[serde(default)]
    pub auto_resolvable: Option<bool>,
    #[serde(default)]
    pub repair_guidance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrimerVerdicts {
    #[serde(default)]
    pub what_this_is: Option<PrimerSectionVerdict>,
    #[serde(default)]
    pub lookup_parameters: Option<PrimerSectionVerdict>,
    #[serde(default)]
    pub interpretation_notes: Option<PrimerSectionVerdict>,
    #[serde(default)]
    pub does_not_include: Option<PrimerSectionVerdict>,
    #[serde(default)]
    pub caveats: Option<PrimerSectionVerdict>,
    #[serde(default)]
    pub typical_uses: Option<PrimerSectionVerdict>,
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
    #[serde(default)]
    pub agent: Option<AgentDescriptor>,
    pub source_verdicts: Vec<SourceVerdict>,
    pub field_verdicts: Vec<FieldVerdict>,
    #[serde(default)]
    pub primer_verdicts: PrimerVerdicts,
    pub status_recommendation: StatusRecommendation,
    pub overall_verdict: OverallVerdict,
    #[serde(default)]
    pub schema_change_required: bool,
    #[serde(default)]
    pub schema_change_notes: Vec<String>,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewIssueType {
    PrimerScopeOnly,
    OverbroadPrimer,
    ValueDisputeCase,
    PrimerScopeOverstatement,
    PrimerFactualImprecision,
    PrimerMissingRequiredSection,
    LookupParametersMissing,
    InterpretationNotesMissing,
    DoesNotIncludeMissing,
    CaveatsMissing,
    ValueConfirmed,
    ValueMismatch,
    CitationLocatorInexact,
    SourcePolicyFailure,
    SchemaMismatch,
    UnsafeRepairMutatedValue,
    Other,
}

impl ReviewIssueType {
    fn as_str(&self) -> &'static str {
        match self {
            ReviewIssueType::PrimerScopeOnly => "primer_scope_only",
            ReviewIssueType::OverbroadPrimer => "overbroad_primer",
            ReviewIssueType::ValueDisputeCase => "value_dispute_case",
            ReviewIssueType::PrimerScopeOverstatement => "primer_scope_overstatement",
            ReviewIssueType::PrimerFactualImprecision => "primer_factual_imprecision",
            ReviewIssueType::PrimerMissingRequiredSection => "primer_missing_required_section",
            ReviewIssueType::LookupParametersMissing => "lookup_parameters_missing",
            ReviewIssueType::InterpretationNotesMissing => "interpretation_notes_missing",
            ReviewIssueType::DoesNotIncludeMissing => "does_not_include_missing",
            ReviewIssueType::CaveatsMissing => "caveats_missing",
            ReviewIssueType::ValueConfirmed => "value_confirmed",
            ReviewIssueType::ValueMismatch => "value_mismatch",
            ReviewIssueType::CitationLocatorInexact => "citation_locator_inexact",
            ReviewIssueType::SourcePolicyFailure => "source_policy_failure",
            ReviewIssueType::SchemaMismatch => "schema_mismatch",
            ReviewIssueType::UnsafeRepairMutatedValue => "unsafe_repair_mutated_value",
            ReviewIssueType::Other => "other",
        }
    }

    fn from_wire_value(value: &str) -> Self {
        match value {
            "primer_scope_only" => ReviewIssueType::PrimerScopeOnly,
            "overbroad_primer" => ReviewIssueType::OverbroadPrimer,
            "value_dispute_case" => ReviewIssueType::ValueDisputeCase,
            "primer_scope_overstatement" => ReviewIssueType::PrimerScopeOverstatement,
            "primer_factual_imprecision" => ReviewIssueType::PrimerFactualImprecision,
            "primer_missing_required_section" => ReviewIssueType::PrimerMissingRequiredSection,
            "lookup_parameters_missing" => ReviewIssueType::LookupParametersMissing,
            "interpretation_notes_missing" => ReviewIssueType::InterpretationNotesMissing,
            "does_not_include_missing" => ReviewIssueType::DoesNotIncludeMissing,
            "caveats_missing" => ReviewIssueType::CaveatsMissing,
            "value_confirmed" => ReviewIssueType::ValueConfirmed,
            "value_mismatch" => ReviewIssueType::ValueMismatch,
            "citation_locator_inexact" => ReviewIssueType::CitationLocatorInexact,
            "source_policy_failure" => ReviewIssueType::SourcePolicyFailure,
            "schema_mismatch" => ReviewIssueType::SchemaMismatch,
            "unsafe_repair_mutated_value" => ReviewIssueType::UnsafeRepairMutatedValue,
            _ => ReviewIssueType::Other,
        }
    }
}

impl Serialize for ReviewIssueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ReviewIssueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(Self::from_wire_value(&value))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewBlockerScope {
    Source,
    Field,
    PrimerSection,
    Schema,
    Other,
}

impl ReviewBlockerScope {
    fn as_str(&self) -> &'static str {
        match self {
            ReviewBlockerScope::Source => "source",
            ReviewBlockerScope::Field => "field",
            ReviewBlockerScope::PrimerSection => "primer_section",
            ReviewBlockerScope::Schema => "schema",
            ReviewBlockerScope::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewBlocker {
    pub scope: ReviewBlockerScope,
    pub identifier: String,
    pub issue_type: ReviewIssueType,
    pub auto_resolvable: bool,
    #[serde(default)]
    pub repair_guidance: String,
    #[serde(default)]
    pub notes: String,
}

fn display_review_issue_type(issue_type: &ReviewIssueType) -> &'static str {
    issue_type.as_str()
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
    #[serde(default = "default_review_recommended_action")]
    pub recommended_action: ReviewRecommendedAction,
    #[serde(default)]
    pub suggested_contract_changes: Vec<String>,
    #[serde(default)]
    pub auto_repair_eligible: bool,
    #[serde(default)]
    pub all_blockers_auto_resolvable: bool,
    #[serde(default)]
    pub auto_resolvable_blockers: Vec<ReviewBlocker>,
    #[serde(default)]
    pub manual_required_blockers: Vec<ReviewBlocker>,
    pub blocking_issues: Vec<String>,
    pub warnings: Vec<String>,
    pub accepted_sources: Vec<AcceptedSource>,
    pub final_value: ValueProposal,
    #[serde(default)]
    pub reference_pack_primer: Option<ReferencePackPrimer>,
    pub metadata_patch: MetadataPatch,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRecommendedAction {
    ApplyApprovedResult,
    AddressVerifierFeedbackAndRerunReview,
    UpdateContractThenRerunPipeline,
    InvestigateSourcesManually,
}

impl std::fmt::Display for ReviewRecommendedAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(display_recommended_action(*self))
    }
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
    pub recommended_action: ReviewRecommendedAction,
    pub suggested_contract_changes: Vec<String>,
    pub auto_repair_eligible: bool,
    pub all_blockers_auto_resolvable: bool,
    pub auto_resolvable_blockers: Vec<ReviewBlocker>,
    pub manual_required_blockers: Vec<ReviewBlocker>,
    pub warnings: Vec<String>,
    pub blocking_issues: Vec<String>,
}

#[derive(Debug, Default, Clone)]
struct ReviewRunOptions {
    primary_output_path: Option<PathBuf>,
    primary_report_path: Option<PathBuf>,
    additional_blocking_issues: Vec<String>,
    additional_manual_required_blockers: Vec<ReviewBlocker>,
}

#[derive(Debug, Clone)]
struct RepairArtifactPaths {
    prompt_path: PathBuf,
    template_path: PathBuf,
    output_path: PathBuf,
    report_path: PathBuf,
    verifier_prompt_path: PathBuf,
}

impl RepairArtifactPaths {
    fn new(run_dir: &Path) -> Self {
        Self {
            prompt_path: run_dir.join("repair_prompt.md"),
            template_path: run_dir.join("repair_template.json"),
            output_path: run_dir.join("repair_output.json"),
            report_path: run_dir.join("repair_report.md"),
            verifier_prompt_path: run_dir.join("repair_verifier_prompt.md"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunAgentsOutcome {
    pub prepared: PreparedRun,
    pub primary: AgentExecutionLog,
    pub verifier: AgentExecutionLog,
    pub repair: Option<RepairExecutionLog>,
    pub review: ReviewOutcome,
    pub applied: Option<ApplyOutcome>,
}

#[derive(Debug, Clone)]
pub struct RepairExecutionLog {
    pub provider: AgentProvider,
    pub model: String,
    pub stdout_log_path: PathBuf,
    pub stderr_log_path: PathBuf,
    pub prompt_path: PathBuf,
    pub template_path: PathBuf,
    pub output_path: PathBuf,
    pub report_path: PathBuf,
    pub verifier_prompt_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ApplyOutcome {
    pub run_id: String,
    pub year: u32,
    pub category: String,
    pub key: String,
    pub run_dir: PathBuf,
    pub reviewed_artifact_path: PathBuf,
    pub reference_pack_path: PathBuf,
    pub reference_manifest_path: PathBuf,
    pub generated_source_path: PathBuf,
    pub metadata_path: PathBuf,
    pub snapshot_path: PathBuf,
}

fn default_review_recommended_action() -> ReviewRecommendedAction {
    ReviewRecommendedAction::ApplyApprovedResult
}

#[derive(Debug, Clone)]
pub struct PipelineStatusReport {
    pub year: u32,
    pub registry_entries: usize,
    pub pipeline_definitions: usize,
    pub reviewed_artifacts: usize,
    pub reference_packs: usize,
    pub legacy_only_entries: usize,
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
    pub reference_pack_exists: bool,
    pub reference_pack_path: PathBuf,
    pub latest_run: Option<PipelineRunSummary>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReferenceManifest {
    bundle_version: String,
    generated_at: Option<String>,
    categories: BTreeMap<String, Vec<String>>,
    pack_count: usize,
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
    Repair,
    RepairVerifier,
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

    let expected_variants =
        ensure_expected_variants(year, category, key, &definition.expected_variants)?;
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
    let mut review = review_run_with_approval_at(engine_root, &prepared.run_id, true, None)?;
    let mut repair = None;
    let mut applied = if review.approved
        && review.blocking_issues.is_empty()
        && review.recommended_action == ReviewRecommendedAction::ApplyApprovedResult
    {
        Some(apply_run_at(engine_root, &prepared.run_id)?)
    } else {
        None
    };

    if applied.is_none() && review.auto_repair_eligible && review.all_blockers_auto_resolvable {
        let repair_artifacts = write_repair_artifacts_at(engine_root, &prepared.run_id, &review)?;
        let repair_log = execute_agent(
            engine_root,
            &prepared.run_dir,
            &config.primary,
            AgentRole::Repair,
        )?;
        repair = Some(RepairExecutionLog {
            provider: repair_log.provider,
            model: repair_log.model.clone(),
            stdout_log_path: repair_log.stdout_log_path.clone(),
            stderr_log_path: repair_log.stderr_log_path.clone(),
            prompt_path: repair_artifacts.prompt_path.clone(),
            template_path: repair_artifacts.template_path.clone(),
            output_path: repair_artifacts.output_path.clone(),
            report_path: repair_artifacts.report_path.clone(),
            verifier_prompt_path: repair_artifacts.verifier_prompt_path.clone(),
        });
        preserve_initial_repair_rereview_artifacts(&prepared.run_dir)?;
        let _repair_verifier = execute_agent(
            engine_root,
            &prepared.run_dir,
            &config.verifier,
            AgentRole::RepairVerifier,
        )?;
        review = review_run_internal(
            engine_root,
            &prepared.run_id,
            Some(true),
            None,
            ReviewRunOptions {
                primary_output_path: Some(repair_artifacts.output_path.clone()),
                primary_report_path: Some(repair_artifacts.report_path.clone()),
                ..ReviewRunOptions::default()
            },
        )?;
        applied = if review.approved
            && review.blocking_issues.is_empty()
            && review.recommended_action == ReviewRecommendedAction::ApplyApprovedResult
        {
            Some(apply_run_at(engine_root, &prepared.run_id)?)
        } else {
            None
        };
    }

    Ok(RunAgentsOutcome {
        prepared,
        primary,
        verifier,
        repair,
        review,
        applied,
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
    let mut reference_packs = 0;
    let mut legacy_only_entries = 0;
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
        let reference_pack_path =
            reference_pack_path_for(engine_root, year, &entry.category, &entry.key);
        let reference_pack_exists = reference_pack_path.exists();
        if reviewed_artifact_exists {
            reviewed_artifacts += 1;
        }
        if reference_pack_exists {
            reference_packs += 1;
        }
        if reviewed_artifact_exists && !reference_pack_exists {
            legacy_only_entries += 1;
        }
        let latest_run = latest_run_summary_for(engine_root, year, &entry.category, &entry.key)?;

        entries.push(PipelineStatusEntry {
            category: entry.category,
            key: entry.key,
            verification_status: entry.verification_status,
            completeness: entry.completeness,
            pipeline_defined,
            reviewed_artifact_exists,
            reference_pack_exists,
            reference_pack_path,
            latest_run,
            notes: entry.notes,
        });
    }

    Ok(PipelineStatusReport {
        year,
        registry_entries: entries.len(),
        pipeline_definitions: definitions.len(),
        reviewed_artifacts,
        reference_packs,
        legacy_only_entries,
        authoritative_entries,
        corroborated_entries,
        derived_entries,
        placeholder_entries,
        entries,
    })
}

pub fn review_run_at(engine_root: &Path, run_ref: &str) -> Result<ReviewOutcome, PipelineError> {
    let approver = default_approver();
    let options = manual_review_run_options(engine_root, run_ref)?;
    review_run_internal(engine_root, run_ref, None, approver, options)
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
    let options = manual_review_run_options(engine_root, run_ref)?;
    review_run_internal(engine_root, run_ref, Some(approved), approved_by, options)
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
    if !review.blocking_issues.is_empty() || !review.manual_required_blockers.is_empty() {
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
    let reference_pack_primer = review.reference_pack_primer.as_ref().ok_or_else(|| {
        PipelineError::new(format!(
            "approved review for run {} is missing reference_pack_primer",
            run_manifest.run_id
        ))
    })?;
    let reference_pack_path =
        write_reference_pack(engine_root, &reviewed_artifact, reference_pack_primer)?;
    let reference_manifest_path = refresh_reference_manifest(engine_root)?;

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
        reference_pack_path,
        reference_manifest_path,
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
    options: ReviewRunOptions,
) -> Result<ReviewOutcome, PipelineError> {
    let run_dir = resolve_run_dir(engine_root, run_ref)?;
    let run_manifest: RunManifest = load_json(&run_dir.join("run.json"))?;
    let definition =
        load_pipeline_definition_at(engine_root, &run_manifest.category, &run_manifest.key)?;
    let registry = load_registry(&metadata_path_for(engine_root, run_manifest.year))?;
    let current_entry = find_registry_entry(&registry, &run_manifest.category, &run_manifest.key)?;
    let current_artifact: CurrentValueArtifact = load_json(&run_dir.join("current_value.json"))?;
    let default_primary_output_path = run_dir.join("primary_output.json");
    let default_primary_report_path = run_dir.join("primary_report.md");
    let primary_output_path = options
        .primary_output_path
        .clone()
        .unwrap_or_else(|| default_primary_output_path.clone());
    let primary_report_path = options
        .primary_report_path
        .clone()
        .unwrap_or_else(|| default_primary_report_path.clone());
    let original_primary: PrimarySubmission = load_json(&default_primary_output_path)?;
    let primary: PrimarySubmission = load_json(&primary_output_path)?;
    let verifier: VerifierSubmission = load_json(&run_dir.join("verifier_output.json"))?;

    if primary.run_id != run_manifest.run_id {
        return Err(PipelineError::new(format!(
            "{} run_id {} does not match run {}",
            primary_output_path.display(),
            primary.run_id,
            run_manifest.run_id
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
    if primary.agent.is_none() {
        warnings.push(
            "primary_output.json did not include agent metadata; continuing with artifact review"
                .to_string(),
        );
    }
    if verifier.agent.is_none() {
        warnings.push(
            "verifier_output.json did not include agent metadata; continuing with artifact review"
                .to_string(),
        );
    }
    let primary_report = load_required_report(
        &primary_report_path,
        "primary_report.md",
        &mut blocking_issues,
    );
    let verifier_report = load_required_report(
        &run_dir.join("verifier_report.md"),
        "verifier_report.md",
        &mut blocking_issues,
    );
    if primary_output_path != default_primary_output_path {
        blocking_issues.extend(validate_safe_repair_submission(
            &original_primary,
            &primary,
            &verifier,
            &primary_output_path,
            &default_primary_output_path,
        ));
    }
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
    blocking_issues.extend(validate_reference_pack_primer(&primary));
    blocking_issues.extend(validate_verifier_submission(
        &primary,
        &verifier,
        &required_field_paths,
    ));
    blocking_issues.extend(validate_primer_verdicts(&primary, &verifier));
    let mut tiebreaker_confirmed = false;
    if primary_output_path != default_primary_output_path {
        suppress_resolved_repair_blocking_issues(
            &original_primary,
            &primary,
            &verifier,
            &mut blocking_issues,
        );
        let (confirmed, tiebreaker_warnings) = suppress_field_evidence_blockers_via_tiebreaker(
            &original_primary,
            &primary,
            &run_dir,
            &mut blocking_issues,
        );
        tiebreaker_confirmed = confirmed;
        warnings.extend(tiebreaker_warnings);
    }
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
    let (
        auto_resolvable_blockers,
        mut manual_required_blockers,
        _all_blockers_auto_resolvable,
        _auto_repair_eligible,
    ) = classify_review_blockers(
        &primary,
        &verifier,
        &blocking_issues,
        primary.schema_change_required || verifier.schema_change_required,
    );
    blocking_issues.extend(options.additional_blocking_issues.clone());
    manual_required_blockers.extend(options.additional_manual_required_blockers.clone());
    let all_blockers_auto_resolvable =
        !auto_resolvable_blockers.is_empty() && manual_required_blockers.is_empty();
    let auto_repair_eligible = !auto_resolvable_blockers.is_empty() && all_blockers_auto_resolvable;
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

    let recommended_action = determine_recommended_action(&primary, &verifier, &blocking_issues);
    let suggested_contract_changes = suggested_contract_changes(&primary, &verifier);

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
        recommended_action,
        &suggested_contract_changes,
        &auto_resolvable_blockers,
        &manual_required_blockers,
        auto_repair_eligible,
        all_blockers_auto_resolvable,
    );

    let approved = if blocking_issues.is_empty() && manual_required_blockers.is_empty() {
        match approval_override {
            Some(value) => value,
            None if tiebreaker_confirmed => {
                eprintln!(
                    "[data-pipeline] auto-approving {} (Gemini tiebreaker verified)",
                    run_manifest.run_id
                );
                true
            }
            None => prompt_for_approval(&run_manifest.run_id)?,
        }
    } else {
        false
    };
    let review_decision = ReviewDecision {
        schema_version: REVIEWED_ARTIFACT_SCHEMA_VERSION,
        run_id: run_manifest.run_id.clone(),
        approved,
        approved_by: approved.then_some(if tiebreaker_confirmed {
            "gemini-3.1-pro-preview (tiebreaker)".to_string()
        } else {
            approved_by.unwrap_or_else(default_approver_name)
        }),
        status_decision,
        recommended_action,
        suggested_contract_changes: suggested_contract_changes.clone(),
        auto_repair_eligible,
        all_blockers_auto_resolvable,
        auto_resolvable_blockers: auto_resolvable_blockers.clone(),
        manual_required_blockers: manual_required_blockers.clone(),
        blocking_issues: blocking_issues.clone(),
        warnings: warnings.clone(),
        accepted_sources: accepted_sources.clone(),
        final_value: primary.value_proposal.clone(),
        reference_pack_primer: primary.reference_pack_primer.clone(),
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
        recommended_action,
        suggested_contract_changes,
        auto_repair_eligible,
        all_blockers_auto_resolvable,
        auto_resolvable_blockers,
        manual_required_blockers,
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
        contract_notes: definition.contract_notes.clone(),
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
        AgentRole::Repair => "repair_prompt.md",
        AgentRole::RepairVerifier => "repair_verifier_prompt.md",
    });
    let prompt = fs::read_to_string(&prompt_path)?;
    let stdout_log_path = run_dir.join(match role {
        AgentRole::Primary => "primary_agent.stdout.log",
        AgentRole::Verifier => "verifier_agent.stdout.log",
        AgentRole::Repair => "repair_agent.stdout.log",
        AgentRole::RepairVerifier => "repair_verifier_agent.stdout.log",
    });
    let stderr_log_path = run_dir.join(match role {
        AgentRole::Primary => "primary_agent.stderr.log",
        AgentRole::Verifier => "verifier_agent.stderr.log",
        AgentRole::Repair => "repair_agent.stderr.log",
        AgentRole::RepairVerifier => "repair_verifier_agent.stderr.log",
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
    if role == AgentRole::Repair {
        command.env(
            "ENTROPYFA_REPAIR_OUTPUT_PATH",
            run_dir.join("repair_output.json"),
        );
        command.env(
            "ENTROPYFA_REPAIR_REPORT_PATH",
            run_dir.join("repair_report.md"),
        );
    }
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
        AgentRole::Verifier | AgentRole::RepairVerifier => {
            ["verifier_output.json", "verifier_report.md"].as_slice()
        }
        AgentRole::Repair => ["repair_output.json", "repair_report.md"].as_slice(),
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
        "reference_pack_primer": {
            "what_this_is": "<short factual description of the dataset and what it contains>",
            "lookup_parameters": [
                "<lookup parameter required to select the right variant>"
            ],
            "interpretation_notes": [
                "<how to read the values: units, thresholds, inclusivity, age semantics, or stacking behavior>"
            ],
            "does_not_include": [
                "<closely related concept this dataset does not contain>"
            ],
            "caveats": [
                "<important limitation, edge case, or interim-guidance warning>"
            ],
            "typical_uses": [
                "<optional common use; keep non-exclusive and dataset-centric>"
            ]
        },
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
                "notes": "",
                "issue_type": "value_confirmed",
                "auto_resolvable": false,
                "repair_guidance": ""
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
        "source_verdicts": [
            {
                "source_id": "<source_id>",
                "verdict": "accept",
                "counts_toward_status": true,
                "reason": "",
                "issue_type": "value_confirmed",
                "auto_resolvable": false,
                "repair_guidance": ""
            }
        ],
        "field_verdicts": field_verdicts,
        "primer_verdicts": {
            "what_this_is": {
                "verdict": "confirm",
                "notes": "",
                "issue_type": "value_confirmed",
                "auto_resolvable": true,
                "repair_guidance": ""
            },
            "lookup_parameters": {
                "verdict": "confirm",
                "notes": "",
                "issue_type": "value_confirmed",
                "auto_resolvable": true,
                "repair_guidance": ""
            },
            "interpretation_notes": {
                "verdict": "confirm",
                "notes": "",
                "issue_type": "value_confirmed",
                "auto_resolvable": true,
                "repair_guidance": ""
            },
            "does_not_include": {
                "verdict": "confirm",
                "notes": "",
                "issue_type": "value_confirmed",
                "auto_resolvable": true,
                "repair_guidance": ""
            },
            "caveats": {
                "verdict": "confirm",
                "notes": "",
                "issue_type": "value_confirmed",
                "auto_resolvable": true,
                "repair_guidance": ""
            },
            "typical_uses": {
                "verdict": "confirm",
                "notes": "",
                "issue_type": "value_confirmed",
                "auto_resolvable": true,
                "repair_guidance": ""
            }
        },
        "status_recommendation": "authoritative",
        "overall_verdict": "pass",
        "schema_change_required": false,
        "schema_change_notes": [],
        "notes": ""
    })
}

fn build_repair_template(primary: &PrimarySubmission) -> Result<Value, PipelineError> {
    serde_json::to_value(primary).map_err(|error| {
        PipelineError::new(format!(
            "failed to serialize primary submission into repair template: {}",
            error
        ))
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
        ValidationProfile::Niit => json!({
            "rate": "<number>",
            "threshold": "<number>"
        }),
        ValidationProfile::Payroll => json!({
            "social_security_rate": "<number>",
            "social_security_wage_base": "<number>",
            "self_employment_tax_rate": "<number>",
            "medicare_rate": "<number>",
            "self_employment_medicare_rate": "<number>",
            "additional_medicare_rate": "<number>",
            "additional_medicare_threshold": "<number>"
        }),
        ValidationProfile::MedicareBasePremiums => json!({
            "part_b_standard_monthly_premium": "<number>",
            "part_b_annual_deductible": "<number>",
            "part_d_base_beneficiary_premium": "<number>"
        }),
        ValidationProfile::SsRetirementEarningsTest => json!({
            "under_fra_annual_exempt_amount": "<number>",
            "under_fra_monthly_exempt_amount": "<number>",
            "year_of_fra_annual_exempt_amount": "<number>",
            "year_of_fra_monthly_exempt_amount": "<number>",
            "under_fra_reduction_rate": "<number_0_to_1>",
            "year_of_fra_reduction_rate": "<number_0_to_1>"
        }),
        ValidationProfile::SocialSecurityFullRetirementAge => json!({
            "benefit_scope": "<string>",
            "january_1_births_use_prior_year": "<boolean>",
            "rules": [
                {
                    "birth_year_min": "<number_or_null>",
                    "birth_year_max": "<number_or_null>",
                    "full_retirement_age_years": "<number>",
                    "full_retirement_age_months": "<number>"
                }
            ]
        }),
        ValidationProfile::Salt => {
            let mut value = serde_json::Map::new();
            if let Some(filing_status) = variant.params.filing_status.as_ref() {
                value.insert("filing_status".into(), Value::String(filing_status.clone()));
            }
            value.insert("cap_amount".into(), json!("<number>"));
            value.insert("phaseout_threshold".into(), json!("<number>"));
            value.insert("phaseout_rate".into(), json!("<number>"));
            value.insert("floor_amount".into(), json!("<number>"));
            Value::Object(value)
        }
        ValidationProfile::Qbi => json!({
            "deduction_rate": "<number>",
            "threshold": "<number>",
            "phase_in_range_end": "<number>",
            "minimum_qbi_deduction": "<number>",
            "minimum_qbi_amount": "<number>"
        }),
        ValidationProfile::AgeDistribution => json!([
            {
                "age": "<number>",
                "distribution_period": "<number>"
            }
        ]),
        ValidationProfile::JointDistribution => json!([
            {
                "owner_age": "<number>",
                "spouse_age": "<number>",
                "distribution_period": "<number>"
            }
        ]),
        ValidationProfile::MortalityQx => json!([
            {
                "age": "<number>",
                "qx": "<number>"
            }
        ]),
        ValidationProfile::DistributionRules => json!({
            "required_beginning": {
                "start_age_rules": [
                    {
                        "birth_year_min": "<number_or_null>",
                        "birth_year_max": "<number_or_null>",
                        "start_age": "<number>",
                        "guidance_status": "<string_or_null>",
                        "notes": "<string_or_null>"
                    }
                ],
                "first_distribution_deadline": "<string>",
                "still_working_exception": {
                    "eligible_plan_categories": ["<string>"],
                    "eligible_account_types": ["<string>"],
                    "disallowed_for_five_percent_owners": "<boolean>"
                }
            },
            "account_applicability": {
                "owner_required_account_types": ["<string>"],
                "owner_exempt_account_types": ["<string>"],
                "inherited_account_types": ["<string>"],
                "supports_pre_1987_403b_exclusion": "<boolean>",
                "designated_roth_owner_exemption_effective_year": "<number_or_null>",
                "pre_1987_403b": {
                    "exclude_until_age": "<number>"
                }
            },
            "beneficiary_distribution": {
                "beneficiary_categories": ["<string>"],
                "recognized_beneficiary_classes": ["<string>"],
                "eligible_designated_beneficiary_classes": ["<string>"],
                "life_expectancy_method_by_class": {
                    "<beneficiary_class>": "<method>"
                },
                "minor_child_majority_age": "<number>",
                "spouse_delay_allowed": "<boolean>",
                "ten_year_rule": {
                    "terminal_year": "<number>",
                    "annual_distributions_required_when_owner_died_on_or_after_rbd": "<boolean>"
                },
                "non_designated_beneficiary_rules": {
                    "when_owner_died_before_required_beginning_date": "<string>",
                    "when_owner_died_on_or_after_required_beginning_date": "<string>"
                },
                "relief_years": ["<number>"]
            }
        }),
        ValidationProfile::SsTaxation => {
            let mut value = serde_json::Map::new();
            if let Some(filing_status) = variant.params.filing_status.as_ref() {
                value.insert("filing_status".into(), Value::String(filing_status.clone()));
            }
            if let Some(lived_with_spouse_during_year) =
                variant.params.lived_with_spouse_during_year
            {
                value.insert(
                    "lived_with_spouse_during_year".into(),
                    Value::Bool(lived_with_spouse_during_year),
                );
            }
            value.insert("base_amount".into(), json!("<number>"));
            value.insert("upper_amount".into(), json!("<number>"));
            value.insert("max_taxable_pct_below_upper".into(), json!("<number>"));
            value.insert("max_taxable_pct_above_upper".into(), json!("<number>"));
            Value::Object(value)
        }
        ValidationProfile::NumericField { ref field } => {
            let mut value = serde_json::Map::new();
            if let Some(filing_status) = variant.params.filing_status.as_ref() {
                value.insert("filing_status".into(), Value::String(filing_status.clone()));
            }
            if let Some(lived_with_spouse_during_year) =
                variant.params.lived_with_spouse_during_year
            {
                value.insert(
                    "lived_with_spouse_during_year".into(),
                    Value::Bool(lived_with_spouse_during_year),
                );
            }
            value.insert(field.clone(), json!("<number>"));
            Value::Object(value)
        }
        ValidationProfile::Irmaa => {
            let mut value = serde_json::Map::new();
            value.insert(
                "filing_status".into(),
                Value::String(
                    variant
                        .params
                        .filing_status
                        .clone()
                        .unwrap_or_else(|| "<filing_status>".into()),
                ),
            );
            if let Some(lived_with_spouse_during_year) =
                variant.params.lived_with_spouse_during_year
            {
                value.insert(
                    "lived_with_spouse_during_year".into(),
                    Value::Bool(lived_with_spouse_during_year),
                );
            }
            value.insert("base_part_b_premium".into(), json!("<number>"));
            value.insert(
                "brackets".into(),
                json!([
                    {
                        "magi_min": "<number>",
                        "magi_max": "<number_or_null>",
                        "monthly_surcharge": "<number>"
                    }
                ]),
            );
            Value::Object(value)
        }
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
## Reference Pack Primer\n\
- explain why the proposed primer sections accurately describe the dataset and its limits:\n\n\
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
## Primer Review\n\
- confirm or dispute each primer section and explain any mismatch in dataset semantics or scope:\n\n\
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
    let contract_note_block = render_contract_note_block(&definition.contract_notes);
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
7. Fill `reference_pack_primer` completely. Keep it data-centric. Describe the dataset, lookup parameters, interpretation semantics, exclusions, and caveats. Do not turn it into generic financial-planning advice or a workflow script.\n\
8. If the official source does not fit the current JSON schema cleanly, set `schema_change_required` to `true`, explain the mismatch in `schema_change_notes[]`, explain it again in `primary_report.md`, and do not invent new JSON keys. Before doing that, read the contract notes in `source_policy.json`. Do not set `schema_change_required` solely because a bracket table uses published interval notation such as `<=`, `>`, `<`, or `>=` if the numeric thresholds fit the documented contract convention.\n\
9. Populate `sources[]` with every source you relied on using this exact object shape:\n\
   `{{\"source_id\",\"url\",\"host\",\"organization\",\"source_class\",\"title\",\"published_at\",\"locator\",\"notes\"}}`. One source record must correspond to exactly one actual URL.\n\
10. Choose stable source ids like `src_cms_1`, `src_ssa_1`, `src_kff_1`. They must be unique within the file.\n\
11. If you relied on two pages from the same publisher, create two source records. For example, `HI 01101.020` and `HI 01120.060` must be separate SSA source ids if both are used.\n\
12. Update `value_proposal` with extracted values in the exact lookup shape already shown in the template.\n\
13. Populate `field_evidence[]` for every required field group using this exact object shape:\n\
   `{{\"field_path\",\"source_id\",\"locator\"}}`.\n\
14. `field_path` values must match the exact paths already implied by the template, for example `{}`.\n\
15. Every `field_evidence.source_id` must reference one of the ids you created in `sources[]`.\n\
16. Record any uncertainty in `unresolved_issues[]`.\n\
17. The task is incomplete until both output files exist on disk.\n\
18. If your environment does not expose a direct file-write tool, use shell commands to create the files at the exact paths above.\n\
19. After writing both files, run `ls -l` on each output path and do not stop until both commands succeed.\n\n\
Required enums and literals:\n\
- `proposed_status`: `authoritative`, `corroborated`, `derived`, or `placeholder`\n\
- `source_class`: `primary`, `supporting_official`, or `secondary`\n\n\
Do not edit any Rust source, metadata, snapshot, or other repo files.\n\
Do not write anything except `primary_output.json` and `primary_report.md`.\n\n\
Pipeline details:\n\
- pipeline: `{}`\n\
- expected variants: `{}`\n\
- search queries: `{}`\n\
{}",
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
        definition.search_queries.join(" | "),
        contract_note_block
    )
}

fn render_repair_prompt(
    run_dir: &Path,
    run_manifest: &RunManifest,
    review: &ReviewOutcome,
) -> String {
    let blocker_lines = review
        .auto_resolvable_blockers
        .iter()
        .map(|blocker| {
            format!(
                "- {} `{}` ({})\n  - repair_guidance: {}\n  - notes: {}",
                blocker.scope.as_str(),
                blocker.identifier,
                display_review_issue_type(&blocker.issue_type),
                if blocker.repair_guidance.trim().is_empty() {
                    "(none provided)"
                } else {
                    blocker.repair_guidance.as_str()
                },
                if blocker.notes.trim().is_empty() {
                    "(none provided)"
                } else {
                    blocker.notes.as_str()
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# Repair Agent\n\n\
Task: repair the blocked run `{}` for `{}/{}` year `{}`.\n\n\
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
1. Start from `repair_template.json`. Copy its structure exactly into `repair_output.json` and preserve every key name.\n\
2. Repair only the auto-resolvable issues listed below.\n\
3. Do not change `value_proposal` or any reviewed numeric values.\n\
4. Do not change `proposed_status`, `schema_change_required`, or `field_evidence`.\n\
5. You may rewrite `reference_pack_primer` and tighten citations or source locators if needed.\n\
6. Keep `run_id` unchanged.\n\
7. Explain exactly what changed in `repair_report.md`.\n\
8. The task is incomplete until both output files exist on disk.\n\
9. If your environment does not expose a direct file-write tool, use shell commands to create the files at the exact paths above.\n\
10. After writing both files, run `ls -l` on each output path and do not stop until both commands succeed.\n\n\
Auto-resolvable issues to repair:\n\
{}\n",
        run_manifest.run_id,
        run_manifest.category,
        run_manifest.key,
        run_manifest.year,
        run_dir.join("primary_output.json").display(),
        run_dir.join("verifier_output.json").display(),
        run_dir.join("review.json").display(),
        run_dir.join("review.md").display(),
        run_dir.join("repair_template.json").display(),
        run_dir.join("repair_output.json").display(),
        run_dir.join("repair_report.md").display(),
        if blocker_lines.is_empty() {
            "- none".to_string()
        } else {
            blocker_lines
        }
    )
}

fn manual_review_run_options(
    engine_root: &Path,
    run_ref: &str,
) -> Result<ReviewRunOptions, PipelineError> {
    let run_dir = resolve_run_dir(engine_root, run_ref)?;
    let repair_output_path = run_dir.join("repair_output.json");
    let repair_report_path = run_dir.join("repair_report.md");

    if repair_output_path.exists() && repair_report_path.exists() {
        Ok(ReviewRunOptions {
            primary_output_path: Some(repair_output_path),
            primary_report_path: Some(repair_report_path),
            ..ReviewRunOptions::default()
        })
    } else {
        Ok(ReviewRunOptions::default())
    }
}

fn preserve_initial_repair_rereview_artifacts(run_dir: &Path) -> Result<(), PipelineError> {
    preserve_artifact_if_present(
        &run_dir.join("verifier_output.json"),
        &run_dir.join("initial_verifier_output.json"),
    )?;
    preserve_artifact_if_present(
        &run_dir.join("verifier_report.md"),
        &run_dir.join("initial_verifier_report.md"),
    )?;
    preserve_artifact_if_present(
        &run_dir.join("review.json"),
        &run_dir.join("initial_review.json"),
    )?;
    preserve_artifact_if_present(
        &run_dir.join("review.md"),
        &run_dir.join("initial_review.md"),
    )?;
    Ok(())
}

fn preserve_artifact_if_present(source: &Path, destination: &Path) -> Result<(), PipelineError> {
    if !source.exists() || destination.exists() {
        return Ok(());
    }

    fs::copy(source, destination).map_err(|error| {
        PipelineError::new(format!(
            "failed to preserve {} at {}: {}",
            source.display(),
            destination.display(),
            error
        ))
    })?;
    Ok(())
}

fn write_repair_artifacts_at(
    engine_root: &Path,
    run_ref: &str,
    review: &ReviewOutcome,
) -> Result<RepairArtifactPaths, PipelineError> {
    let run_dir = resolve_run_dir(engine_root, run_ref)?;
    let run_manifest: RunManifest = load_json(&run_dir.join("run.json"))?;
    let definition =
        load_pipeline_definition_at(engine_root, &run_manifest.category, &run_manifest.key)?;
    if review.approved {
        return Err(PipelineError::new(format!(
            "review for run {} is already approved and does not need repair artifact preparation",
            run_manifest.run_id
        )));
    }
    if !review.auto_repair_eligible || !review.all_blockers_auto_resolvable {
        return Err(PipelineError::new(format!(
            "review for run {} is not eligible for bounded repair artifact preparation",
            run_manifest.run_id
        )));
    }

    let primary: PrimarySubmission = load_json(&run_dir.join("primary_output.json"))?;
    let artifact_paths = RepairArtifactPaths::new(&run_dir);
    write_json_value(
        &artifact_paths.template_path,
        &build_repair_template(&primary)?,
    )?;
    write_text(
        &artifact_paths.prompt_path,
        &render_repair_prompt(&run_dir, &run_manifest, review),
    )?;
    write_text(
        &artifact_paths.verifier_prompt_path,
        &render_repair_verifier_prompt(&run_dir, &run_manifest, &definition),
    )?;

    Ok(artifact_paths)
}

fn render_verifier_prompt(
    run_dir: &Path,
    run_manifest: &RunManifest,
    definition: &PipelineDefinition,
) -> String {
    render_verifier_prompt_for_submission(
        run_dir,
        run_manifest,
        definition,
        "primary_output.json",
        &run_dir.join("primary_output.json"),
    )
}

fn render_repair_verifier_prompt(
    run_dir: &Path,
    run_manifest: &RunManifest,
    definition: &PipelineDefinition,
) -> String {
    render_verifier_prompt_for_submission(
        run_dir,
        run_manifest,
        definition,
        "repair_output.json",
        &run_dir.join("repair_output.json"),
    )
}

fn render_verifier_prompt_for_submission(
    run_dir: &Path,
    run_manifest: &RunManifest,
    definition: &PipelineDefinition,
    candidate_label: &str,
    candidate_path: &Path,
) -> String {
    let contract_note_block = render_contract_note_block(&definition.contract_notes);
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
5. Review `reference_pack_primer` independently. Use `primer_verdicts` to confirm, dispute, or mark uncertain each required section. Dispute sections that overstate what the dataset contains, omit critical lookup parameters, or describe the wrong interpretation semantics.\n\
6. For each disputed source, field, or primer section, set `issue_type`, `auto_resolvable`, and `repair_guidance` in the verdict object.\n\
7. Only mark `auto_resolvable: true` for bounded repairable defects such as primer wording, primer scope, or citation precision. Never mark value, source-policy, or schema-mismatch issues as auto-resolvable.\n\
8. If the source material does not fit the current JSON schema cleanly, set `schema_change_required` to `true`, explain the mismatch in `schema_change_notes[]`, explain it again in `verifier_report.md`, and do not invent new JSON keys. Before doing that, read the contract notes in `source_policy.json`. Do not set `schema_change_required` solely because a bracket table uses published interval notation such as `<=`, `>`, `<`, or `>=` if the numeric thresholds fit the documented contract convention.\n\
9. In `source_verdicts[]`, use this exact object shape:\n\
   `{{\"source_id\",\"verdict\",\"counts_toward_status\",\"reason\",\"issue_type\",\"auto_resolvable\",\"repair_guidance\"}}`.\n\
10. `source_verdicts[].source_id` must match the exact `source_id` values from `{}`. Do not replace ids with URLs.\n\
11. If `{}` relied on multiple pages from the same publisher, expect separate source ids for the actual URLs used. Do not let one source record stand in for multiple pages.\n\
12. In `field_verdicts[]`, use this exact object shape:\n\
   `{{\"field_path\",\"verdict\",\"corrected_value\",\"source_ids\",\"notes\",\"issue_type\",\"auto_resolvable\",\"repair_guidance\"}}`.\n\
13. `field_path` values must match the exact required field paths from the template.\n\
14. Every id in `field_verdicts[].source_ids` must match a `source_id` from `{}`.\n\
15. Use `field_verdicts[]` to judge whether `{}` is supported by the cited or replacement sources, not whether it differs from `current_value.json`.\n\
16. Do not use `dispute` merely because `current_value.json` differs from `{}`. If official sources support the primary proposal and the current embedded value is stale, use `confirm` and explain the stale embedded value in `notes` or `verifier_report.md`.\n\
17. Use `dispute` only when the proposal in `{}` itself is wrong. When you use `dispute`, set `corrected_value` to the source-supported replacement and explain why the proposal is wrong.\n\
18. Confirm, dispute, or mark uncertain each required field group in `field_verdicts[]`.\n\
19. Recommend `authoritative`, `corroborated`, or `needs_human_attention`.\n\
20. If anything is unresolved or inconsistent, set `overall_verdict` accordingly.\n\
21. The task is incomplete until both output files exist on disk.\n\
22. If your environment does not expose a direct file-write tool, use shell commands to create the files at the exact paths above.\n\
23. After writing both files, run `ls -l` on each output path and do not stop until both commands succeed.\n\n\
Required enums and literals:\n\
- `source_verdicts[].verdict`: `accept` or `reject`\n\
- `field_verdicts[].verdict`: `confirm`, `dispute`, or `uncertain`\n\
- `status_recommendation`: `authoritative`, `corroborated`, or `needs_human_attention`\n\
- `overall_verdict`: `pass`, `needs_human_attention`, or `reject`\n\
- `issue_type`: `primer_scope_only`, `overbroad_primer`, `value_dispute_case`, `primer_scope_overstatement`, `primer_factual_imprecision`, `primer_missing_required_section`, `lookup_parameters_missing`, `interpretation_notes_missing`, `does_not_include_missing`, `caveats_missing`, `value_confirmed`, `value_mismatch`, `citation_locator_inexact`, `source_policy_failure`, `schema_mismatch`, `unsafe_repair_mutated_value`, or `other`\n\
- `auto_resolvable`: `true` or `false`\n\
Do not edit any Rust source, metadata, snapshot, or other repo files.\n\
Do not write anything except `verifier_output.json` and `verifier_report.md`.\n\
Pipeline details:\n\
- pipeline: `{}`\n\
- required primary hosts: `{}`\n\
- allowed supporting hosts: `{}`\n\
- allowed secondary hosts: `{}`\n\
{}",
        run_manifest.category,
        run_manifest.key,
        run_manifest.year,
        run_dir.join("source_policy.json").display(),
        run_dir.join("current_value.json").display(),
        candidate_path.display(),
        run_dir.join("verifier_template.json").display(),
        run_dir.join("verifier_report_template.md").display(),
        run_dir.join("verifier_output.json").display(),
        run_dir.join("verifier_report.md").display(),
        candidate_label,
        candidate_label,
        candidate_label,
        candidate_label,
        candidate_label,
        candidate_label,
        definition.pipeline_name,
        definition.required_primary_hosts.join(", "),
        definition.allowed_supporting_hosts.join(", "),
        definition.allowed_secondary_hosts.join(", "),
        contract_note_block
    )
}

fn render_contract_note_block(notes: &[String]) -> String {
    if notes.is_empty() {
        return String::new();
    }

    let rendered = notes
        .iter()
        .map(|note| format!("  - {note}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("- contract notes:\n{rendered}\n")
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
        if matches!(
            definition.validation_profile,
            ValidationProfile::NumericField { .. }
                | ValidationProfile::Irmaa
                | ValidationProfile::SsTaxation
        ) {
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
            if let Some(expected_lived_with_spouse) = variant.params.lived_with_spouse_during_year {
                if let Some(value_obj) = variant.value.as_object() {
                    match value_obj
                        .get("lived_with_spouse_during_year")
                        .and_then(Value::as_bool)
                    {
                        Some(actual_value) if actual_value == expected_lived_with_spouse => {}
                        Some(actual_value) => issues.push(format!(
                            "variant {} lived_with_spouse_during_year {} does not match expected {}",
                            variant.label, actual_value, expected_lived_with_spouse
                        )),
                        None => issues.push(format!(
                            "variant {} is missing lived_with_spouse_during_year in value",
                            variant.label
                        )),
                    }
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
        let covered = evidence_map.contains_key(field_path.as_str())
            || evidence_map.keys().any(|k| {
                k.starts_with(field_path.as_str())
                    && k.as_bytes().get(field_path.len()) == Some(&b'[')
            });
        if !covered {
            issues.push(format!(
                "field_evidence is missing required field path {}",
                field_path
            ));
        }
    }

    issues
}

fn validate_reference_pack_primer(primary: &PrimarySubmission) -> Vec<String> {
    let mut issues = Vec::new();
    let Some(primer) = primary.reference_pack_primer.as_ref() else {
        issues.push("reference_pack_primer.what_this_is is missing".to_string());
        issues.push("reference_pack_primer.lookup_parameters is missing".to_string());
        issues.push("reference_pack_primer.interpretation_notes is missing".to_string());
        issues.push("reference_pack_primer.does_not_include is missing".to_string());
        issues.push("reference_pack_primer.caveats is missing".to_string());
        return issues;
    };

    if primer.what_this_is.trim().is_empty() {
        issues.push("reference_pack_primer.what_this_is is missing".to_string());
    }
    if !has_non_empty_list_items(&primer.lookup_parameters) {
        issues.push("reference_pack_primer.lookup_parameters is missing".to_string());
    }
    if !has_non_empty_list_items(&primer.interpretation_notes) {
        issues.push("reference_pack_primer.interpretation_notes is missing".to_string());
    }
    if !has_non_empty_list_items(&primer.does_not_include) {
        issues.push("reference_pack_primer.does_not_include is missing".to_string());
    }
    if !has_non_empty_list_items(&primer.caveats) {
        issues.push("reference_pack_primer.caveats is missing".to_string());
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

fn validate_primer_verdicts(
    primary: &PrimarySubmission,
    verifier: &VerifierSubmission,
) -> Vec<String> {
    let mut issues = Vec::new();
    let Some(primer) = primary.reference_pack_primer.as_ref() else {
        return issues;
    };

    for (section, verdict) in required_primer_verdicts(&verifier.primer_verdicts) {
        match verdict {
            Some(verdict) if verdict.verdict == FieldVerdictDecision::Confirm => {}
            Some(verdict) => issues.push(format!(
                "verifier marked primer section {} as {}",
                section,
                display_field_verdict(&verdict.verdict)
            )),
            None => issues.push(format!("primer_verdicts.{} is missing", section)),
        }
    }

    if has_non_empty_list_items(&primer.typical_uses) {
        match verifier.primer_verdicts.typical_uses.as_ref() {
            Some(verdict) if verdict.verdict == FieldVerdictDecision::Confirm => {}
            Some(verdict) => issues.push(format!(
                "verifier marked primer section typical_uses as {}",
                display_field_verdict(&verdict.verdict)
            )),
            None => issues.push("primer_verdicts.typical_uses is missing".to_string()),
        }
    }

    issues
}

fn validate_safe_repair_submission(
    original: &PrimarySubmission,
    repaired: &PrimarySubmission,
    verifier: &VerifierSubmission,
    repaired_output_path: &Path,
    original_output_path: &Path,
) -> Vec<String> {
    let mut issues = Vec::new();

    if repaired.run_id != original.run_id {
        issues.push(format!(
            "safe repair validation failed: {} run_id {} does not match original {} run_id {}",
            repaired_output_path.display(),
            repaired.run_id,
            original_output_path.display(),
            original.run_id
        ));
    }
    if repaired.proposed_status != original.proposed_status {
        issues.push(format!(
            "safe repair validation failed: {} changed proposed_status from {} to {}",
            repaired_output_path.display(),
            original.proposed_status,
            repaired.proposed_status
        ));
    }
    if repaired.schema_change_required != original.schema_change_required {
        issues.push(format!(
            "safe repair validation failed: {} changed schema_change_required",
            repaired_output_path.display()
        ));
    }
    if repaired.schema_change_notes != original.schema_change_notes {
        issues.push(format!(
            "safe repair validation failed: {} changed schema_change_notes",
            repaired_output_path.display()
        ));
    }
    if repaired.value_proposal != original.value_proposal {
        issues.push(format!(
            "safe repair validation failed: {} mutated reviewed numeric values",
            repaired_output_path.display()
        ));
    }
    if repaired.unresolved_issues != original.unresolved_issues {
        issues.push(format!(
            "safe repair validation failed: {} changed unresolved_issues",
            repaired_output_path.display()
        ));
    }

    if let Some(issue) = validate_safe_repair_sources(
        original,
        repaired,
        repaired_output_path,
        original_output_path,
    ) {
        issues.push(issue);
    }
    if let Some(issue) = validate_safe_repair_field_evidence(
        original,
        repaired,
        repaired_output_path,
        original_output_path,
    ) {
        issues.push(issue);
    }
    if let Some(issue) =
        validate_safe_repair_primer(original, repaired, verifier, repaired_output_path)
    {
        issues.push(issue);
    }

    issues
}

fn suppress_resolved_repair_blocking_issues(
    original: &PrimarySubmission,
    repaired: &PrimarySubmission,
    verifier: &VerifierSubmission,
    blocking_issues: &mut Vec<String>,
) {
    let mut removed_auto_resolvable_issue = false;
    let mut filtered = Vec::with_capacity(blocking_issues.len());

    for issue in blocking_issues.drain(..) {
        if let Some(section) = issue
            .strip_prefix("verifier marked primer section ")
            .and_then(|rest| rest.split(" as ").next())
        {
            if primer_section_auto_resolved(original, repaired, verifier, section) {
                removed_auto_resolvable_issue = true;
                continue;
            }
        }

        if let Some(section) = issue
            .strip_prefix("reference_pack_primer.")
            .and_then(|rest| rest.split_whitespace().next())
            .map(|value| value.trim_end_matches('.'))
        {
            if primer_section_present(repaired, section) {
                removed_auto_resolvable_issue = true;
                continue;
            }
        }

        filtered.push(issue);
    }

    if removed_auto_resolvable_issue {
        filtered.retain(|issue| issue != "verifier overall verdict is needs_human_attention");
    }

    *blocking_issues = filtered;
}

fn primer_section_auto_resolved(
    original: &PrimarySubmission,
    repaired: &PrimarySubmission,
    verifier: &VerifierSubmission,
    section: &str,
) -> bool {
    let verdict = match section {
        "what_this_is" => verifier.primer_verdicts.what_this_is.as_ref(),
        "lookup_parameters" => verifier.primer_verdicts.lookup_parameters.as_ref(),
        "interpretation_notes" => verifier.primer_verdicts.interpretation_notes.as_ref(),
        "does_not_include" => verifier.primer_verdicts.does_not_include.as_ref(),
        "caveats" => verifier.primer_verdicts.caveats.as_ref(),
        "typical_uses" => verifier.primer_verdicts.typical_uses.as_ref(),
        _ => None,
    };

    verdict
        .and_then(|value| value.auto_resolvable)
        .unwrap_or(false)
        && primer_section_changed(original, repaired, section)
}

fn primer_section_changed(
    original: &PrimarySubmission,
    repaired: &PrimarySubmission,
    section: &str,
) -> bool {
    match section {
        "what_this_is" => {
            primer_string_field(original, section) != primer_string_field(repaired, section)
        }
        "lookup_parameters"
        | "interpretation_notes"
        | "does_not_include"
        | "caveats"
        | "typical_uses" => {
            primer_list_field(original, section) != primer_list_field(repaired, section)
        }
        _ => false,
    }
}

fn primer_section_present(primary: &PrimarySubmission, section: &str) -> bool {
    match section {
        "what_this_is" => !primer_string_field(primary, section).is_empty(),
        "lookup_parameters"
        | "interpretation_notes"
        | "does_not_include"
        | "caveats"
        | "typical_uses" => !primer_list_field(primary, section).is_empty(),
        _ => false,
    }
}

fn primer_string_field(primary: &PrimarySubmission, section: &str) -> String {
    let Some(primer) = primary.reference_pack_primer.as_ref() else {
        return String::new();
    };
    match section {
        "what_this_is" => primer.what_this_is.trim().to_string(),
        _ => String::new(),
    }
}

fn primer_list_field(primary: &PrimarySubmission, section: &str) -> Vec<String> {
    let Some(primer) = primary.reference_pack_primer.as_ref() else {
        return Vec::new();
    };
    let values = match section {
        "lookup_parameters" => &primer.lookup_parameters,
        "interpretation_notes" => &primer.interpretation_notes,
        "does_not_include" => &primer.does_not_include,
        "caveats" => &primer.caveats,
        "typical_uses" => &primer.typical_uses,
        _ => return Vec::new(),
    };

    values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect()
}

fn validate_safe_repair_sources(
    original: &PrimarySubmission,
    repaired: &PrimarySubmission,
    repaired_output_path: &Path,
    original_output_path: &Path,
) -> Option<String> {
    let original_sources = original
        .sources
        .iter()
        .map(|source| (source.source_id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let repaired_sources = repaired
        .sources
        .iter()
        .map(|source| (source.source_id.as_str(), source))
        .collect::<BTreeMap<_, _>>();

    if original_sources.len() != repaired_sources.len() {
        return Some(format!(
            "safe repair validation failed: {} changed the number of sources relative to {}",
            repaired_output_path.display(),
            original_output_path.display()
        ));
    }

    for (source_id, original_source) in &original_sources {
        let Some(repaired_source) = repaired_sources.get(source_id) else {
            return Some(format!(
                "safe repair validation failed: {} removed source_id {} from {}",
                repaired_output_path.display(),
                source_id,
                original_output_path.display()
            ));
        };
        if original_source.url != repaired_source.url
            || original_source.host != repaired_source.host
            || original_source.organization != repaired_source.organization
            || original_source.source_class != repaired_source.source_class
            || original_source.title != repaired_source.title
            || original_source.published_at != repaired_source.published_at
        {
            return Some(format!(
                "safe repair validation failed: {} changed source_id {} outside the allowed citation scope",
                repaired_output_path.display(),
                source_id
            ));
        }
    }

    None
}

fn validate_safe_repair_field_evidence(
    original: &PrimarySubmission,
    repaired: &PrimarySubmission,
    repaired_output_path: &Path,
    original_output_path: &Path,
) -> Option<String> {
    let original_pairs = original
        .field_evidence
        .iter()
        .map(|evidence| {
            (
                evidence.field_path.as_str().to_string(),
                evidence.source_id.as_str().to_string(),
            )
        })
        .collect::<BTreeSet<_>>();
    let repaired_pairs = repaired
        .field_evidence
        .iter()
        .map(|evidence| {
            (
                evidence.field_path.as_str().to_string(),
                evidence.source_id.as_str().to_string(),
            )
        })
        .collect::<BTreeSet<_>>();

    if original_pairs != repaired_pairs {
        return Some(format!(
            "safe repair validation failed: {} changed field_evidence coverage relative to {}",
            repaired_output_path.display(),
            original_output_path.display()
        ));
    }

    None
}

// ---------------------------------------------------------------------------
// Gemini tiebreaker — grounded verification of field_evidence repairs
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct TiebreakerQuery {
    source_id: String,
    source_url: String,
    field_path: String,
    change: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TiebreakerResult {
    confirms_repair: bool,
    reasoning: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TiebreakerOutput {
    model: String,
    queries: Vec<TiebreakerQuery>,
    result: Option<TiebreakerResult>,
    error: Option<String>,
    all_confirmed: bool,
}

fn gemini_tiebreaker_verify(
    queries: &[TiebreakerQuery],
    sources: &[SourceRecord],
) -> Result<TiebreakerResult, String> {
    let api_key = env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY not set".to_string())?;

    let source_map: BTreeMap<&str, &SourceRecord> =
        sources.iter().map(|s| (s.source_id.as_str(), s)).collect();

    let mut prompt_lines = Vec::new();
    prompt_lines.push(
        "You are a factual verification agent. For each source URL below, determine whether \
         the repair agent's field_evidence remapping is more accurate than the original.\n\n\
         Use Google Search to access the source URLs and verify what data each source actually reports.\n"
            .to_string(),
    );

    for query in queries {
        let source = source_map.get(query.source_id.as_str());
        let title = source.map_or("(unknown)", |s| s.title.as_str());
        prompt_lines.push(format!(
            "- Source `{}` ({}) — URL: {}\n  Field: `{}`\n  Change: {}\n",
            query.source_id, title, query.source_url, query.field_path, query.change,
        ));
    }

    prompt_lines.push(
        "\nFor each change listed above, verify whether the source URL actually reports \
         data for the listed field. Then decide:\n\
         - `confirms_repair: true` if ALL remappings are factually correct\n\
         - `confirms_repair: false` if ANY remapping is incorrect\n\n\
         Respond in JSON: {\"confirms_repair\": true/false, \"reasoning\": \"...\"}"
            .to_string(),
    );

    let prompt_text = prompt_lines.join("\n");
    let model = "gemini-3.1-pro-preview";

    let body = json!({
        "contents": [{
            "role": "user",
            "parts": [{"text": prompt_text}]
        }],
        "tools": [{"googleSearch": {}}],
        "generationConfig": {
            "responseMimeType": "application/json",
            "responseSchema": {
                "type": "object",
                "properties": {
                    "confirms_repair": {"type": "boolean"},
                    "reasoning": {"type": "string"}
                },
                "required": ["confirms_repair", "reasoning"]
            },
            "thinkingConfig": {
                "thinkingBudget": 24576
            }
        }
    });

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| format!("failed to build HTTP client: {e}"))?;

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("Gemini API request failed: {e}"))?;

    let status = response.status();
    let response_text = response
        .text()
        .map_err(|e| format!("failed to read Gemini response body: {e}"))?;

    if !status.is_success() {
        return Err(format!(
            "Gemini API returned {}: {}",
            status,
            truncate_for_terminal(&response_text, 200)
        ));
    }

    let response_json: Value = serde_json::from_str(&response_text)
        .map_err(|e| format!("failed to parse Gemini response JSON: {e}"))?;

    // Extract the text part from the first candidate
    let text = response_json
        .pointer("/candidates/0/content/parts")
        .and_then(|parts| {
            parts.as_array().and_then(|arr| {
                arr.iter()
                    .find_map(|part| part.get("text").and_then(Value::as_str))
            })
        })
        .ok_or_else(|| {
            format!(
                "Gemini response missing text content: {}",
                truncate_for_terminal(&response_text, 200)
            )
        })?;

    let result: TiebreakerResult = serde_json::from_str(text).map_err(|e| {
        format!(
            "failed to parse Gemini tiebreaker result: {e} — raw: {}",
            truncate_for_terminal(text, 200)
        )
    })?;

    Ok(result)
}

fn suppress_field_evidence_blockers_via_tiebreaker(
    original: &PrimarySubmission,
    repaired: &PrimarySubmission,
    run_dir: &Path,
    blocking_issues: &mut Vec<String>,
) -> (bool, Vec<String>) {
    let has_field_evidence_blocker = blocking_issues.iter().any(|issue| {
        issue.starts_with("safe repair validation failed:") && issue.contains("field_evidence")
    });
    if !has_field_evidence_blocker {
        return (false, Vec::new());
    }

    if env::var("GEMINI_API_KEY").is_err() {
        return (false, Vec::new());
    }

    // Compute field_evidence diff
    let original_pairs: BTreeSet<(String, String)> = original
        .field_evidence
        .iter()
        .map(|e| (e.field_path.clone(), e.source_id.clone()))
        .collect();
    let repaired_pairs: BTreeSet<(String, String)> = repaired
        .field_evidence
        .iter()
        .map(|e| (e.field_path.clone(), e.source_id.clone()))
        .collect();

    let mut queries = Vec::new();

    for (field_path, source_id) in repaired_pairs.difference(&original_pairs) {
        let source_url = repaired
            .sources
            .iter()
            .find(|s| s.source_id == *source_id)
            .map(|s| s.url.clone())
            .unwrap_or_default();
        queries.push(TiebreakerQuery {
            source_id: source_id.clone(),
            source_url,
            field_path: field_path.clone(),
            change: "added by repair".into(),
        });
    }

    for (field_path, source_id) in original_pairs.difference(&repaired_pairs) {
        let source_url = original
            .sources
            .iter()
            .find(|s| s.source_id == *source_id)
            .map(|s| s.url.clone())
            .unwrap_or_default();
        queries.push(TiebreakerQuery {
            source_id: source_id.clone(),
            source_url,
            field_path: field_path.clone(),
            change: "removed by repair".into(),
        });
    }

    if queries.is_empty() {
        return (false, Vec::new());
    }

    eprintln!(
        "[data-pipeline] running Gemini tiebreaker for {} field_evidence change(s)",
        queries.len()
    );

    let model = "gemini-3.1-pro-preview".to_string();
    let result = gemini_tiebreaker_verify(&queries, &repaired.sources);

    let (tiebreaker_result, error, all_confirmed) = match &result {
        Ok(r) => (Some(r.clone()), None, r.confirms_repair),
        Err(e) => (None, Some(e.clone()), false),
    };

    let output = TiebreakerOutput {
        model,
        queries,
        result: tiebreaker_result,
        error,
        all_confirmed,
    };

    // Write tiebreaker output regardless of outcome
    let output_path = run_dir.join("tiebreaker_output.json");
    if let Ok(json_value) = serde_json::to_value(&output) {
        let _ = write_json_value(&output_path, &json_value);
    }

    if all_confirmed {
        eprintln!("[data-pipeline] Gemini tiebreaker confirmed field_evidence repair");
        blocking_issues.retain(|issue| {
            !(issue.starts_with("safe repair validation failed:")
                && issue.contains("field_evidence"))
        });
        let reasoning = result.as_ref().map(|r| r.reasoning.as_str()).unwrap_or("");
        (
            true,
            vec![format!(
                "Gemini tiebreaker confirmed field_evidence repair: {}",
                reasoning
            )],
        )
    } else {
        let reason = result
            .as_ref()
            .map(|r| r.reasoning.clone())
            .unwrap_or_else(|e| e.clone());
        eprintln!(
            "[data-pipeline] Gemini tiebreaker did not confirm repair: {}",
            reason
        );
        (false, Vec::new())
    }
}

fn validate_safe_repair_primer(
    original: &PrimarySubmission,
    repaired: &PrimarySubmission,
    verifier: &VerifierSubmission,
    repaired_output_path: &Path,
) -> Option<String> {
    let primer_sections = [
        "what_this_is",
        "lookup_parameters",
        "interpretation_notes",
        "does_not_include",
        "caveats",
        "typical_uses",
    ];

    for section in primer_sections {
        if !primer_section_changed(original, repaired, section) {
            continue;
        }
        if !primer_section_allowed_for_safe_repair(verifier, section) {
            return Some(format!(
                "safe repair validation failed: {} changed primer section {} outside the allowed repair scope",
                repaired_output_path.display(),
                section
            ));
        }
    }

    None
}

fn primer_section_allowed_for_safe_repair(verifier: &VerifierSubmission, section: &str) -> bool {
    let verdict = match section {
        "what_this_is" => verifier.primer_verdicts.what_this_is.as_ref(),
        "lookup_parameters" => verifier.primer_verdicts.lookup_parameters.as_ref(),
        "interpretation_notes" => verifier.primer_verdicts.interpretation_notes.as_ref(),
        "does_not_include" => verifier.primer_verdicts.does_not_include.as_ref(),
        "caveats" => verifier.primer_verdicts.caveats.as_ref(),
        "typical_uses" => verifier.primer_verdicts.typical_uses.as_ref(),
        _ => None,
    };

    verdict
        .map(|value| value.auto_resolvable.unwrap_or(false))
        .unwrap_or(false)
}

fn classify_review_blockers(
    primary: &PrimarySubmission,
    verifier: &VerifierSubmission,
    blocking_issues: &[String],
    schema_change_required: bool,
) -> (Vec<ReviewBlocker>, Vec<ReviewBlocker>, bool, bool) {
    let mut auto_resolvable_blockers = Vec::new();
    let mut manual_required_blockers = Vec::new();

    if schema_change_required {
        manual_required_blockers.push(ReviewBlocker {
            scope: ReviewBlockerScope::Schema,
            identifier: "schema_change_required".into(),
            issue_type: ReviewIssueType::SchemaMismatch,
            auto_resolvable: false,
            repair_guidance: "Update the contract before retrying this run.".into(),
            notes: "schema_change_required was set by the agents or review validation".into(),
        });
    }

    for verdict in &verifier.source_verdicts {
        if verdict.verdict != SourceVerdictDecision::Reject {
            continue;
        }
        let issue_type = verdict
            .issue_type
            .clone()
            .unwrap_or(ReviewIssueType::SourcePolicyFailure);
        let auto_resolvable = verdict.auto_resolvable.unwrap_or(false)
            && matches!(issue_type, ReviewIssueType::CitationLocatorInexact);
        let blocker = ReviewBlocker {
            scope: ReviewBlockerScope::Source,
            identifier: verdict.source_id.clone(),
            issue_type,
            auto_resolvable,
            repair_guidance: verdict.repair_guidance.clone(),
            notes: verdict.reason.clone(),
        };
        if blocker.auto_resolvable {
            auto_resolvable_blockers.push(blocker);
        } else {
            manual_required_blockers.push(blocker);
        }
    }

    for verdict in &verifier.field_verdicts {
        if verdict.verdict == FieldVerdictDecision::Confirm {
            continue;
        }
        let issue_type = verdict
            .issue_type
            .clone()
            .unwrap_or(ReviewIssueType::ValueMismatch);
        let auto_resolvable = verdict.auto_resolvable.unwrap_or(false)
            && matches!(issue_type, ReviewIssueType::CitationLocatorInexact);
        let blocker = ReviewBlocker {
            scope: ReviewBlockerScope::Field,
            identifier: verdict.field_path.clone(),
            issue_type,
            auto_resolvable,
            repair_guidance: verdict.repair_guidance.clone(),
            notes: verdict.notes.clone(),
        };
        if blocker.auto_resolvable {
            auto_resolvable_blockers.push(blocker);
        } else {
            manual_required_blockers.push(blocker);
        }
    }

    if let Some(primer) = primary.reference_pack_primer.as_ref() {
        for (section, verdict) in required_primer_verdicts(&verifier.primer_verdicts) {
            let Some(verdict) = verdict else {
                continue;
            };
            if verdict.verdict == FieldVerdictDecision::Confirm {
                continue;
            }
            let issue_type = verdict.issue_type.clone().unwrap_or(match section {
                "lookup_parameters" => ReviewIssueType::LookupParametersMissing,
                "interpretation_notes" => ReviewIssueType::PrimerFactualImprecision,
                "does_not_include" => ReviewIssueType::PrimerScopeOverstatement,
                "caveats" => ReviewIssueType::PrimerScopeOnly,
                _ => ReviewIssueType::PrimerScopeOnly,
            });
            let auto_resolvable = verdict.auto_resolvable.unwrap_or(true)
                && !matches!(
                    issue_type,
                    ReviewIssueType::ValueMismatch
                        | ReviewIssueType::ValueDisputeCase
                        | ReviewIssueType::SourcePolicyFailure
                        | ReviewIssueType::SchemaMismatch
                        | ReviewIssueType::UnsafeRepairMutatedValue
                );
            let blocker = ReviewBlocker {
                scope: ReviewBlockerScope::PrimerSection,
                identifier: section.to_string(),
                issue_type,
                auto_resolvable,
                repair_guidance: verdict.repair_guidance.clone(),
                notes: verdict.notes.clone(),
            };
            if blocker.auto_resolvable {
                auto_resolvable_blockers.push(blocker);
            } else {
                manual_required_blockers.push(blocker);
            }
        }

        if has_non_empty_list_items(&primer.typical_uses) {
            if let Some(verdict) = verifier.primer_verdicts.typical_uses.as_ref() {
                if verdict.verdict != FieldVerdictDecision::Confirm {
                    let issue_type = verdict
                        .issue_type
                        .clone()
                        .unwrap_or(ReviewIssueType::PrimerScopeOnly);
                    let auto_resolvable = verdict.auto_resolvable.unwrap_or(true)
                        && !matches!(
                            issue_type,
                            ReviewIssueType::ValueMismatch
                                | ReviewIssueType::ValueDisputeCase
                                | ReviewIssueType::SourcePolicyFailure
                                | ReviewIssueType::SchemaMismatch
                                | ReviewIssueType::UnsafeRepairMutatedValue
                        );
                    let blocker = ReviewBlocker {
                        scope: ReviewBlockerScope::PrimerSection,
                        identifier: "typical_uses".into(),
                        issue_type,
                        auto_resolvable,
                        repair_guidance: verdict.repair_guidance.clone(),
                        notes: verdict.notes.clone(),
                    };
                    if blocker.auto_resolvable {
                        auto_resolvable_blockers.push(blocker);
                    } else {
                        manual_required_blockers.push(blocker);
                    }
                }
            }
        }
    }

    for issue in blocking_issues {
        if issue.starts_with("safe repair validation failed:") {
            manual_required_blockers.push(ReviewBlocker {
                scope: ReviewBlockerScope::Other,
                identifier: issue.clone(),
                issue_type: ReviewIssueType::UnsafeRepairMutatedValue,
                auto_resolvable: false,
                repair_guidance: "Trim the repair output back to the reviewed value and allowed citation edits only.".into(),
                notes: issue.clone(),
            });
            continue;
        }
        if issue.starts_with("reference_pack_primer.") {
            let identifier = issue
                .strip_prefix("reference_pack_primer.")
                .and_then(|rest| rest.split_whitespace().next())
                .unwrap_or("reference_pack_primer")
                .trim_end_matches('.')
                .to_string();
            if identifier.is_empty() {
                continue;
            }
            auto_resolvable_blockers.push(ReviewBlocker {
                scope: ReviewBlockerScope::PrimerSection,
                identifier,
                issue_type: ReviewIssueType::PrimerMissingRequiredSection,
                auto_resolvable: true,
                repair_guidance: "Add or tighten the missing primer section without changing the reviewed values.".into(),
                notes: issue.clone(),
            });
            continue;
        }
        if issue.starts_with("primer_verdicts.") {
            let identifier = issue
                .strip_prefix("primer_verdicts.")
                .and_then(|rest| rest.split_whitespace().next())
                .unwrap_or("primer_verdicts")
                .trim_end_matches('.')
                .to_string();
            if identifier.is_empty() {
                continue;
            }
            manual_required_blockers.push(ReviewBlocker {
                scope: ReviewBlockerScope::Other,
                identifier,
                issue_type: ReviewIssueType::PrimerMissingRequiredSection,
                auto_resolvable: false,
                repair_guidance:
                    "Repair cannot proceed until the verifier artifact includes the missing primer verdict section."
                        .into(),
                notes: issue.clone(),
            });
            continue;
        }
        if issue.contains("schema_change_required") {
            manual_required_blockers.push(ReviewBlocker {
                scope: ReviewBlockerScope::Schema,
                identifier: "schema_change_required".into(),
                issue_type: ReviewIssueType::SchemaMismatch,
                auto_resolvable: false,
                repair_guidance: "Update the contract, validator, and generator together.".into(),
                notes: issue.clone(),
            });
            continue;
        }
        if issue.contains("field_evidence is missing required field path")
            || issue.contains("value_proposal is missing variant")
            || issue.contains("variant ")
            || issue.contains("primer_verdicts.typical_uses is missing")
            || issue.contains(
                "accepted sources do not satisfy authoritative or corroborated status policy",
            )
            || issue.contains("verifier overall verdict is reject")
        {
            manual_required_blockers.push(ReviewBlocker {
                scope: ReviewBlockerScope::Other,
                identifier: issue.clone(),
                issue_type: if issue.contains(
                    "accepted sources do not satisfy authoritative or corroborated status policy",
                ) {
                    ReviewIssueType::SourcePolicyFailure
                } else {
                    ReviewIssueType::ValueMismatch
                },
                auto_resolvable: false,
                repair_guidance: String::new(),
                notes: issue.clone(),
            });
        }
    }

    let all_blockers_auto_resolvable =
        !auto_resolvable_blockers.is_empty() && manual_required_blockers.is_empty();
    let auto_repair_eligible = !auto_resolvable_blockers.is_empty() && all_blockers_auto_resolvable;

    (
        auto_resolvable_blockers,
        manual_required_blockers,
        all_blockers_auto_resolvable,
        auto_repair_eligible,
    )
}

fn has_non_empty_list_items(items: &[String]) -> bool {
    items.iter().any(|item| !item.trim().is_empty())
}

fn required_primer_verdicts(
    verdicts: &PrimerVerdicts,
) -> [(&'static str, Option<&PrimerSectionVerdict>); 5] {
    [
        ("what_this_is", verdicts.what_this_is.as_ref()),
        ("lookup_parameters", verdicts.lookup_parameters.as_ref()),
        (
            "interpretation_notes",
            verdicts.interpretation_notes.as_ref(),
        ),
        ("does_not_include", verdicts.does_not_include.as_ref()),
        ("caveats", verdicts.caveats.as_ref()),
    ]
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
        let normalize_host = |h: &str| h.strip_prefix("www.").unwrap_or(h).to_ascii_lowercase();
        if normalize_host(parsed_host) != normalize_host(&source.host) {
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

fn determine_recommended_action(
    primary: &PrimarySubmission,
    verifier: &VerifierSubmission,
    blocking_issues: &[String],
) -> ReviewRecommendedAction {
    if primary.schema_change_required || verifier.schema_change_required {
        return ReviewRecommendedAction::UpdateContractThenRerunPipeline;
    }

    if verifier.overall_verdict == OverallVerdict::Reject {
        return ReviewRecommendedAction::InvestigateSourcesManually;
    }

    if blocking_issues.is_empty() {
        return ReviewRecommendedAction::ApplyApprovedResult;
    }

    ReviewRecommendedAction::AddressVerifierFeedbackAndRerunReview
}

fn suggested_contract_changes(
    primary: &PrimarySubmission,
    verifier: &VerifierSubmission,
) -> Vec<String> {
    if !primary.schema_change_required && !verifier.schema_change_required {
        return Vec::new();
    }

    let mut suggestions = BTreeSet::new();
    let notes = primary
        .schema_change_notes
        .iter()
        .chain(primary.unresolved_issues.iter())
        .chain(verifier.schema_change_notes.iter())
        .chain(
            verifier
                .field_verdicts
                .iter()
                .map(|verdict| &verdict.notes)
                .filter(|note| !note.trim().is_empty()),
        )
        .chain(std::iter::once(&verifier.notes))
        .map(|note| note.to_ascii_lowercase())
        .collect::<Vec<_>>();

    let combined = notes.join(" | ");

    if (combined.contains("married-filing-separately")
        || combined.contains("married filing separately"))
        && (combined.contains("lived with spouse")
            || combined.contains("lived apart")
            || combined.contains("lived with their spouse"))
    {
        if combined.contains("social security")
            || combined.contains("publication 915")
            || combined.contains("base amount")
            || combined.contains("upper threshold")
        {
            suggestions.insert("Add a Social Security lookup parameter such as `lived_with_spouse_during_year: bool` so `married_filing_separately` can distinguish the lived-apart thresholds from the lived-with-spouse rule.".to_string());
        } else {
            suggestions.insert("Add an IRMAA lookup parameter such as `lived_with_spouse_during_year: bool` so `married_filing_separately` can distinguish the special CMS/SSA table from taxpayers who use the individual-return table.".to_string());
        }
    }

    if combined.contains("boundary semantics")
        || combined.contains("interval semantics")
        || combined.contains("magi_min")
        || combined.contains("magi_max")
        || combined.contains("<=")
        || combined.contains(">=")
    {
        suggestions.insert("Document bracket boundaries explicitly. Minimum viable fix: define `magi_min` as inclusive and `magi_max` as exclusive, with the final bracket open-ended. If exact published semantics matter, extend the schema with boundary inclusivity metadata.".to_string());
    }

    if suggestions.is_empty() {
        for note in primary
            .schema_change_notes
            .iter()
            .chain(primary.unresolved_issues.iter())
            .chain(verifier.schema_change_notes.iter())
        {
            let trimmed = note.trim();
            if !trimmed.is_empty() {
                suggestions.insert(format!("Review and resolve schema note: {trimmed}"));
            }
        }
    }

    suggestions.into_iter().collect()
}

fn action_steps(action: ReviewRecommendedAction) -> Vec<&'static str> {
    match action {
        ReviewRecommendedAction::ApplyApprovedResult => vec![
            "Apply the approved reviewed artifact.",
            "Regenerate the canonical source and snapshot through `apply`.",
        ],
        ReviewRecommendedAction::AddressVerifierFeedbackAndRerunReview => vec![
            "Fix the agent artifacts or source citations that caused the verifier block.",
            "Rerun `review --run <RUN_ID>` after the artifacts are corrected.",
        ],
        ReviewRecommendedAction::UpdateContractThenRerunPipeline => vec![
            "Update the public lookup contract to represent the official rule cleanly.",
            "Adjust the validator, generator, metadata, and tests to match the new contract.",
            "Prepare a fresh run and rerun the pipeline against the updated contract.",
        ],
        ReviewRecommendedAction::InvestigateSourcesManually => vec![
            "Manually resolve the source conflict or contradiction first.",
            "Only rerun the pipeline after the authoritative source set is settled.",
        ],
    }
}

#[allow(clippy::too_many_arguments)]
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
    recommended_action: ReviewRecommendedAction,
    suggested_contract_changes: &[String],
    auto_resolvable_blockers: &[ReviewBlocker],
    manual_required_blockers: &[ReviewBlocker],
    auto_repair_eligible: bool,
    all_blockers_auto_resolvable: bool,
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
        format!(
            "- recommended action: `{}`",
            display_recommended_action(recommended_action)
        ),
        format!("- auto repair eligible: `{}`", auto_repair_eligible),
        format!(
            "- all blockers auto-resolvable: `{}`",
            all_blockers_auto_resolvable
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
    lines.push("## Reference Pack Primer".into());
    if let Some(primer) = primary.reference_pack_primer.as_ref() {
        lines.extend(render_reference_pack_primer_summary(primer));
    } else {
        lines.push("- missing".into());
    }
    lines.push(String::new());
    lines.push("## Primer Review".into());
    lines.extend(render_primer_verdict_summary(primary, verifier));
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
    lines.push("## Auto-Resolvable Blockers".into());
    if auto_resolvable_blockers.is_empty() {
        lines.push("- none".into());
    } else {
        for blocker in auto_resolvable_blockers {
            lines.push(format!(
                "- {} | {} | {}",
                blocker.scope.as_str(),
                blocker.identifier,
                display_review_issue_type(&blocker.issue_type)
            ));
        }
    }
    lines.push(String::new());
    lines.push("## Manual Required Blockers".into());
    if manual_required_blockers.is_empty() {
        lines.push("- none".into());
    } else {
        for blocker in manual_required_blockers {
            lines.push(format!(
                "- {} | {} | {}",
                blocker.scope.as_str(),
                blocker.identifier,
                display_review_issue_type(&blocker.issue_type)
            ));
        }
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
    lines.push("## Suggested Contract Changes".into());
    if suggested_contract_changes.is_empty() {
        lines.push("- none".into());
    } else {
        for change in suggested_contract_changes {
            lines.push(format!("- {}", change));
        }
    }
    lines.push(String::new());
    lines.push("## What To Do Next".into());
    for step in action_steps(recommended_action) {
        lines.push(format!("- {}", step));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn render_reference_pack_primer_summary(primer: &ReferencePackPrimer) -> Vec<String> {
    let mut lines = vec![format!("- what_this_is: {}", primer.what_this_is.trim())];
    lines.extend(render_primer_list_summary(
        "lookup_parameters",
        &primer.lookup_parameters,
    ));
    lines.extend(render_primer_list_summary(
        "interpretation_notes",
        &primer.interpretation_notes,
    ));
    lines.extend(render_primer_list_summary(
        "does_not_include",
        &primer.does_not_include,
    ));
    lines.extend(render_primer_list_summary("caveats", &primer.caveats));
    if has_non_empty_list_items(&primer.typical_uses) {
        lines.extend(render_primer_list_summary(
            "typical_uses",
            &primer.typical_uses,
        ));
    }
    lines
}

fn render_primer_list_summary(label: &str, items: &[String]) -> Vec<String> {
    let mut lines = vec![format!("- {}:", label)];
    let rendered = normalized_primer_list(items);
    if rendered.is_empty() {
        lines.push("  - missing".into());
    } else {
        lines.extend(rendered.into_iter().map(|item| format!("  - {}", item)));
    }
    lines
}

fn render_primer_verdict_summary(
    primary: &PrimarySubmission,
    verifier: &VerifierSubmission,
) -> Vec<String> {
    let mut lines = required_primer_verdicts(&verifier.primer_verdicts)
        .into_iter()
        .map(|(section, verdict)| render_primer_verdict_line(section, verdict))
        .collect::<Vec<_>>();

    if primary
        .reference_pack_primer
        .as_ref()
        .is_some_and(|primer| has_non_empty_list_items(&primer.typical_uses))
    {
        lines.push(render_primer_verdict_line(
            "typical_uses",
            verifier.primer_verdicts.typical_uses.as_ref(),
        ));
    }

    if lines.is_empty() {
        lines.push("- none".into());
    }

    lines
}

fn render_primer_verdict_line(section: &str, verdict: Option<&PrimerSectionVerdict>) -> String {
    match verdict {
        Some(verdict) if verdict.notes.trim().is_empty() => {
            format!("- {}: {}", section, display_field_verdict(&verdict.verdict))
        }
        Some(verdict) => format!(
            "- {}: {} | {}",
            section,
            display_field_verdict(&verdict.verdict),
            verdict.notes.trim()
        ),
        None => format!("- {}: missing", section),
    }
}

fn render_source(
    target_source_path: &Path,
    definition: &PipelineDefinition,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    match definition.generator_kind {
        GeneratorKind::IrmaaRust => render_irmaa_source(reviewed_artifact),
        GeneratorKind::InsuranceMedicareBasePremiumsRust => {
            render_insurance_medicare_base_premiums_source(reviewed_artifact)
        }
        GeneratorKind::SocialSecurityFullRetirementAgeRust => {
            render_social_security_full_retirement_age_source(reviewed_artifact)
        }
        GeneratorKind::TaxFederalBracketsRust => {
            render_tax_federal_brackets_source(target_source_path, definition, reviewed_artifact)
        }
        GeneratorKind::TaxFederalStandardDeductionsRust => {
            render_tax_federal_standard_deductions_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::TaxFederalCapitalGainsBracketsRust => {
            render_tax_federal_capital_gains_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::TaxFederalCapitalLossLimitRust => {
            render_tax_federal_capital_loss_limit_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::TaxFederalNiitRust => {
            render_tax_federal_niit_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::TaxFederalPayrollRust => {
            render_tax_federal_payroll_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::TaxFederalSaltDeductionRust => {
            render_tax_federal_salt_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::TaxFederalQbiRust => {
            render_tax_federal_qbi_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::TaxFederalEstateExemptionRust => {
            render_tax_federal_estate_exemption_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::TaxFederalEstateApplicableCreditRust => {
            render_tax_federal_estate_applicable_credit_source(
                target_source_path,
                reviewed_artifact,
            )
        }
        GeneratorKind::TaxFederalEstateBracketsRust => {
            render_tax_federal_estate_brackets_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::SocialSecurityRetirementEarningsTestRust => {
            render_social_security_retirement_earnings_test_source(reviewed_artifact)
        }
        GeneratorKind::SocialSecurityTaxationRust => {
            render_social_security_taxation_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::RetirementDistributionRulesRust => {
            render_retirement_distribution_rules_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::RetirementUniformLifetimeRust => {
            render_retirement_uniform_lifetime_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::RetirementSingleLifeRust => {
            render_retirement_single_life_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::RetirementJointLifeRust => {
            render_retirement_joint_life_source(target_source_path, reviewed_artifact)
        }
        GeneratorKind::PensionMortality417eRust => {
            render_pension_mortality_417e_source(target_source_path, reviewed_artifact)
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
    output.push_str("use crate::data::types::{DataError, FilingStatus};\n\n");
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
    output.push_str(
        "pub fn brackets(\n    status: FilingStatus,\n    lived_with_spouse_during_year: Option<bool>,\n) -> Result<Vec<IrmaaBracket>, DataError> {\n",
    );
    output.push_str("    match status {\n");
    for label in [
        "single",
        "married_filing_jointly",
        "head_of_household",
        "qualifying_surviving_spouse",
    ] {
        let Some(brackets) = variant_map.get(label) else {
            return Err(PipelineError::new(format!(
                "reviewed IRMAA artifact is missing variant {}",
                label
            )));
        };
        output.push_str(&format!(
            "        {} => Ok(vec![\n",
            filing_status_arm(label)?
        ));
        append_irmaa_brackets(&mut output, brackets, label)?;
        output.push_str("        ]),\n");
    }
    let Some(mfs_lived_with_spouse) =
        variant_map.get("married_filing_separately_lived_with_spouse")
    else {
        return Err(PipelineError::new(
            "reviewed IRMAA artifact is missing variant married_filing_separately_lived_with_spouse",
        ));
    };
    let Some(mfs_lived_apart) = variant_map.get("married_filing_separately_lived_apart") else {
        return Err(PipelineError::new(
            "reviewed IRMAA artifact is missing variant married_filing_separately_lived_apart",
        ));
    };
    output.push_str(
        "        FilingStatus::MarriedFilingSeparately => match lived_with_spouse_during_year {\n",
    );
    output.push_str("            Some(true) => Ok(vec![\n");
    append_irmaa_brackets(
        &mut output,
        mfs_lived_with_spouse,
        "married_filing_separately_lived_with_spouse",
    )?;
    output.push_str("            ]),\n");
    output.push_str("            Some(false) => Ok(vec![\n");
    append_irmaa_brackets(
        &mut output,
        mfs_lived_apart,
        "married_filing_separately_lived_apart",
    )?;
    output.push_str("            ]),\n");
    output.push_str(
        "            None => Err(DataError::InvalidParams(\"lived_with_spouse_during_year parameter is required for married_filing_separately IRMAA lookups\".to_string())),\n",
    );
    output.push_str("        },\n");
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
    output.push_str("        let b = brackets(FilingStatus::Single, None).unwrap();\n");
    output.push_str(&format!(
        "        assert_eq!(b.len(), {});\n",
        variant_map.get("single").map(Vec::len).unwrap_or(0)
    ));
    output.push_str("        assert_eq!(b[0].monthly_surcharge, 0.0);\n");
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn mfj_irmaa_tiers() {\n");
    output
        .push_str("        let b = brackets(FilingStatus::MarriedFilingJointly, None).unwrap();\n");
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
    output.push_str("    fn mfs_lived_with_spouse_only_three_tiers() {\n");
    output.push_str(
        "        let b = brackets(FilingStatus::MarriedFilingSeparately, Some(true)).unwrap();\n",
    );
    output.push_str(&format!(
        "        assert_eq!(b.len(), {});\n",
        variant_map
            .get("married_filing_separately_lived_with_spouse")
            .map(Vec::len)
            .unwrap_or(0)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn mfs_lived_apart_uses_individual_tiers() {\n");
    output.push_str(
        "        let b = brackets(FilingStatus::MarriedFilingSeparately, Some(false)).unwrap();\n",
    );
    output.push_str(&format!(
        "        assert_eq!(b.len(), {});\n",
        variant_map
            .get("married_filing_separately_lived_apart")
            .map(Vec::len)
            .unwrap_or(0)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn mfs_requires_lived_with_spouse_flag() {\n");
    output.push_str(
        "        assert!(brackets(FilingStatus::MarriedFilingSeparately, None).is_err());\n",
    );
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

fn render_insurance_medicare_base_premiums_source(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let params = validated_medicare_base_premiums(reviewed_artifact)?;

    let mut output = String::new();
    output.push_str("/// Medicare base premiums and deductible for 2026, reviewed artifact.\n");
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    output.push_str("pub struct MedicareBasePremiums {\n");
    output.push_str("    pub part_b_standard_monthly_premium: f64,\n");
    output.push_str("    pub part_b_annual_deductible: f64,\n");
    output.push_str("    pub part_d_base_beneficiary_premium: f64,\n");
    output.push_str("}\n\n");
    output.push_str("pub fn base_premiums() -> MedicareBasePremiums {\n");
    output.push_str("    MedicareBasePremiums {\n");
    output.push_str(&format!(
        "        part_b_standard_monthly_premium: {},\n",
        format_f64(params.0)
    ));
    output.push_str(&format!(
        "        part_b_annual_deductible: {},\n",
        format_f64(params.1)
    ));
    output.push_str(&format!(
        "        part_d_base_beneficiary_premium: {},\n",
        format_f64(params.2)
    ));
    output.push_str("    }\n");
    output.push_str("}\n\n");
    output.push_str("#[cfg(test)]\n");
    output.push_str("mod tests {\n");
    output.push_str("    use super::*;\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn medicare_base_premiums_2026() {\n");
    output.push_str("        let premiums = base_premiums();\n");
    output.push_str(&format!(
        "        assert_eq!(premiums.part_b_standard_monthly_premium, {});\n",
        format_f64(params.0)
    ));
    output.push_str(&format!(
        "        assert_eq!(premiums.part_b_annual_deductible, {});\n",
        format_f64(params.1)
    ));
    output.push_str(&format!(
        "        assert_eq!(premiums.part_d_base_beneficiary_premium, {});\n",
        format_f64(params.2)
    ));
    output.push_str("    }\n");
    output.push_str("}\n");

    Ok(output)
}

fn render_social_security_retirement_earnings_test_source(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let params = validated_ss_retirement_earnings_test(reviewed_artifact)?;

    let mut output = String::new();
    output.push_str(
        "/// Social Security retirement earnings test thresholds for 2026, reviewed artifact.\n",
    );
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    output.push_str("pub struct SsEarningsTestThresholds {\n");
    output.push_str("    pub under_fra_annual_exempt_amount: f64,\n");
    output.push_str("    pub under_fra_monthly_exempt_amount: f64,\n");
    output.push_str("    pub year_of_fra_annual_exempt_amount: f64,\n");
    output.push_str("    pub year_of_fra_monthly_exempt_amount: f64,\n");
    output.push_str("    pub under_fra_reduction_rate: f64,\n");
    output.push_str("    pub year_of_fra_reduction_rate: f64,\n");
    output.push_str("}\n\n");
    output.push_str("pub fn thresholds() -> SsEarningsTestThresholds {\n");
    output.push_str("    SsEarningsTestThresholds {\n");
    for (field, value) in &params {
        output.push_str(&format!("        {field}: {},\n", format_f64(*value)));
    }
    output.push_str("    }\n");
    output.push_str("}\n\n");
    output.push_str("#[cfg(test)]\n");
    output.push_str("mod tests {\n");
    output.push_str("    use super::*;\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn earnings_test_thresholds_2026() {\n");
    output.push_str("        let t = thresholds();\n");
    for (field, value) in &params {
        output.push_str(&format!(
            "        assert_eq!(t.{field}, {});\n",
            format_f64(*value)
        ));
    }
    output.push_str("    }\n");
    output.push_str("}\n");

    Ok(output)
}

fn validated_ss_retirement_earnings_test(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<Vec<(&'static str, f64)>, PipelineError> {
    if reviewed_artifact.value.variants.len() != 1 {
        return Err(PipelineError::new(
            "reviewed earnings test artifact must have exactly one variant",
        ));
    }

    let variant = &reviewed_artifact.value.variants[0];
    let value_errors = validate_value(
        "social_security/retirement_earnings_test_thresholds",
        &variant.label,
        &ValidationProfile::SsRetirementEarningsTest,
        &variant.value,
    );
    if !value_errors.is_empty() {
        return Err(PipelineError::new(format!(
            "reviewed earnings test artifact has invalid variant {}: {}",
            variant.label,
            value_errors.join("; ")
        )));
    }

    let Some(obj) = variant.value.as_object() else {
        return Err(PipelineError::new(
            "reviewed earnings test variant must be an object",
        ));
    };

    let fields: &[&str] = &[
        "under_fra_annual_exempt_amount",
        "under_fra_monthly_exempt_amount",
        "year_of_fra_annual_exempt_amount",
        "year_of_fra_monthly_exempt_amount",
        "under_fra_reduction_rate",
        "year_of_fra_reduction_rate",
    ];

    let mut result = Vec::new();
    for field in fields {
        let value = obj.get(*field).and_then(Value::as_f64).ok_or_else(|| {
            PipelineError::new(format!("reviewed earnings test missing {field}"))
        })?;
        result.push((*field, value));
    }

    Ok(result)
}

fn render_social_security_full_retirement_age_source(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let rule_set = validated_social_security_full_retirement_age(reviewed_artifact)?;
    let benefit_scope = rule_set
        .get("benefit_scope")
        .and_then(Value::as_str)
        .ok_or_else(|| PipelineError::new("full retirement age rules missing benefit_scope"))?;
    let january_1_births_use_prior_year = rule_set
        .get("january_1_births_use_prior_year")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            PipelineError::new("full retirement age rules missing january_1_births_use_prior_year")
        })?;
    let rules = rule_set
        .get("rules")
        .and_then(Value::as_array)
        .ok_or_else(|| PipelineError::new("full retirement age rules missing rules"))?;

    let mut output = String::new();
    output.push_str("/// Social Security full retirement age rule for a birth-year range.\n");
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    output.push_str("pub struct FullRetirementAgeRule {\n");
    output.push_str("    pub birth_year_min: Option<u32>,\n");
    output.push_str("    pub birth_year_max: Option<u32>,\n");
    output.push_str("    pub full_retirement_age_years: u32,\n");
    output.push_str("    pub full_retirement_age_months: u32,\n");
    output.push_str("}\n\n");
    output.push_str(
        "/// Social Security full retirement age rules for retirement and spousal benefits.\n",
    );
    output.push_str("#[derive(Debug, Clone, PartialEq, Eq)]\n");
    output.push_str("pub struct FullRetirementAgeRules {\n");
    output.push_str("    pub benefit_scope: &'static str,\n");
    output.push_str("    pub january_1_births_use_prior_year: bool,\n");
    output.push_str("    pub rules: Vec<FullRetirementAgeRule>,\n");
    output.push_str("}\n\n");
    output.push_str("pub fn full_retirement_age_rules() -> FullRetirementAgeRules {\n");
    output.push_str("    FullRetirementAgeRules {\n");
    output.push_str(&format!("        benefit_scope: {:?},\n", benefit_scope));
    output.push_str(&format!(
        "        january_1_births_use_prior_year: {},\n",
        january_1_births_use_prior_year
    ));
    output.push_str("        rules: vec![\n");
    for rule in rules {
        let Some(obj) = rule.as_object() else {
            return Err(PipelineError::new(
                "full retirement age rule is not an object",
            ));
        };
        let birth_year_min = obj.get("birth_year_min").and_then(Value::as_u64);
        let birth_year_max = obj.get("birth_year_max").and_then(Value::as_u64);
        let full_retirement_age_years = obj
            .get("full_retirement_age_years")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                PipelineError::new("full retirement age rule missing full_retirement_age_years")
            })?;
        let full_retirement_age_months = obj
            .get("full_retirement_age_months")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                PipelineError::new("full retirement age rule missing full_retirement_age_months")
            })?;
        output.push_str("            FullRetirementAgeRule {\n");
        output.push_str(&format!(
            "                birth_year_min: {},\n",
            format_option_u64(birth_year_min)
        ));
        output.push_str(&format!(
            "                birth_year_max: {},\n",
            format_option_u64(birth_year_max)
        ));
        output.push_str(&format!(
            "                full_retirement_age_years: {},\n",
            full_retirement_age_years
        ));
        output.push_str(&format!(
            "                full_retirement_age_months: {},\n",
            full_retirement_age_months
        ));
        output.push_str("            },\n");
    }
    output.push_str("        ],\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    output.push_str(
        "pub fn full_retirement_age_for_birth_year(birth_year: u32) -> Option<(u32, u32)> {\n",
    );
    output.push_str("    let rules = full_retirement_age_rules();\n");
    output.push_str("    rules.rules.into_iter().find_map(|rule| {\n");
    output.push_str(
        "        let min_ok = rule.birth_year_min.is_none_or(|min| birth_year >= min);\n",
    );
    output.push_str(
        "        let max_ok = rule.birth_year_max.is_none_or(|max| birth_year <= max);\n",
    );
    output.push_str("        if min_ok && max_ok {\n");
    output.push_str(
        "            Some((rule.full_retirement_age_years, rule.full_retirement_age_months))\n",
    );
    output.push_str("        } else {\n");
    output.push_str("            None\n");
    output.push_str("        }\n");
    output.push_str("    })\n");
    output.push_str("}\n\n");
    output.push_str("#[cfg(test)]\n");
    output.push_str("mod tests {\n");
    output.push_str("    use super::*;\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn fra_rules_include_january_1_note() {\n");
    output.push_str("        let rules = full_retirement_age_rules();\n");
    output.push_str(&format!(
        "        assert_eq!(rules.january_1_births_use_prior_year, {});\n",
        january_1_births_use_prior_year
    ));
    output.push_str(&format!(
        "        assert_eq!(rules.benefit_scope, {:?});\n",
        benefit_scope
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn fra_birth_year_1937() {\n");
    output
        .push_str("        assert_eq!(full_retirement_age_for_birth_year(1937), Some((65, 0)));\n");
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn fra_birth_year_1959() {\n");
    output.push_str(
        "        assert_eq!(full_retirement_age_for_birth_year(1959), Some((66, 10)));\n",
    );
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn fra_birth_year_1960_and_later() {\n");
    output
        .push_str("        assert_eq!(full_retirement_age_for_birth_year(1960), Some((67, 0)));\n");
    output
        .push_str("        assert_eq!(full_retirement_age_for_birth_year(1985), Some((67, 0)));\n");
    output.push_str("    }\n");
    output.push_str("}\n");

    Ok(output)
}

fn append_irmaa_brackets(
    output: &mut String,
    brackets: &[Value],
    label: &str,
) -> Result<(), PipelineError> {
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
    Ok(())
}

fn render_tax_federal_brackets_source(
    target_source_path: &Path,
    definition: &PipelineDefinition,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let mut year_variant_maps = BTreeMap::new();

    for year in &definition.supported_years {
        let variant_map = if *year == reviewed_artifact.year {
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
            variant_map
        } else {
            let current_value = build_current_value(
                *year,
                &definition.category,
                &definition.key,
                &definition.expected_variants,
            )?;
            let mut variant_map = BTreeMap::new();
            for variant in current_value.variants {
                let Some(brackets) = variant.value.as_array() else {
                    return Err(PipelineError::new(format!(
                        "current tax brackets variant {} must be an array",
                        variant.label
                    )));
                };
                variant_map.insert(variant.label, brackets.clone());
            }
            variant_map
        };
        year_variant_maps.insert(*year, variant_map);
    }

    let section = render_tax_federal_brackets_section(&year_variant_maps)?;
    replace_source_section(
        &existing,
        "// ---------------------------------------------------------------------------\n// Federal income tax brackets",
        "// ---------------------------------------------------------------------------\n// Standard deductions",
        &section,
    )
}

fn render_tax_federal_capital_gains_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map =
        validated_tax_bracket_variant_map("tax/federal_capital_gains_brackets", reviewed_artifact)?;
    let section = render_tax_federal_capital_gains_section(&variant_map)?;
    replace_source_section(
        &existing,
        "// ---------------------------------------------------------------------------\n// Capital gains brackets",
        "// ---------------------------------------------------------------------------\n// Net Investment Income Tax",
        &section,
    )
}

fn render_tax_federal_standard_deductions_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map = validated_numeric_field_variant_map(
        "tax/federal_standard_deductions",
        reviewed_artifact,
        "amount",
    )?;
    let section = render_tax_federal_standard_deductions_section(&variant_map)?;
    replace_source_section(
        &existing,
        "// ---------------------------------------------------------------------------\n// Standard deductions",
        "// ---------------------------------------------------------------------------\n// Capital gains brackets",
        &section,
    )
}

fn render_tax_federal_capital_loss_limit_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map = validated_numeric_field_variant_map(
        "tax/federal_capital_loss_limit",
        reviewed_artifact,
        "limit",
    )?;
    let section = render_tax_federal_capital_loss_limit_section(&variant_map)?;
    let output = replace_source_section(
        &existing,
        "pub fn capital_loss_limit(status: FilingStatus) -> f64 {",
        "// ---------------------------------------------------------------------------\n// SALT deduction parameters",
        &section,
    )?;
    let tests = render_tax_federal_capital_loss_limit_tests(&variant_map)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn capital_loss_limit_mfs() {",
        "    #[test]\n    fn salt_deduction_parameters_single() {",
        &tests,
    )
}

fn render_tax_federal_niit_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map = validated_niit_variant_map(reviewed_artifact)?;
    let section = render_tax_federal_niit_section(&variant_map)?;
    let output = replace_source_section(
        &existing,
        "pub fn niit(status: FilingStatus) -> NiitParams {",
        "pub fn payroll(status: FilingStatus) -> PayrollParams {",
        &section,
    )?;
    let tests = render_tax_federal_niit_tests(&variant_map)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn niit_thresholds() {",
        "    #[test]\n    fn payroll_ss_wage_base_2026() {",
        &tests,
    )
}

fn render_tax_federal_payroll_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map = validated_payroll_variant_map(reviewed_artifact)?;
    let section = render_tax_federal_payroll_section(&variant_map)?;
    let output = replace_source_section(
        &existing,
        "pub fn payroll(status: FilingStatus) -> PayrollParams {",
        "// ---------------------------------------------------------------------------\n// Capital loss limit",
        &section,
    )?;
    let tests = render_tax_federal_payroll_tests(&variant_map)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn payroll_ss_wage_base_2026() {",
        "    #[test]\n    fn salt_deduction_parameters_single() {",
        &tests,
    )
}

fn render_tax_federal_salt_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map = validated_salt_variant_map(reviewed_artifact)?;
    let section = render_tax_federal_salt_section(&variant_map)?;
    let output = replace_source_section(
        &existing,
        "// ---------------------------------------------------------------------------\n// SALT deduction parameters",
        "// ---------------------------------------------------------------------------\n// QBI Deduction parameters",
        &section,
    )?;
    let tests = render_tax_federal_salt_tests(&variant_map)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn salt_deduction_parameters_single() {",
        "    #[test]\n    fn qbi_deduction_mfj() {",
        &tests,
    )
}

fn render_tax_federal_qbi_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map = validated_qbi_variant_map(reviewed_artifact)?;
    let section = render_tax_federal_qbi_section(&variant_map)?;
    let output = replace_source_section(
        &existing,
        "pub fn qbi_deduction(status: FilingStatus) -> QbiDeductionParams {",
        "// ---------------------------------------------------------------------------\n// Tests",
        &section,
    )?;
    let tests = render_tax_federal_qbi_tests(&variant_map)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn qbi_deduction_mfj() {",
        "    #[test]\n    fn qss_equals_mfj() {",
        &tests,
    )
}

fn render_tax_federal_estate_exemption_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map = validated_numeric_field_variant_map(
        "tax/federal_estate_exemption",
        reviewed_artifact,
        "exemption",
    )?;
    let section = render_tax_federal_estate_exemption_section(&variant_map)?;
    let output = replace_source_section(
        &existing,
        "pub fn exemption() -> f64 {",
        "pub fn applicable_credit() -> f64 {",
        &section,
    )?;
    let tests = render_tax_federal_estate_exemption_tests(&variant_map)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn exemption_2026() {",
        "    #[test]\n    fn applicable_credit_2026() {",
        &tests,
    )
}

fn render_tax_federal_estate_applicable_credit_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map = validated_numeric_field_variant_map(
        "tax/federal_estate_applicable_credit",
        reviewed_artifact,
        "applicable_credit",
    )?;
    let section = render_tax_federal_estate_applicable_credit_section(&variant_map)?;
    let output = replace_source_section(
        &existing,
        "pub fn applicable_credit() -> f64 {",
        "pub fn brackets() -> Vec<TaxBracket> {",
        &section,
    )?;
    let tests = render_tax_federal_estate_applicable_credit_tests(&variant_map)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn applicable_credit_2026() {",
        "    #[test]\n    fn bracket_count() {",
        &tests,
    )
}

fn render_tax_federal_estate_brackets_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map =
        validated_tax_bracket_variant_map("tax/federal_estate_brackets", reviewed_artifact)?;
    let section = render_tax_federal_estate_brackets_section(&variant_map)?;
    let output = replace_source_section(
        &existing,
        "pub fn brackets() -> Vec<TaxBracket> {",
        "#[cfg(test)]",
        &section,
    )?;
    let tests = render_tax_federal_estate_brackets_tests(&variant_map)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn bracket_count() {",
        "    #[test]\n    fn brackets_contiguous() {",
        &tests,
    )
}

fn render_social_security_taxation_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let variant_map = validated_ss_taxation_variant_map(reviewed_artifact)?;
    let section = render_social_security_taxation_section(&variant_map)?;
    let output = replace_source_section(
        &existing,
        "pub fn thresholds(\n    status: FilingStatus,\n    lived_with_spouse_during_year: Option<bool>,\n) -> Result<SsTaxationThresholds, DataError> {\n",
        "#[cfg(test)]",
        &section,
    )?;
    let tests = render_social_security_taxation_tests(&variant_map)?;
    replace_source_section(
        &output,
        "    // BEGIN GENERATED TAXATION TESTS\n",
        "    // END GENERATED TAXATION TESTS\n",
        &tests,
    )
}

fn render_retirement_distribution_rules_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let value = validated_distribution_rules_value(reviewed_artifact)?;
    let section = render_retirement_distribution_rules_section(&value)?;
    replace_source_section(
        &existing,
        "pub fn distribution_rules() -> RmdParameters {\n",
        "#[cfg(test)]",
        &section,
    )
}

fn render_retirement_uniform_lifetime_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let rows =
        validated_age_distribution_rows("retirement/uniform_lifetime_table", reviewed_artifact)?;
    let section = render_retirement_uniform_lifetime_section(&rows);
    let output = replace_source_section(
        &existing,
        "// ---------------------------------------------------------------------------\n// Uniform Lifetime Table",
        "// ---------------------------------------------------------------------------\n// Single Life Expectancy Table",
        &section,
    )?;
    let tests = render_retirement_uniform_lifetime_tests(&rows)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn uniform_lifetime_age_72() {",
        "    #[test]\n    fn single_life_starts_at_zero() {",
        &tests,
    )
}

fn render_retirement_single_life_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let rows = validated_age_distribution_rows("retirement/single_life_table", reviewed_artifact)?;
    let section = render_retirement_single_life_section(&rows);
    let output = replace_source_section(
        &existing,
        "// ---------------------------------------------------------------------------\n// Single Life Expectancy Table",
        "// ---------------------------------------------------------------------------\n// Joint Life and Last Survivor Table",
        &section,
    )?;
    let tests = render_retirement_single_life_tests(&rows)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn single_life_starts_at_zero() {",
        "    #[test]\n    fn joint_life_has_entries() {",
        &tests,
    )
}

fn render_retirement_joint_life_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let rows = validated_joint_distribution_rows("retirement/joint_life_table", reviewed_artifact)?;
    let section = render_retirement_joint_life_section(&rows);
    replace_source_section(
        &existing,
        "// ---------------------------------------------------------------------------\n// Joint Life and Last Survivor Table",
        "#[cfg(test)]",
        &section,
    )
}

fn render_pension_mortality_417e_source(
    target_source_path: &Path,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<String, PipelineError> {
    let existing = fs::read_to_string(target_source_path)?;
    let rows = validated_mortality_rows("pension/mortality_417e", reviewed_artifact)?;
    let section = render_pension_mortality_417e_section(&rows);
    let output = replace_source_section(
        &existing,
        "pub fn table_417e() -> Vec<MortalityEntry> {\n",
        "#[cfg(test)]",
        &section,
    )?;
    let tests = render_pension_mortality_417e_tests(&rows)?;
    replace_source_section(
        &output,
        "    #[test]\n    fn table_417e_range() {",
        "    #[test]\n    fn qx_increases_with_age() {",
        &tests,
    )
}

fn validated_age_distribution_rows(
    entry_path: &str,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<Vec<(u32, f64)>, PipelineError> {
    if reviewed_artifact.value.variants.len() != 1 {
        return Err(PipelineError::new(format!(
            "reviewed {} artifact must contain exactly one variant",
            entry_path
        )));
    }

    let variant = &reviewed_artifact.value.variants[0];
    let value_errors = validate_value(
        entry_path,
        &variant.label,
        &ValidationProfile::AgeDistribution,
        &variant.value,
    );
    if !value_errors.is_empty() {
        return Err(PipelineError::new(format!(
            "reviewed {} artifact has invalid variant {}: {}",
            entry_path,
            variant.label,
            value_errors.join("; ")
        )));
    }

    let Some(items) = variant.value.as_array() else {
        return Err(PipelineError::new(format!(
            "reviewed {} variant {} must be an array",
            entry_path, variant.label
        )));
    };

    let mut rows = Vec::with_capacity(items.len());
    for item in items {
        let Some(obj) = item.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed {} rows must be objects",
                entry_path
            )));
        };
        let age = obj
            .get("age")
            .and_then(Value::as_u64)
            .ok_or_else(|| PipelineError::new("age distribution row is missing age"))?;
        if age > u32::MAX as u64 {
            return Err(PipelineError::new(format!(
                "age distribution row age {} exceeds u32 range",
                age
            )));
        }
        let distribution_period = obj
            .get("distribution_period")
            .and_then(Value::as_f64)
            .ok_or_else(|| {
                PipelineError::new("age distribution row is missing distribution_period")
            })?;
        rows.push((age as u32, distribution_period));
    }

    Ok(rows)
}

fn validated_joint_distribution_rows(
    entry_path: &str,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<Vec<(u32, u32, f64)>, PipelineError> {
    if reviewed_artifact.value.variants.len() != 1 {
        return Err(PipelineError::new(format!(
            "reviewed {} artifact must contain exactly one variant",
            entry_path
        )));
    }

    let variant = &reviewed_artifact.value.variants[0];
    let value_errors = validate_value(
        entry_path,
        &variant.label,
        &ValidationProfile::JointDistribution,
        &variant.value,
    );
    if !value_errors.is_empty() {
        return Err(PipelineError::new(format!(
            "reviewed {} artifact has invalid variant {}: {}",
            entry_path,
            variant.label,
            value_errors.join("; ")
        )));
    }

    let Some(items) = variant.value.as_array() else {
        return Err(PipelineError::new(format!(
            "reviewed {} variant {} must be an array",
            entry_path, variant.label
        )));
    };

    let mut rows = Vec::with_capacity(items.len());
    for item in items {
        let Some(obj) = item.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed {} rows must be objects",
                entry_path
            )));
        };
        let owner_age = obj
            .get("owner_age")
            .and_then(Value::as_u64)
            .ok_or_else(|| PipelineError::new("joint distribution row is missing owner_age"))?;
        let spouse_age = obj
            .get("spouse_age")
            .and_then(Value::as_u64)
            .ok_or_else(|| PipelineError::new("joint distribution row is missing spouse_age"))?;
        if owner_age > u32::MAX as u64 || spouse_age > u32::MAX as u64 {
            return Err(PipelineError::new(
                "joint distribution row age exceeds u32 range",
            ));
        }
        let distribution_period = obj
            .get("distribution_period")
            .and_then(Value::as_f64)
            .ok_or_else(|| {
                PipelineError::new("joint distribution row is missing distribution_period")
            })?;
        rows.push((owner_age as u32, spouse_age as u32, distribution_period));
    }

    Ok(rows)
}

fn validated_mortality_rows(
    entry_path: &str,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<Vec<(u32, f64)>, PipelineError> {
    if reviewed_artifact.value.variants.len() != 1 {
        return Err(PipelineError::new(format!(
            "reviewed {} artifact must contain exactly one variant",
            entry_path
        )));
    }

    let variant = &reviewed_artifact.value.variants[0];
    let value_errors = validate_value(
        entry_path,
        &variant.label,
        &ValidationProfile::MortalityQx,
        &variant.value,
    );
    if !value_errors.is_empty() {
        return Err(PipelineError::new(format!(
            "reviewed {} artifact has invalid variant {}: {}",
            entry_path,
            variant.label,
            value_errors.join("; ")
        )));
    }

    let Some(items) = variant.value.as_array() else {
        return Err(PipelineError::new(format!(
            "reviewed {} variant {} must be an array",
            entry_path, variant.label
        )));
    };

    let mut rows = Vec::with_capacity(items.len());
    for item in items {
        let Some(obj) = item.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed {} rows must be objects",
                entry_path
            )));
        };
        let age = obj
            .get("age")
            .and_then(Value::as_u64)
            .ok_or_else(|| PipelineError::new("mortality row is missing age"))?;
        if age > u32::MAX as u64 {
            return Err(PipelineError::new(format!(
                "mortality row age {} exceeds u32 range",
                age
            )));
        }
        let qx = obj
            .get("qx")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("mortality row is missing qx"))?;
        rows.push((age as u32, qx));
    }

    Ok(rows)
}

fn render_retirement_uniform_lifetime_section(rows: &[(u32, f64)]) -> String {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n",
    );
    output.push_str(
        "// Uniform Lifetime Table (Table III) — IRS Pub 590-B (2026, reviewed artifact)\n",
    );
    output.push_str(
        "// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn uniform_lifetime() -> Vec<AgeDistributionPeriod> {\n");
    output.push_str("    vec![\n");
    for (age, distribution_period) in rows {
        output.push_str("        AgeDistributionPeriod {\n");
        output.push_str(&format!("            age: {},\n", age));
        output.push_str(&format!(
            "            distribution_period: {},\n",
            format_f64(*distribution_period)
        ));
        output.push_str("        },\n");
    }
    output.push_str("    ]\n");
    output.push_str("}\n\n");
    output
}

fn render_retirement_single_life_section(rows: &[(u32, f64)]) -> String {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n",
    );
    output.push_str(
        "// Single Life Expectancy Table (Table I) — IRS Pub 590-B (2026, reviewed artifact)\n",
    );
    output.push_str(
        "// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn single_life() -> Vec<AgeDistributionPeriod> {\n");
    output.push_str("    vec![\n");
    for (age, distribution_period) in rows {
        output.push_str("        AgeDistributionPeriod {\n");
        output.push_str(&format!("            age: {},\n", age));
        output.push_str(&format!(
            "            distribution_period: {},\n",
            format_f64(*distribution_period)
        ));
        output.push_str("        },\n");
    }
    output.push_str("    ]\n");
    output.push_str("}\n\n");
    output
}

fn render_retirement_joint_life_section(rows: &[(u32, u32, f64)]) -> String {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n",
    );
    output.push_str(
        "// Joint Life and Last Survivor Table (Table II) — IRS Pub 590-B (2026, reviewed artifact)\n",
    );
    output.push_str(
        "// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn joint_life() -> Vec<JointDistributionPeriod> {\n");
    output.push_str("    vec![\n");
    for (owner_age, spouse_age, distribution_period) in rows {
        output.push_str("        JointDistributionPeriod {\n");
        output.push_str(&format!("            owner_age: {},\n", owner_age));
        output.push_str(&format!("            spouse_age: {},\n", spouse_age));
        output.push_str(&format!(
            "            distribution_period: {},\n",
            format_f64(*distribution_period)
        ));
        output.push_str("        },\n");
    }
    output.push_str("    ]\n");
    output.push_str("}\n\n");
    output
}

fn render_pension_mortality_417e_section(rows: &[(u32, f64)]) -> String {
    let mut output = String::new();
    output.push_str("pub fn table_417e() -> Vec<MortalityEntry> {\n");
    output.push_str("    vec![\n");
    for (age, qx) in rows {
        output.push_str("        MortalityEntry {\n");
        output.push_str(&format!("            age: {},\n", age));
        output.push_str(&format!("            qx: {},\n", format_qx(*qx)));
        output.push_str("        },\n");
    }
    output.push_str("    ]\n");
    output.push_str("}\n\n");
    output
}

fn render_retirement_uniform_lifetime_tests(rows: &[(u32, f64)]) -> Result<String, PipelineError> {
    let first = rows
        .first()
        .ok_or_else(|| PipelineError::new("uniform lifetime table must have at least one row"))?;
    let last = rows
        .last()
        .ok_or_else(|| PipelineError::new("uniform lifetime table must have at least one row"))?;
    let age_72 = rows
        .iter()
        .find(|(age, _)| *age == 72)
        .copied()
        .unwrap_or(*first);

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn uniform_lifetime_age_72() {\n");
    output.push_str("        let table = uniform_lifetime();\n");
    output.push_str(&format!(
        "        let entry = table.iter().find(|e| e.age == {}).unwrap();\n",
        age_72.0
    ));
    output.push_str(&format!(
        "        assert_eq!(entry.distribution_period, {});\n",
        format_f64(age_72.1)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn uniform_lifetime_last_age() {\n");
    output.push_str("        let table = uniform_lifetime();\n");
    output.push_str("        let entry = table.last().unwrap();\n");
    output.push_str(&format!("        assert_eq!(entry.age, {});\n", last.0));
    output.push_str(&format!(
        "        assert_eq!(entry.distribution_period, {});\n",
        format_f64(last.1)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn uniform_lifetime_range() {\n");
    output.push_str("        let table = uniform_lifetime();\n");
    output.push_str(&format!(
        "        assert_eq!(table.first().unwrap().age, {});\n",
        first.0
    ));
    output.push_str(&format!(
        "        assert_eq!(table.last().unwrap().age, {});\n",
        last.0
    ));
    output.push_str(&format!(
        "        assert_eq!(table.len(), {});\n",
        rows.len()
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_retirement_single_life_tests(rows: &[(u32, f64)]) -> Result<String, PipelineError> {
    let first = rows
        .first()
        .ok_or_else(|| PipelineError::new("single life table must have at least one row"))?;
    let last = rows
        .last()
        .ok_or_else(|| PipelineError::new("single life table must have at least one row"))?;
    let age_72 = rows
        .iter()
        .find(|(age, _)| *age == 72)
        .copied()
        .unwrap_or(*first);

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn single_life_starts_at_zero() {\n");
    output.push_str("        let table = single_life();\n");
    output.push_str(&format!(
        "        assert_eq!(table.first().unwrap().age, {});\n",
        first.0
    ));
    output.push_str(&format!(
        "        assert_eq!(table.first().unwrap().distribution_period, {});\n",
        format_f64(first.1)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn single_life_age_72() {\n");
    output.push_str("        let table = single_life();\n");
    output.push_str(&format!(
        "        let entry = table.iter().find(|e| e.age == {}).unwrap();\n",
        age_72.0
    ));
    output.push_str(&format!(
        "        assert_eq!(entry.distribution_period, {});\n",
        format_f64(age_72.1)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn single_life_ends_at_last_age() {\n");
    output.push_str("        let table = single_life();\n");
    output.push_str("        let last = table.last().unwrap();\n");
    output.push_str(&format!("        assert_eq!(last.age, {});\n", last.0));
    output.push_str(&format!(
        "        assert_eq!(last.distribution_period, {});\n",
        format_f64(last.1)
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_pension_mortality_417e_tests(rows: &[(u32, f64)]) -> Result<String, PipelineError> {
    let first = rows
        .first()
        .ok_or_else(|| PipelineError::new("417(e) mortality table must have at least one row"))?;
    let last = rows
        .last()
        .ok_or_else(|| PipelineError::new("417(e) mortality table must have at least one row"))?;
    let sample = rows
        .iter()
        .find(|(age, _)| *age == 65)
        .copied()
        .unwrap_or(rows[rows.len() / 2]);

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn table_417e_range() {\n");
    output.push_str("        let table = table_417e();\n");
    output.push_str(&format!(
        "        assert_eq!(table.first().unwrap().age, {});\n",
        first.0
    ));
    output.push_str(&format!(
        "        assert_eq!(table.last().unwrap().age, {});\n",
        last.0
    ));
    output.push_str(&format!(
        "        assert_eq!(table.len(), {});\n",
        rows.len()
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn table_417e_key_values() {\n");
    output.push_str("        let table = table_417e();\n");
    output.push_str(
        "        let find = |age: u32| table.iter().find(|e| e.age == age).unwrap().qx;\n\n",
    );
    output.push_str(&format!(
        "        assert!((find({}) - {}).abs() < 1e-6);\n",
        first.0,
        format_qx(first.1)
    ));
    output.push_str(&format!(
        "        assert!((find({}) - {}).abs() < 1e-6);\n",
        sample.0,
        format_qx(sample.1)
    ));
    output.push_str(&format!(
        "        assert!((find({}) - {}).abs() < 1e-6);\n",
        last.0,
        format_qx(last.1)
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn validated_tax_bracket_variant_map(
    entry_path: &str,
    reviewed_artifact: &ReviewedArtifact,
) -> Result<BTreeMap<String, Vec<Value>>, PipelineError> {
    let mut variant_map = BTreeMap::new();

    for variant in &reviewed_artifact.value.variants {
        let value_errors = validate_value(
            entry_path,
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

    Ok(variant_map)
}

fn validated_numeric_field_variant_map(
    entry_path: &str,
    reviewed_artifact: &ReviewedArtifact,
    field: &str,
) -> Result<BTreeMap<String, f64>, PipelineError> {
    let mut variant_map = BTreeMap::new();

    for variant in &reviewed_artifact.value.variants {
        let value_errors = validate_value(
            entry_path,
            &variant.label,
            &ValidationProfile::NumericField {
                field: field.to_string(),
            },
            &variant.value,
        );
        if !value_errors.is_empty() {
            return Err(PipelineError::new(format!(
                "reviewed numeric-field artifact has invalid variant {}: {}",
                variant.label,
                value_errors.join("; ")
            )));
        }
        let Some(obj) = variant.value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed numeric-field variant {} must be an object",
                variant.label
            )));
        };
        let Some(number) = obj.get(field).and_then(Value::as_f64) else {
            return Err(PipelineError::new(format!(
                "reviewed numeric-field variant {} is missing {}",
                variant.label, field
            )));
        };
        variant_map.insert(variant.label.clone(), number);
    }

    Ok(variant_map)
}

fn validated_niit_variant_map(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<BTreeMap<String, (f64, f64)>, PipelineError> {
    let mut variant_map = BTreeMap::new();

    for variant in &reviewed_artifact.value.variants {
        let value_errors = validate_value(
            "tax/federal_net_investment_income_tax",
            &variant.label,
            &ValidationProfile::Niit,
            &variant.value,
        );
        if !value_errors.is_empty() {
            return Err(PipelineError::new(format!(
                "reviewed NIIT artifact has invalid variant {}: {}",
                variant.label,
                value_errors.join("; ")
            )));
        }
        let Some(obj) = variant.value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed NIIT variant {} must be an object",
                variant.label
            )));
        };
        let Some(rate) = obj.get("rate").and_then(Value::as_f64) else {
            return Err(PipelineError::new(format!(
                "reviewed NIIT variant {} is missing rate",
                variant.label
            )));
        };
        let Some(threshold) = obj.get("threshold").and_then(Value::as_f64) else {
            return Err(PipelineError::new(format!(
                "reviewed NIIT variant {} is missing threshold",
                variant.label
            )));
        };
        variant_map.insert(variant.label.clone(), (rate, threshold));
    }

    Ok(variant_map)
}

fn validated_payroll_variant_map(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<BTreeMap<String, Value>, PipelineError> {
    let mut variant_map = BTreeMap::new();

    for variant in &reviewed_artifact.value.variants {
        let value_errors = validate_value(
            "tax/federal_payroll_tax_parameters",
            &variant.label,
            &ValidationProfile::Payroll,
            &variant.value,
        );
        if !value_errors.is_empty() {
            return Err(PipelineError::new(format!(
                "reviewed payroll artifact has invalid variant {}: {}",
                variant.label,
                value_errors.join("; ")
            )));
        }
        let Some(obj) = variant.value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed payroll variant {} must be an object",
                variant.label
            )));
        };
        variant_map.insert(variant.label.clone(), Value::Object(obj.clone()));
    }

    Ok(variant_map)
}

fn validated_medicare_base_premiums(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<(f64, f64, f64), PipelineError> {
    if reviewed_artifact.value.variants.len() != 1 {
        return Err(PipelineError::new(
            "reviewed Medicare base premiums artifact must have exactly one variant",
        ));
    }

    let variant = &reviewed_artifact.value.variants[0];
    let value_errors = validate_value(
        "insurance/medicare_base_premiums",
        &variant.label,
        &ValidationProfile::MedicareBasePremiums,
        &variant.value,
    );
    if !value_errors.is_empty() {
        return Err(PipelineError::new(format!(
            "reviewed Medicare base premiums artifact has invalid variant {}: {}",
            variant.label,
            value_errors.join("; ")
        )));
    }

    let Some(obj) = variant.value.as_object() else {
        return Err(PipelineError::new(
            "reviewed Medicare base premiums variant must be an object",
        ));
    };
    let Some(part_b_standard_monthly_premium) = obj
        .get("part_b_standard_monthly_premium")
        .and_then(Value::as_f64)
    else {
        return Err(PipelineError::new(
            "reviewed Medicare base premiums variant is missing part_b_standard_monthly_premium",
        ));
    };
    let Some(part_b_annual_deductible) =
        obj.get("part_b_annual_deductible").and_then(Value::as_f64)
    else {
        return Err(PipelineError::new(
            "reviewed Medicare base premiums variant is missing part_b_annual_deductible",
        ));
    };
    let Some(part_d_base_beneficiary_premium) = obj
        .get("part_d_base_beneficiary_premium")
        .and_then(Value::as_f64)
    else {
        return Err(PipelineError::new(
            "reviewed Medicare base premiums variant is missing part_d_base_beneficiary_premium",
        ));
    };

    Ok((
        part_b_standard_monthly_premium,
        part_b_annual_deductible,
        part_d_base_beneficiary_premium,
    ))
}

fn validated_social_security_full_retirement_age(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<serde_json::Map<String, Value>, PipelineError> {
    if reviewed_artifact.value.variants.len() != 1 {
        return Err(PipelineError::new(
            "reviewed full retirement age artifact must have exactly one variant",
        ));
    }

    let variant = &reviewed_artifact.value.variants[0];
    let value_errors = validate_value(
        "social_security/full_retirement_age_rules",
        &variant.label,
        &ValidationProfile::SocialSecurityFullRetirementAge,
        &variant.value,
    );
    if !value_errors.is_empty() {
        return Err(PipelineError::new(format!(
            "reviewed full retirement age artifact has invalid variant {}: {}",
            variant.label,
            value_errors.join("; ")
        )));
    }

    let Some(obj) = variant.value.as_object() else {
        return Err(PipelineError::new(
            "reviewed full retirement age variant must be an object",
        ));
    };

    Ok(obj.clone())
}

fn validated_salt_variant_map(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<BTreeMap<String, Value>, PipelineError> {
    let mut variant_map = BTreeMap::new();

    for variant in &reviewed_artifact.value.variants {
        let value_errors = validate_value(
            "tax/federal_salt_deduction_parameters",
            &variant.label,
            &ValidationProfile::Salt,
            &variant.value,
        );
        if !value_errors.is_empty() {
            return Err(PipelineError::new(format!(
                "reviewed SALT artifact has invalid variant {}: {}",
                variant.label,
                value_errors.join("; ")
            )));
        }
        let Some(obj) = variant.value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed SALT variant {} must be an object",
                variant.label
            )));
        };
        variant_map.insert(variant.label.clone(), Value::Object(obj.clone()));
    }

    Ok(variant_map)
}

fn validated_qbi_variant_map(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<BTreeMap<String, Value>, PipelineError> {
    let mut variant_map = BTreeMap::new();

    for variant in &reviewed_artifact.value.variants {
        let value_errors = validate_value(
            "tax/federal_qbi_deduction",
            &variant.label,
            &ValidationProfile::Qbi,
            &variant.value,
        );
        if !value_errors.is_empty() {
            return Err(PipelineError::new(format!(
                "reviewed QBI artifact has invalid variant {}: {}",
                variant.label,
                value_errors.join("; ")
            )));
        }
        let Some(obj) = variant.value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed QBI variant {} must be an object",
                variant.label
            )));
        };
        variant_map.insert(variant.label.clone(), Value::Object(obj.clone()));
    }

    Ok(variant_map)
}

fn validated_ss_taxation_variant_map(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<BTreeMap<String, Value>, PipelineError> {
    let mut variant_map = BTreeMap::new();

    for variant in &reviewed_artifact.value.variants {
        let value_errors = validate_value(
            "social_security/benefit_taxation_thresholds",
            &variant.label,
            &ValidationProfile::SsTaxation,
            &variant.value,
        );
        if !value_errors.is_empty() {
            return Err(PipelineError::new(format!(
                "reviewed Social Security taxation artifact has invalid variant {}: {}",
                variant.label,
                value_errors.join("; ")
            )));
        }
        let Some(obj) = variant.value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed Social Security taxation variant {} must be an object",
                variant.label
            )));
        };
        variant_map.insert(variant.label.clone(), Value::Object(obj.clone()));
    }

    Ok(variant_map)
}

fn validated_distribution_rules_value(
    reviewed_artifact: &ReviewedArtifact,
) -> Result<serde_json::Map<String, Value>, PipelineError> {
    if reviewed_artifact.value.variants.len() != 1 {
        return Err(PipelineError::new(
            "reviewed distribution rules artifact must contain exactly one variant",
        ));
    }

    let variant = &reviewed_artifact.value.variants[0];
    if variant.label != "default" {
        return Err(PipelineError::new(format!(
            "reviewed distribution rules artifact expected default variant, found {}",
            variant.label
        )));
    }

    let value_errors = validate_value(
        "retirement/distribution_rules",
        &variant.label,
        &ValidationProfile::DistributionRules,
        &variant.value,
    );
    if !value_errors.is_empty() {
        return Err(PipelineError::new(format!(
            "reviewed distribution rules artifact has invalid variant {}: {}",
            variant.label,
            value_errors.join("; ")
        )));
    }

    let Some(obj) = variant.value.as_object() else {
        return Err(PipelineError::new(
            "reviewed distribution rules variant must be an object",
        ));
    };
    Ok(obj.clone())
}

fn render_tax_federal_brackets_section(
    year_variant_maps: &BTreeMap<u32, BTreeMap<String, Vec<Value>>>,
) -> Result<String, PipelineError> {
    let default_year = *year_variant_maps
        .keys()
        .max()
        .ok_or_else(|| PipelineError::new("tax brackets section is missing supported years"))?;
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n\
// Federal income tax brackets (2025-2026, reviewed artifacts)\n\
// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn brackets(status: FilingStatus) -> Vec<TaxBracket> {\n");
    output.push_str(&format!(
        "    brackets_for_year({}, status).expect(\"default federal tax brackets year is supported\")\n",
        default_year
    ));
    output.push_str("}\n\n");
    output.push_str(
        "pub fn brackets_for_year(year: u32, status: FilingStatus) -> Result<Vec<TaxBracket>, DataError> {\n",
    );
    output.push_str("    match year {\n");
    for year in year_variant_maps.keys() {
        output.push_str(&format!(
            "        {} => Ok(brackets_{}(status)),\n",
            year, year
        ));
    }
    output.push_str("        _ => Err(DataError::UnsupportedYear(year)),\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    for (year, variant_map) in year_variant_maps {
        output.push_str(&format!(
            "fn brackets_{}(status: FilingStatus) -> Vec<TaxBracket> {{\n",
            year
        ));
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
                    "tax brackets artifact for {} is missing variant {}",
                    year, label
                )));
            };
            output.push_str(&format!("        {} => vec![\n", filing_status_arm(label)?));
            for bracket in brackets {
                let Some(obj) = bracket.as_object() else {
                    return Err(PipelineError::new(format!(
                        "tax bracket for {} {} is not an object",
                        year, label
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
    }

    Ok(output)
}

fn render_tax_federal_standard_deductions_section(
    variant_map: &BTreeMap<String, f64>,
) -> Result<String, PipelineError> {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n\
// Standard deductions (2026, reviewed artifact)\n\
// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn standard_deductions(status: FilingStatus) -> f64 {\n");
    output.push_str("    match status {\n");
    for label in [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ] {
        let Some(amount) = variant_map.get(label) else {
            return Err(PipelineError::new(format!(
                "reviewed standard deductions artifact is missing variant {}",
                label
            )));
        };
        output.push_str(&format!(
            "        {} => {},\n",
            filing_status_arm(label)?,
            format_f64(*amount)
        ));
    }
    output.push_str("    }\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn render_tax_federal_capital_loss_limit_section(
    variant_map: &BTreeMap<String, f64>,
) -> Result<String, PipelineError> {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n\
// Capital loss limit (2026, reviewed artifact)\n\
// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn capital_loss_limit(status: FilingStatus) -> f64 {\n");
    output.push_str("    match status {\n");
    for label in [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ] {
        let Some(limit) = variant_map.get(label) else {
            return Err(PipelineError::new(format!(
                "reviewed capital loss limit artifact is missing variant {}",
                label
            )));
        };
        output.push_str(&format!(
            "        {} => {},\n",
            filing_status_arm(label)?,
            format_f64(*limit)
        ));
    }
    output.push_str("    }\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn render_tax_federal_niit_section(
    variant_map: &BTreeMap<String, (f64, f64)>,
) -> Result<String, PipelineError> {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n\
// Net Investment Income Tax (2026, reviewed artifact)\n\
// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn niit(status: FilingStatus) -> NiitParams {\n");
    output.push_str("    let (rate, threshold) = match status {\n");
    for label in [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ] {
        let Some((rate, threshold)) = variant_map.get(label) else {
            return Err(PipelineError::new(format!(
                "reviewed NIIT artifact is missing variant {}",
                label
            )));
        };
        output.push_str(&format!(
            "        {} => ({}, {}),\n",
            filing_status_arm(label)?,
            format_f64(*rate),
            format_f64(*threshold)
        ));
    }
    output.push_str("    };\n");
    output.push_str("    NiitParams { rate, threshold }\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn render_tax_federal_payroll_section(
    variant_map: &BTreeMap<String, Value>,
) -> Result<String, PipelineError> {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n\
// Payroll tax parameters (2026, reviewed artifact)\n\
// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn payroll(status: FilingStatus) -> PayrollParams {\n");
    output.push_str("    match status {\n");
    for label in [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ] {
        let Some(value) = variant_map.get(label) else {
            return Err(PipelineError::new(format!(
                "reviewed payroll artifact is missing variant {}",
                label
            )));
        };
        let Some(obj) = value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed payroll variant {} must be an object",
                label
            )));
        };
        let social_security_rate = obj
            .get("social_security_rate")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("payroll missing social_security_rate"))?;
        let social_security_wage_base = obj
            .get("social_security_wage_base")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("payroll missing social_security_wage_base"))?;
        let self_employment_tax_rate = obj
            .get("self_employment_tax_rate")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("payroll missing self_employment_tax_rate"))?;
        let medicare_rate = obj
            .get("medicare_rate")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("payroll missing medicare_rate"))?;
        let self_employment_medicare_rate = obj
            .get("self_employment_medicare_rate")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("payroll missing self_employment_medicare_rate"))?;
        let additional_medicare_rate = obj
            .get("additional_medicare_rate")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("payroll missing additional_medicare_rate"))?;
        let additional_medicare_threshold = obj
            .get("additional_medicare_threshold")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("payroll missing additional_medicare_threshold"))?;

        output.push_str(&format!(
            "        {} => PayrollParams {{\n",
            filing_status_arm(label)?
        ));
        output.push_str(&format!(
            "            social_security_rate: {},\n",
            format_f64(social_security_rate)
        ));
        output.push_str(&format!(
            "            social_security_wage_base: {},\n",
            format_f64(social_security_wage_base)
        ));
        output.push_str(&format!(
            "            self_employment_tax_rate: {},\n",
            format_f64(self_employment_tax_rate)
        ));
        output.push_str(&format!(
            "            medicare_rate: {},\n",
            format_f64(medicare_rate)
        ));
        output.push_str(&format!(
            "            self_employment_medicare_rate: {},\n",
            format_f64(self_employment_medicare_rate)
        ));
        output.push_str(&format!(
            "            additional_medicare_rate: {},\n",
            format_f64(additional_medicare_rate)
        ));
        output.push_str(&format!(
            "            additional_medicare_threshold: {},\n",
            format_f64(additional_medicare_threshold)
        ));
        output.push_str("        },\n");
    }
    output.push_str("    }\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn render_tax_federal_salt_section(
    variant_map: &BTreeMap<String, Value>,
) -> Result<String, PipelineError> {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n\
// SALT deduction parameters (2026, reviewed artifact)\n\
// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str(
        "pub fn salt_deduction_parameters(status: FilingStatus) -> SaltDeductionParams {\n",
    );
    output.push_str("    match status {\n");
    for label in [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ] {
        let Some(value) = variant_map.get(label) else {
            return Err(PipelineError::new(format!(
                "reviewed SALT artifact is missing variant {}",
                label
            )));
        };
        let Some(obj) = value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed SALT variant {} must be an object",
                label
            )));
        };
        let cap_amount = obj
            .get("cap_amount")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("SALT params missing cap_amount"))?;
        let phaseout_threshold = obj
            .get("phaseout_threshold")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("SALT params missing phaseout_threshold"))?;
        let phaseout_rate = obj
            .get("phaseout_rate")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("SALT params missing phaseout_rate"))?;
        let floor_amount = obj
            .get("floor_amount")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("SALT params missing floor_amount"))?;

        output.push_str(&format!(
            "        {} => SaltDeductionParams {{\n",
            filing_status_arm(label)?
        ));
        output.push_str(&format!(
            "            cap_amount: {},\n",
            format_f64(cap_amount)
        ));
        output.push_str(&format!(
            "            phaseout_threshold: {},\n",
            format_f64(phaseout_threshold)
        ));
        output.push_str(&format!(
            "            phaseout_rate: {},\n",
            format_f64(phaseout_rate)
        ));
        output.push_str(&format!(
            "            floor_amount: {},\n",
            format_f64(floor_amount)
        ));
        output.push_str("        },\n");
    }
    output.push_str("    }\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn render_tax_federal_qbi_section(
    variant_map: &BTreeMap<String, Value>,
) -> Result<String, PipelineError> {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n\
// QBI Deduction parameters (Section 199A, 2026, reviewed artifact)\n\
// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn qbi_deduction(status: FilingStatus) -> QbiDeductionParams {\n");
    output.push_str("    match status {\n");
    for label in [
        "single",
        "married_filing_jointly",
        "married_filing_separately",
        "head_of_household",
        "qualifying_surviving_spouse",
    ] {
        let Some(value) = variant_map.get(label) else {
            return Err(PipelineError::new(format!(
                "reviewed QBI artifact is missing variant {}",
                label
            )));
        };
        let Some(obj) = value.as_object() else {
            return Err(PipelineError::new(format!(
                "reviewed QBI variant {} must be an object",
                label
            )));
        };
        let deduction_rate = obj
            .get("deduction_rate")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("qbi missing deduction_rate"))?;
        let threshold = obj
            .get("threshold")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("qbi missing threshold"))?;
        let phase_in_range_end = obj
            .get("phase_in_range_end")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("qbi missing phase_in_range_end"))?;
        let minimum_qbi_deduction = obj
            .get("minimum_qbi_deduction")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("qbi missing minimum_qbi_deduction"))?;
        let minimum_qbi_amount = obj
            .get("minimum_qbi_amount")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("qbi missing minimum_qbi_amount"))?;

        output.push_str(&format!(
            "        {} => QbiDeductionParams {{\n",
            filing_status_arm(label)?
        ));
        output.push_str(&format!(
            "            deduction_rate: {},\n",
            format_f64(deduction_rate)
        ));
        output.push_str(&format!(
            "            threshold: {},\n",
            format_f64(threshold)
        ));
        output.push_str(&format!(
            "            phase_in_range_end: {},\n",
            format_f64(phase_in_range_end)
        ));
        output.push_str(&format!(
            "            minimum_qbi_deduction: {},\n",
            format_f64(minimum_qbi_deduction)
        ));
        output.push_str(&format!(
            "            minimum_qbi_amount: {},\n",
            format_f64(minimum_qbi_amount)
        ));
        output.push_str("        },\n");
    }
    output.push_str("    }\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn render_social_security_taxation_section(
    variant_map: &BTreeMap<String, Value>,
) -> Result<String, PipelineError> {
    let load_params = |label: &str| -> Result<(f64, f64, f64, f64), PipelineError> {
        let value = variant_map.get(label).ok_or_else(|| {
            PipelineError::new(format!(
                "reviewed Social Security taxation artifact is missing variant {}",
                label
            ))
        })?;
        let obj = value.as_object().ok_or_else(|| {
            PipelineError::new(format!(
                "reviewed Social Security taxation variant {} must be an object",
                label
            ))
        })?;
        let base_amount = obj
            .get("base_amount")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("ss taxation missing base_amount"))?;
        let upper_amount = obj
            .get("upper_amount")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("ss taxation missing upper_amount"))?;
        let below_pct = obj
            .get("max_taxable_pct_below_upper")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("ss taxation missing max_taxable_pct_below_upper"))?;
        let above_pct = obj
            .get("max_taxable_pct_above_upper")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("ss taxation missing max_taxable_pct_above_upper"))?;
        Ok((base_amount, upper_amount, below_pct, above_pct))
    };

    let single = load_params("single")?;
    let mfj = load_params("married_filing_jointly")?;
    let mfs_lived_with_spouse = load_params("married_filing_separately_lived_with_spouse")?;
    let mfs_lived_apart = load_params("married_filing_separately_lived_apart")?;
    let hoh = load_params("head_of_household")?;
    let qss = load_params("qualifying_surviving_spouse")?;

    let mut output = String::new();
    output.push_str("// Social Security benefit taxation thresholds (2026, reviewed artifact).\n");
    output.push_str(
        "pub fn thresholds(\n    status: FilingStatus,\n    lived_with_spouse_during_year: Option<bool>,\n) -> Result<SsTaxationThresholds, DataError> {\n",
    );
    output.push_str("    match status {\n");
    for (label, params) in [
        ("single", single),
        ("married_filing_jointly", mfj),
        ("head_of_household", hoh),
        ("qualifying_surviving_spouse", qss),
    ] {
        output.push_str(&format!(
            "        {} => Ok(SsTaxationThresholds {{\n",
            filing_status_arm(label)?
        ));
        output.push_str(&format!(
            "            base_amount: {},\n",
            format_f64(params.0)
        ));
        output.push_str(&format!(
            "            upper_amount: {},\n",
            format_f64(params.1)
        ));
        output.push_str(&format!(
            "            max_taxable_pct_below_upper: {},\n",
            format_f64(params.2)
        ));
        output.push_str(&format!(
            "            max_taxable_pct_above_upper: {},\n",
            format_f64(params.3)
        ));
        output.push_str("        }),\n");
    }
    output.push_str(
        "        FilingStatus::MarriedFilingSeparately => match lived_with_spouse_during_year {\n",
    );
    output.push_str("            Some(true) => Ok(SsTaxationThresholds {\n");
    output.push_str(&format!(
        "                base_amount: {},\n",
        format_f64(mfs_lived_with_spouse.0)
    ));
    output.push_str(&format!(
        "                upper_amount: {},\n",
        format_f64(mfs_lived_with_spouse.1)
    ));
    output.push_str(&format!(
        "                max_taxable_pct_below_upper: {},\n",
        format_f64(mfs_lived_with_spouse.2)
    ));
    output.push_str(&format!(
        "                max_taxable_pct_above_upper: {},\n",
        format_f64(mfs_lived_with_spouse.3)
    ));
    output.push_str("            }),\n");
    output.push_str("            Some(false) => Ok(SsTaxationThresholds {\n");
    output.push_str(&format!(
        "                base_amount: {},\n",
        format_f64(mfs_lived_apart.0)
    ));
    output.push_str(&format!(
        "                upper_amount: {},\n",
        format_f64(mfs_lived_apart.1)
    ));
    output.push_str(&format!(
        "                max_taxable_pct_below_upper: {},\n",
        format_f64(mfs_lived_apart.2)
    ));
    output.push_str(&format!(
        "                max_taxable_pct_above_upper: {},\n",
        format_f64(mfs_lived_apart.3)
    ));
    output.push_str("            }),\n");
    output.push_str("            None => Err(DataError::InvalidParams(\"lived_with_spouse_during_year parameter is required for married_filing_separately Social Security benefit taxation lookups\".to_string())),\n");
    output.push_str("        },\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn render_retirement_distribution_rules_section(
    value: &serde_json::Map<String, Value>,
) -> Result<String, PipelineError> {
    let required_beginning = value
        .get("required_beginning")
        .and_then(Value::as_object)
        .ok_or_else(|| PipelineError::new("distribution rules missing required_beginning"))?;
    let account_applicability = value
        .get("account_applicability")
        .and_then(Value::as_object)
        .ok_or_else(|| PipelineError::new("distribution rules missing account_applicability"))?;
    let beneficiary_distribution = value
        .get("beneficiary_distribution")
        .and_then(Value::as_object)
        .ok_or_else(|| PipelineError::new("distribution rules missing beneficiary_distribution"))?;

    let start_age_rules = required_beginning
        .get("start_age_rules")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new("distribution rules missing required_beginning.start_age_rules")
        })?;
    let first_distribution_deadline = required_beginning
        .get("first_distribution_deadline")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing required_beginning.first_distribution_deadline",
            )
        })?;
    let still_working_exception = required_beginning
        .get("still_working_exception")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing required_beginning.still_working_exception",
            )
        })?;
    let still_working_plan_categories = still_working_exception
        .get("eligible_plan_categories")
        .and_then(Value::as_array)
        .ok_or_else(|| PipelineError::new("distribution rules missing required_beginning.still_working_exception.eligible_plan_categories"))?;
    let still_working_account_types = still_working_exception
        .get("eligible_account_types")
        .and_then(Value::as_array)
        .ok_or_else(|| PipelineError::new("distribution rules missing required_beginning.still_working_exception.eligible_account_types"))?;
    let still_working_disallowed_for_five_percent_owners = still_working_exception
        .get("disallowed_for_five_percent_owners")
        .and_then(Value::as_bool)
        .ok_or_else(|| PipelineError::new("distribution rules missing required_beginning.still_working_exception.disallowed_for_five_percent_owners"))?;
    let owner_required_account_types = account_applicability
        .get("owner_required_account_types")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing account_applicability.owner_required_account_types",
            )
        })?;
    let owner_exempt_account_types = account_applicability
        .get("owner_exempt_account_types")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing account_applicability.owner_exempt_account_types",
            )
        })?;
    let inherited_account_types = account_applicability
        .get("inherited_account_types")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing account_applicability.inherited_account_types",
            )
        })?;
    let supports_pre_1987_403b_exclusion = account_applicability
        .get("supports_pre_1987_403b_exclusion")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing account_applicability.supports_pre_1987_403b_exclusion",
            )
        })?;
    let designated_roth_owner_exemption_effective_year = account_applicability
        .get("designated_roth_owner_exemption_effective_year")
        .and_then(Value::as_u64);
    let pre_1987_exclude_until_age = account_applicability
        .get("pre_1987_403b")
        .and_then(Value::as_object)
        .and_then(|obj| obj.get("exclude_until_age"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing account_applicability.pre_1987_403b.exclude_until_age",
            )
        })?;

    let beneficiary_categories = beneficiary_distribution
        .get("beneficiary_categories")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing beneficiary_distribution.beneficiary_categories",
            )
        })?;
    let recognized_beneficiary_classes = beneficiary_distribution
        .get("recognized_beneficiary_classes")
        .and_then(Value::as_array)
        .ok_or_else(|| PipelineError::new("distribution rules missing beneficiary_distribution.recognized_beneficiary_classes"))?;
    let eligible_designated_beneficiary_classes = beneficiary_distribution
        .get("eligible_designated_beneficiary_classes")
        .and_then(Value::as_array)
        .ok_or_else(|| PipelineError::new("distribution rules missing beneficiary_distribution.eligible_designated_beneficiary_classes"))?;
    let life_expectancy_method_by_class = beneficiary_distribution
        .get("life_expectancy_method_by_class")
        .and_then(Value::as_object)
        .ok_or_else(|| PipelineError::new("distribution rules missing beneficiary_distribution.life_expectancy_method_by_class"))?;
    let minor_child_majority_age = beneficiary_distribution
        .get("minor_child_majority_age")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing beneficiary_distribution.minor_child_majority_age",
            )
        })?;
    let spouse_delay_allowed = beneficiary_distribution
        .get("spouse_delay_allowed")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing beneficiary_distribution.spouse_delay_allowed",
            )
        })?;
    let ten_year_rule = beneficiary_distribution
        .get("ten_year_rule")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new("distribution rules missing beneficiary_distribution.ten_year_rule")
        })?;
    let non_designated_beneficiary_rules = beneficiary_distribution
        .get("non_designated_beneficiary_rules")
        .and_then(Value::as_object)
        .ok_or_else(|| PipelineError::new("distribution rules missing beneficiary_distribution.non_designated_beneficiary_rules"))?;
    let relief_years = beneficiary_distribution
        .get("relief_years")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new("distribution rules missing beneficiary_distribution.relief_years")
        })?;

    let terminal_year = ten_year_rule
        .get("terminal_year")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new(
                "distribution rules missing beneficiary_distribution.ten_year_rule.terminal_year",
            )
        })?;
    let annual_distributions_required = ten_year_rule
        .get("annual_distributions_required_when_owner_died_on_or_after_rbd")
        .and_then(Value::as_bool)
        .ok_or_else(|| PipelineError::new("distribution rules missing beneficiary_distribution.ten_year_rule.annual_distributions_required_when_owner_died_on_or_after_rbd"))?;
    let non_designated_before_rbd = non_designated_beneficiary_rules
        .get("when_owner_died_before_required_beginning_date")
        .and_then(Value::as_str)
        .ok_or_else(|| PipelineError::new("distribution rules missing beneficiary_distribution.non_designated_beneficiary_rules.when_owner_died_before_required_beginning_date"))?;
    let non_designated_on_or_after_rbd = non_designated_beneficiary_rules
        .get("when_owner_died_on_or_after_required_beginning_date")
        .and_then(Value::as_str)
        .ok_or_else(|| PipelineError::new("distribution rules missing beneficiary_distribution.non_designated_beneficiary_rules.when_owner_died_on_or_after_required_beginning_date"))?;

    let mut output = String::new();
    output
        .push_str("/// Build the full RMD parameter set for 2026, compatible with the existing\n");
    output.push_str("/// `RmdParameters` struct in `models/retirement_rmd.rs`.\n");
    output.push_str("pub fn distribution_rules() -> RmdParameters {\n");
    output.push_str("    let mut life_expectancy_methods = HashMap::new();\n");
    let mut method_entries = life_expectancy_method_by_class.iter().collect::<Vec<_>>();
    method_entries.sort_by(|a, b| a.0.cmp(b.0));
    for (class, method) in method_entries {
        let method = method.as_str().ok_or_else(|| {
            PipelineError::new(
                "beneficiary_distribution.life_expectancy_method_by_class values must be strings",
            )
        })?;
        output.push_str(&format!(
            "    life_expectancy_methods.insert(\"{}\".to_string(), \"{}\".to_string());\n",
            escape_rust_string(class),
            escape_rust_string(method)
        ));
    }
    output.push('\n');
    output.push_str("    RmdParameters {\n");
    output.push_str("        uniform_lifetime_table: rmd_tables::uniform_lifetime(),\n");
    output.push_str("        joint_life_table: rmd_tables::joint_life(),\n");
    output.push_str("        single_life_table: rmd_tables::single_life(),\n");
    output.push_str("        required_beginning: RequiredBeginningRules {\n");
    output.push_str("            start_age_rules: vec![\n");
    for rule in start_age_rules {
        let obj = rule.as_object().ok_or_else(|| {
            PipelineError::new("required_beginning.start_age_rules entries must be objects")
        })?;
        let birth_year_min = obj.get("birth_year_min").and_then(Value::as_u64);
        let birth_year_max = obj.get("birth_year_max").and_then(Value::as_u64);
        let start_age = obj
            .get("start_age")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                PipelineError::new("required_beginning.start_age_rules entry missing start_age")
            })?;
        output.push_str("                StartAgeRule {\n");
        output.push_str(&format!(
            "                    birth_year_min: {},\n",
            format_option_u64(birth_year_min)
        ));
        output.push_str(&format!(
            "                    birth_year_max: {},\n",
            format_option_u64(birth_year_max)
        ));
        output.push_str(&format!("                    start_age: {},\n", start_age));
        match obj.get("guidance_status").and_then(Value::as_str) {
            Some(value) => output.push_str(&format!(
                "                    guidance_status: Some(\"{}\".to_string()),\n",
                escape_rust_string(value)
            )),
            None => output.push_str("                    guidance_status: None,\n"),
        }
        match obj.get("notes").and_then(Value::as_str) {
            Some(value) => output.push_str(&format!(
                "                    notes: Some(\"{}\".to_string()),\n",
                escape_rust_string(value)
            )),
            None => output.push_str("                    notes: None,\n"),
        }
        output.push_str("                },\n");
    }
    output.push_str("            ],\n");
    output.push_str(&format!(
        "            first_distribution_deadline: \"{}\".to_string(),\n",
        escape_rust_string(first_distribution_deadline)
    ));
    output.push_str("            still_working_exception_plan_categories: vec![\n");
    for value in still_working_plan_categories {
        let value = value.as_str().ok_or_else(|| {
            PipelineError::new(
                "required_beginning.still_working_exception.eligible_plan_categories entries must be strings",
            )
        })?;
        output.push_str(&format!(
            "                \"{}\".to_string(),\n",
            escape_rust_string(value)
        ));
    }
    output.push_str("            ],\n");
    output.push_str("            still_working_exception_eligible_account_types: vec![\n");
    for value in still_working_account_types {
        let value = value.as_str().ok_or_else(|| {
            PipelineError::new(
                "required_beginning.still_working_exception.eligible_account_types entries must be strings",
            )
        })?;
        output.push_str(&format!(
            "                \"{}\".to_string(),\n",
            escape_rust_string(value)
        ));
    }
    output.push_str("            ],\n");
    output.push_str(&format!(
        "            still_working_exception_disallowed_for_five_percent_owners: {},\n",
        still_working_disallowed_for_five_percent_owners
    ));
    output.push_str("        },\n");
    output.push_str("        account_rules: AccountRules {\n");
    output.push_str("            owner_required_account_types: vec![\n");
    for value in owner_required_account_types {
        let value = value.as_str().ok_or_else(|| {
            PipelineError::new(
                "account_applicability.owner_required_account_types entries must be strings",
            )
        })?;
        output.push_str(&format!(
            "                \"{}\".to_string(),\n",
            escape_rust_string(value)
        ));
    }
    output.push_str("            ],\n");
    output.push_str("            owner_exempt_account_types: vec![\n");
    for value in owner_exempt_account_types {
        let value = value.as_str().ok_or_else(|| {
            PipelineError::new(
                "account_applicability.owner_exempt_account_types entries must be strings",
            )
        })?;
        output.push_str(&format!(
            "                \"{}\".to_string(),\n",
            escape_rust_string(value)
        ));
    }
    output.push_str("            ],\n");
    output.push_str("            inherited_account_types: vec![\n");
    for value in inherited_account_types {
        let value = value.as_str().ok_or_else(|| {
            PipelineError::new(
                "account_applicability.inherited_account_types entries must be strings",
            )
        })?;
        output.push_str(&format!(
            "                \"{}\".to_string(),\n",
            escape_rust_string(value)
        ));
    }
    output.push_str("            ],\n");
    output.push_str(&format!(
        "            supports_pre_1987_403b_exclusion: {},\n",
        supports_pre_1987_403b_exclusion
    ));
    output.push_str(&format!(
        "            designated_roth_owner_exemption_effective_year: {},\n",
        format_option_u64(designated_roth_owner_exemption_effective_year)
    ));
    output.push_str("        },\n");
    output.push_str("        beneficiary_rules: BeneficiaryRules {\n");
    output.push_str("            beneficiary_categories: vec![\n");
    for value in beneficiary_categories {
        let value = value.as_str().ok_or_else(|| {
            PipelineError::new(
                "beneficiary_distribution.beneficiary_categories entries must be strings",
            )
        })?;
        output.push_str(&format!(
            "                \"{}\".to_string(),\n",
            escape_rust_string(value)
        ));
    }
    output.push_str("            ],\n");
    output.push_str("            recognized_beneficiary_classes: vec![\n");
    for value in recognized_beneficiary_classes {
        let value = value.as_str().ok_or_else(|| {
            PipelineError::new(
                "beneficiary_distribution.recognized_beneficiary_classes entries must be strings",
            )
        })?;
        output.push_str(&format!(
            "                \"{}\".to_string(),\n",
            escape_rust_string(value)
        ));
    }
    output.push_str("            ],\n");
    output.push_str("            eligible_designated_beneficiary_classes: vec![\n");
    for value in eligible_designated_beneficiary_classes {
        let value = value.as_str().ok_or_else(|| {
            PipelineError::new(
                "beneficiary_distribution.eligible_designated_beneficiary_classes entries must be strings",
            )
        })?;
        output.push_str(&format!(
            "                \"{}\".to_string(),\n",
            escape_rust_string(value)
        ));
    }
    output.push_str("            ],\n");
    output.push_str("            life_expectancy_method_by_class: life_expectancy_methods,\n");
    output.push_str(&format!(
        "            minor_child_majority_age: {},\n",
        minor_child_majority_age
    ));
    output.push_str(&format!(
        "            spouse_delay_allowed: {},\n",
        spouse_delay_allowed
    ));
    output.push_str(
        "            non_designated_beneficiary_rules: NonDesignatedBeneficiaryRules {\n",
    );
    output.push_str(&format!(
        "                when_owner_died_before_required_beginning_date: \"{}\".to_string(),\n",
        escape_rust_string(non_designated_before_rbd)
    ));
    output.push_str(&format!(
        "                when_owner_died_on_or_after_required_beginning_date: \"{}\".to_string(),\n",
        escape_rust_string(non_designated_on_or_after_rbd)
    ));
    output.push_str("            },\n");
    output.push_str("        },\n");
    output.push_str("        ten_year_rule: TenYearRule {\n");
    output.push_str(&format!("            terminal_year: {},\n", terminal_year));
    output.push_str(&format!(
        "            annual_distributions_required_when_owner_died_on_or_after_rbd: {},\n",
        annual_distributions_required
    ));
    output.push_str("        },\n");
    output.push_str("        relief_years: vec![");
    for (index, year) in relief_years.iter().enumerate() {
        let year = year.as_u64().ok_or_else(|| {
            PipelineError::new("beneficiary_distribution.relief_years entries must be numbers")
        })?;
        if index > 0 {
            output.push_str(", ");
        }
        output.push_str(&year.to_string());
    }
    output.push_str("],\n");
    output.push_str("        pre_1987_403b_rules: Pre1987Rules {\n");
    output.push_str(&format!(
        "            exclude_until_age: {},\n",
        pre_1987_exclude_until_age
    ));
    output.push_str("        },\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn render_tax_federal_capital_loss_limit_tests(
    variant_map: &BTreeMap<String, f64>,
) -> Result<String, PipelineError> {
    let single = variant_map.get("single").ok_or_else(|| {
        PipelineError::new("reviewed capital loss limit artifact is missing variant single")
    })?;
    let mfs = variant_map
        .get("married_filing_separately")
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed capital loss limit artifact is missing variant married_filing_separately",
            )
        })?;

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn capital_loss_limit_mfs() {\n");
    output.push_str(&format!(
        "        assert_eq!(capital_loss_limit(FilingStatus::MarriedFilingSeparately), {});\n",
        format_f64(*mfs)
    ));
    output.push_str(&format!(
        "        assert_eq!(capital_loss_limit(FilingStatus::Single), {});\n",
        format_f64(*single)
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_tax_federal_niit_tests(
    variant_map: &BTreeMap<String, (f64, f64)>,
) -> Result<String, PipelineError> {
    let single = variant_map
        .get("single")
        .ok_or_else(|| PipelineError::new("reviewed NIIT artifact is missing variant single"))?;
    let mfj = variant_map.get("married_filing_jointly").ok_or_else(|| {
        PipelineError::new("reviewed NIIT artifact is missing variant married_filing_jointly")
    })?;
    let mfs = variant_map
        .get("married_filing_separately")
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed NIIT artifact is missing variant married_filing_separately",
            )
        })?;

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn niit_thresholds() {\n");
    output.push_str(&format!(
        "        assert_eq!(niit(FilingStatus::Single).threshold, {});\n",
        format_f64(single.1)
    ));
    output.push_str(&format!(
        "        assert_eq!(niit(FilingStatus::MarriedFilingJointly).threshold, {});\n",
        format_f64(mfj.1)
    ));
    output.push_str(&format!(
        "        assert_eq!(niit(FilingStatus::MarriedFilingSeparately).threshold, {});\n",
        format_f64(mfs.1)
    ));
    output.push_str(&format!(
        "        assert_eq!(niit(FilingStatus::Single).rate, {});\n",
        format_f64(single.0)
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_tax_federal_payroll_tests(
    variant_map: &BTreeMap<String, Value>,
) -> Result<String, PipelineError> {
    let single = variant_map
        .get("single")
        .and_then(Value::as_object)
        .ok_or_else(|| PipelineError::new("reviewed payroll artifact is missing variant single"))?;
    let social_security_wage_base = single
        .get("social_security_wage_base")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed payroll artifact missing wage base"))?;
    let social_security_rate = single
        .get("social_security_rate")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed payroll artifact missing rate"))?;

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn payroll_ss_wage_base_2026() {\n");
    output.push_str("        let p = payroll(FilingStatus::Single);\n");
    output.push_str(&format!(
        "        assert_eq!(p.social_security_wage_base, {});\n",
        format_f64(social_security_wage_base)
    ));
    output.push_str(&format!(
        "        assert_eq!(p.social_security_rate, {});\n",
        format_f64(social_security_rate)
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_tax_federal_salt_tests(
    variant_map: &BTreeMap<String, Value>,
) -> Result<String, PipelineError> {
    let single = variant_map
        .get("single")
        .and_then(Value::as_object)
        .ok_or_else(|| PipelineError::new("reviewed SALT artifact is missing variant single"))?;
    let mfs = variant_map
        .get("married_filing_separately")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed SALT artifact is missing variant married_filing_separately",
            )
        })?;

    let single_cap = single
        .get("cap_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed SALT artifact missing single cap_amount"))?;
    let single_threshold = single
        .get("phaseout_threshold")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new("reviewed SALT artifact missing single phaseout_threshold")
        })?;
    let single_rate = single
        .get("phaseout_rate")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed SALT artifact missing single phaseout_rate"))?;
    let single_floor = single
        .get("floor_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed SALT artifact missing single floor_amount"))?;
    let mfs_cap = mfs
        .get("cap_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed SALT artifact missing mfs cap_amount"))?;
    let mfs_floor = mfs
        .get("floor_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed SALT artifact missing mfs floor_amount"))?;

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn salt_deduction_parameters_single() {\n");
    output.push_str("        let salt = salt_deduction_parameters(FilingStatus::Single);\n");
    output.push_str(&format!(
        "        assert_eq!(salt.cap_amount, {});\n",
        format_f64(single_cap)
    ));
    output.push_str(&format!(
        "        assert_eq!(salt.phaseout_threshold, {});\n",
        format_f64(single_threshold)
    ));
    output.push_str(&format!(
        "        assert_eq!(salt.phaseout_rate, {});\n",
        format_f64(single_rate)
    ));
    output.push_str(&format!(
        "        assert_eq!(salt.floor_amount, {});\n",
        format_f64(single_floor)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn salt_deduction_parameters_mfs() {\n");
    output.push_str(
        "        let salt = salt_deduction_parameters(FilingStatus::MarriedFilingSeparately);\n",
    );
    output.push_str(&format!(
        "        assert_eq!(salt.cap_amount, {});\n",
        format_f64(mfs_cap)
    ));
    output.push_str(&format!(
        "        assert_eq!(salt.floor_amount, {});\n",
        format_f64(mfs_floor)
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_social_security_taxation_tests(
    variant_map: &BTreeMap<String, Value>,
) -> Result<String, PipelineError> {
    let single = variant_map
        .get("single")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact is missing variant single",
            )
        })?;
    let mfj = variant_map
        .get("married_filing_jointly")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact is missing variant married_filing_jointly",
            )
        })?;
    let mfs_lived_with_spouse = variant_map
        .get("married_filing_separately_lived_with_spouse")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact is missing variant married_filing_separately_lived_with_spouse",
            )
        })?;
    let mfs_lived_apart = variant_map
        .get("married_filing_separately_lived_apart")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact is missing variant married_filing_separately_lived_apart",
            )
        })?;

    let single_base = single
        .get("base_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact missing single base_amount",
            )
        })?;
    let single_upper = single
        .get("upper_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact missing single upper_amount",
            )
        })?;
    let mfj_base = mfj
        .get("base_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new("reviewed Social Security taxation artifact missing mfj base_amount")
        })?;
    let mfj_upper = mfj
        .get("upper_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact missing mfj upper_amount",
            )
        })?;
    let mfs_lived_with_spouse_base = mfs_lived_with_spouse
        .get("base_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact missing mfs_lived_with_spouse base_amount",
            )
        })?;
    let mfs_lived_with_spouse_upper = mfs_lived_with_spouse
        .get("upper_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact missing mfs_lived_with_spouse upper_amount",
            )
        })?;
    let mfs_lived_apart_base = mfs_lived_apart
        .get("base_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact missing mfs_lived_apart base_amount",
            )
        })?;
    let mfs_lived_apart_upper = mfs_lived_apart
        .get("upper_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new(
                "reviewed Social Security taxation artifact missing mfs_lived_apart upper_amount",
            )
        })?;

    let mut output = String::new();
    output.push_str("    // BEGIN GENERATED TAXATION TESTS\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn single_thresholds() {\n");
    output.push_str("        let t = thresholds(FilingStatus::Single, None).unwrap();\n");
    output.push_str(&format!(
        "        assert_eq!(t.base_amount, {});\n",
        format_f64(single_base)
    ));
    output.push_str(&format!(
        "        assert_eq!(t.upper_amount, {});\n",
        format_f64(single_upper)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn mfj_thresholds() {\n");
    output.push_str(
        "        let t = thresholds(FilingStatus::MarriedFilingJointly, None).unwrap();\n",
    );
    output.push_str(&format!(
        "        assert_eq!(t.base_amount, {});\n",
        format_f64(mfj_base)
    ));
    output.push_str(&format!(
        "        assert_eq!(t.upper_amount, {});\n",
        format_f64(mfj_upper)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn mfs_lived_with_spouse_thresholds() {\n");
    output.push_str(
        "        let t = thresholds(FilingStatus::MarriedFilingSeparately, Some(true)).unwrap();\n",
    );
    output.push_str(&format!(
        "        assert_eq!(t.base_amount, {});\n",
        format_f64(mfs_lived_with_spouse_base)
    ));
    output.push_str(&format!(
        "        assert_eq!(t.upper_amount, {});\n",
        format_f64(mfs_lived_with_spouse_upper)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn mfs_lived_apart_thresholds() {\n");
    output.push_str(
        "        let t = thresholds(FilingStatus::MarriedFilingSeparately, Some(false)).unwrap();\n",
    );
    output.push_str(&format!(
        "        assert_eq!(t.base_amount, {});\n",
        format_f64(mfs_lived_apart_base)
    ));
    output.push_str(&format!(
        "        assert_eq!(t.upper_amount, {});\n",
        format_f64(mfs_lived_apart_upper)
    ));
    output.push_str("    }\n");
    Ok(output)
}

fn render_tax_federal_qbi_tests(
    variant_map: &BTreeMap<String, Value>,
) -> Result<String, PipelineError> {
    let single = variant_map
        .get("single")
        .and_then(Value::as_object)
        .ok_or_else(|| PipelineError::new("reviewed QBI artifact is missing variant single"))?;
    let mfj = variant_map
        .get("married_filing_jointly")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new("reviewed QBI artifact is missing variant married_filing_jointly")
        })?;

    let single_threshold = single
        .get("threshold")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed QBI artifact missing single threshold"))?;
    let single_phase_in_end = single
        .get("phase_in_range_end")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new("reviewed QBI artifact missing single phase_in_range_end")
        })?;
    let mfj_threshold = mfj
        .get("threshold")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed QBI artifact missing mfj threshold"))?;
    let mfj_phase_in_end = mfj
        .get("phase_in_range_end")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new("reviewed QBI artifact missing mfj phase_in_range_end")
        })?;
    let mfj_deduction_rate = mfj
        .get("deduction_rate")
        .and_then(Value::as_f64)
        .ok_or_else(|| PipelineError::new("reviewed QBI artifact missing mfj deduction_rate"))?;
    let mfj_minimum_deduction = mfj
        .get("minimum_qbi_deduction")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new("reviewed QBI artifact missing mfj minimum_qbi_deduction")
        })?;
    let mfj_minimum_amount = mfj
        .get("minimum_qbi_amount")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new("reviewed QBI artifact missing mfj minimum_qbi_amount")
        })?;

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn qbi_deduction_mfj() {\n");
    output.push_str("        let q = qbi_deduction(FilingStatus::MarriedFilingJointly);\n");
    output.push_str(&format!(
        "        assert_eq!(q.threshold, {});\n",
        format_f64(mfj_threshold)
    ));
    output.push_str(&format!(
        "        assert_eq!(q.phase_in_range_end, {});\n",
        format_f64(mfj_phase_in_end)
    ));
    output.push_str(&format!(
        "        assert_eq!(q.deduction_rate, {});\n",
        format_f64(mfj_deduction_rate)
    ));
    output.push_str(&format!(
        "        assert_eq!(q.minimum_qbi_deduction, {});\n",
        format_f64(mfj_minimum_deduction)
    ));
    output.push_str(&format!(
        "        assert_eq!(q.minimum_qbi_amount, {});\n",
        format_f64(mfj_minimum_amount)
    ));
    output.push_str("    }\n\n");
    output.push_str("    #[test]\n");
    output.push_str("    fn qbi_deduction_single() {\n");
    output.push_str("        let q = qbi_deduction(FilingStatus::Single);\n");
    output.push_str(&format!(
        "        assert_eq!(q.threshold, {});\n",
        format_f64(single_threshold)
    ));
    output.push_str(&format!(
        "        assert_eq!(q.phase_in_range_end, {});\n",
        format_f64(single_phase_in_end)
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_tax_federal_estate_exemption_section(
    variant_map: &BTreeMap<String, f64>,
) -> Result<String, PipelineError> {
    let amount = variant_map.get("default").ok_or_else(|| {
        PipelineError::new("reviewed estate exemption artifact is missing default variant")
    })?;

    let mut output = String::new();
    output.push_str("/// Basic exclusion amount (exemption) for 2026, reviewed artifact.\n");
    output.push_str("pub fn exemption() -> f64 {\n");
    output.push_str(&format!("    {}\n", format_f64(*amount)));
    output.push_str("}\n\n");
    Ok(output)
}

fn render_tax_federal_estate_exemption_tests(
    variant_map: &BTreeMap<String, f64>,
) -> Result<String, PipelineError> {
    let amount = variant_map.get("default").ok_or_else(|| {
        PipelineError::new("reviewed estate exemption artifact is missing default variant")
    })?;

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn exemption_2026() {\n");
    output.push_str(&format!(
        "        assert_eq!(exemption(), {});\n",
        format_f64(*amount)
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_tax_federal_estate_applicable_credit_section(
    variant_map: &BTreeMap<String, f64>,
) -> Result<String, PipelineError> {
    let amount = variant_map.get("default").ok_or_else(|| {
        PipelineError::new("reviewed estate applicable credit artifact is missing default variant")
    })?;

    let mut output = String::new();
    output.push_str("/// Applicable credit amount for 2026, reviewed artifact.\n");
    output.push_str("pub fn applicable_credit() -> f64 {\n");
    output.push_str(&format!("    {}\n", format_f64(*amount)));
    output.push_str("}\n\n");
    Ok(output)
}

fn render_tax_federal_estate_applicable_credit_tests(
    variant_map: &BTreeMap<String, f64>,
) -> Result<String, PipelineError> {
    let amount = variant_map.get("default").ok_or_else(|| {
        PipelineError::new("reviewed estate applicable credit artifact is missing default variant")
    })?;

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn applicable_credit_2026() {\n");
    output.push_str(&format!(
        "        assert_eq!(applicable_credit(), {});\n",
        format_f64(*amount)
    ));
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_tax_federal_estate_brackets_section(
    variant_map: &BTreeMap<String, Vec<Value>>,
) -> Result<String, PipelineError> {
    let brackets = variant_map.get("default").ok_or_else(|| {
        PipelineError::new("reviewed estate brackets artifact is missing default variant")
    })?;

    let mut output = String::new();
    output.push_str("pub fn brackets() -> Vec<TaxBracket> {\n");
    output.push_str("    vec![\n");
    for bracket in brackets {
        let Some(obj) = bracket.as_object() else {
            return Err(PipelineError::new(
                "estate tax bracket for default is not an object",
            ));
        };
        let min = obj
            .get("min")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("estate tax bracket missing min"))?;
        let max = obj.get("max").and_then(Value::as_f64);
        let rate = obj
            .get("rate")
            .and_then(Value::as_f64)
            .ok_or_else(|| PipelineError::new("estate tax bracket missing rate"))?;
        output.push_str("        TaxBracket {\n");
        output.push_str(&format!("            min: {},\n", format_f64(min)));
        match max {
            Some(value) => {
                output.push_str(&format!("            max: Some({}),\n", format_f64(value)))
            }
            None => output.push_str("            max: None,\n"),
        }
        output.push_str(&format!("            rate: {},\n", format_f64(rate)));
        output.push_str("        },\n");
    }
    output.push_str("    ]\n");
    output.push_str("}\n\n");
    Ok(output)
}

fn render_tax_federal_estate_brackets_tests(
    variant_map: &BTreeMap<String, Vec<Value>>,
) -> Result<String, PipelineError> {
    let brackets = variant_map.get("default").ok_or_else(|| {
        PipelineError::new("reviewed estate brackets artifact is missing default variant")
    })?;
    let first_rate = brackets
        .first()
        .and_then(Value::as_object)
        .and_then(|obj| obj.get("rate"))
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new("reviewed estate brackets artifact is missing first rate")
        })?;
    let last_rate = brackets
        .last()
        .and_then(Value::as_object)
        .and_then(|obj| obj.get("rate"))
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            PipelineError::new("reviewed estate brackets artifact is missing last rate")
        })?;

    let mut output = String::new();
    output.push_str("    #[test]\n");
    output.push_str("    fn bracket_count() {\n");
    output.push_str("        let b = brackets();\n");
    output.push_str(&format!(
        "        assert_eq!(b.len(), {});\n",
        brackets.len()
    ));
    output.push_str(&format!(
        "        assert_eq!(b[0].rate, {});\n",
        format_f64(first_rate)
    ));
    output.push_str(&format!(
        "        assert_eq!(b[b.len() - 1].rate, {});\n",
        format_f64(last_rate)
    ));
    output.push_str("        assert_eq!(b[b.len() - 1].max, None);\n");
    output.push_str("    }\n\n");
    Ok(output)
}

fn render_tax_federal_capital_gains_section(
    variant_map: &BTreeMap<String, Vec<Value>>,
) -> Result<String, PipelineError> {
    let mut output = String::new();
    output.push_str(
        "// ---------------------------------------------------------------------------\n\
// Capital gains brackets (2026, reviewed artifact)\n\
// ---------------------------------------------------------------------------\n\n",
    );
    output.push_str("pub fn capital_gains_brackets(status: FilingStatus) -> Vec<TaxBracket> {\n");
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
                    "capital gains bracket for {} is not an object",
                    label
                )));
            };
            let min = obj
                .get("min")
                .and_then(Value::as_f64)
                .ok_or_else(|| PipelineError::new("capital gains bracket missing min"))?;
            let max = obj.get("max").and_then(Value::as_f64);
            let rate = obj
                .get("rate")
                .and_then(Value::as_f64)
                .ok_or_else(|| PipelineError::new("capital gains bracket missing rate"))?;
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

fn format_option_u64(value: Option<u64>) -> String {
    match value {
        Some(value) => format!("Some({value})"),
        None => "None".to_string(),
    }
}

fn escape_rust_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
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
    year: u32,
    category: &str,
    key: &str,
    configured: &[ExpectedVariant],
) -> Result<Vec<ExpectedVariant>, PipelineError> {
    let live_variants = lookup_entry_variants(category, key, year)?
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
    match &definition.validation_profile {
        ValidationProfile::Brackets => Ok(variants
            .iter()
            .map(|variant| format!("variants[{}].value", variant.label))
            .collect()),
        ValidationProfile::NumericField { field } => Ok(variants
            .iter()
            .map(|variant| format!("variants[{}].value.{}", variant.label, field))
            .collect()),
        ValidationProfile::Niit => {
            let mut paths = Vec::new();
            for variant in variants {
                paths.push(format!("variants[{}].value.rate", variant.label));
                paths.push(format!("variants[{}].value.threshold", variant.label));
            }
            Ok(paths)
        }
        ValidationProfile::Payroll => {
            let mut paths = Vec::new();
            for variant in variants {
                for field in [
                    "social_security_rate",
                    "social_security_wage_base",
                    "self_employment_tax_rate",
                    "medicare_rate",
                    "self_employment_medicare_rate",
                    "additional_medicare_rate",
                    "additional_medicare_threshold",
                ] {
                    paths.push(format!("variants[{}].value.{}", variant.label, field));
                }
            }
            Ok(paths)
        }
        ValidationProfile::MedicareBasePremiums => {
            let mut paths = Vec::new();
            for variant in variants {
                for field in [
                    "part_b_standard_monthly_premium",
                    "part_b_annual_deductible",
                    "part_d_base_beneficiary_premium",
                ] {
                    paths.push(format!("variants[{}].value.{}", variant.label, field));
                }
            }
            Ok(paths)
        }
        ValidationProfile::SsRetirementEarningsTest => {
            let mut paths = Vec::new();
            for variant in variants {
                for field in [
                    "under_fra_annual_exempt_amount",
                    "under_fra_monthly_exempt_amount",
                    "year_of_fra_annual_exempt_amount",
                    "year_of_fra_monthly_exempt_amount",
                    "under_fra_reduction_rate",
                    "year_of_fra_reduction_rate",
                ] {
                    paths.push(format!("variants[{}].value.{}", variant.label, field));
                }
            }
            Ok(paths)
        }
        ValidationProfile::SocialSecurityFullRetirementAge => {
            let mut paths = Vec::new();
            for variant in variants {
                for field in ["benefit_scope", "january_1_births_use_prior_year", "rules"] {
                    paths.push(format!("variants[{}].value.{}", variant.label, field));
                }
            }
            Ok(paths)
        }
        ValidationProfile::Salt => {
            let mut paths = Vec::new();
            for variant in variants {
                for field in [
                    "cap_amount",
                    "phaseout_threshold",
                    "phaseout_rate",
                    "floor_amount",
                ] {
                    paths.push(format!("variants[{}].value.{}", variant.label, field));
                }
            }
            Ok(paths)
        }
        ValidationProfile::Qbi => {
            let mut paths = Vec::new();
            for variant in variants {
                for field in [
                    "deduction_rate",
                    "threshold",
                    "phase_in_range_end",
                    "minimum_qbi_deduction",
                    "minimum_qbi_amount",
                ] {
                    paths.push(format!("variants[{}].value.{}", variant.label, field));
                }
            }
            Ok(paths)
        }
        ValidationProfile::AgeDistribution
        | ValidationProfile::JointDistribution
        | ValidationProfile::MortalityQx => Ok(variants
            .iter()
            .map(|variant| format!("variants[{}].value", variant.label))
            .collect()),
        ValidationProfile::DistributionRules => {
            let mut paths = Vec::new();
            for variant in variants {
                for field in [
                    "required_beginning.start_age_rules",
                    "required_beginning.first_distribution_deadline",
                    "required_beginning.still_working_exception",
                    "account_applicability.owner_required_account_types",
                    "account_applicability.owner_exempt_account_types",
                    "account_applicability.inherited_account_types",
                    "account_applicability.supports_pre_1987_403b_exclusion",
                    "account_applicability.designated_roth_owner_exemption_effective_year",
                    "account_applicability.pre_1987_403b",
                    "beneficiary_distribution.beneficiary_categories",
                    "beneficiary_distribution.recognized_beneficiary_classes",
                    "beneficiary_distribution.eligible_designated_beneficiary_classes",
                    "beneficiary_distribution.life_expectancy_method_by_class",
                    "beneficiary_distribution.minor_child_majority_age",
                    "beneficiary_distribution.spouse_delay_allowed",
                    "beneficiary_distribution.ten_year_rule",
                    "beneficiary_distribution.non_designated_beneficiary_rules",
                    "beneficiary_distribution.relief_years",
                ] {
                    paths.push(format!("variants[{}].value.{}", variant.label, field));
                }
            }
            Ok(paths)
        }
        ValidationProfile::SsTaxation => {
            let mut paths = Vec::new();
            for variant in variants {
                for field in [
                    "base_amount",
                    "upper_amount",
                    "max_taxable_pct_below_upper",
                    "max_taxable_pct_above_upper",
                ] {
                    paths.push(format!("variants[{}].value.{}", variant.label, field));
                }
            }
            Ok(paths)
        }
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

fn reference_root_for(engine_root: &Path) -> PathBuf {
    engine_root
        .parent()
        .unwrap_or(engine_root)
        .join("reference")
}

fn reference_pack_path_for(engine_root: &Path, year: u32, category: &str, key: &str) -> PathBuf {
    reference_root_for(engine_root)
        .join(category)
        .join(year.to_string())
        .join(reference_pack_file_name(category, key))
}

fn reference_pack_file_name(category: &str, key: &str) -> String {
    if category == "retirement" && key == "single_life_table" {
        "single_life_expectancy_table.md".to_string()
    } else {
        format!("{key}.md")
    }
}

fn write_reference_pack(
    engine_root: &Path,
    reviewed_artifact: &ReviewedArtifact,
    primer: &ReferencePackPrimer,
) -> Result<PathBuf, PipelineError> {
    let path = reference_pack_path_for(
        engine_root,
        reviewed_artifact.year,
        &reviewed_artifact.category,
        &reviewed_artifact.key,
    );
    let contents = render_reference_pack_markdown(reviewed_artifact, primer)?;
    write_text(&path, &contents)?;
    Ok(path)
}

fn refresh_reference_manifest(engine_root: &Path) -> Result<PathBuf, PipelineError> {
    let reference_root = reference_root_for(engine_root);
    fs::create_dir_all(&reference_root)?;
    let manifest_path = reference_root.join("manifest.json");
    let manifest = build_reference_manifest(&reference_root, &manifest_path)?;
    write_json(&manifest_path, &manifest)?;
    Ok(manifest_path)
}

fn build_reference_manifest(
    reference_root: &Path,
    manifest_path: &Path,
) -> Result<ReferenceManifest, PipelineError> {
    let mut bundle_version = "dev".to_string();
    if let Ok(contents) = fs::read_to_string(manifest_path) {
        if let Ok(existing) = serde_json::from_str::<ReferenceManifest>(&contents) {
            bundle_version = existing.bundle_version;
        }
    }

    let mut categories = BTreeMap::<String, Vec<String>>::new();
    let mut pack_count = 0usize;

    if reference_root.exists() {
        for category_entry in fs::read_dir(reference_root)? {
            let category_entry = category_entry?;
            if !category_entry.file_type()?.is_dir() {
                continue;
            }
            let category_name = category_entry.file_name().to_string_lossy().to_string();
            let mut years = BTreeSet::<String>::new();

            for year_entry in fs::read_dir(category_entry.path())? {
                let year_entry = year_entry?;
                if !year_entry.file_type()?.is_dir() {
                    continue;
                }
                let year_name = year_entry.file_name().to_string_lossy().to_string();
                let year_pack_count = fs::read_dir(year_entry.path())?
                    .filter_map(Result::ok)
                    .filter(|entry| {
                        entry
                            .file_type()
                            .map(|kind| kind.is_file())
                            .unwrap_or(false)
                    })
                    .filter(|entry| {
                        entry.path().extension().and_then(|ext| ext.to_str()) == Some("md")
                    })
                    .count();
                if year_pack_count > 0 {
                    pack_count += year_pack_count;
                    years.insert(year_name);
                }
            }

            if !years.is_empty() {
                categories.insert(category_name, years.into_iter().collect());
            }
        }
    }

    Ok(ReferenceManifest {
        bundle_version,
        generated_at: None,
        categories,
        pack_count,
    })
}

fn normalized_primer_list(items: &[String]) -> Vec<String> {
    items
        .iter()
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn write_reference_pack_list_section(output: &mut String, heading: &str, items: &[String]) {
    output.push_str(&format!("## {heading}\n\n"));
    for item in normalized_primer_list(items) {
        output.push_str(&format!("- {item}\n"));
    }
    output.push('\n');
}

fn render_reference_pack_markdown(
    reviewed_artifact: &ReviewedArtifact,
    primer: &ReferencePackPrimer,
) -> Result<String, PipelineError> {
    let title = title_case_key(&reviewed_artifact.key);
    let machine_block = serde_json::to_string_pretty(reviewed_artifact)?;
    let reviewed_artifact_slug = format!(
        "{}/{}/{}",
        reviewed_artifact.category, reviewed_artifact.year, reviewed_artifact.key
    );
    let mut output = String::new();
    output.push_str("---\n");
    output.push_str(&format!("category: {}\n", reviewed_artifact.category));
    output.push_str(&format!("year: {}\n", reviewed_artifact.year));
    output.push_str(&format!("key: {}\n", reviewed_artifact.key));
    output.push_str(&format!("title: {}\n", title));
    output.push_str(&format!("reviewed_artifact: {}\n", reviewed_artifact_slug));
    output.push_str("bundle_version: dev\n");
    output.push_str(&format!(
        "verification_status: {}\n",
        reviewed_artifact.verification_status
    ));
    output.push_str("review_status: reviewed\n");
    output.push_str("---\n\n");
    output.push_str(&format!("# {title}\n\n"));
    output.push_str("## What This Is\n\n");
    output.push_str(primer.what_this_is.trim());
    output.push_str("\n\n");
    write_reference_pack_list_section(&mut output, "Lookup Parameters", &primer.lookup_parameters);
    write_reference_pack_list_section(
        &mut output,
        "Interpretation Notes",
        &primer.interpretation_notes,
    );
    write_reference_pack_list_section(&mut output, "Does Not Include", &primer.does_not_include);
    write_reference_pack_list_section(&mut output, "Caveats", &primer.caveats);
    if has_non_empty_list_items(&primer.typical_uses) {
        write_reference_pack_list_section(&mut output, "Typical Uses", &primer.typical_uses);
    }
    output.push_str("## Machine Block\n\n```json\n");
    output.push_str(&machine_block);
    output.push_str("\n```\n\n");
    output.push_str("## Sources\n\n");
    if reviewed_artifact.accepted_sources.is_empty() {
        output.push_str("- none\n");
    } else {
        for source in &reviewed_artifact.accepted_sources {
            output.push_str(&format!(
                "- {} — {} — {}\n",
                source.organization, source.title, source.url
            ));
        }
    }
    Ok(output)
}

fn title_case_key(key: &str) -> String {
    key.split('_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => {
                    let rest = chars.collect::<String>();
                    format!("{}{}", first.to_uppercase(), rest)
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
    if !path.exists() {
        return Err(PipelineError::new(format!(
            "pipeline definition not found for {}/{} at {}",
            category,
            key,
            path.display()
        )));
    }
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

fn deserialize_unresolved_issues<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Vec::<Value>::deserialize(deserializer)?;
    let mut normalized = Vec::with_capacity(raw.len());

    for issue in raw {
        match issue {
            Value::String(text) => normalized.push(text),
            Value::Object(mut obj) => {
                let severity = obj
                    .remove("severity")
                    .and_then(|value| value.as_str().map(str::to_string));
                if let Some(text) = obj
                    .remove("issue")
                    .and_then(|value| value.as_str().map(str::to_string))
                {
                    match severity {
                        Some(severity) if !severity.trim().is_empty() => {
                            normalized.push(format!("{text} (severity: {severity})"));
                        }
                        _ => normalized.push(text),
                    }
                } else {
                    normalized.push(Value::Object(obj).to_string());
                }
            }
            other => {
                return Err(D::Error::custom(format!(
                    "invalid unresolved issue entry {}; expected string or object",
                    other
                )));
            }
        }
    }

    Ok(normalized)
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

fn format_qx(value: f64) -> String {
    let mut formatted = format!("{value:.6}");
    while formatted.contains('.') && formatted.ends_with('0') {
        formatted.pop();
    }
    if formatted.ends_with('.') {
        formatted.push('0');
    }
    formatted
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
            Self::Repair => "repair",
            Self::RepairVerifier => "repair_verifier",
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

fn display_recommended_action(action: ReviewRecommendedAction) -> &'static str {
    match action {
        ReviewRecommendedAction::ApplyApprovedResult => "apply_approved_result",
        ReviewRecommendedAction::AddressVerifierFeedbackAndRerunReview => {
            "address_verifier_feedback_and_rerun_review"
        }
        ReviewRecommendedAction::UpdateContractThenRerunPipeline => {
            "update_contract_then_rerun_pipeline"
        }
        ReviewRecommendedAction::InvestigateSourcesManually => "investigate_sources_manually",
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
