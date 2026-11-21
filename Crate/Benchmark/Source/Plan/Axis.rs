//! The named axes of the experiment grid.
//!
//! Grounded in `README.md:8-9` (three-dimensional build matrix) and
//! `prun_lab.py:223-225` (refs x builds x feature sets), with the workload axis
//! applied later by `RUNNER_DRIVER:122-124`.

/// One expansion axis of the benchmark grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Axis {
    Source,
    Build,
    Feature,
    Workload,
}

impl Axis {
    /// Stable label matching the manifest key (schema.json:8-16).
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Source => "refs",
            Self::Build => "builds",
            Self::Feature => "feature_sets",
            Self::Workload => "workloads",
        }
    }
}
