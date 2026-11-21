//! Recoverable warnings for every non-happy path of the contract surface.
//!
//! A missing schema reference, a colliding operation ID, an unknown event kind,
//! a stale policy version or an unreadable document must produce a clear
//! warning and degrade, never a panic (`docs/25-failure-analysis.md:36-58`,
//! `docs/26-acceptance-and-build-plan.md:60-80`). Nothing here fabricates a
//! passing result.

use std::fmt;

/// A recoverable deviation from the contract happy path.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Warning {
    /// A declared version differs from the version this model understands.
    Drift { domain: &'static str, expected: u32, found: u32 },
    /// A key was present but is not part of the versioned model.
    Unknown { domain: &'static str, key: String },
    /// Two identifiers claimed the same namespace slot.
    Collision { domain: &'static str, id: String },
    /// A referenced artifact was absent; validation continues with a finding.
    MissingReference { domain: &'static str, id: String },
    /// A named document could not be read; the caller must degrade.
    Unreadable { path: String },
    /// A value could not be interpreted and a documented default applies.
    Malformed { domain: &'static str, detail: String },
    /// A numeric field fell outside its declared range.
    InvalidValue { field: &'static str, detail: String },
    /// A named service was unreachable; no request is retried blindly.
    Unreachable { service: &'static str },
    /// An action was refused by a pinned, versioned policy decision.
    Denied { subject: &'static str, reason: &'static str },
    /// An exact-duplicate identifier was rejected.
    Duplicate { domain: &'static str, id: String },
    /// A required namespace contained no entries at all.
    Empty { domain: &'static str },
}

impl Warning {
    /// Render one human-readable diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Drift { domain, expected, found } => {
                format!("warning: {domain} version drift, expected {expected}, found {found}")
            }
            Self::Unknown { domain, key } => {
                format!("warning: unknown {domain} key {key} ignored")
            }
            Self::Collision { domain, id } => {
                format!("warning: colliding {domain} identifier {id}")
            }
            Self::MissingReference { domain, id } => {
                format!("warning: {domain} reference {id} missing; recorded as finding")
            }
            Self::Unreadable { path } => {
                format!("warning: contract document {path} unreadable; degrading")
            }
            Self::Malformed { domain, detail } => {
                format!("warning: malformed {domain} input ({detail}); using safe default")
            }
            Self::InvalidValue { field, detail } => {
                format!("warning: invalid {field} ({detail}); value rejected")
            }
            Self::Unreachable { service } => {
                format!("warning: service {service} unreachable; degrading")
            }
            Self::Denied { subject, reason } => {
                format!("warning: {subject} denied ({reason})")
            }
            Self::Duplicate { domain, id } => {
                format!("warning: duplicate {domain} identifier {id}")
            }
            Self::Empty { domain } => format!("warning: {domain} namespace is empty"),
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}

impl std::error::Error for Warning {}
