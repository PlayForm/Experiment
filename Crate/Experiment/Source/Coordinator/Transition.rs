//! Typed state machine: forward states, terminal outcomes and retry events.
//!
//! Grounded in `ARCHITECTURE.md:98-104` (the exact transition chain and the
//! eleven failure/retry events), `examples/event.v3.json:3` (`PATCH_SUBMITTED`)
//! and `STORAGE.md:3` (append-only envelopes with previous/next state). State
//! is projected from events; the sequence is not an instruction to an LLM.

/// A named transition or outcome in the experiment lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Transition {
    /// Campaign is active.
    CampaignActive,
    /// Generation parameters frozen before fanout.
    GenerationFrozen,
    /// Immutable task issued to a patcher.
    TaskAllocated,
    /// One committed mutation submitted.
    PatchSubmitted,
    /// Novelty gate has decided.
    NoveltyChecked,
    /// Candidate compiled into an immutable artifact.
    Compiled,
    /// Trusted oracle passed.
    OraclePassed,
    /// Candidate queued for exclusive measurement.
    MeasurementQueued,
    /// Paired trial measured.
    Measured,
    /// Statistical evaluation completed.
    Analyzed,
    /// Eligible for sealed-holdout confirmation.
    ConfirmationEligible,
    /// Human-independent review completed.
    Reviewed,
    /// Accepted into the lineage.
    Accepted,
    /// Rejected with a recorded reason.
    Rejected,
    /// Inconclusive; preserved, never rewritten as a win.
    Inconclusive,
    /// Work-order lease expired and was reclaimed.
    LeaseExpired,
    /// Patch failed the scope gate.
    PatchInvalid,
    /// Exact duplicate already known.
    AlreadyKnown,
    /// Reproducible build failed.
    BuildFailed,
    /// Oracle rejected the candidate.
    OracleFailed,
    /// Benchmark run was contaminated.
    BenchContaminated,
    /// Result moved to quarantine with raw data retained.
    ResultQuarantined,
    /// A budget cap was hit.
    BudgetExhausted,
    /// Worker restarted from the durable transcript.
    WorkerRestarted,
    /// An explicit human halt.
    HumanHalt,
}

impl Transition {
    /// The forward chain before branch outcomes (`ARCHITECTURE.md:100`).
    pub const FORWARD: [Self; 13] = [
        Self::CampaignActive,
        Self::GenerationFrozen,
        Self::TaskAllocated,
        Self::PatchSubmitted,
        Self::NoveltyChecked,
        Self::Compiled,
        Self::OraclePassed,
        Self::MeasurementQueued,
        Self::Measured,
        Self::Analyzed,
        Self::ConfirmationEligible,
        Self::Reviewed,
        Self::Accepted,
    ];

    /// The exact event-kind token used in the event envelope.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::CampaignActive => "CAMPAIGN_ACTIVE",
            Self::GenerationFrozen => "GENERATION_FROZEN",
            Self::TaskAllocated => "TASK_ALLOCATED",
            Self::PatchSubmitted => "PATCH_SUBMITTED",
            Self::NoveltyChecked => "NOVELTY_CHECKED",
            Self::Compiled => "COMPILED",
            Self::OraclePassed => "ORACLE_PASSED",
            Self::MeasurementQueued => "MEASUREMENT_QUEUED",
            Self::Measured => "MEASURED",
            Self::Analyzed => "ANALYZED",
            Self::ConfirmationEligible => "CONFIRMATION_ELIGIBLE",
            Self::Reviewed => "REVIEWED",
            Self::Accepted => "ACCEPTED",
            Self::Rejected => "REJECTED",
            Self::Inconclusive => "INCONCLUSIVE",
            Self::LeaseExpired => "LEASE_EXPIRED",
            Self::PatchInvalid => "PATCH_INVALID",
            Self::AlreadyKnown => "ALREADY_KNOWN",
            Self::BuildFailed => "BUILD_FAILED",
            Self::OracleFailed => "ORACLE_FAILED",
            Self::BenchContaminated => "BENCH_CONTAMINATED",
            Self::ResultQuarantined => "RESULT_QUARANTINED",
            Self::BudgetExhausted => "BUDGET_EXHAUSTED",
            Self::WorkerRestarted => "WORKER_RESTARTED",
            Self::HumanHalt => "HUMAN_HALT",
        }
    }

    /// Whether this transition ends the lane.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Accepted
                | Self::Rejected
                | Self::Inconclusive
                | Self::BudgetExhausted
                | Self::HumanHalt
        )
    }

    /// Whether this is a failure/retry event (`ARCHITECTURE.md:102-104`).
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(
            self,
            Self::LeaseExpired
                | Self::PatchInvalid
                | Self::AlreadyKnown
                | Self::BuildFailed
                | Self::OracleFailed
                | Self::BenchContaminated
                | Self::ResultQuarantined
                | Self::BudgetExhausted
                | Self::WorkerRestarted
                | Self::HumanHalt
        )
    }
}
