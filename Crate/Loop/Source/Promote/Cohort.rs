//! The hardware and environment cohort a release is scoped to.
//! Grounded in `RESEARCH_GATE.md:97-117` (a stamp is valid only inside its
//! declared protocol/hardware cohort) and `MARKET.md:94-96` (many iterations
//! on one machine are not independent machine-level replications).

/// The cohort binding of a measurement and its release.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cohort {
    /// Hardware topology and CPU class token.
    pub hardware: String,
    /// Kernel, filesystem and environment token.
    pub environment: String,
}

impl Cohort {
    /// Bind a decision to a hardware and environment cohort.
    #[must_use]
    pub fn of(hardware: &str, environment: &str) -> Self {
        Self {
            hardware: hardware.to_owned(),
            environment: environment.to_owned(),
        }
    }

    /// Whether the cohort is fully specified.
    #[must_use]
    pub fn is_scoped(&self) -> bool {
        !self.hardware.is_empty() && !self.environment.is_empty()
    }

    /// Whether another cohort is the same binding.
    #[must_use]
    pub fn matches(&self, other: &Self) -> bool {
        self.hardware == other.hardware && self.environment == other.environment
    }
}
