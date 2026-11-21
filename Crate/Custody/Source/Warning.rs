//! Recoverable custody, license and worker findings.
//!
//! A missing grant, malformed digest, stale fence, colliding identifier or
//! unreachable policy service is a warning the caller can act on. Nothing here
//! aborts the process, and no warning text is ever sent to an offline worker
//! as an enforcement reason (`docs/17:9`, `docs/22:47-69`).

use std::fmt;

/// One recoverable deviation from the happy path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Warning {
    /// A required input was not supplied.
    Missing { domain: &'static str },
    /// A field could not be interpreted; the safe default is retained.
    Malformed { domain: &'static str, detail: String },
    /// Two definitions claimed the same namespace slot.
    Collision { domain: &'static str, id: String },
    /// A digest is not a 64-character lowercase-hex string.
    DigestShape { field: &'static str, detail: String },
    /// A lease, epoch, grant or rule fell outside its validity window.
    Expired { subject: String },
    /// A stale fencing token tried to commit a staged transfer.
    Fenced { lease: String, expected: u64, found: u64 },
    /// A repeated idempotency key was seen; replay is refused.
    Replay { key: String },
    /// Bytes were delivered but not verified; nothing was committed.
    Unverified { subject: String },
    /// Confidential detail was withheld from an untrusted view.
    Withheld { subject: &'static str },
    /// A trusted policy or audit service was unreachable; deny by default.
    Unreachable { service: &'static str },
    /// A value carried a reason code into a worker-visible type.
    DisclosureLeak { domain: &'static str },
    /// The artifact or tool would escape the worker scratch boundary.
    Escape { path: String },
}

impl Warning {
    /// Render one diagnostics-line for an authorized operator only.
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
            Self::DigestShape { field, detail } => {
                format!("warning: digest field {field} ill-formed ({detail}); reference only")
            }
            Self::Expired { subject } => format!("warning: {subject} expired; authority refused"),
            Self::Fenced {
                lease,
                expected,
                found,
            } => format!(
                "warning: lease {lease} fenced, expected {expected}, found {found}; quarantined"
            ),
            Self::Replay { key } => {
                format!("warning: idempotency key {key} already recorded; replay refused")
            }
            Self::Unverified { subject } => {
                format!("warning: {subject} unverified; not committed")
            }
            Self::Withheld { subject } => {
                format!("warning: {subject} withheld from untrusted view")
            }
            Self::Unreachable { service } => {
                format!("warning: {service} unreachable; operation stays denied")
            }
            Self::DisclosureLeak { domain } => {
                format!("warning: {domain} would disclose a reason to a worker; refused")
            }
            Self::Escape { path } => {
                format!("warning: path {path} escapes scratch; treated as absent")
            }
        }
    }

    /// Whether this finding is safe to surface through a generic worker status.
    /// Only a bounded local status qualifies; all other findings are audit-only.
    #[must_use]
    pub fn worker_safe(&self) -> bool {
        matches!(self, Self::Escape { .. } | Self::Missing { .. })
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}
