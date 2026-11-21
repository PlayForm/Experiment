//! Separate cross-project integration institution.
//!
//! Grounded in `SYSTEM.md:59,101-102` (may propose applying a result elsewhere
//! but cannot mark it portable until a distinct target-repository experiment
//! passes) and `PUBLISHING.md:107-116` (a source speedup is not portable
//! evidence; a knowledge card carries applicability and negative-transfer
//! evidence and yields a new preregistered hypothesis).

use crate::Permission;

/// The transfer researcher, distinct from publisher and evaluator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Integrator {
    /// Distinct integrator identity (`SECURITY.md:28`).
    pub integrator_identity: &'static str,
}

impl Integrator {
    /// Construct the integrator model.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            integrator_identity: "independent_integration_researcher",
        }
    }

    /// Portability requires a fresh target-repository experiment
    /// (`SYSTEM.md:102`, `SYSTEM.md:131-134`).
    #[must_use]
    pub const fn may_mark_portable_without_target_experiment() -> Permission {
        Permission::Deny
    }

    /// A measured effect in repository A cannot be imported as evidence for B
    /// (`PUBLISHING.md:109-112`, `REPOSITORIES.md:51-53`).
    #[must_use]
    pub const fn may_import_speedup_claim() -> Permission {
        Permission::Deny
    }
}
