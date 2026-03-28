mod workflow;

pub use workflow::{
    apply_run, apply_run_at, default_pipeline_definition_path, default_pipeline_definitions_dir,
    default_reviewed_root, default_runs_root, prepare_run, prepare_run_at, review_run,
    review_run_at, review_run_with_approval, review_run_with_approval_at, run_agents,
    run_agents_at, status_report, status_report_at, AgentExecutionLog, AgentInvocationConfig,
    AgentProvider, ApplyOutcome, PipelineDefinition, PipelineRunSummary, PipelineStatusEntry,
    PipelineStatusReport, PreparedRun, RepairExecutionLog, ReviewOutcome, ReviewRecommendedAction,
    RunAgentsConfig, RunAgentsOutcome,
};

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::data;
use crate::data::taxonomy::CoverageEntry;
use crate::data::types::{FilingStatus, LookupParams};

const REGISTRY_VERSION: u32 = 1;
const SNAPSHOT_VERSION: u32 = 1;

#[derive(Debug)]
pub struct PipelineError(String);

impl PipelineError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PipelineError {}

impl From<std::io::Error> for PipelineError {
    fn from(error: std::io::Error) -> Self {
        Self::new(format!("io error: {error}"))
    }
}

impl From<serde_json::Error> for PipelineError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(format!("json error: {error}"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryDocument {
    pub registry_version: u32,
    pub year: u32,
    pub entries: Vec<RegistryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub category: String,
    pub key: String,
    pub verification_status: VerificationStatus,
    pub completeness: Completeness,
    pub validation: ValidationProfile,
    pub source_documents: Vec<SourceDocument>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Authoritative,
    Corroborated,
    Derived,
    Placeholder,
}

impl fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authoritative => f.write_str("authoritative"),
            Self::Corroborated => f.write_str("corroborated"),
            Self::Derived => f.write_str("derived"),
            Self::Placeholder => f.write_str("placeholder"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Completeness {
    Full,
    Partial,
}

impl fmt::Display for Completeness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full => f.write_str("full"),
            Self::Partial => f.write_str("partial"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ValidationProfile {
    Brackets,
    NumericField { field: String },
    Niit,
    Payroll,
    MedicareBasePremiums,
    SocialSecurityFullRetirementAge,
    Salt,
    Qbi,
    AgeDistribution,
    JointDistribution,
    DistributionRules,
    SsTaxation,
    SsRetirementEarningsTest,
    Irmaa,
    MortalityQx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceDocument {
    pub authority: String,
    pub title: String,
    pub section: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDocument {
    pub snapshot_version: u32,
    pub year: u32,
    pub entries: Vec<SnapshotEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotEntry {
    pub category: String,
    pub key: String,
    pub variants: Vec<SnapshotVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotVariant {
    pub label: String,
    pub params: SnapshotParams,
    pub checksum: String,
    pub value_kind: String,
    pub item_count: Option<usize>,
    pub value: Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotParams {
    pub filing_status: Option<String>,
    pub lived_with_spouse_during_year: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub coverage_entries: usize,
    pub metadata_entries: usize,
    pub snapshot_entries: usize,
    pub variants_checked: usize,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl ValidationReport {
    pub fn is_success(&self, strict: bool) -> bool {
        let _ = strict;
        self.errors.is_empty()
    }
}

pub fn engine_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

pub fn workspace_root() -> PathBuf {
    engine_root()
        .parent()
        .expect("engine crate is inside workspace root")
        .to_path_buf()
}

pub fn data_registry_root() -> PathBuf {
    engine_root().join("data_registry")
}

pub fn default_registry_dir_for_year(year: u32) -> PathBuf {
    data_registry_root().join(year.to_string())
}

pub fn default_registry_dir() -> PathBuf {
    default_registry_dir_for_year(2026)
}

pub fn default_metadata_path_for_year(year: u32) -> PathBuf {
    default_registry_dir_for_year(year).join("metadata.json")
}

pub fn default_metadata_path() -> PathBuf {
    default_metadata_path_for_year(2026)
}

pub fn default_snapshot_path_for_year(year: u32) -> PathBuf {
    default_registry_dir_for_year(year).join("snapshot.json")
}

pub fn default_snapshot_path() -> PathBuf {
    default_snapshot_path_for_year(2026)
}

pub fn load_registry(path: &Path) -> Result<RegistryDocument, PipelineError> {
    let contents = fs::read_to_string(path)?;
    let registry: RegistryDocument = serde_json::from_str(&contents)?;

    if registry.registry_version != REGISTRY_VERSION {
        return Err(PipelineError::new(format!(
            "unsupported registry version {} in {}",
            registry.registry_version,
            path.display()
        )));
    }

    Ok(registry)
}

pub fn write_registry(path: &Path, registry: &RegistryDocument) -> Result<(), PipelineError> {
    let json = serde_json::to_string_pretty(registry)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, format!("{json}\n"))?;
    Ok(())
}

pub fn load_snapshot(path: &Path) -> Result<SnapshotDocument, PipelineError> {
    let contents = fs::read_to_string(path)?;
    let snapshot: SnapshotDocument = serde_json::from_str(&contents)?;

    if snapshot.snapshot_version != SNAPSHOT_VERSION {
        return Err(PipelineError::new(format!(
            "unsupported snapshot version {} in {}",
            snapshot.snapshot_version,
            path.display()
        )));
    }

    Ok(snapshot)
}

pub fn write_snapshot(path: &Path, snapshot: &SnapshotDocument) -> Result<(), PipelineError> {
    let json = serde_json::to_string_pretty(snapshot)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, format!("{json}\n"))?;
    Ok(())
}

pub fn lookup_entry_variants(
    category: &str,
    key: &str,
    year: u32,
) -> Result<Vec<SnapshotVariant>, PipelineError> {
    let coverage = data::coverage(None);
    let entry = coverage
        .into_iter()
        .find(|candidate| candidate.category == category && candidate.key == key)
        .ok_or_else(|| {
            PipelineError::new(format!("coverage entry not found for {category}/{key}"))
        })?;

    let mut variants = Vec::new();
    for variant in variants_for_entry(&entry) {
        let lookup_params = LookupParams {
            filing_status: variant.params.filing_status.clone(),
            lived_with_spouse_during_year: variant.params.lived_with_spouse_during_year,
        };
        let value = data::lookup(category, key, year, &lookup_params).map_err(|err| {
            PipelineError::new(format!(
                "lookup failed for {category}/{key} [{}]: {err}",
                variant.label
            ))
        })?;
        let canonical_value = canonicalize(&value);
        let canonical_json = serde_json::to_vec(&canonical_value)?;

        variants.push(SnapshotVariant {
            label: variant.label,
            params: variant.params,
            checksum: fnv1a64(&canonical_json),
            value_kind: value_kind(&canonical_value).to_string(),
            item_count: value_item_count(&canonical_value),
            value: canonical_value,
        });
    }

    Ok(variants)
}

pub fn generate_snapshot(registry: &RegistryDocument) -> Result<SnapshotDocument, PipelineError> {
    let coverage = data::coverage(None)
        .into_iter()
        .filter(|entry| entry.years.contains(&registry.year))
        .collect::<Vec<_>>();
    let registry_keys = registry_entry_keys(&registry.entries);
    let coverage_keys = coverage_entry_keys(&coverage);

    let missing_from_registry: Vec<_> = coverage_keys.difference(&registry_keys).cloned().collect();
    if !missing_from_registry.is_empty() {
        return Err(PipelineError::new(format!(
            "registry metadata missing entries: {}",
            missing_from_registry.join(", ")
        )));
    }

    let unknown_registry_entries: Vec<_> =
        registry_keys.difference(&coverage_keys).cloned().collect();
    if !unknown_registry_entries.is_empty() {
        return Err(PipelineError::new(format!(
            "registry metadata contains unknown entries: {}",
            unknown_registry_entries.join(", ")
        )));
    }

    let mut entries = Vec::new();
    let mut sorted_coverage = coverage;
    sorted_coverage.sort_by(|a, b| (&a.category, &a.key).cmp(&(&b.category, &b.key)));

    for entry in sorted_coverage {
        let mut variants = Vec::new();
        for variant in variants_for_entry(&entry) {
            let lookup_params = LookupParams {
                filing_status: variant.params.filing_status.clone(),
                lived_with_spouse_during_year: variant.params.lived_with_spouse_during_year,
            };
            let value = data::lookup(&entry.category, &entry.key, registry.year, &lookup_params)
                .map_err(|err| {
                    PipelineError::new(format!(
                        "lookup failed for {}/{} [{}]: {err}",
                        entry.category, entry.key, variant.label
                    ))
                })?;
            let canonical_value = canonicalize(&value);
            let canonical_json = serde_json::to_vec(&canonical_value)?;

            variants.push(SnapshotVariant {
                label: variant.label,
                params: variant.params,
                checksum: fnv1a64(&canonical_json),
                value_kind: value_kind(&canonical_value).to_string(),
                item_count: value_item_count(&canonical_value),
                value: canonical_value,
            });
        }

        entries.push(SnapshotEntry {
            category: entry.category,
            key: entry.key,
            variants,
        });
    }

    Ok(SnapshotDocument {
        snapshot_version: SNAPSHOT_VERSION,
        year: registry.year,
        entries,
    })
}

pub fn validate_registry(
    registry: &RegistryDocument,
    expected_snapshot: &SnapshotDocument,
    strict: bool,
) -> Result<ValidationReport, PipelineError> {
    let coverage = data::coverage(None)
        .into_iter()
        .filter(|entry| entry.years.contains(&registry.year))
        .collect::<Vec<_>>();
    let live_snapshot = generate_snapshot(registry)?;
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    if expected_snapshot.year != registry.year {
        errors.push(format!(
            "snapshot year {} does not match registry year {}",
            expected_snapshot.year, registry.year
        ));
    }

    let coverage_keys = coverage_entry_keys(&coverage);
    let registry_keys = registry_entry_keys(&registry.entries);
    let snapshot_keys = snapshot_entry_keys(&expected_snapshot.entries);

    for missing in coverage_keys.difference(&registry_keys) {
        errors.push(format!("metadata missing coverage entry {missing}"));
    }
    for extra in registry_keys.difference(&coverage_keys) {
        errors.push(format!("metadata contains unknown entry {extra}"));
    }
    for missing in registry_keys.difference(&snapshot_keys) {
        errors.push(format!("snapshot missing metadata entry {missing}"));
    }
    for extra in snapshot_keys.difference(&registry_keys) {
        errors.push(format!("snapshot contains unknown entry {extra}"));
    }

    let metadata_by_key: BTreeMap<_, _> = registry
        .entries
        .iter()
        .map(|entry| (entry_key(&entry.category, &entry.key), entry))
        .collect();
    let expected_by_key: BTreeMap<_, _> = expected_snapshot
        .entries
        .iter()
        .map(|entry| (entry_key(&entry.category, &entry.key), entry))
        .collect();
    let live_by_key: BTreeMap<_, _> = live_snapshot
        .entries
        .iter()
        .map(|entry| (entry_key(&entry.category, &entry.key), entry))
        .collect();

    let mut variants_checked = 0usize;

    for key in registry_keys.intersection(&coverage_keys) {
        let metadata = metadata_by_key
            .get(key.as_str())
            .expect("metadata entry exists");
        let expected = expected_by_key.get(key.as_str());
        let live = live_by_key.get(key.as_str());

        if metadata.source_documents.is_empty() {
            errors.push(format!(
                "{key}: metadata must include at least one source document"
            ));
        }

        let mut entry_has_caveat = false;
        let mut entry_is_strict_blocking = false;
        match metadata.verification_status {
            VerificationStatus::Authoritative => {}
            VerificationStatus::Corroborated => {
                warnings.push(format!(
                    "{key}: verification_status is {}",
                    metadata.verification_status
                ));
                entry_has_caveat = true;
            }
            VerificationStatus::Derived | VerificationStatus::Placeholder => {
                let message = format!(
                    "{key}: verification_status is {}",
                    metadata.verification_status
                );
                warnings.push(message.clone());
                if strict {
                    errors.push(format!("strict: {message}"));
                }
                entry_has_caveat = true;
                entry_is_strict_blocking = true;
            }
        }
        if metadata.completeness != Completeness::Full {
            let message = format!("{key}: completeness is {}", metadata.completeness);
            warnings.push(message.clone());
            if strict {
                errors.push(format!("strict: {message}"));
            }
            entry_has_caveat = true;
            entry_is_strict_blocking = true;
        }
        if entry_has_caveat {
            if let Some(notes) = metadata.notes.as_ref() {
                if !notes.trim().is_empty() {
                    let message = format!("{key}: {}", notes.trim());
                    warnings.push(message.clone());
                    if strict && entry_is_strict_blocking {
                        errors.push(format!("strict: {message}"));
                    }
                }
            }
        }

        let Some(expected_entry) = expected else {
            continue;
        };
        let Some(live_entry) = live else {
            continue;
        };

        if expected_entry.variants.len() != live_entry.variants.len() {
            errors.push(format!(
                "{key}: snapshot variant count {} does not match live variant count {}",
                expected_entry.variants.len(),
                live_entry.variants.len()
            ));
            continue;
        }

        let expected_variants: BTreeMap<_, _> = expected_entry
            .variants
            .iter()
            .map(|variant| (variant.label.as_str(), variant))
            .collect();
        let live_variants: BTreeMap<_, _> = live_entry
            .variants
            .iter()
            .map(|variant| (variant.label.as_str(), variant))
            .collect();

        for label in expected_variants.keys().chain(live_variants.keys()) {
            let Some(expected_variant) = expected_variants.get(label) else {
                errors.push(format!(
                    "{key}: live variant {} not found in snapshot",
                    label
                ));
                continue;
            };
            let Some(live_variant) = live_variants.get(label) else {
                errors.push(format!(
                    "{key}: snapshot variant {} not found in live data",
                    label
                ));
                continue;
            };

            variants_checked += 1;

            if expected_variant.params != live_variant.params {
                errors.push(format!(
                    "{key} [{}]: parameter mismatch between snapshot and live data",
                    label
                ));
            }

            if expected_variant.checksum != live_variant.checksum {
                errors.push(format!(
                    "{key} [{}]: checksum mismatch (snapshot {}, live {})",
                    label, expected_variant.checksum, live_variant.checksum
                ));
            }

            errors.extend(validate_value(
                key,
                label,
                &metadata.validation,
                &live_variant.value,
            ));
        }
    }
    Ok(ValidationReport {
        coverage_entries: coverage.len(),
        metadata_entries: registry.entries.len(),
        snapshot_entries: expected_snapshot.entries.len(),
        variants_checked,
        warnings,
        errors,
    })
}

#[derive(Debug, Clone)]
struct VariantRequest {
    label: String,
    params: SnapshotParams,
}

fn variants_for_entry(entry: &CoverageEntry) -> Vec<VariantRequest> {
    let has_filing_status = entry.params.iter().any(|param| param == "filing_status");
    let has_lived_with_spouse = entry
        .params
        .iter()
        .any(|param| param == "lived_with_spouse_during_year");

    if has_filing_status && has_lived_with_spouse {
        FilingStatus::all()
            .iter()
            .flat_map(|status| match status {
                FilingStatus::MarriedFilingSeparately => vec![
                    VariantRequest {
                        label: "married_filing_separately_lived_with_spouse".to_string(),
                        params: SnapshotParams {
                            filing_status: Some(status.to_string()),
                            lived_with_spouse_during_year: Some(true),
                        },
                    },
                    VariantRequest {
                        label: "married_filing_separately_lived_apart".to_string(),
                        params: SnapshotParams {
                            filing_status: Some(status.to_string()),
                            lived_with_spouse_during_year: Some(false),
                        },
                    },
                ],
                _ => vec![VariantRequest {
                    label: status.to_string(),
                    params: SnapshotParams {
                        filing_status: Some(status.to_string()),
                        lived_with_spouse_during_year: None,
                    },
                }],
            })
            .collect()
    } else if has_filing_status {
        FilingStatus::all()
            .iter()
            .map(|status| VariantRequest {
                label: status.to_string(),
                params: SnapshotParams {
                    filing_status: Some(status.to_string()),
                    lived_with_spouse_during_year: None,
                },
            })
            .collect()
    } else {
        vec![VariantRequest {
            label: "default".to_string(),
            params: SnapshotParams::default(),
        }]
    }
}

fn entry_key(category: &str, key: &str) -> String {
    format!("{category}/{key}")
}

fn coverage_entry_keys(entries: &[CoverageEntry]) -> BTreeSet<String> {
    entries
        .iter()
        .map(|entry| entry_key(&entry.category, &entry.key))
        .collect()
}

fn registry_entry_keys(entries: &[RegistryEntry]) -> BTreeSet<String> {
    entries
        .iter()
        .map(|entry| entry_key(&entry.category, &entry.key))
        .collect()
}

fn snapshot_entry_keys(entries: &[SnapshotEntry]) -> BTreeSet<String> {
    entries
        .iter()
        .map(|entry| entry_key(&entry.category, &entry.key))
        .collect()
}

fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = BTreeMap::new();
            for (key, child) in map {
                sorted.insert(key.clone(), canonicalize(child));
            }
            let mut output = Map::new();
            for (key, child) in sorted {
                output.insert(key, child);
            }
            Value::Object(output)
        }
        Value::Array(values) => Value::Array(values.iter().map(canonicalize).collect()),
        _ => value.clone(),
    }
}

fn fnv1a64(bytes: &[u8]) -> String {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    let mut hash = OFFSET;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(PRIME);
    }
    format!("{hash:016x}")
}

fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn value_item_count(value: &Value) -> Option<usize> {
    match value {
        Value::Array(values) => Some(values.len()),
        Value::Object(map) => Some(map.len()),
        _ => None,
    }
}

fn validate_value(
    entry_key: &str,
    variant_label: &str,
    profile: &ValidationProfile,
    value: &Value,
) -> Vec<String> {
    match profile {
        ValidationProfile::Brackets => validate_brackets(entry_key, variant_label, value),
        ValidationProfile::NumericField { field } => {
            validate_numeric_field(entry_key, variant_label, value, field)
        }
        ValidationProfile::Niit => validate_niit(entry_key, variant_label, value),
        ValidationProfile::Payroll => validate_payroll(entry_key, variant_label, value),
        ValidationProfile::MedicareBasePremiums => {
            validate_medicare_base_premiums(entry_key, variant_label, value)
        }
        ValidationProfile::SocialSecurityFullRetirementAge => {
            validate_social_security_full_retirement_age(entry_key, variant_label, value)
        }
        ValidationProfile::Salt => validate_salt(entry_key, variant_label, value),
        ValidationProfile::Qbi => validate_qbi(entry_key, variant_label, value),
        ValidationProfile::AgeDistribution => {
            validate_age_distribution(entry_key, variant_label, value)
        }
        ValidationProfile::JointDistribution => {
            validate_joint_distribution(entry_key, variant_label, value)
        }
        ValidationProfile::DistributionRules => {
            validate_distribution_rules(entry_key, variant_label, value)
        }
        ValidationProfile::SsTaxation => validate_ss_taxation(entry_key, variant_label, value),
        ValidationProfile::SsRetirementEarningsTest => {
            validate_ss_retirement_earnings_test(entry_key, variant_label, value)
        }
        ValidationProfile::Irmaa => validate_irmaa(entry_key, variant_label, value),
        ValidationProfile::MortalityQx => validate_mortality(entry_key, variant_label, value),
    }
}

fn validate_brackets(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(items) = value.as_array() else {
        errors.push(format!(
            "{entry_key} [{variant_label}]: expected array of brackets"
        ));
        return errors;
    };
    if items.is_empty() {
        errors.push(format!(
            "{entry_key} [{variant_label}]: bracket array is empty"
        ));
        return errors;
    }

    let mut previous_max: Option<f64> = None;
    for (index, item) in items.iter().enumerate() {
        let Some(obj) = item.as_object() else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: bracket {} is not an object",
                index
            ));
            continue;
        };
        let min = obj.get("min").and_then(Value::as_f64);
        let max = obj.get("max").and_then(Value::as_f64);
        let rate = obj.get("rate").and_then(Value::as_f64);

        let Some(min) = min else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: bracket {} missing numeric min",
                index
            ));
            continue;
        };
        let Some(rate) = rate else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: bracket {} missing numeric rate",
                index
            ));
            continue;
        };

        if index == 0 && min != 0.0 {
            errors.push(format!(
                "{entry_key} [{variant_label}]: first bracket must start at 0"
            ));
        }
        if !(0.0..=1.0).contains(&rate) {
            errors.push(format!(
                "{entry_key} [{variant_label}]: bracket {} rate {} out of range",
                index, rate
            ));
        }
        if let Some(previous_max) = previous_max {
            if (min - previous_max).abs() > f64::EPSILON {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: bracket {} min {} does not continue from previous max {}",
                    index, min, previous_max
                ));
            }
        }
        if let Some(max) = max {
            if max <= min {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: bracket {} max {} must be greater than min {}",
                    index, max, min
                ));
            }
        } else if index != items.len() - 1 {
            errors.push(format!(
                "{entry_key} [{variant_label}]: only final bracket may have max = null"
            ));
        }

        previous_max = max;
    }

    errors
}

