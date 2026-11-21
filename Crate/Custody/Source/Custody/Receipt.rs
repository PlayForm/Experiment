//! The immutable custody receipt and its deliberately empty worker view.
//!
//! Mirrors `examples/v61-custody-receipt.example.json` and the neutral receipt
//! of `examples/v61-worker-neutral-receipt.example.json`. A worker projection
//! carries exactly one status, `submission_received`; no decision rationale,
//! embargo cause, identity or success claim can be recovered from it
//! (`docs/19:43-49`, `docs/22:56-59`).

use super::phase::Phase;
use crate::digest::Digest;
use crate::warning::Warning;

/// One trusted commit record, or one opaque neutral acknowledgement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Receipt {
    /// `receipt_id`, unique and immutable.
    pub receipt_id: String,
    /// `request_id` that authorized the transfer.
    pub request_id: String,
    /// `decision_id` retained only in the trusted audit ledger.
    pub decision_id: String,
    /// `artifact_digest` identity of the transferred bytes.
    pub artifact_digest: String,
    /// `source_holder_id` of the attested sender.
    pub source_holder_id: String,
    /// `target_holder_id` of the attested recipient.
    pub target_holder_id: String,
    /// `transfer_nonce` bound at reservation.
    pub transfer_nonce: String,
    /// `encrypted_payload_digest` written to quarantine.
    pub encrypted_payload_digest: String,
    /// `verified_plaintext_digest` checked in the receiving trust domain.
    pub verified_plaintext_digest: String,
    /// `holder_attestation_digest` checked before staging.
    pub holder_attestation_digest: String,
    /// `storage_lease_id` that fenced the reservation.
    pub storage_lease_id: String,
    /// `committed_event_id` in the canonical journal.
    pub committed_event_id: String,
    /// `committed_at` canonical UTC time.
    pub committed_at: String,
    /// The custody phase this record describes.
    pub phase: Phase,
    /// The single worker-visible status string.
    pub worker_status: &'static str,
    /// The single worker-visible human sentence.
    pub worker_message: &'static str,
}

impl Receipt {
    /// Construct the only acknowledgement an offline worker may receive.
    #[must_use]
    pub fn neutral(receipt_id: &str, nonce: &str) -> Self {
        Self {
            receipt_id: receipt_id.to_owned(),
            request_id: String::new(),
            decision_id: String::new(),
            artifact_digest: String::new(),
            source_holder_id: String::new(),
            target_holder_id: String::new(),
            transfer_nonce: nonce.to_owned(),
            encrypted_payload_digest: String::new(),
            verified_plaintext_digest: String::new(),
            holder_attestation_digest: String::new(),
            storage_lease_id: String::new(),
            committed_event_id: String::new(),
            committed_at: String::new(),
            phase: Phase::Requested,
            worker_status: crate::NEUTRAL_STATUS,
            worker_message: crate::NEUTRAL_MESSAGE,
        }
    }

    /// Attach the trusted evidence that produced a committed receipt.
    #[must_use]
    pub fn committed(
        mut self,
        request_id: &str,
        decision: &str,
        artifact: &str,
        encrypted: &str,
        plaintext: &str,
        attestation: &str,
        lease: &str,
        event: &str,
        at: &str,
    ) -> Self {
        self.request_id = request_id.to_owned();
        self.decision_id = decision.to_owned();
        self.artifact_digest = artifact.to_owned();
        self.encrypted_payload_digest = encrypted.to_owned();
        self.verified_plaintext_digest = plaintext.to_owned();
        self.holder_attestation_digest = attestation.to_owned();
        self.storage_lease_id = lease.to_owned();
        self.committed_event_id = event.to_owned();
        self.committed_at = at.to_owned();
        self.phase = Phase::Committed;
        self
    }

    /// The entire worker projection: two fixed strings, nothing else.
    #[must_use]
    pub fn worker_view(&self) -> (&'static str, &'static str) {
        (self.worker_status, self.worker_message)
    }

    /// Report an uncommitted or malformed receipt without asserting success.
    #[must_use]
    pub fn validate(&self) -> Vec<Warning> {
        let identity = [("receipt id", self.receipt_id.is_empty())]
            .into_iter()
            .filter_map(|(field, missing)| {
                missing.then(|| Warning::Malformed {
                    domain: field,
                    detail: "a receipt must be identifiable".to_owned(),
                })
            });
        let trusted = match self.phase {
            Phase::Committed => [
                ("request id", self.request_id.is_empty()),
                ("decision id", self.decision_id.is_empty()),
                ("encrypted digest", self.encrypted_payload_digest.is_empty()),
                ("plaintext digest", self.verified_plaintext_digest.is_empty()),
                ("attestation", self.holder_attestation_digest.is_empty()),
                ("lease", self.storage_lease_id.is_empty()),
                ("event", self.committed_event_id.is_empty()),
                ("commit time", self.committed_at.is_empty()),
            ]
            .into_iter()
            .filter_map(|(field, missing)| {
                missing.then(|| Warning::Unverified {
                    subject: field.to_owned(),
                })
            })
            .collect::<Vec<_>>(),
            _ => Vec::new(),
        };
        let digests = [
            self.artifact_digest.as_str(),
            self.encrypted_payload_digest.as_str(),
            self.verified_plaintext_digest.as_str(),
        ]
        .into_iter()
        .filter(|value| !value.is_empty())
        .filter_map(|value| {
            (!Digest::of(value).is_valid()).then(|| Warning::DigestShape {
                field: "receipt digest",
                detail: Digest::of(value).describe(),
            })
        });
        identity.chain(trusted).chain(digests).collect()
    }
}
