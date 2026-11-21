//! Model-only re-synthesis of the v2 adaptive-optimization DESIGN bundle
//! (`Research/prun_optimization_v2/`).
//!
//! The bundle extends the v0 `prun_lab` starter, it does not replace it: the
//! heuristic planner, autonomous week, PGO and hardware-aware release are
//! future-facing contracts and are **not** implemented here (`README.md:5-9`,
//! `:51-58`, `REQUIREMENTS.md:1145-1149`). Every leaf therefore models typed
//! values and named transitions; nothing here plans a campaign, spawns an
//! agent, opens SQLite, writes YAML/SQL/JSON, runs Hyperfine, or promotes a
//! generated artifact over measurement or policy.
//!
//! Grounding (read in full before writing):
//! - `README.md` - bundle order and the design boundary.
//! - `REQUIREMENTS.md` - 22 numbered sections, design laws, REQ-* families,
//!   acceptance gates, research questions and the lineage proof chain.
//! - `examples/campaign.v2.json` / `schemas/campaign.v2.schema.json` - the
//!   concrete v2 document and its Draft 2020-12 schema (schema version 2).
//! - `playbooks/autonomous-week.yaml` - 102-line, 168-hour playbook contract.
//! - `playbooks/AGENT_CONTRACT.md` - bounded local-agent mission and roles.
//! - `sql/lineage.sql` - illustrative SQLite/WAL event and lineage tables.
//!
//! Cross-crate coupling is expressed by typed path/name values, not by a hard
//! dependency on `Crate/Benchmark`: the v0 config/plan types are consumed as
//! names (`benchmark::config::Manifest`, `benchmark::plan::Matrix`) so this
//! crate stays independent of a sibling that it must not edit.
//!
//! On this case-sensitive volume the physical directories are PascalCase, so
//! every Rust module is declared through an explicit `#[path]` attribute.

#![forbid(unsafe_code)]

#[path = "Fault.rs"]
mod fault;
#[path = "Warning.rs"]
mod warning;

pub use fault::Fault;
pub use warning::Warning;

#[path = "Campaign/mod.rs"]
pub mod campaign;
#[path = "Requirement/mod.rs"]
pub mod requirement;
#[path = "Heuristic/mod.rs"]
pub mod heuristic;
#[path = "Week/mod.rs"]
pub mod week;
#[path = "Agent/mod.rs"]
pub mod agent;
#[path = "Lineage/mod.rs"]
pub mod lineage;

/// The v2 document schema version (`schema.json:25`, `campaign.v2.json:2`).
pub const SCHEMA_VERSION: u32 = 2;

/// The v1 plan representation version this design extends without silently
/// reinterpreting it (`README.md:7-9`, `README.md:37-38`, LAW-017).
pub const PLAN_VERSION: u32 = 1;

/// The bounded autonomous-week duration (`autonomous-week.yaml:9`,
/// `campaign.v2.json:14`).
pub const WEEK_HOURS: u32 = 168;

/// The v0 configuration type this campaign extends, by path and type name
/// (`Benchmark/Source/Config/Manifest.rs`).
pub const BENCHMARK_CONFIG_PATH: &str = "benchmark::config::Manifest";

/// The v0 matrix type the campaign dimensions sit on top of
/// (`Benchmark/Source/Plan/Matrix.rs`, `REQUIREMENTS.md:381-403`).
pub const BENCHMARK_PLAN_PATH: &str = "benchmark::plan::Matrix";
/// The SUT event model consumed as a typed path name from Runner.
pub const RUNNER_EVENT_PATH: &str = "runner::event::Kind";
