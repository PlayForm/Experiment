//! Named, guarded transitions between aggregate states.
//!
//! Grounded in `docs/06-state-machines.md:9-77`: every edge has exactly one
//! authorized guard (owner command, signed trusted event, timed software rule
//! or deterministic validator) and free-form text is never a transition input.
//! The full published edge set is enumerated here as typed values; the
//! universal transition transaction (`:78-90`) is enforced by `Machine`.

/// One authorized state transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Transition {
    /// `draft -> validated` by policy-service.
    CampaignValidated,
    /// `validated -> active` on owner command.
    CampaignActivated,
    /// `active -> paused` on owner command.
    CampaignPaused,
    /// `paused -> active` on owner command.
    CampaignResumed,
    /// `active -> draining` on owner command.
    CampaignDrained,
    /// `draining -> completed` when all jobs terminal.
    CampaignCompleted,
    /// `active -> failed` on critical fault.
    CampaignFailed,
    /// `paused|active -> cancelled` on owner command.
    CampaignCancelled,
    /// `proposed -> admitted` by scope-validator.
    MutationAdmitted,
    /// `proposed -> rejected` by scope-validator.
    MutationRejected,
    /// `admitted -> built` by builder attestation.
    MutationBuilt,
    /// `built -> oracle_failed` by oracle attestation.
    OracleFailed,
    /// `built -> oracle_passed` by oracle attestation.
    OraclePassed,
    /// `oracle_passed -> measured` by runner attestation.
    MutationMeasured,
    /// `measured -> evaluated` by evaluator attestation.
    MutationEvaluated,
    /// `evaluated -> eligible` by policy-engine.
    MutationEligible,
    /// `eligible -> promoted` by release-gate.
    MutationPromoted,
    /// `oracle_failed|rejected -> archived` by coordinator.
    MutationArchived,
    /// `idle -> assigned` by scheduler lease.
    WorkerAssigned,
    /// `assigned -> running` by supervisor.
    WorkerStarted,
    /// `running -> staging` by supervisor.
    WorkerStaging,
    /// `staging -> idle` on artifact seal.
    WorkerSealed,
    /// `running|assigned -> failed` by supervisor.
    WorkerFailed,
    /// `failed -> quarantined` by host-verifier.
    WorkerQuarantined,
    /// `quarantined -> terminated` by operator.
    WorkerTerminated,
    /// `queued -> leased` by exclusive scheduler.
    BenchmarkLeased,
    /// `leased -> running` by runner start.
    BenchmarkStarted,
    /// `running -> sealed` by runner evidence.
    BenchmarkSealed,
    /// `running -> contaminated` by runner integrity.
    BenchmarkContaminated,
    /// `sealed -> evaluated` by evaluator attestation.
    BenchmarkEvaluated,
    /// `contaminated -> queued` with a new attempt id.
    BenchmarkRetried,
    /// `draft -> private-reviewed` by review-authority.
    PaperReviewed,
    /// `private-reviewed -> sanitization-pending` by publisher.
    SanitizationPending,
    /// `sanitization-pending -> sanitized` by sanitizer.
    PaperSanitized,
    /// `sanitized -> internal-released` by publication-authority.
    InternalReleased,
    /// `internal-released -> public-released` on owner policy.
    PublicReleased,
    /// `draft|sanitization-pending -> rejected` by authority or sanitizer.
    PaperRejected,
}

