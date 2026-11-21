//! Fatal decode faults for contract descriptors.
//!
//! A fault is reserved for input that cannot be interpreted at all: a path
//! without a leading slash, a `$ref` that names no document, a required key
//! that is absent. Version drift, unknown keys and missing *references* are
//! recoverable and become [`Warning`](crate::warning::Warning) values instead.
//! A fault is a typed value returned to the caller; it never aborts a process.

use std::fmt;

/// A non-recoverable decoding fault.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Fault {
    /// Bytes could not be decoded as JSON at the named document path.
    Json { path: &'static str, detail: String },
    /// A required named field was absent and no documented default applies.
    Missing { key: &'static str },
    /// A path or `$ref` had none of the recognized shapes.
    Shape { domain: &'static str, detail: String },
    /// Two definitions claimed the same identifier and neither may be folded.
    Conflicting { domain: &'static str, detail: String },
}

impl Fault {
    /// Wrap a `serde_json` error without leaking it as a panic.
    #[must_use]
    pub fn json(path: &'static str, error: &serde_json::Error) -> Self {
        Self::Json { path, detail: error.to_string() }
    }

    /// Render one human-readable diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Json { path, detail } => format!("fault: malformed JSON at {path}: {detail}"),
            Self::Missing { key } => format!("fault: required input {key} is missing"),
            Self::Shape { domain, detail } => format!("fault: invalid {domain} shape ({detail})"),
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
