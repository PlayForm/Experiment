//! One versioned migration step: an ordered bundle of additive changes.

use super::additive::Additive;
use super::change::Change;
use super::finding::Finding;
use super::guard::Guard;
use super::version::Version;
use crate::schema::table::Table;

/// One step of the additive chain, grounded in one research file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
    version: Version,
    changes: Vec<Change>,
    classification: Additive,
    source: &'static str,
}

impl Step {
    /// Declare a step and classify every change it contains.
    #[must_use]
    pub fn of(version: Version, changes: Vec<Change>, source: &'static str) -> Self {
        let classification = changes
            .iter()
            .fold(Additive::Additive, |state, change| {
                state.and(change.additive())
            });
        Self {
            version,
            changes,
            classification,
            source,
        }
    }

    /// Build the step for a version from the typed table catalog.
    #[must_use]
    pub fn catalogued(version: Version) -> Self {
        let tables = crate::schema::catalog::Catalog::at(version);
        let changes = tables.into_iter().map(Change::AddTable).collect();
        Self::of(version, changes, version.source())
    }

    /// The step version.
    #[must_use]
    pub fn version(&self) -> Version {
        self.version
    }

    /// The research file this step is grounded in.
    #[must_use]
    pub fn source(&self) -> &'static str {
        self.source
    }

    /// The changes in declared order.
    #[must_use]
    pub fn changes(&self) -> &[Change] {
        &self.changes
    }

    /// The overall additive classification.
    #[must_use]
    pub fn classification(&self) -> Additive {
        self.classification
    }

    /// The tables created by this step.
    #[must_use]
    pub fn tables(&self) -> Vec<&Table> {
        self.changes
            .iter()
            .filter_map(|change| match change {
                Change::AddTable(table) => Some(table),
                _ => None,
            })
            .collect()
    }

    /// Review the step; a destructive change yields a warning-level finding.
    #[must_use]
    pub fn review(&self) -> Finding {
        Guard::step(self)
    }
}
