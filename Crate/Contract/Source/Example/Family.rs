//! Example families, including the `page-v61` projection family.
//!
//! The corpus has base, page, `page-v61` and `v61` example stems. `page-v61`
//! examples are metadata projections of v6.1 records and remain in the v6.1
//! namespace (`examples/page-v61-custody-request.example.json`,
//! `docs/23-interface-inventory.md:17-35`).

/// Family of an example document.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Family {
    /// Base v6 record example.
    Base,
    /// Base paged projection example.
    Page,
    /// v6.1 record example.
    V61,
    /// Paged v6.1 projection example.
    PageV61,
}

impl Family {
    /// Inventory spelling of the family.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Page => "page",
            Self::V61 => "v61",
            Self::PageV61 => "page-v61",
        }
    }

    /// The schema family that backs this example family.
    #[must_use]
    pub const fn schema_family(self) -> crate::schema::Family {
        match self {
            Self::Base => crate::schema::Family::Base,
            Self::Page => crate::schema::Family::Page,
            Self::V61 | Self::PageV61 => crate::schema::Family::V61,
        }
    }
}
