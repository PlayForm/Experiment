//! The genesis commitment, explicitly not an encryption master key.
//!
//! Mirrors `schemas/genesis-commitment.schema.json`: protocol revision,
//! schema digest, policy digest, first event commitment, issuer public-key
//! digest, creation time and governance signature. `docs/06:27-30` states it
//! is a safe audit anchor, not the encryption seed; `docs/01:41-48` rejects a
//! single reconstructing seed.

use crate::verdict::Verdict;

/// An offline, signed statement of the starting state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Root {
    /// Declared protocol revision, fixed to `6.2`.
    pub revision: String,
    /// Digest of the initial schema manifest.
    pub schema_digest: String,
    /// Digest of the initial policy manifest.
    pub policy_digest: String,
    /// Commitment to the first event-log head.
    pub first_event_commitment: String,
    /// Digest of the issuer public key, not the key itself.
    pub issuer_digest: String,
    /// Canonical creation instant supplied by a trusted signer.
    pub created_at: String,
    /// Governance signature over the canonical commitment.
    pub signature: String,
}

impl Root {
    /// Record a signed commitment from trusted fields.
    #[must_use]
    pub fn of(
        schema: &str,
        policy: &str,
        first_event: &str,
        issuer: &str,
        created: &str,
        signature: &str,
    ) -> Self {
        Self {
            revision: crate::PROTOCOL_REVISION.to_owned(),
            schema_digest: schema.to_owned(),
            policy_digest: policy.to_owned(),
            first_event_commitment: first_event.to_owned(),
            issuer_digest: issuer.to_owned(),
            created_at: created.to_owned(),
            signature: signature.to_owned(),
        }
    }

    /// Whether every committed digest has the required shape.
    #[must_use]
    pub fn well_formed(&self) -> Verdict {
        Verdict::of(self.revision == crate::PROTOCOL_REVISION)
            .and(Verdict::digest(&self.schema_digest))
            .and(Verdict::digest(&self.policy_digest))
            .and(Verdict::digest(&self.first_event_commitment))
            .and(Verdict::digest(&self.issuer_digest))
            .and(Verdict::filled(&self.created_at))
            .and(Verdict::filled(&self.signature))
    }

    /// A commitment attests state; it never encrypts or derives a data key.
    #[must_use]
    pub const fn is_master_key() -> bool {
        false
    }

    /// Attempting to use the public genesis metadata as a seed is refused.
    #[must_use]
    pub const fn as_seed() -> Verdict {
        Verdict::Deny
    }
}
