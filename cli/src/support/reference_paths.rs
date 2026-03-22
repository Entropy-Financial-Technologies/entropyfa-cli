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
            Self::BinaryOnly => "binary_only",
            Self::Full => "full",
            Self::Platform => "platform",
            Self::Unknown => "unknown",
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
    current_exe: Option<&Path>,
    home_dir: Option<&Path>,
) -> InstallProfile {
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
}