fn validate_numeric_field(
    entry_key: &str,
    variant_label: &str,
    value: &Value,
    field: &str,
) -> Vec<String> {
    let mut errors = Vec::new();
    let Some(obj) = value.as_object() else {
        errors.push(format!("{entry_key} [{variant_label}]: expected object"));
        return errors;
    };

    match obj.get(field).and_then(Value::as_f64) {
        Some(number) if number >= 0.0 => {}
        Some(number) => errors.push(format!(
            "{entry_key} [{variant_label}]: field {field} must be non-negative, got {number}"
        )),
        None => errors.push(format!(
            "{entry_key} [{variant_label}]: missing numeric field {field}"
        )),
    }

    errors
}

fn validate_niit(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let mut errors = validate_numeric_field(entry_key, variant_label, value, "threshold");
    let Some(obj) = value.as_object() else {
        return errors;
    };
    match obj.get("rate").and_then(Value::as_f64) {
        Some(rate) if (0.0..=1.0).contains(&rate) => {}
        Some(rate) => errors.push(format!(
            "{entry_key} [{variant_label}]: rate must be between 0 and 1, got {rate}"
        )),
        None => errors.push(format!(
            "{entry_key} [{variant_label}]: missing numeric field rate"
        )),
    }
    errors
}

fn validate_payroll(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let Some(obj) = value.as_object() else {
        return vec![format!("{entry_key} [{variant_label}]: expected object")];
    };

    let mut errors = Vec::new();
    for field in [
        "social_security_rate",
        "social_security_wage_base",
        "self_employment_tax_rate",
        "medicare_rate",
        "self_employment_medicare_rate",
        "additional_medicare_rate",
        "additional_medicare_threshold",
    ] {
        match obj.get(field).and_then(Value::as_f64) {
            Some(number) => {
                if field.ends_with("_rate") && !(0.0..=1.0).contains(&number) {
                    errors.push(format!(
                        "{entry_key} [{variant_label}]: {field} must be between 0 and 1, got {number}"
                    ));
                } else if number < 0.0 {
                    errors.push(format!(
                        "{entry_key} [{variant_label}]: {field} must be non-negative, got {number}"
                    ));
                }
            }
            None => errors.push(format!(
                "{entry_key} [{variant_label}]: missing numeric field {field}"
            )),
        }
    }
    errors
}

