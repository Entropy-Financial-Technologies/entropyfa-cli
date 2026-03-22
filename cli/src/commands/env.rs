use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::{json, Map, Value};

use crate::output;
use crate::reference_paths::{
    detect_install_profile, resolve_reference_root, ResolvedReferenceRoot,
};

#[derive(Deserialize)]
struct ReferenceManifest {
    bundle_version: Option<String>,
    generated_at: Option<String>,
    categories: Option<Value>,
    pack_count: Option<u64>,
}

pub fn run_env(json_output: bool, explicit_reference_root: Option<PathBuf>) {
    let binary_path = std::env::current_exe().ok();
    let home_dir = std::env::var_os("HOME").map(PathBuf::from);
    let install_profile = detect_install_profile(binary_path.as_deref(), home_dir.as_deref());
    let resolved_reference_root = resolve_reference_root(
        explicit_reference_root,
        std::env::var("ENTROPYFA_REFERENCE_ROOT").ok(),
        home_dir,
        install_profile,
    );
    let reference = collect_reference_state(&resolved_reference_root);
    let data = json!({
        "version": env!("CARGO_PKG_VERSION"),
        "binary_path": binary_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        "reference": reference,
        "install_profile": install_profile.as_str(),
    });

    if json_output {
        output::print_success(data);
        return;
    }

    print_human_summary(&data);
}

fn collect_reference_state(resolved_reference_root: &ResolvedReferenceRoot) -> Value {
    let manifest_path = resolved_reference_root.path.join("manifest.json");
    let manifest = load_manifest(&manifest_path);
    let packs_present = manifest["pack_count"].as_u64().unwrap_or(0) > 0;

    json!({
        "resolved_root": resolved_reference_root.path.display().to_string(),
        "resolution_source": resolved_reference_root.source,
        "packs_present": packs_present,
        "manifest": manifest,
    })
}

fn load_manifest(path: &Path) -> Value {
    let Ok(contents) = fs::read_to_string(path) else {
        return Value::Null;
    };
    let Ok(manifest) = serde_json::from_str::<ReferenceManifest>(&contents) else {
        return Value::Null;
    };

    let mut obj = Map::new();
    obj.insert(
        "bundle_version".into(),
        manifest
            .bundle_version
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    obj.insert(
        "generated_at".into(),
        manifest
            .generated_at
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    obj.insert(
        "categories".into(),
        manifest.categories.unwrap_or(Value::Null),
    );
    obj.insert(
        "pack_count".into(),
        manifest
            .pack_count
            .map(|value| json!(value))
            .unwrap_or(Value::Null),
    );
    Value::Object(obj)
}

fn print_human_summary(data: &Value) {
    let version = data["version"].as_str().unwrap_or("unknown");
    let binary_path = data["binary_path"].as_str().unwrap_or("unknown");
    let install_profile = data["install_profile"].as_str().unwrap_or("unknown");
    let reference = &data["reference"];

    println!("Version: {version}");
    println!("Binary path: {binary_path}");
    println!("Install profile: {install_profile}");
    println!(
        "Reference root: {} ({})",
        reference["resolved_root"].as_str().unwrap_or("unknown"),
        reference["resolution_source"].as_str().unwrap_or("unknown"),
    );
    println!(
        "Reference packs present: {}",
        yes_no(reference["packs_present"].as_bool().unwrap_or(false))
    );

    if let Some(manifest) = reference["manifest"].as_object() {
        println!(
            "Reference bundle version: {}",
            manifest
                .get("bundle_version")
                .and_then(Value::as_str)
                .unwrap_or("unknown"),
        );
        println!(
            "Reference pack count: {}",
            manifest
                .get("pack_count")
                .map(display_json_scalar)
                .unwrap_or_else(|| "unknown".to_string()),
        );
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn display_json_scalar(value: &Value) -> String {
    match value {
        Value::Null => "unknown".to_string(),
        Value::String(s) => s.clone(),
        _ => value.to_string(),
    }
}
