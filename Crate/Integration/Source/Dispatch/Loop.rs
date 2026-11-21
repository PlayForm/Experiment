//! The `loop` dispatch: the bounded autonomous re-benchmark cycle.
//!
//! Grounded in `Research/prun_laboratory_v6_1/docs/26-acceptance-and-build-plan.md`
//! (34-58) - a trusted coordinator runs paired measurements and the evaluator
//! preserves regressions and inconclusive outcomes - and in the v2 bounded
//! autonomous week (`Crate/Campaign`, `WEEK_HOURS`). The classification is
//! owned by `experiment::evaluate::verdict::Verdict`; a generated artifact can
//! never promote itself. This module holds no authority of its own.

/// One bounded loop decision: the evaluator's verdict and the week budget.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Loop {
    /// The authoritative, non-LLM classification (`Crate/Experiment`).
    pub verdict: experiment::evaluate::verdict::Verdict,
    /// The bounded campaign duration (`Crate/Campaign::WEEK_HOURS`).
    pub hours: u32,
}

impl Loop {
    /// Assemble a loop decision from a trusted verdict. No measurement or
    /// promotion happens here; the verdict's own rule decides promotability.
    #[must_use]
    pub const fn of(verdict: experiment::evaluate::verdict::Verdict) -> Self {
        Self {
            verdict,
            hours: campaign::WEEK_HOURS,
        }
    }

    /// Whether the trusted evaluator permits promotion to review.
    #[must_use]
    pub const fn promotable(self) -> bool {
        self.verdict.promotable()
    }

    /// Whether this outcome must stay discoverable as negative evidence.
    #[must_use]
    pub const fn negative(self) -> bool {
        self.verdict.is_negative()
    }
}
