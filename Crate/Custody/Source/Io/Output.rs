//! The single model-visible outgoing manifest.
//!
//! Mirrors `schemas/v61-offline-submission.schema.json` and
//! `examples/v61-offline-submission.example.json`. It refers only to fixed
//! local artifacts under scratch, and its `claim_of_success` is explicitly not
//! authoritative (`docs/28:15-27`, `docs/22:56-59`).

use super::schema::Schema;
use crate::claim::Claim;
use crate::permission::Permission;
use crate::warning::Warning;
use crate::worker::scratch::Scratch;

/// The complete worker-visible output value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Output {
    /// `local_nonce`, copied from the task by the supervisor.
    pub local_nonce: String,
    /// `patch_file`, or absent when no patch was submitted.
    pub patch_file: Option<String>,
    /// `claims_file`, or absent when no claims were submitted.
    pub claims_file: Option<String>,
    /// `test_proposals_dir`, or absent when none were submitted.
    pub test_proposals_dir: Option<String>,
    /// `paper_draft_file`, or absent when no draft was assigned.
    pub paper_draft_file: Option<String>,
    /// `claim_of_success`, never a release authority.
    pub claim_of_success: Claim,
}

impl Output {
    /// Build an empty manifest for a task-local nonce.
    #[must_use]
    pub fn of(nonce: &str) -> Self {
        Self {
            local_nonce: nonce.to_owned(),
            patch_file: None,
            claims_file: None,
            test_proposals_dir: None,
            paper_draft_file: None,
            claim_of_success: Claim::None,
        }
    }

    /// Bind a submitted artifact whose path was already checked.
    #[must_use]
    pub fn with(mut self, field: crate::role::Role, path: &str) -> Self {
        let value = Some(path.to_owned());
        match field {
            crate::role::Role::Artifact => {
                self.patch_file = value;
            }
            crate::role::Role::Submission => {
                self.claims_file = value;
            }
            crate::role::Role::Feedback => {
                self.test_proposals_dir = value;
            }
            crate::role::Role::References => {
                self.paper_draft_file = value;
            }
            crate::role::Role::Task | crate::role::Role::Repository | crate::role::Role::Unknown => {}
        }
        self
    }

    /// Set the worker's non-authoritative success claim.
    #[must_use]
    pub fn claiming(mut self, claim: Claim) -> Self {
        self.claim_of_success = claim;
        self
    }

    /// All fixed artifact paths named in the manifest.
    #[must_use]
    pub fn paths(&self) -> Vec<&str> {
        [
            self.patch_file.as_deref(),
            self.claims_file.as_deref(),
            self.test_proposals_dir.as_deref(),
            self.paper_draft_file.as_deref(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    /// Validate the manifest shape and scratch containment.
    #[must_use]
    pub fn validate(&self, scratch: &Scratch) -> Vec<Warning> {
        let nonce_ok = !self.local_nonce.is_empty()
            && self.local_nonce.len() <= 64
            && self.local_nonce.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
            });
        let bounded = self.paths().iter().all(|path| scratch.within(path) == Permission::Allow);
        let claim_bound = !self.claim_of_success.authoritative();
        [
            ("offline submission nonce", nonce_ok),
            ("submission paths", bounded && claim_bound),
        ]
        .into_iter()
        .filter_map(|(domain, valid)| {
            (!valid).then(|| Warning::Malformed {
                domain,
                detail: "a field is outside the pinned schema".to_owned(),
            })
        })
        .collect()
    }

    /// The outbound schema that validates this value.
    #[must_use]
    pub fn schema(&self) -> Schema {
        Schema::Submission
    }
}
