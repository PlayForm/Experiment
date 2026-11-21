//! One frozen five-way generation of candidate workers.
//! Grounded in `ARCHITECTURE.md:64-96` (freeze base commit, baseline binary,
//! toolchain, fixtures, cohort, protocol, oracle and policy; then allocate
//! five disjoint scopes) and `18-emergent-evolution.md:30-33` (an oracle is
//! established before a problem is given to a patcher).

/// A generation and the immutable context it freezes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Generation {
    /// Opaque generation identifier.
    pub id: String,
    /// The frozen parent commit.
    pub base_commit: String,
    /// Number of disjoint workers in the fanout.
    pub fanout: usize,
    /// Digest recorded when the context was frozen; absent before freeze.
    pub frozen_at: Option<String>,
}

impl Generation {
    /// Open a generation at a pinned commit using the configured fanout.
    #[must_use]
    pub fn of(id: &str, base_commit: &str) -> Self {
        Self {
            id: id.to_owned(),
            base_commit: base_commit.to_owned(),
            fanout: crate::FANOUT,
            frozen_at: None,
        }
    }

    /// Freeze the generation against a context digest.
    #[must_use]
    pub fn freeze(self, digest: &str) -> Self {
        Self {
            frozen_at: Some(digest.to_owned()),
            ..self
        }
    }

    /// Whether the generation context is immutable and admissible.
    #[must_use]
    pub const fn is_frozen(&self) -> bool {
        self.frozen_at.is_some()
    }

    /// The disjoint mutation scopes allocated to this fanout.
    #[must_use]
    pub fn scopes(&self) -> Vec<crate::propose::scope::Scope> {
        crate::propose::scope::Scope::all().into_iter().take(self.fanout).collect()
    }
}
