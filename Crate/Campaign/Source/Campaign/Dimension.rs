//! The eight v2 dimensions and their coupling to the v0 benchmark axes.
//!
//! Grounded in `README.md:15-17` (the example shows
//! source/build/feature/heuristic/hardware/workload/agent/safety dimensions)
//! and `REQUIREMENTS.md:336-350` (formal experiment identity) with `:381-403`
//! (workload taxonomy). The v0 matrix is the three-axis product
//! refs × builds × feature_sets in `benchmark::plan::Matrix` (by path/name);
//! v2 extends it, it does not reinterpret it.

/// A named dimension of a v2 campaign.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Dimension {
    /// Which source revision and project the campaign targets.
    Source,
    /// Which reproducible build recipe produced the binary.
    Build,
    /// Which capability/selection feature groups were compiled in.
    Feature,
    /// Which heuristic planner mode was active.
    Heuristic,
    /// Which host and execution-environment cohort was used.
    Hardware,
    /// Which fixture, command and split were measured.
    Workload,
    /// Which bounded coding agent produced a mutation.
    Agent,
    /// Which safety permission governed the run.
    Safety,
}

impl Dimension {
    /// All eight dimensions in README order.
    pub const ALL: [Self; 8] = [
        Self::Source,
        Self::Build,
        Self::Feature,
        Self::Heuristic,
        Self::Hardware,
        Self::Workload,
        Self::Agent,
        Self::Safety,
    ];

    /// The lowercase key used in the example document.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Source => "project",
            Self::Build => "builds",
            Self::Feature => "feature_groups",
            Self::Heuristic => "heuristics",
            Self::Hardware => "hardware",
            Self::Workload => "workloads",
            Self::Agent => "agents",
            Self::Safety => "safety",
        }
    }

    /// Recover a dimension from its document key.
    #[must_use]
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|dimension| dimension.key() == key)
    }

    /// The v0 axis this dimension extends, by path/type name.
    #[must_use]
    pub const fn v0_axis(self) -> Option<&'static str> {
        match self {
            Self::Source => Some("benchmark::config::Manifest.refs"),
            Self::Build => Some("benchmark::config::Manifest.builds"),
            Self::Feature => Some("benchmark::config::Manifest.feature_sets"),
            Self::Workload => Some("benchmark::config::Manifest.workloads"),
            _ => None,
        }
    }

    /// Whether this dimension is new in v2.
    #[must_use]
    pub const fn is_v2(self) -> bool {
        self.v0_axis().is_none()
    }
}
