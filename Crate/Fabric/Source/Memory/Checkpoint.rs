//! A sealed snapshot used to shorten journal replay on restart.
//!
//! `docs/06:40-43` restores a sealed snapshot then replays the remaining
//! committed records. `docs/01:45-48` requires encrypted artifacts with keys
//! escrowed separately and rotation history. A checkpoint stores no key and
//! is restored by a trusted controller only.

use crate::verdict::Verdict;

/// One sealed snapshot reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    /// Digest of the sealed snapshot bytes.
    pub snapshot_digest: String,
    /// Epoch the snapshot was sealed at.
    pub epoch: u64,
    /// Journal head included in the snapshot.
    pub head: String,
}

impl Checkpoint {
    /// Record a checkpoint reference from trusted fields.
    #[must_use]
    pub fn of(snapshot_digest: &str, epoch: u64, head: &str) -> Self {
        Self {
            snapshot_digest: snapshot_digest.to_owned(),
            epoch,
            head: head.to_owned(),
        }
    }

    /// Whether the snapshot can be considered for restoration. No bytes are
    /// opened and no key is required here.
    #[must_use]
    pub fn restorable(&self) -> Verdict {
        Verdict::digest(&self.snapshot_digest)
            .and(Verdict::digest(&self.head))
            .and(Verdict::of(self.epoch > 0))
    }

    /// A checkpoint is never the only copy of history; the journal remains
    /// authoritative (`README.md:27-29`).
    #[must_use]
    pub const fn sole_history() -> bool {
        false
    }
}
