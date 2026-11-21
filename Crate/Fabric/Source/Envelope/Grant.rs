//! A proof-bound capability grant, never the secret it protects.
//!
//! Mirrors all twelve fields of `schemas/proof-bound-grant.schema.json`:
//! grant_id, issuer, subject, resource, action, purpose, proof_key_digest,
//! policy_digest, epoch, not_before, expires and signature. `docs/09:69-73`
//! requires `not_before < expires` and a recognized resource/action/purpose
//! association, which the schema does not prove. Nothing is signed here.

use crate::verdict::Verdict;

/// One signed capability grant bound to an authenticated workload key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grant {
    /// Opaque grant identifier.
    pub grant_id: String,
    /// Trusted issuer identifier.
    pub issuer: String,
    /// Authenticated subject identifier.
    pub subject: String,
    /// Opaque resource handle.
    pub resource: String,
    /// Authorized action.
    pub action: String,
    /// Authorized purpose.
    pub purpose: String,
    /// Digest of the bound proof key.
    pub proof_key_digest: String,
    /// Digest of the authorizing policy.
    pub policy_digest: String,
    /// Epoch in which the grant is current.
    pub epoch: u64,
    /// Canonical not-before instant.
    pub not_before: String,
    /// Canonical expiry instant.
    pub expires: String,
    /// Signature over the canonical grant.
    pub signature: String,
}

impl Grant {
    /// Record a grant from trusted fields.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn of(
        grant_id: &str,
        issuer: &str,
        subject: &str,
        resource: &str,
        action: &str,
        purpose: &str,
        proof: &str,
        policy: &str,
        epoch: u64,
        not_before: &str,
        expires: &str,
        signature: &str,
    ) -> Self {
        Self {
            grant_id: grant_id.to_owned(),
            issuer: issuer.to_owned(),
            subject: subject.to_owned(),
            resource: resource.to_owned(),
            action: action.to_owned(),
            purpose: purpose.to_owned(),
            proof_key_digest: proof.to_owned(),
            policy_digest: policy.to_owned(),
            epoch,
            not_before: not_before.to_owned(),
            expires: expires.to_owned(),
            signature: signature.to_owned(),
        }
    }

    /// Validate grant shape and the cross-field time order the schema cannot
    /// prove. Canonical UTC strings compare lexically.
    #[must_use]
    pub fn cross_field_valid(&self) -> Verdict {
        Verdict::filled(&self.grant_id)
            .and(Verdict::filled(&self.issuer))
            .and(Verdict::filled(&self.subject))
            .and(Verdict::filled(&self.resource))
            .and(Verdict::filled(&self.action))
            .and(Verdict::filled(&self.purpose))
            .and(Verdict::digest(&self.proof_key_digest))
            .and(Verdict::digest(&self.policy_digest))
            .and(Verdict::filled(&self.signature))
            .and(Verdict::of(self.epoch >= 1))
            .and(Verdict::of(self.not_before < self.expires))
    }

    /// A grant is authorization, not secret content (`docs/04:40-41`).
    #[must_use]
    pub const fn is_content() -> bool {
        false
    }

    /// A worker control operation never receives a grant
    /// (`docs/08:30-32`).
    #[must_use]
    pub const fn worker_control_grant() -> Verdict {
        Verdict::Deny
    }
}
