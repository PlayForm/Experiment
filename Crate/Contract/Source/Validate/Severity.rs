//! Severity of a structural finding.
//!
//! `docs/26-acceptance-and-build-plan.md:60-80` distinguishes contract
//! pass/fail checks from missing-reference warnings. A `Fault` marks a real
//! contract violation; a `Warning` marks a reference that could not be
//! resolved; `Info` records a count.

/// Severity of one finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Severity {
    /// Informational observation.
    Info,
    /// Recoverable missing reference or drift.
    Warning,
    /// Contract violation that fails the structural check.
    Fault,
}

impl Severity {
    /// Wire spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Fault => "fault",
        }
    }

    /// Numeric rank for ordering in a report.
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warning => 1,
            Self::Fault => 2,
        }
    }
}
