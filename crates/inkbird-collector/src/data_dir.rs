//! Resolving the *default* data directory for a standalone run of the binary.
//!
//! When neither `--data-dir` nor `INKBIRD_DATA_DIR` is set (the usual case when
//! someone runs a downloaded binary directly), the collector has to decide where
//! to write. It tries, in order:
//!
//! 1. an XDG-style per-user data directory (honouring `XDG_DATA_HOME`, and using
//!    the platform default otherwise);
//! 2. the directory that contains the running executable;
//! 3. nothing writable was found, so the caller falls back to printing readings
//!    to standard output.
//!
//! Only path *construction* lives here, and it is pure: the environment lookup
//! and the executable's directory are passed in, so every branch is unit-tested
//! without touching the real environment or filesystem. Deciding which candidate
//! is actually writable is the caller's job (it probes each in order).

use std::path::{Path, PathBuf};

/// Directory name used under the per-user data location (for example
/// `~/.local/share/myinkbird`).
pub const APP_DIR: &str = "myinkbird";

/// How a data-directory candidate was derived (used only for logging).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataDirSource {
    /// A per-user data directory: `XDG_DATA_HOME` or the platform default.
    UserData,
    /// The directory that contains the running executable.
    ExecutableDir,
}

impl DataDirSource {
    /// A short human-readable label for log messages.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::UserData => "per-user data directory",
            Self::ExecutableDir => "executable directory",
        }
    }
}

/// A candidate default data directory and how it was derived.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataDirCandidate {
    /// Directory readings would be written under (`<path>/readings/...`).
    pub path: PathBuf,
    /// How this candidate was derived.
    pub source: DataDirSource,
}

/// The platforms whose per-user data-directory conventions differ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    /// Linux and other XDG-style Unixes.
    Linux,
    /// Apple macOS.
    MacOs,
    /// Microsoft Windows.
    Windows,
}

impl Platform {
    /// The platform this binary was compiled for.
    ///
    /// All three arms are referenced here, so none is ever reported as unused on
    /// a given build; `cfg!` selects the value without removing the other arms.
    pub const CURRENT: Self = if cfg!(target_os = "windows") {
        Self::Windows
    } else if cfg!(target_os = "macos") {
        Self::MacOs
    } else {
        Self::Linux
    };
}

/// Build the ordered list of candidate default data directories, most-preferred
/// first, from an environment lookup and the executable's directory.
///
/// This does not touch the filesystem: the caller probes each candidate for
/// writability in order and uses the first that works.
#[must_use]
pub fn candidate_data_dirs(
    env: impl Fn(&str) -> Option<String>,
    exe_dir: Option<&Path>,
    platform: Platform,
) -> Vec<DataDirCandidate> {
    let mut candidates = Vec::new();

    if let Some(base) = user_data_base(&env, platform) {
        candidates.push(DataDirCandidate {
            path: base.join(APP_DIR),
            source: DataDirSource::UserData,
        });
    }

    if let Some(dir) = exe_dir {
        candidates.push(DataDirCandidate {
            path: dir.to_path_buf(),
            source: DataDirSource::ExecutableDir,
        });
    }

    candidates
}

/// Candidates for the real process environment: uses the process's environment
/// variables and the directory of the current executable.
#[must_use]
pub fn default_data_dir_candidates() -> Vec<DataDirCandidate> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf));
    candidate_data_dirs(
        |key| std::env::var(key).ok(),
        exe_dir.as_deref(),
        Platform::CURRENT,
    )
}

/// The per-user data base directory, before the `myinkbird` subdirectory is
/// appended. `None` when the environment variables it needs are absent.
fn user_data_base(env: &impl Fn(&str) -> Option<String>, platform: Platform) -> Option<PathBuf> {
    // An explicit, absolute `XDG_DATA_HOME` wins on every platform (some people
    // set it on macOS and Windows too). The XDG spec says a relative value is
    // invalid and must be ignored.
    if let Some(dir) = non_empty(env("XDG_DATA_HOME")) {
        let path = PathBuf::from(dir);
        if path.is_absolute() {
            return Some(path);
        }
    }

    match platform {
        Platform::Linux => {
            non_empty(env("HOME")).map(|home| PathBuf::from(home).join(".local/share"))
        }
        Platform::MacOs => non_empty(env("HOME"))
            .map(|home| PathBuf::from(home).join("Library/Application Support")),
        Platform::Windows => non_empty(env("APPDATA"))
            .or_else(|| non_empty(env("LOCALAPPDATA")))
            .map(PathBuf::from),
    }
}

