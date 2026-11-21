//! One signed journal commitment record.
//!
//! Mirrors all eight fields of `schemas/audit-commitment.schema.json`:
//! sequence, previous, event_ciphertext_digest, producer, epoch, policy_digest,
//! timestamp and signature. The record points to ciphertext; it never holds
//! plaintext and does not verify the signature itself.

use crate::verdict::Verdict;

/// One append-only journal commitment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    /// Monotonic sequence in the chain.
    pub sequence: u64,
    /// Digest of the previous commitment.
    pub previous: String,
    /// Digest of the encrypted event payload.
    pub ciphertext_digest: String,
    /// Opaque producer identifier.
    pub producer: String,
    /// Key and policy epoch at append time.
    pub epoch: u64,
    /// Digest of the active policy.
    pub policy_digest: String,
    /// Trusted authoritative timestamp.
    pub timestamp: String,
    /// Producer signature over the canonical record.
    pub signature: String,
}

impl Entry {
    /// Record a commitment from trusted fields.
    #[must_use]
    pub fn of(
        sequence: u64,
        previous: &str,
        ciphertext: &str,
        producer: &str,
        epoch: u64,
        policy: &str,
        timestamp: &str,
        signature: &str,
    ) -> Self {
        Self {
            sequence,
            previous: previous.to_owned(),
            ciphertext_digest: ciphertext.to_owned(),
            producer: producer.to_owned(),
            epoch,
            policy_digest: policy.to_owned(),
            timestamp: timestamp.to_owned(),
            signature: signature.to_owned(),
        }
    }

    /// Whether every field has the required shape. No cryptography runs.
    #[must_use]
    pub fn well_formed(&self) -> Verdict {
        Verdict::digest(&self.previous)
            .and(Verdict::digest(&self.ciphertext_digest))
            .and(Verdict::filled(&self.producer))
            .and(Verdict::of(self.epoch >= 1))
            .and(Verdict::digest(&self.policy_digest))
            .and(Verdict::filled(&self.timestamp))
            .and(Verdict::filled(&self.signature))
    }

    /// A reordered or duplicate sequence must be rejected by the trusted
    /// writer (`docs/09:35-36`).
    #[must_use]
    pub fn in_order_after(&self, prior: u64) -> Verdict {
        Verdict::of(self.sequence == prior + 1)
    }
}
