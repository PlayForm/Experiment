//! The offline OpenSSH certificate authority.
//!
//! Grounded in `security/src/ssh_ca.rs:23-38`: `initialize` refuses to reuse an
//! existing CA key, generates an Ed25519 `PrivateKey`, stores it with 0600 mode
//! under `offline/ssh-ca.key`, publishes `public/ssh-ca.pub`, and journals
//! `SSH_CA_CREATED` with the public-key fingerprint. No key is created here.

use crate::warning::Warning;

/// A typed description of the SSH CA bootstrap.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ca {
    /// Offline key path (`ssh_ca.rs:24`).
    pub key_path: &'static str,
    /// Public key path (`ssh_ca.rs:30`).
    pub public_path: &'static str,
    /// Key algorithm (`ssh_ca.rs:28`).
    pub algorithm: &'static str,
    /// Secret file mode (`storage.rs:49`).
    pub mode: u32,
    /// Bootstrap audit event (`ssh_ca.rs:31-37`).
    pub audit_event: &'static str,
}

impl Ca {
    /// Construct the prototype's SSH CA definition.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            key_path: "offline/ssh-ca.key",
            public_path: "public/ssh-ca.pub",
            algorithm: "Ed25519",
            mode: 0o600,
            audit_event: "SSH_CA_CREATED",
        }
    }

    /// Bootstrap refuses to overwrite an existing CA key
    /// (`ssh_ca.rs:25-27`); degrade with a collision warning instead of a panic.
    pub fn check_absent(&self, key_exists: bool) -> Result<(), Warning> {
        match key_exists {
            true => Err(Warning::Collision {
                domain: "ssh-ca",
                id: self.key_path.to_owned(),
            }),
            false => Ok(()),
        }
    }
}
