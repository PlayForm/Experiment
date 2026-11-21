//! A reproducible build recipe in the v2 campaign.
//!
//! Grounded in `campaign.v2.json:67-100`, the schema at
//! `schemas/campaign.v2.schema.json:319-365` and `REQUIREMENTS.md:336-357`
//! (formal experiment identity). This consumes the v0 build axis by path/type
//! name (`benchmark::build::Recipe`) and never edits `Crate/Benchmark`.
//!
//! Exception declared: `Profile` is declared beside `Build` because it is the
//! typed form of this one field and has no independent lifecycle.

use serde::{Deserialize, Serialize};

use crate::BENCHMARK_PLAN_PATH;

/// Cargo profile of a build recipe (`schemas/campaign.v2.schema.json:328-331`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Profile {
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "dev")]
    Dev,
}

impl Profile {
    /// The JSON enum token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Dev => "dev",
        }
    }
}

/// One build in the campaign matrix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Build {
    pub id: String,
    #[serde(rename = "cargo_profile")]
    pub cargo_profile: Profile,
    #[serde(rename = "target_cpu")]
    pub target_cpu: String,
    pub features: Vec<String>,
    #[serde(rename = "runtime_strategy")]
    pub runtime_strategy: String,
    pub rustflags: Vec<String>,
}

impl Build {
    /// Recognised keys (`schemas/campaign.v2.schema.json:355-361`).
    pub const KEYS: [&'static str; 6] = [
        "id",
        "cargo_profile",
        "target_cpu",
        "features",
        "runtime_strategy",
        "rustflags",
    ];

    /// The v0 matrix type this recipe extends, by path/type name.
    #[must_use]
    pub const fn v0_plan() -> &'static str {
        BENCHMARK_PLAN_PATH
    }

    /// Sorted capability set: build identity is over sorted, versioned
    /// metadata (`REQUIREMENTS.md:352-357`).
    #[must_use]
    pub fn capability_set(&self) -> Vec<&str> {
        let mut set: Vec<&str> = self
            .features
            .iter()
            .chain(self.rustflags.iter())
            .map(String::as_str)
            .collect();
        set.sort_unstable();
        set.dedup();
        set
    }

    /// Whether this build is the untouched reference arm
    /// (`REQ-BEN-09`, `campaign.v2.json:69-75`).
    #[must_use]
    pub fn is_reference(&self) -> bool {
        self.runtime_strategy == "reference"
    }
}
