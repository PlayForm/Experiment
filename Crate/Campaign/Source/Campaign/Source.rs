//! The `project` object: which source the campaign is about.
//!
//! Grounded in `campaign.v2.json:3-9` and `schemas/campaign.v2.schema.json:27-59`.
//! The `source_ref` names a Git ref of the SUT; mapping it to the v1 plan's
//! `refs` axis is an explicit migration, never a silent reinterpretation
//! (`README.md:7-9`, LAW-017).
//!
//! Exception declared: `Compatibility` is declared beside `Source` because it
//! is the typed form of this one field and has no independent lifecycle.

use serde::{Deserialize, Serialize};

/// The declared equivalence tier of a candidate
/// (`REQUIREMENTS.md:190-192` REQ-ARC-06).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Compatibility {
    /// Exact externally observable behaviour; the only tier eligible for
    /// automatic runtime selection under standard compatibility mode.
    #[serde(rename = "exact")]
    Exact,
    /// Behaviour preserved under stated conditions.
    #[serde(rename = "conditional")]
    Conditional,
    /// Experimental behaviour; requires opt-in and independent confirmation.
    #[serde(rename = "experimental")]
    Experimental,
}

impl Compatibility {
    /// The JSON enum token (`schemas/campaign.v2.schema.json:48`).
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Conditional => "conditional",
            Self::Experimental => "experimental",
        }
    }

    /// Whether the tier may be selected automatically.
    #[must_use]
    pub const fn automatic(self) -> bool {
        matches!(self, Self::Exact)
    }
}

/// The project descriptor of a v2 campaign document.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "name")]
    pub name: String,
    /// Repository URL (`campaign.v2.json:5`).
    #[serde(rename = "repository")]
    pub repository: String,
    #[serde(rename = "source_ref")]
    pub source_ref: String,
    #[serde(rename = "target_binary")]
    pub target_binary: String,
    pub compatibility: Compatibility,
}

impl Source {
    /// Recognised keys (`schemas/campaign.v2.schema.json:51-57`).
    pub const KEYS: [&'static str; 5] = [
        "name",
        "repository",
        "source_ref",
        "target_binary",
        "compatibility",
    ];

    /// A human-readable identity that is explicitly not the experiment ID
    /// (`REQUIREMENTS.md:352-354`).
    #[must_use]
    pub fn human_name(&self) -> String {
        format!("{}@{}", self.name, self.source_ref)
    }
}
