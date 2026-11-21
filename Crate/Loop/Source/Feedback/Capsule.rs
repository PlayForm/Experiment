//! A signed, restricted feedback capsule for one worker's own lineage.
//! Grounded in `ARCHITECTURE.md:88-93` (the patcher receives only a signed
//! structured capsule for its own lineage) and `AGENTS.md:4` (compact capsules
//! contain goal, verdict, budget and next actions, never sibling transcripts).

/// Private, scoped feedback returned to one worker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capsule {
    /// Effect estimate from the trusted evaluator.
    pub effect: String,
    /// Bounded error classes safe to reveal.
    pub error_classes: Vec<String>,
    /// Remaining budget for that worker's lane.
    pub budget_remaining: u32,
}

impl Capsule {
    /// Compose a capsule from trusted, scoped values only.
    #[must_use]
    pub fn of(effect: &str, error_classes: Vec<String>, budget_remaining: u32) -> Self {
        Self {
            effect: effect.to_owned(),
            error_classes,
            budget_remaining,
        }
    }

    /// A capsule never carries sealed holdout detail.
    #[must_use]
    pub fn has_no_holdout(&self) -> bool {
        self.error_classes
            .iter()
            .all(|reason| !reason.contains("holdout"))
    }

    /// Whether the capsule is complete enough to be useful.
    #[must_use]
    pub fn is_readable(&self) -> bool {
        !self.effect.is_empty() && self.has_no_holdout()
    }
}
