//! One Cargo feature selection from `feature_sets[]`.
//!
//! Grounded in `prun-bench.schema.json:138-161`, `prun-bench.json:43-49` and
//! `prun_lab.py:316-319`, which append `--no-default-features` and
//! `--features <comma-list>` to the build command.
//!
//! File name `FeatureSet` is mandated by the card; its two words are kept
//! glued only to satisfy that explicit naming contract.

use serde::{Deserialize, Serialize};

use super::DefaultFeatures;

/// A Cargo feature-set axis member.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeatureSet {
    /// Stable identifier used in variant IDs.
    pub id: String,
    /// Named features to enable together.
    #[serde(default)]
    pub features: Vec<String>,
    /// Whether Cargo default features are kept.
    #[serde(rename = "no_default_features", default)]
    pub defaults: DefaultFeatures,
}

impl FeatureSet {
    /// The `--features` argument, present only when the set is nonempty
    /// (prun_lab.py:318-319).
    #[must_use]
    pub fn selection(&self) -> Option<String> {
        (!self.features.is_empty()).then(|| self.features.join(","))
    }
}
