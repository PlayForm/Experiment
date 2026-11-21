//! The signed, length-checked HPKE envelope record.
//!
//! Grounded in `security/src/crypto.rs:49-67` (`ProtectedHeader` with nine
//! fields, `EncryptedEnvelope` with protected header plus encapsulation,
//! ciphertext and signature), `crypto.rs:68-73` (the three signed parts),
//! `crypto.rs:146` (8 MiB plaintext cap), `crypto.rs:188-214` (version, nonce,
//! time-window, kind and ciphertext-size checks). This record is data only.

use serde::{Deserialize, Serialize};

use crate::{MAX_MESSAGE_BYTES, PROTOCOL_VERSION, warning::Warning};

/// A typed mirror of the v4 encrypted envelope.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    /// Envelope format version; must equal `PROTOCOL_VERSION` (`crypto.rs:190`).
    pub version: u32,
    /// Message ID; must equal `nonce_hex` (`crypto.rs:193`).
    pub message_id: String,
    /// Author identity (`crypto.rs:154`).
    pub sender: String,
    /// Recipient identity (`crypto.rs:155`).
    pub recipient: String,
    /// Authorized message kind (`crypto.rs:204`).
    pub kind: String,
    /// Campaign label (`crypto.rs:165`).
    pub campaign: String,
    /// Creation time, Unix seconds.
    pub created_unix: i64,
    /// Expiry time, Unix seconds; window is at most one hour (`crypto.rs:198`).
    pub expires_unix: i64,
    /// Hex nonce bound to `message_id`.
    pub nonce_hex: String,
    /// Base64 HPKE encapsulated key.
    pub encapsulation_b64: String,
    /// Base64 authenticated ciphertext.
    pub ciphertext_b64: String,
    /// Base64 Ed25519 signature over the signed parts.
    pub signature_b64: String,
}

impl Envelope {
    /// The three parts covered by the Ed25519 signature
    /// (`crypto.rs:69-73,130-137`).
    pub const SIGNED_PARTS: [&'static str; 3] = ["protected", "encapsulation_b64", "ciphertext_b64"];

    /// Maximum permitted validity window in seconds (`crypto.rs:198`: 3600).
    pub const MAX_WINDOW_SECONDS: i64 = 3_600;

    /// Check the envelope's static invariants without decoding or verifying
    /// anything (`crypto.rs:188-200`).
    pub fn check(&self, now: i64) -> Result<(), Warning> {
        let version = match self.version == PROTOCOL_VERSION {
            true => Ok(()),
            false => Err(Warning::Drift {
                domain: "envelope-version",
                expected: PROTOCOL_VERSION,
                found: self.version,
            }),
        };
        version
            .and_then(|()| match self.message_id == self.nonce_hex && self.nonce_hex.len() == 32 {
                true => Ok(()),
                false => Err(Warning::Malformed {
                    domain: "envelope-nonce",
                    detail: self.nonce_hex.clone(),
                }),
            })
            .and_then(|()| {
                match now + 60 < self.created_unix
                    || now >= self.expires_unix
                    || self.expires_unix - self.created_unix > Self::MAX_WINDOW_SECONDS
                {
                    true => Err(Warning::Expired {
                        subject: self.message_id.clone(),
                    }),
                    false => Ok(()),
                }
            })
    }

    /// Check a decoded ciphertext length against the prototype's cap
    /// (`crypto.rs:210-212`). The cap is the plaintext maximum plus a 64-byte
    /// AEAD overhead.
    pub fn check_ciphertext_length(&self, decoded_bytes: usize) -> Result<(), Warning> {
        match decoded_bytes <= MAX_MESSAGE_BYTES + 64 {
            true => Ok(()),
            false => Err(Warning::Malformed {
                domain: "envelope-ciphertext",
                detail: decoded_bytes.to_string(),
            }),
        }
    }
}
