//! The append transition over the canonical event ledger.
//!
//! Appending is a typed decision: a new idempotency key commits, a repeated
//! key replays the original receipt, and an invalid event is refused with a
//! warning. No table is opened and no row is written (`docs/14:26-29`).

use super::event::Event;
use crate::warning::Warning;

/// The result of an append attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Append {
    /// A new event was accepted by the transition.
    Committed(Event),
    /// The key already had an event; the original receipt is returned.
    Duplicate(String),
    /// The event was malformed and was not appended.
    Refused(Warning),
}

impl Append {
    /// Decide an append against the keys already recorded.
    #[must_use]
    pub fn of(event: Event, seen_keys: &[String]) -> Self {
        let duplicate = seen_keys
            .iter()
            .find(|key| **key == event.idempotency_key)
            .cloned();
        match duplicate {
            Some(key) => Self::Duplicate(key),
            None => match event.validate().into_iter().next() {
                Some(warning) => Self::Refused(warning),
                None => Self::Committed(event),
            },
        }
    }

    /// The committed event, when this append produced one.
    #[must_use]
    pub fn committed(&self) -> Option<&Event> {
        match self {
            Self::Committed(event) => Some(event),
            Self::Duplicate(_) | Self::Refused(_) => None,
        }
    }

    /// The idempotency key that makes this append safe to retry.
    #[must_use]
    pub fn key(&self) -> &str {
        match self {
            Self::Committed(event) => &event.idempotency_key,
            Self::Duplicate(key) => key,
            Self::Refused(_) => "",
        }
    }
}
