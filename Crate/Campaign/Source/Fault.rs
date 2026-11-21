//! Fatal decode faults for the v2 campaign model.
//!
//! A fault is reserved for input that cannot be interpreted at all: unreadable
//! bytes or malformed JSON. Schema drift, unknown keys, colliding identifiers
//! and out-of-range values are recoverable and become `Warning` values instead
//! (the same split the v0 model uses, `Benchmark/Source/Fault.rs`,
//! `Benchmark/Source/Warning.rs`). Nothing here aborts the process.

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
        }
    }
}

impl fmt::Display for Fault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}

impl std::error::Error for Fault {}
