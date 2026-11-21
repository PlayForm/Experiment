//! Visibility classes for feedback and knowledge.
//! Grounded in `17-v61-constitution.md:45-51` (fast private versus slow
//! sanctioned cross-repository knowledge) and `PUBLISHING.md:13-32` (the
//! publication status ladder from hypothesis to sanitized preprint).

/// How widely a piece of feedback or knowledge may travel.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Visibility {
    /// Private to the owning worker's lineage.
    Private,
    /// Sanitized and evidence-linked inside the campaign.
    Sanitized,
    /// Sanctioned for cross-repository transfer.
    Public,
}

impl Visibility {
    /// Whether this visibility permits cross-repository transfer.
    #[must_use]
    pub const fn allows_cross_repo(self) -> bool {
        matches!(self, Self::Public)
    }

    /// Whether this visibility exposes sibling data.
    #[must_use]
    pub const fn exposes_siblings(self) -> bool {
        false
    }

    /// Stable key for the release manifest.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::Sanitized => "sanitized",
            Self::Public => "public",
        }
    }
}
