//! Schema families: base, page projections and v6.1 records.
//!
//! Base schemas use the `https://schemas.lab.invalid/v6/` namespace, v6.1
//! schemas use `https://schemas.lab.invalid/v6.1/`, and `page-*` schemas are
//! collection projections over a base or v6.1 record
//! (`schemas/page.schema.json:1-23`, `schemas/v61-custody-request.schema.json:3`).

/// A schema family.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Family {
    /// Base v6 record schema.
    Base,
    /// Paged projection schema.
    Page,
    /// v6.1 custody/evolution/licensing record schema.
    V61,
}

impl Family {
    /// Wire spelling used in inventory tables.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Page => "page",
            Self::V61 => "v61",
        }
    }

    /// Canonical `$id` namespace for this family.
    #[must_use]
    pub const fn namespace(self) -> &'static str {
        match self {
            Self::Base | Self::Page => "https://schemas.lab.invalid/v6/",
            Self::V61 => "https://schemas.lab.invalid/v6.1/",
        }
    }

    /// Classify a schema stem.
    #[must_use]
    pub fn from_stem(stem: &str) -> Self {
        match (stem.starts_with("page-v61-"), stem.starts_with("v61-"), stem.starts_with("page-")) {
            (true, _, _) | (_, true, _) => Self::V61,
            (_, _, true) => Self::Page,
            _ => Self::Base,
        }
    }
}
