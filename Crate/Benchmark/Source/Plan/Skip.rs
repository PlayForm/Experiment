//! Exclusion decision for one matrix candidate.
//!
//! Grounded in `README.md:102` (`main__debug__bounded`) and
//! `prun_lab.py:222-228`, where a generated variant identifier found in
//! `exclude` is skipped from the plan.

use std::collections::BTreeSet;

/// Whether a candidate survives the exclude set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Skip {
    Keep,
    Excluded(String),
}

impl Skip {
    /// Decide membership in the exclusion set.
    #[must_use]
    pub fn decide(id: &str, exclusions: &BTreeSet<&str>) -> Self {
        exclusions
            .contains(id)
            .then(|| Self::Excluded(id.to_owned()))
            .unwrap_or(Self::Keep)
    }

    /// The rejected identifier, when excluded.
    #[must_use]
    pub fn rejected(&self) -> Option<&str> {
        match self {
            Self::Keep => None,
            Self::Excluded(id) => Some(id),
        }
    }
}
