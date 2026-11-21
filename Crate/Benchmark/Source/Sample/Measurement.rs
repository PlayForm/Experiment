//! One Hyperfine measurement record.
//!
//! Grounded in `prun_lab.py:615-616` (`results[].measurements[]` and the
//! per-record `time_wall_clock` value).

use serde::{Deserialize, Serialize};

use super::Observation;

/// A single measurement object in a result.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub time_wall_clock: Option<Observation>,
}
