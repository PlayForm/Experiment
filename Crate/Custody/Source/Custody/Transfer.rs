//! The two-stage transfer decision with idempotency and fencing.
//!
//! Authorization and reservation happen before access; delivery, verification
//! and commit happen after. A stale fence or a repeated nonce quarantines the
//! staged bytes instead of producing a second valid receipt
//! (`docs/17:46`, `docs/19:31-47`, `docs/25:29,33`).

use super::phase::Phase;
use crate::permission::Permission;

/// One guarded edge between two custody phases.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transfer {
    /// Phase before the attempted edge.
    pub from: Phase,
    /// Phase after the attempted edge.
    pub to: Phase,
    /// Idempotency key bound to the request.
    pub key: String,
    /// One-use transfer nonce bound at reservation.
    pub nonce: String,
    /// Current fencing token of the reservation lease.
    pub fence: u64,
    /// Fencing token recorded when the lease was issued.
    pub issued_fence: u64,
}

impl Transfer {
    /// The two and only two externally visible transfer stages.
    #[must_use]
    pub const fn stages() -> [Phase; 2] {
        [Phase::Reserved, Phase::Committed]
    }

    /// Decide one edge without staging bytes or committing anything.
    #[must_use]
    pub fn decide(&self) -> Permission {
        self.from
            .permit(self.to)
            .and(Permission::filled(&self.key))
            .and(Permission::filled(&self.nonce))
            .and(Permission::of(self.fence > 0 && self.fence == self.issued_fence))
    }

    /// Fence an in-flight transfer whose policy epoch or lease went stale.
    #[must_use]
    pub fn stale(&self, recorded_policy: &str, active_policy: &str) -> Permission {
        let matches = match recorded_policy == active_policy {
            true => Permission::Allow,
            false => Permission::Deny,
        };
        matches.and(self.decide())
    }

    /// The phase an accepted edge lands in; rejected edges stay put.
    #[must_use]
    pub fn landing(&self) -> Phase {
        match self.decide() {
            Permission::Allow => self.to,
            Permission::Deny => self.from.fence(),
        }
    }
}
