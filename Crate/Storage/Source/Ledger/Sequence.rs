//! Global and per-aggregate positions in the immutable journal.

/// One journal position pair: global order and aggregate-local order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sequence {
    /// Position in the global journal.
    pub global: u64,
    /// Position within one aggregate, starting at one.
    pub aggregate: u64,
}

impl Sequence {
    /// The origin before any event is recorded.
    #[must_use]
    pub fn origin() -> Self {
        Self {
            global: 0,
            aggregate: 0,
        }
    }

    /// The first position of a new aggregate.
    #[must_use]
    pub fn first(global: u64) -> Self {
        Self {
            global,
            aggregate: 1,
        }
    }

    /// Advance both positions by one.
    #[must_use]
    pub fn advance(self) -> Self {
        Self {
            global: self.global + 1,
            aggregate: self.aggregate + 1,
        }
    }

    /// Advance only the global position, as when another aggregate commits.
    #[must_use]
    pub fn advance_global(self) -> Self {
        Self {
            global: self.global + 1,
            aggregate: self.aggregate,
        }
    }

    /// Whether this position follows another without a gap.
    #[must_use]
    pub fn follows(&self, prior: &Self) -> crate::permission::Permission {
        match (self.global == prior.global + 1, self.aggregate > prior.aggregate) {
            (true, true) => crate::permission::Permission::Allow,
            _ => crate::permission::Permission::Deny,
        }
    }

    /// Describe the position.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("global={}, aggregate={}", self.global, self.aggregate)
    }
}
