//! Research exchange: exact-duplicate gate and two-step allocation.
//!
//! Grounded in `SYSTEM.md:85-87` (evaluates signed bids, reserves compute,
//! enforces diversity and publishes receipts; cannot define its own policy or
//! create money), `MARKET.md:40-47` (guaranteed minimum for contrarian and
//! replication tracks first, then calibrated ranking for remaining capacity),
//! `sql/v5_migration.sql:57` (unique assignment plus payload) and
//! `rust/market-prototype/src/main.rs:95-159` (deterministic planning only).

use crate::Warning;

/// The deterministic research exchange model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exchange {
    /// Exchange identifier.
    pub exchange_id: &'static str,
    /// Name of the policy version it enforces but cannot change.
    pub policy_version: &'static str,
}

impl Exchange {
    /// The allocation order is two-step, never a single self-reported ranking.
    pub const STEPS: [&'static str; 2] = [
        "reserve_guaranteed_minimum_capacity",
        "rank_eligible_bids_for_remaining_capacity",
    ];

    /// Construct the exchange model.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            exchange_id: "research_exchange",
            policy_version: "signed_policy_version",
        }
    }

    /// The exchange cannot author its own allocation policy (`SYSTEM.md:87`).
    #[must_use]
    pub const fn may_define_allocation_policy() -> crate::Permission {
        crate::Permission::Deny
    }

    /// The exchange cannot create money or credits (`SYSTEM.md:87`).
    #[must_use]
    pub const fn may_create_credit() -> crate::Permission {
        crate::Permission::Deny
    }

    /// An exact duplicate is refused, not merged (`sql/v5_migration.sql:57`,
    /// `OPERATIONS.md:63-64`: semantic similarity never hard-proves equivalence).
    pub fn check_exact_duplicate(&self, seen: bool, bid_id: &str) -> Result<(), Warning> {
        match seen {
            true => Err(Warning::Duplicate {
                domain: "bid",
                id: bid_id.to_owned(),
            }),
            false => Ok(()),
        }
    }
}
