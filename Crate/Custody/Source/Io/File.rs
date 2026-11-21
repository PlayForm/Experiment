//! A worker-visible local file and its bounded role.
//!
//! Absence of an unmounted file is the only signal a worker receives; no
//! filename, digest or metadata may reveal an embargoed peer or a policy reason
//! (`docs/22:60-69`, `docs/28:10-13,29-33`).

use super::schema::Schema;
use crate::permission::Permission;
use crate::role::Role;

/// One path in the offline worker file surface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct File {
    /// Absolute local path pinned by the supervisor.
    pub path: String,
    /// Governed role of the path.
    pub role: Role,
    /// Whether the path is mounted in this task view.
    pub mounted: crate::switch::Switch,
}

impl File {
    /// Declare the fixed task manifest.
    #[must_use]
    pub fn task() -> Self {
        Self {
            path: crate::TASK_PATH.to_owned(),
            role: Role::Task,
            mounted: crate::switch::Switch::On,
        }
    }

    /// Declare the fixed submission manifest.
    #[must_use]
    pub fn submission() -> Self {
        Self {
            path: crate::SUBMISSION_PATH.to_owned(),
            role: Role::Submission,
            mounted: crate::switch::Switch::On,
        }
    }

    /// Declare an optional local path that may be withheld without disclosure.
    #[must_use]
    pub fn optional(path: &str, role: Role) -> Self {
        Self {
            path: path.to_owned(),
            role,
            mounted: crate::switch::Switch::Off,
        }
    }

    /// Whether this file is one of the two schema-bearing paths.
    #[must_use]
    pub fn schema(&self) -> Option<Schema> {
        match self.role {
            Role::Task => Some(Schema::Task),
            Role::Submission => Some(Schema::Submission),
            Role::Repository | Role::References | Role::Feedback | Role::Artifact | Role::Unknown => {
                None
            }
        }
    }

    /// Whether the file may be read from this view.
    #[must_use]
    pub fn readable(&self) -> Permission {
        self.mounted.permit().and(Permission::of(self.role != Role::Unknown))
    }
}
