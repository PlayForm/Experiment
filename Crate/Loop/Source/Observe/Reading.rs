//! A read-only projection of the SUT view, carrying degradations.
//! Grounded in `26-acceptance-and-build-plan.md:57-60` (the operator inspects
//! status through public, read-only projections) and the defensive rule that
//! missing or malformed input degrades to a warning rather than a panic.

use crate::warning::Warning;

use super::snapshot::Snapshot;

/// The result of observing the SUT without mutating it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reading {
    /// The frozen view that was read.
    pub snapshot: Snapshot,
    /// Warnings collected while building the view.
    pub warnings: Vec<Warning>,
}

impl Reading {
    /// Read a snapshot and carry its warnings forward.
    #[must_use]
    pub fn of(snapshot: &Snapshot) -> Self {
        Self {
            snapshot: snapshot.clone(),
            warnings: snapshot.warnings.clone(),
        }
    }

    /// Append a degradation discovered during observation.
    #[must_use]
    pub fn degraded(mut self, message: &str) -> Self {
        self.warnings.push(Warning::of(message));
        self
    }

    /// Whether the read produced an admissible view.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.snapshot.is_readable() && self.warnings.iter().all(|w| w.worker_safe())
    }
}
