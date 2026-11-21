//! Host facts captured into `latest_build.json`.
//!
//! Grounded in `prun_lab.py:423-436`: platform, machine, the first cpuinfo
//! model line, and the sorted scheduler affinity set (empty where the host
//! exposes no `sched_getaffinity`).

use serde::{Deserialize, Serialize};

/// Host metadata for one build.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Host {
    pub platform: String,
    pub machine: String,
    pub cpu_model: String,
    pub affinity: Vec<u32>,
}
