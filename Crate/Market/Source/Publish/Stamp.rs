//! The publication stamp and its distinct signer.
//!
//! Grounded in `PUBLISHING.md:70` (an issuer identity distinct from researcher,
//! benchmark verifier and integrator), `V4_TRUST_BRIDGE.md:25-31` (the publisher
//! signs a sanitized paper hash, grant and release signature),
//! `sql/v5_migration.sql:230-240` (visibility, manifest digest, issuer and
//! signature) and `SYSTEM.md:96-99` (the publication authority never upgrades
//! an unreplicated claim). A stamp here is data; nothing is signed.

use crate::{Digest, Permission, Warning};

/// A signed release stamp on a sanitized paper derivative.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stamp {
    /// Distinct publication issuer (`PUBLISHING.md:70`).
    pub issuer_identity: String,
    /// Digest of the signed release manifest (`sql:236`).
    pub manifest_sha256: Digest,
    /// Digest of the signature artifact (`sql:238`).
    pub signature_sha256: Digest,
    /// Visibility label of the release (`sql:234`).
    pub visibility: &'static str,
}

impl Stamp {
    /// Declared release visibilities (`sql:234`).
    pub const VISIBILITIES: [&'static str; 2] = ["tenant", "public"];

    /// Validate the stamp without verifying cryptography.
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            self.issuer_identity.is_empty(),
            Self::VISIBILITIES.contains(&self.visibility),
        ) {
            (false, true) => Ok(()),
            (true, _) => Err(Warning::Denied {
                subject: "publication stamp",
                reason: "missing distinct issuer",
            }),
            (_, false) => Err(Warning::InvalidValue {
                field: "visibility",
                detail: self.visibility.to_owned(),
            }),
        }
    }

    /// A researcher cannot sign the release of their own paper
    /// (`PUBLISHING.md:70`, `AGENTS.md:17`).
    #[must_use]
    pub const fn researcher_may_sign() -> Permission {
        Permission::Deny
    }

    /// A benchmark verifier's measurement signature is a separate act
    /// (`V4_TRUST_BRIDGE.md:57-63`, `SECURITY.md:79-80`).
    #[must_use]
    pub const fn benchmark_verifier_may_sign() -> Permission {
        Permission::Deny
    }

    /// An integrator cannot sign an evidence promotion
    /// (`V4_TRUST_BRIDGE.md:28`, `SYSTEM.md:101-102`).
    #[must_use]
    pub const fn integrator_may_sign() -> Permission {
        Permission::Deny
    }
}
