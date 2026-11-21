//! Severity used by assurance findings and threat consequences.
//!
//! Mirrors the operational presentation of failures in `docs/09:5-51` without
//! claiming that a failure was observed. Severity orders operator attention;
//! it never becomes a worker status.

/// One operator-facing severity level.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Severity {
    /// A trust boundary was breached; stop and review.
    Critical,
    /// A capability, journal or key invariant was violated.
    High,
    /// A recoverable deviation that must be retained as evidence.
    Medium,
    /// A bounded deviation with a safe default.
    Low,
    /// Informational context for a later audit.
    Informational,
}

impl Severity {
    /// Stable audit label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Informational => "informational",
        }
    }

    /// Whether this severity requires a human owner decision.
    #[must_use]
    pub const fn requires_review(&self) -> bool {
        matches!(self, Self::Critical | Self::High)
    }

    /// Whether the finding may be reported as a worker-visible pass. No
    /// severity is ever converted into a fabricated success (`docs/07:30`).
    #[must_use]
    pub const fn reports_pass() -> bool {
        false
    }
}
