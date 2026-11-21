//! Crate/Loop: the bounded, auto-recursive self-improvement loop for the
//! system under test at `Module/Run`, which this crate never touches.
//!
//! The loop repeatedly proposes, builds, verifies, measures, evaluates and
//! promotes staged candidate changes while leaving the SUT submodule
//! untouched. Authority follows v6.1/v6.2: the LLM is an untrusted proposer
//! of staged artifacts; trusted deterministic software owns selection,
//! measurement, evidence, promotion and release.
//!
//! Grounding (all read before writing):
//! - `prun_optimization_v3/ARCHITECTURE.md`, `RESEARCH_GATE.md` - deterministic
//!   coordinator, five-way fanout, four-pass novelty gate, oracle, exclusive
//!   queue, statistical evaluator, state machine and stopping policy.
//! - `prun_optimization_v2/playbooks/AGENT_CONTRACT.md`, `autonomous-week.yaml`
//!   - one conceptual mutation, bounded 168-hour budget, recovery classes.
//! - `prun_optimization_v5/architecture/{OPERATIONS,AGENTS,MARKET,PUBLISHING}.md`
//!   - roles, two-timescale feedback, embargo, Pareto preservation, negative
//!     evidence and the publication ladder.
//! - `prun_laboratory_v6_1/docs/{00,17,18,22,26}.md` - constitutional
//!   invariants, emergent (non-GA) evolution, offline worker boundary and
//!   executable acceptance plan.
//! - `prun_laboratory_v62/docs/{02,06}.md` - trusted boundaries and sealed
//!   in-memory lineage with recoverable migrations.
//! - Public type names of Benchmark, Runner, Campaign and Experiment on disk.
//!
//! Cross-crate coupling uses real typed imports for the four crates whose
//! exact public shapes are verified, and typed path constants for the rest.
//! No generated artifact gains authority over measurement or policy here.

#![forbid(unsafe_code)]

#[path = "Fault.rs"]
pub mod fault;
#[path = "Warning.rs"]
pub mod warning;

#[path = "Loop/mod.rs"]
pub mod r#loop;
#[path = "Observe/mod.rs"]
pub mod observe;
#[path = "Propose/mod.rs"]
pub mod propose;
#[path = "Novelty/mod.rs"]
pub mod novelty;
#[path = "Verify/mod.rs"]
pub mod verify;
#[path = "Measure/mod.rs"]
pub mod measure;
#[path = "Evaluate/mod.rs"]
pub mod evaluate;
#[path = "Promote/mod.rs"]
pub mod promote;
#[path = "Feedback/mod.rs"]
pub mod feedback;
#[path = "Candidates/mod.rs"]
pub mod candidates;
#[path = "Entry/mod.rs"]
pub mod entry;

/// The loop model schema version (a new policy version does not retroactively
/// change an older pinned outcome; `00-system-constitution.md:57-61`).
pub const SCHEMA_VERSION: u32 = 6;

/// The bounded autonomous-week duration (`autonomous-week.yaml:9`,
/// `campaign::WEEK_HOURS`).
pub const WEEK_HOURS: u32 = campaign::WEEK_HOURS;

/// The configured five-way fanout (`ARCHITECTURE.md:64-96`,
/// `experiment::FANOUT`).
pub const FANOUT: usize = experiment::FANOUT;

/// The only directory where a materialised candidate `.rs` file may be
/// written; the SUT submodule is never a target.
pub const CANDIDATE_ROOT: &str = "Crate/Loop/Source/Candidates/";

/// The v0 configuration model consumed as a name (`Benchmark/Source/Config`).
pub const BENCHMARK_CONFIG_PATH: &str = "benchmark::config::Manifest";
/// The v0 matrix model consumed as a name (`Benchmark/Source/Plan`).
pub const BENCHMARK_PLAN_PATH: &str = "benchmark::plan::Matrix";
/// The v0 runner group/lease model consumed as a typed import.
pub const BENCHMARK_LEASE_PATH: &str = "benchmark::runner::Lease";
/// The SUT CLI contract consumed as a typed import (`Runner/Source/Cli`).
pub const RUNNER_CLI_PATH: &str = "runner::cli::command::Command";
/// The authoritative evaluator verdict consumed as a typed import.
pub const VERDICT_PATH: &str = "experiment::evaluate::verdict::Verdict";
/// The v3 novelty gate consumed as a typed import.
pub const GATE_IDENTITY_PATH: &str = "experiment::gate::identity::Identity";
/// The single `prun` command surface that launches the loop.
pub const INTEGRATION_LOOP_PATH: &str = "integration::dispatch::loop_::Loop";
/// A report is a projection, never authoritative measurement.
pub const INTEGRATION_REPORT_PATH: &str = "integration::report::view::View";
/// Custody grants gate any release of staged material.
pub const CUSTODY_RECEIPT_PATH: &str = "custody::custody::receipt::Receipt";
/// The append-only result store owns durable evidence.
pub const STORAGE_CHAIN_PATH: &str = "storage::migration::chain::Chain";
/// The v6.2 fabric policy decision gates any release claim; no generated
/// stamp can emit a decision.
pub const FABRIC_DECISION_PATH: &str = "fabric::policy::decision::Decision";
