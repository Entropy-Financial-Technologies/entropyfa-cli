use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const REPO: &str = "Entropy-Financial-Technologies/entropyfa-cli";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CACHE_MAX_AGE_SECS: u64 = 24 * 60 * 60;

fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

fn cache_dir() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".entropyfa")
}

fn cache_path() -> PathBuf {
    cache_dir().join("version-cache.json")
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn parse_semver(s: &str) -> Option<(u32, u32, u32)> {
    let s = s.strip_prefix('v').unwrap_or(s);
    let parts: Vec<&str> = s.trim().split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    Some((
        parts[0].parse().ok()?,
        parts[1].parse().ok()?,
        parts[2].parse().ok()?,
    ))
}

fn is_newer(latest: &str, current: &str) -> bool {
    match (parse_semver(latest), parse_semver(current)) {
        (Some(l), Some(c)) => l > c,
        _ => false,
    }
}

/// Read cached latest version and checked_at timestamp.
fn read_cache() -> Option<(String, u64)> {
    let data = fs::read_to_string(cache_path()).ok()?;
    let version = data
        .split("\"latest_version\"")
        .nth(1)?
        .split('"')
        .nth(1)?
        .to_string();
    let checked_at: u64 = data
        .split("\"checked_at\"")
        .nth(1)
        .and_then(|s| {
            let start = s.find(|c: char| c.is_ascii_digit())?;
            let end = s[start..]
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(s[start..].len());
            s[start..start + end].parse().ok()
        })
        .unwrap_or(0);
    Some((version, checked_at))
}

fn write_cache(version: &str) {
    let dir = cache_dir();
    let _ = fs::create_dir_all(&dir);
    let json = format!(
        "{{\"latest_version\":\"{}\",\"checked_at\":{}}}",
        version,
        now_unix()
    );
    let _ = fs::write(cache_path(), json);
}

/// Spawn a background curl to refresh the version cache (fire-and-forget).
fn spawn_background_refresh() {
    let cache = cache_path();
    let dir = cache_dir();
    let _ = fs::create_dir_all(&dir);

    let script = format!(
        r#"TAG=$(curl -fsSL --max-time 5 "https://api.github.com/repos/{REPO}/releases/latest" 2>/dev/null | grep '"tag_name"' | sed -E 's/.*"v?([^"]+)".*/\1/'); if [ -n "$TAG" ]; then printf '{{"latest_version":"%s","checked_at":%s}}' "$TAG" "$(date +%s)" > "{}"; fi"#,
        cache.display()
    );

    let _ = Command::new("sh")
        .args(["-c", &script])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

/// Called on every CLI invocation. Prints an update warning to stderr if a newer
/// version is cached. Refreshes the cache in the background if stale/missing.
pub fn check_and_warn() {
    match read_cache() {
        Some((latest, checked_at)) => {
            if is_newer(&latest, CURRENT_VERSION) {
                eprintln!(
                    "[entropyfa] Update available: {} -> {}. Run 'entropyfa upgrade' or 'entropyfa update' to update.",
                    CURRENT_VERSION, latest
                );
            }
            if now_unix().saturating_sub(checked_at) > CACHE_MAX_AGE_SECS {
                spawn_background_refresh();
            }
        }
        None => {
            spawn_background_refresh();
        }
    }
}

/// Fetch the latest version tag from GitHub releases API (blocking).
fn fetch_latest_version() -> Result<String, String> {
    let output = Command::new("curl")
        .args([
            "-fsSL",
            "--max-time",
            "10",
            &format!("https://api.github.com/repos/{REPO}/releases/latest"),
        ])
        .output()
        .map_err(|e| format!("Failed to run curl: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "GitHub API request failed (status {})",
            output.status
        ));
    }

    let body = String::from_utf8_lossy(&output.stdout);
    let tag = body
        .split("\"tag_name\"")
        .nth(1)
        .and_then(|s| s.split('"').nth(1))
        .ok_or("Could not parse tag_name from GitHub response")?;

    let version = tag.strip_prefix('v').unwrap_or(tag);
    Ok(version.to_string())
}

fn detect_target() -> Result<String, String> {
    let os = match std::env::consts::OS {
        "macos" => "apple-darwin",
        "linux" => "unknown-linux-gnu",
        other => return Err(format!("Unsupported OS: {other}")),
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        other => return Err(format!("Unsupported architecture: {other}")),
    };
    Ok(format!("{arch}-{os}"))
}

fn remove_stale_cargo_binary() {
    if let Some(home) = home_dir() {
        let cargo_bin = home.join(".cargo/bin/entropyfa");
        if cargo_bin.exists() {
            eprintln!(
                "[entropyfa] Removing stale {} (shadows /usr/local/bin/entropyfa)",
                cargo_bin.display()
            );
            let _ = fs::remove_file(&cargo_bin);
        }
    }
}

/// Full self-update flow.
pub fn run_upgrade() {
    eprintln!("[entropyfa] Checking for updates...");

    let latest = match fetch_latest_version() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[entropyfa] Error: {e}");
            std::process::exit(1);
        }
    };

    eprintln!("[entropyfa] Current version: {CURRENT_VERSION}");
    eprintln!("[entropyfa] Latest version:  {latest}");

    if !is_newer(&latest, CURRENT_VERSION) {
        eprintln!("[entropyfa] Already up to date.");
        write_cache(&latest);
        return;
    }

    let target = match detect_target() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[entropyfa] Error: {e}");
            std::process::exit(1);
        }
    };

    let url =
        format!("https://github.com/{REPO}/releases/download/v{latest}/entropyfa-{target}.tar.gz");
    eprintln!("[entropyfa] Downloading {url}");

    let tmp_binary = format!("/tmp/entropyfa-{target}");
    let status = Command::new("sh")
        .args(["-c", &format!("curl -fsSL '{url}' | tar xz -C /tmp")])
        .status();

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            eprintln!("[entropyfa] Download failed (exit {})", s);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("[entropyfa] Download failed: {e}");
            std::process::exit(1);
        }
    }

    let current_exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[entropyfa] Cannot determine current executable path: {e}");
            std::process::exit(1);
        }
    };

    let dest = current_exe
        .to_str()
        .unwrap_or("/usr/local/bin/entropyfa")
        .to_string();

    eprintln!("[entropyfa] Replacing {dest}");

    // Try direct copy first, fall back to sudo mv
    let ok = fs::copy(&tmp_binary, &dest).is_ok() || {
        Command::new("sudo")
            .args(["mv", &tmp_binary, &dest])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    };

    if !ok {
        eprintln!("[entropyfa] Failed to replace binary. You can manually run:");
        eprintln!("  sudo mv {tmp_binary} {dest}");
        std::process::exit(1);
    }

    let _ = Command::new("chmod").args(["+x", &dest]).status();

    write_cache(&latest);
    remove_stale_cargo_binary();

    eprintln!("[entropyfa] Upgraded successfully: {CURRENT_VERSION} -> {latest}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_semver_basic() {
        assert_eq!(parse_semver("1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_semver("v1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_semver("0.1.0"), Some((0, 1, 0)));
        assert_eq!(parse_semver("10.20.30"), Some((10, 20, 30)));
    }

    #[test]
    fn test_parse_semver_invalid() {
        assert_eq!(parse_semver("1.2"), None);
        assert_eq!(parse_semver("abc"), None);
        assert_eq!(parse_semver("1.2.x"), None);
        assert_eq!(parse_semver(""), None);
    }

    #[test]
    fn test_is_newer() {
        assert!(is_newer("0.2.0", "0.1.0"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(is_newer("0.1.1", "0.1.0"));
        assert!(!is_newer("0.1.0", "0.1.0"));
        assert!(!is_newer("0.0.9", "0.1.0"));
        assert!(is_newer("v0.2.0", "0.1.0"));
    }

    #[test]
    fn test_detect_target() {
        let target = detect_target();
        assert!(target.is_ok());
        let t = target.unwrap();
        assert!(t.contains("apple-darwin") || t.contains("unknown-linux-gnu"));
    }
}
