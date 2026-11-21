//! Signed authoritative events and resumable ordering.
//!
//! Grounded in `docs/05-event-correspondence.md:43-108` (the 57-event
//! vocabulary, append-only signed ledger ordered by `global_seq`),
//! `docs/04-internal-api.md:11-19` (compare-and-swap with expected revision and
//! last hash) and `docs/03-external-api.md:12-14,26-30` (SSE resumes by
//! sequence, never wall-clock). A missing message is recovered from the ledger,
//! never treated as a new transition (`docs/02:36`, `docs/15` BND-017).

/// One authoritative event in the signed append-only ledger.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    /// Registered event kind from the canonical vocabulary.
    pub kind: &'static str,
    /// Monotonic global sequence assigned by the ledger.
    pub seq: u64,
    /// Aggregate whose revision this event advances.
    pub aggregate: &'static str,
    /// Idempotency key of the command that produced the event.
    pub idempotency: String,
    /// Hash of the previous event, for the hash chain.
    pub previous_hash: crate::Digest,
}

impl Event {
    /// Build an event, refusing an empty idempotency key with a warning.
    #[must_use]
    pub fn of(
        kind: &'static str,
        seq: u64,
        aggregate: &'static str,
        idempotency: String,
        previous_hash: crate::Digest,
    ) -> Result<Self, crate::Warning> {
        match idempotency.is_empty() {
            true => Err(crate::Warning::Replay {
                message_id: "empty idempotency key".to_owned(),
            }),
            false => Ok(Self {
                kind,
                seq,
                aggregate,
                idempotency,
                previous_hash,
            }),
        }
    }

    /// Whether this event is the same command replayed after a crash.
    #[must_use]
    pub fn is_replay_of(&self, other: &Self) -> bool {
        self.idempotency == other.idempotency && self.kind == other.kind
    }

    /// Cursor to resume a projection after this sequence.
    #[must_use]
    pub const fn resume_cursor(&self) -> u64 {
        self.seq
    }
}
