//! Fatal decode faults for the laboratory model.
//!
//! A fault is reserved for input that cannot be interpreted at all: unreadable
//! bytes, malformed JSON, a missing required field, a bad digest or two
//! definitions that cannot be folded. Version drift, unknown keys, colliding
//! identifiers, stale epochs and contaminated measurements are recoverable and
//! become [`Warning`](crate::warning::Warning) values instead. Nothing here
//! aborts the process; the corpus is a design artifact, not a deployed service
//! (`README.md:3-7`, `docs/16-completeness-and-open-questions.md:24-33`).

use std::fmt;

/// A non-recoverable decoding fault.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fault {
    /// Bytes could not be decoded as JSON at the named document path.
    JSON { path: &'static str, detail: String },
    /// A required named field was absent and no documented default applies.
    Missing { key: &'static str },
    /// A fixed-width field had a different width than the schema requires.
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
                format!("fault: {field} width expected {expected}, found {found}")
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
