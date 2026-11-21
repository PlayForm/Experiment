//! A CA-attestable public identity record.
//!
//! Grounded in `security/src/crypto.rs:32-43` (the nine `PublicIdentity`
//! fields), `crypto.rs:109-129` (identity-name, revocation, validity-window and
//! authority-signature checks) and `storage.rs:196-206` (field construction at
//! issuance). This is a typed mirror of that record; it issues nothing.

use serde::{Deserialize, Serialize};

use crate::warning::Warning;

/// The typed public identity binding issued by the prototype CA.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    /// Principal name; validated as 2-64 ASCII letters/digits/`_`/`-`.
    pub identity: String,
    /// Role label recorded at issuance.
    pub role: String,
    /// Hex Ed25519 verifying key.
    pub sign_public_hex: String,
    /// Base64 HPKE public key.
    pub encrypt_public_b64: String,
    /// SHA-256 of the TLS leaf certificate DER.
    pub tls_certificate_sha256: String,
    /// Message kinds this identity may send (`crypto.rs:151,204`).
    pub allowed_message_types: Vec<String>,
    /// Inclusive validity start, Unix seconds.
    pub created_unix: i64,
    /// Exclusive validity end, Unix seconds.
    pub expires_unix: i64,
    /// SHA-256 issuance identifier (`storage.rs:205`).
    pub issuance_id: String,
}

impl Identity {
    /// Validate the identity name without touching the filesystem
    /// (`storage.rs:36-41`).
    pub fn check_name(&self) -> Result<(), Warning> {
        let bytes = self.identity.as_bytes();
        let well_formed = (2..=64).contains(&bytes.len())
            && bytes.iter().all(|byte| byte.is_ascii_alphanumeric() || *byte == b'-' || *byte == b'_');
        match well_formed {
            true => Ok(()),
            false => Err(Warning::Malformed {
                domain: "identity-name",
                detail: self.identity.clone(),
            }),
        }
    }

    /// Whether a message kind is within the sender's authorization
    /// (`crypto.rs:151,204`).
    #[must_use]
    pub fn permits(&self, kind: &str) -> bool {
        self.allowed_message_types.iter().any(|allowed| allowed == kind)
    }

    /// Whether the supplied Unix time lies inside the validity window
    /// (`crypto.rs:124-127`).
    #[must_use]
    pub fn valid_at(&self, now: i64) -> bool {
        now >= self.created_unix && now < self.expires_unix
    }

    /// Classify a time relative to the validity window as a typed decision.
    pub fn validity(&self, now: i64) -> Result<(), Warning> {
        match self.valid_at(now) {
            true => Ok(()),
            false => Err(Warning::Expired {
                subject: self.identity.clone(),
            }),
        }
    }
}
