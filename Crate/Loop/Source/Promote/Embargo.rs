//! An embargo that blocks release without leaking its reason to a worker.
//! Grounded in `22-hidden-worker-and-correspondence.md:47-69` (no event saying
//! `embargo violated` or `trade denied` reaches a model),
//! `17-v61-constitution.md:45-57` (no enforcement diagnostics to workers) and
//! `26-acceptance-and-build-plan.md:51-53` (release denied under embargo).

/// An embargo with separate operator and worker views.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Embargo {
    /// Epoch at which the embargo lifts.
    pub until_epoch: u32,
    /// Full reason retained for the protected audit view.
    pub operator_reason: String,
    /// The single neutral token a worker ever sees.
    pub worker_reason: &'static str,
}

impl Embargo {
    /// Declare an embargo with an operator-only reason.
    #[must_use]
    pub fn until(until_epoch: u32, operator_reason: &str) -> Self {
        Self {
            until_epoch,
            operator_reason: operator_reason.to_owned(),
            worker_reason: "submission_received",
        }
    }

    /// Whether the embargo blocks a release at this epoch.
    #[must_use]
    pub const fn blocks(&self, epoch: u32) -> bool {
        epoch < self.until_epoch
    }

    /// The worker view never reveals the embargo reason.
    #[must_use]
    pub const fn worker_view(&self) -> &'static str {
        self.worker_reason
    }
}
