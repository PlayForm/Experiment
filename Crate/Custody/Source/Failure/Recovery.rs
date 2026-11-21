//! Deterministic recovery controls for each failure case.
//!
//! A failure either denies, quarantines, fences a retry, reschedules on another
//! holder, asks an owner, or retains an inconclusive outcome. No branch ends in
//! a manufactured success (`docs/25:3-34`, `docs/19:87-93`).

use super::case::Case;
use super::cause::Cause;
use super::severity::Severity;

/// One trusted response to a failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Recovery {
    /// Refuse the operation and leave trusted state unchanged.
    Deny,
    /// Preserve staged bytes but exclude them from commit.
    Quarantine,
    /// Retry with the same idempotency token and a fresh fence.
    Fenced,
    /// Ask the scheduler to reschedule on another eligible holder.
    Reschedule,
    /// Stop and request an authorized human owner decision.
    Review,
    /// Raise an incident priority without notifying the worker.
    Escalate,
    /// Preserve an inconclusive result as an outcome.
    Retain,
    /// End a signed campaign lifecycle with no further proposals.
    Terminate,
}

impl Recovery {
    /// Choose a recovery control from a typed failure case.
    #[must_use]
    pub fn for_case(case: &Case) -> Self {
        match (case.severity, case.cause) {
            (Severity::Critical, Cause::Sandbox) => Self::Terminate,
            (Severity::Critical, Cause::Rights | Cause::Disclosure) => Self::Review,
            (Severity::Critical, _) => Self::Quarantine,
            (Severity::High, Cause::Capacity) => Self::Reschedule,
            (Severity::High, Cause::Idempotency | Cause::Epoch) => Self::Fenced,
            (Severity::High, Cause::Evidence | Cause::Oracle) => Self::Retain,
            (Severity::High, _) => Self::Deny,
            (Severity::Medium, Cause::Recursion) => Self::Terminate,
            (Severity::Medium, Cause::Convergence | Cause::Cost) => Self::Retain,
            (Severity::Medium, _) => Self::Deny,
        }
    }

    /// Whether the control may report a passing test to a worker.
    #[must_use]
    pub fn reports_pass(&self) -> bool {
        match self {
            Self::Deny
            | Self::Quarantine
            | Self::Fenced
            | Self::Reschedule
            | Self::Review
            | Self::Escalate
            | Self::Retain
            | Self::Terminate => false,
        }
    }
}
