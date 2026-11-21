//! Per-variant summary statistics from `results[].summary`.
//!
//! Grounded in `prun_lab.py:612-616`: the CSV reads the mean wall-clock time
//! and a stddev that defaults to zero when absent.

use serde::{Deserialize, Serialize};

use super::Clock;

/// Summary block of one Hyperfine result.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub time_wall_clock: Clock,
}

impl Summary {
    /// Mean wall-clock seconds (prun_lab.py:616).
    #[must_use]
    pub fn mean(&self) -> f64 {
        self.time_wall_clock.mean
    }

    /// Stddev wall-clock seconds, defaulting to zero (prun_lab.py:617).
    #[must_use]
    pub fn stddev(&self) -> f64 {
        self.time_wall_clock.stddev
    }
}
