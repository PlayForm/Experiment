//! One compiler/profile configuration from `builds[]`.
//!
//! Grounded in `prun-bench.schema.json:106-137` and
//! `prun_lab.py::builder_dockerfile` (309-332): profile, `RUSTFLAGS`, and the
//! sorted `CARGO_PROFILE_*` environment overrides.
//!
//! File name `BuildConfig` is mandated by the card; its two words are kept
//! glued here only to satisfy that explicit naming contract.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::Profile;

/// A Cargo build configuration axis member.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Stable identifier used in variant IDs.
    pub id: String,
    /// Cargo profile to compile.
    pub profile: Profile,
    /// Flags placed verbatim into `ENV RUSTFLAGS`.
    #[serde(default)]
    pub rustflags: Vec<String>,
    /// Sorted `CARGO_PROFILE_(RELEASE|DEV)_*` overrides.
    #[serde(default)]
    pub profile_env: BTreeMap<String, String>,
}

impl BuildConfig {
    /// The `RUSTFLAGS` value as a single joined string (prun_lab.py:314).
    #[must_use]
    pub fn rustflag_value(&self) -> String {
        self.rustflags.join(" ")
    }

    /// Environment overrides in deterministic sorted order (prun_lab.py:321).
    #[must_use]
    pub fn ordered_env(&self) -> Vec<(&str, &str)> {
        self.profile_env
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect()
    }
}
