//! One typed allocation decision under resource constraints.
//!
//! Grounded in `rust/market-prototype/src/main.rs:82-93,131-158` (a decision
//! names bid ID, accepted flag and reason; a plan carries used and remaining
//! vectors), `MARKET.md:40-47` (two-step assignment) and `MARKET.md:75-79` (a
//! reservation decreases available capacity atomically and settlement consumes
//! only actual spend). The boolean accepted flag is modelled as a
//! [`Permission`](crate::Permission), keeping the decision a typed value.

use crate::{credit::Resource, Permission};

use super::Reason;

/// The decision recorded for one bid in an allocation round.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Allocation {
    /// Bid this decision concerns (`main.rs:84`).
    pub bid_id: String,
    /// Allow means reserved in the deterministic plan; Deny means refused.
    pub decision: Permission,
    /// Why the decision was made (`main.rs:86`).
    pub reason: Reason,
    /// Upper bound reserved when accepted (`main.rs:143`).
    pub reserved: Resource,
}

impl Allocation {
    /// Record a decision value; no capacity is actually changed.
    #[must_use]
    pub const fn of(bid_id: String, reason: Reason, reserved: Resource) -> Self {
        let decision = match reason.is_acceptance() {
            true => Permission::Allow,
            false => Permission::Deny,
        };
        Self {
            bid_id,
            decision,
            reason,
            reserved,
        }
    }

    /// Whether this decision reserved any capacity.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        self.decision.as_bool()
    }

    /// The plan's used vector is the sum of accepted upper bounds; settlement
    /// may later return the unused part (`MARKET.md:77`).
    #[must_use]
    pub const fn reserved_upper_bound(&self) -> Resource {
        self.reserved
    }
}
