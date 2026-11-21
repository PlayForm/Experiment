//! Deterministic Cartesian expansion of the build matrix.
//!
//! Grounded in `prun_lab.py::matrix` (220-241): the product order is
//! refs -> builds -> feature_sets, exclusions are applied by generated id,
//! and an all-excluded matrix is an error. `crossed` adds the workload axis
//! exactly as `RUNNER_DRIVER:122-124` loops workloads then variants.

use std::collections::BTreeSet;

use crate::config::{Exclude, Manifest, Workload};
use crate::Fault;

use super::{Candidate, Skip, Variant};

/// The ordered plan of variants, with the identifiers that were excluded.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Matrix {
    /// Variants in deterministic product order.
    pub variants: Vec<Variant>,
    /// Exclusion keys that matched a generated candidate.
    pub skipped: Vec<String>,
}

impl Matrix {
    /// Expand the manifest into a matrix, preserving controller order.
    pub fn expand(manifest: &Manifest) -> Result<Self, Fault> {
        let exclusions: BTreeSet<&str> = manifest
            .exclude
            .iter()
            .map(Exclude::as_str)
            .collect();
        let decisions: Vec<(Candidate<'_>, Skip)> = manifest
            .refs
            .iter()
            .flat_map(|reference| {
                manifest.builds.iter().flat_map(move |build| {
                    manifest
                        .feature_sets
                        .iter()
                        .map(move |feature| Candidate {
                            reference,
                            build,
                            feature,
                        })
                })
            })
            .map(|candidate| {
                let id = candidate.id();
                let skip = Skip::decide(&id, &exclusions);
                (candidate, skip)
            })
            .collect();
        let variants = decisions
            .iter()
            .filter(|(_, skip)| matches!(skip, Skip::Keep))
            .map(|(candidate, _)| Variant::from_candidate(candidate))
            .collect();
        let skipped = decisions
            .iter()
            .filter_map(|(_, skip)| skip.rejected().map(ToOwned::to_owned))
            .collect();
        (!variants.is_empty())
            .then_some(Self { variants, skipped })
            .ok_or(Fault::Empty)
    }

    /// Cross each variant with every workload in the driver's order.
    #[must_use]
    pub fn crossed<'a>(&'a self, workloads: &'a [Workload]) -> Vec<(&'a Variant, &'a Workload)> {
        workloads
            .iter()
            .flat_map(|workload| {
                self.variants
                    .iter()
                    .map(move |variant| (variant, workload))
            })
            .collect()
    }
}