fn validate_qbi(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let Some(obj) = value.as_object() else {
        return vec![format!("{entry_key} [{variant_label}]: expected object")];
    };

    let mut errors = Vec::new();
    let deduction_rate = obj.get("deduction_rate").and_then(Value::as_f64);
    if !matches!(deduction_rate, Some(rate) if (0.0..=1.0).contains(&rate)) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: deduction_rate must be between 0 and 1"
        ));
    }
    for field in [
        "threshold",
        "phase_in_range_end",
        "minimum_qbi_deduction",
        "minimum_qbi_amount",
    ] {
        match obj.get(field).and_then(Value::as_f64) {
            Some(number) if number >= 0.0 => {}
            Some(number) => errors.push(format!(
                "{entry_key} [{variant_label}]: {field} must be non-negative, got {number}"
            )),
            None => errors.push(format!(
                "{entry_key} [{variant_label}]: missing numeric field {field}"
            )),
        }
    }
    if let (Some(threshold), Some(phase_in_end)) = (
        obj.get("threshold").and_then(Value::as_f64),
        obj.get("phase_in_range_end").and_then(Value::as_f64),
    ) {
        if phase_in_end < threshold {
            errors.push(format!(
                "{entry_key} [{variant_label}]: phase_in_range_end must be >= threshold"
            ));
        }
    }
    errors
}

