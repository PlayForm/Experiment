//! Monotonic journal and key epochs with rollback rejection.
//!
//! `docs/06:32-37` requires a signed transition and forbids erasing journal
//! entries on rollback; `docs/01:46-48` rejects one universal seed and records
//! explicit epochs. An epoch is a typed transition, not a clock read.

use crate::verdict::Verdict;

/// One recorded epoch with its chain head.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Epoch {
    /// Monotonic epoch number.
    pub number: u64,
    /// Journal head at this epoch.
    pub head: String,
    /// Stage reached by the transition into this epoch.
    pub stage: crate::stage::Stage,
}

impl Epoch {
    /// Record an epoch from trusted replay fields.
    #[must_use]
    pub fn of(number: u64, head: &str, stage: crate::stage::Stage) -> Self {
        Self {
            number,
            head: head.to_owned(),
            stage,
        }
    }

    /// A successor must advance the number and carry a digest head.
    #[must_use]
    pub fn advances(&self, next: &Self) -> Verdict {
        Verdict::of(next.number > self.number)
            .and(Verdict::digest(&next.head))
            .and(Verdict::of(next.stage != crate::stage::Stage::Aborted))
    }

    /// A rollback is rejected unless a new authorized transition is prepared.
    #[must_use]
    pub fn rejects_rollback(&self, candidate: &Self) -> Verdict {
        Verdict::of(candidate.number >= self.number)
    }

    /// Whether this epoch is an activated authoritative state.
    #[must_use]
    pub const fn active(&self) -> bool {
        self.stage.active()
    }
}
