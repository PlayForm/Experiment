//! The cached Git commit-signing check.
//!
//! Grounded in `Fn/Binary/Command/Parallel/GPG/Git.rs:10-32`: a global
//! `OnceCell<bool>` holds whether `git config --get commit.gpgsign` trims to
//! "true"; a failed or missing git command degrades to `false` (`:25-31`).

/// The result of the once-cached `commit.gpgsign` lookup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Git {
    /// `commit.gpgsign` read as "true".
    SigningEnabled,
    /// "false", absent, or git could not be executed.
    SigningDisabled,
}

impl Git {
    /// The single config key queried (`Git.rs:25-26`).
    pub const KEY: &'static str = "commit.gpgsign";
    /// argv after the `git` program (`Git.rs:25`).
    pub const ARGS: [&'static str; 3] = ["config", "--get", Self::KEY];

    /// Resolve the cached boolean from command stdout. Any other value,
    /// including no output at all, means signing is disabled.
    #[must_use]
    pub fn from_output(stdout: &str) -> Self {
        match stdout.trim() == "true" {
            true => Self::SigningEnabled,
            false => Self::SigningDisabled,
        }
    }

    /// Whether commands may need the GPG mutex.
    #[must_use]
    pub const fn is_signing(self) -> bool {
        matches!(self, Self::SigningEnabled)
    }
}
