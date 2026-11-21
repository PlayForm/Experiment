//! The validity timescale of a bi-temporal fact.
//!
//! `valid_from` and optional `valid_to` are distinct from observation and
//! recording times. Timestamps are canonical UTC RFC3339 strings, so lexical
//! order equals chronological order for the pinned format
//! (`docs/21:39-46`; `examples/v61-temporal-fact`).

use crate::permission::Permission;

/// The interval during which a claim was true.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Valid {
    /// `valid_from` canonical UTC instant.
    pub from: String,
    /// `valid_to` canonical UTC instant, absent while still valid.
    pub to: Option<String>,
}

impl Valid {
    /// Whether an instant lies inside the validity interval.
    #[must_use]
    pub fn covers(&self, at: &str) -> Permission {
        let started = Permission::filled(&self.from).and(Permission::of(at >= self.from.as_str()));
        let ended = match &self.to {
            None => Permission::Allow,
            Some(until) => Permission::of(at <= until.as_str()),
        };
        started.and(ended)
    }

    /// Whether the interval is still open.
    #[must_use]
    pub fn open(&self) -> bool {
        self.to.is_none()
    }
}
