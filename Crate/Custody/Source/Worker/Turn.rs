//! One archived visible turn in the correspondence vault.
//!
//! Fields follow `docs/22:29-38`: role, model family/build, prompt-template
//! revision, bounded context digest, encrypted input/output references, tool
//! calls, timeouts, view and supervisor receipt. Hidden reasoning is never
//! claimed to be stored (`docs/22:37-39`).

use crate::permission::Permission;
use crate::switch::Switch;

/// One prompt/response exchange sealed by the trusted supervisor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Turn {
    /// `turn_id` within a segment.
    pub turn_id: String,
    /// Task-local role identifier, never an internal worker ID.
    pub role: String,
    /// Model family and build digest.
    pub model_digest: String,
    /// Prompt-template revision actually used.
    pub template_revision: String,
    /// Bounded context snapshot digest.
    pub context_digest: String,
    /// Encrypted input envelope CAS digest.
    pub input_digest: String,
    /// Encrypted output envelope CAS digest.
    pub output_digest: String,
    /// Number of local tool calls in the turn.
    pub tool_calls: usize,
    /// Whether the turn ended in a timeout.
    pub timed_out: Switch,
    /// Effective view identifier at invocation.
    pub view_id: String,
    /// Supervisor-sealed receipt digest.
    pub receipt_digest: String,
}

impl Turn {
    /// Whether the turn carries the required provenance fields.
    #[must_use]
    pub fn complete(&self) -> Permission {
        Permission::filled(&self.turn_id)
            .and(Permission::filled(&self.role))
            .and(Permission::filled(&self.model_digest))
            .and(Permission::filled(&self.template_revision))
            .and(Permission::digest(&self.context_digest))
            .and(Permission::digest(&self.input_digest))
            .and(Permission::digest(&self.output_digest))
            .and(Permission::filled(&self.view_id))
            .and(Permission::digest(&self.receipt_digest))
    }

    /// Whether a hidden reasoning transcript is claimed; never true here.
    #[must_use]
    pub fn claims_hidden_reasoning(&self) -> bool {
        match self.timed_out {
            Switch::On | Switch::Off => false,
        }
    }
}
