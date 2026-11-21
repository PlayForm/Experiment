//! The signed declassification grant.
//!
//! Grounded in `PUBLISHING.md:66-70` (publication events require a grant and an
//! issuer distinct from researcher, verifier and integrator),
//! `paper.v5.schema.json:134-136` (optional grant ID),
//! `sql/v5_migration.sql:234-239` (required grant digest and issuer on a
//! release), `V4_TRUST_BRIDGE.md:24-25` (publisher sends grant and release
//! signature) and `playbooks/research-market-week.yaml:56-62`. No document is
//! declassified by this value.

use crate::{case::Privacy, Digest, Permission, Warning};

/// Authorization to move a sanitized derivative into a wider audience.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Declassification {
    /// Grant identifier (`paper schema:135`).
    pub grant_id: String,
    /// Digest of the signed grant artifact (`sql:235`).
    pub grant_sha256: Digest,
    /// Distinct issuer identity (`PUBLISHING.md:70`, `sql:237`).
    pub issuer_identity: String,
    /// Audience this grant unlocks.
    pub audience: Privacy,
}

impl Declassification {
    /// Validate grant shape without checking its signature.
    pub fn validate(&self) -> Result<(), Warning> {
        match (self.grant_id.is_empty(), self.issuer_identity.is_empty()) {
            (false, false) => Ok(()),
            (true, _) => Err(Warning::MissingArtifact {
                digest: "declassification_grant_id".to_owned(),
            }),
            (_, true) => Err(Warning::Denied {
                subject: "declassification",
                reason: "missing distinct issuer",
            }),
        }
    }

    /// Whether a case's privacy class needs this grant
    /// (`PUBLISHING.md:66-70`).
    #[must_use]
    pub fn required_for(audience: Privacy) -> Permission {
        audience.requires_grant()
    }

    /// A researcher, benchmark verifier or integrator cannot self-grant
    /// (`PUBLISHING.md:70`, `V4_TRUST_BRIDGE.md:28-31`).
    #[must_use]
    pub const fn may_be_self_granted() -> Permission {
        Permission::Deny
    }
}
