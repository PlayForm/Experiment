//! A typed, cohort-scoped promotion decision.
//! Grounded in `00-system-constitution.md:30-32` (no promotion on unsigned
//! source-of-truth measurements or an LLM-only verdict) and
//! `26-acceptance-and-build-plan.md:49-53` (release denied under embargo until
//! a separately approved change). Coupled to `experiment`'s authoritative
//! verdict and to the local signed policy.

use experiment::evaluate::verdict::Verdict;

use super::{cohort::Cohort, embargo::Embargo, policy::Policy};

/// A reversible, traceable release decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decision {
    /// Trusted evaluator classification.
    pub verdict: Verdict,
    /// Signed policy outcome.
    pub policy: Policy,
    /// Hardware and environment cohort this decision is scoped to.
    pub cohort: Cohort,
}

impl Decision {
    /// Assemble a decision from trusted inputs only.
    #[must_use]
    pub fn of(verdict: Verdict, cohort: Cohort) -> Self {
        Self {
            verdict,
            policy: Policy::decide(verdict),
            cohort,
        }
    }

    /// Whether the decision may be released for this cohort under an embargo.
    #[must_use]
    pub fn releasable(&self, embargo: &Embargo, epoch: u32) -> bool {
        match (
            self.verdict.promotable(),
            self.policy,
            embargo.blocks(epoch),
            self.cohort.is_scoped(),
        ) {
            (true, Policy::Allow, false, true) => true,
            _ => false,
        }
    }
}
