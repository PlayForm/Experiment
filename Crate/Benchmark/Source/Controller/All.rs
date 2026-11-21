//! The `all` subcommand: build then benchmark.
//!
//! Grounded in `prun_lab.py:686-689`: both phases run in order; each keeps its
//! own concurrency policy (parallel builds, sequential-in-group benchmarks).

use super::{Bench, Build};

/// A request to build and then benchmark in one invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct All {
    /// Build phase.
    pub build: Build,
    /// Benchmark phase.
    pub bench: Bench,
}

impl All {
    /// Ordered phases of the combined command.
    pub const PHASES: [&'static str; 2] = ["build", "bench"];
}