fn validate_salt(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let Some(obj) = value.as_object() else {
        return vec![format!("{entry_key} [{variant_label}]: expected object")];
    };

    let mut errors = Vec::new();
    let cap_amount = obj.get("cap_amount").and_then(Value::as_f64);
    let phaseout_threshold = obj.get("phaseout_threshold").and_then(Value::as_f64);
    let phaseout_rate = obj.get("phaseout_rate").and_then(Value::as_f64);
    let floor_amount = obj.get("floor_amount").and_then(Value::as_f64);

    if !matches!(cap_amount, Some(number) if number >= 0.0) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: cap_amount missing or negative"
        ));
    }
    if !matches!(phaseout_threshold, Some(number) if number >= 0.0) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: phaseout_threshold missing or negative"
        ));
    }
    if !matches!(phaseout_rate, Some(number) if (0.0..=1.0).contains(&number)) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: phaseout_rate missing or invalid"
        ));
    }
    if !matches!(floor_amount, Some(number) if number >= 0.0) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: floor_amount missing or negative"
        ));
    }
    if let (Some(cap_amount), Some(floor_amount)) = (cap_amount, floor_amount) {
        if floor_amount > cap_amount {
            errors.push(format!(
                "{entry_key} [{variant_label}]: floor_amount must be <= cap_amount"
            ));
        }
    }

    errors
}

