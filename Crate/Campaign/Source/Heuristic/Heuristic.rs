//! The `heuristics` block: a versioned feature family, not a hardcoded switch.
//!
//! Grounded in `campaign.v2.json:125-137`, the schema at
//! `schemas/campaign.v2.schema.json:406-470` and REQ-HEU-01..16
//! (`REQUIREMENTS.md:194-242`). The analyzer time budget is the field that
//! makes planner overhead measurable (`:237-239`, `:832-858`). Every boolean
//! is a typed `Permission`; no rule is evaluated and no command is probed.

use serde::{Deserialize, Serialize};

use crate::{
    agent::Permission,
    heuristic::{Fallback, Mode, Overhead},
    Warning,
};

/// The heuristics configuration of a v2 campaign.
// `f64` is not `Eq`, so comparison is deliberately partial.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Heuristic {
    #[serde(rename = "feature_family")]
    pub feature_family: String,
    pub modes: Vec<Mode>,
    #[serde(rename = "default_mode")]
    pub default_mode: Mode,
    #[serde(rename = "analyzer_time_budget_us")]
    pub analyzer_time_budget_us: u64,
    #[serde(rename = "filesystem_sampling_enabled", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub filesystem_sampling: Permission,
    #[serde(rename = "allow_active_command_probing", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub command_probing: Permission,
    #[serde(rename = "confidence_threshold")]
    pub confidence_threshold: f64,
    #[serde(rename = "fallback_strategy")]
    pub fallback_strategy: String,
    #[serde(rename = "persist_raw_commands", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub persist_raw_commands: Permission,
    #[serde(rename = "policy_artifact")]
    pub policy_artifact: Option<String>,
    #[serde(rename = "explain_plan_opt_in", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub explain_plan: Permission,
    /// Measured overhead fields, attached by a future trusted measurement step.
    #[serde(skip)]
    pub overhead: Option<Overhead>,
}

impl Heuristic {
    /// Recognised keys (`schemas/campaign.v2.schema.json:462-468`).
    pub const KEYS: [&'static str; 11] = [
        "feature_family",
        "modes",
        "default_mode",
        "analyzer_time_budget_us",
        "filesystem_sampling_enabled",
        "allow_active_command_probing",
        "confidence_threshold",
        "fallback_strategy",
        "persist_raw_commands",
        "policy_artifact",
        "explain_plan_opt_in",
    ];

    /// Warn on unsafe or malformed planner configuration instead of assuming
    /// a safe runtime will be written.
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        match self.command_probing {
            Permission::Allow => warnings.push(Warning::Bound {
                domain: "heuristics",
                key: "allow_active_command_probing violates LAW-003".to_owned(),
            }),
            Permission::Deny => {}
        }
        match self.confidence_threshold.is_finite() && (0.0..=1.0).contains(&self.confidence_threshold) {
            true => {}
            false => warnings.push(Warning::Range {
                domain: "heuristics",
                key: "confidence_threshold".to_owned(),
            }),
        }
        match self.modes.is_empty() {
            true => warnings.push(Warning::Empty {
                domain: "heuristics.modes",
            }),
            false => {}
        }
        match self.feature_family.is_empty() {
            true => warnings.push(Warning::Empty {
                domain: "heuristics.feature_family",
            }),
            false => {}
        }
    }

    /// The typed fallback named by the document (`campaign.v2.json:133`).
    #[must_use]
    pub fn fallback(&self) -> Fallback {
        Fallback::from_key(&self.fallback_strategy)
    }

    /// Whether a mode is enabled in this campaign.
    #[must_use]
    pub fn mode_enabled(&self, mode: Mode) -> bool {
        self.modes.contains(&mode)
    }

    /// Whether a confidence value clears the declared threshold. A non-finite
    /// confidence is never sufficient (REQ-HEU-09).
    #[must_use]
    pub fn confidence_sufficient(&self, confidence: f64) -> bool {
        confidence.is_finite() && confidence >= self.confidence_threshold
    }

    /// The overhead record seeded from the configured analyzer budget.
    #[must_use]
    pub fn measured_overhead(&self) -> Overhead {
        self.overhead.clone().unwrap_or(Overhead {
            budget_us: self.analyzer_time_budget_us,
            ..Overhead::default()
        })
    }
}
