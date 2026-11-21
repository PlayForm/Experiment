//! Fatal decode faults for the v3 experiment model.
//!
//! A fault is reserved for input that cannot be interpreted at all: unreadable
//! bytes, malformed JSON, or a required named input with no default. Schema
//! drift, unknown keys, colliding identifiers and out-of-range values are
//! recoverable and become `Warning` values instead. Nothing here aborts the
//! process (`REQUIREMENTS_V3.md` INV-005, `STORAGE.md:2`, defensive/non-happy-path
//! rule).

use std::fmt;

/// A non-recoverable decoding fault.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fault {
    /// Bytes could not be decoded as JSON at the named document path.
    JSON { path: &'static str, detail: String },
    /// A required named input was absent and no documented default applies.
    Missing { key: &'static str },
    /// Two definitions claimed the same identifier and neither may be folded.
    Conflicting { domain: &'static str, detail: String },
    /// A named artifact could not be read from the backing CAS.
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
