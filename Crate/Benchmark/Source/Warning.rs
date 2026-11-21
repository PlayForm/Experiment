//! Non-fatal diagnostics captured while modelling degraded inputs.
//!
//! The v0 controller aborts on several unusual conditions (prun_lab.py:191,
//! :196, :608). This model degrades instead: it records a `Warning`, applies a
//! documented default, and continues so a caller can still inspect the plan.

use std::fmt;

use serde_json::Value;

/// A recoverable diagnostic that does not stop the model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Warning {
    /// A JSON object carried a key the model does not recognise.
    Unknown { domain: &'static str, key: String },
    /// A numeric value fell outside its documented range and was clamped.
    Range { domain: &'static str, key: String },
    /// A schema version differed from the supported one.
    Version { found: String, expected: String },
    /// A reference named an identifier with no matching definition.
    Reference { from: String, to: String },
    /// A colliding identifier was folded by keeping the first occurrence.
    Collision { domain: &'static str, id: String },
    /// A runtime value was unavailable and a safe substitute was used.
    Unavailable { source: String, substitute: String },
    /// A required collection was empty; the model continued with no entries.
    Empty { domain: &'static str },
}

impl Warning {
    /// Render one human-readable diagnostic line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Unknown { domain, key } => {
                format!("warning: unknown key {key} in {domain}; ignored")
            }
            Self::Range { domain, key } => {
                format!("warning: {domain}.{key} was out of range; clamped")
            }
            Self::Version { found, expected } => {
                format!("warning: version {found} differs from {expected}; using defaults")
            }
            Self::Reference { from, to } => {
                format!("warning: {from} references undefined {to}; entry skipped")
            }
            Self::Collision { domain, id } => {
                format!("warning: duplicate {id} in {domain}; first kept")
            }
            Self::Unavailable { source, substitute } => {
                format!("warning: {source} unavailable; using {substitute}")
            }
            Self::Empty { domain } => format!("warning: {domain} is empty"),
        }
    }

    /// Collect one `Unknown` warning for every key outside `known`.
    #[must_use]
    pub fn unknown(value: &Value, known: &[&str], domain: &'static str) -> Vec<Self> {
        value
            .as_object()
            .into_iter()
            .flat_map(|object| object.keys())
            .filter(|key| !known.contains(&key.as_str()))
            .map(|key| Self::Unknown {
                domain,
                key: key.clone(),
            })
            .collect()
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}
