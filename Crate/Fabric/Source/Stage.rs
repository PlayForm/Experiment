//! The three signed phases of a schema or policy migration.
//!
//! Mirrors the `phase` enumeration of `schemas/migration-transition.schema.json`
//! and the write-ahead prepare / atomic promote / final activation sequence in
//! `docs/06:32-37`. A rollback is a new authorized transition, never an erase.

/// One phase of an immutable migration transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage {
    /// Invariants validated; nothing activated.
    Prepared,
    /// Atomically promoted and recorded.
    Activated,
    /// Aborted with the journal left intact.
    Aborted,
}

impl Stage {
    /// Wire spelling from the migration schema.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::Activated => "activated",
            Self::Aborted => "aborted",
        }
    }

    /// Whether this phase makes a new schema reader authoritative.
    #[must_use]
    pub const fn active(&self) -> bool {
        matches!(self, Self::Activated)
    }

    /// A rollback transition must prepare a new activation, never delete an
    /// earlier journal record (`docs/06:37`).
    #[must_use]
    pub const fn rollback_requires_transition() -> crate::verdict::Verdict {
        crate::verdict::Verdict::Allow
    }
}
