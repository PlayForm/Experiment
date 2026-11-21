//! Availability of the optional OpenPGP helper.
//!
//! Grounded in `security/Cargo.toml:8-16` (`openpgp` feature maps to the
//! optional `pgp-compat` dependency), `main.rs:5-6,35-43` (verbs exist only
//! with that feature) and `openpgp.rs:1-3` (import and verify keys through the
//! CA registry first). A disabled or missing helper must degrade, not panic.

use crate::{permission::Permission, warning::Warning};

/// The optional helper's typed availability contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Helper {
    /// Cargo feature that enables the helper (`Cargo.toml:10`).
    pub feature: &'static str,
    /// Whether it is the native message protocol (`openpgp.rs:1-2`).
    pub native_protocol: Permission,
    /// The registry that must vouch for a public key (`openpgp.rs:2-3`).
    pub trust_gate: &'static str,
}

impl Helper {
    /// Construct the prototype's helper definition.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            feature: "openpgp",
            native_protocol: Permission::Deny,
            trust_gate: "CA identity registry",
        }
    }

    /// Combine the feature flag with availability, denying by default.
    #[must_use]
    pub const fn availability(feature_enabled: Permission) -> Permission {
        Permission::Deny.and(feature_enabled)
    }

    /// A dead or absent helper yields a warning rather than a panic.
    #[must_use]
    pub const fn unavailable() -> Warning {
        Warning::DeadSubprocess { helper: "openpgp" }
    }
}
