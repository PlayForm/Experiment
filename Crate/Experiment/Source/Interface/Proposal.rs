//! Typed mirror of `interfaces/proposal.v3.schema.json` and
//! `examples/proposal.v3.json`.
//!
//! A proposal is an untrusted patcher artifact: it carries content hashes and
//! a mechanism hypothesis but grants no authority over measurement or policy.
//! `interfaces/Proposal.rs` is distinct from `director::Proposal`, which is an
//! advisory research plan from the parent agent. Exceptions declared:
//! `Mechanism`, `Prediction` and `Effect` are typed forms of this document's
//! fields and have no independent lifecycle outside `Proposal`.

use serde::{Deserialize, Serialize};

/// The mechanism taxonomy (`proposal.v3.schema.json:51-65`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Mechanism {
    #[serde(rename = "allocation")]
    Allocation,
    #[serde(rename = "scheduling")]
    Scheduling,
    #[serde(rename = "filesystem")]
    Filesystem,
    #[serde(rename = "matching")]
    Matching,
    #[serde(rename = "process_executor")]
    ProcessExecutor,
    #[serde(rename = "heuristic_policy")]
    HeuristicPolicy,
    #[serde(rename = "serialization")]
    Serialization,
    #[serde(rename = "syscall_overhead")]
    SyscallOverhead,
    #[serde(rename = "compiler")]
    Compiler,
    #[serde(rename = "integration")]
    Integration,
    #[serde(rename = "other")]
    Other,
}

/// A predicted effect on one workload family (`:119-124`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Prediction {
    #[serde(rename = "speedup")]
    Speedup,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "regression")]
    Regression,
    #[serde(rename = "unknown")]
    Unknown,
}

/// One declared workload prediction (`:112-125`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Effect {
    pub workload_family: String,
    pub prediction: Prediction,
}

/// The formal patch-submission document (schema version 3).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Proposal {
    pub schema_version: u32,
    pub proposal_id: String,
    pub work_order_id: String,
    pub generation_id: String,
    pub parent_source_commit: String,
    pub patch_sha256: String,
    pub source_tree_sha256: String,
    pub mechanism_family: Mechanism,
    pub mechanism_hypothesis: String,
    pub changed_paths: Vec<String>,
    pub changed_symbols: Vec<String>,
    pub build_spec_sha256: String,
    pub role_policy_sha256: String,
    pub novelty_fingerprint_version: String,
    pub semantic_contract_sha256: String,
    pub risk_notes: Vec<String>,
    pub expected_workload_effect: Vec<Effect>,
    pub parent_mutation_id: Option<String>,
    pub prompt_context_sha256: String,
    pub agent_session_id: String,
}

impl Proposal {
    pub const SCHEMA_VERSION: u32 = 3;

    /// A proposal alone never authorizes a measurement; a committed patch must
    /// already exist (`REQUIREMENTS_V3.md` AGT3-004).
    #[must_use]
    pub const fn authorizes_measurement() -> bool {
        false
    }

    /// The declared mechanism family token, for the novelty gate.
    #[must_use]
    pub fn family(&self) -> &Mechanism {
        &self.mechanism_family
    }
}
