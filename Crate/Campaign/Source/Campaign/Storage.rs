//! Evidence storage and schema-migration policy.
//!
//! Grounded in `campaign.v2.json:232-241`, the schema at
//! `schemas/campaign.v2.schema.json:680-721`, `REQUIREMENTS.md:718-734`
//! (SQLite/WAL, content-addressed artifacts) and `:736-750` (REQ-LIN-01..10).
//! Paths are typed strings; no database file is opened and no object is
//! written.
//!
//! Exception declared: `Integrity` and `MigrationMode` are declared beside
//! `Storage` because each is the typed form of one field and has no
//! independent lifecycle.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// The only integrity mechanism in the design (`campaign.v2.json:239`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Integrity {
    Sha256,
}

/// Schema evolution must be explicit, never implicit (LAW-017).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationMode {
    Explicit,
}

/// Local evidence-store descriptor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Storage {
    pub database: String,
    #[serde(rename = "artifact_root")]
    pub artifact_root: String,
    pub journal: String,
    #[serde(rename = "append_only_evidence", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub append_only: Permission,
    #[serde(rename = "store_agent_prompts", default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub store_prompts: Permission,
    #[serde(rename = "prompt_redaction", default)]
    pub prompt_redaction: String,
    #[serde(rename = "artifact_integrity")]
    pub integrity: Integrity,
    #[serde(rename = "schema_migration_mode", default)]
    pub migration_mode: MigrationMode,
}

impl Storage {
    /// Recognised keys (`schemas/campaign.v2.schema.json:714-720`).
    pub const KEYS: [&'static str; 8] = [
        "database",
        "artifact_root",
        "journal",
        "append_only_evidence",
        "store_agent_prompts",
        "prompt_redaction",
        "artifact_integrity",
        "schema_migration_mode",
    ];

    /// The canonical artifact address
    /// `objects/sha256/<first-two>/<digest>` (`REQUIREMENTS.md:728-729`).
    #[must_use]
    pub fn object_path(&self, digest: &str) -> String {
        let prefix = digest.get(0..2).unwrap_or("??");
        format!("{}/{prefix}/{digest}", self.artifact_root)
    }

    /// Whether raw evidence is immutable (LAW-006, REQ-LIN-10).
    #[must_use]
    pub fn evidence_is_append_only(&self) -> Permission {
        self.append_only
    }

    /// Whether an unknown schema version is rejected rather than guessed
    /// (REQ-ID-07, LAW-017).
    #[must_use]
    pub fn migration_is_explicit(&self) -> bool {
        matches!(self.migration_mode, MigrationMode::Explicit)
    }
}
