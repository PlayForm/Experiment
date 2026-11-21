//! A typed description of one bounded loop launch.
//! Grounded in `26-acceptance-and-build-plan.md:34-58` (the first vertical
//! slice runs as a bounded sequence of trusted stages) and the Integration
/// crate's `prun loop` dispatch. This value never starts a process.

use super::args::Args;

/// The launch plan for one bounded campaign.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Run {
    /// Parsed launch arguments.
    pub args: Args,
    /// The bounded playbook budget in force.
    pub budget: crate::r#loop::budget::Budget,
    /// The frozen generation this run works within.
    pub generation: crate::r#loop::generation::Generation,
}

impl Run {
    /// Prepare a run against a frozen generation.
    #[must_use]
    pub fn of(args: Args, generation: crate::r#loop::generation::Generation) -> Self {
        Self {
            args,
            budget: crate::r#loop::budget::Budget::playbook(),
            generation,
        }
    }

    /// Whether the run may start: the context is frozen and args are clean.
    #[must_use]
    pub fn is_admissible(&self) -> bool {
        self.generation.is_frozen() && self.args.is_clean()
    }

    /// The command token that owns this launch.
    #[must_use]
    pub const fn command(&self) -> &'static str {
        "loop"
    }

    /// Whether an explicit hour override is inside the playbook budget.
    #[must_use]
    pub fn within_budget(&self) -> bool {
        self.args
            .hours_override
            .map_or(true, |hours| hours <= self.budget.hours)
    }
}
