//! The purpose of a mutation-parent edge.
//!
//! Grounded in `sql/lineage.sql:51-58`: `mutation_parents.purpose` is
//! constrained to `('causal', 'merge')`, the many-to-many ancestry relation.
//! A merge requires a new baseline and never inherits a parent's performance
//! claim (LAW-007, REQ-MUT-04).

/// Why one mutation claims another as parent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Purpose {
    /// A one-change causal lane with a single parent (REQ-LIN-01).
    Causal,
    /// A merge/integration that enumerates all parents.
    Merge,
}

impl Purpose {
    /// The exact SQL `CHECK` token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Causal => "causal",
            Self::Merge => "merge",
        }
    }

    /// Whether a merged-baseline measurement is required
    /// (`REQUIREMENTS.md:326-327`).
    #[must_use]
    pub const fn needs_new_baseline(self) -> bool {
        matches!(self, Self::Merge)
    }
}
