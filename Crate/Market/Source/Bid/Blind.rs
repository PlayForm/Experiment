//! Blind phase marker for independent opinion.
//!
//! Grounded in `AGENTS.md:24-30` (phase A gives the same minimal task and
//! different context capsules, never sibling responses; phase B may expose
//! disagreements under a new context ID), `sql/v5_migration.sql:43`
//! (`blinded_phase` in {0,1}), `MARKET.md:105-106` (no peer drafts in the blind
//! phase) and `OPERATIONS.md:52` (blind researchers cannot query siblings via
//! API, worktree, logs or telemetry). This is a typed marker, not a scheduler.

use crate::Permission;

/// The blind phase an assignment is currently in.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Blind {
    /// Same frozen task, independent context capsules, no peer exposure.
    PhaseA,
    /// Deliberate disagreement and rebuttal under a new context ID.
    PhaseB,
}

impl Blind {
    /// Both phases in order.
    pub const ALL: [Self; 2] = [Self::PhaseA, Self::PhaseB];

    /// The SQL `blinded_phase` value.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::PhaseA => 0,
            Self::PhaseB => 1,
        }
    }

    /// A peer draft is never visible while blind (`OPERATIONS.md:52`).
    #[must_use]
    pub const fn may_see_peer_draft(self) -> Permission {
        match self {
            Self::PhaseA => Permission::Deny,
            Self::PhaseB => Permission::Allow,
        }
    }

    /// Sealed holdout selection is invisible in either phase
    /// (`AGENTS.md:12`, `MARKET.md:92-94`).
    #[must_use]
    pub const fn may_see_holdout() -> Permission {
        Permission::Deny
    }

    /// Same model plus different prompts is not independence
    /// (`AGENTS.md:27`, `MARKET.md:96-100`).
    #[must_use]
    pub const fn proves_statistical_independence() -> Permission {
        Permission::Deny
    }
}
