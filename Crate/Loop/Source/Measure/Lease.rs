//! A disjoint CPU lease adopted from the v0 runner model.
//! Grounded in `ARCHITECTURE.md:56-57` (the runner owns the hardware lease and
//! applies resource constraints) and `AGENT_CONTRACT.md:49-51` (confirm host
//! capabilities and resource lease before measuring). No lease is acquired
//! here; a trusted model value is adopted and inspected.

use benchmark::runner::Lease as BenchmarkLease;

use super::cpuset::Cpuset;

/// A read-only model of a runner group's CPU lease.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    /// The owning runner group.
    pub group: benchmark::runner::Group,
    /// Cores reserved for that group.
    pub cpuset: Cpuset,
}

impl Lease {
    /// Adopt a trusted lease value without acquiring hardware.
    #[must_use]
    pub fn adopt(inner: &BenchmarkLease) -> Self {
        Self {
            group: inner.group,
            cpuset: Cpuset {
                inner: inner.cpuset.clone(),
            },
        }
    }

    /// Whether two leases overlap on any core.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.cpuset.overlaps(&other.cpuset)
    }

    /// The group tag used in result filenames.
    #[must_use]
    pub const fn tag(&self) -> &'static str {
        self.group.tag()
    }
}
