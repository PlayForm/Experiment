//! Key kinds and encodings used by the prototype.
//!
//! Grounded in `security/src/crypto.rs:85-102` (Ed25519 signing key is a
//! 32-byte hex value, HPKE key is a base64 value), `crypto.rs:81-84`
//! (`fixed_32` rejects anything but 32 bytes), `storage.rs:191-194` (TLS key
//! PEM) and `ssh_ca.rs:28-30` (OpenSSH Ed25519 CA key). This type describes
//! key shape only; no key material is held or generated here.

use crate::fault::Fault;

/// A key kind in the v4 prototype.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Key {
    /// Ed25519 message-signing key, 32 bytes, hex public half
    /// (`crypto.rs:93-97`, `crypto.rs:85-87`).
    Signing,
    /// HPKE X25519 recipient key, 32 bytes, base64 public half
    /// (`crypto.rs:98-102`).
    Encryption,
    /// TLS leaf private key in PEM (`storage.rs:193`).
    Tls,
    /// OpenSSH Ed25519 CA key in OpenSSH format (`ssh_ca.rs:28`).
    Ssh,
}

impl Key {
    /// All key kinds in declaration order.
    pub const ALL: [Self; 4] = [Self::Signing, Self::Encryption, Self::Tls, Self::Ssh];

    /// The fixed key length enforced by `fixed_32` (`crypto.rs:81-84`).
    pub const REQUIRED_BYTES: usize = 32;

    /// External encoding of this key kind.
    #[must_use]
    pub const fn encoding(self) -> &'static str {
        match self {
            Self::Signing => "hex",
            Self::Encryption => "base64",
            Self::Tls => "pem",
            Self::Ssh => "openssh",
        }
    }

    /// Validate a supplied key length, returning a fault instead of panicking
    /// on a wrong-length value (`crypto.rs:81-84`).
    pub fn check_length(found: usize) -> Result<(), Fault> {
        match found == Self::REQUIRED_BYTES {
            true => Ok(()),
            false => Err(Fault::Length {
                field: "key",
                expected: Self::REQUIRED_BYTES,
                found,
            }),
        }
    }

    /// Whether this key is restricted to recipient-side use
    /// (`crypto.rs:1`: recipient-only HPKE).
    #[must_use]
    pub const fn is_recipient_only(self) -> bool {
        matches!(self, Self::Encryption)
    }
}
