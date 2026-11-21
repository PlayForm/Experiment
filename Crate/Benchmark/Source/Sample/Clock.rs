//! Summary statistics for one metric, as exported in a Hyperfine v2 summary.
//!
//! Grounded in `prun_lab.py:612-616`, which reads
//! `results[].summary.time_wall_clock.mean` and `.stddev` (defaulting a
//! missing stddev to zero).

use serde::{Deserialize, Serialize};

/// Mean and standard deviation of one timing metric.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Clock {
    #[serde(default)]
    pub mean: f64,
    #[serde(default)]
    pub stddev: f64,
}
