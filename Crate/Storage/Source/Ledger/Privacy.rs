//! The four privacy classes accepted by the v6 journal.

/// Privacy class of a journaled event (`sql/v6_migration.sql:21-28`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Privacy {
    /// Public metadata only.
    PublicMeta,
    /// Internal metadata, not exposed on the public plane.
    InternalMeta,
    /// Restricted metadata for an authorized tenant.
    Restricted,
    /// Sealed content; only an approved metadata shape is disclosed.
    Sealed,
}

impl Privacy {
    /// The exact declared value.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::PublicMeta => "public-meta",
            Self::InternalMeta => "internal-meta",
            Self::Restricted => "restricted",
            Self::Sealed => "sealed",
        }
    }

    /// Whether an event of this class may appear in a public metadata view.
    #[must_use]
    pub fn public_view(&self) -> crate::permission::Permission {
        match self {
            Self::PublicMeta => crate::permission::Permission::Allow,
            Self::InternalMeta | Self::Restricted | Self::Sealed => {
                crate::permission::Permission::Deny
            }
        }
    }
}
