//! One declared workload in the benchmark matrix.
//!
//! Grounded in `docs/10-benchmark-mesh.md:9-11,41-47`,
//! `docs/02-pitfalls-and-verification.md:10-15` and
//! `docs/00-system-constitution.md:5`: cache regime, dataset, fixture digest,
//! hardware class and warm/cold separation are declared separately so a
//! meaningful workload regime is never erased. Optional stopping is forbidden;
//! the holdout is never development data (`docs/02:12`).

/// A declared workload and its frozen measurement conditions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Workload {
    /// Stable workload identifier.
    pub id: String,
    /// Digest of the frozen dataset and fixtures.
    pub digest: crate::Digest,
    /// Declared cache regime: `cold`, `warm` or `paired`.
    pub cache_regime: String,
    /// Hardware class this workload is characterized on.
    pub hardware_class: String,
    /// Whether the holdout has been consumed for confirmation only.
    pub holdout_sealed: crate::Permission,
}

impl Workload {
    /// Declare a workload; an empty identifier is refused with a warning.
    #[must_use]
    pub fn of(
        id: String,
        digest: crate::Digest,
        cache_regime: String,
        hardware_class: String,
        holdout_sealed: crate::Permission,
    ) -> Result<Self, crate::Warning> {
        match id.is_empty() {
            true => Err(crate::Warning::InvalidValue {
                field: "workload id",
                detail: "empty workload identifier".to_owned(),
            }),
            false => Ok(Self {
                id,
                digest,
                cache_regime,
                hardware_class,
                holdout_sealed,
            }),
        }
    }

    /// Whether this workload may be used as development data.
    #[must_use]
    pub const fn development_use(&self) -> crate::Permission {
        match self.holdout_sealed {
            crate::Permission::Allow => crate::Permission::Deny,
            crate::Permission::Deny => crate::Permission::Allow,
        }
    }
}
