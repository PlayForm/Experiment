//! A rigorous win classification with confidence and practical threshold.
//!
//! Grounded in `RESEARCH_GATE.md:86-88` (multiple-testing and winner's-curse
//! risk; confirmatory thresholds) and `REQUIREMENTS_V3.md` EXP-009 (decisions
//! use predeclared practical gains, statistical uncertainty, regression
//! guardrails and multiple-attempt context). A win is computed by the
//! deterministic evaluator, never self-declared by a patcher or director.
//!
//! Exception declared: `Confirmation` is the typed form of this one field and
//! has no independent lifecycle outside `Win`.

/// Whether a sealed-holdout confirmation has passed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Confirmation {
    /// No confirmation has passed yet.
    Pending,
    /// A fixed-checkpoint confirmation passed.
    Confirmed,
}

/// The detailed evidence behind a `Verdict::Win`.
#[derive(Clone, Debug, PartialEq)]
pub struct Win {
    /// Observed speedup ratio, greater than 1.0 means faster.
    pub ratio: f64,
    /// Lower bound of the confidence interval.
    pub ci_low: f64,
    /// Upper bound of the confidence interval.
    pub ci_high: f64,
    /// The predeclared practical gain that was met.
    pub practical_gain: f64,
    /// Number of attempts already spent on this lineage.
    pub attempts: u32,
    /// Whether a sealed-holdout confirmation already passed.
    pub confirmation: Confirmation,
}

impl Win {
    /// Whether this win clears both the practical threshold and the
    /// confidence lower bound.
    #[must_use]
    pub fn clears_threshold(&self) -> bool {
        self.ratio >= 1.0 + self.practical_gain && self.ci_low > 1.0
    }

    /// Whether an independent confirmation is still required before promotion.
    #[must_use]
    pub const fn needs_confirmation(&self) -> bool {
        matches!(self.confirmation, Confirmation::Pending)
    }

    /// A compact hint suitable for a scoped feedback capsule.
    #[must_use]
    pub fn hint(&self) -> String {
        format!(
            "ratio={:.3} ci=[{:.3},{:.3}] attempts={}",
            self.ratio, self.ci_low, self.ci_high, self.attempts
        )
    }
}
