//! Classification of commands that write to the git index.
//!
//! Grounded in `Fn/Binary/Command/Index.rs:17-36` (`Fn(&str) -> bool`): the
//! string is split on whitespace *for classification only*, the first token
//! must be `git`, and the second token must be one of twelve subcommands.
//! Execution still passes the original string to `sh -c` (`Process.rs:27-32`).

/// Whether a command requires the per-directory git index lock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Index {
    /// A read-only or non-git command (`Index.rs:20-22`).
    ReadOnly,
    /// A git command that writes to the index (`Index.rs:24-35`).
    Modifying,
}

impl Index {
    /// The exact subcommand set from `Index.rs:26-34`.
    pub const MODIFYING: [&'static str; 12] = [
        "add",
        "apply",
        "checkout",
        "cherry-pick",
        "commit",
        "merge",
        "mv",
        "rebase",
        "reset",
        "restore",
        "rm",
        "stash",
    ];

    /// Classify a command string using the SUT's token rule.
    #[must_use]
    pub fn from_command(command: &str) -> Self {
        let parts: Vec<&str> = command.split_whitespace().collect();

        match (parts.first().copied(), parts.get(1).copied()) {
            (Some("git"), Some(subcommand)) if Self::MODIFYING.contains(&subcommand) => {
                Self::Modifying
            }
            _ => Self::ReadOnly,
        }
    }

    /// Whether the caller must wait on `.git/index.lock`.
    #[must_use]
    pub const fn modifies(self) -> bool {
        matches!(self, Self::Modifying)
    }
}
