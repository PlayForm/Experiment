//! Explicit, precondition-gated paper status transitions.
//!
//! Grounded in `PUBLISHING.md:13-40` (the status ladder and assembly pipeline),
//! `SYSTEM.md:104-126` (predecessor requirements and immutable evidence
//! fingerprints per decision state), `README.md:46-49` (no automatic promotion)
//! and `paper.v5.schema.json:50-60`. Each transition names its from/to rungs
//! and required evidence; `permit` returns a typed decision or a warning and
//! never mutates a paper. No paper is actually promoted.

use crate::{claim::Evidence, claim::Status as EvidenceStatus, Permission, Warning};

use super::{Sanitization, Status};

/// A named, non-automatic move along the paper status ladder.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Transition {
    /// Hypothesis becomes an internal observation (`PUBLISHING.md:15-16`).
    ToInternal,
    /// Observation becomes a measured lab report (`PUBLISHING.md:19`).
    ToMeasured,
    /// Measurement becomes independently replicated (`PUBLISHING.md:20-22`).
    ToReplicated,
    /// Replication becomes a sanitized preprint (`PUBLISHING.md:23-25`).
    ToPreprint,
    /// Preprint becomes externally reviewed (`PUBLISHING.md:25-27`).
    ToReviewed,
    /// An immutable correction supersedes prior bytes (`PUBLISHING.md:28`).
    ToCorrected,
    /// An immutable retraction remains searchable (`PUBLISHING.md:28`).
    ToRetracted,
}

impl Transition {
    /// The predecessor rung.
    #[must_use]
    pub const fn from(self) -> Status {
        match self {
            Self::ToInternal => Status::Hypothesis,
            Self::ToMeasured => Status::Internal,
            Self::ToReplicated => Status::Measured,
            Self::ToPreprint => Status::Replicated,
            Self::ToReviewed => Status::Sanitized,
            Self::ToCorrected | Self::ToRetracted => Status::Replicated,
        }
    }

    /// The successor rung.
    #[must_use]
    pub const fn to(self) -> Status {
        match self {
            Self::ToInternal => Status::Internal,
            Self::ToMeasured => Status::Measured,
            Self::ToReplicated => Status::Replicated,
            Self::ToPreprint => Status::Sanitized,
            Self::ToReviewed => Status::Reviewed,
            Self::ToCorrected => Status::Corrected,
            Self::ToRetracted => Status::Retracted,
        }
    }

    /// The evidence required before this transition can be permitted
    /// (`SYSTEM.md:124-126`).
    #[must_use]
    pub const fn required(self) -> &'static str {
        match self {
            Self::ToInternal => "signed observation or trace",
            Self::ToMeasured => "signed measurement with cohort and protocol",
            Self::ToReplicated => "independent replication on a separate run",
            Self::ToPreprint => "passed sanitization plus declassification grant",
            Self::ToReviewed => "genuine external evaluation proof",
            Self::ToCorrected => "signed amendment record",
            Self::ToRetracted => "signed withdrawal record",
        }
    }

    /// Decide whether a transition may proceed from typed preconditions.
    /// A generated artifact can never supply the grant, external review or
    /// measurement authority itself (worker model rule).
    pub fn permit(
        &self,
        evidence: &Evidence,
        sanitization: Sanitization,
        grant: Permission,
        external_review: Permission,
    ) -> Result<Permission, Warning> {
        let measured = evidence.is_measured();
        // Exception: a match guard is needed because the replicated rung
        // requires replication evidence specifically, not merely any measured
        // evidence (PUBLISHING.md:20-22, SYSTEM.md:124-126).
        let decision = match (self, measured, sanitization, grant, external_review) {
            (Self::ToInternal | Self::ToMeasured, Permission::Allow, _, _, _) => Permission::Allow,
            (Self::ToReplicated, Permission::Allow, _, _, _)
                if matches!(evidence, Evidence::Replication { .. }) => Permission::Allow,
            (Self::ToPreprint, _, Sanitization::Passed, Permission::Allow, _) => Permission::Allow,
            (Self::ToReviewed, _, Sanitization::Passed, Permission::Allow, Permission::Allow) => {
                Permission::Allow
            }
            (Self::ToCorrected | Self::ToRetracted, _, _, _, _) => Permission::Allow,
            _ => Permission::Deny,
        };
        match decision {
            Permission::Allow => Ok(Permission::Allow),
            Permission::Deny => Err(Warning::Denied {
                subject: "paper transition",
                reason: self.required(),
            }),
        }
    }

    /// A paper never promotes itself (`README.md:46-49`).
    #[must_use]
    pub const fn may_auto_promote() -> Permission {
        Permission::Deny
    }

    /// Grade produced by this transition, for the paper's claim graph.
    #[must_use]
    pub const fn evidence_grade(&self) -> EvidenceStatus {
        match self {
            Self::ToInternal => EvidenceStatus::Unsupported,
            Self::ToMeasured => EvidenceStatus::Measured,
            Self::ToReplicated => EvidenceStatus::IndependentlyReplicated,
            Self::ToPreprint | Self::ToReviewed => EvidenceStatus::IndependentlyReplicated,
            Self::ToCorrected | Self::ToRetracted => EvidenceStatus::Unsupported,
        }
    }
}
