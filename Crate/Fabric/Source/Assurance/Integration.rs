//! The verification levels and the two this package actually demonstrates.
//!
//! Mirrors `docs/09:52-66` and `VALIDATION_REPORT.md:3-20`: design review and
//! schema-level validation only. Cryptographic, deployment and audit levels
//! are named as future gates, never presented as attained.

use crate::status::Evidence;
use crate::verdict::Verdict;

/// The integration level claimed for a finding or the package itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Integration {
    /// Evidence level being claimed.
    pub level: Evidence,
    /// Digest of the artifact the claim refers to, when available.
    pub artifact_digest: String,
}

impl Integration {
    /// Record an integration claim from trusted fields.
    #[must_use]
    pub fn of(level: Evidence, digest: &str) -> Self {
        Self {
            level,
            artifact_digest: digest.to_owned(),
        }
    }

    /// Whether the claim is honest for this package.
    #[must_use]
    pub fn honest(&self) -> Verdict {
        self.level.attained().and(Verdict::of(
            self.artifact_digest.is_empty() || Verdict::digest(&self.artifact_digest) == Verdict::Allow,
        ))
    }

    /// No claim in this crate reaches security-audited status.
    #[must_use]
    pub const fn audited() -> bool {
        false
    }
}
