//! Rebuildable mirror of authoritative state, stale until replayed.
//!
//! Grounded in `docs/08-data-and-storage.md:1-24` (SQLite projections and OTLP
//! stores are rebuildable; the signed journal is canonical),
//! `docs/03-external-api.md:12-14,26-30` (reads carry an explicit
//! `snapshot_seq`; SSE resumes by sequence) and
//! `docs/02-pitfalls-and-verification.md:36` (state read at t differs from an
//! event arriving at t+delta). A mirror never assumes wall-clock ordering and
//! never repairs a missing authoritative event by interpolation
//! (`docs/07-dual-telemetry.md:128-133`).

use crate::{Permission, Warning};

/// A materialized mirror pinned to a ledger sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mirror {
    /// Sequence the mirror was materialized at.
    pub snapshot_seq: u64,
    /// Highest ledger sequence known to have been appended.
    pub ledger_seq: u64,
    /// Whether telemetry correlated into this mirror was complete.
    pub telemetry_complete: crate::Permission,
}

impl Mirror {
    /// Pin a mirror at a snapshot sequence.
    #[must_use]
    pub const fn of(snapshot_seq: u64, ledger_seq: u64) -> Self {
        Self {
            snapshot_seq,
            ledger_seq,
            telemetry_complete: crate::Permission::Deny,
        }
    }

    /// Whether the mirror lags the authoritative journal.
    #[must_use]
    pub fn stale(&self) -> crate::Permission {
        crate::Permission::from(self.snapshot_seq < self.ledger_seq)
    }

    /// Rebuild to a newer ledger sequence; the journal remains authoritative.
    #[must_use]
    pub fn advance(&mut self, ledger_seq: u64) -> Result<Permission, crate::Warning> {
        match ledger_seq >= self.ledger_seq {
            true => {
                self.snapshot_seq = ledger_seq;
                self.ledger_seq = ledger_seq;
                Ok(Permission::Allow)
            }
            false => Err(Warning::StaleProjection {
                have: self.ledger_seq,
                want: ledger_seq,
            }),
        }
    }
}
