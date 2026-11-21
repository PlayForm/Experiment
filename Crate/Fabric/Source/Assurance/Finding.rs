//! One assurance finding with its honest level and severity.
//!
//! Mirrors the evidence-level discussion in `docs/09:52-66` and the operator
//! presentation of failures in `docs/09:15`. A finding records a statement and
//! the level at which it is held; it never converts a planned negative case
//! into a passing test, nor a severity into a worker-visible success.

use crate::status::Evidence;
use crate::severity::Severity;
use crate::verdict::Verdict;

/// One recorded assurance finding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Finding {
    /// Domain that produced the finding.
    pub domain: &'static str,
    /// Human-readable statement retained in full.
    pub statement: String,
    /// Severity ordering operator attention.
    pub severity: Severity,
    /// Evidence level actually held for the statement.
    pub evidence: Evidence,
}

impl Finding {
    /// Record a finding from trusted fields.
    #[must_use]
    pub fn of(domain: &'static str, statement: &str, severity: Severity, evidence: Evidence) -> Self {
        Self {
            domain,
            statement: statement.to_owned(),
            severity,
            evidence,
        }
    }

    /// A finding is honest only when it claims an attained level.
    #[must_use]
    pub fn honest(&self) -> Verdict {
        self.evidence.attained().and(Verdict::filled(&self.statement))
    }

    /// No finding may be reported as a worker-visible pass.
    #[must_use]
    pub const fn reports_pass() -> bool {
        Severity::reports_pass()
    }
}
