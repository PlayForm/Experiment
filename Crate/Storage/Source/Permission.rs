//! A typed decision that never silently grants authority.

/// The outcome of a typed guard over schema, ledger, CAS or lease state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Permission {
    /// The operation satisfies the guard.
    Allow,
    /// The operation is refused; a [`crate::warning::Warning`] explains why.
    Deny,
}

impl Permission {
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

    /// Name the decision for diagnostics.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
        }
    }
}
