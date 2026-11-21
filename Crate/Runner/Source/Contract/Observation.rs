//! One recorded observation against a characterization invariant.
//!
//! An observation is a typed expectation a benchmark may compare against;
//! it performs no assertion and never grants a generated artifact the right
//! to change a verdict. The worker model stays deterministic and external.

use crate::contract::{behavior::Behavior, invariant::Invariant};

/// A single expected result on one behaviour axis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Observation {
    behavior: Behavior,
    expected: String,
}

impl Observation {
    /// Record an expectation without evaluating it.
    #[must_use]
    pub fn build(behavior: Behavior, expected: String) -> Self {
        Self { behavior, expected }
    }

    /// Derive an observation directly from a grounded invariant.
    #[must_use]
    pub fn of(invariant: &Invariant) -> Self {
        Self::build(
            invariant.behavior(),
            format!("preserve: {} [{}]", invariant.rule(), invariant.source()),
        )
    }

    /// The axis observed.
    #[must_use]
    pub const fn behavior(&self) -> Behavior {
        self.behavior
    }

    /// The expected value, verbatim.
    #[must_use]
    pub fn expected(&self) -> &str {
        &self.expected
    }
}
