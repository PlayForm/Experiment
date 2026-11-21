//! The four artifact encryption/storage classes.

/// Disclosure class of bytes in the content-addressed store
/// (`docs/08-data-and-storage.md:28-31`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageClass {
    /// Sealed: private plaintext never exposed to an observer.
    Sealed,
    /// Private to one tenant.
    Private,
    /// Internal to the laboratory.
    Internal,
    /// Public metadata or bytes.
    Public,
}

impl StorageClass {
    /// The declared storage class name.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Sealed => "sealed",
            Self::Private => "private",
            Self::Internal => "internal",
            Self::Public => "public",
        }
    }

    /// Whether public metadata may carry this class.
    #[must_use]
    pub fn public_view(&self) -> crate::permission::Permission {
        match self {
            Self::Public => crate::permission::Permission::Allow,
            Self::Sealed | Self::Private | Self::Internal => crate::permission::Permission::Deny,
        }
    }
}
