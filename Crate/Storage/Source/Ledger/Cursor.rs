//! A replay position over the canonical journal.

use super::event::Event;
use crate::permission::Permission;

/// A cursor that selects events after a sequence, optionally by kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cursor {
    /// Last sequence already applied by the projection.
    pub after: u64,
    /// Maximum number of events to replay in one pass.
    pub limit: u16,
    /// Optional event-kind filter.
    pub kind: Option<String>,
}

impl Cursor {
    /// Declare a cursor.
    #[must_use]
    pub fn of(after: u64, limit: u16, kind: Option<String>) -> Self {
        Self { after, limit, kind }
    }

    /// The cursor at the journal origin.
    #[must_use]
    pub fn begin() -> Self {
        Self::of(0, 500, None)
    }

    /// Whether an event falls after this cursor and within its filter.
    #[must_use]
    pub fn admits(&self, event: &Event) -> Permission {
        let in_window = event.global_sequence > self.after
            && event.global_sequence <= self.after + u64::from(self.limit);
        let kind_matches = self
            .kind
            .as_ref()
            .map_or(true, |wanted| wanted == &event.kind);
        match (in_window, kind_matches) {
            (true, true) => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Advance the cursor past an applied event.
    #[must_use]
    pub fn advance(self, event: &Event) -> Self {
        Self {
            after: event.global_sequence,
            limit: self.limit,
            kind: self.kind,
        }
    }

    /// Describe the cursor.
    #[must_use]
    pub fn describe(&self) -> String {
        self.kind.as_ref().map_or_else(
            || format!("after={}, limit={}", self.after, self.limit),
            |kind| format!("after={}, limit={}, kind={kind}", self.after, self.limit),
        )
    }
}
