//! A raw axis tuple before exclusion.
//!
//! Grounded in `prun_lab.py:223-226`: `itertools.product(refs, builds,
//! feature_sets)` producing the `<ref>__<build>__<feature>` identifier.

use crate::config::{BuildConfig, FeatureSet, Ref};

/// One point in the three-axis Cartesian product.
#[derive(Clone, Copy, Debug)]
pub struct Candidate<'a> {
    /// Chosen source ref.
    pub reference: &'a Ref,
    /// Chosen build config.
    pub build: &'a BuildConfig,
    /// Chosen feature set.
    pub feature: &'a FeatureSet,
}

impl<'a> Candidate<'a> {
    /// The exclude key for this candidate (prun_lab.py:226).
    #[must_use]
    pub fn id(&self) -> String {
        format!("{}__{}__{}", self.reference.id, self.build.id, self.feature.id)
    }
}
