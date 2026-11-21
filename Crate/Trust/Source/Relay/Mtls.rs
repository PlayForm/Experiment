//! Peer resolution from a mutually authenticated TLS handshake.
//!
//! Grounded in `security/src/relay.rs:176-200`: the server hashes the first
//! peer certificate with `crypto::sha`, walks the registry for a valid identity
//! whose `tls_certificate_sha256` equals that fingerprint, and rejects the
//! connection when the credential is missing, unregistered or revoked. A
//! user-supplied identity header is never trusted (`relay.rs:184-185`).
//! No handshake is performed here.

use crate::permission::Permission;

/// The outcome of resolving an mTLS peer to a registered identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Mtls {
    /// The fingerprint mapped to a valid, non-revoked identity
    /// (`relay.rs:194-196`).
    Resolved {
        identity: String,
        fingerprint: String,
    },
    /// The TLS handshake carried no peer certificate (`relay.rs:178-181`).
    MissingCertificate,
    /// No registered identity matched the fingerprint (`relay.rs:200`).
    Unregistered { fingerprint: String },
    /// A matching identity was found but was revoked (`relay.rs:193`).
    Revoked { identity: String },
}

impl Mtls {
    /// The identity record field compared to the certificate fingerprint
    /// (`crypto.rs:38`, `relay.rs:194`).
    pub const FINGERPRINT_FIELD: &'static str = "tls_certificate_sha256";

    /// Whether this resolution may proceed to request handling.
    #[must_use]
    pub const fn is_accepted(&self) -> Permission {
        match self {
            Self::Resolved { .. } => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// The registry name when resolution succeeded, as a typed reference.
    #[must_use]
    pub fn identity(&self) -> Option<&str> {
        match self {
            Self::Resolved { identity, .. } | Self::Revoked { identity } => Some(identity),
            Self::MissingCertificate | Self::Unregistered { .. } => None,
        }
    }
}
