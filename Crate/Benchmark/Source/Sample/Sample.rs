//! One Hyperfine result entry, i.e. one variant's measurements.
//!
//! Grounded in `prun_lab.py:612-629`: the label falls back from `name` to
//! `command`, raw runs come from `measurements`, and CV/speedup are blank when
//! the mean is zero.

use serde::{Deserialize, Serialize};

use super::{Measurement, Summary};

/// A single variant's entry in a schema-v2 export.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Sample {
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub summary: Summary,
    #[serde(default)]
    pub measurements: Vec<Measurement>,
}

impl Sample {
    /// Variant label: `name` when present, otherwise `command` (:622).
    #[must_use]
    pub fn variant(&self) -> &str {
        self.name.as_deref().unwrap_or(self.command.as_str())
    }

    /// Raw wall-clock values across measurements (prun_lab.py:615).
    #[must_use]
    pub fn raw(&self) -> Vec<f64> {
        self.measurements
            .iter()
            .filter_map(|item| item.time_wall_clock.as_ref().map(|clock| clock.value))
            .collect()
    }

    /// Number of raw runs.
    #[must_use]
    pub fn runs(&self) -> usize {
        self.raw().len()
    }

    /// Mean wall-clock seconds.
    #[must_use]
    pub fn mean(&self) -> f64 {
        self.summary.mean()
    }

    /// Stddev wall-clock seconds.
    #[must_use]
    pub fn stddev(&self) -> f64 {
        self.summary.stddev()
    }

    /// Coefficient of variation in percent, blank when mean is zero (:626).
    #[must_use]
    pub fn cv(&self) -> Option<f64> {
        let mean = self.mean();
        (mean != 0.0).then(|| self.stddev() / mean * 100.0)
    }

    /// Speedup against a baseline mean, blank when mean is zero (:627).
    #[must_use]
    pub fn speedup(&self, baseline: f64) -> Option<f64> {
        (self.mean() != 0.0).then(|| baseline / self.mean())
    }
}
