//! A trusted request to move one artifact between qualified holders.
//!
//! Mirrors `examples/v61-custody-request.example.json`. A request never
//! carries enforcement reasons, peer identities or a delivery instruction
//! (`docs/19:28-34`, `docs/20:57-70`).

use crate::digest::Digest;
use crate::warning::Warning;

/// The pinned fields of an authorization/reservation request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    /// `request_id` from the custody request example.
    pub request_id: String,
    /// `idempotency_key`; a repeated key is a replay, not a second transfer.
    pub idempotency_key: String,
    /// `artifact_digest`; an identity, never an entitlement.
    pub artifact_digest: String,
    /// `artifact_policy_id` naming the bound rights record.
    pub artifact_policy_id: String,
    /// `source_holder_id` of the attested sender.
    pub source_holder_id: String,
    /// `target_holder_id` of the attested recipient.
    pub target_holder_id: String,
    /// `requested_purpose`, checked against the bound policy.
    pub requested_purpose: String,
    /// `requested_bytes` the recipient must be able to reserve.
    pub requested_bytes: u64,
    /// `lease_expires_at` as a canonical UTC timestamp.
    pub lease_expires_at: String,
    /// `repository_id` scope of the transfer.
    pub repository_id: String,
    /// `policy_revision` digest bound at preflight.
    pub policy_revision: String,
    /// `transfer_nonce`, one-use and bound to the reservation.
    pub transfer_nonce: String,
}

impl Request {
    /// Declare the invariant fields; callers add trusted observations.
    #[must_use]
    pub fn of(
        request_id: &str,
        artifact_digest: &str,
        source: &str,
        target: &str,
        purpose: &str,
        bytes: u64,
    ) -> Self {
        Self {
            request_id: request_id.to_owned(),
            idempotency_key: String::new(),
            artifact_digest: artifact_digest.to_owned(),
            artifact_policy_id: String::new(),
            source_holder_id: source.to_owned(),
            target_holder_id: target.to_owned(),
            requested_purpose: purpose.to_owned(),
            requested_bytes: bytes,
            lease_expires_at: String::new(),
            repository_id: String::new(),
            policy_revision: String::new(),
            transfer_nonce: String::new(),
        }
    }

    /// Attach the one-use nonce and idempotency key from the broker.
    #[must_use]
    pub fn bound(mut self, key: &str, nonce: &str) -> Self {
        self.idempotency_key = key.to_owned();
        self.transfer_nonce = nonce.to_owned();
        self
    }

    /// Attach the policy identity, revision, repository and lease deadline.
    #[must_use]
    pub fn scoped(
        mut self,
        policy: &str,
        revision: &str,
        repository: &str,
        expires: &str,
    ) -> Self {
        self.artifact_policy_id = policy.to_owned();
        self.policy_revision = revision.to_owned();
        self.repository_id = repository.to_owned();
        self.lease_expires_at = expires.to_owned();
        self
    }

    /// Report malformed input as warnings rather than staging any bytes.
    #[must_use]
    pub fn validate(&self) -> Vec<Warning> {
        let empty = [
            ("request id", self.request_id.is_empty()),
            ("idempotency key", self.idempotency_key.is_empty()),
            ("artifact policy", self.artifact_policy_id.is_empty()),
            ("source holder", self.source_holder_id.is_empty()),
            ("target holder", self.target_holder_id.is_empty()),
            ("purpose", self.requested_purpose.is_empty()),
            ("lease deadline", self.lease_expires_at.is_empty()),
            ("repository", self.repository_id.is_empty()),
            ("policy revision", self.policy_revision.is_empty()),
            ("nonce", self.transfer_nonce.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, missing)| {
            missing.then(|| Warning::Malformed {
                domain: field,
                detail: "a custody request must name this field".to_owned(),
            })
        });
        let shape = [
            ("artifact_digest", self.artifact_digest.as_str()),
            ("policy_revision", self.policy_revision.as_str()),
        ]
        .into_iter()
        .filter_map(|(field, value)| {
            (!Digest::of(value).is_valid()).then(|| Warning::DigestShape {
                field,
                detail: Digest::of(value).describe(),
            })
        });
        let zero_bytes = (self.requested_bytes == 0).then(|| Warning::Malformed {
            domain: "requested_bytes",
            detail: "a staged artifact must have positive size".to_owned(),
        });
        empty.chain(shape).chain(zero_bytes).collect()
    }
}