fn validate_medicare_base_premiums(
    entry_key: &str,
    variant_label: &str,
    value: &Value,
) -> Vec<String> {
    let Some(obj) = value.as_object() else {
        return vec![format!("{entry_key} [{variant_label}]: expected object")];
    };

    let mut errors = Vec::new();
    for field in [
        "part_b_standard_monthly_premium",
        "part_b_annual_deductible",
        "part_d_base_beneficiary_premium",
    ] {
        match obj.get(field).and_then(Value::as_f64) {
            Some(number) if number >= 0.0 => {}
            Some(number) => errors.push(format!(
                "{entry_key} [{variant_label}]: {field} must be non-negative, got {number}"
            )),
            None => errors.push(format!(
                "{entry_key} [{variant_label}]: missing numeric field {field}"
            )),
        }
    }

    errors
}

fn validate_social_security_full_retirement_age(
    entry_key: &str,
    variant_label: &str,
    value: &Value,
) -> Vec<String> {
    let Some(obj) = value.as_object() else {
        return vec![format!("{entry_key} [{variant_label}]: expected object")];
    };

    let mut errors = Vec::new();

    match obj.get("benefit_scope").and_then(Value::as_str) {
        Some(scope) if !scope.trim().is_empty() => {}
        _ => errors.push(format!(
            "{entry_key} [{variant_label}]: benefit_scope missing or invalid"
        )),
    }

    if !obj
        .get("january_1_births_use_prior_year")
        .is_some_and(Value::is_boolean)
    {
        errors.push(format!(
            "{entry_key} [{variant_label}]: january_1_births_use_prior_year missing or invalid"
        ));
    }

    let Some(rules) = obj.get("rules").and_then(Value::as_array) else {
        errors.push(format!(
            "{entry_key} [{variant_label}]: missing array field rules"
        ));
        return errors;
    };

    if rules.is_empty() {
        errors.push(format!(
            "{entry_key} [{variant_label}]: rules array is empty"
        ));
        return errors;
    }

    let mut expected_min: Option<u64> = None;
    let mut open_ended_seen = false;

    for (index, rule) in rules.iter().enumerate() {
        let Some(rule_obj) = rule.as_object() else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: rules[{index}] is not an object"
            ));
            continue;
        };

        let birth_year_min = rule_obj.get("birth_year_min").and_then(Value::as_u64);
        let birth_year_max = rule_obj.get("birth_year_max").and_then(Value::as_u64);
        let age_years = rule_obj
            .get("full_retirement_age_years")
            .and_then(Value::as_u64);
        let age_months = rule_obj
            .get("full_retirement_age_months")
            .and_then(Value::as_u64);

        if !matches!(age_years, Some(years) if (65..=67).contains(&years)) {
            errors.push(format!(
                "{entry_key} [{variant_label}]: rules[{index}].full_retirement_age_years missing or invalid"
            ));
        }
        if !matches!(age_months, Some(months) if months < 12 && months % 2 == 0) {
            errors.push(format!(
                "{entry_key} [{variant_label}]: rules[{index}].full_retirement_age_months missing or invalid"
            ));
        }

        if open_ended_seen {
            errors.push(format!(
                "{entry_key} [{variant_label}]: only the final rule may have birth_year_max = null"
            ));
            continue;
        }

        match (birth_year_min, birth_year_max, expected_min) {
            (None, Some(max), None) => {
                expected_min = Some(max + 1);
            }
            (Some(min), Some(max), Some(expected)) => {
                if min != expected {
                    errors.push(format!(
                        "{entry_key} [{variant_label}]: rules[{index}] birth_year_min {min} does not continue from previous max {}",
                        expected - 1
                    ));
                }
                if max < min {
                    errors.push(format!(
                        "{entry_key} [{variant_label}]: rules[{index}] birth_year_max {max} must be >= birth_year_min {min}"
                    ));
                } else {
                    expected_min = Some(max + 1);
                }
            }
            (Some(_), None, Some(expected)) => {
                if birth_year_min != Some(expected) {
                    errors.push(format!(
                        "{entry_key} [{variant_label}]: final open-ended rule must begin at birth year {}",
                        expected
                    ));
                }
                open_ended_seen = true;
            }
            _ => errors.push(format!(
                "{entry_key} [{variant_label}]: rules[{index}] birth_year_min/birth_year_max structure is invalid"
            )),
        }
    }

    if !open_ended_seen {
        errors.push(format!(
            "{entry_key} [{variant_label}]: final rule must be open-ended with birth_year_max = null"
        ));
    }

    errors
}

