//! Multi-resource treasury: immutable authorizations and settlements.
//!
//! Grounded in `MARKET.md:70-79`: the treasury emits immutable authorization,
//! reservation, usage, settlement, refund, expiration and adjustment records,
//! each with a causal transaction ID, signed actor and rationale; consumption
//! is metered by trusted software, not reported by the agent. No agent can
//! transfer, mint, lend, forgive or buy a verdict. `sql/v5_migration.sql:110-149`
//! and its immutability triggers `:267-278` back this model. Nothing is booked
//! here.

use crate::Permission;

/// The owner-funded, non-transferable treasury of one campaign.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Treasury {
    /// Treasury identifier (`sql/v5_migration.sql:111`).
    pub treasury_id: &'static str,
    /// Digest of the versioned allocation policy (`:114`).
    pub policy_sha256: &'static str,
}

impl Treasury {
    /// The immutable transaction ledger this treasury writes to, by typed name.
    pub const LEDGER_PATH: &'static str = "crate::credit::Ledger";

    /// Construct the treasury model.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            treasury_id: "treasury_id",
            policy_sha256: "policy_sha256",
        }
    }

    /// Agents cannot mint credits (`MARKET.md:79`).
    #[must_use]
    pub const fn agent_may_mint() -> Permission {
        Permission::Deny
    }

    /// Credit is non-transferable, not a wallet (`README.md:50-52`, `MARKET.md:79`).
    #[must_use]
    pub const fn agent_may_transfer() -> Permission {
        Permission::Deny
    }

    /// No lending, forgiving or debt rearrangement (`MARKET.md:79`).
    #[must_use]
    pub const fn agent_may_lend_or_forgive() -> Permission {
        Permission::Deny
    }

    /// A benchmark verdict cannot be bought at any price (`MARKET.md:13,79`).
    #[must_use]
    pub const fn agent_may_buy_verdict() -> Permission {
        Permission::Deny
    }

    /// Actual consumption comes from trusted metering (`MARKET.md:74`).
    #[must_use]
    pub const fn agent_may_report_own_spend() -> Permission {
        Permission::Deny
    }
}
