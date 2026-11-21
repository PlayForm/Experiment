//! The offline root custody boundary.
//!
//! Grounded in `security/src/storage.rs:124-162` (init creates `offline/` with
//! mode 0o700 and places the message-authority key and TLS root key inside),
//! `crypto.rs:104`, `ssh_ca.rs:24`, `security/README.md:31-34`, and
//! `main.rs:1-3` (the relay code path never loads the root secret).
//!
//! This is a typed record of the custody rule. It generates no key, holds no
//! secret, and provides no protection guarantee.

use crate::permission::Permission;

/// A typed record of the offline root's location and custody rule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Root {
    /// Path of the Ed25519 message-authority signing key inside `offline/`.
    pub authority_key: &'static str,
    /// Path of the TLS root CA private key inside `offline/`.
    pub tls_key: &'static str,
    /// Path of the OpenSSH CA key inside `offline/`.
    pub ssh_key: &'static str,
    /// Unix mode applied to the offline directory (`storage.rs:129-130`).
    pub directory_mode: u32,
}

impl Root {
    /// The message-authority key path (`crypto.rs:104`, `storage.rs:66`).
    pub const AUTHORITY_KEY: &'static str = "offline/message-authority.hex";
    /// The TLS root key path (`storage.rs:151`).
    pub const TLS_KEY: &'static str = "offline/tls-root.key.pem";
    /// The SSH CA key path (`ssh_ca.rs:24`).
    pub const SSH_KEY: &'static str = "offline/ssh-ca.key";
    /// The offline directory mode (`storage.rs:130`).
    pub const MODE: u32 = 0o700;

    /// Construct the prototype's offline root definition.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            authority_key: Self::AUTHORITY_KEY,
            tls_key: Self::TLS_KEY,
            ssh_key: Self::SSH_KEY,
            directory_mode: Self::MODE,
        }
    }

    /// The root secret may never leave offline custody into a relay or agent.
    /// This is the invariant from `README.md:31` and `compose.dev.yaml:2`;
    /// no enforcement is performed here.
    #[must_use]
    pub const fn may_leave_offline() -> Permission {
        Permission::Deny
    }

    /// Whether a named key path belongs to offline custody.
    #[must_use]
    pub fn is_offline_path(&self, path: &str) -> bool {
        path == self.authority_key || path == self.tls_key || path == self.ssh_key
    }
}