fn validate_age_distribution(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let Some(items) = value.as_array() else {
        return vec![format!(
            "{entry_key} [{variant_label}]: expected array of age/distribution pairs"
        )];
    };
    if items.is_empty() {
        return vec![format!("{entry_key} [{variant_label}]: array is empty")];
    }

    let mut errors = Vec::new();
    let mut previous_age = None;
    let mut previous_dp = None;
    for (index, item) in items.iter().enumerate() {
        let Some(obj) = item.as_object() else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} is not an object",
                index
            ));
            continue;
        };
        let age = obj.get("age").and_then(Value::as_u64);
        let distribution_period = obj.get("distribution_period").and_then(Value::as_f64);
        let Some(age) = age else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} missing age",
                index
            ));
            continue;
        };
        let Some(distribution_period) = distribution_period else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} missing distribution_period",
                index
            ));
            continue;
        };
        if distribution_period <= 0.0 {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} distribution_period must be positive",
                index
            ));
        }
        if let Some(previous_age) = previous_age {
            if age <= previous_age {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: ages must increase strictly"
                ));
            }
        }
        if let Some(previous_dp) = previous_dp {
            if distribution_period > previous_dp {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: distribution_period should not increase with age"
                ));
            }
        }
        previous_age = Some(age);
        previous_dp = Some(distribution_period);
    }
    errors
}

fn validate_joint_distribution(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let Some(items) = value.as_array() else {
        return vec![format!(
            "{entry_key} [{variant_label}]: expected array of joint distribution entries"
        )];
    };
    if items.is_empty() {
        return vec![format!("{entry_key} [{variant_label}]: array is empty")];
    }

    let mut errors = Vec::new();
    let mut seen_pairs = BTreeSet::new();
    for (index, item) in items.iter().enumerate() {
        let Some(obj) = item.as_object() else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} is not an object",
                index
            ));
            continue;
        };
        let owner_age = obj.get("owner_age").and_then(Value::as_u64);
        let spouse_age = obj.get("spouse_age").and_then(Value::as_u64);
        let distribution_period = obj.get("distribution_period").and_then(Value::as_f64);
        let (Some(owner_age), Some(spouse_age), Some(distribution_period)) =
            (owner_age, spouse_age, distribution_period)
        else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} missing required fields",
                index
            ));
            continue;
        };

        if distribution_period <= 0.0 {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} distribution_period must be positive",
                index
            ));
        }
        if !seen_pairs.insert((owner_age, spouse_age)) {
            errors.push(format!(
                "{entry_key} [{variant_label}]: duplicate owner/spouse age pair ({owner_age}, {spouse_age})"
            ));
        }
    }

    errors
}

