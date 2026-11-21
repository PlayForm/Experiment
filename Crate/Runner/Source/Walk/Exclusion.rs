//! The exclusion set, including the documented default glob.
//!
//! Grounded in `Fn/Binary/Command/Entry.rs:30-43` (build one `Glob` per
//! pattern into a `GlobSet`, `expect` on failure) and `README.md:118`,
//! `Fn/Binary/Command.rs:54` (the default
//! `**/{node_modules,.git,target,dist,vendor}/**/*`).

use crate::fault::Fault;
use crate::walk::glob::Glob;

/// The ordered set of globs that `filter_entry` rejects against.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exclusion {
    globs: Vec<Glob>,
}

impl Exclusion {
    /// The single default pattern (`README.md:118`, `Command.rs:54`).
    pub const DEFAULT: &'static str = crate::DEFAULT_EXCLUDE;

    /// Compile every pattern; the first malformed one becomes a `Fault`
    /// instead of the SUT's `expect` panic (`Entry.rs:40,43`).
    pub fn build(patterns: impl IntoIterator<Item = String>) -> Result<Self, Fault> {
        let globs = patterns
            .into_iter()
            .map(|pattern| Glob::build(&pattern))
            .collect::<Result<Vec<Glob>, Fault>>()?;

        Ok(Self { globs })
    }

    /// The clap default exclusion set.
    pub fn default_set() -> Result<Self, Fault> {
        Self::build([Self::DEFAULT.to_owned()])
    }

    /// `ExcludeSet.is_match(e.path())` OR-ed across globs (`Entry.rs:48`).
    #[must_use]
    pub fn is_excluded(&self, path: &str) -> bool {
        self.globs.iter().any(|glob| glob.is_match(path))
    }

    /// Number of compiled globs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.globs.len()
    }

    /// True when the set excludes nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.globs.is_empty()
    }
}
