//! The two and only two schemas visible to an offline worker.
//!
//! `v61-offline-task.schema.json` and `v61-offline-submission.schema.json` are
//! the complete wire protocol; there is no model-to-model envelope and no
//! internal event catalog in the worker view
//! (`docs/28:1-38`; the two `$id` values in the schema files).

/// One of the exactly two worker-visible JSON schemas.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Schema {
    /// Inbound `/workspace/task.json`.
    OfflineTask,
    /// Outbound `/workspace/out/submission.json`.
    OfflineSubmission,
}

impl Schema {
    /// The two schemas in protocol order.
    #[must_use]
    pub fn all() -> [Self; 2] {
        [Self::OfflineTask, Self::OfflineSubmission]
    }

    /// The schema `$id` recorded in the research corpus.
    #[must_use]
    pub fn id(&self) -> &'static str {
        match self {
            Self::OfflineTask => crate::TASK_SCHEMA_ID,
            Self::OfflineSubmission => crate::SUBMISSION_SCHEMA_ID,
        }
    }

    /// The local path at which the schema applies.
    #[must_use]
    pub fn path(&self) -> &'static str {
        match self {
            Self::OfflineTask => crate::TASK_PATH,
            Self::OfflineSubmission => crate::SUBMISSION_PATH,
        }
    }

    /// Whether this schema is inbound to the worker.
    #[must_use]
    pub fn inbound(&self) -> bool {
        match self {
            Self::OfflineTask => true,
            Self::OfflineSubmission => false,
        }
    }
}
