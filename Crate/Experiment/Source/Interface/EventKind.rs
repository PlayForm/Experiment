//! Typed event-kind vocabulary of the v3 event envelope.
//!
//! File name is the card-mandated `EventKind.rs`; the Rust type is the
//! single-word `Kind` in the `interface::kind` module (declared house-style
//! exception: the file name is fixed by the task, the identifier is not).
//!
//! Grounded in `ARCHITECTURE.md:98-104` (the forward chain and eleven
//! failure/retry events), `examples/event.v3.json:3` (`PATCH_SUBMITTED`) and
//! `interfaces/event-envelope.v3.schema.json:27-30`. These are deterministic
//! transition markers; an LLM never authors or signs one.

/// The discriminant of a v3 event envelope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Kind {
    /// Campaign became active.
    CampaignActive,
    /// Generation parameters frozen.
    GenerationFrozen,
    /// Immutable task allocated.
    TaskAllocated,
    /// Committed patch submitted.
    PatchSubmitted,
    /// Novelty gate decided.
    NoveltyChecked,
    /// Source compiled.
    Compiled,
    /// Oracle passed.
    OraclePassed,
    /// Measurement queued.
    MeasurementQueued,
    /// Paired measurement completed.
    Measured,
    /// Evaluation completed.
    Analyzed,
    /// Eligible for confirmation.
    ConfirmationEligible,
    /// Review completed.
    Reviewed,
    /// Accepted.
    Accepted,
    /// Rejected with a reason.
    Rejected,
    /// Inconclusive, preserved.
    Inconclusive,
    /// Lease expired and was reclaimed.
    LeaseExpired,
    /// Patch failed scope gate.
    PatchInvalid,
    /// Already known exact duplicate.
    AlreadyKnown,
    /// Build failed.
    BuildFailed,
    /// Oracle failed.
    OracleFailed,
    /// Benchmark contaminated.
    BenchContaminated,
    /// Result quarantined.
    ResultQuarantined,
    /// Budget exhausted.
    BudgetExhausted,
    /// Worker restarted from ledger.
    WorkerRestarted,
    /// Explicit human halt.
    HumanHalt,
}

impl Kind {
    /// The exact schema token (`ArchitECTURE.md:98-104`).
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

    /// Parse an envelope token into a typed kind, degrading to `None` on an
    /// unrecognized token rather than panicking.
    #[must_use]
    pub fn parse(token: &str) -> Option<Self> {
        Self::all().into_iter().find(|kind| kind.key() == token)
    }

    /// All twenty-five event kinds, in declaration order.
    #[must_use]
    pub fn all() -> [Self; 25] {
        [
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
            Self::Rejected,
            Self::Inconclusive,
            Self::LeaseExpired,
            Self::PatchInvalid,
            Self::AlreadyKnown,
            Self::BuildFailed,
            Self::OracleFailed,
            Self::BenchContaminated,
            Self::ResultQuarantined,
            Self::BudgetExhausted,
            Self::WorkerRestarted,
            Self::HumanHalt,
        ]
    }

    /// Whether this kind ends a lane.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Accepted | Self::Rejected | Self::Inconclusive | Self::HumanHalt
        )
    }
}
