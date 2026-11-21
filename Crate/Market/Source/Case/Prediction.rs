//! A preregistered calibrated probability forecast.
//!
//! Grounded in `schemas/research-bid.v5.schema.json:108-126` (event,
//! probability in [0,1], `registered_before_outcome: const true`),
//! `MARKET.md:61-68` (Brier score `(p-y)^2`, 0 best; one input to future
//! admission priority, never a moral or scientific status label) and
//! `AGENTS.md:8` (forecast frozen before seeing outcomes). Probability is held
//! in integer thousandths; no binary floating-point is used for accounting.

use crate::{Digest, Permission, Warning};

/// A frozen binary-outcome prediction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Prediction {
    /// The event whose outcome is later measured (`schema:112-115`).
    pub event: String,
    /// Probability in thousandths, 0..=1000 (`schema:116-120`).
    pub probability_milli: u32,
    /// Whether registration preceded the outcome (`schema:121-123`).
    pub registered_before_outcome: Permission,
    /// Content commitment of the context capsule (`schema:52-55`).
    pub context_snapshot: Digest,
}

impl Prediction {
    /// Full certainty, in thousandths.
    pub const CERTAIN: u32 = 1_000;
    /// Scale of the Brier result: ten-thousandths of squared probability.
    pub const BRIER_SCALE: u32 = 100;

    /// Validate the probability and preregistration order.
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            self.probability_milli <= Self::CERTAIN,
            self.registered_before_outcome,
        ) {
            (true, Permission::Allow) => Ok(()),
            (true, Permission::Deny) => Err(Warning::Denied {
                subject: "prediction",
                reason: "not registered before outcome",
            }),
            (false, _) => Err(Warning::InvalidValue {
                field: "probability",
                detail: format!("{} milli > {}", self.probability_milli, Self::CERTAIN),
            }),
        }
    }

    /// Brier score `(p-y)^2` for an observed binary outcome (`MARKET.md:63-68`).
    /// Returns a warning instead of scoring a late or malformed forecast.
    pub fn brier(&self, outcome: bool) -> Result<u32, Warning> {
        self.validate()?;
        let observed = match outcome {
            true => Self::CERTAIN,
            false => 0,
        };
        let gap = self.probability_milli.abs_diff(observed);
        Ok(gap.saturating_mul(gap) / Self::BRIER_SCALE)
    }

    /// A score never rewards corrupting the evaluation (`MARKET.md:65-67`).
    #[must_use]
    pub const fn may_reward_evaluation_corruption() -> Permission {
        Permission::Deny
    }
}
