//! Exclusive benchmark and sealed-holdout custodian.
//!
//! Grounded in `SYSTEM.md:53,93-95` (exclusive lease, sealed holdout, agents
//! cannot rewrite fixtures), `MARKET.md:9-13` (holdout exposure is a scientific
//! access quota, never purchasable), `MARKET.md:93-96` (many iterations on one
//! machine are not independent machine-level replications) and
//! `playbooks/research-market-week.yaml:15` (one runner per host).

use crate::case::Holdout;
use crate::Permission;

/// The isolated benchmark authority of a campaign.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Benchmark {
    /// Oracle whose fixtures gate a benchmark run.
    pub oracle_id: &'static str,
    /// Holdout policy for this case.
    pub holdout: Holdout,
    /// Exclusive runners per host (`playbooks/research-market-week.yaml:15`).
    pub runners_per_host: u8,
}

impl Benchmark {
    /// Construct the benchmark authority model.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            oracle_id: "reference_behavior_v1",
            holdout: Holdout::Sealed,
            runners_per_host: crate::BENCHMARK_RUNNERS_PER_HOST,
        }
    }

    /// Holdout exposure cannot be bought (`MARKET.md:11-13`).
    #[must_use]
    pub const fn may_be_purchased() -> Permission {
        Permission::Deny
    }

    /// Researchers cannot rewrite fixtures or the measurement protocol
    /// (`SYSTEM.md:93-95`, `AGENTS.md:1`).
    #[must_use]
    pub const fn may_be_altered_by_agent() -> Permission {
        Permission::Deny
    }

    /// Repeated timing on the same host is not a machine-level replication
    /// (`MARKET.md:94-95`).
    #[must_use]
    pub const fn is_independent_replication(same_host: bool) -> Permission {
        match same_host {
            true => Permission::Deny,
            false => Permission::Allow,
        }
    }
}
