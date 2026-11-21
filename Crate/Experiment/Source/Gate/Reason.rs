//! Discoverable failure and rejection reasons, including negative evidence.
//!
//! Grounded in `REQUIREMENTS_V3.md` INV-010 (rejected and inconclusive
//! experiments are discoverable with a reason), `RESEARCH_GATE.md:93-95`
//! (negative evidence preserved, known failures searchable), and
//! `ARCHITECTURE.md:102-104` (failure/retry event vocabulary). A reason never
//! grants a generated artifact authority; it only records why a deterministic
//! decision was made.

/// A typed reason attached to a gate, oracle, benchmark or review decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Reason {
    /// Patch touched a forbidden module or exceeded its scope.
    ScopeViolation,
    /// Exact experiment key already measured.
    ExactDuplicate,
    /// Close but non-identical prior; advisory.
    NearDuplicate,
    /// Mutually exclusive or interacting feature conflict.
    FeatureConflict,
    /// Not enough evidence to decide.
    InsufficientProof,
    /// Reproducible compile failed.
    BuildFailed,
    /// Trusted oracle reported a minimal violation category.
    OracleViolation,
    /// Benchmark run was contaminated.
    BenchContamination,
    /// Result was quarantined with raw data retained.
    ResultQuarantined,
    /// A budget cap was hit.
    BudgetExhausted,
    /// Performance regression guardrail fired.
    Regression,
    /// Statistical result was inconclusive.
    Inconclusive,
    /// Raw sample or manifest failed integrity checks.
    InvalidSample,
    /// Sealed holdout exposure was attempted.
    HoldoutLeak,
    /// An explicit human halt was issued.
    HumanHalt,
}

impl Reason {
    /// All reasons, in declaration order.
    pub const ALL: [Self; 15] = [
        Self::ScopeViolation,
        Self::ExactDuplicate,
        Self::NearDuplicate,
        Self::FeatureConflict,
        Self::InsufficientProof,
        Self::BuildFailed,
        Self::OracleViolation,
        Self::BenchContamination,
        Self::ResultQuarantined,
        Self::BudgetExhausted,
        Self::Regression,
        Self::Inconclusive,
        Self::InvalidSample,
        Self::HoldoutLeak,
        Self::HumanHalt,
    ];

    /// The exact reason token used in the ledger.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::ScopeViolation => "SCOPE_VIOLATION",
            Self::ExactDuplicate => "EXACT_DUPLICATE",
            Self::NearDuplicate => "NEAR_DUPLICATE",
            Self::FeatureConflict => "FEATURE_CONFLICT",
            Self::InsufficientProof => "INSUFFICIENT_PROOF",
            Self::BuildFailed => "BUILD_FAILED",
            Self::OracleViolation => "ORACLE_VIOLATION",
            Self::BenchContamination => "BENCH_CONTAMINATION",
            Self::ResultQuarantined => "RESULT_QUARANTINED",
            Self::BudgetExhausted => "BUDGET_EXHAUSTED",
            Self::Regression => "REGRESSION",
            Self::Inconclusive => "INCONCLUSIVE",
            Self::InvalidSample => "INVALID_SAMPLE",
            Self::HoldoutLeak => "HOLDOUT_LEAK",
            Self::HumanHalt => "HUMAN_HALT",
        }
    }

    /// Whether this reason is negative evidence that future agents may search.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        !matches!(self, Self::HumanHalt | Self::InsufficientProof)
    }
}
