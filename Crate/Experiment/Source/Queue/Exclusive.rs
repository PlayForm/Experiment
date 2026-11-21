//! Physical isolation assessment for an exclusive benchmark lease.
//!
//! Grounded in `REQUIREMENTS_V3.md` EXP-001, `CONTAINER_MATRIX.md:61-64`
//! (cpuset alone does not guarantee physical isolation: sibling SMT, LLC
//! sharing, NUMA, thermal/power and disk interference must be assessed) and
//! `RUNBOOK.md:88-92` (quarantine on throttling, thermal drift or contention).
//! A degraded host is quarantined rather than silently trusted.

/// The observed class of cross-host interference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Interference {
    /// Reserved cores are physically isolated.
    None,
    /// Sibling SMT threads share execution resources.
    SiblingSmt,
    /// Last-level cache is shared with other work.
    SharedCache,
    /// Disk or network contention was observed.
    SharedIo,
    /// The interference class cannot be established.
    Unknown,
}

/// The health state of a benchmark host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Health {
    /// The host may serve precision trials.
    Healthy,
    /// The host is quarantined until a fresh baseline is measured.
    Quarantined,
}

/// An exclusive reservation of benchmark hardware.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exclusive {
    /// Reserved CPU set.
    pub cpuset: String,
    /// Worker or runner holding the reservation.
    pub holder: String,
    /// Assessed interference class.
    pub interference: Interference,
    /// Current host health.
    pub health: Health,
}

impl Exclusive {
    /// Whether this reservation is suitable for a precision trial.
    #[must_use]
    pub const fn is_exclusive(&self) -> bool {
        matches!(
            (self.interference, self.health),
            (Interference::None, Health::Healthy)
        )
    }

    /// Whether a fresh baseline is required before reuse.
    #[must_use]
    pub const fn needs_fresh_baseline(&self) -> bool {
        matches!(self.health, Health::Quarantined)
    }
}
