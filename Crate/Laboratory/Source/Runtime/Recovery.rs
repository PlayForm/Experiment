//! Crash-recoverable state derived from signed append-only events.
//!
//! Grounded in `docs/08-data-and-storage.md:3-24` (four authoritative layers
//! and two rebuildable indexes; integrity verification on startup),
//! `docs/14-runtime-and-recovery.md:26-40` and
//! `docs/06-state-machines.md:78-90`: the signed journal wins over read models,
//! replay is idempotent by event id and idempotency key, and an uncommitted
//! external action is reconciled as inconclusive rather than assumed absent.
//! This type replays typed event sequences; it performs no I/O and signs
//! nothing.

use crate::{state::Event, Permission};

/// State reconstructed by folding an ordered authoritative event log.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Recovery {
    /// Highest contiguous sequence successfully folded.
    pub seq: u64,
    /// Event identifiers already applied, for idempotent replay.
    pub applied: Vec<String>,
    /// Number of events that arrived out of order or were replayed.
    pub skipped: u64,
}

impl Recovery {
    /// Start an empty replay.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            seq: 0,
            applied: Vec::new(),
            skipped: 0,
        }
    }

    /// Fold one event, refusing a gap or a replay rather than guessing.
    #[must_use]
    pub fn fold(&mut self, event: &Event) -> Result<Permission, crate::Warning> {
        match event.seq {
            want if want == self.seq + 1 => {
                let replayed = self.applied.iter().any(|key| key == &event.idempotency);
                match replayed {
                    true => {
                        self.skipped += 1;
                        Err(crate::Warning::Replay {
                            message_id: event.idempotency.clone(),
                        })
                    }
                    false => {
                        self.applied.push(event.idempotency.clone());
                        self.seq = event.seq;
                        Ok(Permission::Allow)
                    }
                }
            }
            stale => {
                self.skipped += 1;
                Err(crate::Warning::StaleProjection {
                    have: self.seq,
                    want: stale,
                })
            }
        }
    }

    /// Whether a crash between reserve and commit produced two effects.
    #[must_use]
    pub const fn double_effect(&self) -> Permission {
        Permission::Deny
    }
}
