//! An identity-bound, encrypted private research notebook.
//!
//! Grounded in `SYSTEM.md:67-70` (raw notebooks never connect directly to the
//! public library, evaluator or sibling researchers), `PUBLISHING.md:34-46`
//! (encrypted per researcher; authors submit explicit capsules to a privileged
//! publisher with no universal decryption key), `AGENTS.md:37-43` (signed
//! context snapshots and access-controlled artifacts) and
//! `V4_TRUST_BRIDGE.md:37-41` (a researcher may spend tokens locally without
//! making every token public). No notebook is opened or decrypted here.

use crate::{case::Privacy, Digest, Permission};

/// A private notebook owned by one researcher identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Notebook {
    /// Owning researcher identity (`SYSTEM.md:67`).
    pub owner_identity: String,
    /// Digest of the recipient-encrypted capsule (`AGENTS.md:38`).
    pub encrypted_capsule: Digest,
    /// Visibility class of this notebook (`schema privacy`, `SYSTEM.md:53`).
    pub visibility: Privacy,
}

impl Notebook {
    /// Construct a private notebook value.
    #[must_use]
    pub const fn of(owner_identity: String, encrypted_capsule: Digest) -> Self {
        Self {
            owner_identity,
            encrypted_capsule,
            visibility: Privacy::Private,
        }
    }

    /// Raw notebooks never connect to the public library (`SYSTEM.md:67-70`).
    #[must_use]
    pub const fn may_connect_to_library() -> Permission {
        Permission::Deny
    }

    /// Raw notebooks never connect to the evaluator (`SYSTEM.md:67-70`).
    #[must_use]
    pub const fn may_connect_to_evaluator() -> Permission {
        Permission::Deny
    }

    /// Siblings cannot query a peer's drafts by API, worktree, logs or
    /// telemetry (`OPERATIONS.md:52`).
    #[must_use]
    pub const fn may_connect_to_sibling() -> Permission {
        Permission::Deny
    }

    /// An explicit, recipient-encrypted capsule may be submitted to the scoped
    /// publisher (`PUBLISHING.md:42-46`, `V4_TRUST_BRIDGE.md:38-39`).
    #[must_use]
    pub const fn may_submit_capsule_to_publisher() -> Permission {
        Permission::Allow
    }
}
