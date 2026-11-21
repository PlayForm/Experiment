//! Versioned statistical protocol for predeclared comparisons.
//!
//! Grounded in `RESEARCH_GATE.md:80-88` (predeclared comparisons, practical
//! thresholds, paired ABBA/BAAB order, warmup/reset policy, sample counts,
//! drift screening, multiple-testing and confirmatory thresholds) and
//! `REQUIREMENTS_V3.md` EXP-009. Every verdict is scoped to a protocol version;
//! changing the protocol creates a new cohort, never a silent reinterpretation.

use crate::queue::workload::Order;

/// Whether a sealed-holdout confirmation is required before promotion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Confirmation {
    /// A confirmation at a fixed checkpoint is required.
    Required,
    /// No confirmation is required for this comparison class.
    NotRequired,
}

/// A versioned statistical protocol.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Protocol {
    /// Version of this protocol, recorded on every verdict.
    pub version: String,
    /// Predeclared practical gain, as a ratio above 1.0.
    pub practical_gain: f64,
    /// Predeclared confidence level.
    pub confidence_level: f64,
    /// Minimum number of paired samples.
    pub samples_min: u32,
    /// Paired order policy.
    pub order: Order,
    /// Multiple-testing correction applied across attempts.
    pub correction: String,
    /// Whether confirmation is required.
    pub confirmation: Confirmation,
}

impl Protocol {
    /// Whether a candidate clears the practical and confidence thresholds.
    #[must_use]
    pub fn is_win(&self, ratio: f64, ci_low: f64) -> bool {
        ratio >= 1.0 + self.practical_gain && ci_low > 1.0
    }

    /// Whether the regression guardrail fires.
    #[must_use]
    pub fn is_regression(&self, ci_high: f64) -> bool {
        ci_high < 1.0
    }

    /// Whether this protocol requires a sealed-holdout confirmation.
    #[must_use]
    pub const fn needs_confirmation(&self) -> bool {
        matches!(self.confirmation, Confirmation::Required)
    }
}
