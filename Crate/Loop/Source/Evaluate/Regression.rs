//! A preserved critical regression.
//! Grounded in `ARCHITECTURE.md:59-60` (classify regression) and
//! `AGENT_CONTRACT.md:73-75` (machine-readable verdict includes critical
//! regressions) and `PUBLISHING.md:95-101` (report reported regressions, not
//! only positive speedup).

/// A regression observed on one workload/hardware cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Regression {
    /// Cohort on which the regression was observed.
    pub cohort: String,
    /// Magnitude of the slowdown in percent.
    pub magnitude_percent: u32,
}

impl Regression {
    /// Record a regression on a cohort.
    #[must_use]
    pub fn of(cohort: &str, magnitude_percent: u32) -> Self {
        Self {
            cohort: cohort.to_owned(),
            magnitude_percent,
        }
    }

    /// Whether this regression blocks promotion.
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        self.magnitude_percent > 0
    }
}
