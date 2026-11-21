//! Secret and public write classes with explicit handling warnings.
//!
//! Grounded in `security/src/storage.rs:42-64`: `secret` creates a parent
//! directory, opens with `create_new` and mode 0o600, writes and `sync_all`s;
//! `public` opens with `create_new` and syncs; `mark_once` is a secret write.
//! `storage.rs:248-250` and `main.rs:1-3` state that root secrets and HPKE
//! decryption keys never enter the relay view. No file is written here.

use crate::{permission::Permission, warning::Warning};

/// A write class in the prototype's storage contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Secret {
    /// A confidential file (`storage.rs:42-54`).
    Confidential,
    /// A public trust file (`storage.rs:55-63`).
    Public,
    /// An atomic replay marker (`storage.rs:64`, `crypto.rs:235`).
    ReplayMarker,
}

impl Secret {
    /// All write classes.
    pub const ALL: [Self; 3] = [Self::Confidential, Self::Public, Self::ReplayMarker];

    /// Unix mode for this class, where declared (`storage.rs:49`).
    #[must_use]
    pub const fn mode(self) -> u32 {
        match self {
            Self::Confidential | Self::ReplayMarker => 0o600,
            Self::Public => 0o644,
        }
    }

    /// Writes are create-only, so an existing path degrades to a collision
    /// warning (`storage.rs:47,59`).
    pub fn check_absent(&self, exists: bool, path: &str) -> Result<(), Warning> {
        match exists {
            true => Err(Warning::Collision {
                domain: "storage-create-new",
                id: path.to_owned(),
            }),
            false => Ok(()),
        }
    }

    /// Secrets and replay markers never appear in a relay-only view
    /// (`storage.rs:248-250`, `Ca/Stage.rs`).
    #[must_use]
    pub const fn may_enter_relay_view(self) -> Permission {
        match self {
            Self::Confidential | Self::ReplayMarker => Permission::Deny,
            Self::Public => Permission::Allow,
        }
    }
}
