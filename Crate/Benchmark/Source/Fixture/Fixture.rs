//! A deterministic fixture specification from `fixtures[]`.
//!
//! Grounded in `prun-bench.schema.json:162-187`, `prun-bench.json:50-63` and
//! `prun_lab.py::FIXTURE_DRIVER` (147-161), which creates `directories` unit
//! folders under shards and levels, each holding one `bench.marker` plus
//! `noise_files` noise files.

use serde::{Deserialize, Serialize};

/// A named, reproducible directory tree used by one workload.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fixture {
    /// Stable identifier referenced by workloads.
    pub id: String,
    /// Number of unit directories, each with one marker.
    pub directories: u32,
    /// Nesting depth below each shard.
    pub depth: u32,
    /// Noise files written beside each marker.
    pub noise_files: u32,
}

impl Fixture {
    /// Marker filename written in every unit (prun_lab.py:157).
    pub const MARKER: &'static str = "bench.marker";

    /// Expected matched-directory count for preflight (prun_lab.py:133).
    #[must_use]
    pub const fn expected(&self) -> usize {
        self.directories as usize
    }
}
