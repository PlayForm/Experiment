//! Model-only re-synthesis of the v6.1 machine-readable contract surface
//! (`Research/prun_laboratory_v6_1/`).
//!
//! This crate models the CONTRACT: operation catalogs, the signed event
//! taxonomy, schema and example families, versioned policy data and structural
//! validation. It is **not** a served HTTP API, an event bus, a JSON Schema
//! engine, a policy decision point or a validator execution. No route is
//! mounted, no message is signed or verified, no document is read from disk,
//! no policy is applied and no measurement is produced. Every rule below is a
//! typed value or a named transition; every deviation is a [`Warning`] or a
//! [`Permission`], never a panic (`docs/26-acceptance-and-build-plan.md:82-89`).
//!
//! Grounding (all read in full or sampled across families before writing):
//! - `api/HTTP_INDEX.md:7-128` - 67 external and 53 internal operations.
//! - `api/public.openapi.yaml`, `api/internal.openapi.yaml` - OpenAPI 3.1.1
//!   metadata, mTLS security, operation IDs and `$ref` response schemas.
//! - `api/events.asyncapi.yaml:1-199` - AsyncAPI 3.1.0, 15 channels, one
//!   `AuditEvent` message, trusted subscribers only.
//! - `events/catalog.json` - 87 signed event kinds (57 canonical + 30 v6.1).
//! - `events/correspondence-catalog.json:1-208` - 29 typed transfers.
//! - `policy/*.json`, `policy/*.yaml` - roles, trust domains, record classes,
//!   state machines and dual telemetry.
//! - `docs/05`, `docs/23`, `docs/26` - correspondence, interface inventory,
//!   acceptance honesty.
//! - `schemas/*.schema.json`, `examples/*.example.json` - 2020-12 families.
//! - `MANIFEST.sha256:1-295` - content-addressed catalog identity scheme.
//!
//! This crate does NOT duplicate `Crate/Laboratory`'s constitution rules. The
//! deterministic-kernel and authority-separation invariants live there and are
//! referenced here only by typed path name.
//!
//! House-style exception: card-named compound types (`AccessMatrix`,
//! `StateMachine`, `TelemetryContract`) and canonical wire event-kind variants
//! are kept byte-stable with the contract they model; a rename would break the
//! very identifiers this crate exists to describe.

#![forbid(unsafe_code)]

#[path = "Warning.rs"]
mod warning;
#[path = "Fault.rs"]
mod fault;
#[path = "Permission.rs"]
mod permission;
#[path = "Digest.rs"]
mod digest;
#[path = "Catalog.rs"]
pub mod catalog;

pub use digest::Digest;
pub use fault::Fault;
pub use permission::Permission;
pub use warning::Warning;

#[path = "Api/mod.rs"]
pub mod api;
#[path = "Event/mod.rs"]
pub mod event;
#[path = "Schema/mod.rs"]
pub mod schema;
#[path = "Example/mod.rs"]
pub mod example;
#[path = "Policy/mod.rs"]
pub mod policy;
#[path = "Validate/mod.rs"]
pub mod validate;

/// External (operator/observer) HTTP operations (`HTTP_INDEX.md:9-75`,
/// `docs/03-external-api.md:32-78`).
pub const EXTERNAL_OPERATIONS: usize = 67;

/// Internal trusted-service HTTP operations (`HTTP_INDEX.md:76-128`,
/// `docs/04-internal-api.md:26-55`); 26 base plus 27 v6.1 additions.
pub const INTERNAL_OPERATIONS: usize = 53;

/// v6.1 internal service operations (`HTTP_INDEX.md:102-128`,
/// `docs/23-interface-inventory.md:43-68`).
pub const INTERNAL_NEW_OPERATIONS: usize = 27;

/// v6.1 public projections (`HTTP_INDEX.md:52-75`,
/// `docs/23-interface-inventory.md:17-35`).
pub const PUBLIC_NEW_OPERATIONS: usize = 24;

/// Canonical signed event kinds (`docs/05-event-correspondence.md:43-108`).
pub const CANONICAL_EVENTS: usize = 57;

/// v6.1 signed event kinds (`events/catalog.json`, families custody..curriculum).
pub const NEW_EVENTS: usize = 30;

/// All signed event kinds in `events/catalog.json`.
pub const TOTAL_EVENTS: usize = 87;

/// AsyncAPI channels (`api/events.asyncapi.yaml:9-101`).
pub const EVENT_CHANNELS: usize = 15;

/// Typed correspondence transfers (`correspondence-catalog.json:3-207`).
pub const CORRESPONDENCES: usize = 29;

/// Content-addressed entries in `MANIFEST.sha256`.
pub const MANIFEST_ENTRIES: usize = 295;

/// The crate-level catalog name that `Crate/Laboratory` records as a typed
/// dependency (`Crate/Laboratory/Source/lib.rs:121`; no Rust import).
pub const SELF_CATALOG_PATH: &str = "contract::catalog::Catalog";

/// The constitutional invariant vocabulary this crate references but never
/// redefines (`Crate/Laboratory/Source/Constitution/Invariant.rs`).
pub const LABORATORY_CONSTITUTION_PATH: &str =
    "laboratory::constitution::invariant::Invariant";

/// The v4 signed-claim signature this crate models by name only; no signature
/// is created or verified here (`trust::bridge::signature::Signature`).
pub const TRUST_SIGNATURE_PATH: &str = "trust::bridge::signature::Signature";

/// The v3 release decision no example or generated artifact may fabricate
/// (`docs/00-system-constitution.md:53-55`).
pub const EXPERIMENT_RELEASE_PATH: &str = "experiment::evaluate::Release";

/// The v5 signed claim a v6.1 knowledge-release record may reference, by typed
/// name (`docs/17-v61-constitution.md:96`).
pub const MARKET_CLAIM_PATH: &str = "market::claim::claim::Claim";

/// The repository-adapter model the admission contract feeds, by typed name
/// (`docs/13-repository-agnostic.md:17-30`).
pub const LABORATORY_ADAPTER_PATH: &str = "laboratory::repository::adapter::Adapter";
