//! Campaign objectives and promotion thresholds.
//!
//! Grounded in `campaign.v2.json:16-23`, the schema at
//! `schemas/campaign.v2.schema.json:86-135`, `REQUIREMENTS.md:508-520`
//! (success criteria) and `:522-530` (Pareto objectives). A scalar score is a
//! view over the frontier, not a hidden policy; its weights stay in the data.
//!
//! Exception declared: `Primary` is declared beside `Objective` because it is
//! the typed form of this one field and has no independent lifecycle.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::Warning;

/// The primary optimization objective (`campaign.v2.json:17`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Primary {
    /// Full end-to-end paired wall time (REQ-BEN-01).
    #[serde(rename = "paired_wall_time")]
    WallTime,
    /// Throughput at fixed resources.
    #[serde(rename = "throughput")]
    Throughput,
    /// CPU time.
    #[serde(rename = "cpu_time")]
    CpuTime,
}

impl Primary {
    /// The JSON enum token (`schemas/campaign.v2.schema.json:91-95`).
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::WallTime => "paired_wall_time",
            Self::Throughput => "throughput",
            Self::CpuTime => "cpu_time",
        }
    }
}

/// Weights, guardrails and confidence for promotion.
// `f64` is not `Eq`; comparison is deliberately partial (house style).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Objective {
    pub primary: Primary,
    pub weights: BTreeMap<String, f64>,
    pub secondary: Vec<String>,
    #[serde(rename = "min_practical_speedup_fraction")]
    pub min_speedup: f64,
    #[serde(rename = "maximum_critical_regression_fraction")]
    pub max_regression: f64,
    #[serde(rename = "confidence_level")]
    pub confidence: f64,
}

impl Objective {
    /// Recognised keys (`schemas/campaign.v2.schema.json:127-133`).
    pub const KEYS: [&'static str; 6] = [
        "primary",
        "weights",
        "secondary",
        "min_practical_speedup_fraction",
        "maximum_critical_regression_fraction",
        "confidence_level",
    ];

    /// Warn on values outside the schema ranges rather than clamping silently.
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        let checks = [
            ("min_practical_speedup_fraction", self.min_speedup),
            ("maximum_critical_regression_fraction", self.max_regression),
        ];
        checks
            .into_iter()
            .filter(|(_, value)| !value.is_finite() || !(0.0..=1.0).contains(value))
            .for_each(|(key, _)| warnings.push(Warning::Range {
                domain: "objectives",
                key: key.to_owned(),
            }));
        match self.confidence.is_finite() && (0.0..1.0).contains(&self.confidence) {
            true => {}
            false => warnings.push(Warning::Range {
                domain: "objectives",
                key: "confidence_level".to_owned(),
            }),
        }
        match self.weights.is_empty() {
            true => warnings.push(Warning::Empty {
                domain: "objectives.weights",
            }),
            false => {}
        }
    }

    /// Whether an observed paired speedup clears the declared minimum
    /// practical effect (`REQUIREMENTS.md:511-513`). A non-finite observation
    /// is inconclusive, never a win.
    #[must_use]
    pub fn clears_speedup(&self, observed: f64) -> bool {
        observed.is_finite() && observed >= self.min_speedup
    }

    /// Whether a critical regression breaches the declared guardrail
    /// (`REQUIREMENTS.md:514-515`).
    #[must_use]
    pub fn breaches_regression(&self, observed: f64) -> bool {
        !observed.is_finite() || observed > self.max_regression
    }
}
