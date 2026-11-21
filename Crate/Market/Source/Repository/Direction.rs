//! Optimization direction of a declared metric.
//!
//! Grounded in `REPOSITORIES.md:25` (performance metrics and direction) and
//! `:39-40` (metrics are repository-specific; a public Python project and a
//! GPU project do not share one metric). Direction is policy data; no metric
//! is measured here.

use crate::Warning;

/// Whether a smaller or larger metric value is the improvement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Direction {
    /// Lower values are better, e.g. elapsed seconds (`REPOSITORIES.md:25`).
    LowerIsBetter,
    /// Higher values are better, e.g. throughput.
    HigherIsBetter,
}

impl Direction {
    /// Both directions.
    pub const ALL: [Self; 2] = [Self::LowerIsBetter, Self::HigherIsBetter];

    /// Decode a declared direction label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "lower_is_better" => Ok(Self::LowerIsBetter),
            "higher_is_better" => Ok(Self::HigherIsBetter),
            other => Err(Warning::InvalidValue {
                field: "metric direction",
                detail: format!("unknown direction {other}"),
            }),
        }
    }

    /// The wire label of this direction.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::LowerIsBetter => "lower_is_better",
            Self::HigherIsBetter => "higher_is_better",
        }
    }
}
