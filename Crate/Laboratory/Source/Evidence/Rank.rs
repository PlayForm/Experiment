//! The seven rungs of the evidence hierarchy.
//!
//! Grounded in `docs/00-system-constitution.md:49-55`:
//! `worker-opinion < supervisor-ingested-artifact < trusted-build <
//! trusted-oracle < trusted-runner < independent-statistical-evaluation <
//! release-gate`. A paper describes evidence, a stamp identifies a signer, and
//! neither can manufacture a passing test. Failure and inconclusive states are
//! preserved. The ordering is a typed value so no call site can silently
//! promote an opinion to a measurement.

/// One rung in the canonical evidence hierarchy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Rank {
    /// Untrusted authored opinion from an offline worker.
    WorkerOpinion,
    /// Artifact ingested by a trusted supervisor.
    SupervisorIngestedArtifact,
    /// Deterministic build of a pinned source tree.
    TrustedBuild,
    /// Frozen correctness oracle result.
    TrustedOracle,
    /// Signed trace-off runner measurement.
    TrustedRunner,
    /// Predeclared independent statistical evaluation.
    IndependentEvaluation,
    /// Authorized, reversible, cohort-scoped release decision.
    ReleaseGate,
}

impl Rank {
    /// Position in the hierarchy, lowest first.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::WorkerOpinion => 0,
            Self::SupervisorIngestedArtifact => 1,
            Self::TrustedBuild => 2,
            Self::TrustedOracle => 3,
            Self::TrustedRunner => 4,
            Self::IndependentEvaluation => 5,
            Self::ReleaseGate => 6,
        }
    }

    /// Whether evidence at this rank can authorize a performance claim.
    #[must_use]
    pub const fn authorizes_measurement(self) -> crate::Permission {
        match self {
            Self::TrustedRunner | Self::IndependentEvaluation | Self::ReleaseGate => {
                crate::Permission::Allow
            }
            Self::WorkerOpinion
            | Self::SupervisorIngestedArtifact
            | Self::TrustedBuild
            | Self::TrustedOracle => crate::Permission::Deny,
        }
    }

    /// The signer or producer of evidence at this rank.
    #[must_use]
    pub const fn producer(self) -> &'static str {
        match self {
            Self::WorkerOpinion => "offline-worker",
            Self::SupervisorIngestedArtifact => "supervisor",
            Self::TrustedBuild => "builder",
            Self::TrustedOracle => "oracle",
            Self::TrustedRunner => "runner",
            Self::IndependentEvaluation => "evaluator",
            Self::ReleaseGate => "release-gate",
        }
    }
}
