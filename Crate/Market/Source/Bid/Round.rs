//! One bounded allocation epoch with a two-step assignment order.
//!
//! Grounded in `MARKET.md:40-47` (first reserve guaranteed minimum capacity
//! for contrarian/replication tracks and underrepresented approaches, then
//! rank eligible bids for remaining capacity), `:44-47` (global, per-project,
//! per-agent, per-model and per-topic caps plus cold-start exploration),
//! `:117-123` (epochs, expiry unless rolled over, return to treasury, no
//! recursive self-spawning) and `rust/market-prototype/src/main.rs:95-159`.
//! Nothing is scheduled or reserved; the round is a typed policy definition.

use crate::{case::Stance,credit::Resource,Permission,Warning};

/// A bounded allocation epoch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Round {
    /// Native capacity available this epoch (`main.rs:76`).
    pub capacity: Resource,
    /// Stances with guaranteed minimum capacity (`MARKET.md:41`).
    pub mandatory: Vec<Stance>,
    /// Per-model-family cap (`main.rs:79`).
    pub max_per_model_cohort: usize,
    /// Per-mechanism cap (`main.rs:80`).
    pub max_per_mechanism: usize,
    /// Epoch length in hours (`MARKET.md:118`).
    pub epoch_hours: u32,
}

impl Round {
    /// The two-step order is fixed, never inferred from a model's ranking.
    pub const STEPS: [&'static str; 2] = [
        "reserve_guaranteed_minimum_capacity",
        "rank_eligible_bids_for_remaining_capacity",
    ];

    /// Validate positive caps and a positive epoch (`main.rs:97-99`).
    pub fn validate(&self) -> Result<(), Warning> {
        match (
            self.max_per_model_cohort > 0,
            self.max_per_mechanism > 0,
            self.epoch_hours > 0,
        ) {
            (true, true, true) => Ok(()),
            _ => Err(Warning::InvalidValue {
                field: "allocation_caps",
                detail: "caps and epoch must be positive".to_owned(),
            }),
        }
    }

    /// Protected stances enter the first tier (`MARKET.md:41`, `main.rs:117-126`).
    #[must_use]
    pub fn tier(&self, stance: Stance) -> u8 {
        match self.mandatory.contains(&stance) {
            true => 0,
            false => 1,
        }
    }

    /// Budget expires at the epoch boundary unless a signed rollover exists
    /// (`MARKET.md:118-119`, `sql/v5_migration.sql:164`).
    #[must_use]
    pub const fn may_roll_over_without_policy() -> Permission {
        Permission::Deny
    }

    /// No recursive self-spawning without signed admission (`MARKET.md:123`,
    /// `playbooks/research-market-week.yaml:55`).
    #[must_use]
    pub const fn may_self_spawn() -> Permission {
        Permission::Deny
    }
}
