//! The independent novelty and correctness oracle.
//!
//! Grounded in `SYSTEM.md:51,93-95` (the experiment authority owns fixture
//! definitions, oracle acceptance, sample integrity and benchmark leases;
//! researchers cannot rewrite these), `README.md:44-45` (agents cannot alter an
//! oracle) and `SYSTEM.md:120` (oracle_pass is not proof or a scientific stamp).

use crate::Permission;

/// The two independent oracles of a research case.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Oracle {
    /// Semantic correctness under defined tests (`PUBLISHING.md:17-19`).
    Correctness,
    /// Novelty fingerprint and duplicate detection (`MARKET.md:34,59-61`).
    Novelty,
}

impl Oracle {
    /// Both oracles in declaration order.
    pub const ALL: [Self; 2] = [Self::Correctness, Self::Novelty];

    /// Name of the oracle family.
    #[must_use]
    pub const fn family(self) -> &'static str {
        match self {
            Self::Correctness => "semantic_correctness",
            Self::Novelty => "novelty_fingerprint",
        }
    }

    /// No researcher may rewrite an oracle (`SYSTEM.md:93-95`).
    #[must_use]
    pub const fn may_be_altered_by_agent() -> Permission {
        Permission::Deny
    }

    /// Passing an oracle is not proof of all behaviour (`PUBLISHING.md:17-19`,
    /// `SYSTEM.md:120`).
    #[must_use]
    pub const fn is_proof() -> Permission {
        Permission::Deny
    }
}
