//! Per-runner CPU affinity leases from `runner_cpusets`.
//!
//! Grounded in README.md:64-82, `prun-bench.schema.json:74-85`,
//! `prun-bench.json:15` (empty in the active config) and
//! `prun_lab.py:513,530-551` (missing or overlapping sets are rejected).
//!
//! File name `CpuSets` is mandated by the card; `CPU` stays a fully uppercase
//! acronym in the type while the mandated spelling is kept for the file.

use serde::{Deserialize, Serialize};

/// Optional affinity strings for the two runner groups.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CpuSets {
    #[serde(default)]
    pub release: Option<String>,
    #[serde(default)]
    pub diagnostic: Option<String>,
}

impl CpuSets {
    /// Named, non-empty leases in deterministic order (release first).
    #[must_use]
    pub fn leases(&self) -> Vec<(&str, &str)> {
        [
            ("release", self.release.as_deref()),
            ("diagnostic", self.diagnostic.as_deref()),
        ]
        .into_iter()
        .filter_map(|(name, value)| value.filter(|text| !text.is_empty()).map(|v| (name, v)))
        .collect()
    }
}
