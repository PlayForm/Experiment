//! One built binary record from `build_variant`.
//!
//! Grounded in `prun_lab.py:396-407`: each artifact records id, commit SHA,
//! git ref, profile, runner group, binary relpath and SHA-256, the captured
//! lock digest, and the originating build/feature config.

use serde::{Deserialize, Serialize};

use crate::config::{BuildConfig, FeatureSet, Profile};
use crate::runner::Group;

use super::Digest;

/// A cached, relocatable Run binary ready for benchmarking.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    /// Variant identifier.
    pub id: String,
    /// Resolved commit SHA.
    pub sha: String,
    /// Original git ref.
    pub git_ref: String,
    /// Cargo profile used.
    pub profile: Profile,
    /// Runner group.
    pub group: Group,
    /// Binary path relative to the artifacts directory.
    #[serde(rename = "binary_relpath")]
    pub binary: String,
    /// SHA-256 of the binary bytes.
    pub binary_sha256: Digest,
    /// SHA-256 of the captured lock.
    pub cargo_lock_sha256: Digest,
    /// Build configuration.
    pub build: BuildConfig,
    /// Feature selection.
    pub features: FeatureSet,
}

impl Artifact {
    /// Artifact directory tail `<id>__<16-hex fingerprint>` (prun_lab.py:378).
    #[must_use]
    pub fn directory(&self, fingerprint: &Digest) -> String {
        format!("{}__{}", self.id, fingerprint.prefix(16))
    }
}
