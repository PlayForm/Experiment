//! Seals worker output into an immutable, untrusted receipt.
//! Grounded in `22-hidden-worker-and-correspondence.md:47-69` (a submission
//! receives an opaque receipt meaning only `submission_received`; no
//! acceptance, correctness or benchmark claim is revealed) and
//! `17-v61-constitution.md:7` (only trusted software operates the lab).

use crate::warning::Warning;

use super::{artifact::Artifact, patch::Patch, worker::Worker};

/// An immutable receipt for a sealed, untrusted submission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Staging {
    /// Neutral receipt token handed back to the worker.
    pub receipt: String,
    /// Digest of the sealed output bundle.
    pub seal_digest: String,
    /// Warnings withheld from the worker and kept for the operator.
    pub warnings: Vec<Warning>,
}

impl Staging {
    /// Seal a worker submission without judging or executing it.
    #[must_use]
    pub fn seal(worker: &Worker, artifact: &Artifact, patch: &Patch) -> Self {
        let valid = artifact.is_admissible() && patch.within_scope() && worker.is_offline();
        let warnings = match valid {
            true => Vec::new(),
            false => vec![Warning::of("submission sealed with scope degradations")],
        };
        Self {
            receipt: "submission_received".to_owned(),
            seal_digest: patch.digest.clone(),
            warnings,
        }
    }

    /// The only status a worker may observe.
    #[must_use]
    pub const fn worker_status(&self) -> &'static str {
        "submission_received"
    }

    /// Whether the sealed bundle is admissible for the novelty gate.
    #[must_use]
    pub fn is_admissible(&self) -> bool {
        self.warnings.is_empty()
    }
}
