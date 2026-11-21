//! Deterministic scoring values for allocation admission.
//!
//! Grounded in `MARKET.md:48-68` (expected useful knowledge per bounded cost,
//! an illustrative weighted priority divided by one plus normalized scarce
//! resource cost, and the strictly proper Brier score for precommitted binary
//! forecasts) and `rust/market-prototype/src/main.rs:120-125` (an exogenous
//! information-gain estimate divided by one plus normalized cost). Weights are
//! data supplied by a signed policy; no score is a truth or status label.

/// Fixed-point admission and calibration scores.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Score {
    /// Priority in millionths of the exogenous estimate over normalized cost.
    pub priority_micros: u128,
    /// Brier score in ten-thousandths from `crate::case::Prediction::brier`.
    pub brier_milli: u32,
    /// Externally estimated information gain in thousandths (`main.rs:68-71`).
    pub information_gain_milli: u32,
}

impl Score {
    /// Compute a priority value from an exogenous estimate and a normalized
    /// cost. The estimate must not be the bidder's own confidence
    /// (`MARKET.md:58-60`, `main.rs:68-71`).
    #[must_use]
    pub fn priority(information_gain_milli: u32, cost_micros: u128) -> u128 {
        let scale = u128::from(crate::FIXED_SCALE);
        let gain = u128::from(information_gain_milli) * scale / 1_000;
        gain * scale / (scale + cost_micros)
    }

    /// Construct a score value from typed inputs.
    #[must_use]
    pub const fn of(
        priority_micros: u128,
        brier_milli: u32,
        information_gain_milli: u32,
    ) -> Self {
        Self {
            priority_micros,
            brier_milli,
            information_gain_milli,
        }
    }

    /// Brier zero is best (`MARKET.md:63-65`).
    #[must_use]
    pub const fn is_perfect_calibration(&self) -> bool {
        self.brier_milli == 0
    }

    /// A calibration score is one admission input, never a moral status
    /// (`MARKET.md:65-66`).
    #[must_use]
    pub const fn is_scientific_status() -> crate::Permission {
        crate::Permission::Deny
    }
}
