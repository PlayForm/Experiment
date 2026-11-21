//! A bound license grant from the pinned v6.1 example.
//!
//! Unknown rights fail closed: `Decision::Pending` and `Decision::Prohibited`
//! both deny propagation until a separate owner review record exists
//! (`docs/20:8-11,20-24`; `examples/v61-license-grant`).

use crate::decision::Decision;
use crate::permission::Permission;
use crate::switch::Switch;
use crate::warning::Warning;

/// The rights record a custody preflight consults.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grant {
    /// `grant_id` of the bound license record.
    pub grant_id: String,
    /// `repository_id` the grant belongs to.
    pub repository_id: String,
    /// `source_digest` the terms were reviewed against.
    pub source_digest: String,
    /// `spdx_expression` recorded verbatim.
    pub spdx_expression: String,
    /// `license_text_digest` of preserved LICENSE/NOTICE text.
    pub license_text_digest: String,
    /// `eula_artifact_digest` of verbatim EULA terms.
    pub eula_artifact_digest: String,
    /// `license_status` mapped to a conservative decision.
    pub status: Decision,
    /// `allowed_uses` such as build, test, research.
    pub allowed_uses: Vec<String>,
    /// `derivative_obligations` such as retaining notices.
    pub derivative_obligations: Vec<String>,
    /// `redistribution_allowed` from the example.
    pub redistribution_allowed: Switch,
    /// `model_training_allowed` from the example.
    pub model_training_allowed: Switch,
    /// `jurisdiction_review_required` flag.
    pub jurisdiction_review_required: Switch,
    /// `review_authority` that may resolve ambiguity.
    pub review_authority: String,
    /// `review_record_digest` of the approval, if any.
    pub review_record_digest: String,
    /// `valid_from` canonical UTC instant.
    pub valid_from: String,
}

impl Grant {
    /// Whether a named use is within the recorded grant.
    #[must_use]
    pub fn allows(&self, use_kind: &str) -> Permission {
        Permission::of(self.allowed_uses.iter().any(|item| item == use_kind))
    }

    /// The conservative propagation decision for a use and its rights.
    #[must_use]
    pub fn propagate(&self, use_kind: &str) -> Permission {
        self.status
            .permit()
            .and(self.allows(use_kind))
            .and(Permission::filled(&self.valid_from))
            .and(Permission::digest(&self.license_text_digest))
    }

    /// Findings for a grant that cannot safely authorize a transfer.
    #[must_use]
    pub fn validate(&self) -> Vec<Warning> {
        let absent = [
            ("grant id", self.grant_id.is_empty()),
            ("repository", self.repository_id.is_empty()),
            ("expression", self.spdx_expression.is_empty()),
            ("review authority", self.review_authority.is_empty()),
            ("valid from", self.valid_from.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, missing)| {
            missing.then(|| Warning::Malformed {
                domain: field,
                detail: "a license grant must name this field".to_owned(),
            })
        });
        let undecided = match self.status {
            Decision::Compatible => None,
            Decision::Incompatible | Decision::Pending | Decision::Prohibited => {
                Some(Warning::Withheld {
                    subject: "license rights",
                })
            }
        };
        let untrained = match self.model_training_allowed {
            Switch::Off => None,
            Switch::On => match Permission::digest(&self.review_record_digest) {
                Permission::Allow => None,
                Permission::Deny => Some(Warning::Unverified {
                    subject: "model training review".to_owned(),
                }),
            },
        };
        absent.chain(undecided).chain(untrained).collect()
    }
}
