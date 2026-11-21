//! Workload identity, command argv and holdout split.
//!
//! Grounded in `campaign.v2.json:170-198`, the schema at
//! `schemas/campaign.v2.schema.json:522-576` and `REQUIREMENTS.md:381-424`
//! (REQ-WLD-04, REQ-WLD-09, REQ-BEN-11). Commands are structured argv data -
//! the planner never rewrites or shells them (`REQ-SEC-03`, LAW-001).
//!
//! Exception declared: `Split` is declared beside `Workload` because it is
//! the typed form of this one field and has no independent lifecycle.

use serde::{Deserialize, Serialize};

use crate::agent::Permission;

/// Immutable split assignment (`schemas/campaign.v2.schema.json:550-558`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Split {
    Training,
    Development,
    Holdout,
    /// Final sealed validation set; invisible to the mutator (REQ-WLD-04).
    #[serde(rename = "sealed_holdout")]
    Sealed,
}

impl Split {
    /// The JSON enum token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Training => "training",
            Self::Development => "development",
            Self::Holdout => "holdout",
            Self::Sealed => "sealed_holdout",
        }
    }

    /// Whether policy may train on this split
    /// (`REQUIREMENTS.md:502-503`, REQ-BEN-11).
    #[must_use]
    pub const fn trainable(self) -> bool {
        matches!(self, Self::Training | Self::Development)
    }
}

/// One measured workload.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Workload {
    pub id: String,
    pub fixture: String,
    pub command: Vec<String>,
    pub pattern: String,
    #[serde(default = "Permission::default", deserialize_with = "Permission::from_bool")]
    pub parallel: Permission,
    pub split: Split,
    pub oracle: String,
}

impl Workload {
    /// Recognised keys (`schemas/campaign.v2.schema.json:564-572`).
    pub const KEYS: [&'static str; 7] = [
        "id",
        "fixture",
        "command",
        "pattern",
        "parallel",
        "split",
        "oracle",
    ];

    /// Whether this workload is hidden from a coding agent
    /// (`campaign.v2.json:210`, `campaign.v2.json:195`).
    #[must_use]
    pub fn visible_to_mutator(&self) -> Permission {
        match self.split {
            Split::Sealed => Permission::Deny,
            Split::Training | Split::Development | Split::Holdout => Permission::Allow,
        }
    }

    /// Structured argv, copied rather than interpreted.
    #[must_use]
    pub fn argv(&self) -> &[String] {
        &self.command
    }
}
