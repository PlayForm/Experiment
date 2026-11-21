//! Clean-room model of the v0 `prun_lab` Docker/Hyperfine benchmark
//! laboratory (Research/prun_lab/README.md, prun_lab.py).
//!
//! This crate models commands, artifacts and measurements as typed data. It
//! spawns no processes, writes no files, and grants no generated artifact
//! authority over measurement or policy. Later crates attach execution.
//!
//! Every domain directory is PascalCase and reached through an explicit
//! `#[path]` attribute so the lowercase Rust module name resolves on the
//! case-sensitive volume that hosts the tree.

#![forbid(unsafe_code)]

#[path = "Fault.rs"]
mod fault;
#[path = "Warning.rs"]
mod warning;

pub use fault::Fault;
pub use warning::Warning;

#[path = "Config/mod.rs"]
pub mod config;
#[path = "Plan/mod.rs"]
pub mod plan;
#[path = "Fixture/mod.rs"]
pub mod fixture;
#[path = "Build/mod.rs"]
pub mod build;
#[path = "Runner/mod.rs"]
pub mod runner;
#[path = "Sample/mod.rs"]
pub mod sample;
#[path = "Report/mod.rs"]
pub mod report;
#[path = "Controller/mod.rs"]
pub mod controller;

/// The supported `prun-bench.json` schema version (schema.json:19,
/// prun_lab.py:191).
pub const CONFIG_VERSION: u32 = 1;

/// The pinned Hyperfine version shared by both runner images
/// (README.md:13-14, prun_lab.py:96).
pub const HYPERFINE_VERSION: &str = "2.0.0";
