//! The authorized human legal-review boundary.
//!
//! A compliance workflow, not a legal-opinion engine: ambiguous or negotiated
//! rights require an authorized owner, and a version change never retroactively
//! grants an earlier transfer (`docs/20:9-11,22-24`, `docs/25:26`).

use crate::decision::Decision;
use crate::permission::Permission;
use crate::switch::Switch;

/// The review record that can resolve an ambiguous license question.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    /// `review_id` of the signed record.
    pub review_id: String,
    /// Bound decision from the owner.
    pub decision: Decision,
    /// `review_authority` that issued it.
    pub reviewer_id: String,
    /// `review_record_digest` of the signed record.
    pub record_digest: String,
    /// Whether the authority is authorized for this scope.
    pub authorized: Switch,
    /// Version of the terms this review applies to.
    pub terms_version: String,
    /// Canonical UTC time the review was recorded.
    pub recorded_at: String,
}

impl Review {
    /// A safe default that fails closed when no record exists.
    #[must_use]
    pub fn absent() -> Self {
        Self {
            review_id: String::new(),
            decision: Decision::Pending,
            reviewer_id: String::new(),
            record_digest: String::new(),
            authorized: Switch::Off,
            terms_version: String::new(),
            recorded_at: String::new(),
        }
    }

    /// Whether this record may authorize a propagation decision.
    #[must_use]
    pub fn authorizes(&self) -> Permission {
        self.authorized
            .permit()
            .and(self.decision.permit())
            .and(Permission::filled(&self.review_id))
            .and(Permission::filled(&self.reviewer_id))
            .and(Permission::digest(&self.record_digest))
            .and(Permission::filled(&self.terms_version))
    }

    /// Whether this review applies to a cited terms version.
    #[must_use]
    pub fn covers(&self, version: &str) -> Permission {
        self.authorizes().and(Permission::of(self.terms_version == version))
    }
}
