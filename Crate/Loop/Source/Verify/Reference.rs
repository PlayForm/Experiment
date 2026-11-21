//! The frozen reference contract used by the oracle.
//! Grounded in `18-emergent-evolution.md:30-33` (the correct reference
//! contract is established before giving a problem to a patcher) and in
//! `00-system-constitution.md:7-9` (behavioural equivalence with the approved
//! reference contract is the first optimized property).

/// An immutable reference point for behavioural equivalence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reference {
    /// Frozen source commit of the reference.
    pub commit: String,
    /// Digest of the approved fixture set.
    pub fixture_digest: String,
}

impl Reference {
    /// Freeze a reference at a commit and fixture digest.
    #[must_use]
    pub fn freeze(commit: &str, fixture_digest: &str) -> Self {
        Self {
            commit: commit.to_owned(),
            fixture_digest: fixture_digest.to_owned(),
        }
    }

    /// Whether the reference is fully pinned.
    #[must_use]
    pub fn is_frozen(&self) -> bool {
        !self.commit.is_empty() && !self.fixture_digest.is_empty()
    }
}
