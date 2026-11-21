//! Typed mirror of `interfaces/agent-turn.v3.schema.json` and
//! `examples/agent-turn.v3.json`.
//!
//! A turn is a provenance record, not benchmark truth (`STORAGE.md:4`,
//! `AGENT_CONTRACTS.md:59-85`). It carries hashes for content, context snapshot
//! and role policy, plus privacy classification and token timings. Exception
//! declared: `Kind` is the typed form of this document's kind field and has no
//! independent lifecycle outside `Turn`.

use serde::{Deserialize, Serialize};

/// The eight turn kinds (`:40-51`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool_call")]
    ToolCall,
    #[serde(rename = "tool_result")]
    ToolResult,
    #[serde(rename = "feedback")]
    Feedback,
    #[serde(rename = "compaction")]
    Compaction,
}

/// One immutable agent-turn provenance record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Turn {
    pub schema_version: u32,
    pub conversation_id: String,
    pub session_id: String,
    pub turn_id: String,
    pub seq: u64,
    pub kind: Kind,
    pub role_policy_sha256: String,
    pub context_snapshot_sha256: String,
    pub content_sha256: String,
    pub redaction_policy_id: String,
    pub created_utc: String,
    pub model_id: Option<String>,
    pub prompt_template_sha256: Option<String>,
    pub prior_turns: Vec<String>,
    pub tools_used: Vec<String>,
    pub ttft_ms: Option<f64>,
    pub ttfb_ms: Option<f64>,
    pub tokens_in: Option<u64>,
    pub tokens_out: Option<u64>,
    pub finish_reason: Option<String>,
}

impl Turn {
    pub const SCHEMA_VERSION: u32 = 3;

    /// Whether this turn is a compaction referencing source turns
    /// (`AGENT_CONTRACTS.md:74`).
    #[must_use]
    pub const fn is_compaction(&self) -> bool {
        matches!(self.kind, Kind::Compaction)
    }

    /// Whether the turn records a model request or feedback only.
    #[must_use]
    pub const fn is_model_output(&self) -> bool {
        matches!(self.kind, Kind::Assistant | Kind::ToolCall)
    }
}
