//! The independent trusted evaluator behind a study outcome.
//!
//! Model self-reported scores never count, and research papers cannot become
//! experimental evidence without signed independent measurement
//! (`docs/27:32-40`, `docs/18:53-59`, `docs/25:9`).

use super::outcome::Outcome;
use crate::permission::Permission;
use crate::switch::Switch;
use crate::verdict::Verdict;

/// A trusted judge with sealed evaluation inputs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Judge {
    /// `evaluator_id` of the trusted evaluator service.
    pub evaluator_id: String,
    /// `policy_digest` that fixed the acceptance criteria.
    pub policy_digest: String,
    /// Whether this judge is independent of the proposing track.
    pub independent: Switch,
    /// Whether evaluation used sealed inputs.
    pub sealed: Switch,
    /// Model-lineage digest correlated with the proposing track.
    pub lineage_digest: String,
}

impl Judge {
    /// Whether this judge may issue an authoritative verdict.
    #[must_use]
    pub fn authorized(&self) -> Permission {
        Permission::filled(&self.evaluator_id)
            .and(Permission::digest(&self.policy_digest))
            .and(self.independent.permit())
            .and(self.sealed.permit())
    }

    /// Whether the judge may promote the outcome's candidate.
    #[must_use]
    pub fn promotion(&self, outcome: &Outcome) -> Permission {
        self.authorized()
            .and(outcome.evidenced())
            .and(outcome.verdict.promotes())
            .and(Permission::filled(&outcome.evaluator_id))
    }

    /// A same-family model lineage flags correlation, never independence.
    #[must_use]
    pub fn correlated(&self, other: &Self) -> bool {
        self.lineage_digest == other.lineage_digest
    }

    /// The verdict recorded when evidence is withheld or inconclusive.
    /// An unpromoted Pass is downgraded to Inconclusive, never reported as a
    /// win the judge did not authorize.
    #[must_use]
    pub fn honest_verdict(&self, outcome: &Outcome) -> Verdict {
        match (self.promotion(outcome), outcome.verdict) {
            (Permission::Allow, _) => Verdict::Pass,
            (Permission::Deny, Verdict::Pass) => Verdict::Inconclusive,
            (Permission::Deny, other) => other,
        }
    }
}
