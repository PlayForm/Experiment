//! A frozen, read-only view of the system under test.
//! Grounded in `22-hidden-worker-and-correspondence.md:3-11` (the worker sees
//! a pinned repository snapshot at a virtual fixed path) and in
//! `ARCHITECTURE.md:127-129` (build and benchmark source are immutable and
//! CAS-addressed). This value never writes into `Module/Run`.

use crate::warning::Warning;

/// An immutable snapshot a worker is allowed to read.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    /// Pinned commit of the source tree.
    pub commit: String,
    /// Keyed digest of the full tree at that commit.
    pub tree_digest: String,
    /// Virtual path presented to the worker, never the real host path.
    pub virtual_root: String,
    /// Degradations collected while preparing the view.
    pub warnings: Vec<Warning>,
}

impl Snapshot {
    /// The canonical virtual root shared by every offline worker
    /// (`22-hidden-worker-and-correspondence.md:5`).
    pub const VIRTUAL_ROOT: &'static str = "/workspace/repo";

    /// Prepare a snapshot from a pinned commit and tree digest.
    #[must_use]
    pub fn of(commit: &str, tree_digest: &str) -> Self {
        Self {
            commit: commit.to_owned(),
            tree_digest: tree_digest.to_owned(),
            virtual_root: Self::VIRTUAL_ROOT.to_owned(),
            warnings: Vec::new(),
        }
    }

    /// Record a degradation instead of failing the observation.
    #[must_use]
    pub fn degraded(mut self, message: &str) -> Self {
        self.warnings.push(Warning::of(message));
        self
    }

    /// Whether the snapshot is complete enough to hand to a worker.
    #[must_use]
    pub fn is_readable(&self) -> bool {
        !self.commit.is_empty() && !self.tree_digest.is_empty()
    }

    /// Whether the view leaked a real host path.
    #[must_use]
    pub fn is_virtual(&self) -> bool {
        self.virtual_root == Self::VIRTUAL_ROOT
    }
}
