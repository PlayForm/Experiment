//! A typed guard result that never silently grants authority.
//!
//! Deny is the default (`policy/v61-roles.json:2,68`), and an unknown enum or
//! an unreachable policy service maps to [`Permission::Deny`], never to an
//! implicit allow (`docs/25:19`).

use crate::digest::Digest;

/// The outcome of a typed guard over custody, rights, embargo or worker state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Permission {
    /// The guarded operation satisfies every rule.
    Allow,
    /// The guarded operation is refused; an operator warning explains why.
    Deny,
}

impl Permission {
    /// Convert a boolean at the one place a boolean becomes a decision.
    #[must_use]
    pub fn of(condition: bool) -> Self {
        match condition {
            true => Self::Allow,
            false => Self::Deny,
        }
    }

    /// A non-empty field is structurally admissible.
    #[must_use]
    pub fn filled(text: &str) -> Self {
        Self::of(!text.is_empty())
    }

    /// A digest is admissible only when it has the pinned shape.
    #[must_use]
    pub fn digest(text: &str) -> Self {
        Self::of(Digest::of(text).is_valid())
    }

    /// Conjoin two guards without a branch at the call site.
    #[must_use]
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Allow, Self::Allow) => Self::Allow,
            (Self::Allow, Self::Deny) | (Self::Deny, Self::Allow) | (Self::Deny, Self::Deny) => {
                Self::Deny
            }
        }
    }

    /// Override is never available: explicit deny outranks explicit permit.
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::Deny, _) => Self::Deny,
            (Self::Allow, other) => other,
        }
    }

    /// The stable label used by operator diagnostics.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
        }
    }
}
