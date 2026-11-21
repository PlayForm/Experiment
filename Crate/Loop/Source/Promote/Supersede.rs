//! Immutable artifacts are superseded by corrections, never mutated.
//! Grounded in `00-system-constitution.md:57-61` (canonical artifacts are
//! immutable; corrections supersede rather than mutate them) and
//! `06-memory-and-lineage.md:32-37` (migration records previous and new state).

/// A correction record that points at an immutable original.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Supersede {
    /// Digest of the original, which is never edited.
    pub original_digest: String,
    /// Digest of the correcting artifact.
    pub correction_digest: String,
    /// Epoch at which the correction became authoritative.
    pub epoch: u32,
}

impl Supersede {
    /// Record a correction of an immutable artifact.
    #[must_use]
    pub fn of(original_digest: &str, correction_digest: &str, epoch: u32) -> Self {
        Self {
            original_digest: original_digest.to_owned(),
            correction_digest: correction_digest.to_owned(),
            epoch,
        }
    }

    /// Whether the original is preserved alongside the correction.
    #[must_use]
    pub fn original_is_immutable(&self) -> bool {
        !self.original_digest.is_empty() && self.original_digest != self.correction_digest
    }

    /// Whether the correction supersedes a genuine earlier artifact.
    #[must_use]
    pub fn supersedes(&self) -> bool {
        self.epoch > 0
    }
}
