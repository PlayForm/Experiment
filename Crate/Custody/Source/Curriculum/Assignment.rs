//! One software-assigned independent study.
//!
//! Mirrors `examples/v61-study-assignment.example.json` and
//! `docs/27:11-14,32-40`: one research track, one constrained view, one local
//! task, fixed output classes and a deterministic assignment rule.

use super::challenge::Challenge;
use crate::permission::Permission;
use crate::warning::Warning;

/// A curriculum assignment issued by the trusted service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Assignment {
    /// `study_assignment_id` of this record.
    pub assignment_id: String,
    /// `curriculum_id` this study belongs to.
    pub curriculum_id: String,
    /// `research_track_id` that receives the task.
    pub track_id: String,
    /// `view_id` of the constrained source view.
    pub view_id: String,
    /// `challenge_family` assigned by the policy rule.
    pub family: Challenge,
    /// `worker_visible_task_digest` of the bounded task text.
    pub task_digest: String,
    /// `allowed_output_classes` such as patch or research-claim.
    pub allowed_outputs: Vec<String>,
    /// `assigned_by`, always the trusted curriculum service.
    pub assigned_by: String,
    /// `allocation_id` from the resource scheduler.
    pub allocation_id: String,
    /// `assignment_policy_digest` version that made the choice.
    pub policy_digest: String,
    /// `created_at` canonical UTC instant.
    pub created_at: String,
}

impl Assignment {
    /// Whether the assignment is complete and trusted-issued.
    #[must_use]
    pub fn validate(&self) -> Vec<Warning> {
        let absent = [
            ("assignment id", self.assignment_id.is_empty()),
            ("curriculum", self.curriculum_id.is_empty()),
            ("track", self.track_id.is_empty()),
            ("view", self.view_id.is_empty()),
            ("assigned by", self.assigned_by.is_empty()),
            ("allocation", self.allocation_id.is_empty()),
            ("policy", self.policy_digest.is_empty()),
            ("created at", self.created_at.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, missing)| {
            missing.then(|| Warning::Malformed {
                domain: field,
                detail: "a study assignment must name this field".to_owned(),
            })
        });
        let digest = match crate::digest::Digest::of(&self.task_digest).is_valid() {
            true => None,
            false => Some(Warning::DigestShape {
                field: "worker_visible_task_digest",
                detail: crate::digest::Digest::of(&self.task_digest).describe(),
            }),
        };
        absent.chain(digest).collect()
    }

    /// Whether a proposed output class was assigned to this study.
    #[must_use]
    pub fn allows_output(&self, name: &str) -> Permission {
        Permission::of(self.allowed_outputs.iter().any(|item| item == name))
    }

    /// Whether a model could have selected the family itself.
    #[must_use]
    pub fn model_chosen(&self) -> bool {
        self.family.model_selected()
    }
}
