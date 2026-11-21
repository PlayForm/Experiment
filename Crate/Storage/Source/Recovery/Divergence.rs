//! A disagreement between a checkpoint and a replayed projection.

use super::outcome::Outcome;

/// One sequence-level divergence and the event where it was observed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Divergence {
    /// Sequence the checkpoint declared.
    pub expected: u64,
    /// Sequence the replay observed.
    pub observed: u64,
    /// Event identifier where the disagreement appeared.
    pub at_event: Option<String>,
}

impl Divergence {
    /// Declare a divergence.
    #[must_use]
    pub fn of(expected: u64, observed: u64, at_event: Option<String>) -> Self {
        Self {
            expected,
            observed,
            at_event,
        }
    }

    /// Classify the divergence; an irreconcilable state stays inconclusive.
    #[must_use]
    pub fn classify(&self) -> Outcome {
        match self.expected == self.observed {
            true => Outcome::Continue,
            false => Outcome::Inconclusive,
        }
    }

    /// Render the divergence as a warning.
    #[must_use]
    pub fn warning(&self) -> crate::warning::Warning {
        crate::warning::Warning::Divergence {
            expected: self.expected,
            observed: self.observed,
        }
    }
}
