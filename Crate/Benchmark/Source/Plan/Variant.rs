//! One built variant in the deterministic matrix.
//!
//! Grounded in `prun_lab.py:229-238`: each surviving candidate is annotated
//! with its `profile` and its `runner` group (`release` for release profile,
//! `diagnostic` otherwise).

use crate::config::{BuildConfig, FeatureSet, Profile, Ref};
use crate::runner::Group;

use super::Candidate;

/// A planned build-and-benchmark variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variant {
    /// Stable `<ref>__<build>__<feature>` identifier.
    pub id: String,
    /// Source snapshot selector.
    pub reference: Ref,
    /// Compiler configuration.
    pub build: BuildConfig,
    /// Feature selection.
    pub features: FeatureSet,
    /// Resolved Cargo profile.
    pub profile: Profile,
    /// Runner group assigned by profile (prun_lab.py:236).
    pub group: Group,
}

impl Variant {
    /// Materialise a variant from a surviving candidate.
    #[must_use]
    pub fn from_candidate(candidate: &Candidate<'_>) -> Self {
        Self {
            id: candidate.id(),
            reference: candidate.reference.clone(),
            build: candidate.build.clone(),
            features: candidate.feature.clone(),
            profile: candidate.build.profile,
            group: Group::from(candidate.build.profile),
        }
    }
}
