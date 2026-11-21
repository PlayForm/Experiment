//! The provenance of a candidate patch.
//! Grounded in `ARCHITECTURE.md:106-109` (actor id, previous state, logical
//! sequence, request idempotency key) and `18-emergent-evolution.md:61-67` (a
//! candidate inherits a verified tree, declared features and explicit
//! provenance, never a sibling's hidden conversation).

/// Immutable provenance attached to every candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Origin {
    /// Pinned parent commit.
    pub parent_commit: String,
    /// Opaque local worker id.
    pub worker_local_id: String,
    /// Idempotency key for the submission.
    pub request_key: String,
}

impl Origin {
    /// Record the provenance of a staged candidate.
    #[must_use]
    pub fn of(parent_commit: &str, worker_local_id: &str, request_key: &str) -> Self {
        Self {
            parent_commit: parent_commit.to_owned(),
            worker_local_id: worker_local_id.to_owned(),
            request_key: request_key.to_owned(),
        }
    }

    /// Whether provenance is complete for the audit ledger.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        [self.parent_commit.as_str(), self.worker_local_id.as_str(), self.request_key.as_str()]
            .iter()
            .all(|value| !value.is_empty())
    }
}
