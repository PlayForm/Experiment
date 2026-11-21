//! Recoverable diagnostics for degraded v2 campaign inputs.
//!
//! The design is strict about correctness but tolerant when modelling a draft
//! document: unknown keys, version drift, empty axes, collisions, dangling
//! references and values outside a documented range are recorded as warnings
//! and the model continues with a documented default (`README.md:53-58`,
//! `REQUIREMENTS.md:101-103` LAW-017, `schemas/campaign.v2.schema.json:7,24-26`).
//! A warning never grants a generated artifact authority over measurement or
//! policy.

use std::fmt;

use serde_json::Value;

/// A recoverable diagnostic that does not stop the model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Warning {
    /// A JSON object carried a key outside the schema's `properties`.
    Unknown { domain: &'static str, key: String },
    /// A value was clamped into its documented range.
    Range { domain: &'static str, key: String },
    /// A schema version differed from the supported version.
    Version { found: String, expected: String },
    /// A reference named an identifier with no matching definition.
    Reference { from: String, to: String },
    /// A colliding identifier was folded by keeping the first occurrence.
    Collision { domain: &'static str, id: String },
    /// A runtime value was unavailable and a safe substitute was used.
    Unavailable { source: String, substitute: String },
    /// A required collection was empty; the model continued with no entries.
    Empty { domain: &'static str },
    /// A field is a future-facing design contract, not implemented behaviour.
    Designed { field: &'static str, contract: &'static str },
    /// A v1 field was NOT reinterpreted into v2 without an explicit mapping.
    Unmapped { from: &'static str, to: &'static str },
    /// A budget or permission bound was exceeded by an observed value.
    Bound { domain: &'static str, key: String },
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
            Self::Designed { field, contract } => {
                format!("warning: {field} is a design contract ({contract}); not executable")
            }
            Self::Unmapped { from, to } => {
                format!("warning: {from} was not reinterpreted as {to}; explicit migration required")
            }
            Self::Bound { domain, key } => {
                format!("warning: {domain}.{key} exceeded its bound; campaign must pause")
            }
        }
    }

    /// Collect one `Unknown` warning for each key outside `known`.
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

    /// Fold repeated identifiers, keeping the first occurrence
    /// (`REQUIREMENTS.md:352-357` - the human name is not the identity).
    pub fn collisions<T>(
        items: &[T],
        domain: &'static str,
        field: impl Fn(&T) -> &str,
        warnings: &mut Vec<Self>,
    ) {
        let counts = items.iter().fold(
            std::collections::BTreeMap::<&str, usize>::new(),
            |mut tally, item| {
                *tally.entry(field(item)).or_default() += 1;
                tally
            },
        );
        counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .for_each(|(key, _)| {
                warnings.push(Self::Collision {
                    domain,
                    id: key.to_owned(),
                })
            });
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}
