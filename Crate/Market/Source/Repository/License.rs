//! Repository licensing policy of an adapter.
//!
//! Grounded in `repository-adapter.v5.schema.json:56-58`,
//! `PUBLISHING.md:65-68` (licensing is checked for public open-source repos and
//! publishing still needs owner policy) and `REPOSITORIES.md:72-75`
//! (explicit publication licenses in open collaboration).

use crate::Warning;

/// License policy attached to an adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum License {
    /// Public release allowed by the license (`schema:57`).
    PublicAllowed,
    /// Private use only (`schema:57`).
    PrivateOnly,
    /// Human review required before release (`schema:57`).
    ReviewRequired,
}

impl License {
    /// All license policies.
    pub const ALL: [Self; 3] = [Self::PublicAllowed, Self::PrivateOnly, Self::ReviewRequired];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "public_allowed" => Ok(Self::PublicAllowed),
            "private_only" => Ok(Self::PrivateOnly),
            "review_required" => Ok(Self::ReviewRequired),
            other => Err(Warning::InvalidValue {
                field: "license policy",
                detail: format!("unknown policy {other}"),
            }),
        }
    }

    /// The wire label of this policy.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PublicAllowed => "public_allowed",
            Self::PrivateOnly => "private_only",
            Self::ReviewRequired => "review_required",
        }
    }

    /// Whether a release needs a separate review decision
    /// (`PUBLISHING.md:66-68`).
    #[must_use]
    pub const fn needs_review(self) -> crate::Permission {
        match self {
            Self::ReviewRequired | Self::PrivateOnly => crate::Permission::Allow,
            Self::PublicAllowed => crate::Permission::Deny,
        }
    }
}
