//! The confidential sanitization scan contract.
//!
//! Grounded in `PUBLISHING.md:63-70`: check secrets, client paths, proprietary
//! source, repository licensing, personal data, code snippets, credentials,
//! agent instructions and prompt injection from fetched sources; no
//! auto-publishing for unknown external repositories; uncertain cases go to a
//! human; static scanning is helpful but not a guarantee. This is a typed scan
//! policy applied to findings supplied by a trusted caller; no document is read.

use crate::{paper::Sanitization, Warning};
use super::source::Source;

/// A policy-driven sensitive-material scan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sanitizer {
    /// The provenance class of the source (`PUBLISHING.md:65-66`).
    pub source: Source,
}

impl Sanitizer {
    /// Construct a sanitizer with the source trust fact.
    #[must_use]
    pub const fn of(source: Source) -> Self {
        Self { source }
    }

    /// Apply the scan to already-classified findings. An empty finding set over
    /// a known source passes; any finding or unknown source is denied with a
    /// warning and routed to human review (`PUBLISHING.md:68-69`).
    pub fn scan(&self, findings: &[&str]) -> Result<Sanitization, Warning> {
        match (self.source, findings.is_empty()) {
            (Source::Known, true) => Ok(Sanitization::Passed),
            (Source::External, _) => Err(Warning::Denied {
                subject: "sanitization",
                reason: "unknown external repository requires human review",
            }),
            (Source::Known, false) => Err(Warning::Denied {
                subject: "sanitization",
                reason: findings.first().copied().unwrap_or("sensitive material"),
            }),
        }
    }

    /// Static scanning is not a guarantee of secrecy (`PUBLISHING.md:68`).
    #[must_use]
    pub const fn is_a_secrecy_guarantee() -> crate::Permission {
        crate::Permission::Deny
    }
}
