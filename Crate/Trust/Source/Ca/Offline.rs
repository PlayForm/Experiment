//! The offline directory and its isolation rule.
//!
//! Grounded in `security/src/storage.rs:124-162` (`init` creates `offline/`
//! and stores the signing and TLS root keys there), `security/README.md:31-34`
//! and `deploy/compose.dev.yaml:1-2` (the offline directory is never mounted
//! into a relay or agent container). A failed guard degrades via a warning.

use crate::{ca::root::Root, permission::Permission, warning::Warning};

/// The offline CA directory as a typed custody value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Offline {
    /// The root definition held in this directory.
    pub root: Root,
    /// Unix mode applied at creation (`storage.rs:129-130`).
    pub mode: u32,
}

impl Offline {
    /// Directory name (`storage.rs:128`).
    pub const DIRECTORY: &'static str = "offline";

    /// Construct the prototype's offline definition without touching a file.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            root: Root::prototype(),
            mode: Root::MODE,
        }
    }

    /// Decide whether a named view may mount the offline directory. Only the
    /// host-custody zone is permitted; relay and agent views are refused with
    /// a clear warning instead of a panic (`README.md:31`, `compose.dev.yaml:2`).
    pub fn mount_decision(view: &str) -> Result<Permission, Warning> {
        match view {
            "host-custody" => Ok(Permission::Allow),
            _ => Err(Warning::OfflineMountAttempt { view: "relay/agent" }),
        }
    }
}
