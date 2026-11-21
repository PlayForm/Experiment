//! The planner's return value and its feasibility decision.
//!
//! Grounded in REQ-HEU-02 (`REQUIREMENTS.md:218-219`) and the cost model at
//! `:834-850`: feasibility is separate from cost, and a prediction is never
//! presented as a measured guarantee. A plan is data for a future executor;
//! it grants no artifact authority over measurement or policy.
//!
//! Exception declared: `Eligibility` is declared beside `Plan` because it is
//! the typed form of one field and has no independent lifecycle.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::heuristic::Fallback;

/// Feasibility of a plan (`REQUIREMENTS.md:837`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Eligibility {
    Yes,
    No,
    /// Unknown environmental values are never treated as equal (REQ-ID-09).
    Unknown,
}

impl Eligibility {
    /// Token used in explain-plan output.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Yes => "yes",
            Self::No => "no",
            Self::Unknown => "unknown",
        }
    }
}

/// A deterministic execution plan proposal.
// `f64` is not `Eq`, so comparison is deliberately partial.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    pub stages: Vec<String>,
    #[serde(rename = "worker_limits")]
    pub worker_limits: BTreeMap<String, u32>,
    #[serde(rename = "queue_limits")]
    pub queue_limits: BTreeMap<String, u32>,
    #[serde(rename = "ordering_policy")]
    pub ordering_policy: String,
    #[serde(rename = "output_policy")]
    pub output_policy: String,
    pub rationale: String,
    #[serde(rename = "predicted_cost")]
    pub predicted_cost: f64,
    pub eligibility: Eligibility,
    pub fallback: Fallback,
    #[serde(rename = "policy_version")]
    pub policy_version: String,
}

impl Plan {
    /// REQ-HEU-02's ten fields.
    pub const KEYS: [&'static str; 10] = [
        "stages",
        "worker_limits",
        "queue_limits",
        "ordering_policy",
        "output_policy",
        "rationale",
        "predicted_cost",
        "eligibility",
        "fallback",
        "policy_version",
    ];

    /// Whether the plan may be selected at all. Only `Yes` is eligible.
    #[must_use]
    pub const fn is_eligible(&self) -> bool {
        matches!(self.eligibility, Eligibility::Yes)
    }

    /// Explain-plan payload required by LAW-013 and REQ-HEU-11: model or rule
    /// version, inputs, confidence, selected policy, predicted benefit and
    /// fallback rationale. This is diagnostic text, never stdout noise.
    #[must_use]
    pub fn explanation(&self) -> String {
        format!(
            "policy={} stages={} predicted={} eligibility={} fallback={} rationale={}",
            self.policy_version,
            self.stages.join(","),
            self.predicted_cost,
            self.eligibility.key(),
            self.fallback.key(),
            self.rationale
        )
    }
}
