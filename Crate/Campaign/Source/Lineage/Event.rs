//! The append-only evidence event row.
//!
//! Grounded in `sql/lineage.sql:178-189` (the `events` table) and `:201-213`
//! (`deny_events_update` / `deny_events_delete` triggers). REQ-AGT-07
//! (`REQUIREMENTS.md:594`) requires events to be persisted before a state move.
//! This type values an event; it never writes one or grants it authority.

use crate::lineage::Digest;

use serde::{Deserialize, Serialize};

/// One immutable evidence event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// `sequence INTEGER PRIMARY KEY AUTOINCREMENT` (`lineage.sql:180`).
    pub sequence: u64,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "campaign_id")]
    pub campaign_id: Option<String>,
    #[serde(rename = "entity_kind")]
    pub entity_kind: String,
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    #[serde(rename = "event_kind")]
    pub event_kind: String,
    #[serde(rename = "event_schema_version")]
    pub event_schema_version: u32,
    #[serde(rename = "payload_sha256")]
    pub payload_sha256: Digest,
    #[serde(rename = "created_utc")]
    pub created_utc: String,
}

impl Event {
    /// Column order of the `events` table (`lineage.sql:180-189`).
    pub const KEYS: [&'static str; 9] = [
        "sequence",
        "event_id",
        "campaign_id",
        "entity_kind",
        "entity_id",
        "event_kind",
        "event_schema_version",
        "payload_sha256",
        "created_utc",
    ];

    /// Render one event line for an audit journal.
    #[must_use]
    pub fn line(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.sequence,
            self.event_kind,
            self.entity_kind,
            self.entity_id,
            self.payload_sha256.short()
        )
    }

    /// Events are immutable by schema; no derived summary may overwrite them
    /// (LAW-006, REQ-LIN-10).
    #[must_use]
    pub const fn is_immutable() -> bool {
        true
    }
}
