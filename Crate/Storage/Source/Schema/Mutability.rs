//! Whether a table is an immutable record or a rebuildable projection.
//!
//! The chain protects journals, correspondence, receipts, policy, feedback,
//! facts, turns, claims, bids, reviews, manuscripts, attestations and study
//! outcomes with `BEFORE UPDATE` and `BEFORE DELETE` triggers; queue and
//! outbox state may change (`sql/v6_migration.sql:224-262`,
//! `sql/prerequisites/v3_migration.sql:191-229`).

/// The mutation policy of one table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mutability {
    /// Append-only: update and delete are refused.
    Immutable,
    /// A rebuildable projection whose state may be advanced.
    Projection,
}

impl Mutability {
    /// Guard an update or delete against this table.
    #[must_use]
    pub fn mutation(&self) -> crate::permission::Permission {
        match self {
            Self::Immutable => crate::permission::Permission::Deny,
            Self::Projection => crate::permission::Permission::Allow,
        }
    }

    /// Name the policy.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Immutable => "append-only",
            Self::Projection => "rebuildable projection",
        }
    }
}
