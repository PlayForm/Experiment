//! The fixed set of local tools an offline worker may invoke.
//!
//! Mirrors `allowed_tools` in `schemas/v61-offline-task.schema.json:33-47`
//! and `docs/22:13-21`. Local `cargo test` output is a worker claim; only the
//! independent oracle establishes authoritative correctness.

/// One permitted local Rust tool, never an arbitrary command string.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalTool {
    /// `rg` source search.
    Ripgrep,
    /// `rustfmt` formatter.
    Rustfmt,
    /// `cargo check` on local inputs.
    Check,
    /// `cargo test` on local untrusted inputs.
    Test,
    /// Ordinary local file reader.
    Reader,
    /// Bounded `git diff` in a throwaway worktree.
    GitDiff,
}

impl LocalTool {
    /// The exact wire spelling from the offline task schema.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Ripgrep => "rg",
            Self::Rustfmt => "rustfmt",
            Self::Check => "cargo-check",
            Self::Test => "cargo-test-local",
            Self::ReadFile => "read-file",
            Self::GitDiff => "git-diff",
        }
    }

    /// Whether the tool's output can establish trusted correctness.
    #[must_use]
    pub fn authoritative(&self) -> bool {
        match self {
            Self::Ripgrep | Self::Rustfmt | Self::Check | Self::Test | Self::Reader
            | Self::GitDiff => false,
        }
    }

    /// Whether a tool name from a task manifest is recognized.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "rg" => Some(Self::Ripgrep),
            "rustfmt" => Some(Self::Rustfmt),
            "cargo-check" => Some(Self::Check),
            "cargo-test-local" => Some(Self::Test),
            "read-file" => Some(Self::Reader),
            "git-diff" => Some(Self::GitDiff),
            _ => None,
        }
    }
}
