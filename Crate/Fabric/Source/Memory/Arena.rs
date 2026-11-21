//! Arenas optimize locality; they are not vaults or databases.
//!
//! `docs/06:1-8` permits arenas for temporary parse/compile state, freezes
//! long-lived snapshots and swaps versions atomically after migration.
//! `docs/01:28-33` forbids retaining references across reclamation. Two
//! structural states replace a boolean field.

/// One arena lifetime state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arena {
    /// Temporary parse/compile state subject to reclamation.
    Growing,
    /// Frozen read-only snapshot swapped in atomically.
    Frozen,
}

impl Arena {
    /// Whether references into this arena may outlive a reclamation.
    #[must_use]
    pub const fn retains_across_reclaim(&self) -> bool {
        matches!(self, Self::Frozen)
    }

    /// An arena is never a durable or confidential store on its own
    /// (`README.md:27-29`).
    #[must_use]
    pub const fn holds_immutable_history() -> bool {
        false
    }

    /// The durable backing required behind any projection.
    #[must_use]
    pub const fn backing() -> &'static str {
        "encrypted signed append-only journal"
    }
}
