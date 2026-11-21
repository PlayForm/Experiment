//! Additive-only compatibility with the v6.1 operator surface.
//!
//! `docs/10:1-9` replaces no v6.1 endpoint, event, task schema or SQL table;
//! it adds plane -1 and a generic encrypted transport. The public operator
//! API stays version-compatible and may still return names for authorized
//! views (`README.md:8-9`). `docs/05:26-27` retains old schema readers.

use crate::verdict::Verdict;

/// One compatibility classification of a proposed change.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Compatibility {
    /// Adds a plane or operation without changing an existing contract.
    Additive,
    /// Keeps the external version and behavior unchanged.
    VersionCompatible,
    /// Replaces or removes an existing contract.
    Replacing,
}

impl Compatibility {
    /// Whether the proposed change is acceptable.
    #[must_use]
    pub const fn accepted(&self) -> Verdict {
        match self {
            Self::Additive | Self::VersionCompatible => Verdict::Allow,
            Self::Replacing => Verdict::Deny,
        }
    }

    /// Old schema readers are retained until retention expires
    /// (`docs/05:26-27`).
    #[must_use]
    pub const fn retains_old_readers() -> bool {
        true
    }
}
