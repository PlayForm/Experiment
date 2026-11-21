//! A purpose-specific signer capability.
//!
//! Grounded in `V4_TRUST_BRIDGE.md:11-15,28-31`: each payload carries a
//! `payload_schema_id`, a message-type-specific signer capability and a policy
//! version; "authenticated identity alone does not authorize funding,
//! publication or release". A capability is granted only when both the
//! identity/role check and the payload attestations pass. No capability is
//! granted here.

use crate::permission::Permission;

/// A typed capability for one payload purpose.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Capability {
    /// Schema identifier of the signed payload (`V4_TRUST_BRIDGE.md:28`).
    pub payload_schema_id: &'static str,
    /// Version of the policy that defines this capability (`:28`).
    pub policy_version: &'static str,
    /// The signer the capability is bound to (`:28`).
    pub signer: &'static str,
    /// Whether a bare identity/role check is sufficient (`:30-31`).
    pub identity_alone: Permission,
}

impl Capability {
    /// Construct a capability with deny-by-default authority.
    #[must_use]
    pub const fn new(payload_schema_id: &'static str, signer: &'static str) -> Self {
        Self {
            payload_schema_id,
            policy_version: "v1",
            signer,
            identity_alone: Permission::Deny,
        }
    }

    /// Require both the identity check and the payload evidence check.
    #[must_use]
    pub const fn grant(identity_ok: Permission, evidence_ok: Permission) -> Permission {
        identity_ok.and(evidence_ok)
    }

    /// No generated artifact may mint its own capability: trusted selection is
    /// deterministic software (`worker model` rule on this card).
    #[must_use]
    pub const fn artifact_may_self_grant() -> Permission {
        Permission::Deny
    }
}
