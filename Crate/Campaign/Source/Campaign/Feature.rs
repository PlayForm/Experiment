//! Feature groups: capabilities, selection and exclusivity.
//!
//! Grounded in `campaign.v2.json:101-124`, the schema at
//! `schemas/campaign.v2.schema.json:366-405` and `REQUIREMENTS.md:860-892`
//! (REQ-FEA-01..06). Cargo features are additive capabilities; choosing one
//! active walker or scheduler belongs to runtime selection or a distinct
//! build recipe, never to feature unification
//! (`REQUIREMENTS.md:860-869`).
//!
//! Exception declared: `Kind` and `Selection` are declared beside `Feature`
//! because each is the typed form of one field and has no independent
//! lifecycle.

use serde::{Deserialize, Deserializer, Serialize};

use crate::Warning;

/// The dimension a feature group belongs to
/// (`REQUIREMENTS.md:871-883`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Kind {
    /// Chosen runtime strategy or policy.
    #[serde(rename = "runtime_strategy")]
    Strategy,
    /// Capability compiled into the artifact.
    #[serde(rename = "cargo_capability")]
    Capability,
    /// Toolchain, profile, target, LTO, allocator, PGO/BOLT profile.
    #[serde(rename = "compiler_variant")]
    Variant,
    /// Timing and tracing instrumentation.
    #[serde(rename = "measurement")]
    Measurement,
}

impl Kind {
    /// The JSON enum token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Strategy => "runtime_strategy",
            Self::Capability => "cargo_capability",
            Self::Variant => "compiler_variant",
            Self::Measurement => "measurement",
        }
    }
}

/// Whether a group permits one active member only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Selection {
    /// Exactly one member may be active - declared outside Cargo.
    Exclusive,
    /// Several additive capabilities may be built in.
    Shared,
}

impl Selection {
    /// Deserialize `exclusive_active_selection` into a typed decision.
    pub fn from_bool<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        bool::deserialize(deserializer).map(|flag| match flag {
            true => Self::Exclusive,
            false => Self::Shared,
        })
    }

    /// The JSON boolean this came from.
    #[must_use]
    pub const fn as_bool(self) -> bool {
        matches!(self, Self::Exclusive)
    }
}

/// A declared feature group.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub kind: Kind,
    pub members: Vec<String>,
    #[serde(rename = "exclusive_active_selection", deserialize_with = "Selection::from_bool")]
    pub selection: Selection,
}

impl Feature {
    /// Recognised keys (`schemas/campaign.v2.schema.json:396-401`).
    pub const KEYS: [&'static str; 4] = ["id", "kind", "members", "exclusive_active_selection"];

    /// Warn on an empty member list or on a colliding member id
    /// (REQ-FEA-01, REQ-FEA-03, REQ-FEA-06).
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        match self.members.is_empty() {
            true => warnings.push(Warning::Empty {
                domain: "feature_groups.members",
            }),
            false => {}
        }
        crate::Warning::collisions(&self.members, "feature_groups.members", |item| item.as_str(), warnings);
    }

    /// Whether a requested strategy is a member of this group.
    #[must_use]
    pub fn contains(&self, strategy: &str) -> bool {
        self.members.iter().any(|member| member == strategy)
    }
}
