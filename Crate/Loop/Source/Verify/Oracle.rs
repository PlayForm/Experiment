//! The three trusted oracle kinds, all sourced from frozen material.
//! Grounded in `ARCHITECTURE.md:52-55` (tests and fixtures come from frozen
//! trusted commits; an agent-proposed oracle adjustment is a separate
//! reviewable mutation) and `18-emergent-evolution.md:30-35` (proposed tests
//! enter quarantine and cannot change trusted oracle inputs).

/// A trusted correctness check; none can be supplied by a worker.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Oracle {
    /// Behaviour against the frozen reference contract.
    Reference,
    /// Old-versus-new output on the same frozen inputs.
    Differential,
    /// Reachability of required trusted sentences.
    Coverage,
}

impl Oracle {
    /// All trusted oracle kinds.
    pub const ALL: [Self; 3] = [Self::Reference, Self::Differential, Self::Coverage];

    /// Every oracle here is trusted by construction.
    #[must_use]
    pub const fn is_trusted(&self) -> bool {
        true
    }

    /// A worker-supplied test can never become an oracle.
    #[must_use]
    pub const fn from_worker() -> Option<Self> {
        None
    }

    /// Stable key for the oracle attestation.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::Differential => "differential",
            Self::Coverage => "coverage",
        }
    }
}
