//! An invalid result whose raw data is retained.
//! Grounded in `ARCHITECTURE.md:102-104` (BENCH_CONTAMINATED,
//! RESULT_QUARANTINED) and `AGENT_CONTRACT.md:79-83` (quarantine on drift,
//! reschedule, never edit data).

/// An outcome invalidated by integrity or oracle failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Invalid {
    /// Invalidation reason, operator-only.
    pub reason: String,
    /// Digest of the retained raw artifact.
    pub raw_digest: String,
}

impl Invalid {
    /// Record an invalid outcome and keep its raw data.
    #[must_use]
    pub fn of(reason: &str, raw_digest: &str) -> Self {
        Self {
            reason: reason.to_owned(),
            raw_digest: raw_digest.to_owned(),
        }
    }

    /// Raw data is never discarded with the verdict.
    #[must_use]
    pub fn retains_raw(&self) -> bool {
        !self.raw_digest.is_empty()
    }

    /// Whether a fresh baseline is required to retry.
    #[must_use]
    pub const fn requires_rebaseline(&self) -> bool {
        true
    }
}
