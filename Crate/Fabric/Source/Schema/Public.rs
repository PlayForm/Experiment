//! The public spec, safe to read and never an authority.
//!
//! `README.md:56-69` states that hiding source filenames is not a security
//! requirement and a public spec should be safe to read. `docs/05:39-49`
//! permits readable design-time names and a fixed small envelope, while
//! insisting randomness is not a substitute for enforced property checks.

use crate::verdict::Verdict;

/// A readable public specification view of the fabric.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Public {
    /// Fixed envelope field names from `docs/05:47`.
    pub envelope: [&'static str; 4],
    /// Declared protocol revision.
    pub revision: &'static str,
}

impl Public {
    /// The published public view for this protocol revision.
    #[must_use]
    pub const fn current() -> Self {
        Self {
            envelope: crate::PUBLIC_FIELDS,
            revision: crate::PROTOCOL_REVISION,
        }
    }

    /// A public spec is safe to read by design.
    #[must_use]
    pub const fn readable(&self) -> Verdict {
        Verdict::Allow
    }

    /// Randomized names in a public envelope carry no authority
    /// (`docs/01:24-25`, `README.md:22-24`).
    #[must_use]
    pub const fn authority(&self) -> Verdict {
        Verdict::Deny
    }

    /// Whether naming a private field in the readable spec bypasses a
    /// requirement. It does not; enforcement lives in the trusted runtime
    /// (`docs/05:39-46`).
    #[must_use]
    pub const fn bypasses_requirement() -> bool {
        false
    }
}
