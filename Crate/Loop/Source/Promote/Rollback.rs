//! Rollback as a new authorized transition, never history erasure.
//! Grounded in `06-memory-and-lineage.md:32-44` (rollback requires a new
//! authorized transition and cannot erase historical journal entries) and in
//! `PUBLISHING.md:13-32` (corrected, retracted and superseded are immutable
//! amendment records).

/// A rollback recorded as a fresh transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rollback {
    /// Epoch to restore as the active state.
    pub to_epoch: u32,
    /// Digest of the newly authorized transition.
    pub transition_digest: String,
    /// Digest of the prior head, retained for the audit chain.
    pub prior_head: String,
}

impl Rollback {
    /// Author a rollback transition against a prior head.
    #[must_use]
    pub fn of(to_epoch: u32, transition_digest: &str, prior_head: &str) -> Self {
        Self {
            to_epoch,
            transition_digest: transition_digest.to_owned(),
            prior_head: prior_head.to_owned(),
        }
    }

    /// The prior head is never deleted.
    #[must_use]
    pub fn never_erases_history(&self) -> bool {
        !self.prior_head.is_empty()
    }

    /// Whether the rollback targets an earlier epoch.
    #[must_use]
    pub const fn is_backward(&self, current_epoch: u32) -> bool {
        self.to_epoch < current_epoch
    }
}
