//! Repository consent, fail-closed.
//!
//! Grounded in `docs/13-repository-agnostic.md:32-38` (the worker sees only an
//! allowlisted file tree; dependencies are pre-fetched by trusted ingestion;
//! repository A cannot read repository B without a sanctioned knowledge card),
//! `docs/17-v61-constitution.md:40-43` (possession is a capability; a valid
//! signature never implies entitlement) and `docs/25` F05/F26 (licensing
//! ambiguities fail closed pending authorized human review; no retroactive
//! grant). Withheld is the default so an unmentioned repository is never
//! silently admitted.

/// Consent state for one repository trust domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Consent {
    /// The repository explicitly consented to admission.
    Granted,
    /// No consent was recorded; the default.
    Withheld,
    /// Previously granted consent was revoked.
    Revoked,
}

impl Consent {
    /// Whether an artifact may be staged into this repository.
    #[must_use]
    pub const fn admits(self) -> crate::Permission {
        match self {
            Self::Granted => crate::Permission::Allow,
            Self::Withheld | Self::Revoked => crate::Permission::Deny,
        }
    }

    /// Revocation restricts future access; it does not erase past knowledge
    /// (`docs/25-failure-analysis.md` F14).
    #[must_use]
    pub const fn restrict_future(self) -> crate::Permission {
        match self {
            Self::Revoked => crate::Permission::Deny,
            Self::Granted | Self::Withheld => crate::Permission::Allow,
        }
    }
}

impl Default for Consent {
    fn default() -> Self {
        Self::Withheld
    }
}
