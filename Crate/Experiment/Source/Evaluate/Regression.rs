//! Performance regression guardrail; behavior preservation wins over speed.
//!
//! Grounded in `REQUIREMENTS_V3.md` INV-009 (runtime behavior-preservation
//! takes precedence over a speedup; failures exit without promotion) and
//! EXP-009 (decisions use predeclared regression guardrails). A regression is a
//! typed guardrail result, never a narrative judgment by an LLM.

use crate::gate::reason::Reason;

/// A regression guardrail result.
#[derive(Clone, Debug, PartialEq)]
pub struct Regression {
    /// Upper confidence bound of the speedup ratio.
    pub ci_high: f64,
    /// Predeclared tolerable lower bound, normally 1.0.
    pub guardrail: f64,
    /// Digest of the raw samples backing the estimate.
    pub raw_samples_sha256: String,
}

impl Regression {
    /// Whether the guardrail fired.
    #[must_use]
    pub fn triggered(&self) -> bool {
        self.ci_high < self.guardrail
    }

    /// The reason recorded when the guardrail fires.
    #[must_use]
    pub const fn reason() -> Reason {
        Reason::Regression
    }

    /// Whether this regression alone blocks promotion. It does.
    #[must_use]
    pub const fn blocks_promotion() -> bool {
        true
    }
}
