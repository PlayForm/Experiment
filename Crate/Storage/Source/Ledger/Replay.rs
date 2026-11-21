//! Deterministic replay from the immutable journal.
//!
//! A subscriber that missed messages replays the outbox from the ledger
//! sequence and applies events idempotently (`docs/14-runtime-and-recovery.md:36`).
//! The projection built here is a value, never the only copy of state.

use super::cursor::Cursor;
use super::event::Event;
use crate::warning::Warning;

/// A bounded replay over a supplied event slice.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Replay {
    cursor: Cursor,
    events: Vec<Event>,
}

impl Replay {
    /// Declare a replay from a cursor and the journal events available to it.
    #[must_use]
    pub fn of(cursor: Cursor, events: Vec<Event>) -> Self {
        Self { cursor, events }
    }

    /// Events selected by the cursor in journal order.
    #[must_use]
    pub fn selected(&self) -> Vec<&Event> {
        let mut selected = self
            .events
            .iter()
            .filter(|event| self.cursor.admits(*event) == crate::permission::Permission::Allow)
            .collect::<Vec<_>>();
        selected.sort_by_key(|event| event.global_sequence);
        selected
    }

    /// Verify the hash chain of the selected events.
    #[must_use]
    pub fn chain_findings(&self) -> Vec<Warning> {
        let selected = self.selected();
        selected
            .windows(2)
            .filter_map(|pair| match pair {
                [left, right] if left.event_hash != right.previous_hash => {
                    Some(Warning::Divergence {
                        expected: left.global_sequence,
                        observed: right.global_sequence,
                    })
                }
                _ => None,
            })
            .collect()
    }

    /// The last selected event, used to advance a projection cursor.
    #[must_use]
    pub fn last(&self) -> Option<&Event> {
        self.selected().into_iter().last()
    }

    /// Report gaps between selected sequences.
    #[must_use]
    pub fn gaps(&self) -> Vec<Warning> {
        self.selected()
            .windows(2)
            .filter_map(|pair| match pair {
                [left, right] if right.global_sequence != left.global_sequence + 1 => {
                    Some(Warning::OutOfOrder {
                        have: left.global_sequence,
                        want: right.global_sequence,
                    })
                }
                _ => None,
            })
            .collect()
    }
}
