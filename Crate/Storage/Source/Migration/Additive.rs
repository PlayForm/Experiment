//! Classification of a schema change by additivity.

use crate::permission::Permission;

/// Whether a change preserves every table and column declared before it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Additive {
    /// The change only creates new schema objects.
    Additive,
    /// The change alters or removes existing schema: report, never apply.
    Destructive,
}

impl Additive {
    /// Conjoin two classifications without a branch at the call site.
    #[must_use]
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Additive, Self::Additive) => Self::Additive,
            _ => Self::Destructive,
        }
    }

    /// Whether a trusted applier may take this step.
    #[must_use]
    pub fn permits(&self) -> Permission {
        match self {
            Self::Additive => Permission::Allow,
            Self::Destructive => Permission::Deny,
        }
    }

    /// The classification label.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Additive => "additive",
            Self::Destructive => "destructive",
        }
    }
}
