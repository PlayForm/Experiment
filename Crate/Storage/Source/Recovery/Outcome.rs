//! The honest outcome of a recovery decision.

/// What a recovery fold is allowed to report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    /// Projection matches the ledger; continue serving.
    Continue,
    /// State was preserved but excluded from authority.
    Quarantine,
    /// The result cannot be decided; uncertainty is preserved.
    Inconclusive,
    /// A non-canonical service is down; the ledger path still works.
    Degraded,
}

impl Outcome {
    /// Whether this outcome may produce authoritative evidence.
    #[must_use]
    pub fn authoritative(&self) -> crate::permission::Permission {
        match self {
            Self::Continue => crate::permission::Permission::Allow,
            Self::Quarantine | Self::Inconclusive | Self::Degraded => {
                crate::permission::Permission::Deny
            }
        }
    }

    /// The declared outcome name.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Continue => "continue",
            Self::Quarantine => "quarantine",
            Self::Inconclusive => "inconclusive",
            Self::Degraded => "degraded",
        }
    }
}
