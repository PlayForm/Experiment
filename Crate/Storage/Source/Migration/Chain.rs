//! The ordered, additive-only migration chain.
//!
//! A chain owns ordering and review; it never opens a database or executes a
//! statement. Applying an accepted step is trusted software outside this crate.

use super::finding::Finding;
use super::guard::Guard;
use super::step::Step;
use super::version::Version;
use crate::permission::Permission;
use crate::warning::Warning;

/// An ordered set of versioned migration steps.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chain {
    steps: Vec<Step>,
}

impl Chain {
    /// Declare a chain from its steps.
    #[must_use]
    pub fn of(steps: Vec<Step>) -> Self {
        Self { steps }
    }

    /// The canonical chain built from the typed table catalog.
    #[must_use]
    pub fn canonical() -> Self {
        Self::of(Version::all().into_iter().map(Step::catalogued).collect())
    }

    /// The steps in declared order.
    #[must_use]
    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    /// Steps sorted by version, without mutating the declared chain.
    #[must_use]
    pub fn ordered(&self) -> Vec<&Step> {
        let mut ordered = self.steps.iter().collect::<Vec<_>>();
        ordered.sort_by_key(|step| step.version());
        ordered
    }

    /// Find a step by version.
    #[must_use]
    pub fn find(&self, version: Version) -> Option<&Step> {
        self.steps.iter().find(|step| step.version() == version)
    }

    /// Review every step in apply order.
    #[must_use]
    pub fn validate(&self) -> Vec<Finding> {
        Guard::chain(self)
    }

    /// Whether the whole chain is additive.
    #[must_use]
    pub fn admits(&self) -> Permission {
        Guard::admits(self)
    }

    /// Steps at or after a given version.
    #[must_use]
    pub fn from(&self, version: Version) -> Vec<&Step> {
        self.ordered()
            .into_iter()
            .filter(|step| step.version() >= version)
            .collect()
    }

    /// Report duplicate versions or missing predecessors without panicking.
    #[must_use]
    pub fn integrity(&self) -> Vec<Warning> {
        let duplicates = self
            .ordered()
            .windows(2)
            .filter_map(|pair| match pair {
                [left, right] if left.version() == right.version() => {
                    Some(Warning::Collision {
                        domain: "migration version",
                        id: left.version().label().to_owned(),
                    })
                }
                _ => None,
            });
        let missing = self.steps.iter().filter_map(|step| {
            step.version().predecessor().and_then(|needed| {
                self.find(needed).is_none().then(|| Warning::MissingTable {
                    name: format!("step {}", needed.label()),
                })
            })
        });
        duplicates.chain(missing).collect()
    }
}
