//! Typed mirror of `interfaces/feedback-capsule.v3.schema.json` and
//! `examples/feedback-capsule.v3.json`.
//!
//! The capsule is the only result view a patcher may receive: own development
//! data, evidence references and minimal categories, never sealed holdout or
//! peer data (`ARCHITECTURE.md:113-117`, `RESEARCH_GATE.md:78-95`). Exceptions
//! declared: `Status`, `Visibility`, `NextAction` and `Holdout` are typed forms
//! of this document's fields and have no independent lifecycle.

use serde::{Deserialize, Serialize};

/// The evaluation status carried by a capsule (`:36-46`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "oracle_failed")]
    OracleFailed,
    #[serde(rename = "benchmark_win")]
    BenchmarkWin,
    #[serde(rename = "benchmark_loss")]
    BenchmarkLoss,
    #[serde(rename = "inconclusive")]
    Inconclusive,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "cache_hit")]
    CacheHit,
    #[serde(rename = "not_yet_measured")]
    NotYetMeasured,
}

/// The only permitted dataset visibility (`:47-49`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "own_development_only")]
    OwnDevelopmentOnly,
}

/// The sanctioned next action for the receiving patcher (`:94-102`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NextAction {
    #[serde(rename = "revise_as_new_mutation")]
    ReviseAsNewMutation,
    #[serde(rename = "stop_lane")]
    StopLane,
    #[serde(rename = "confirm_later")]
    ConfirmLater,
    #[serde(rename = "await_review")]
    AwaitReview,
    #[serde(rename = "repair_correctness")]
    RepairCorrectness,
}

/// Typed form of the schema's `sealed_holdout_included: false` constant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Holdout {
    /// Holdout data is always excluded from a patcher capsule.
    Excluded,
}

/// A scoped, redacted feedback capsule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Capsule {
    pub schema_version: u32,
    pub capsule_id: String,
    pub recipient_work_order_id: String,
    pub proposal_id: String,
    pub evaluation_status: Status,
    pub dataset_visibility: Visibility,
    pub baseline_binary_sha256: String,
    pub candidate_binary_sha256: String,
    pub evidence_sha256: String,
    pub policy_sha256: String,
    pub issued_utc: String,
    pub speedup_ratio: Option<f64>,
    pub ci_low: Option<f64>,
    pub ci_high: Option<f64>,
    pub failure_categories: Vec<String>,
    pub aggregate_hints: Vec<String>,
    pub next_action: NextAction,
    pub related_analysis_id: Option<String>,
}

impl Capsule {
    pub const SCHEMA_VERSION: u32 = 3;

    /// Holdout data is excluded by construction.
    #[must_use]
    pub const fn sealed_holdout_included(&self) -> Holdout {
        Holdout::Excluded
    }

    /// Whether the capsule is addressed to exactly one work order.
    #[must_use]
    pub fn is_scoped(&self) -> bool {
        !self.recipient_work_order_id.is_empty()
    }
}
