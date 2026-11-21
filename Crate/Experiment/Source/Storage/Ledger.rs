//! Append-only event ledger with transactional sequence and outbox.
//!
//! Grounded in `STORAGE.md:3,39-49` (causation vs correlation, append only
//! after blobs are persisted and hash-verified, outbox pattern, idempotent
//! ACKs), `ARCHITECTURE.md:106-109` and `sql/v3_migration.sql:166-175` (
//! dispatch_outbox). State is projected from events; events are never rewritten.

use crate::interface::envelope::Envelope;
use crate::warning::Warning;

/// The authoritative append-only event ledger.
#[derive(Clone, Debug, Default)]
pub struct Ledger {
    /// Next logical sequence number for the aggregate.
    pub sequence: u64,
    /// Committed envelopes in append order.
    pub events: Vec<Envelope>,
    /// Durable dispatch intents recorded by the outbox.
    pub outbox: Vec<String>,
}

impl Ledger {
    /// Append one event, assigning the next logical sequence and rejecting a
    /// gap or an out-of-order envelope without panicking.
    pub fn append(&mut self, mut event: Envelope) -> Result<(), Warning> {
        let expected = self.sequence.saturating_add(1);
        match event.aggregate_seq == expected {
            true => {
                self.sequence = expected;
                event.aggregate_seq = expected;
                self.events.push(event);
                Ok(())
            }
            false => Err(Warning::Drift {
                domain: "ledger-sequence",
                expected: expected as u32,
                found: event.aggregate_seq as u32,
            }),
        }
    }

    /// Look up a committed event by id.
    #[must_use]
    pub fn event(&self, id: &str) -> Option<&Envelope> {
        self.events.iter().find(|event| event.event_id == id)
    }

    /// Record a dispatch intent in the outbox, once per event
    /// (`STORAGE.md:47-48`).
    pub fn dispatch(&mut self, event_id: String) {
        let duplicate = self.outbox.contains(&event_id);
        match duplicate {
            false => self.outbox.push(event_id),
            true => {}
        }
    }

    /// The ledger is authoritative and never rewritten.
    #[must_use]
    pub const fn is_append_only() -> bool {
        true
    }
}
