use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

pub const PLATFORM_REFERENCE_ROOT: &str = "/opt/entropyfa/reference";
const USER_REFERENCE_ROOT_SUFFIX: &str = ".entropyfa/reference";
const MANAGED_REFERENCE_MARKER: &str = ".entropyfa-managed";
const INSTALL_METADATA_FILE_NAME: &str = "entropyfa.install.json";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstallProfile {
    BinaryOnly,
    Full,
    Platform,
    Unknown,
}

impl InstallProfile {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BinaryOnly => "binary-only",
            Self::Full => "full",
            Self::Platform => "platform",
            Self::Unknown => "unknown",
        }
    }

    pub fn from_env_value(value: &str) -> Option<Self> {
        match value.trim() {
            "binary-only" => Some(Self::BinaryOnly),
            "full" => Some(Self::Full),
            "platform" => Some(Self::Platform),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct InstallMetadata {
    pub install_profile: String,
    pub reference_root: Option<PathBuf>,
}

impl InstallMetadata {
    pub fn parsed_profile(&self) -> Option<InstallProfile> {
        InstallProfile::from_env_value(&self.install_profile)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ResolvedReferenceRoot {
    pub path: PathBuf,
    pub source: &'static str,
}

pub fn load_install_metadata(current_exe: &Path) -> Option<InstallMetadata> {
    let metadata_path = install_metadata_path(current_exe)?;
    let contents = fs::read_to_string(metadata_path).ok()?;
    let metadata = serde_json::from_str::<InstallMetadata>(&contents).ok()?;
    metadata.parsed_profile()?;
    Some(metadata)
}

pub fn resolve_reference_root(
    explicit: Option<PathBuf>,
    env_var: Option<String>,
    metadata_root: Option<PathBuf>,
    home_dir: Option<PathBuf>,
    profile_hint: InstallProfile,
) -> ResolvedReferenceRoot {
    if let Some(path) = explicit {
        return ResolvedReferenceRoot {
            path,
            source: "flag",
        };
    }

    if let Some(path) = env_var.filter(|value| !value.trim().is_empty()) {
        return ResolvedReferenceRoot {
            path: PathBuf::from(path),
            source: "env",
        };
    }

    if let Some(path) = metadata_root {
        return ResolvedReferenceRoot {
            path,
            source: "install-metadata",
        };
    }

    let path = match profile_hint {
        InstallProfile::Platform => PathBuf::from(PLATFORM_REFERENCE_ROOT),
        InstallProfile::BinaryOnly | InstallProfile::Full | InstallProfile::Unknown => home_dir
            .unwrap_or_else(|| PathBuf::from("."))
            .join(USER_REFERENCE_ROOT_SUFFIX),
    };

    ResolvedReferenceRoot {
        path,
        source: "default",
    }
}

pub fn resolve_compute_reference_root(
    current_exe: Option<&Path>,
    home_dir: Option<&Path>,
) -> ResolvedReferenceRoot {
    resolve_compute_reference_root_with_context(
        current_exe,
        home_dir,
        std::env::var("ENTROPYFA_REFERENCE_ROOT").ok(),
        std::env::var("ENTROPYFA_INSTALL_PROFILE").ok(),
    )
}

fn resolve_compute_reference_root_with_context(
    current_exe: Option<&Path>,
    home_dir: Option<&Path>,
    reference_root_env: Option<String>,
    explicit_profile: Option<String>,
) -> ResolvedReferenceRoot {
    let install_metadata = current_exe.and_then(load_install_metadata);
    let install_profile =
        detect_install_profile(explicit_profile.as_deref(), current_exe, home_dir);

    resolve_reference_root(
        None,
        reference_root_env,
        install_metadata
            .as_ref()
            .and_then(|metadata| metadata.reference_root.clone()),
        home_dir.map(Path::to_path_buf),
        install_profile,
    )
}

pub fn detect_install_profile(
    explicit_profile: Option<&str>,
    current_exe: Option<&Path>,
    home_dir: Option<&Path>,
) -> InstallProfile {
    let install_metadata_profile = current_exe
        .and_then(load_install_metadata)
        .and_then(|metadata| metadata.parsed_profile());
    let user_reference_root = home_dir.map(|home| home.join(USER_REFERENCE_ROOT_SUFFIX));

    detect_install_profile_with_context(
        explicit_profile,
        current_exe,
        install_metadata_profile,
        user_reference_root.as_deref(),
        Path::new(PLATFORM_REFERENCE_ROOT),
    )
}

pub fn detect_installed_profile(
    current_exe: Option<&Path>,
    home_dir: Option<&Path>,
) -> InstallProfile {
    detect_install_profile(None, current_exe, home_dir)
}

fn detect_install_profile_with_context(
    explicit_profile: Option<&str>,
    current_exe: Option<&Path>,
    install_metadata_profile: Option<InstallProfile>,
    user_reference_root: Option<&Path>,
    platform_reference_root: &Path,
) -> InstallProfile {
    if let Some(explicit_profile) = explicit_profile.and_then(InstallProfile::from_env_value) {
        return explicit_profile;
    }

    if let Some(install_metadata_profile) = install_metadata_profile {
        return install_metadata_profile;
    }

    let Some(current_exe) = current_exe else {
        return InstallProfile::Unknown;
    };

    if current_exe.starts_with("/opt/entropyfa/") {
        return InstallProfile::Platform;
    }

    if current_exe == Path::new("/usr/local/bin/entropyfa")
        && has_managed_reference_root(platform_reference_root)
    {
        return InstallProfile::Platform;
    }

    if let Some(user_reference_root) = user_reference_root {
        if let Some(user_install_root) = user_reference_root.parent() {
            if current_exe.starts_with(user_install_root) {
                if has_managed_reference_root(user_reference_root) {
                    return InstallProfile::Full;
                }

                return InstallProfile::BinaryOnly;
            }
        }
    }

    InstallProfile::BinaryOnly
}

fn install_metadata_path(current_exe: &Path) -> Option<PathBuf> {
    current_exe
        .parent()
        .map(|parent| parent.join(INSTALL_METADATA_FILE_NAME))
}

fn has_managed_reference_root(path: &Path) -> bool {
    path.join(MANAGED_REFERENCE_MARKER).is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
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
            "entropyfa-reference-paths-{label}-{}-{nonce}",
            timestamp
        ));
        fs::create_dir_all(&path).expect("temp test directory should be creatable");
        path
    }

    fn touch_managed_reference_marker(path: &Path) {
        fs::create_dir_all(path).expect("reference root should be creatable");
        fs::write(path.join(MANAGED_REFERENCE_MARKER), b"managed")
            .expect("managed reference marker should be writable");
    }

    fn write_install_metadata(path: &Path, profile: &str, reference_root: Option<&Path>) {
        let metadata_dir = path.parent().expect("binary path should have parent");
        fs::create_dir_all(metadata_dir).expect("metadata directory should exist");
        let reference_root_json = reference_root
            .map(|root| format!("\"{}\"", root.display()))
            .unwrap_or_else(|| "null".to_string());
        fs::write(
            metadata_dir.join(INSTALL_METADATA_FILE_NAME),
            format!(
                "{{\"install_profile\":\"{profile}\",\"reference_root\":{reference_root_json}}}"
            ),
        )
        .expect("install metadata should be writable");
    }

    #[test]
    fn load_install_metadata_reads_sidecar() {
        let install_dir = unique_temp_dir("install-metadata");
        let binary_path = install_dir.join("bin/entropyfa");
        let reference_root = install_dir.join("reference");
        write_install_metadata(&binary_path, "full", Some(&reference_root));

        let metadata = load_install_metadata(&binary_path).expect("metadata should load");

        assert_eq!(
            metadata,
            InstallMetadata {
                install_profile: "full".to_string(),
                reference_root: Some(reference_root),
            }
        );
    }

    #[test]
    fn resolve_reference_root_prefers_explicit_path() {
        let resolved = resolve_reference_root(
            Some(PathBuf::from("/tmp/explicit")),
            Some("/tmp/env".to_string()),
            Some(PathBuf::from("/tmp/install-metadata")),
            Some(PathBuf::from("/home/test")),
            InstallProfile::BinaryOnly,
        );

        assert_eq!(
            resolved,
            ResolvedReferenceRoot {
                path: PathBuf::from("/tmp/explicit"),
                source: "flag",
            }
        );
    }

    #[test]
    fn resolve_reference_root_uses_install_metadata_before_defaults() {
        let resolved = resolve_reference_root(
            None,
            None,
            Some(PathBuf::from("/tmp/install-metadata")),
            Some(PathBuf::from("/home/test")),
            InstallProfile::BinaryOnly,
        );

        assert_eq!(
            resolved,
            ResolvedReferenceRoot {
                path: PathBuf::from("/tmp/install-metadata"),
                source: "install-metadata",
            }
        );
    }

    #[test]
    fn resolve_reference_root_uses_platform_default_for_platform_profile() {
        let resolved = resolve_reference_root(None, None, None, None, InstallProfile::Platform);

        assert_eq!(
            resolved,
            ResolvedReferenceRoot {
                path: PathBuf::from(PLATFORM_REFERENCE_ROOT),
                source: "default",
            }
        );
    }

    #[test]
    fn resolve_compute_reference_root_uses_install_metadata() {
        let install_dir = unique_temp_dir("compute-root-install-metadata");
        let binary_path = install_dir.join("bin/entropyfa");
        let reference_root = install_dir.join("reference");
        write_install_metadata(&binary_path, "full", Some(&reference_root));

        let resolved = resolve_compute_reference_root_with_context(
            Some(binary_path.as_path()),
            Some(install_dir.as_path()),
            None,
            None,
        );

        assert_eq!(
            resolved,
            ResolvedReferenceRoot {
                path: reference_root,
                source: "install-metadata",
            }
        );
    }

    #[test]
    fn install_profile_uses_wire_vocabulary() {
        assert_eq!(InstallProfile::BinaryOnly.as_str(), "binary-only");
        assert_eq!(InstallProfile::Full.as_str(), "full");
        assert_eq!(InstallProfile::Platform.as_str(), "platform");
    }

    #[test]
    fn detect_install_profile_uses_explicit_override() {
        let profile = detect_install_profile(
            Some("platform"),
            Some(Path::new("/usr/local/bin/entropyfa")),
            Some(Path::new("/Users/test")),
        );

        assert_eq!(profile, InstallProfile::Platform);
    }

    #[test]
    fn detect_install_profile_prefers_install_metadata_for_custom_layouts() {
        let home_dir = unique_temp_dir("custom-layout-home");
        let binary_path = home_dir.join("custom/bin/entropyfa");
        let reference_root = home_dir.join("custom/reference");
        write_install_metadata(&binary_path, "full", Some(&reference_root));

        let profile =
            detect_install_profile(None, Some(binary_path.as_path()), Some(home_dir.as_path()));

        assert_eq!(profile, InstallProfile::Full);
    }

    #[test]
    fn detect_install_profile_detects_platform_from_path_when_no_override() {
        let profile = detect_install_profile(
            None,
            Some(Path::new("/opt/entropyfa/bin/entropyfa")),
            Some(Path::new("/Users/test")),
        );

        assert_eq!(profile, InstallProfile::Platform);
    }

    #[test]
    fn detect_install_profile_detects_full_from_user_root() {
        let home_dir = unique_temp_dir("full-user-root");
        let user_reference_root = home_dir.join(USER_REFERENCE_ROOT_SUFFIX);
        touch_managed_reference_marker(&user_reference_root);

        let profile = detect_install_profile_with_context(
            None,
            Some(home_dir.join(".entropyfa/bin/entropyfa").as_path()),
            None,
            Some(&user_reference_root),
            Path::new("/nonexistent/platform/reference"),
        );

        assert_eq!(profile, InstallProfile::Full);
    }

    #[test]
    fn detect_install_profile_falls_back_to_binary_only() {
        let home_dir = unique_temp_dir("binary-only-user-root");
        let user_reference_root = home_dir.join(USER_REFERENCE_ROOT_SUFFIX);

        let profile = detect_install_profile_with_context(
            None,
            Some(home_dir.join(".entropyfa/bin/entropyfa").as_path()),
            None,
            Some(&user_reference_root),
            Path::new("/nonexistent/platform/reference"),
        );

        assert_eq!(profile, InstallProfile::BinaryOnly);
    }

    #[test]
    fn detect_install_profile_detects_platform_from_system_binary_when_managed_root_exists() {
        let home_dir = unique_temp_dir("platform-system-home");
        let platform_reference_root = unique_temp_dir("platform-system-root");
        touch_managed_reference_marker(&platform_reference_root);

        let profile = detect_install_profile_with_context(
            None,
            Some(Path::new("/usr/local/bin/entropyfa")),
            None,
            Some(home_dir.join(USER_REFERENCE_ROOT_SUFFIX).as_path()),
            &platform_reference_root,
        );

        assert_eq!(profile, InstallProfile::Platform);
    }

    #[test]
    fn detect_install_profile_ignores_invalid_override() {
        let home_dir = unique_temp_dir("invalid-override-home");
        let user_reference_root = home_dir.join(USER_REFERENCE_ROOT_SUFFIX);
        touch_managed_reference_marker(&user_reference_root);

        let profile = detect_install_profile_with_context(
            Some("bogus"),
            Some(home_dir.join(".entropyfa/bin/entropyfa").as_path()),
            None,
            Some(&user_reference_root),
            Path::new("/nonexistent/platform/reference"),
        );

        assert_eq!(profile, InstallProfile::Full);
    }
}
