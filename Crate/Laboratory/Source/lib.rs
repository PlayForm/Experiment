//! Model-only re-synthesis of the v6.1 laboratory control plane
//! (`Research/prun_laboratory_v6_1/`).
//!
//! This crate models the CONSTITUTION and TOPOLOGY of the deterministic
//! laboratory: what may never happen, the three trusted planes plus the offline
//! worker boundary, the evidence hierarchy, deterministic control state,
//! the benchmark mesh, agent contracts, repository-agnostic admission and
//! crash recovery. It is **not** a deployed laboratory, an HTTP service, a
//! sandbox, a PKI, a benchmark runner or a policy engine (`README.md:3-7`,
//! `docs/16-completeness-and-open-questions.md:24-33`). No API is served, no
//! subprocess is spawned, no container is controlled, no signature is created
//! or verified, no model is invoked and no measurement is produced. Every rule
//! below is a typed value or a named transition; every deviation is a
//! [`Warning`] or [`Permission`], never a panic.
//!
//! Foundational boundary (`docs/00-system-constitution.md:17-55`,
//! `docs/17-v61-constitution.md:5-63`): the only untrusted generative input is
//! LLM output, accepted solely as a staged artifact. Selection, evidence,
//! scheduling, promotion, custody and release are deterministic trusted
//! software. The state transition function is deterministic given a pinned
//! policy and an ordered authoritative event log; wall-clock and hardware
//! measurements are noisy observations, never control inputs.
//!
//! Grounding (all read in full before writing):
//! - `README.md` - one external API, two telemetry planes, zero LLM control.
//! - `docs/00` .. `docs/16` - constitution, topology, APIs, events, states,
//!   telemetry, storage, trust, mesh, agents, dashboard, recovery, acceptance.
//! - `docs/17` - v6.1 constitutional amendment and authority separation.
//! - `docs/22` - hidden offline worker and non-disclosing enforcement.
//! - `docs/23` - interface inventory (24 + 27 + curriculum operations).
//! - `docs/24` - dual observability, telemetry is never canonical evidence.
//! - `docs/25` - failure register F01..F30 and explicit security limits.
//! - `docs/26` - all-Rust implementation components and honest completion.
//! - `architecture/*.mmd` - laboratory, offline sequence, custody and
//!   knowledge flows.
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

#[path = "Constitution/mod.rs"]
pub mod constitution;
#[path = "Plane/mod.rs"]
pub mod plane;
#[path = "Evidence/mod.rs"]
pub mod evidence;
#[path = "State/mod.rs"]
pub mod state;
#[path = "Mesh/mod.rs"]
pub mod mesh;
#[path = "Agent/mod.rs"]
pub mod agent;
#[path = "Dashboard/mod.rs"]
pub mod dashboard;
#[path = "Repository/mod.rs"]
pub mod repository;
#[path = "Runtime/mod.rs"]
pub mod runtime;

/// Documented Plane 0 HTTP operations (`docs/03-external-api.md:32-78`,
/// `README.md:9`).
pub const PUBLIC_OPERATIONS: u8 = 67;

/// Documented Plane 1 internal HTTP operations (`docs/04-internal-api.md:26-55`,
/// `README.md:11`).
pub const INTERNAL_OPERATIONS: u8 = 53;

/// Trusted planes; Boundary X is deliberately not counted
/// (`docs/01-topology-and-boundaries.md:41-47`).
pub const TRUSTED_PLANES: u8 = 3;

/// Rungs in the canonical evidence hierarchy (`docs/00:49-51`).
pub const EVIDENCE_RANKS: u8 = 7;

/// The only two POST operations exposed on Plane 0
/// (`docs/03-external-api.md:24-26`).
pub const OWNER_WRITE_OPERATIONS: u8 = 2;

/// Maximum page size on a metadata projection (`docs/03-external-api.md:17`).
pub const PUBLIC_PAGE_LIMIT: u16 = 500;

/// Separately authenticated telemetry domains (`docs/07-dual-telemetry.md:8-20`).
pub const TELEMETRY_PLANES: u8 = 2;

/// The v4 identity model a trusted service may hold but a worker never sees,
/// recorded by typed name (`docs/09-cryptographic-trust.md:31-49`).
pub const TRUST_IDENTITY_PATH: &str = "trust::identity::identity::Identity";

/// The v3 release decision no generated stamp can fabricate, by typed name
/// (`docs/00-system-constitution.md:53-55`).
pub const EXPERIMENT_RELEASE_PATH: &str = "experiment::queue::release::Release";

/// The v5 federated economy this laboratory's publication bridge feeds, by
/// typed name (`docs/17-v61-constitution.md:96`, `docs/26:22-24`).
pub const MARKET_CLAIM_PATH: &str = "market::claim::claim::Claim";

/// The v0 runnable benchmark configuration a Mesh workload dimension rests on,
/// by typed name (`docs/10-benchmark-mesh.md:41-47`, `docs/13-repository-agnostic.md:17-30`).
pub const BENCHMARK_CONFIG_PATH: &str = "benchmark::config::Manifest";

/// The later contract card that owns concrete request/response field lists;
/// this crate owns only the constitution and topology
/// (`docs/23-interface-inventory.md:5-15`).
pub const CONTRACT_CATALOG_PATH: &str = "contract::catalog::Catalog";
