//! Severity levels used by the failure register.
//!
//! All thirty cases carry an explicit level; no case is silently downgraded to
//! a cosmetic notice (`docs/25:3-34`).

/// Incident priority assigned by trusted review.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Severity {
    /// A breach of isolation, rights or non-disclosure.
    Critical,
    /// A serious challenge to measurement or enforcement integrity.
    High,
    /// A bounded fairness, cost or liveness concern.
    Medium,
}

impl Severity {
    /// The wire spelling used in the failure register.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Critical => "Critical",
            Self::High => "High",
            Self::Medium => "Medium",
        }
    }

    /// Whether the incident must reach an operator immediately.
    #[must_use]
    pub fn escalate(&self) -> bool {
        match self {
            Self::Critical | Self::High => true,
            Self::Medium => false,
        }
    }
}
