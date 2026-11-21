//! One conceptual source mutation and its durable state machine.
//!
//! Grounded in `REQUIREMENTS.md:321-334` (REQ-MUT-01..08), `:604-612` (the
//! 15-state lane plus seven side exits), `AGENT_CONTRACT.md:25-45` (strict
//! mutator instructions) and `sql/lineage.sql:35-58` (mutations and
//! mutation_parents). A mutation here is a typed record; no patch is applied.
//!
//! Exception declared: the `State` enum is declared beside `Mutation` because
//! it is the state of this one type and has no independent lifecycle.

use serde::{Deserialize, Serialize};

use crate::{Fault, Warning};

/// The state lane and its side exits (`REQUIREMENTS.md:606-612`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum State {
    /// Main lane, in declared order.
    Proposed,
    Scoped,
    Worktree,
    Patched,
    Checked,
    Tested,
    Built,
    Oracle,
    Queued,
    Benchmarked,
    Analyzed,
    Confirmed,
    Eligible,
    Reviewed,
    Integrated,
    /// Side exits.
    Rejected,
    Invalid,
    Inconclusive,
    Retry,
    Paused,
    Quarantined,
    Aborted,
}

impl State {
    /// Index in the main lane; side exits have no forward index.
    #[must_use]
    pub const fn lane(self) -> Option<u8> {
        match self {
            Self::Proposed => Some(0),
            Self::Scoped => Some(1),
            Self::Worktree => Some(2),
            Self::Patched => Some(3),
            Self::Checked => Some(4),
            Self::Tested => Some(5),
            Self::Built => Some(6),
            Self::Oracle => Some(7),
            Self::Queued => Some(8),
            Self::Benchmarked => Some(9),
            Self::Analyzed => Some(10),
            Self::Confirmed => Some(11),
            Self::Eligible => Some(12),
            Self::Reviewed => Some(13),
            Self::Integrated => Some(14),
            Self::Rejected
            | Self::Invalid
            | Self::Inconclusive
            | Self::Retry
            | Self::Paused
            | Self::Quarantined
            | Self::Aborted => None,
        }
    }

    /// Whether this is one of the seven documented side exits.
    #[must_use]
    pub const fn is_exit(self) -> bool {
        self.lane().is_none()
    }
}

/// A single-change mutation candidate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Mutation {
    pub id: String,
    #[serde(rename = "campaign_id")]
    pub campaign_id: String,
    #[serde(rename = "hypothesis_id")]
    pub hypothesis_id: String,
    #[serde(rename = "parent_mutation_id")]
    pub parent_mutation_id: Option<String>,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "changed_stages")]
    pub changed_stages: Vec<String>,
    #[serde(rename = "source_commit")]
    pub source_commit: String,
    #[serde(rename = "parent_commit")]
    pub parent_commit: String,
    #[serde(rename = "patch_sha256")]
    pub patch_sha256: Option<String>,
    #[serde(rename = "author_agent_id")]
    pub author_agent_id: Option<String>,
    pub state: State,
}

impl Mutation {
    /// Advance along the lane or take a documented side exit.
    ///
    /// An interrupted `Benchmarked` state is never assumed successful: only an
    /// explicit transition to `Analyzed` may follow it
    /// (`REQUIREMENTS.md:609-612`).
    pub fn transition(&self, next: State) -> Result<Self, crate::Fault> {
        let allowed = match (self.state.lane(), next.lane()) {
            (Some(from), Some(to)) => to == from + 1,
            (Some(_), None) => next.is_exit(),
            (None, Some(0)) => next == State::Proposed,
            (None, None) => false,
        };
        match allowed {
            true => {
                let mut advanced = self.clone();
                advanced.state = next;
                Ok(advanced)
            }
            false => Err(Fault::Conflicting {
                domain: "mutation.state",
                detail: format!("cannot move from {:?} to {next:?}", self.state),
            }),
        }
    }

    /// Whether exactly one conceptual stage was changed
    /// (REQ-MUT-02, REQ-AGT-08).
    #[must_use]
    pub fn is_single_concept(&self) -> bool {
        self.changed_stages.len() == 1
    }

    /// Record a multi-parent merge as a separate experiment, never inheriting
    /// a parent's performance claim (REQ-MUT-04, LAW-007).
    #[must_use]
    pub fn merge_notice(&self, parents: &[String]) -> Vec<Warning> {
        match parents.len() {
            0 | 1 => Vec::new(),
            _ => vec![Warning::Bound {
                domain: "mutation.merge",
                key: "requires_new_merged_baseline".to_owned(),
            }],
        }
    }
}
