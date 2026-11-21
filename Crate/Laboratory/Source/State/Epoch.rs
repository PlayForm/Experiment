//! Pinned policy epochs and stale-epoch fencing.
//!
//! Grounded in `docs/23-interface-inventory.md:75-88` (every POST carries an
//! expected policy epoch and bounded expiry), `docs/25-failure-analysis.md` F10
//! (conflicting policy epochs are fenced and quarantined) and
//! `docs/06-state-machines.md:80-84` (validate scope and policy digest before
//! appending). A stale epoch never silently permits an operation.

use crate::{Digest, Permission};

/// A versioned policy epoch bound to a signed policy digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Epoch {
    /// Monotonic epoch number.
    pub number: u64,
    /// Digest of the pinned policy for this epoch.
    pub digest: Digest,
}

impl Epoch {
    /// Pin a new epoch to a policy digest.
    #[must_use]
    pub const fn of(number: u64, digest: Digest) -> Self {
        Self { number, digest }
    }

    /// Whether a caller's claimed epoch matches the pinned one.
    #[must_use]
    pub fn accepts(&self, claimed: &Self) -> Permission {
        match (self.number == claimed.number, self.digest == claimed.digest) {
            (true, true) => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// A stale claim produces a typed warning, never a silent permission.
    #[must_use]
    pub fn check(&self, claimed: &Self) -> Result<Permission, crate::Warning> {
        match self.accepts(claimed) {
            Permission::Allow => Ok(Permission::Allow),
            Permission::Deny => Err(crate::Warning::StaleEpoch {
                expected: self.number,
                found: claimed.number,
            }),
        }
    }
}
