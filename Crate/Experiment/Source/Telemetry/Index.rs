//! Signal index: join planes by immutable ids, never by metric labels.
//!
//! Grounded in `TELEMETRY.md:21-25,87-89,160-170` (join with campaign,
//! experiment, mutation, binary, trial ids and an explicit correlation event;
//! high-cardinality ids go in trace attributes, not metric labels; a
//! correlation fabric maps both boards to immutable ids). The index is a
//! rebuildable projection, never authoritative (`STORAGE.md:1-21`).

/// A rebuildable signal index projection.
#[derive(Clone, Debug, Default)]
pub struct Index {
    /// Immutable correlation ids available on both boards.
    pub correlation_ids: Vec<String>,
    /// Low-cardinality label keys admitted to metric aggregates.
    pub label_keys: Vec<String>,
    /// Observed projection lag in event sequence numbers.
    pub lag_events: u64,
}

impl Index {
    /// Whether a key is safe as a metric label. Campaign, run, patch and trial
    /// ids are explicitly excluded (`TELEMETRY.md:87-89`).
    #[must_use]
    pub fn is_label_safe(&self, key: &str) -> bool {
        self.label_keys.iter().any(|safe| safe == key)
    }

    /// Record a correlation id once, ignoring duplicates.
    pub fn correlate(&mut self, id: String) {
        let duplicate = self.correlation_ids.contains(&id);
        match duplicate {
            false => self.correlation_ids.push(id),
            true => {}
        }
    }

    /// Whether the index is fresh enough to render a dashboard board.
    #[must_use]
    pub const fn is_fresh(&self, tolerance: u64) -> bool {
        self.lag_events <= tolerance
    }
}
