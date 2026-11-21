//! The restricted relay-only staging view.
//!
//! Grounded in `security/src/storage.rs:248-294` (`stage_runtime`) and
//! `security/README.md:31-34`: the staged bundle carries only public trust
//! anchors and one service's TLS certificate, private TLS key and signing key.
//! The offline CA, other identities' secret keys, HPKE decryption keys and
//! replay secrets are never copied. No file is copied here.

use crate::warning::Warning;

/// A typed description of what staging copies and what it excludes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stage {
    /// Public files copied from `public/` (`storage.rs:261-266`).
    pub public_files: [&'static str; 3],
    /// Identity files copied for the one local identity (`storage.rs:281`).
    pub identity_files: [&'static str; 3],
    /// Secret material the relay view must never contain (`storage.rs:248-250`).
    pub excluded: [&'static str; 3],
}

impl Stage {
    /// Construct the prototype's staging contract.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            public_files: ["tls-root.cert.pem", "message-authority.hex", "ssh-ca.pub"],
            identity_files: ["tls.cert.pem", "tls.key.pem", "sign.secret.hex"],
            excluded: ["offline", "encrypt.secret.b64", "replay"],
        }
    }

    /// Refuse an identity file name that is not in the relay allow-list.
    /// An unknown name degrades with a warning rather than being copied.
    pub fn copy_decision(&self, name: &str) -> Result<(), Warning> {
        match self.identity_files.iter().any(|allowed| *allowed == name) {
            true => Ok(()),
            false => Err(Warning::Malformed {
                domain: "stage-allow-list",
                detail: name.to_owned(),
            }),
        }
    }

    /// The staged TLS private key mode (`storage.rs:286-291`): 0o600.
    #[must_use]
    pub const fn tls_key_mode() -> u32 {
        0o600
    }
}
