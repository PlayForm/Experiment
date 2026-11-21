//! A staged patch with a canonical digest, never executed here.
//! Grounded in `AGENT_CONTRACT.md:40-45` (structured final artifact with
//! mutation id, commit, patch digest, modified paths and risks) and
//! `ARCHITECTURE.md:75-80` (exactly one conceptual patch plus content hashes).

/// An untrusted patch proposal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Patch {
    /// Digest of the normalized diff.
    pub digest: String,
    /// Scope the patch claims to mutate.
    pub scope: super::scope::Scope,
    /// Paths the patch claims to change.
    pub changed_paths: Vec<String>,
    /// Number of changed lines, for scope accounting only.
    pub lines: u64,
}

impl Patch {
    /// Assemble a staged patch from its declared parts.
    #[must_use]
    pub fn of(digest: &str, scope: super::scope::Scope, changed_paths: Vec<String>, lines: u64) -> Self {
        Self {
            digest: digest.to_owned(),
            scope,
            changed_paths,
            lines,
        }
    }

    /// Whether the patch stays inside its declared scope root.
    #[must_use]
    pub fn within_scope(&self) -> bool {
        self.changed_paths
            .iter()
            .all(|path| path.starts_with(self.scope.key()))
    }

    /// Whether the patch is a single conceptual change by the contract.
    #[must_use]
    pub const fn is_atomic(&self) -> bool {
        self.lines > 0 && self.lines < 4096
    }
}
