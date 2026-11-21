//! A two-variant capability used where a boolean would hide a decision.
//!
//! The external contract keys (`owner_approved`, `network_mode`,
//! `encryption_at_rest`) are documented on the fields that carry this value.

/// Presence or absence of a governed property.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Switch {
    /// The property is present and was attested by a trusted service.
    On,
    /// The property is absent; the guarded operation fails closed.
    Off,
}

impl Switch {
    /// Read a switch as a guard.
    #[must_use]
    pub fn permit(self) -> crate::permission::Permission {
        match self {
            Self::On => crate::permission::Permission::Allow,
            Self::Off => crate::permission::Permission::Deny,
        }
    }

    /// The wire spelling used by the pinned examples.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::On => "true",
            Self::Off => "false",
        }
    }
}
