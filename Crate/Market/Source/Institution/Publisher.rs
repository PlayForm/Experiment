//! Publication authority: privileged, not omniscient.
//!
//! Grounded in `SYSTEM.md:56,96-99` (validates evidence linkage and policy-
//! compliant sanitization but never upgrades an unreplicated claim because its
//! prose is persuasive), `PUBLISHING.md:42-46` (authorized recipient only, no
//! universal decryption key) and `OPERATIONS.md:61` (no public release without
//! independent science and sanitation attestations).

use crate::Permission;

/// The scoped scientific publication institution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Publisher {
    /// Distinct publisher identity (`PUBLISHING.md:70`).
    pub publisher_identity: &'static str,
    /// Digest of the release policy it enforces.
    pub release_policy_sha256: &'static str,
}

impl Publisher {
    /// Construct the publication authority model.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            publisher_identity: "independent_publisher",
            release_policy_sha256: "release_policy_sha256",
        }
    }

    /// The publisher cannot read unrelated private notebooks
    /// (`PUBLISHING.md:46`, `SECURITY.md:58`).
    #[must_use]
    pub const fn may_read_unsubmitted_notebook() -> Permission {
        Permission::Deny
    }

    /// Persuasive prose never upgrades an unreplicated claim
    /// (`SYSTEM.md:96-99`, `README.md:46-49`).
    #[must_use]
    pub const fn may_upgrade_unreplicated_claim() -> Permission {
        Permission::Deny
    }

    /// External peer review may not be fabricated (`PUBLISHING.md:25-27`).
    #[must_use]
    pub const fn may_assert_external_review_without_proof() -> Permission {
        Permission::Deny
    }
}
