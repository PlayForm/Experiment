//! Invalid result with raw data retained for audit.
//!
//! Grounded in `REQUIREMENTS_V3.md` EXP-007 (valid, contaminated, incomplete,
//! quarantined verdicts with raw data retained even when invalid), INV-009
//! (failures exit without promotion and enter audit history) and
//! `feedback-capsule.v3.schema.json:37-45` (`invalid`, `oracle_failed` statuses).

use crate::gate::reason::Reason;

/// An invalid evaluation, quarantined but never deleted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Invalid {
    /// Digest of the retained raw samples.
    pub raw_samples_sha256: String,
    /// Typed reasons the result was invalidated.
    pub reasons: Vec<Reason>,
    /// Digest of the result manifest that failed integrity checks.
    pub manifest_sha256: String,
    /// Analysis or oracle protocol version in force.
    pub protocol_version: String,
}

impl Invalid {
    /// An invalid result can never be promoted or copied to another run key.
    #[must_use]
    pub const fn may_promote() -> bool {
        false
    }

    /// Raw data remains available for a later audit or reanalysis.
    #[must_use]
    pub fn retains_raw(&self) -> bool {
        !self.raw_samples_sha256.is_empty()
    }

    /// The capsule status token for this result.
    #[must_use]
    pub const fn status() -> &'static str {
        "invalid"
    }
}
