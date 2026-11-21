//! The exclusive benchmark queue.
//! Grounded in `ARCHITECTURE.md:82-86` (multiple ready candidates queue; do
//! not run five competing measurements on shared cores and disks) and in
//! `AGENT_CONTRACT.md:53-55` (never benchmark candidate families
//! simultaneously on shared hardware). Coupled to the v0 controller dispatch.

use benchmark::controller::{Dispatch, ParallelRunners};

/// A queued measurement described by the trusted controller dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Queue {
    /// The dispatch description; never an executed command.
    pub dispatch: Dispatch,
    /// Configured fanout that must be serialised onto this queue.
    pub fanout: usize,
}

impl Queue {
    /// The authoritative queue: serial inside a group, no parallel overlaps.
    #[must_use]
    pub fn exclusive() -> Self {
        Self {
            dispatch: Dispatch::Bench {
                parallel: ParallelRunners::Serial,
            },
            fanout: crate::FANOUT,
        }
    }

    /// Whether this queue runs measurements exclusively.
    #[must_use]
    pub fn is_exclusive(&self) -> bool {
        matches!(
            self.dispatch,
            Dispatch::Bench {
                parallel: ParallelRunners::Serial
            }
        )
    }

    /// Whether the configured fanout is bounded by the queue.
    #[must_use]
    pub const fn is_bounded(&self) -> bool {
        self.fanout <= crate::FANOUT
    }
}
