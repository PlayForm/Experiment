//! Contract revisions and additive migration.
//!
//! Grounded in `docs/00-system-constitution.md:57-62`: every contract carries
//! a version; old attestations and samples remain valid for their old pinned
//! policy and format; new policies do not retroactively change outcomes;
//! migrations are additive; canonical artifacts are immutable and corrections
//! supersede rather than mutate. A revision is a typed value, so a later policy
//! can never silently rewrite an earlier signed result.

use crate::{Digest, Permission, Warning};

/// A pinned contract revision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Revision {
    /// Monotonic version number.
    pub version: u32,
    /// Digest of the signed policy this revision pins.
    pub policy: Digest,
    /// Whether this revision may be applied backward to old outcomes.
    pub retroactive: Permission,
}

impl Revision {
    /// Build a non-retroactive revision from a version and pinned policy digest.
    #[must_use]
    pub const fn of(version: u32, policy: Digest) -> Self {
        Self {
            version,
            policy,
            retroactive: Permission::Deny,
        }
    }

    /// Whether an attestation under `other` remains valid under this revision.
    #[must_use]
    pub fn admits(&self, other: &Self) -> Result<Permission, Warning> {
        match self.retroactive {
            Permission::Allow => Ok(Permission::Allow),
            Permission::Deny => match other.version == self.version {
                true => Ok(Permission::Allow),
                false => Err(Warning::Drift {
                    domain: "contract-revision",
                    expected: self.version,
                    found: other.version,
                }),
            },
        }
    }

    /// Record a superseding revision; the old one is never mutated.
    #[must_use]
    pub fn supersede(&self, version: u32, policy: Digest) -> Result<Self, Warning> {
        match version > self.version {
            true => Ok(Self::of(version, policy)),
            false => Err(Warning::InvalidValue {
                field: "version",
                detail: format!("superseding version {version} must exceed {}", self.version),
            }),
        }
    }
}
