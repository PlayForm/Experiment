//! Fenced resource lease with an explicit expiry.
//!
//! Grounded in `sql/v5_migration.sql:152-169` (one initial transaction, state,
//! positive fencing token, expiry), `SYSTEM.md:146-148` (signed capability
//! request, coordinator-assigned lease with fencing token, idempotent retry),
//! and `OPERATIONS.md:34,46` (durable outbox, fencing, reclaim only after the
//! worker is no longer authorized to commit). No lease is granted here.

use crate::{Permission, Warning};

use super::LeaseState;

/// A fenced reservation lease projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    /// Reservation identifier (`sql:153`).
    pub reservation_id: &'static str,
    /// Monotonic fencing token; must be positive (`sql:167`).
    pub fencing_token: u64,
    /// Current reservation state (`sql:158-165`).
    pub state: LeaseState,
    /// Expiry timestamp (`sql:168`).
    pub expires_utc: &'static str,
}

impl Lease {
    /// The smallest valid fencing token (`sql:167`: `> 0`).
    pub const MIN_FENCE: u64 = 1;

    /// Validate the lease without granting it.
    pub fn validate(&self) -> Result<(), Warning> {
        match self.fencing_token >= Self::MIN_FENCE {
            true => Ok(()),
            false => Err(Warning::InvalidValue {
                field: "fencing_token",
                detail: "a fencing token must be positive".to_owned(),
            }),
        }
    }

    /// A stale fence must be refused, never retried against a live worker
    /// (`SYSTEM.md:146-148`, `OPERATIONS.md:34`).
    pub fn check_fence(&self, presented: u64) -> Result<(), Warning> {
        self.validate()?;
        match presented == self.fencing_token {
            true => Ok(()),
            false => Err(Warning::Expired {
                subject: self.reservation_id.to_owned(),
            }),
        }
    }

    /// An expired lease may be reclaimed only after authorization is revoked
    /// (`OPERATIONS.md:46`). The first condition is the typed state, the second
    /// is a signed decision supplied by the caller.
    pub fn may_reclaim(&self, still_authorized: Permission) -> Result<Permission, Warning> {
        self.validate()?;
        match (self.state, still_authorized) {
            (LeaseState::Expired, Permission::Deny) => Ok(Permission::Allow),
            _ => Ok(Permission::Deny),
        }
    }
}
