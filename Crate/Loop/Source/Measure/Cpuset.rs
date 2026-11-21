//! CPU affinity parsing delegated to the v0 runner model.
//! Grounded in `AGENT_CONTRACT.md:49-51` (confirm host capabilities and
//! immutable hashes) and `ARCHITECTURE.md:119-125` (broker-enforced API scope
//! and reserved CPUs). Malformed affinity degrades to a typed fault.

/// A parsed CPU set owned by the v0 benchmark crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cpuset {
    /// The trusted parsed set.
    pub inner: benchmark::runner::CpuSet,
}

impl Cpuset {
    /// Parse an affinity expression, returning a typed fault on error.
    pub fn parse(text: &str) -> Result<Self, benchmark::Fault> {
        benchmark::runner::CpuSet::parse(text).map(|inner| Self { inner })
    }

    /// Whether two sets share a core.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.inner.overlaps(&other.inner)
    }

    /// Whether the set reserves no cores.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
