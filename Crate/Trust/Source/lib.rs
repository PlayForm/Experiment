//! Model-only re-synthesis of the v4 Rust trust-fabric PROTOTYPE
//! (`Research/prun_optimization_v4/security/`) and its v5 signed-claim bridge
//! (`Research/prun_optimization_v5/architecture/V4_TRUST_BRIDGE.md`).
//!
//! Honest status: experimental, not production-ready, never compiled in the
//! corpus. The prototype was never built, tested for races or interoperability,
//! or deployment-verified (`security/README.md:11-15`, `V4_TRUST_BRIDGE.md:3-9`).
//! This crate models its protocol and boundaries as typed values and named
//! transitions. It holds NO keys, NO secrets, NO network, NO crypto execution
//! and NO Docker behaviour: no byte is encrypted, no socket is opened, no file
//! is written, no certificate is signed and no signature is verified. NO
//! security guarantee is claimed or provided by this crate.
//!
//! Grounding (all read in full before writing):
//! - `security/README.md` - prototype status, offline-root rule, restricted
//!   relay-only staging, wire protocol is length-prefixed JSON over TLS.
//! - `security/src/main.rs` - CLI verbs; the relay path never loads the root.
//! - `security/src/crypto.rs` - Ed25519 identity attestation, signed HPKE
//!   envelope, domain separation, validity and size checks, replay mark-once.
//! - `security/src/storage.rs` - file-backed PKI, 0600/0700 modes,
//!   hash-chained signed journals, fail-closed revocation, staged runtime view.
//! - `security/src/relay.rs` - rustls mTLS, byte-length prefix, peer from
//!   certificate fingerprint, no HPKE private keys, serialized filesystem.
//! - `security/src/ssh_ca.rs` - short-lived OpenSSH user certificates.
//! - `security/src/openpgp.rs` - optional, feature-gated helper only.
//! - `security/dns/lab.internal.zone`, `security/dns/named.toml` - private zone.
//! - `security/deploy/*` - demonstration-only container hardening.
//! - `V4_TRUST_BRIDGE.md` - purpose capabilities, disclosure policy, key
//!   rotation, and the non-conflation of CA signatures with scientific validity.
//! - v4 `README.md`, `ARCHITECTURE.md`, `AGENT_CONTRACTS.md`, `RESEARCH_GATE.md`,
//!   `STORAGE.md`, `TELEMETRY.md`, `DASHBOARD.md`, `CONTAINER_MATRIX.md`,
//!   `RUNBOOK.md` - the surrounding deterministic-core / reasoning-edge design.
//!
//! On this case-sensitive volume the directories are PascalCase, so each Rust
//! module is declared through an explicit `#[path]` attribute and a lowercase
//! Rust identifier, matching the sibling crates.

#![forbid(unsafe_code)]

#[path = "Fault.rs"]
pub mod fault;
#[path = "Warning.rs"]
pub mod warning;
#[path = "Permission.rs"]
pub mod permission;

#[path = "Ca/mod.rs"]
pub mod ca;
#[path = "Identity/mod.rs"]
pub mod identity;
#[path = "Crypto/mod.rs"]
pub mod crypto;
#[path = "Relay/mod.rs"]
pub mod relay;
#[path = "Ssh/mod.rs"]
pub mod ssh;
#[path = "OpenPgp/mod.rs"]
pub mod open_pgp;
#[path = "Dns/mod.rs"]
pub mod dns;
#[path = "Storage/mod.rs"]
pub mod storage;
#[path = "Bridge/mod.rs"]
pub mod bridge;

/// Envelope version the prototype seals and verifies (`crypto.rs:161`).
pub const PROTOCOL_VERSION: u32 = 4;

/// Maximum plaintext accepted by `seal` (`crypto.rs:146`): 8 MiB.
pub const MAX_MESSAGE_BYTES: usize = 8 * 1024 * 1024;

/// Maximum application frame accepted by the relay (`relay.rs:26`): 9 MiB.
pub const MAX_FRAME_BYTES: usize = 9 * 1024 * 1024;

/// Ed25519 / HPKE raw key length enforced by `fixed_32` (`crypto.rs:81-84`).
pub const KEY_BYTES: usize = 32;

/// Nonce and message-ID length (`crypto.rs:157`, `crypto.rs:193`).
pub const NONCE_BYTES: usize = 16;

/// Private-zone record TTL (`dns/lab.internal.zone:2`).
pub const DNS_TTL_SECONDS: u32 = 60;

/// Typed name of the v3 attestation model the v5 bridge extends with signed
/// scientific claims. Recorded as a name, not a Rust dependency on a crate a
/// later card owns (`V4_TRUST_BRIDGE.md:23`).
pub const EXPERIMENT_ATTESTATION_PATH: &str = "experiment::interface::attestation::Attestation";

/// Typed name of the v3 release decision a `publication.release` payload feeds
/// (`V4_TRUST_BRIDGE.md:25`); a coupling by name, never an import.
pub const EXPERIMENT_RELEASE_PATH: &str = "experiment::queue::release::Release";
