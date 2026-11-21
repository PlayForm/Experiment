//! Benchmark protocol: paired, seeded, contamination-aware measurement.
//!
//! Grounded in `campaign.v2.json:199-212`, the schema at
//! `schemas/campaign.v2.schema.json:577-634` and `REQUIREMENTS.md:453-506`
//! (REQ-BEN-01..13). The protocol is typed data for a future trusted runner;
//! no Hyperfine process is invoked and no sample is fabricated.
//!
//! Exception declared: `Engine`, `Order` and `Invalid` are declared beside
//! `Bench` because each is the typed form of one field and has no independent
//! lifecycle.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// The only supported engine in the v0 and v2 designs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Engine {
    Hyperfine,
}

/// Block ordering (`schemas/campaign.v2.schema.json:603-606`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Order {
    /// ABBA/BAAB blocks with seeded reversal (`REQUIREMENTS.md:467-469`).
    #[serde(rename = "abba_with_seeded_reversal")]
    AbbaSeeded,
    /// Fully randomized blocks.
    #[serde(rename = "randomized_blocks")]
    Randomized,
}

/// Response to a contaminated environment (`campaign.v2.json:211`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Invalid {
    /// Keep the sample, quarantine it and reschedule with a fresh baseline.
    #[serde(rename = "quarantine_and_reschedule")]
    Quarantine,
    /// Stop the campaign.
    #[serde(rename = "fail_closed")]
    FailClosed,
}

/// The measurement protocol of a v2 campaign.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bench {
    pub engine: Engine,
    #[serde(rename = "engine_version")]
    pub engine_version: String,
    #[serde(rename = "warmups_per_candidate", default)]
    pub warmups: u32,
    #[serde(rename = "samples_per_block")]
    pub samples_per_block: u32,
    #[serde(rename = "minimum_blocks", default)]
    pub minimum_blocks: u32,
    #[serde(rename = "paired_comparison", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub paired: Permission,
    #[serde(rename = "block_order")]
    pub block_order: Order,
    #[serde(rename = "periodic_reference_blocks", default)]
    pub periodic_reference_blocks: u32,
    #[serde(rename = "profile_during_primary_timing", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub profile_during_timing: Permission,
    #[serde(rename = "keep_raw_samples", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub keep_raw_samples: Permission,
    #[serde(rename = "holdout_visible_to_mutator", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub holdout_visible: Permission,
    #[serde(rename = "invalid_environment_policy", default)]
    pub invalid_policy: Invalid,
}

impl Bench {
    /// Recognised keys (`schemas/campaign.v2.schema.json:625-632`).
    pub const KEYS: [&'static str; 12] = [
        "engine",
        "engine_version",
        "warmups_per_candidate",
        "samples_per_block",
        "minimum_blocks",
        "paired_comparison",
        "block_order",
        "periodic_reference_blocks",
        "profile_during_primary_timing",
        "keep_raw_samples",
        "holdout_visible_to_mutator",
        "invalid_environment_policy",
    ];

    /// Warn on a protocol that cannot produce a paired comparison
    /// (REQ-BEN-03, REQ-BEN-07).
    pub fn validate(&self, warnings: &mut Vec<crate::Warning>) {
        match self.samples_per_block >= 2 {
            true => {}
            false => warnings.push(crate::Warning::Range {
                domain: "benchmark",
                key: "samples_per_block".to_owned(),
            }),
        }
        match self.paired {
            Permission::Deny => warnings.push(crate::Warning::Bound {
                domain: "benchmark",
                key: "paired_comparison is required".to_owned(),
            }),
            Permission::Allow => {}
        }
    }

    /// Whether primary timing is kept free of profiling instrumentation
    /// (`REQUIREMENTS.md:485-486`).
    #[must_use]
    pub fn timing_is_clean(&self) -> Permission {
        self.profile_during_timing
    }
}
