//! Model-only re-synthesis of the v6.1 custody, evidence-release and offline
//! worker amendments (`Research/prun_laboratory_v6_1/docs/17` through `22`,
//! `docs/25` through `28`, `policy/v61-roles.json`,
//! `policy/v61-state-machines.yaml`, `examples/v61-*`, `schemas/v61-*`).
//!
//! This crate represents two-phase artifact custody, holder qualification,
//! embargo/licensing/trade, bitemporal two-timescale propagation, hidden
//! worker correspondence, failure taxonomy, research curriculum and the
//! offline I/O protocol as typed values and named transitions. It stages no
//! bytes, opens no socket, mounts no file, signs nothing, evaluates no policy
//! at runtime and never presents a generated artifact as a measurement or a
//! release decision. A worker sees exactly two JSON schemas and one receipt
//! whose only meaning is `submission_received`.
//!
//! Grounding read in full before writing:
//! - `docs/17-v61-constitution.md` - ten laws, authority separation.
//! - `docs/18-emergent-evolution.md` - no genetic optimizer, blind siblings.
//! - `docs/19-artifact-sovereignty.md` - seven-stage transfer, holder table.
//! - `docs/20-embargo-licensing-and-trade.md` - embargo, license, trade graph.
//! - `docs/21-two-timescale-propagation.md` - fast/slow loops, bitemporality.
//! - `docs/22-hidden-worker-and-correspondence.md` - offline ABI, quiet
//!   enforcement, correspondence fields.
//! - `docs/25-failure-analysis.md` - thirty typed failure cases.
//! - `docs/26-acceptance-and-build-plan.md` - completion honesty.
//! - `docs/27-research-curriculum.md` - software-assigned challenges.
//! - `docs/28-offline-io-protocol.md` - the two and only two visible schemas.
//! - `policy/v61-roles.json`, `policy/v61-state-machines.yaml`.
//! - `examples/v61-*.example.json`, `examples/page-v61-*.example.json`.
//!
//! Cross-crate couplings are typed path names, not imports of sibling crates
//! this card must not edit. On this case-sensitive volume the domain
//! directories are PascalCase, so every module is declared through an explicit
//! `#[path]` attribute and a lowercase Rust identifier.

#![forbid(unsafe_code)]

#[path = "Warning.rs"]
pub mod warning;
#[path = "Permission.rs"]
pub mod permission;
#[path = "Digest.rs"]
pub mod digest;
#[path = "Switch.rs"]
pub mod switch;
#[path = "Network.rs"]
pub mod network;
#[path = "Right.rs"]
pub mod right;
#[path = "Decision.rs"]
pub mod decision;
#[path = "Operator.rs"]
pub mod operator;
#[path = "Direction.rs"]
pub mod direction;
#[path = "Verdict.rs"]
pub mod verdict;
#[path = "Claim.rs"]
pub mod claim;
#[path = "Role.rs"]
pub mod role;

#[path = "Custody/mod.rs"]
pub mod custody;
#[path = "Embargo/mod.rs"]
pub mod embargo;
#[path = "License/mod.rs"]
pub mod license;
#[path = "Trade/mod.rs"]
pub mod trade;
#[path = "Bitemporal/mod.rs"]
pub mod bitemporal;
#[path = "Worker/mod.rs"]
pub mod worker;
#[path = "Io/mod.rs"]
pub mod io;
#[path = "Failure/mod.rs"]
pub mod failure;
#[path = "Curriculum/mod.rs"]
pub mod curriculum;

/// Length of a lowercase-hex SHA-256 identity (`schemas/v61-*: pattern`).
pub const DIGEST_LENGTH: usize = 64;

/// The single model-visible input path (`docs/28:8`, offline task schema).
pub const TASK_PATH: &str = "/workspace/task.json";
/// Read-only pinned source view (`docs/28:9`).
pub const REPOSITORY_PATH: &str = "/workspace/repo";
/// Optional approved local references (`docs/28:10`).
pub const REFERENCES_PATH: &str = "/workspace/references";
/// Optional prior own feedback (`docs/28:10`).
pub const FEEDBACK_PATH: &str = "/workspace/feedback.txt";
/// The only writable worker directory (`docs/28:15`).
pub const OUTPUT_DIRECTORY: &str = "/workspace/out";
/// The single model-visible output manifest (`docs/28:15`).
pub const SUBMISSION_PATH: &str = "/workspace/out/submission.json";

/// The only status an opaque submission receipt may carry (`docs/22:56-59`).
pub const NEUTRAL_STATUS: &str = "submission_received";
/// The only message an opaque receipt may carry (`examples/v61-worker-neutral-receipt`).
pub const NEUTRAL_MESSAGE: &str = "Submission received.";

/// `$id` of the inbound worker-visible schema (`schemas/v61-offline-task.schema.json:3`).
pub const TASK_SCHEMA_ID: &str = "https://schemas.lab.invalid/v6.1/v61-offline-task.schema.json";
/// `$id` of the outbound worker-visible schema
/// (`schemas/v61-offline-submission.schema.json:3`).
pub const SUBMISSION_SCHEMA_ID: &str =
    "https://schemas.lab.invalid/v6.1/v61-offline-submission.schema.json";

/// The boundary invariant: embargo causes, peer identities, trade edges and
/// policy decisions never enter a worker-visible value (`docs/17:9`,
/// `docs/20:57-70`, `docs/28:29-38`).
pub const NON_DISCLOSURE: &str =
    "non-disclosure is a typed invariant; no worker value carries a reason code";

/// The v6.1 store that records custody receipts without creating them.
pub const STORAGE_MIGRATION_PATH: &str = "storage::migration::chain::Chain";
/// The event catalog a trusted audit denial names.
pub const CONTRACT_EVENT_PATH: &str = "contract::event::kind::Kind";
/// The constitution no generated artifact may amend.
pub const LABORATORY_CONSTITUTION_PATH: &str = "laboratory::constitution::invariant::Invariant";
/// The release decision no worker claim can fabricate.
pub const EXPERIMENT_RELEASE_PATH: &str = "experiment::queue::release::Release";
/// The signature model trusted audit uses; this crate verifies nothing.
pub const TRUST_SIGNATURE_PATH: &str = "trust::bridge::signature::Signature";
/// The signed claim a sanitized knowledge release may reference.
pub const MARKET_CLAIM_PATH: &str = "market::claim::claim::Claim";
