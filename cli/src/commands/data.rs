use std::fs;

use entropyfa_engine::data;
use entropyfa_engine::data::taxonomy;
use entropyfa_engine::data::types::LookupParams;
use entropyfa_engine::data_pipeline;
use serde_json::{json, Value};

use crate::output;

/// Handle `data lookup --category <cat> --key <key> [--year <y>] [--filing-status <fs>]`
pub fn run_lookup(
    category: &str,
    key: &str,
    year: u32,
    filing_status: Option<&str>,
    lived_with_spouse_during_year: Option<bool>,
) {
    let entries = taxonomy::all_entries();
    let entry = entries
        .iter()
        .find(|e| e.category == category && e.key == key);

    let entry = match entry {
        Some(e) => e,
        None => {
            output::print_error(
                "not_found",
                &format!("No data entry for category={category}, key={key}"),
            );
            std::process::exit(1);
        }
    };

    if !entry.years.contains(&year) {
        output::print_error(
            "unsupported_year",
            &format!(
                "Year {year} not available for {category}/{key}. Available: {:?}",
                entry.years
            ),
        );
        std::process::exit(1);
    }

    let params = LookupParams {
        filing_status: filing_status.map(str::to_string),
        lived_with_spouse_during_year,
    };

    match data::lookup(category, key, year, &params) {
        Ok(result) => output::print_success(build_lookup_response(category, key, year, result)),
        Err(error) => {
            output::print_error("lookup_error", &error.to_string());
            std::process::exit(1);
        }
    }
}

fn build_lookup_response(category: &str, key: &str, year: u32, value: Value) -> Value {
    let mut response = json!({
        "category": category,
        "key": key,
        "year": year,
        "value": value,
    });

    let Ok(registry) = data_pipeline::load_registry(&data_pipeline::default_metadata_path()) else {
        return response;
    };
    let Some(entry) = registry
        .entries
        .iter()
        .find(|entry| entry.category == category && entry.key == key)
    else {
        return response;
    };

    let (pipeline_reviewed, source_origin, sources) =
        load_lookup_sources(year, category, key, entry);
    let response_obj = response
        .as_object_mut()
        .expect("lookup response should be an object");
    response_obj.insert(
        "verification_status".into(),
        json!(entry.verification_status),
    );
    response_obj.insert("completeness".into(), json!(entry.completeness));
    response_obj.insert("pipeline_reviewed".into(), json!(pipeline_reviewed));
    response_obj.insert("source_origin".into(), json!(source_origin));
    response_obj.insert("sources".into(), Value::Array(sources));
    if let Some(notes) = &entry.notes {
        response_obj.insert("notes".into(), json!(notes));
    }

    response
}

fn load_lookup_sources(
    year: u32,
    category: &str,
    key: &str,
    entry: &data_pipeline::RegistryEntry,
) -> (bool, &'static str, Vec<Value>) {
    let reviewed_path = data_pipeline::default_reviewed_root(year)
        .join(category)
        .join(format!("{key}.json"));

    if let Ok(contents) = fs::read_to_string(&reviewed_path) {
        if let Ok(reviewed) = serde_json::from_str::<Value>(&contents) {
            if let Some(accepted_sources) =
                reviewed.get("accepted_sources").and_then(Value::as_array)
            {
                let sources = accepted_sources
                    .iter()
                    .map(|source| {
                        json!({
                            "authority": source.get("organization").cloned().unwrap_or(Value::Null),
                            "title": source.get("title").cloned().unwrap_or(Value::Null),
                            "url": source.get("url").cloned().unwrap_or(Value::Null),
                            "published_at": source.get("published_at").cloned().unwrap_or(Value::Null),
                            "locator": source.get("locator").cloned().unwrap_or(Value::Null),
                            "source_class": source.get("source_class").cloned().unwrap_or(Value::Null),
                            "counts_toward_status": source
                                .get("counts_toward_status")
                                .cloned()
                                .unwrap_or(Value::Null),
                        })
                    })
                    .collect::<Vec<_>>();
                return (true, "reviewed_artifact", sources);
            }
        }
    }

    let sources = entry
        .source_documents
        .iter()
        .map(|source| {
            json!({
                "authority": source.authority,
                "title": source.title,
                "url": Value::Null,
                "published_at": Value::Null,
                "locator": source.section,
                "source_class": Value::Null,
                "counts_toward_status": Value::Null,
            })
        })
        .collect::<Vec<_>>();
    (false, "registry_metadata", sources)
}

/// Handle `data coverage [--category <cat>] [--year <y>]`
pub fn run_coverage(category: Option<&str>, year: Option<u32>) {
    let entries = taxonomy::all_entries();
    let filtered: Vec<_> = entries
        .into_iter()
        .filter(|e| {
            if let Some(cat) = category {
                if e.category != cat {
                    return false;
                }
            }
            if let Some(y) = year {
                if !e.years.contains(&y) {
                    return false;
                }
            }
            true
        })
        .map(|e| {
            json!({
                "category": e.category,
                "key": e.key,
                "years": e.years,
                "params": e.params,
                "description": e.description,
                "tags": e.tags,
                "related_topics": e.related_topics,
            })
        })
        .collect();

    output::print_success(json!({
        "count": filtered.len(),
        "entries": filtered,
    }));
}
