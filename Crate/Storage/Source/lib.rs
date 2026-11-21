//! Model-only re-synthesis of the v6.1 laboratory persistent result store
//! (`Research/prun_laboratory_v6_1/sql/`, `docs/08`, `docs/14`).
//!
//! This crate represents the additive SQLite/WAL migration chain as typed
//! schemas and named transitions. It executes no SQL, opens no database,
//! writes no `.sql` file and keeps no runtime state in JSONL. A destructive
//! migration is classified and reported as a warning-level finding; this
//! crate never applies one. An in-memory projection is never the only copy:
//! the append-only ledger and its signed events remain the source of truth
//! from which projections are rebuilt (`docs/08-data-and-storage.md:3-24`,
//! `docs/14-runtime-and-recovery.md:24-40`).
//!
//! Grounding (all read in full before writing):
//! - `sql/prerequisites/v2_lineage.sql` - WAL, lineage tables, events,
//!   immutable-evidence triggers and the first five indexes.
//! - `sql/prerequisites/v3_migration.sql` - work orders, leases, turns,
//!   attestations, outbox and the append-only audit tables.
//! - `sql/prerequisites/v5_migration.sql` - research economy, seven
//!   non-fungible resource dimensions, reservations and superseding records.
//! - `sql/v6_migration.sql` - canonical event journal, command inbox,
//!   projection outbox, correspondence, leases, credentials, telemetry refs,
//!   correlation indexes and immutability triggers.
//! - `sql/v61_migration.sql` - custody, policy, tenancy, curriculum, study
//!   and temporal-fact extension; no existing table is modified.
//! - `docs/08-data-and-storage.md`, `docs/14-runtime-and-recovery.md`.
//! - The v2/v3/v5 optimization SQL files are byte-identical copies of the
//!   laboratory prerequisites and add no separate rule.
//!
//! Cross-crate couplings are recorded as typed path names, not Rust imports
//! of sibling crates this card must not edit. On this case-sensitive volume
//! the physical directories are PascalCase, so every module is declared
//! through an explicit `#[path]` attribute and a lowercase Rust identifier.

#![forbid(unsafe_code)]

#[path = "Warning.rs"]
pub mod warning;
#[path = "Permission.rs"]
pub mod permission;

#[path = "Schema/mod.rs"]
pub mod schema;
#[path = "Migration/mod.rs"]
pub mod migration;
#[path = "Ledger/mod.rs"]
pub mod ledger;
#[path = "Cas/mod.rs"]
pub mod cas;
#[path = "Lease/mod.rs"]
pub mod lease;
#[path = "Lineage/mod.rs"]
pub mod lineage;
#[path = "Recovery/mod.rs"]
pub mod recovery;

/// Apply order of the additive chain (`docs/08-data-and-storage.md:70-80`).
pub const CHAIN_ORDER: &[&str] = &["v2", "v3", "v5", "v6", "v6.1"];

/// Length of a SHA-256 hex digest as a string, checked on every digest column
/// in the chain (`sql/v6_migration.sql:17-20`).
pub const DIGEST_LENGTH: usize = 64;

/// The abstract CAS identity scheme, never a grant by itself
/// (`docs/08-data-and-storage.md:26-31`).
pub const CAS_SCHEME: &str = "cas://sha256/";

/// The authoritative event catalog owned by the contract crate; this crate
/// names journal event kinds from it but does not duplicate its 87 entries.
pub const CONTRACT_EVENT_PATH: &str = "contract::event::kind::Kind";

/// The constitutional rule that generated artifacts never gain authority over
/// measurement or policy (`docs/14-runtime-and-recovery.md:42-48`).
pub const LABORATORY_CONSTITUTION_PATH: &str = "laboratory::constitution::invariant::Invariant";

/// The two-phase custody receipt model owned by the sibling custody card;
/// migration v6.1 stores its digests but never creates a receipt.
pub const CUSTODY_RECEIPT_PATH: &str = "custody::receipt::Receipt";

/// The signed claim a knowledge release may reference, by typed name
/// (`docs/17-v61-constitution.md:96`).
pub const MARKET_CLAIM_PATH: &str = "market::claim::claim::Claim";

/// The v3 release decision no generated stamp can fabricate.
pub const EXPERIMENT_RELEASE_PATH: &str = "experiment::evaluate::Release";

/// The v4 signature model that signs a journal event; this crate records the
/// digest, it does not verify the signature.
pub const TRUST_SIGNATURE_PATH: &str = "trust::bridge::signature::Signature";
