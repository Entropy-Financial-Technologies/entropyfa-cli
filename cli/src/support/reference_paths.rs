use std::path::{Path, PathBuf};

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
    if let Some(explicit_profile) = explicit_profile.and_then(InstallProfile::from_env_value) {
        return explicit_profile;
    }

    let Some(current_exe) = current_exe else {
        return InstallProfile::Unknown;
    };

    if current_exe.starts_with("/opt/entropyfa/") {
        return InstallProfile::Platform;
    }

    if let Some(home_dir) = home_dir {
        let user_root = home_dir.join(".entropyfa");
        if current_exe.starts_with(&user_root) {
            return InstallProfile::Full;
        }
    }

    InstallProfile::BinaryOnly
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let profile = detect_install_profile(
            None,
            Some(Path::new("/Users/test/.entropyfa/bin/entropyfa")),
            Some(Path::new("/Users/test")),
        );

        assert_eq!(profile, InstallProfile::Full);
    }

    #[test]
    fn detect_install_profile_falls_back_to_binary_only() {
        let profile = detect_install_profile(
            None,
            Some(Path::new("/usr/local/bin/entropyfa")),
            Some(Path::new("/Users/test")),
        );

        assert_eq!(profile, InstallProfile::BinaryOnly);
    }

    #[test]
    fn detect_install_profile_ignores_invalid_override() {
        let profile = detect_install_profile(
            Some("bogus"),
            Some(Path::new("/Users/test/.entropyfa/bin/entropyfa")),
            Some(Path::new("/Users/test")),
        );

        assert_eq!(profile, InstallProfile::Full);
    }
}
