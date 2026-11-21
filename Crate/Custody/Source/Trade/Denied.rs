//! A trusted denial record with a worker-blind projection.
//!
//! The record keeps the cause, actor and policy decision hash for an operator;
//! the worker receives only a fixed status. Payload shape stays constant so
//! which rule fired cannot be inferred from wording
//! (`docs/20:57-70`, `docs/22:60-69`, `docs/25:6`).

use super::reason::Reason;
use crate::permission::Permission;

/// One denied lateral exchange retained in restricted audit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Denied {
    /// `audit_id` of the denial event.
    pub audit_id: String,
    /// Request that was refused.
    pub request_id: String,
    /// Trusted reason code, never worker-visible.
    pub reason: Reason,
    /// Actor identity retained only by the operator.
    pub actor_id: String,
    /// Policy decision hash retained in the audit ledger.
    pub decision_digest: String,
    /// Operator visibility class, e.g. `restricted`.
    pub operator_visibility: String,
    /// Whether the subject worker was notified of anything.
    pub worker_notified: Permission,
}

impl Denied {
    /// Record a denial without notifying the worker of its cause.
    #[must_use]
    pub fn of(audit: &str, request: &str, reason: Reason, decision: &str) -> Self {
        Self {
            audit_id: audit.to_owned(),
            request_id: request.to_owned(),
            reason,
            actor_id: String::new(),
            decision_digest: decision.to_owned(),
            operator_visibility: "restricted".to_owned(),
            worker_notified: Permission::Deny,
        }
    }

    /// The fixed generic status emitted to the worker for every denial class.
    #[must_use]
    pub fn worker_view(&self) -> &'static str {
        crate::NEUTRAL_STATUS
    }

    /// The worker view never contains a reason code or peer identity.
    #[must_use]
    pub fn discloses(&self) -> Permission {
        match self.reason.worker_visible() {
            true => Permission::Deny,
            false => match self.worker_notified {
                Permission::Allow => Permission::Deny,
                Permission::Deny => Permission::Allow,
            },
        }
    }
}
