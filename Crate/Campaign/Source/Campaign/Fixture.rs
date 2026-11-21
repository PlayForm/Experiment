//! Synthetic fixture descriptors.
//!
//! Grounded in `campaign.v2.json:138-169`, the schema at
//! `schemas/campaign.v2.schema.json:471-521` and `REQUIREMENTS.md:405-424`
//! (REQ-WLD-01..13). A fixture here is a typed recipe with a seed; no tree is
//! created and no file is touched.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// A seeded, reproducible workload fixture.
// `f64` is not `Eq`, so comparison is deliberately partial.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fixture {
    pub id: String,
    pub generator: String,
    pub seed: u64,
    pub directories: u32,
    pub depth: u32,
    #[serde(rename = "files_per_dir", default)]
    pub files_per_dir: u32,
    #[serde(rename = "match_density")]
    pub match_density: f64,
    #[serde(default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub writable: Permission,
}

impl Fixture {
    /// Recognised keys (`schemas/campaign.v2.schema.json:509-517`).
    pub const KEYS: [&'static str; 8] = [
        "id",
        "generator",
        "seed",
        "directories",
        "depth",
        "files_per_dir",
        "match_density",
        "writable",
    ];

    /// Warn on out-of-range values instead of creating an invalid fixture.
    pub fn validate(&self, warnings: &mut Vec<crate::Warning>) {
        let zero_forbidden = [
            ("directories", self.directories),
            ("depth", self.depth),
        ];
        zero_forbidden
            .into_iter()
            .filter(|(_, value)| *value == 0)
            .for_each(|(key, _)| warnings.push(crate::Warning::Range {
                domain: "fixtures",
                key: key.to_owned(),
            }));
        match self.match_density.is_finite() && (0.0..=1.0).contains(&self.match_density) {
            true => {}
            false => warnings.push(crate::Warning::Range {
                domain: "fixtures",
                key: "match_density".to_owned(),
            }),
        }
    }

    /// A writable fixture must only ever be a disposable per-trial copy
    /// (REQ-WLD-01).
    #[must_use]
    pub fn is_disposable(&self) -> Permission {
        self.writable
    }
}
