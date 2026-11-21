//! One immutable, signed journal record.
//!
//! Mirrors `v6_event_journal` (`sql/v6_migration.sql:4-30`). The signature
//! itself is created and verified by trusted software elsewhere; this type
//! records the artifact digest and never asserts truth from a signature.

use crate::cas::digest::Digest;
use super::privacy::Privacy;
use super::sequence::Sequence;

/// One append-only event with correlation and hash-chain identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    /// Global monotonically increasing journal position.
    pub global_sequence: u64,
    /// Unique event identifier.
    pub event_id: String,
    /// Kind from the contract event catalog.
    pub kind: String,
    /// Aggregate this event belongs to.
    pub aggregate_id: String,
    /// Per-aggregate monotonically increasing position, starting at one.
    pub aggregate_sequence: u64,
    /// UTC recording time.
    pub recorded_utc: String,
    /// Acting subject.
    pub actor_id: String,
    /// Credential that authorized the transition.
    pub credential_id: String,
    /// Correlation identifier shared by a causal chain of events.
    pub correlation_id: String,
    /// Optional id of the event that caused this one.
    pub causation_event_id: Option<String>,
    /// Idempotency key that makes retries safe.
    pub idempotency_key: String,
    /// Digest of the serialized payload.
    pub payload_digest: Digest,
    /// Hash of the preceding event; the chain root for the first event.
    pub previous_hash: Digest,
    /// Hash of this event.
    pub event_hash: Digest,
    /// Digest of the signature artifact attesting this event.
    pub signature_artifact_digest: Digest,
    /// Disclosure class.
    pub privacy: Privacy,
}

impl Event {
    /// Declare the invariant fields of an event; derived fields are added by
    /// the named setters below.
    #[must_use]
    pub fn of(
        event_id: &str,
        kind: &str,
        aggregate_id: &str,
        aggregate_sequence: u64,
        correlation_id: &str,
        idempotency_key: &str,
        payload_digest: Digest,
    ) -> Self {
        Self {
            global_sequence: 0,
            event_id: event_id.to_owned(),
            kind: kind.to_owned(),
            aggregate_id: aggregate_id.to_owned(),
            aggregate_sequence,
            recorded_utc: String::new(),
            actor_id: String::new(),
            credential_id: String::new(),
            correlation_id: correlation_id.to_owned(),
            causation_event_id: None,
            idempotency_key: idempotency_key.to_owned(),
            payload_digest,
            previous_hash: Digest::zero(),
            event_hash: Digest::zero(),
            signature_artifact_digest: Digest::zero(),
            privacy: Privacy::Restricted,
        }
    }

    /// Attach the global sequence assigned by the journal.
    #[must_use]
    pub fn sequenced(mut self, sequence: Sequence) -> Self {
        self.global_sequence = sequence.global;
        self.aggregate_sequence = sequence.aggregate;
        self
    }

    /// Attach the UTC recording time and the acting subjects.
    #[must_use]
    pub fn attributed(mut self, utc: &str, actor: &str, credential: &str) -> Self {
        self.recorded_utc = utc.to_owned();
        self.actor_id = actor.to_owned();
        self.credential_id = credential.to_owned();
        self
    }

    /// Attach the hash-chain and signature digests.
    #[must_use]
    pub fn chained(
        mut self,
        previous: Digest,
        current: Digest,
        signature: Digest,
    ) -> Self {
        self.previous_hash = previous;
        self.event_hash = current;
        self.signature_artifact_digest = signature;
        self
    }

    /// Attach causation and disclosure.
    #[must_use]
    pub fn caused(mut self, cause: Option<String>, privacy: Privacy) -> Self {
        self.causation_event_id = cause;
        self.privacy = privacy;
        self
    }

    /// Report malformed fields rather than panicking.
    #[must_use]
    pub fn validate(&self) -> Vec<crate::warning::Warning> {
        let empty = [
            ("event id", self.event_id.is_empty()),
            ("event kind", self.kind.is_empty()),
            ("aggregate id", self.aggregate_id.is_empty()),
            ("correlation id", self.correlation_id.is_empty()),
            ("idempotency key", self.idempotency_key.is_empty()),
            ("recorded utc", self.recorded_utc.is_empty()),
            ("actor id", self.actor_id.is_empty()),
            ("credential id", self.credential_id.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, missing)| {
            missing.then(|| crate::warning::Warning::Malformed {
                domain: field,
                detail: "a committed journal event must name this field".to_owned(),
            })
        });
        let bad_sequence = (self.aggregate_sequence == 0).then(|| {
            crate::warning::Warning::InvalidSequence {
                expected: 1,
                found: self.aggregate_sequence,
            }
        });
        let bad_digests = [
            ("payload digest", &self.payload_digest),
            ("previous hash", &self.previous_hash),
            ("event hash", &self.event_hash),
            ("signature digest", &self.signature_artifact_digest),
        ]
        .into_iter()
        .filter_map(|(field, digest)| {
            (!digest.is_valid()).then(|| crate::warning::Warning::DigestShape {
                field,
                detail: digest.describe(),
            })
        });
        empty.chain(bad_sequence).chain(bad_digests).collect()
    }
}
