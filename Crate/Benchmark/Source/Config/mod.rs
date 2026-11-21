//! Typed decoding of `prun-bench.json`.
//!
//! Grounded in `prun-bench.schema.json` (Draft 2020-12) and in
//! `prun_lab.py::load_config` (lines 189-217) with the `DEFAULT` document
//! (lines 30-91). Unknown keys, version drift and value drift are warnings;
//! only unreadable or unparseable JSON becomes a `Fault`.

#[path = "Manifest.rs"]
pub mod manifest;
#[path = "Ref.rs"]
pub mod reference;
#[path = "BuildConfig.rs"]
pub mod build_config;
#[path = "FeatureSet.rs"]
pub mod feature_set;
#[path = "Workload.rs"]
pub mod workload;
#[path = "Invariant.rs"]
pub mod invariant;
#[path = "CpuSets.rs"]
pub mod cpu_sets;
#[path = "Exclude.rs"]
pub mod exclude;

#[path = "Profile.rs"]
pub mod profile;
#[path = "DefaultFeatures.rs"]
pub mod default_features;
#[path = "Parallel.rs"]
pub mod parallel;

pub use build_config::BuildConfig;
pub use cpu_sets::CpuSets;
pub use default_features::DefaultFeatures;
pub use exclude::Exclude;
pub use feature_set::FeatureSet;
pub use invariant::Invariant;
pub use manifest::Manifest;
pub use parallel::Parallel;
pub use profile::Profile;
pub use reference::Ref;
pub use workload::Workload;
