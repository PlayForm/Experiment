//! Experiment invariants from `invariants` plus their versioned defaults.
//!
//! Grounded in `prun-bench.schema.json:21-87`, `prun-bench.json:3-16` and
//! `prun_lab.py::DEFAULT` (30-43). The default document pins Hyperfine 2.0.0,
//! `LC_ALL=C`, 3 warmups, 15 runs and 4g memory.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::CpuSets;

/// Host-independent constraints shared by every variant.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Invariant {
    /// Builder image base (rust:slim-bookworm).
    pub builder_base: String,
    /// Runner image base (debian:bookworm-slim).
    pub runner_base: String,
    /// Pinned Hyperfine version (README.md:13-14).
    pub hyperfine_version: String,
    /// Global CPU affinity; empty means unpinned (README.md:137).
    pub cpuset: String,
    /// Docker memory and memory-swap limit.
    pub memory: String,
    /// Hyperfine warmup count.
    pub warmups: u32,
    /// Hyperfine run count; the schema floor is 2.
    pub runs: u32,
    /// Environment variables injected into runners.
    pub environment: BTreeMap<String, String>,
    /// Hyperfine metrics expression; `default` when absent (prun_lab.py:565).
    #[serde(default)]
    pub metrics: Option<String>,
    /// Per-runner affinity leases used by `--parallel-runners`.
    #[serde(default)]
    pub runner_cpusets: CpuSets,
}

impl Invariant {
    /// The metrics expression actually passed to Hyperfine (prun_lab.py:565).
    #[must_use]
    pub fn metrics(&self) -> &str {
        self.metrics.as_deref().unwrap_or("default")
    }

    /// Resolve affinity for a runner group, falling back to the global cpuset
    /// exactly as `prun_lab.py:513` does (`dedicated or global`).
    #[must_use]
    pub fn affinity(&self, group: &str) -> Option<&str> {
        let dedicated = match group {
            "release" => self.runner_cpusets.release.as_deref(),
            "diagnostic" => self.runner_cpusets.diagnostic.as_deref(),
            _ => None,
        };
        dedicated
            .filter(|value| !value.is_empty())
            .or_else(|| (!self.cpuset.is_empty()).then_some(self.cpuset.as_str()))
    }
}

impl Default for Invariant {
    fn default() -> Self {
        Self {
            builder_base: "rust:slim-bookworm".to_owned(),
            runner_base: "debian:bookworm-slim".to_owned(),
            hyperfine_version: crate::HYPERFINE_VERSION.to_owned(),
            cpuset: String::new(),
            memory: "4g".to_owned(),
            warmups: 3,
            runs: 15,
            environment: BTreeMap::from([("LC_ALL".to_owned(), "C".to_owned())]),
            metrics: Some(
                "time_wall_clock,time_cpu,time_user,time_system,memory_peak_resident".to_owned(),
            ),
            runner_cpusets: CpuSets::default(),
        }
    }
}
