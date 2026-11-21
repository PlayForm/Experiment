//! Who may know what about a failure.
//!
//! Worker-blind enforcement removes confidential detail but keeps operational
//! status fully observable to authorized humans. Exact timing
//! indistinguishability is not promised, and a verification pass is never
//! forged to hide a failure (`docs/22:47-69`, `docs/28:34-37`).

use crate::permission::Permission;

/// The audience a failure detail is allowed to reach.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Disclosure {
    /// Nothing is emitted to the worker; the fact stays in restricted audit.
    None,
    /// A fixed generic status common to a whole failure class.
    Generic,
    /// Bounded local compiler or test output explicitly allowed.
    Bounded,
    /// Authorized operator view only.
    Restricted,
    /// A public signed statement with no private detail.
    Public,
}

impl Disclosure {
    /// Whether this class may reach an offline worker.
    #[must_use]
    pub fn worker_safe(&self) -> Permission {
        match self {
            Self::None | Self::Generic | Self::Bounded => Permission::Allow,
            Self::Restricted | Self::Public => Permission::Deny,
        }
    }

    /// Whether the detail carries a reason code or peer identity.
    #[must_use]
    pub fn carries_reason(&self) -> bool {
        match self {
            Self::None | Self::Generic | Self::Bounded | Self::Public => false,
            Self::Restricted => true,
        }
    }
}
