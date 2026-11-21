//! A signed policy epoch checked at admission and again before delivery.
//!
//! A conflict between epochs fences the transfer; a stale permit never grants
//! access (`docs/17:46`, `docs/20:35-36`, `docs/21:48-55`,
//! `docs/25:10,15`).

use crate::permission::Permission;

/// One version of the signed policy governing knowledge and custody.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Epoch {
    /// `epoch_id` of the policy version.
    pub epoch_id: String,
    /// Digest of the policy in force during this epoch.
    pub policy_digest: String,
    /// First instant the epoch is active.
    pub from: String,
    /// Optional last instant the epoch is active.
    pub to: Option<String>,
    /// Canonical ledger sequence at which the epoch was issued.
    pub sequence: u64,
}

impl Epoch {
    /// Whether the epoch governs an instant.
    #[must_use]
    pub fn active_at(&self, at: &str) -> Permission {
        let started = Permission::filled(&self.from).and(Permission::of(at >= self.from.as_str()));
        let ended = match &self.to {
            None => Permission::Allow,
            Some(until) => Permission::of(at <= until.as_str()),
        };
        started
            .and(ended)
            .and(Permission::digest(&self.policy_digest))
            .and(Permission::of(self.sequence > 0))
    }

    /// Compare the epoch bound to an operation with the currently active one.
    #[must_use]
    pub fn agrees(&self, bound_digest: &str) -> Permission {
        Permission::of(self.policy_digest == bound_digest)
    }
}
