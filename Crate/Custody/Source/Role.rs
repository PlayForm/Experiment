//! The role a local file plays in the offline worker interface.
//!
//! Only the task manifest and the submission manifest are JSON schemas; every
//! other path is an ordinary local file whose absence reveals no policy
//! reason (`docs/28:8-22`).

/// The governed role of a worker-visible path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    /// Inbound task manifest at `/workspace/task.json`.
    Task,
    /// Read-only pinned source tree.
    Repository,
    /// Optional approved local references.
    References,
    /// Optional prior own feedback.
    Feedback,
    /// Outbound submission manifest.
    Submission,
    /// One staged artifact under scratch.
    Artifact,
    /// An unrecognized path, denied by default.
    Unknown,
}

impl Role {
    /// Whether the role belongs to the two-schema wire protocol.
    #[must_use]
    pub fn schema_bearing(&self) -> bool {
        match self {
            Self::Task | Self::Submission => true,
            Self::Repository | Self::References | Self::Feedback | Self::Artifact | Self::Unknown => {
                false
            }
        }
    }
}
