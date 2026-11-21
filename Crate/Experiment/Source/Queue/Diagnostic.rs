//! Diagnostic runner jobs, kept separate from primary release timing.
//!
//! Grounded in `TELEMETRY.md:29-40,145-157`, `REQUIREMENTS_V3.md` TEL-004..008
//! and `playbooks/fanout-5.yaml:69-83` (persistent release runner in trace-off
//! mode; diagnostic jobs separate and clearly labeled non-primary). Diagnostic
//! stage timings never enter a release leaderboard.
//!
//! Exception declared: `Contribution` is the typed form of the primary/non-
//! primary distinction and has no independent lifecycle outside `Diagnostic`.

use crate::permission::Permission;

/// Whether a job may contribute to the primary comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Contribution {
    /// A primary, uninstrumented release trial.
    Primary,
    /// A labeled diagnostic execution.
    NonPrimary,
}

/// One diagnostic measurement job.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// Stable diagnostic job identifier.
    pub job_id: String,
    /// Instrumentation mode of the binary under test.
    pub instrumentation_mode: String,
    /// Whether this job may contribute to a primary verdict.
    pub contribution: Contribution,
    /// Digest of the resulting span or profile artifact.
    pub trace_sha256: String,
}

impl Diagnostic {
    /// Diagnostic jobs never contribute to primary timing truth.
    #[must_use]
    pub const fn may_contribute_to_release(&self) -> Permission {
        Permission::Deny
    }

    /// Whether this job is explicitly labeled non-primary for the dashboard.
    #[must_use]
    pub const fn is_labeled(&self) -> bool {
        matches!(self.contribution, Contribution::NonPrimary)
    }

    /// The comparison cohort label for this diagnostic mode.
    #[must_use]
    pub fn cohort(&self) -> String {
        format!("diagnostic/{}", self.instrumentation_mode)
    }
}
