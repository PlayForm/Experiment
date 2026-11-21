//! One row of the Hyperfine CSV summary.
//!
//! Grounded in `prun_lab.py:618-629`: columns are runner, workload, variant,
//! runs, mean, stddev, CV percent and speedup against the first entry, with
//! blank CV/speedup when the mean is zero.

use crate::sample::Sample;

/// A single CSV summary record.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub runner: String,
    pub workload: String,
    pub variant: String,
    pub runs: usize,
    pub mean_s: f64,
    pub stddev_s: f64,
    pub cv_percent: Option<f64>,
    pub speedup_vs_first: Option<f64>,
}

impl Row {
    /// Column order from `csv.DictWriter` (prun_lab.py:633).
    pub const HEADERS: [&'static str; 8] = [
        "runner",
        "workload",
        "variant",
        "runs",
        "mean_s",
        "stddev_s",
        "cv_percent",
        "speedup_vs_first",
    ];

    /// Compose a row from one result and the group baseline.
    #[must_use]
    pub fn compose(runner: &str, workload: &str, sample: &Sample, baseline: f64) -> Self {
        Self {
            runner: runner.to_owned(),
            workload: workload.to_owned(),
            variant: sample.variant().to_owned(),
            runs: sample.runs(),
            mean_s: Self::scaled(sample.mean(), 8),
            stddev_s: Self::scaled(sample.stddev(), 8),
            cv_percent: sample.cv().map(|value| Self::scaled(value, 2)),
            speedup_vs_first: sample.speedup(baseline).map(|value| Self::scaled(value, 3)),
        }
    }

    /// Render the row's cells in header order.
    #[must_use]
    pub fn cells(&self) -> Vec<String> {
        vec![
            Self::quote(&self.runner),
            Self::quote(&self.workload),
            Self::quote(&self.variant),
            self.runs.to_string(),
            self.mean_s.to_string(),
            self.stddev_s.to_string(),
            self.cv_percent.map_or_else(String::new, |value| value.to_string()),
            self.speedup_vs_first
                .map_or_else(String::new, |value| value.to_string()),
        ]
    }

    fn scaled(value: f64, digits: i32) -> f64 {
        let factor = 10_f64.powi(digits);
        (value * factor).round() / factor
    }

    fn quote(value: &str) -> String {
        let special = value.contains(',') || value.contains('"') || value.contains('\n');
        special
            .then(|| format!("\"{}\"", value.replace('"', "\"\"")))
            .unwrap_or_else(|| value.to_owned())
    }
}