fn validate_distribution_rules(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let Some(obj) = value.as_object() else {
        return vec![format!("{entry_key} [{variant_label}]: expected object")];
    };

    let mut errors = Vec::new();
    let required_beginning = match obj.get("required_beginning").and_then(Value::as_object) {
        Some(value) => value,
        None => {
            errors.push(format!(
                "{entry_key} [{variant_label}]: missing object field required_beginning"
            ));
            return errors;
        }
    };
    let account_applicability = match obj.get("account_applicability").and_then(Value::as_object) {
        Some(value) => value,
        None => {
            errors.push(format!(
                "{entry_key} [{variant_label}]: missing object field account_applicability"
            ));
            return errors;
        }
    };
    let beneficiary_distribution = match obj
        .get("beneficiary_distribution")
        .and_then(Value::as_object)
    {
        Some(value) => value,
        None => {
            errors.push(format!(
                "{entry_key} [{variant_label}]: missing object field beneficiary_distribution"
            ));
            return errors;
        }
    };

    match required_beginning
        .get("start_age_rules")
        .and_then(Value::as_array)
    {
        Some(items) if !items.is_empty() => {
            for (index, item) in items.iter().enumerate() {
                let Some(rule) = item.as_object() else {
                    errors.push(format!(
                        "{entry_key} [{variant_label}]: required_beginning.start_age_rules[{index}] must be an object"
                    ));
                    continue;
                };
                if rule.get("start_age").and_then(Value::as_u64).is_none() {
                    errors.push(format!(
                        "{entry_key} [{variant_label}]: required_beginning.start_age_rules[{index}].start_age missing or invalid"
                    ));
                }
                if let Some(status) = rule.get("guidance_status") {
                    if !status.is_string() && !status.is_null() {
                        errors.push(format!(
                            "{entry_key} [{variant_label}]: required_beginning.start_age_rules[{index}].guidance_status must be a string or null"
                        ));
                    }
                }
                if let Some(notes) = rule.get("notes") {
                    if !notes.is_string() && !notes.is_null() {
                        errors.push(format!(
                            "{entry_key} [{variant_label}]: required_beginning.start_age_rules[{index}].notes must be a string or null"
                        ));
                    }
                }
            }
        }
        Some(_) => errors.push(format!(
            "{entry_key} [{variant_label}]: required_beginning.start_age_rules must not be empty"
        )),
        None => errors.push(format!(
            "{entry_key} [{variant_label}]: missing array field required_beginning.start_age_rules"
        )),
    }

    if required_beginning
        .get("first_distribution_deadline")
        .and_then(Value::as_str)
        .is_none()
    {
        errors.push(format!(
            "{entry_key} [{variant_label}]: required_beginning.first_distribution_deadline missing or invalid"
        ));
    }

    match required_beginning
        .get("still_working_exception")
        .and_then(Value::as_object)
    {
        Some(still_working) => {
            for field in ["eligible_plan_categories", "eligible_account_types"] {
                match still_working.get(field).and_then(Value::as_array) {
                    Some(items) if !items.is_empty() => {}
                    Some(_) => errors.push(format!(
                        "{entry_key} [{variant_label}]: required_beginning.still_working_exception.{field} must not be empty"
                    )),
                    None => errors.push(format!(
                        "{entry_key} [{variant_label}]: missing array field required_beginning.still_working_exception.{field}"
                    )),
                }
            }
            if still_working
                .get("disallowed_for_five_percent_owners")
                .and_then(Value::as_bool)
                .is_none()
            {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: required_beginning.still_working_exception.disallowed_for_five_percent_owners missing or invalid"
                ));
            }
        }
        None => errors.push(format!(
            "{entry_key} [{variant_label}]: missing object field required_beginning.still_working_exception"
        )),
    }

    for field in [
        "owner_required_account_types",
        "owner_exempt_account_types",
        "inherited_account_types",
    ] {
        match account_applicability.get(field).and_then(Value::as_array) {
            Some(items) if !items.is_empty() => {}
            Some(_) => errors.push(format!(
                "{entry_key} [{variant_label}]: account_applicability.{field} must not be empty"
            )),
            None => errors.push(format!(
                "{entry_key} [{variant_label}]: missing array field account_applicability.{field}"
            )),
        }
    }

    if !account_applicability.contains_key("designated_roth_owner_exemption_effective_year")
        || (!account_applicability["designated_roth_owner_exemption_effective_year"].is_null()
            && account_applicability["designated_roth_owner_exemption_effective_year"]
                .as_u64()
                .is_none())
    {
        errors.push(format!(
            "{entry_key} [{variant_label}]: account_applicability.designated_roth_owner_exemption_effective_year missing or invalid"
        ));
    }
    if account_applicability
        .get("supports_pre_1987_403b_exclusion")
        .and_then(Value::as_bool)
        .is_none()
    {
        errors.push(format!(
            "{entry_key} [{variant_label}]: account_applicability.supports_pre_1987_403b_exclusion missing or invalid"
        ));
    }
    match account_applicability
        .get("pre_1987_403b")
        .and_then(Value::as_object)
    {
        Some(pre_1987) => {
            if pre_1987
                .get("exclude_until_age")
                .and_then(Value::as_u64)
                .is_none()
            {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: account_applicability.pre_1987_403b.exclude_until_age missing or invalid"
                ));
            }
        }
        None => errors.push(format!(
            "{entry_key} [{variant_label}]: missing object field account_applicability.pre_1987_403b"
        )),
    }

    for field in [
        "beneficiary_categories",
        "recognized_beneficiary_classes",
        "eligible_designated_beneficiary_classes",
    ] {
        match beneficiary_distribution.get(field).and_then(Value::as_array) {
            Some(items) if !items.is_empty() => {}
            Some(_) => errors.push(format!(
                "{entry_key} [{variant_label}]: beneficiary_distribution.{field} must not be empty"
            )),
            None => errors.push(format!(
                "{entry_key} [{variant_label}]: missing array field beneficiary_distribution.{field}"
            )),
        }
    }
    match beneficiary_distribution
        .get("life_expectancy_method_by_class")
        .and_then(Value::as_object)
    {
        Some(methods) if !methods.is_empty() => {}
        Some(_) => errors.push(format!(
            "{entry_key} [{variant_label}]: beneficiary_distribution.life_expectancy_method_by_class must not be empty"
        )),
        None => errors.push(format!(
            "{entry_key} [{variant_label}]: missing object field beneficiary_distribution.life_expectancy_method_by_class"
        )),
    }
    if beneficiary_distribution
        .get("minor_child_majority_age")
        .and_then(Value::as_u64)
        .is_none()
    {
        errors.push(format!(
            "{entry_key} [{variant_label}]: beneficiary_distribution.minor_child_majority_age missing or invalid"
        ));
    }
    if beneficiary_distribution
        .get("spouse_delay_allowed")
        .and_then(Value::as_bool)
        .is_none()
    {
        errors.push(format!(
            "{entry_key} [{variant_label}]: beneficiary_distribution.spouse_delay_allowed missing or invalid"
        ));
    }
    match beneficiary_distribution
        .get("ten_year_rule")
        .and_then(Value::as_object)
    {
        Some(ten_year_rule) => {
            if ten_year_rule
                .get("terminal_year")
                .and_then(Value::as_u64)
                .is_none()
            {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: beneficiary_distribution.ten_year_rule.terminal_year missing or invalid"
                ));
            }
            if ten_year_rule
                .get("annual_distributions_required_when_owner_died_on_or_after_rbd")
                .and_then(Value::as_bool)
                .is_none()
            {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: beneficiary_distribution.ten_year_rule annual distribution flag missing or invalid"
                ));
            }
        }
        None => errors.push(format!(
            "{entry_key} [{variant_label}]: missing object field beneficiary_distribution.ten_year_rule"
        )),
    }
    match beneficiary_distribution
        .get("non_designated_beneficiary_rules")
        .and_then(Value::as_object)
    {
        Some(non_designated) => {
            for field in [
                "when_owner_died_before_required_beginning_date",
                "when_owner_died_on_or_after_required_beginning_date",
            ] {
                if non_designated.get(field).and_then(Value::as_str).is_none() {
                    errors.push(format!(
                        "{entry_key} [{variant_label}]: beneficiary_distribution.non_designated_beneficiary_rules.{field} missing or invalid"
                    ));
                }
            }
        }
        None => errors.push(format!(
            "{entry_key} [{variant_label}]: missing object field beneficiary_distribution.non_designated_beneficiary_rules"
        )),
    }
    if beneficiary_distribution
        .get("relief_years")
        .and_then(Value::as_array)
        .is_none()
    {
        errors.push(format!(
            "{entry_key} [{variant_label}]: missing array field beneficiary_distribution.relief_years"
        ));
    }

    errors
}

fn validate_ss_taxation(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let Some(obj) = value.as_object() else {
        return vec![format!("{entry_key} [{variant_label}]: expected object")];
    };

    let mut errors = Vec::new();
    let base_amount = obj.get("base_amount").and_then(Value::as_f64);
    let upper_amount = obj.get("upper_amount").and_then(Value::as_f64);
    let below_pct = obj
        .get("max_taxable_pct_below_upper")
        .and_then(Value::as_f64);
    let above_pct = obj
        .get("max_taxable_pct_above_upper")
        .and_then(Value::as_f64);

    if !matches!(base_amount, Some(number) if number >= 0.0) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: base_amount missing or negative"
        ));
    }
    if !matches!(upper_amount, Some(number) if number >= 0.0) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: upper_amount missing or negative"
        ));
    }
    if let (Some(base_amount), Some(upper_amount)) = (base_amount, upper_amount) {
        if upper_amount < base_amount {
            errors.push(format!(
                "{entry_key} [{variant_label}]: upper_amount must be >= base_amount"
            ));
        }
    }
    if !matches!(below_pct, Some(number) if (0.0..=1.0).contains(&number)) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: max_taxable_pct_below_upper missing or invalid"
        ));
    }
    if !matches!(above_pct, Some(number) if (0.0..=1.0).contains(&number)) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: max_taxable_pct_above_upper missing or invalid"
        ));
    }

    errors
}

