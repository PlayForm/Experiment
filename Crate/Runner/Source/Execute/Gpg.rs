//! Classification of commands that must hold the global GPG mutex.
//!
//! Grounded in `Fn/Binary/Command/Parallel/GPG.rs:17-28` (git + one of five
//! subcommands delegates to `Git::Fn()`) and `Parallel.rs:19,90-95`, where
//! a single `static GPG_MUTEX: Lazy<Mutex<()>>` serializes those commands.
//!
//! The type is written `GPG` (acronym, fully cased); the leaf file keeps the
//! card's required name `Gpg.rs` so the named module exists verbatim.

use crate::execute::git::Git;

/// Whether the parallel executor acquires the process-wide GPG mutex.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GPG {
    /// Acquire `GPG_MUTEX` around process spawn (`Parallel.rs:90-92`).
    Required,
    /// Spawn without the mutex (`Parallel.rs:93-95`).
    NotRequired,
}

impl GPG {
    /// The signing-related subcommands (`GPG.rs:22`).
    pub const SIGNING: [&'static str; 5] = ["commit", "tag", "merge", "rebase", "cherry-pick"];

    /// Compose the subcommand test (`GPG.rs:20-24`) with the cached Git check
    /// (`Git.rs:23-32`). The real SUT calls `Git::Fn()` inside the matched arm;
    /// here the Git state is supplied as a value, so the model never spawns.
    #[must_use]
    pub fn from_command(command: &str, git: Git) -> Self {
        let parts: Vec<&str> = command.split_whitespace().collect();

        match (parts.first().copied(), parts.get(1).copied()) {
            (Some("git"), Some(subcommand)) if Self::SIGNING.contains(&subcommand) => {
                match git {
                    Git::SigningEnabled => Self::Required,
                    Git::SigningDisabled => Self::NotRequired,
                }
            }
            _ => Self::NotRequired,
        }
    }

    /// Whether the mutex is taken.
    #[must_use]
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }
}
