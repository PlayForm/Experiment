//! Heuristic modes: an independently benchmarked feature family.
//!
//! Grounded in `REQUIREMENTS.md:194-214` (the nine ablation modes),
//! `campaign.v2.json:127` and `schemas/campaign.v2.schema.json:413-427`.
//! Cargo features are additive capabilities; only runtime strategy selection
//! picks one active mode (`REQUIREMENTS.md:860-869`).

use serde::{Deserialize, Serialize};

/// A heuristic feature-family mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    /// Reference execution; no analyzer or planner.
    #[serde(rename = "off")]
    Off,
    /// Parsed options and cheap process-visible facts only.
    #[serde(rename = "static")]
    Static,
    /// Adds bounded read-only filesystem metadata sampling.
    #[serde(rename = "fs-bounded")]
    Bounded,
    /// Adds low-overhead hardware and resource classification.
    #[serde(rename = "hardware")]
    Hardware,
    /// Deterministic versioned ruleset, cost family 1.
    #[serde(rename = "cost-v1")]
    CostOne,
    /// Tiny calibrated cost model with deterministic fallback, cost family 2.
    #[serde(rename = "cost-v2")]
    CostTwo,
    /// Computes a plan but executes the reference plan.
    #[serde(rename = "shadow")]
    Shadow,
    /// Explains decisions and records counters in diagnostic builds.
    #[serde(rename = "heuristic-telemetry")]
    Telemetry,
    /// Forces any eligible strategy for diagnosis and reproduction.
    #[serde(rename = "heuristic-override")]
    Override,
}

impl Mode {
    /// All documented modes in design order (`REQUIREMENTS.md:199-214`).
    pub const ALL: [Self; 9] = [
        Self::Off,
        Self::Static,
        Self::Bounded,
        Self::Hardware,
        Self::CostOne,
        Self::CostTwo,
        Self::Shadow,
        Self::Telemetry,
        Self::Override,
    ];

    /// The JSON enum or example token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Static => "static",
            Self::Bounded => "fs-bounded",
            Self::Hardware => "hardware",
            Self::CostOne => "cost-v1",
            Self::CostTwo => "cost-v2",
            Self::Shadow => "shadow",
            Self::Telemetry => "heuristic-telemetry",
            Self::Override => "heuristic-override",
        }
    }

    /// Recover a mode from its token; unknown tokens degrade to `None`.
    #[must_use]
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|mode| mode.key() == key)
    }

    /// Whether this mode may change the executed plan
    /// (`REQUIREMENTS.md:819-820`: adaptive selection starts in shadow mode).
    #[must_use]
    pub const fn plans_actively(self) -> bool {
        matches!(self, Self::Static | Self::Bounded | Self::Hardware | Self::CostOne | Self::CostTwo | Self::Override)
    }

    /// Whether the mode introduces analyzer work that must be timed
    /// (REQ-HEU-14, REQ-HEU-15).
    #[must_use]
    pub const fn has_analysis(self) -> bool {
        !matches!(self, Self::Off)
    }
}
