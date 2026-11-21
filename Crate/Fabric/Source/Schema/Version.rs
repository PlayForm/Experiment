//! Immutable schema versions with staged compatibility and retained readers.
//!
//! `docs/05:26-27` requires staged compatibility checks, atomic activation
//! after a signed decision, and retention of old readers until obligations
//! expire. `docs/06:32-37` forbids erasing historical journal entries on
//! rollback. No activation is performed here.

use crate::verdict::Verdict;

/// One immutable schema version record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Version {
    /// Monotonic authored revision.
    pub revision: u64,
    /// Key epoch in which this version is current.
    pub epoch: u64,
    /// Digest of the compiled version.
    pub version_digest: String,
    /// Retention obligation expressed as a canonical expiry instant.
    pub retain_until: String,
}

impl Version {
    /// Record a version from trusted fields.
    #[must_use]
    pub fn of(revision: u64, epoch: u64, digest: &str, retain_until: &str) -> Self {
        Self {
            revision,
            epoch,
            version_digest: digest.to_owned(),
            retain_until: retain_until.to_owned(),
        }
    }

    /// A successor must advance both revision and epoch and carry a digest.
    #[must_use]
    pub fn compatible_after(&self, successor: &Self) -> Verdict {
        Verdict::of(successor.revision > self.revision)
            .and(Verdict::of(successor.epoch > self.epoch))
            .and(Verdict::digest(&successor.version_digest))
    }

    /// Old readers stay available until the retention obligation expires.
    #[must_use]
    pub fn retainable(&self) -> Verdict {
        Verdict::filled(&self.retain_until).and(Verdict::digest(&self.version_digest))
    }
}
