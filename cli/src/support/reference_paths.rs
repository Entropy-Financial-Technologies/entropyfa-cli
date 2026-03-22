use std::path::{Path, PathBuf};

const PLATFORM_REFERENCE_ROOT: &str = "/opt/entropyfa/reference";
const USER_REFERENCE_ROOT_SUFFIX: &str = ".entropyfa/reference";
const MANAGED_REFERENCE_MARKER: &str = ".entropyfa-managed";

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

#[derive(Debug, Eq, PartialEq)]
pub struct ResolvedReferenceRoot {
    pub path: PathBuf,
    pub source: &'static str,
}

pub fn resolve_reference_root(
    explicit: Option<PathBuf>,
    env_var: Option<String>,
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

    let path = match profile_hint {
        InstallProfile::Platform => PathBuf::from("/opt/entropyfa/reference"),
        InstallProfile::BinaryOnly | InstallProfile::Full | InstallProfile::Unknown => home_dir
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".entropyfa/reference"),
    };

    ResolvedReferenceRoot {
        path,
        source: "default",
    }
}

pub fn detect_install_profile(
    explicit_profile: Option<&str>,
    current_exe: Option<&Path>,
    home_dir: Option<&Path>,
) -> InstallProfile {
    let user_reference_root = home_dir.map(|home| home.join(USER_REFERENCE_ROOT_SUFFIX));
    detect_install_profile_with_roots(
        explicit_profile,
        current_exe,
        user_reference_root.as_deref(),
        Path::new(PLATFORM_REFERENCE_ROOT),
    )
}

fn detect_install_profile_with_roots(
    explicit_profile: Option<&str>,
    current_exe: Option<&Path>,
    user_reference_root: Option<&Path>,
    platform_reference_root: &Path,
) -> InstallProfile {
    if let Some(explicit_profile) = explicit_profile.and_then(InstallProfile::from_env_value) {
        return explicit_profile;
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

fn has_managed_reference_root(path: &Path) -> bool {
    path.join(MANAGED_REFERENCE_MARKER).is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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

    #[test]
    fn resolve_reference_root_prefers_explicit_path() {
        let resolved = resolve_reference_root(
            Some(PathBuf::from("/tmp/explicit")),
            Some("/tmp/env".to_string()),
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
    fn resolve_reference_root_uses_platform_default_for_platform_profile() {
        let resolved = resolve_reference_root(None, None, None, InstallProfile::Platform);

        assert_eq!(
            resolved,
            ResolvedReferenceRoot {
                path: PathBuf::from("/opt/entropyfa/reference"),
                source: "default",
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

        let profile = detect_install_profile_with_roots(
            None,
            Some(home_dir.join(".entropyfa/bin/entropyfa").as_path()),
            Some(&user_reference_root),
            Path::new("/nonexistent/platform/reference"),
        );

        assert_eq!(profile, InstallProfile::Full);
    }

    #[test]
    fn detect_install_profile_falls_back_to_binary_only() {
        let home_dir = unique_temp_dir("binary-only-user-root");
        let user_reference_root = home_dir.join(USER_REFERENCE_ROOT_SUFFIX);

        let profile = detect_install_profile_with_roots(
            None,
            Some(home_dir.join(".entropyfa/bin/entropyfa").as_path()),
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

        let profile = detect_install_profile_with_roots(
            None,
            Some(Path::new("/usr/local/bin/entropyfa")),
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

        let profile = detect_install_profile_with_roots(
            Some("bogus"),
            Some(home_dir.join(".entropyfa/bin/entropyfa").as_path()),
            Some(&user_reference_root),
            Path::new("/nonexistent/platform/reference"),
        );

        assert_eq!(profile, InstallProfile::Full);
    }
}
