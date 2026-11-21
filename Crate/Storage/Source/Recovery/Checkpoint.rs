//! An integrity-verified cut of the immutable journal.
//!
//! The journal is verified on startup and at a checkpoint
//! (`docs/08-data-and-storage.md:5-8`); a checkpoint is a typed pair of
//! digests and a sequence, not a replacement for the underlying events.

use super::epoch::Epoch;
use crate::cas::digest::Digest;

/// A pinned journal cut used as the start of a replay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    /// Sequence through which the journal was verified.
    pub sequence: u64,
    /// Epoch in which the checkpoint was sealed.
    pub epoch: Epoch,
    /// Digest of the projection state at this cut.
    pub state_digest: Digest,
    /// Digest of the journal prefix at this cut.
    pub journal_digest: Digest,
}

impl Checkpoint {
    /// Declare a checkpoint.
    #[must_use]
    pub fn of(sequence: u64, epoch: Epoch, state_digest: Digest, journal_digest: Digest) -> Self {
        Self {
            sequence,
            epoch,
            state_digest,
            journal_digest,
        }
    }

    /// The origin checkpoint before any event is verified.
    #[must_use]
    pub fn origin() -> Self {
        Self::of(
            0,
            Epoch::origin(),
            Digest::zero(),
            Digest::zero(),
        )
    }

    /// Both declared digests must be well-formed for the cut to be trusted.
    #[must_use]
    pub fn verify(&self) -> crate::permission::Permission {
        let states = match self.state_digest.is_valid() {
            true => crate::permission::Permission::Allow,
            false => crate::permission::Permission::Deny,
        };
        let journal = match self.journal_digest.is_valid() {
            true => crate::permission::Permission::Allow,
            false => crate::permission::Permission::Deny,
        };
        states.and(journal)
    }

    /// Report an unverifiable checkpoint without panicking.
    #[must_use]
    pub fn finding(&self) -> Option<crate::warning::Warning> {
        match self.verify() {
            crate::permission::Permission::Allow => None,
            crate::permission::Permission::Deny => Some(crate::warning::Warning::Unverified {
                subject: format!("checkpoint at sequence {}", self.sequence),
            }),
        }
    }
}
