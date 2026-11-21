//! The oracle verdict, kept separate from the statistical verdict.
//! Grounded in `00-system-constitution.md:49-55` (the evidence hierarchy puts
//! trusted oracle above trusted build but below independent evaluation) and
//! `17-v61-constitution.md:9` (never fabricate a passing verification).

/// The trusted oracle outcome for one candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Verdict {
    /// Behaviour preserved on the frozen contracts.
    Pass,
    /// A bounded semantic failure category; full details stay trusted.
    SemanticFailure { category: String },
}

impl Verdict {
    /// Whether the candidate passed the oracle.
    #[must_use]
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass)
    }

    /// Whether any detail may be shown to the worker.
    #[must_use]
    pub fn worker_visible(&self) -> bool {
        match self {
            Self::Pass => true,
            Self::SemanticFailure { .. } => false,
        }
    }

    /// Stable key for the attestation record.
    #[must_use]
    pub fn key(&self) -> &'static str {
        match self {
            Self::Pass => "oracle_passed",
            Self::SemanticFailure { .. } => "oracle_failed",
        }
    }
}
