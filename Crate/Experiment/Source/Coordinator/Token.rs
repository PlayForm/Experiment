//! Capability token issued by the deterministic coordinator.
//!
//! Grounded in `ARCHITECTURE.md:119-125`, `REQUIREMENTS_V3.md` AGT3-003/
//! AGT3-011 and `STORAGE.md:3`. A token carries the assigned role policy,
//! view checksum, worktree scope and an idempotency key; no capability is
//! implied by conversation membership alone. The check degrades to a warning
//! on expiry rather than panicking.

use crate::coordinator::authority::Authority;
use crate::permission::Permission;
use crate::warning::Warning;

/// A scoped, expiring task capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    /// Stable token identifier, unique per grant.
    pub id: String,
    /// The actor that may exercise this capability.
    pub holder: Authority,
    /// Digest of the authorized source/worktree view.
    pub scope_sha256: String,
    /// Digest of the versioned role policy in force.
    pub role_policy_sha256: String,
    /// Digest of the exact context snapshot sent for the turn.
    pub context_snapshot_sha256: String,
    /// Retry-safe idempotency key for the assigned work order.
    pub idempotency_key: String,
    /// Logical sequence at which this token expires.
    pub deadline_seq: u64,
}

impl Token {
    /// Validate the token at an observed logical sequence.
    #[must_use]
    pub fn validate(&self, observed_seq: u64) -> Result<Permission, Warning> {
        let remaining = self.deadline_seq.saturating_sub(observed_seq);
        match remaining {
            0 => Err(Warning::ExpiredLease {
                work_order: self.id.clone(),
            }),
            _ => Ok(Permission::Allow),
        }
    }

    /// Whether the token authorizes a reasoning edge to edit evidence. It
    /// never does, whatever the scope says.
    #[must_use]
    pub fn may_edit_evidence(&self) -> Permission {
        self.holder.may_edit_evidence()
    }

    /// The exact view digest, so a stale context cannot be silently reused.
    #[must_use]
    pub fn view_sha256(&self) -> &str {
        &self.scope_sha256
    }
}
