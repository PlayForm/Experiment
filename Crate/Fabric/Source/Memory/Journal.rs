//! Append-only, signed, encrypted journal with a compare-and-swap head.
//!
//! `docs/06:17` records encrypted event records, previous-hash commitments,
//! signatures, nonces, authoritative timestamps and replay fencing.
//! `docs/08:43-44` adds compare-and-swap on the expected previous head.
//! `docs/06:39-44` requires signature and chain verification on restart.
//! Nothing is appended, encrypted or verified here.

use crate::verdict::Verdict;

/// One journal head observed by a trusted writer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Journal {
    /// Digest of the current chain head.
    pub head: String,
    /// Epoch of the head record.
    pub epoch: u64,
    /// Digest of the policy active at the head.
    pub policy_digest: String,
}

impl Journal {
    /// Record a chain head from trusted replay fields.
    #[must_use]
    pub fn of(head: &str, epoch: u64, policy: &str) -> Self {
        Self {
            head: head.to_owned(),
            epoch,
            policy_digest: policy.to_owned(),
        }
    }

    /// Whether a supplied expected head matches before an append is accepted.
    #[must_use]
    pub fn compare_and_swap(&self, expected_prev: &str) -> Verdict {
        Verdict::digest(&self.head)
            .and(Verdict::of(expected_prev == self.head))
            .and(Verdict::digest(&self.policy_digest))
    }

    /// Whether the previous-hash chain is structurally intact for replay.
    /// No hash is recomputed; only supplied digests are compared.
    #[must_use]
    pub fn chain_intact(&self, recorded_previous: &str) -> Verdict {
        Verdict::digest(recorded_previous)
            .and(Verdict::of(recorded_previous == self.head))
    }

    /// A transaction appends through a stub-facing boundary, never from a
    /// generated artifact (`docs/10:33`, `docs/09:46-48`).
    #[must_use]
    pub const fn worker_append() -> Verdict {
        Verdict::Deny
    }
}
