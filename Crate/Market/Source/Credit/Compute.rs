//! Compute-time resource dimensions.
//!
//! Grounded in `MARKET.md:9` (CPU core seconds, exclusive benchmark seconds,
//! GPU seconds), `MARKET.md:15-19` (an exclusive benchmark CPU is never
//! perfectly substituted by a shadow price) and
//! `playbooks/research-market-week.yaml:15` (one exclusive runner per host).

/// One booked compute dimension.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Compute {
    /// Aggregate CPU core seconds (`MARKET.md:9`).
    CpuCore,
    /// Exclusive benchmark seconds (`MARKET.md:9`).
    Benchmark,
    /// Accelerator seconds (`MARKET.md:9`).
    Gpu,
}

impl Compute {
    /// All compute dimensions.
    pub const ALL: [Self; 3] = [Self::CpuCore, Self::Benchmark, Self::Gpu];

    /// Native-unit key in the resource vector.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::CpuCore => "cpu_core_seconds",
            Self::Benchmark => "benchmark_seconds",
            Self::Gpu => "gpu_seconds",
        }
    }

    /// Benchmark seconds are an exclusive admission limit, not a soft price
    /// (`MARKET.md:15-19`, `MARKET.md:112-114`).
    #[must_use]
    pub const fn is_exclusive(self) -> bool {
        matches!(self, Self::Benchmark | Self::Gpu)
    }
}
