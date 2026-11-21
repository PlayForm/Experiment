//! The build manifest written to `.prun-bench/latest_build.json`.
//!
//! Grounded in `prun_lab.py:437-459`: timestamp, host facts, Docker and rustc
//! versions, the invariants, built artifacts and runner image identities. This
//! is also copied into every session as `metadata.json` (:535).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::build::Artifact;
use crate::config::Invariant;

use super::{Host, ImageRecord};

/// The complete build manifest.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub built_at_utc: String,
    pub host: Host,
    pub docker_version: String,
    pub rustc_version: String,
    pub invariants: Invariant,
    pub variants: Vec<Artifact>,
    pub runner_images: BTreeMap<String, ImageRecord>,
}

impl Metadata {
    /// Build-manifest filename (prun_lab.py:459).
    pub const NAME: &'static str = "latest_build.json";
}
