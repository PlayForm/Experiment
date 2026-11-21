//! Fatal decode faults for the trust model.
//!
//! A fault is reserved for input that cannot be interpreted at all: unreadable
//! bytes, malformed JSON, a missing required field, a wrong key length, or two
//! definitions that cannot be folded. Version drift, unknown keys, colliding
//! identifiers and expired credentials are recoverable and become `Warning`
//! values instead. Nothing here aborts the process (`security/README.md:11-15`,
//! `crypto.rs:81-84`, defensive non-happy-path rule).

use std::fmt;

/// A non-recoverable decoding fault.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fault {
    /// Bytes could not be decoded as JSON at the named document path.
    JSON { path: &'static str, detail: String },
    /// A required named input was absent and no documented default applies.
    Missing { key: &'static str },
    /// A fixed-length field had a different length than the protocol requires.
    Length { field: &'static str, expected: usize, found: usize },
    /// Two definitions claimed the same identifier and neither may be folded.
    Conflicting { domain: &'static str, detail: String },
    /// A named artifact could not be read from its backing store.
    Unreadable { digest: String },
}

impl Fault {
    /// Wrap a `serde_json` error without leaking it as a panic.
    #[must_use]
    pub fn json(path: &'static str, error: &serde_json::Error) -> Self {
        Self::JSON {
            path,
            detail: error.to_string(),
        }
    }

    /// Render one human-readable diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::JSON { path, detail } => {
                format!("fault: malformed JSON at {path}: {detail}")
            }
            Self::Missing { key } => format!("fault: required input {key} is missing"),
            Self::Length { field, expected, found } => {
                format!("fault: {field} length expected {expected}, found {found}")
            }
            Self::Conflicting { domain, detail } => {
                format!("fault: conflicting {domain}: {detail}")
            }
            Self::Unreadable { digest } => {
                format!("fault: artifact {digest} is unreadable")
            }
        }
    }
}

impl fmt::Display for Fault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}

impl std::error::Error for Fault {}
