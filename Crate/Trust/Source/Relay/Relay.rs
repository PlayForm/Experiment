//! The relay's authority and custody boundaries.
//!
//! Grounded in `security/src/relay.rs:1-4` (the relay verifies peer identities
//! but never holds HPKE private keys), `relay.rs:176-200` (the peer identity is
//! derived from the attested TLS certificate fingerprint, never from a
//! user-supplied header), `relay.rs:165,204` (one mutex serializes filesystem
//! access), `relay.rs:202-203` (validation errors return no candidate data) and
//! `V4_TRUST_BRIDGE.md:33-47` (the relay sees metadata and ciphertext only).

use crate::permission::Permission;

/// A typed description of the narrow relay authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Relay {
    /// The relay must never hold recipient decryption keys (`relay.rs:1-4`).
    pub hpke_key_custody: Permission,
    /// Where the authenticated peer identity comes from (`relay.rs:184-200`).
    pub peer_identity_source: &'static str,
    /// How filesystem mutations are serialized (`relay.rs:165,204`).
    pub filesystem_serialization: &'static str,
    /// What the relay is allowed to observe (`V4_TRUST_BRIDGE.md:35-37`).
    pub visible_metadata: &'static str,
}

impl Relay {
    /// Construct the prototype's relay authority definition.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            hpke_key_custody: Permission::Deny,
            peer_identity_source: "tls-certificate-fingerprint",
            filesystem_serialization: "one-process-mutex",
            visible_metadata: "sender,recipient,kind,size,timestamps,ciphertext",
        }
    }

    /// The relay may never hold an HPKE private key.
    #[must_use]
    pub const fn may_hold_private_key() -> Permission {
        Permission::Deny
    }

    /// Whether a claimed peer identity may be trusted from a request header.
    /// The prototype explicitly rejects that (`relay.rs:184-185`).
    #[must_use]
    pub const fn trusts_identity_header() -> bool {
        false
    }
}
