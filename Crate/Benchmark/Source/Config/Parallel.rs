//! Serial versus parallel workload execution, modelled as a two-variant enum
//! rather than a bare `bool` (house style).
//!
//! Grounded in `prun-bench.json` workload `parallel` fields and
//! `prun_lab.py::RUNNER_DRIVER` line 119, which appends `--Parallel` to the
//! measured Run command.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Concurrency policy for one workload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Parallel {
    Serial,
    Concurrent,
}

impl Parallel {
    /// The Run binary flag implied by this policy (prun_lab.py:119).
    #[must_use]
    pub const fn flag(self) -> Option<&'static str> {
        match self {
            Self::Serial => None,
            Self::Concurrent => Some("--Parallel"),
        }
    }
}

impl Default for Parallel {
    fn default() -> Self {
        Self::Serial
    }
}

impl<'de> Deserialize<'de> for Parallel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        bool::deserialize(deserializer).map(|enabled| match enabled {
            true => Self::Concurrent,
            false => Self::Serial,
        })
    }
}

impl Serialize for Parallel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Serial => false.serialize(serializer),
            Self::Concurrent => true.serialize(serializer),
        }
    }
}
