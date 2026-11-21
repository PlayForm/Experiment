//! Network reachability of a processing domain.
//!
//! An offline worker has `None` with no routes, credentials or service
//! principals mounted (`policy/v61-roles.json:4-12`, `docs/28:1-6`).

/// The network posture a holder may attest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Network {
    /// No route, no resolver, no socket, no credentials.
    None,
    /// Local loopback or local IPC only.
    Local,
    /// Routed service access, reserved for trusted services.
    Routed,
}

impl Network {
    /// Whether this posture satisfies the offline worker boundary.
    #[must_use]
    pub fn offline(&self) -> crate::permission::Permission {
        match self {
            Self::None => crate::permission::Permission::Allow,
            Self::Local | Self::Routed => crate::permission::Permission::Deny,
        }
    }

    /// The wire spelling used by the pinned examples.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Local => "local",
            Self::Routed => "routed",
        }
    }
}
