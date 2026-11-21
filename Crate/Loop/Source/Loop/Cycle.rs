//! One cycle is one atomic mutation of the campaign state.
//! Grounded in `ARCHITECTURE.md:106-109` (every transition is an append-only
//! typed envelope) and `18-emergent-evolution.md:21-39` (one conceptual diff
//! per implementor; a compound change stays one candidate until decomposed).

use crate::r#loop::budget::Budget;

/// A single atomic transition in a lineage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cycle {
    /// Sequence of this cycle within its generation.
    pub sequence: u64,
    /// Parent cycle; absent for the frozen B0 root.
    pub parent: Option<u64>,
    /// Budget accumulated on entering this cycle.
    pub spent: Budget,
}

impl Cycle {
    /// The frozen root B0: no parent, no spend.
    #[must_use]
    pub fn genesis() -> Self {
        Self {
            sequence: 0,
            parent: None,
            spent: Budget::none(),
        }
    }

    /// The single child of this cycle: one atomic mutation.
    #[must_use]
    pub fn child(self, spend: Budget) -> Self {
        Self {
            sequence: self.sequence + 1,
            parent: Some(self.sequence),
            spent: self.spent.plus(spend),
        }
    }

    /// Whether this cycle and another are two different ancestors, which
    /// makes their union a new integration candidate with fresh evidence
    /// (`ARCHITECTURE.md:134-136`).
    #[must_use]
    pub fn is_integration(&self, other: &Self) -> bool {
        other.parent.is_some() && self.parent != other.parent
    }

    /// Whether this is the frozen root.
    #[must_use]
    pub const fn is_genesis(&self) -> bool {
        self.parent.is_none()
    }
}
