//! Exclusive physical isolation for authority-grade measurements.
//!
//! Grounded in `docs/02-pitfalls-and-verification.md:8-9`,
//! `docs/06-state-machines.md:92-102` and `docs/25-failure-analysis.md` F08:
//! `cpuset` alone is not exclusive reservation - sibling hyperthreads, caches
//! and memory bandwidth can still contaminate a run. A held lease must cover
//! the declared physical topology; a contaminated run gets a distinct state and
//! stays in the ledger. This enum makes the distinction structural.

/// Isolation guarantee of a benchmark reservation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Isolation {
    /// Physical cores, siblings and cache topology reserved exclusively.
    ExclusivePhysical,
    /// A cpuset was set but siblings and bandwidth remain shared.
    SharedHyperthreads,
    /// Drift or interference invalidated the samples.
    Contaminated,
}

impl Isolation {
    /// Whether samples under this isolation may be authoritative.
    #[must_use]
    pub const fn authoritative(self) -> crate::Permission {
        match self {
            Self::ExclusivePhysical => crate::Permission::Allow,
            Self::SharedHyperthreads | Self::Contaminated => crate::Permission::Deny,
        }
    }

    /// The state name a contaminated run transitions into
    /// (`docs/06:61-64`).
    #[must_use]
    pub const fn state(self) -> &'static str {
        match self {
            Self::ExclusivePhysical => "sealed",
            Self::SharedHyperthreads => "leased",
            Self::Contaminated => "contaminated",
        }
    }
}
