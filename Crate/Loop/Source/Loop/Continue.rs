//! Typed reasons to keep a lane alive.
//! Grounded in `ARCHITECTURE.md:87-96` (a selected candidate may receive a
//! marginal comparison and a confirmation on held-out workloads; the director
//! may trigger a new fanout or cancel a track) and `AGENT_CONTRACT.md:77-89`
//! (recovery paths that retry within budget rather than fabricate success).

/// A progress signal that authorises the next generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Continue {
    /// A novel candidate is ready for build and oracle.
    NovelCandidate,
    /// A candidate passed development and is confirmation eligible.
    ConfirmationEligible,
    /// The budget envelope still admits another mutation.
    BudgetRemains,
    /// Independent replication is required before promotion.
    ReplicationRequired,
}

impl Continue {
    /// Stable key for the operator projection.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::NovelCandidate => "novel_candidate",
            Self::ConfirmationEligible => "confirmation_eligible",
            Self::BudgetRemains => "budget_remains",
            Self::ReplicationRequired => "replication_required",
        }
    }

    /// Whether this signal allows a new fanout to be scheduled.
    #[must_use]
    pub const fn schedules_fanout(self) -> bool {
        matches!(self, Self::NovelCandidate | Self::BudgetRemains)
    }
}
