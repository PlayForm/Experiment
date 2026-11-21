//! An immutable, signed paper review.
//!
//! Grounded in `sql/v5_migration.sql:203-228` (manuscript, reviewer assignment,
//! review kind in {methodology, replication, citations, sanitization,
//! release_policy}, outcome in {pass, fail, revise, inconclusive}, digests,
//! signature and a unique triple) and `OPERATIONS.md:61` (publication needs
//! independent science and sanitation attestations). No review is executed.

use crate::{Digest, Warning};

/// One signed review record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    /// Reviewer assignment identifier (`sql:207`).
    pub reviewer_assignment_id: String,
    /// Review kind label (`sql:208-215`).
    pub kind_label: &'static str,
    /// Outcome label (`sql:217-218`).
    pub outcome_label: &'static str,
    /// Content digest of the review (`sql:220`).
    pub review_sha256: Digest,
    /// Signature artifact digest (`sql:221`).
    pub signature_sha256: Digest,
}

impl Review {
    /// Declared review kinds (`sql:209-214`).
    pub const KINDS: [&'static str; 5] = [
        "methodology",
        "replication",
        "citations",
        "sanitization",
        "release_policy",
    ];
    /// Declared outcomes (`sql:218`).
    pub const OUTCOMES: [&'static str; 4] = ["pass", "fail", "revise", "inconclusive"];

    /// Validate kind, outcome and required identity.
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            Self::KINDS.contains(&self.kind_label),
            Self::OUTCOMES.contains(&self.outcome_label),
            self.reviewer_assignment_id.is_empty(),
        ) {
            (true, true, false) => Ok(()),
            (false, _, _) => Err(Warning::InvalidValue {
                field: "review kind",
                detail: self.kind_label.to_owned(),
            }),
            (_, false, _) => Err(Warning::InvalidValue {
                field: "review outcome",
                detail: self.outcome_label.to_owned(),
            }),
            (_, _, true) => Err(Warning::Denied {
                subject: "review",
                reason: "missing reviewer assignment",
            }),
        }
    }

    /// A review is an opinion or an attestation, never a promotion by itself
    /// (`README.md:44-46`, `AGENTS.md:17`).
    #[must_use]
    pub const fn is_self_promotion() -> crate::Permission {
        crate::Permission::Deny
    }
}
