//! One node in the hardware-classified benchmark mesh.
//!
//! Grounded in `docs/00-system-constitution.md:14-15` (preserve the full
//! cohort/hardware matrix, do not conflate one CPU with global best),
//! `docs/02-pitfalls-and-verification.md:8-9` and
//! `docs/13-repository-agnostic.md:47-53` (scale via dedicated physical
//! hardware cohorts). A node carries its topology and its isolation guarantee;
//! separate hardware classes are never merged.

use crate::mesh::Isolation;

/// A benchmark host classified by its physical topology.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    /// Stable host identifier within its cohort.
    pub id: String,
    /// Hardware class this node belongs to.
    pub hardware_class: String,
    /// Declared physical core and sibling topology.
    pub topology: String,
    /// Current isolation guarantee of the reservation.
    pub isolation: Isolation,
}

impl Node {
    /// Declare a node, refusing an empty topology with a warning.
    #[must_use]
    pub fn of(
        id: String,
        hardware_class: String,
        topology: String,
        isolation: Isolation,
    ) -> Result<Self, crate::Warning> {
        match topology.is_empty() {
            true => Err(crate::Warning::InvalidValue {
                field: "physical topology",
                detail: "a benchmark node must declare its core topology".to_owned(),
            }),
            false => Ok(Self {
                id,
                hardware_class,
                topology,
                isolation,
            }),
        }
    }

    /// Whether this node is currently eligible for authority-grade timing.
    #[must_use]
    pub const fn authoritative(&self) -> crate::Permission {
        self.isolation.authoritative()
    }
}
