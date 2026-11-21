//! Typed mirror of `interfaces/event-envelope.v3.schema.json` and
//! `examples/event.v3.json`.
//!
//! Every required field in the schema is a typed field here; optional fields
//! are `Option`. The envelope is the append-only unit of authoritative truth
//! (`TELEMETRY.md:3-6`, `ARCHITECTURE.md:106-109`). No LLM-authored value can
//! change a committed sequence. Exception declared: `Actor` is the typed form
//! of this one nested object and has no independent lifecycle.

use serde::{Deserialize, Serialize};

/// The actor that emitted an event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    pub id: String,
    pub role: String,
}

/// One append-only event envelope (schema version 3).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub schema_version: u32,
    pub event_id: String,
    pub event_kind: String,
    pub campaign_id: Option<String>,
    pub aggregate_id: String,
    pub aggregate_seq: u64,
    pub occurred_utc: String,
    pub actor: Actor,
    pub idempotency_key: String,
    pub payload_sha256: String,
    pub correlation_id: String,
    pub causation_event_id: Option<String>,
    pub trace_context: Option<String>,
    pub source_artifacts: Vec<String>,
    pub view_policy_id: Option<String>,
}

impl Envelope {
    /// The schema version every envelope must carry (`schema:20-22`).
    pub const SCHEMA_VERSION: u32 = 3;

    /// Whether this envelope follows another in logical sequence.
    #[must_use]
    pub fn follows(&self, other: &Self) -> bool {
        self.aggregate_id == other.aggregate_id && self.aggregate_seq > other.aggregate_seq
    }

    /// Whether the envelope declares a causal predecessor.
    #[must_use]
    pub fn has_cause(&self) -> bool {
        self.causation_event_id.is_some()
    }
}
