//! The playbook budget that bounds the loop.
//! Grounded in `autonomous-week.yaml:9-22` (168 hours, 160 mutations, 3
//! concurrent agents, 2 builds, 1 timing job per host, 3 build attempts,
//! stop at 25 GB free disk) and `ARCHITECTURE.md:131-142` (finite per-lane
//! depth, global cap, token budget, wall clock, disk quota, watchdog).

/// A bounded resource envelope for one campaign or one cycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Budget {
    /// Wall-clock hours authorised for the campaign.
    pub hours: u32,
    /// Maximum number of atomic mutations.
    pub mutations: u32,
    /// Maximum build attempts per mutation.
    pub build_attempts: u32,
    /// Free disk floor in gigabytes below which new builds stop.
    pub disk_floor_gb: u32,
}

impl Budget {
    /// An empty spend accumulator: no cycle has consumed anything yet.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            hours: 0,
            mutations: 0,
            build_attempts: 0,
            disk_floor_gb: 0,
        }
    }

    /// The full playbook budget, sourced from the trusted campaign constant.
    #[must_use]
    pub const fn playbook() -> Self {
        Self {
            hours: campaign::WEEK_HOURS,
            mutations: 160,
            build_attempts: 3,
            disk_floor_gb: 25,
        }
    }

    /// Accumulate a spend into this budget; the disk floor is the stricter of
    /// the two floors.
    #[must_use]
    pub const fn plus(self, other: Self) -> Self {
        Self {
            hours: self.hours + other.hours,
            mutations: self.mutations + other.mutations,
            build_attempts: self.build_attempts + other.build_attempts,
            disk_floor_gb: self.disk_floor_gb.max(other.disk_floor_gb),
        }
    }

    /// Whether accumulated spend has reached either hard cap.
    #[must_use]
    pub const fn exhausted(self, used: Self) -> bool {
        used.hours >= self.hours || used.mutations >= self.mutations
    }

    /// Whether one further mutation is inside the envelope.
    #[must_use]
    pub const fn allows_mutation(self, used: Self) -> bool {
        !self.exhausted(used)
    }

    /// The remaining mutation count, saturating at zero.
    #[must_use]
    pub const fn remaining(self, used: Self) -> u32 {
        self.mutations.saturating_sub(used.mutations)
    }
}
