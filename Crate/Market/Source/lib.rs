//! Model-only re-synthesis of the v5 federated research economy
//! (`Research/prun_optimization_v5/`).
//!
//! This crate models opinion/fact separation, non-transferable multi-resource
//! research credit, the claim -> paper -> sanitized publication pipeline,
//! separated institutions and the repository-adapter contract. It is **not**
//! an operational research service, an auction engine, a wallet, a token or a
//! cryptocurrency (`README.md:3-7`, `MARKET.md:1-19`). No bid is executed, no
//! credit is minted or transferred, no process is spawned, no file is opened,
//! no signature is created or verified, no paper is promoted and no source is
//! declassified. Every rule below is a typed value or a named transition, and
//! every reported deviation is a [`Warning`] or [`Permission`], never a panic.
//!
//! Foundational boundary (`README.md:39-49`, `SYSTEM.md:118-122`): independent
//! opinions may conflict; measured facts come only from signed, independently
//! produced evidence. Neither funding, votes, persuasive prose nor a valid CA
//! signature substitutes for correctness, replication or benchmark
//! confirmation. An LLM-supplied artifact is an untrusted proposer; trusted
//! selection, evidence, scheduling, promotion and release stay deterministic.
//!
//! Grounding (all read in full before writing):
//! - `README.md` - foundational distinctions and integration order.
//! - `architecture/SYSTEM.md` - institution graph, authority vs knowledge,
//!   case lifecycle, decision states, recursion, scaling.
//! - `architecture/MARKET.md` - resource vector, two-step assignment, scoring,
//!   ledger semantics, Goodhart controls, epochs and KPIs.
//! - `architecture/AGENTS.md` - role taxonomy, blind phases, independence.
//! - `architecture/PUBLISHING.md` - status ladder, assembly, privacy, evidence
//!   quality, paper format, cross-repository transfer.
//! - `architecture/SECURITY.md` - v4 reuse, principals, envelopes, publication
//!   bridge, supply-chain limits.
//! - `architecture/REPOSITORIES.md` - adapter contract, ontology, deployment.
//! - `architecture/TELEMETRY.md` - three planes, cockpit, indicators.
//! - `architecture/OPERATIONS.md` - week loop, failure modes, acceptance.
//! - `architecture/V4_TRUST_BRIDGE.md` - role payloads, disclosure, rotation.
//! - `schemas/*.v5.schema.json` and `examples/*.v5.json` - validated contracts.
//! - `sql/v5_migration.sql` - 16 additive tables and 14 immutability triggers.
//! - `graphs/research-economy.mmd` - typed research-economy graph.
//! - `playbooks/research-market-week.yaml` - campaign funding and policy.
//! - `rust/market-prototype/src/main.rs` - *uncompiled* deterministic planning
//!   allocator, read as a specification, never executed or copied verbatim.
//!
//! Cross-crate couplings are recorded as typed path names, not Rust imports of
//! sibling crates this card must not edit. On this case-sensitive volume the
//! physical directories are PascalCase, so every module is declared through an
//! explicit `#[path]` attribute and a lowercase Rust identifier.

#![forbid(unsafe_code)]

#[path = "Fault.rs"]
mod fault;
#[path = "Warning.rs"]
mod warning;
#[path = "Permission.rs"]
mod permission;
#[path = "Digest.rs"]
mod digest;

pub use digest::Digest;
pub use fault::Fault;
pub use permission::Permission;
pub use warning::Warning;

#[path = "Institution/mod.rs"]
pub mod institution;
#[path = "Case/mod.rs"]
pub mod case;
#[path = "Claim/mod.rs"]
pub mod claim;
#[path = "Paper/mod.rs"]
pub mod paper;
#[path = "Bid/mod.rs"]
pub mod bid;
#[path = "Credit/mod.rs"]
pub mod credit;
#[path = "Repository/mod.rs"]
pub mod repository;
#[path = "Publish/mod.rs"]
pub mod publish;
#[path = "Graph/mod.rs"]
pub mod graph;
#[path = "Telemetry/mod.rs"]
pub mod telemetry;

/// The v5 document schema version (`schemas/*.v5.schema.json:25`, all examples).
pub const SCHEMA_VERSION: u32 = 5;

/// Native resource dimensions in the owner-funded vector (`MARKET.md:9`,
/// `sql/v5_migration.sql:137-143`). They are booked in native units, never
/// converted into one fungible balance.
pub const RESOURCE_DIMENSIONS: usize = 7;

/// Fixed-point scale for accounting and scoring (`MARKET.md:18-19`): integer
/// millionths replace binary floating point in caps and prices.
pub const FIXED_SCALE: u64 = 1_000_000;

/// Bounded campaign duration (`playbooks/research-market-week.yaml:4`,
/// `OPERATIONS.md:3`).
pub const WEEK_HOURS: u32 = 168;

/// Exclusive benchmark runners per host (`playbooks/research-market-week.yaml:15`).
pub const BENCHMARK_RUNNERS_PER_HOST: u8 = 1;

/// Protected dissent reserve (`playbooks/research-market-week.yaml:51`).
pub const DISSENT_RESERVE_MILLI: u32 = 150;

/// Non-discretionary replication reserve (`playbooks/research-market-week.yaml:52`,
/// `MARKET.md:102-104`).
pub const REPLICATION_RESERVE_MILLI: u32 = 200;

/// Maximum share of a campaign one bid may request
/// (`playbooks/research-market-week.yaml:53`).
pub const MAXIMUM_BID_MILLI: u32 = 100;

/// Maximum simultaneous agent assignments
/// (`playbooks/research-market-week.yaml:54`).
pub const MAX_ACTIVE_ASSIGNMENTS: u8 = 20;

/// The v4 identity model this economy binds claims to, by typed name
/// (`V4_TRUST_BRIDGE.md:19-31`). No Rust dependency on `Crate/Trust`.
pub const TRUST_IDENTITY_PATH: &str = "trust::identity::identity::Identity";

/// The v4 publication signature contract this economy's stamp extends, by name
/// (`V4_TRUST_BRIDGE.md:27,49-55`). No Rust dependency on `Crate/Trust`.
pub const TRUST_SIGNATURE_PATH: &str = "trust::bridge::signature::Signature";

/// The v3 attestation type an `experiment.result.attest` payload feeds
/// (`V4_TRUST_BRIDGE.md:23`, `EXPERIMENT_ATTESTATION` in `Crate/Experiment`).
pub const EXPERIMENT_ATTESTATION_PATH: &str = "experiment::interface::attestation::Attestation";

/// The v3 release decision a publication stamp must not fabricate
/// (`V4_TRUST_BRIDGE.md:25`, `SYSTEM.md:96-99`).
pub const EXPERIMENT_RELEASE_PATH: &str = "experiment::queue::release::Release";

/// The v2 requirement catalog a research case inherits its campaign from
/// (`README.md:66-68`, `Crate/Campaign`).
pub const CAMPAIGN_REQUIREMENT_PATH: &str = "campaign::requirement::Requirement";

/// The v0 configuration type an adapter's workload dimension rests on
/// (`README.md:66-68`, `REPOSITORIES.md:3-11`).
pub const BENCHMARK_CONFIG_PATH: &str = "benchmark::config::Manifest";
