//! Publication signatures, historical keys and scientific validity.
//!
//! Grounded in `V4_TRUST_BRIDGE.md:49-63`: short-lived keys, signed issuance
//! log, issuer key ID and rotation version, revocation list and replay cache;
//! publication signatures must remain verifiable against historical issuer keys
//! after expiry; mTLS freshness and historical paper validity are separate
//! questions; certificate issuance attests identity/key binding, not benchmark
//! correctness, and a correctly signed paper may still be invalid. No signature
//! is created or checked here.

use crate::permission::Permission;

/// A typed publication-signature validity contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature {
    /// Issuer key identifier recorded with the signature (`:51`).
    pub issuer_key_id: &'static str,
    /// Rotation version of the issuing key (`:51`).
    pub rotation_version: u32,
    /// Whether the signature remains verifiable after key expiry (`:54`).
    pub verifiable_after_expiry: Permission,
    /// Whether a valid CA signature implies scientific validity (`:57-63`).
    pub scientific_validity: Permission,
}

impl Signature {
    /// Construct the publication-signature contract.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            issuer_key_id: "message-authority",
            rotation_version: 1,
            verifiable_after_expiry: Permission::Allow,
            scientific_validity: Permission::Deny,
        }
    }

    /// mTLS credential freshness and historical paper validity are separate
    /// questions (`:54-55`).
    #[must_use]
    pub const fn freshness_is_validity() -> Permission {
        Permission::Deny
    }

    /// A generated artifact never signs measurement or policy; only a trusted
    /// evaluator or authorized human does (worker model rule, `AGENTS.md:1`
    /// and `V4_TRUST_BRIDGE.md:57-63`).
    #[must_use]
    pub const fn generated_artifact_may_sign() -> Permission {
        Permission::Deny
    }
}
