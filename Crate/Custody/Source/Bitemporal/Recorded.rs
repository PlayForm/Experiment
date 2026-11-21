//! The recording timescale of a bi-temporal fact.
//!
//! UTC is only an approximate cross-host clock; a hash-linked ledger sequence
//! establishes authoritative receipt order, and monotonic clocks measure
//! duration rather than causal order (`docs/21:39-46`, `docs/25:22`).

use crate::permission::Permission;

/// The ledger identity and recording time of a claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Recorded {
    /// `recorded_at` canonical UTC instant.
    pub at: String,
    /// `source_event_id` linking to the authoritative journal.
    pub source_event_id: String,
    /// Monotonic sequence position assigned by the ledger.
    pub sequence: u64,
}

impl Recorded {
    /// Whether a fact was known from a given recorded view.
    #[must_use]
    pub fn before(&self, known_at: &str) -> Permission {
        Permission::filled(&self.at)
            .and(Permission::filled(&self.source_event_id))
            .and(Permission::of(self.at <= known_at))
    }

    /// Whether one recorded fact precedes another in the ledger sequence.
    #[must_use]
    pub fn precedes(&self, other: &Self) -> Permission {
        Permission::of(self.sequence > 0 && self.sequence < other.sequence)
    }
}
