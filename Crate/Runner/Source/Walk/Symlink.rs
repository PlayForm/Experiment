//! The symlink policy of the walk.
//!
//! Grounded in `Fn/Binary/Command/Entry.rs:46`: the SUT calls
//! `follow_links(false)` before iterating, so links are never traversed.
//! This is a typed value so the benchmark cannot drift to `true` silently.

/// Whether `WalkDir` follows symbolic links.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Symlink {
    /// Do not descend into links (the SUT's fixed behaviour).
    Ignore,
    /// Descend into links (a future variant, not the SUT).
    Follow,
}

impl Symlink {
    /// The SUT's `follow_links(false)` (`Entry.rs:46`).
    pub const DEFAULT: Self = Self::Ignore;

    /// Resolve the boolean passed to `follow_links`.
    #[must_use]
    pub const fn from_follow_links(enabled: bool) -> Self {
        match enabled {
            true => Self::Follow,
            false => Self::Ignore,
        }
    }

    /// Whether links are traversed.
    #[must_use]
    pub const fn follows(self) -> bool {
        matches!(self, Self::Follow)
    }
}
