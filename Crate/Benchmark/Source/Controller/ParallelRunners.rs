//! `--parallel-runners`, modelled as a two-variant enum rather than a flag.
//!
//! Grounded in `prun_lab.py:656-660,540-551` and README.md:64-82: release and
//! diagnostic groups may overlap only when disjoint `runner_cpusets` exist.
//!
//! File name `ParallelRunners` is mandated by the card; its two words are kept
//! glued only to satisfy that explicit naming contract.

use crate::config::Invariant;
use crate::runner::{Group, Lease};
use crate::Fault;

/// Concurrency policy for runner groups.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParallelRunners {
    Serial,
    Parallel,
}

impl ParallelRunners {
    /// Whether groups may be benchmarked concurrently.
    #[must_use]
    pub const fn enabled(self) -> bool {
        matches!(self, Self::Parallel)
    }

    /// Validate leases for parallel mode; serial mode needs none
    /// (prun_lab.py:540-551, 589-594).
    pub fn validate(self, groups: &[Group], invariant: &Invariant) -> Result<Vec<Lease>, Fault> {
        match self {
            Self::Serial => Ok(Vec::new()),
            Self::Parallel => match groups.len() > 1 {
                true => Lease::assign(groups, invariant),
                false => Ok(Vec::new()),
            },
        }
    }
}
