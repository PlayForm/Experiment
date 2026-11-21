//! Authorized embargo release, never a retroactive grant.
//!
//! `release_after` and owner authorization are both required, and a change to
//! an embargo affects only future deliveries and key uses
//! (`docs/20:35-36`, `docs/21:52-55`, `docs/25:26`).

use crate::permission::Permission;
use crate::switch::Switch;

/// A versioned decision to end an embargo for future accesses.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Release {
    /// `release_id` of the signed decision.
    pub release_id: String,
    /// `embargo_id` the decision refers to.
    pub embargo_id: String,
    /// Earliest instant at which future access may proceed.
    pub allowed_at: String,
    /// Instant at which the owner authorized the change.
    pub authorized_at: String,
    /// Owner authorization state.
    pub owner_approved: Switch,
    /// Digest of the policy that made the change.
    pub policy_digest: String,
}

impl Release {
    /// Whether the release authorizes a future access at `at`.
    #[must_use]
    pub fn permits(&self, at: &str) -> Permission {
        Permission::filled(&self.release_id)
            .and(self.owner_approved.permit())
            .and(Permission::digest(&self.policy_digest))
            .and(Permission::of(
                !self.authorized_at.is_empty()
                    && self.authorized_at <= at
                    && self.allowed_at <= at.to_owned(),
            ))
    }

    /// Whether an access already made before the change stays as it was.
    #[must_use]
    pub fn retroactive(&self) -> bool {
        match self.owner_approved {
            Switch::On | Switch::Off => false,
        }
    }
}
