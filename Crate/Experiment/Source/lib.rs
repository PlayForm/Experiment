//! Model-only re-synthesis of the v3 autonomous recursive optimization
//! laboratory (`Research/prun_optimization_v3/`): the deterministic-coordinator
//! / replaceable-reasoning-edge architecture.
//!
//! This crate is a typed value-and-transition model. It does not schedule a
//! task, hold a model conversation, compile a binary, run an oracle, take a
//! benchmark lease, sign a measurement, change active policy, open a database,
//! or emit telemetry. Every leaf models a rule from the v3 design as a typed
//! value or a named transition. The LLM is an untrusted proposer of staged
//! artifacts; trusted selection, evidence, scheduling, promotion and release
//! are deterministic software. An LLM never signs a measurement and never
//! changes active policy (`ARCHITECTURE.md:28-31`, `AGENT_CONTRACTS.md:1-23`,
//! `REQUIREMENTS_V3.md` INV-001/INV-005/INV-009).
//!
//! Grounding (all read in full before writing):
//! - `README.md`, `ARCHITECTURE.md` - topology, trust zones, state machine,
//!   five-candidate fanout, containment zones.
//! - `REQUIREMENTS_V3.md` - INV/AGT3/NOV/CTX/TEL/EXP/DAT families.
//! - `AGENT_CONTRACTS.md`, `RESEARCH_GATE.md` - role taxonomy, restricted
//!   context, four-pass gate, feature collision, evidence stamps.
//! - `STORAGE.md`, `TELEMETRY.md`, `DASHBOARD.md`, `RUNBOOK.md`,
//!   `CONTAINER_MATRIX.md` - event sourcing, two telemetry boards, rollout.
//! - `interfaces/*.schema.json` and `examples/*.json` - the five typed
//!   interface mirrors (schema version 3).
//! - `playbooks/fanout-5.yaml`, `sql/v3_migration.sql`,
//!   `telemetry/otel-collector-optional.yaml`.
//!
//! Cross-crate coupling is expressed by typed path/name values, not by hard
//! dependencies on sibling crates this card must not edit: `Benchmark::Config`,
//! `Runner::Event`, and `Campaign::Requirement` are consumed as names.
//!
//! On this case-sensitive volume the physical directories are PascalCase, so
//! every Rust module is declared through an explicit `#[path]` attribute and a
//! lowercase Rust identifier.

#![forbid(unsafe_code)]

#[path = "Fault.rs"]
pub mod fault;
#[path = "Warning.rs"]
pub mod warning;
#[path = "Permission.rs"]
pub mod permission;

#[path = "Coordinator/mod.rs"]
pub mod coordinator;
#[path = "Director/mod.rs"]
pub mod director;
#[path = "Patcher/mod.rs"]
pub mod patcher;
#[path = "Gate/mod.rs"]
pub mod gate;
#[path = "Build/mod.rs"]
pub mod build;
#[path = "Oracle/mod.rs"]
pub mod oracle;
#[path = "Queue/mod.rs"]
pub mod queue;
#[path = "Evaluate/mod.rs"]
pub mod evaluate;
#[path = "Interface/mod.rs"]
pub mod interface;
#[path = "Telemetry/mod.rs"]
pub mod telemetry;
#[path = "Storage/mod.rs"]
pub mod storage;

/// The v3 interface document schema version (`interfaces/*.schema.json:20-22`,
/// `examples/*.json:2`).
pub const SCHEMA_VERSION: u32 = 3;

/// The playbook schema version (`playbooks/fanout-5.yaml:1`).
pub const PLAYBOOK_VERSION: u32 = 3;

/// The v2 campaign schema this overlay extends without replacing it
/// (`README.md:5-7`, `examples/campaign-overlay.v3.json:3`).
pub const EXTENDS_CAMPAIGN_SCHEMA: u32 = 2;

/// The configured five-way fanout (`ARCHITECTURE.md:64-96`,
/// `playbooks/fanout-5.yaml:35-37`).
pub const FANOUT: usize = 5;

/// The maximum number of parallel measurements per benchmark host
/// (`playbooks/fanout-5.yaml:69-73`, `campaign-overlay.v3.json:10`).
pub const MAX_PARALLEL_MEASUREMENTS: usize = 1;

/// The name of the v0 configuration model this overlay extends, by path
/// (`Benchmark/Source/Config/Manifest.rs`); a hard dependency is deliberately
/// avoided.
pub const BENCHMARK_CONFIG_PATH: &str = "benchmark::config::Manifest";

/// The name of the v0 event stream consumed as the oracle observation
/// vocabulary (`Runner/Source/Event/Kind.rs`); an LLM never signs these events.
pub const RUNNER_EVENT_PATH: &str = "runner::event::Kind";

/// The name of the v2 requirement model extended by this design
/// (`Campaign/Source/Requirement/Requirement.rs`).
pub const CAMPAIGN_REQUIREMENT_PATH: &str = "campaign::requirement::Requirement";
