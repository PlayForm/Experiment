//! Recoverable, operator-only findings for every deviation from the happy path.
//!
//! A missing file, malformed config, colliding identifier, stale epoch,
//! unreachable policy service or attempted scratch escape produces a clear
//! operator warning and a safe default. Nothing here aborts the process, and
//! no warning text is ever encoded into a worker status as an enforcement
//! reason (`docs/07:28-30`, `docs/09:15`, `docs/08:52-56`).

use std::fmt;

/// One recoverable deviation from an expected structural condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Warning {
    /// A required input was not supplied; the operation is deferred.
    Missing { domain: &'static str },
    /// A field could not be interpreted; a safe default is retained.
    Malformed { domain: &'static str, detail: String },
    /// Two definitions claimed the same namespace slot; nothing is inserted.
    Collision { domain: &'static str, id: String },
    /// A revision conflict was detected; activation is refused.
    VersionConflict { domain: &'static str, found: u64 },
    /// A digest is not a 64-character lowercase-hex string.
    DigestShape { field: &'static str, detail: String },
    /// An epoch, grant, pseudonym or lease fell outside its validity window.
    Expired { subject: String },
    /// A repeated nonce or idempotency key was seen; replay is refused.
    Replayed { key: String },
    /// Confidential detail was withheld from an untrusted view.
    Withheld { subject: &'static str },
    /// A trusted policy, broker or audit service was unreachable; deny.
    Unreachable { service: &'static str },
    /// A value would escape the worker scratch boundary; treat as absent.
    Escape { path: String },
}

impl Warning {
    /// Render one diagnostics line for an authorized operator only.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Missing { domain } => format!("warning: {domain} absent; operation deferred"),
            Self::Malformed { domain, detail } => {
                format!("warning: malformed {domain} ({detail}); safe default retained")
            }
            Self::Collision { domain, id } => {
                format!("warning: colliding {domain} identifier {id}; refused")
            }
            Self::VersionConflict { domain, found } => {
                format!("warning: {domain} revision {found} conflicts; activation refused")
            }
            Self::DigestShape { field, detail } => {
                format!("warning: digest field {field} ill-formed ({detail}); reference only")
            }
            Self::Expired { subject } => format!("warning: {subject} expired; authority refused"),
            Self::Replayed { key } => {
                format!("warning: nonce {key} already seen; replay refused")
            }
            Self::Withheld { subject } => {
                format!("warning: {subject} withheld from untrusted view")
            }
            Self::Unreachable { service } => {
                format!("warning: {service} unreachable; operation stays denied")
            }
            Self::Escape { path } => {
                format!("warning: path {path} escapes scratch; treated as absent")
            }
        }
    }

    /// The only status a worker may observe, regardless of the finding.
    #[must_use]
    pub const fn worker_status(&self) -> &'static str {
        crate::NEUTRAL_STATUS
    }

    /// No finding is ever safe to relay as a worker-visible reason.
    #[must_use]
    pub const fn worker_safe(&self) -> bool {
        false
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}
