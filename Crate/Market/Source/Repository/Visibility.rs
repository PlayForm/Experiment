//! Repository visibility class of an adapter.
//!
//! Grounded in `repository-adapter.v5.schema.json:53-55` and
//! `REPOSITORIES.md:13,71-75`: public sources may be admitted with owner
//! consent; private and internal sources never auto-publish.

use crate::Warning;

/// Visibility of the adapted repository.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Visibility {
    /// Public source (`schema:54`).
    Public,
    /// Private source (`schema:54`).
    Private,
    /// Internal-only source (`schema:54`).
    Internal,
}

impl Visibility {
    /// All visibility classes.
    pub const ALL: [Self; 3] = [Self::Public, Self::Private, Self::Internal];

    /// Decode the schema label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "public" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            "internal" => Ok(Self::Internal),
            other => Err(Warning::InvalidValue {
                field: "visibility",
                detail: format!("unknown class {other}"),
            }),
        }
    }

    /// The wire label of this class.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Internal => "internal",
        }
    }

    /// No unknown repository auto-publishes (`PUBLISHING.md:65-66`).
    #[must_use]
    pub const fn auto_publish() -> crate::Permission {
        crate::Permission::Deny
    }
}
