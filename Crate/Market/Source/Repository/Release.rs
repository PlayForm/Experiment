//! Publication policy of an adapter (schema `publication_policy`).
//!
//! Grounded in `repository-adapter.v5.schema.json:154-156`
//! (`internal_only`, `manual_review`, `preapproved_public`) and
//! `PUBLISHING.md:65-70` (no auto-publishing for unknown external repositories;
//! a public repo still needs owner-defined policy). The one-word type name
//! carries the schema key in its labels.

use crate::Warning;

/// Release policy declared by an adapter owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Release {
    /// Results stay inside the tenant (`schema:155`).
    InternalOnly,
    /// A human reviews every release (`schema:155`).
    ManualReview,
    /// Owner pre-approved public release (`schema:155`).
    PreapprovedPublic,
}

impl Release {
    /// All release policies.
    pub const ALL: [Self; 3] = [Self::InternalOnly, Self::ManualReview, Self::PreapprovedPublic];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "internal_only" => Ok(Self::InternalOnly),
            "manual_review" => Ok(Self::ManualReview),
            "preapproved_public" => Ok(Self::PreapprovedPublic),
            other => Err(Warning::InvalidValue {
                field: "publication policy",
                detail: format!("unknown policy {other}"),
            }),
        }
    }

    /// The wire label of this policy.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::InternalOnly => "internal_only",
            Self::ManualReview => "manual_review",
            Self::PreapprovedPublic => "preapproved_public",
        }
    }

    /// Even pre-approval does not replace the signed declassification grant
    /// (`PUBLISHING.md:66-70`).
    #[must_use]
    pub const fn bypasses_declassification() -> crate::Permission {
        crate::Permission::Deny
    }
}
