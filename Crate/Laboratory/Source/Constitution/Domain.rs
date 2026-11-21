//! The four separate information domains.
//!
//! Grounded in `docs/00-system-constitution.md:41-47` and
//! `docs/01-topology-and-boundaries.md:28-49`: human observation and owner
//! control (Plane 0), control/orchestration evidence (Plane 1), product/SUT
//! execution evidence (Plane 2), and offline untrusted model execution
//! (Boundary X). Boundary X is **not** a fourth trusted plane. The types here
//! make that trust distinction structural rather than a prose note.

use crate::Permission;

/// One of the four isolated information domains.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Domain {
    /// Human observation and signed owner commands.
    HumanObservation,
    /// Trusted control evidence and telemetry.
    ControlEvidence,
    /// Product/SUT execution evidence and diagnostics.
    ProductEvidence,
    /// Offline, untrusted model execution; Boundary X.
    OfflineWorker,
}

impl Domain {
    /// Whether this domain is a trusted plane at all.
    #[must_use]
    pub const fn is_trusted(self) -> Permission {
        match self {
            Self::HumanObservation | Self::ControlEvidence | Self::ProductEvidence => {
                Permission::Allow
            }
            Self::OfflineWorker => Permission::Deny,
        }
    }

    /// Whether evidence produced here may be authoritative for a release gate.
    #[must_use]
    pub const fn authoritative(self) -> Permission {
        match self {
            Self::ControlEvidence | Self::ProductEvidence => Permission::Allow,
            Self::HumanObservation | Self::OfflineWorker => Permission::Deny,
        }
    }

    /// The grounding section for this domain.
    #[must_use]
    pub const fn source(self) -> &'static str {
        match self {
            Self::HumanObservation => "docs/00:43, docs/03",
            Self::ControlEvidence => "docs/00:44, docs/07:13-20",
            Self::ProductEvidence => "docs/00:45, docs/07:31-39",
            Self::OfflineWorker => "docs/00:46-47, docs/01:45-49",
        }
    }
}
