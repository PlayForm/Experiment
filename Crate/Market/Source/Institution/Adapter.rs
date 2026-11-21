//! Repository-adapter admission at the institution boundary.
//!
//! Grounded in `OPERATIONS.md:38` (`unknown_repo_adapter` is a typed failure
//! mode), `REPOSITORIES.md:5,77-83` (define a versioned adapter, statically
//! check it, verify owner consent, then freeze a baseline) and `SYSTEM.md:25-30`
//! (no adapter means exploratory profiling only, never an "improvement" claim).
//! This is an admission decision value, not a parsed manifest.

use crate::Warning;

/// Admission state of a repository adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Adapter {
    /// A validated, consented adapter is enrolled.
    Known,
    /// No adapter is registered for this repository.
    Unknown,
    /// The adapter was revoked or its consent withdrew.
    Revoked,
}

impl Adapter {
    /// All admission states.
    pub const ALL: [Self; 3] = [Self::Known, Self::Unknown, Self::Revoked];

    /// Admit only a known, consented adapter; degrade with a warning otherwise.
    pub fn admit(self, repo_id: &str) -> Result<(), Warning> {
        match self {
            Self::Known => Ok(()),
            Self::Unknown => Err(Warning::Unreachable {
                service: "unknown_repo_adapter",
            }),
            Self::Revoked => Err(Warning::Denied {
                subject: repo_id,
                reason: "adapter consent revoked",
            }),
        }
    }

    /// Without a reliable oracle the lab may profile but cannot label a result
    /// an improvement (`REPOSITORIES.md:82-83`).
    #[must_use]
    pub const fn may_claim_improvement(self) -> crate::Permission {
        match self {
            Self::Known => crate::Permission::Allow,
            Self::Unknown | Self::Revoked => crate::Permission::Deny,
        }
    }
}
