//! One synthetic workload from `workloads[]`.
//!
//! Grounded in `prun-bench.schema.json:188-215`, `prun-bench.json:64-93`,
//! `prun_lab.py:116-120` (the Run binary arguments) and README.md:124-126
//! (these workloads are synthetic, not real repository status).

use serde::{Deserialize, Serialize};

use super::Parallel;

/// A fixture-bind + pattern + command workload.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Workload {
    /// Stable identifier used in result filenames.
    pub id: String,
    /// Identifier of the fixture this workload binds.
    pub fixture: String,
    /// Glob pattern passed to Run as `--Pattern`.
    pub pattern: String,
    /// Serial or concurrent traversal.
    #[serde(default)]
    pub parallel: Parallel,
    /// Command Run spawns on each match, passed as `--Command`.
    pub command: String,
}
