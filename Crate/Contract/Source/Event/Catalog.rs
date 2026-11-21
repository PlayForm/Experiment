//! The signed event catalog and its correspondence network.
//!
//! Bundles the `87` kind entries, the 15 AsyncAPI channels and the 29
//! authorized transfers. All lookups return borrowed or owned typed values and
//! degrade to `None` or an empty slice for an unknown identifier; nothing
//! panics on a missing reference.

/// The v6.1 event catalog.
pub struct Catalog {
    entries: Vec<crate::event::Entry>,
    transfers: Vec<crate::event::Correspondence>,
}

impl Catalog {
    /// Build the canonical event catalog.
    #[must_use]
    pub fn of() -> Self {
        Self {
            entries: crate::event::Entry::all(),
            transfers: crate::event::Correspondence::all(),
        }
    }

    /// Number of signed event kinds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the catalog is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Look up one entry by kind.
    #[must_use]
    pub fn find(&self, kind: crate::event::Kind) -> Option<&crate::event::Entry> {
        self.entries.iter().find(|entry| entry.kind == kind)
    }

    /// Iterate over all entries.
    pub fn iter(&self) -> impl Iterator<Item = &crate::event::Entry> {
        self.entries.iter()
    }

    /// All payload schema references, for structural validation.
    #[must_use]
    pub fn references(&self) -> Vec<&'static str> {
        self.entries.iter().map(|entry| entry.schema).collect()
    }

    /// All authorized correspondence transfers.
    #[must_use]
    pub fn transfers(&self) -> &[crate::event::Correspondence] {
        &self.transfers
    }

    /// Kinds declared more than once, for structural findings.
    #[must_use]
    pub fn duplicates(&self) -> Vec<crate::event::Kind> {
        self.entries
            .iter()
            .map(|entry| entry.kind)
            .fold((Vec::new(), Vec::new()), |(seen, dupes), kind| match seen.contains(&kind) {
                true => (seen, [dupes, vec![kind]].concat()),
                false => ([seen, vec![kind]].concat(), dupes),
            })
            .1
    }
}
