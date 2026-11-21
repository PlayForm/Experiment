//! Model-only re-synthesis of the v6.2 cryptographic schema fabric
//! (`Research/prun_laboratory_v62/README.md`, `docs/01` through `docs/10`,
//! `schemas/*.schema.json`, `examples/*.json`, `tests/validate.py`,
//! `VALIDATION_REPORT.md`, `MANIFEST.sha256`, and the uncompiled reference
//! `rust/lab-identifier/src/lib.rs`).
//!
//! This crate models, as typed values and named transitions: opaque
//! scope-specific identifiers with fail-closed collision and version checks;
//! dedicated key custody where a genesis commitment is not a master key; a
//! private schema IR with deterministic compilation, a sealed resolver and a
//! separate Cedar-like policy; an encrypted append-only journal with arenas,
//! epochs, signed migrations and crash recovery; privacy-preserving projections
//! with audited privileged views; generic opaque wire envelopes; an assurance
//! plan with explicit limitations; and container provisioning that gives no
//! worker keys, policy registry, internal API or egress.
//!
//! It stages no bytes, generates no key, performs no encryption, signature,
//! decryption, hashing, KDF, network call or policy interpretation at runtime.
//! Digest and identifier checks inspect shape only. No embedded constant is a
//! secret. No generated artifact is given authority over measurement or
//! policy. Honest status: proposed contracts and design plus schema-level
//! validation only — no audited cryptography, no enclave, no operational
//! proof (`README.md:3-6`, `docs/09:65-66`, `VALIDATION_REPORT.md:14-20`).
//!
//! Cross-crate couplings are typed path names, not imports of sibling crates
//! this card must not edit. Domain directories are PascalCase on a
//! case-sensitive volume, so every module is declared through an explicit
//! `#[path]` attribute and a lowercase Rust identifier.

#![forbid(unsafe_code)]

#[path = "Verdict.rs"]
pub mod verdict;
#[path = "Warning.rs"]
pub mod warning;
#[path = "Digest.rs"]
pub mod digest;
#[path = "Status.rs"]
pub mod status;
#[path = "Stage.rs"]
pub mod stage;
#[path = "Severity.rs"]
pub mod severity;
#[path = "Network.rs"]
pub mod network;

#[path = "Identifier/mod.rs"]
pub mod identifier;
#[path = "Key/mod.rs"]
pub mod key;
#[path = "Schema/mod.rs"]
pub mod schema;
#[path = "Policy/mod.rs"]
pub mod policy;
#[path = "Memory/mod.rs"]
pub mod memory;
#[path = "Privacy/mod.rs"]
pub mod privacy;
#[path = "Envelope/mod.rs"]
pub mod envelope;
#[path = "Assurance/mod.rs"]
pub mod assurance;
#[path = "Container/mod.rs"]
pub mod container;

/// Declared wire revision (`schemas/opaque-envelope.schema.json:4`, `v` const).
pub const PROTOCOL_REVISION: &str = "6.2";
/// Length of a lowercase-hex SHA-256 identity (`tests/generate_contracts.py:11`).
pub const DIGEST_LENGTH: usize = 64;
/// Base64url-without-padding length of a 32-byte opaque route
/// (`docs/03:9`, `tests/generate_contracts.py:10`).
pub const OPAQUE_ID_LENGTH: usize = 43;
/// Documentation-only schema base; no network resolution is permitted
/// (`tests/generate_contracts.py:9`).
pub const CONTRACT_BASE: &str = "https://contracts.example.invalid/v6.2/";
/// The fixed generic envelope; all semantic fields live inside ciphertext
/// (`docs/05:47-48`, `schemas/opaque-envelope.schema.json`).
pub const ENVELOPE_FIELDS: [&str; 6] = ["v", "rt", "e", "enc", "ct", "s"];
/// The only public-facing envelope shape named in the design note
/// (`docs/05:47`).
pub const PUBLIC_FIELDS: [&str; 4] = ["v", "rt", "e", "body"];
/// Opaque URN prefix emitted only for a trusted resolver
/// (`docs/05:19`, `docs/06:4`).
pub const SCHEMA_URN: &str = "urn:lab:schema:";
/// The trusted IPC boundary; the worker can never reach it
/// (`docs/08:22`, `README.md:20-21`).
pub const TRUSTED_IPC: &str = "trusted-ipc";
/// The single neutral status a worker status may carry (`docs/07:28-30`).
pub const NEUTRAL_STATUS: &str = "accepted";
/// The non-disclosure invariant for every worker-visible value
/// (`docs/07:28-30`, `docs/02:26-27`).
pub const NON_DISCLOSURE: &str =
    "policy reasons, private mappings and key handles are never worker-observable";

/// Storage owns the durable tables; this crate only names the digest chain.
pub const STORAGE_MIGRATION_PATH: &str = "storage::migration::chain::Chain";
/// Custody grants gate any release of a data-encryption key.
pub const CUSTODY_RECEIPT_PATH: &str = "custody::receipt::Receipt";
/// The journal names canonical events without duplicating the catalog.
pub const CONTRACT_EVENT_PATH: &str = "contract::event::kind::Kind";
/// No generated artifact may amend the laboratory constitution.
pub const LABORATORY_CONSTITUTION_PATH: &str = "laboratory::constitution::invariant::Invariant";
/// Grant signatures are bound here but verified by the trust bridge.
pub const TRUST_SIGNATURE_PATH: &str = "trust::bridge::signature::Signature";
/// Released knowledge claims may reference a bound capability.
pub const MARKET_CLAIM_PATH: &str = "market::claim::claim::Claim";
/// No worker claim or generated stamp can fabricate a release decision.
pub const EXPERIMENT_RELEASE_PATH: &str = "experiment::evaluate::Release";
