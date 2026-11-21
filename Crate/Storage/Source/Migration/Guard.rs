//! The additive-only guard over a migration step or chain.
//!
//! A guard never mutates schema. It classifies and reports; a destructive step
//! is a warning-level finding and is refused to any trusted applier that asks.

use super::additive::Additive;
use super::chain::Chain;
use super::change::Change;
use super::finding::Finding;
use super::step::Step;
use crate::permission::Permission;
use crate::warning::Warning;

/// The deterministic reviewer of migration steps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Guard;

impl Guard {
    /// Review one change.
    #[must_use]
    pub fn change(change: &Change) -> Permission {
        change.additive().permits()
    }

    /// Review one step, reporting the first destructive change.
    #[must_use]
    pub fn step(step: &Step) -> Finding {
        step.changes()
            .iter()
            .find(|change| change.additive() == Additive::Destructive)
            .map_or(Finding::Accept(step.version()), |change| {
                Finding::Reported(
                    step.version(),
                    Warning::NonAdditive {
                        version: step.version().label().to_owned(),
                        change: change.describe(),
                    },
                )
            })
    }

    /// Review every step in a chain, preserving apply order.
    #[must_use]
    pub fn chain(chain: &Chain) -> Vec<Finding> {
        chain.steps().iter().map(Self::step).collect()
    }

    /// Whether every step of a chain is additive.
    #[must_use]
    pub fn admits(chain: &Chain) -> Permission {
        Self::chain(chain)
            .iter()
            .fold(Permission::Allow, |state, finding| {
                state.and(finding.verdict())
            })
    }
}
