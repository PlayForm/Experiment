//! One reviewed migration: a step plus its additive-only finding.
//!
//! A [`Migration`] is the value a trusted applier may inspect before it opens
//! a database. This crate never applies it; a destructive step yields a
//! warning-level finding and is refused (`sql/v61_migration.sql:1-2`,
//! `docs/08-data-and-storage.md:70-80`).

use super::finding::Finding;
use super::step::Step;
use super::version::Version;
use crate::permission::Permission;

/// A versioned step together with the review that gates it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Migration {
    step: Step,
    finding: Finding,
}

impl Migration {
    /// Review a step without executing any part of it.
    #[must_use]
    pub fn of(step: Step) -> Self {
        let finding = step.review();
        Self { step, finding }
    }

    /// The step under review.
    #[must_use]
    pub fn step(&self) -> &Step {
        &self.step
    }

    /// The additive-only finding.
    #[must_use]
    pub fn finding(&self) -> &Finding {
        &self.finding
    }

    /// The version this migration belongs to.
    #[must_use]
    pub fn version(&self) -> Version {
        self.step.version()
    }

    /// Whether a trusted applier may take this step.
    #[must_use]
    pub fn eligible(&self) -> Permission {
        self.finding.verdict()
    }

    /// A runbook line describing the decision and its research source.
    #[must_use]
    pub fn runbook(&self) -> String {
        format!(
            "{}: {} from {}; {}",
            self.version().label(),
            self.finding().line(),
            self.step().source(),
            self.eligible().label()
        )
    }
}