/// `Some(value)` only when `value` is present and not just whitespace.
fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|s| !s.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build an environment lookup from a list of key/value pairs.
    fn env_of(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let owned: Vec<(String, String)> = pairs
            .iter()
            .map(|&(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |key: &str| {
            owned
                .iter()
                .find(|(k, _)| k.as_str() == key)
                .map(|(_, v)| v.clone())
        }
    }

    #[test]
    fn linux_uses_xdg_data_home_when_absolute() {
        let env = env_of(&[("XDG_DATA_HOME", "/xdg/data"), ("HOME", "/home/me")]);
        let candidates = candidate_data_dirs(env, None, Platform::Linux);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].path, PathBuf::from("/xdg/data").join(APP_DIR));
        assert_eq!(candidates[0].source, DataDirSource::UserData);
    }

    #[test]
    fn linux_ignores_relative_xdg_data_home_and_uses_home() {
        let env = env_of(&[("XDG_DATA_HOME", "relative/data"), ("HOME", "/home/me")]);
        let candidates = candidate_data_dirs(env, None, Platform::Linux);
        assert_eq!(
            candidates[0].path,
            PathBuf::from("/home/me").join(".local/share").join(APP_DIR)
        );
    }

    #[test]
    fn linux_falls_back_to_home_local_share() {
        let env = env_of(&[("HOME", "/home/me")]);
        let candidates = candidate_data_dirs(env, None, Platform::Linux);
        assert_eq!(
            candidates[0].path,
            PathBuf::from("/home/me").join(".local/share").join(APP_DIR)
        );
        assert_eq!(candidates[0].source, DataDirSource::UserData);
    }

    #[test]
    fn macos_uses_application_support() {
        let env = env_of(&[("HOME", "/Users/me")]);
        let candidates = candidate_data_dirs(env, None, Platform::MacOs);
        assert_eq!(
            candidates[0].path,
            PathBuf::from("/Users/me")
                .join("Library/Application Support")
                .join(APP_DIR)
        );
    }

    #[test]
    fn windows_prefers_appdata_then_localappdata() {
        let env = env_of(&[
            ("APPDATA", "C:\\Users\\me\\AppData\\Roaming"),
            ("LOCALAPPDATA", "C:\\Users\\me\\AppData\\Local"),
        ]);
        let candidates = candidate_data_dirs(env, None, Platform::Windows);
        assert_eq!(
            candidates[0].path,
            PathBuf::from("C:\\Users\\me\\AppData\\Roaming").join(APP_DIR)
        );

        let env = env_of(&[("LOCALAPPDATA", "C:\\Users\\me\\AppData\\Local")]);
        let candidates = candidate_data_dirs(env, None, Platform::Windows);
        assert_eq!(
            candidates[0].path,
            PathBuf::from("C:\\Users\\me\\AppData\\Local").join(APP_DIR)
        );
    }

    #[test]
    fn executable_dir_is_second_choice() {
        let env = env_of(&[("HOME", "/home/me")]);
        let exe = PathBuf::from("/opt/inkbird");
        let candidates = candidate_data_dirs(env, Some(exe.as_path()), Platform::Linux);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].source, DataDirSource::UserData);
        assert_eq!(candidates[1].source, DataDirSource::ExecutableDir);
        assert_eq!(candidates[1].path, exe);
    }

    #[test]
    fn executable_dir_is_only_choice_without_home() {
        let exe = PathBuf::from("/opt/inkbird");
        let candidates = candidate_data_dirs(env_of(&[]), Some(exe.as_path()), Platform::Linux);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].source, DataDirSource::ExecutableDir);
        assert_eq!(candidates[0].path, exe);
    }

    #[test]
    fn empty_environment_yields_no_candidates() {
        let candidates = candidate_data_dirs(env_of(&[]), None, Platform::Linux);
        assert!(candidates.is_empty());
    }

    #[test]
    fn whitespace_home_is_ignored() {
        let env = env_of(&[("HOME", "   ")]);
        let candidates = candidate_data_dirs(env, None, Platform::Linux);
        assert!(candidates.is_empty());
    }

    #[test]
    fn label_is_stable() {
        assert_eq!(DataDirSource::UserData.label(), "per-user data directory");
        assert_eq!(DataDirSource::ExecutableDir.label(), "executable directory");
    }
}