fn validate_ss_retirement_earnings_test(
    entry_key: &str,
    variant_label: &str,
    value: &Value,
) -> Vec<String> {
    let Some(obj) = value.as_object() else {
        return vec![format!("{entry_key} [{variant_label}]: expected object")];
    };

    let mut errors = Vec::new();
    for field in [
        "under_fra_annual_exempt_amount",
        "under_fra_monthly_exempt_amount",
        "year_of_fra_annual_exempt_amount",
        "year_of_fra_monthly_exempt_amount",
    ] {
        match obj.get(field).and_then(Value::as_f64) {
            Some(number) if number >= 0.0 => {}
            Some(number) => errors.push(format!(
                "{entry_key} [{variant_label}]: {field} must be non-negative, got {number}"
            )),
            None => errors.push(format!(
                "{entry_key} [{variant_label}]: missing numeric field {field}"
            )),
        }
    }
    for field in ["under_fra_reduction_rate", "year_of_fra_reduction_rate"] {
        match obj.get(field).and_then(Value::as_f64) {
            Some(number) if (0.0..=1.0).contains(&number) => {}
            Some(number) => errors.push(format!(
                "{entry_key} [{variant_label}]: {field} must be between 0 and 1, got {number}"
            )),
            None => errors.push(format!(
                "{entry_key} [{variant_label}]: missing numeric field {field}"
            )),
        }
    }

    errors
}

fn validate_irmaa(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let Some(obj) = value.as_object() else {
        return vec![format!("{entry_key} [{variant_label}]: expected object")];
    };

    let mut errors = Vec::new();
    let filing_status = obj.get("filing_status").and_then(Value::as_str);
    if let Some("married_filing_separately") = filing_status {
        if obj
            .get("lived_with_spouse_during_year")
            .and_then(Value::as_bool)
            .is_none()
        {
            errors.push(format!(
                "{entry_key} [{variant_label}]: married_filing_separately IRMAA values must include lived_with_spouse_during_year"
            ));
        }
    }

    if !matches!(
        obj.get("base_part_b_premium").and_then(Value::as_f64),
        Some(number) if number > 0.0
    ) {
        errors.push(format!(
            "{entry_key} [{variant_label}]: base_part_b_premium missing or invalid"
        ));
    }

    let Some(brackets) = obj.get("brackets").and_then(Value::as_array) else {
        errors.push(format!(
            "{entry_key} [{variant_label}]: missing brackets array"
        ));
        return errors;
    };
    if brackets.is_empty() {
        errors.push(format!(
            "{entry_key} [{variant_label}]: brackets array is empty"
        ));
        return errors;
    }

    let mut previous_max: Option<f64> = None;
    let mut previous_surcharge: Option<f64> = None;
    for (index, bracket) in brackets.iter().enumerate() {
        let Some(bracket) = bracket.as_object() else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: bracket {} is not an object",
                index
            ));
            continue;
        };
        let min = bracket.get("magi_min").and_then(Value::as_f64);
        let max = bracket.get("magi_max").and_then(Value::as_f64);
        let surcharge = bracket.get("monthly_surcharge").and_then(Value::as_f64);

        let (Some(min), Some(surcharge)) = (min, surcharge) else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: bracket {} missing required numeric fields",
                index
            ));
            continue;
        };

        if let Some(previous_max) = previous_max {
            if (min - previous_max).abs() > f64::EPSILON {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: bracket {} min {} does not continue from previous max {}",
                    index, min, previous_max
                ));
            }
        }
        if let Some(max) = max {
            if max <= min {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: bracket {} max {} must be greater than min {}",
                    index, max, min
                ));
            }
        } else if index != brackets.len() - 1 {
            errors.push(format!(
                "{entry_key} [{variant_label}]: only final IRMAA bracket may have magi_max = null"
            ));
        }
        if surcharge < 0.0 {
            errors.push(format!(
                "{entry_key} [{variant_label}]: bracket {} surcharge must be non-negative",
                index
            ));
        }
        if let Some(previous_surcharge) = previous_surcharge {
            if surcharge < previous_surcharge {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: surcharges must be non-decreasing"
                ));
            }
        }

        previous_max = max;
        previous_surcharge = Some(surcharge);
    }

    errors
}

fn validate_mortality(entry_key: &str, variant_label: &str, value: &Value) -> Vec<String> {
    let Some(items) = value.as_array() else {
        return vec![format!(
            "{entry_key} [{variant_label}]: expected array of mortality rows"
        )];
    };
    if items.is_empty() {
        return vec![format!("{entry_key} [{variant_label}]: array is empty")];
    }

    let mut errors = Vec::new();
    let mut previous_age = None;
    let mut previous_qx = None;
    for (index, item) in items.iter().enumerate() {
        let Some(obj) = item.as_object() else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} is not an object",
                index
            ));
            continue;
        };
        let age = obj.get("age").and_then(Value::as_u64);
        let qx = obj.get("qx").and_then(Value::as_f64);
        let (Some(age), Some(qx)) = (age, qx) else {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} missing age or qx",
                index
            ));
            continue;
        };
        if !(0.0..=1.0).contains(&qx) {
            errors.push(format!(
                "{entry_key} [{variant_label}]: row {} qx {} out of range",
                index, qx
            ));
        }
        if let Some(previous_age) = previous_age {
            if age <= previous_age {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: ages must increase strictly"
                ));
            }
        }
        if let Some(previous_qx) = previous_qx {
            if qx < previous_qx {
                errors.push(format!(
                    "{entry_key} [{variant_label}]: qx should not decrease with age"
                ));
            }
        }
        previous_age = Some(age);
        previous_qx = Some(qx);
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalization_sorts_object_keys() {
        let value = serde_json::json!({
            "b": 1,
            "a": { "d": 2, "c": 3 }
        });

        let canonical = canonicalize(&value);
        let bytes = serde_json::to_vec(&canonical).unwrap();

        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            r#"{"a":{"c":3,"d":2},"b":1}"#
        );
    }

    #[test]
    fn fnv_checksum_is_stable() {
        assert_eq!(fnv1a64(b"entropyfa"), "b1b615357e506187");
    }
}
