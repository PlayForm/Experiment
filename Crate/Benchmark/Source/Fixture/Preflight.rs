//! Mandatory preflight correctness gate before any timing.
//!
//! Grounded in `README.md:19-21,127-131` and `prun_lab.py::RUNNER_DRIVER`
//! (128-136): every matched directory must run `bench-signal` successfully and
//! the number of touched check files must equal `fixture.directories`.

use crate::Fault;

/// The outcome of evaluating the preflight count contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Preflight {
    Pass { expected: usize, found: usize },
    Fail { expected: usize, found: usize, exit: i32 },
}

impl Preflight {
    /// Evaluate the contract (prun_lab.py:134): zero exit AND exact count.
    #[must_use]
    pub fn evaluate(expected: usize, found: usize, exit: i32) -> Self {
        match (exit == 0, found == expected) {
            (true, true) => Self::Pass { expected, found },
            _ => Self::Fail { expected, found, exit },
        }
    }

    /// Convert a failure into the typed fault used by callers.
    #[must_use]
    pub fn fault(&self) -> Option<Fault> {
        match self {
            Self::Pass { .. } => None,
            Self::Fail { expected, found, .. } => Some(Fault::Count {
                found: *found,
                expected: *expected,
            }),
        }
    }
}
