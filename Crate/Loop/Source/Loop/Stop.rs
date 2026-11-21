//! Typed reasons to stop or re-scope a lane.
//! Grounded in `ARCHITECTURE.md:138-142` (repeated nulls, falling expected
//! value, novelty loss, repeated semantic failures, unestablished confidence,
//! finite caps and watchdog), `MARKET.md:117-123` and
//! `OPERATIONS.md:11-29` (stale evidence, repeated invalid oracles, provider
//! outage, operator stop, information-value threshold).

/// A stop signal, never a worker-visible failure reason.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Stop {
    /// The playbook budget is exhausted.
    BudgetExhausted,
    /// Repeated null results in the lane.
    RepeatedNull,
    /// No admissible novelty remains.
    NoveltyExhausted,
    /// The same semantic failure keeps recurring.
    SemanticFailuresRepeat,
    /// Confidence could not be established within budget.
    ConfidenceUnestablished,
    /// Free disk fell below the floor.
    DiskPressure,
    /// Host health alarm: fail closed.
    HostHealthAlarm,
    /// An operator halted the campaign.
    OperatorHalt,
    /// Expected value of information dropped below marginal cost.
    ValueBelowCost,
}

impl Stop {
    /// All stop signals, in the order the operator report lists them.
    pub const ALL: [Self; 9] = [
        Self::BudgetExhausted,
        Self::RepeatedNull,
        Self::NoveltyExhausted,
        Self::SemanticFailuresRepeat,
        Self::ConfidenceUnestablished,
        Self::DiskPressure,
        Self::HostHealthAlarm,
        Self::OperatorHalt,
        Self::ValueBelowCost,
    ];

    /// Stable key for the ledger and operator report.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::BudgetExhausted => "budget_exhausted",
            Self::RepeatedNull => "repeated_null",
            Self::NoveltyExhausted => "novelty_exhausted",
            Self::SemanticFailuresRepeat => "semantic_failures_repeat",
            Self::ConfidenceUnestablished => "confidence_unestablished",
            Self::DiskPressure => "disk_pressure",
            Self::HostHealthAlarm => "host_health_alarm",
            Self::OperatorHalt => "operator_halt",
            Self::ValueBelowCost => "value_below_cost",
        }
    }
}
