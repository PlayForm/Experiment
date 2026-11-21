//! The aggregate machine-readable contract catalog.
//!
//! This is the typed name `contract::catalog::Catalog` that the laboratory core
//! records as its downstream contract owner
//! (`Crate/Laboratory/Source/lib.rs:121`). It bundles the six surfaces reworked
//! by this card without importing any sibling crate: coupling is recorded as
//! typed path names in `lib.rs`, never as a Rust dependency.

/// The complete v6.1 contract as typed data.
pub struct Catalog {
    /// External and internal operation catalog.
    pub api: crate::api::Index,
    /// Signed event kinds and correspondence rules.
    pub events: crate::event::Catalog,
    /// Schema family descriptors.
    pub schemas: Vec<crate::schema::Definition>,
    /// Typed example registry.
    pub examples: Vec<crate::example::Example>,
    /// Pinned, versioned authorization data.
    pub policy: crate::policy::AccessMatrix,
}

impl Catalog {
    /// Build the canonical v6.1 catalog from typed descriptors.
    #[must_use]
    pub fn of() -> Self {
        Self {
            api: crate::api::Index::of(),
            events: crate::event::Catalog::of(),
            schemas: crate::schema::Definition::of(),
            examples: crate::example::Example::of(),
            policy: crate::policy::AccessMatrix::of(),
        }
    }

    /// Structural validation entry point. The caller owns execution; this
    /// method only assembles a report from typed findings and never panics on
    /// a missing reference.
    #[must_use]
    pub fn inspect(&self) -> crate::validate::Report {
        crate::validate::Validator::of().validate(self)
    }
}
