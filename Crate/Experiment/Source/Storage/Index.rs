//! Rebuildable search, telemetry and lineage projections.
//!
//! Grounded in `STORAGE.md:1-21,34-37` (dashboard projections and leaderboards
//! are not authoritative; indexes are rebuildable) and `REQUIREMENTS_V3.md`
//! DAT-001/DAT-007. The UI is never primary storage and must detect missing
//! events on cursor resume. Exception declared: `Freshness` is the typed form
//! of this projection's state and has no independent lifecycle outside `Index`.

use crate::storage::ledger::Ledger;

/// Whether a projection is current with the ledger.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Freshness {
    /// The projection has consumed the full ledger.
    Current,
    /// The projection lags; the UI must replay missing events.
    Lagging,
}

/// A rebuildable projection over authoritative records.
#[derive(Clone, Debug, Default)]
pub struct Index {
    /// Number of indexed events.
    pub indexed: u64,
    /// Identifier of the projection kind, for dashboards.
    pub name: String,
}

impl Index {
    /// Rebuild a projection from the ledger, recording the consumed count.
    #[must_use]
    pub fn from_ledger(ledger: &Ledger) -> Self {
        Self {
            indexed: ledger.events.len() as u64,
            name: "lineage".to_string(),
        }
    }

    /// Whether the projection is current relative to the ledger.
    #[must_use]
    pub fn freshness(&self, ledger: &Ledger) -> Freshness {
        match self.indexed == ledger.events.len() as u64 {
            true => Freshness::Current,
            false => Freshness::Lagging,
        }
    }

    /// A projection is never authoritative (`DAT-001`).
    #[must_use]
    pub const fn is_authoritative() -> bool {
        false
    }
}
