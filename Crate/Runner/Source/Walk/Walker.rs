//! The walker configuration and its two-stage path filter.
//!
//! Grounded in `Fn/Binary/Command/Entry.rs:29-52`: exclusion is applied by
//! `filter_entry` before `filter_map(Result::ok)`, then the `-F` file filter;
//! the surviving paths are collected in walk order.

use crate::cli::file::File;
use crate::walk::{entry::Entry, exclusion::Exclusion, symlink::Symlink};

/// A walk that can be evaluated against a pre-enumerated candidate list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Walker {
    root: String,
    follow: Symlink,
    file: File,
    exclusions: Exclusion,
}

impl Walker {
    /// Assemble the walk configuration from the resolved CLI parts.
    #[must_use]
    pub fn build(root: &str, follow: Symlink, file: File, exclusions: Exclusion) -> Self {
        Self {
            root: root.to_owned(),
            follow,
            file,
            exclusions,
        }
    }

    /// The start directory (`WalkDir::new(&Option.Root)`, `Entry.rs:45`).
    #[must_use]
    pub fn root(&self) -> &str {
        &self.root
    }

    /// The link policy (`Entry.rs:46`).
    #[must_use]
    pub const fn follow(&self) -> Symlink {
        self.follow
    }

    /// Exclusion first, then the `-F` filter: the SUT's two-stage pipeline
    /// (`Entry.rs:48,50`) expressed as one structural decision.
    #[must_use]
    pub fn accepts(&self, path: &str, is_file: bool) -> bool {
        match self.exclusions.is_excluded(path) {
            true => false,
            false => match self.file.targets_files() {
                true => is_file,
                false => true,
            },
        }
    }

    /// Collect surviving candidates in input order (`Entry.rs:51-52`).
    #[must_use]
    pub fn collect(&self, candidates: &[Entry]) -> Vec<Entry> {
        candidates
            .iter()
            .filter(|entry| self.accepts(entry.path(), entry.is_file()))
            .cloned()
            .collect()
    }
}
