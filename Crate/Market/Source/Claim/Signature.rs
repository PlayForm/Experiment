//! A claim signature and the authority it does not confer.
//!
//! Grounded in `claim.v5.schema.json:101-105` (`signature_b64`),
//! `V4_TRUST_BRIDGE.md:25-31` (researcher signer for claims, experiment signer
//! for measurements, publisher signer for releases; authenticated identity
//! alone does not authorize funding or publication) and `:57-63` (a valid
//! signature is not scientific validity). Nothing is signed or verified here.

use crate::Digest;

/// A typed signature binding a signer to a payload commitment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature {
    /// Signing identity (`V4_TRUST_BRIDGE.md:25`).
    pub signer_identity: String,
    /// Deterministic serialization commitment over the claim bytes (`:16-17`).
    pub payload_sha256: Digest,
    /// Context snapshot bound into the signature (`claim schema:94-97`).
    pub context_snapshot: Digest,
    /// Whether this signer is the independent experiment authority (`:26`).
    pub experiment_authority: crate::Permission,
    /// Wire signature label; a demo value is unverified (`claim example:16`).
    pub signature_b64: String,
}

impl Signature {
    /// Minimum encoded signature length from the schema (`schema:104`).
    pub const MIN_LENGTH: usize = 16;

    /// Validate signature shape without verifying cryptography.
    pub fn validate(&self) -> Result<(), crate::Warning> {
        match (
            self.signer_identity.is_empty(),
            self.signature_b64.len() >= Self::MIN_LENGTH,
        ) {
            (false, true) => Ok(()),
            (true, _) => Err(crate::Warning::Denied {
                subject: "claim signature",
                reason: "missing signer identity",
            }),
            (_, false) => Err(crate::Warning::InvalidValue {
                field: "signature_b64",
                detail: format!("shorter than {}", Self::MIN_LENGTH),
            }),
        }
    }

    /// Only the experiment signer may sign a measurement
    /// (`V4_TRUST_BRIDGE.md:26`, `AGENTS.md:17`).
    #[must_use]
    pub fn may_sign_measurement(&self) -> crate::Permission {
        self.experiment_authority
    }

    /// A correct CA signature never proves scientific validity
    /// (`V4_TRUST_BRIDGE.md:57-63`).
    #[must_use]
    pub const fn implies_scientific_validity() -> crate::Permission {
        crate::Permission::Deny
    }
}