impl Transition {
    /// Source and destination state names, in that order.
    #[must_use]
    pub const fn states(self) -> [&'static str; 2] {
        match self {
            Self::CampaignValidated => ["draft", "validated"],
            Self::CampaignActivated => ["validated", "active"],
            Self::CampaignPaused => ["active", "paused"],
            Self::CampaignResumed => ["paused", "active"],
            Self::CampaignDrained => ["active", "draining"],
            Self::CampaignCompleted => ["draining", "completed"],
            Self::CampaignFailed => ["active", "failed"],
            Self::CampaignCancelled => ["paused", "cancelled"],
            Self::MutationAdmitted => ["proposed", "admitted"],
            Self::MutationRejected => ["proposed", "rejected"],
            Self::MutationBuilt => ["admitted", "built"],
            Self::OracleFailed => ["built", "oracle_failed"],
            Self::OraclePassed => ["built", "oracle_passed"],
            Self::MutationMeasured => ["oracle_passed", "measured"],
            Self::MutationEvaluated => ["measured", "evaluated"],
            Self::MutationEligible => ["evaluated", "eligible"],
            Self::MutationPromoted => ["eligible", "promoted"],
            Self::MutationArchived => ["oracle_failed", "archived"],
            Self::WorkerAssigned => ["idle", "assigned"],
            Self::WorkerStarted => ["assigned", "running"],
            Self::WorkerStaging => ["running", "staging"],
            Self::WorkerSealed => ["staging", "idle"],
            Self::WorkerFailed => ["running", "failed"],
            Self::WorkerQuarantined => ["failed", "quarantined"],
            Self::WorkerTerminated => ["quarantined", "terminated"],
            Self::BenchmarkLeased => ["queued", "leased"],
            Self::BenchmarkStarted => ["leased", "running"],
            Self::BenchmarkSealed => ["running", "sealed"],
            Self::BenchmarkContaminated => ["running", "contaminated"],
            Self::BenchmarkEvaluated => ["sealed", "evaluated"],
            Self::BenchmarkRetried => ["contaminated", "queued"],
            Self::PaperReviewed => ["draft", "private-reviewed"],
            Self::SanitizationPending => ["private-reviewed", "sanitization-pending"],
            Self::PaperSanitized => ["sanitization-pending", "sanitized"],
            Self::InternalReleased => ["sanitized", "internal-released"],
            Self::PublicReleased => ["internal-released", "public-released"],
            Self::PaperRejected => ["draft", "rejected"],
        }
    }

    /// The sole authorized guard for this edge (`docs/06:9-77`).
    #[must_use]
    pub const fn guard(self) -> &'static str {
        match self {
            Self::CampaignValidated | Self::MutationRejected | Self::PaperSanitized => {
                "deterministic-validator"
            }
            Self::CampaignActivated
            | Self::CampaignPaused
            | Self::CampaignResumed
            | Self::CampaignDrained
            | Self::CampaignCancelled
            | Self::WorkerTerminated => "owner-command",
            Self::CampaignCompleted => "coordinator-on-all-jobs-terminal",
            Self::CampaignFailed => "coordinator-on-critical-fault",
            Self::MutationAdmitted => "scope-validator",
            Self::MutationBuilt => "builder-attested",
            Self::OracleFailed | Self::OraclePassed => "oracle-attested",
            Self::MutationMeasured => "runner-attested",
            Self::MutationEvaluated | Self::BenchmarkEvaluated => "evaluator-attested",
            Self::MutationEligible => "policy-engine",
            Self::MutationPromoted => "release-gate",
            Self::MutationArchived => "coordinator",
            Self::WorkerAssigned | Self::BenchmarkLeased => "scheduler-exclusive",
            Self::WorkerStarted | Self::WorkerStaging | Self::WorkerFailed => "supervisor",
            Self::WorkerSealed => "supervisor-on-artifact-seal",
            Self::WorkerQuarantined => "host-verifier",
            Self::BenchmarkStarted => "runner-start",
            Self::BenchmarkSealed => "runner-evidence",
            Self::BenchmarkContaminated => "runner-integrity",
            Self::BenchmarkRetried => "coordinator-retry-new-attempt",
            Self::PaperReviewed => "review-authority",
            Self::SanitizationPending => "publisher",
            Self::InternalReleased | Self::PaperRejected => "publication-authority",
            Self::PublicReleased => "publication-authority-on-owner-policy",
        }
    }

    /// Whether the named pair is this exact transition.
    #[must_use]
    pub const fn matches(self, from: &str, to: &str) -> bool {
        let states = self.states();
        states[0] == from && states[1] == to
    }
}
