//! One raw timing observation from `results[].measurements`.
//!
//! Grounded in `prun_lab.py:615`, which reads `m["time_wall_clock"]["value"]`
//! for every measurement in a Hyperfine v2 result.

use serde::{Deserialize, Serialize};

/// A single measured value with its unit.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    #[serde(default)]
    pub value: f64,
    #[serde(default)]
    pub unit: String,
}
