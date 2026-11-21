//! An offline, untrusted LLM worker.
//! Grounded in `22-hidden-worker-and-correspondence.md:1-28` (plain pinned
//! snapshot, no service hosts, credentials, schema or privileged environment;
//! the worker sees only its assigned scope) and `17-v61-constitution.md:5`
//! (access-control-enforced independence, no lateral communication).

use super::scope::Scope;

/// A worker identity that is opaque outside the local task.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Worker {
    /// Opaque, task-local identifier; never an internal worker id.
    pub local_id: String,
    /// Model lineage fingerprint used only for correlation accounting.
    pub model_lineage: String,
    /// The single scope this worker is permitted to mutate.
    pub scope: Scope,
    /// A service endpoint; absent means the worker is offline.
    pub endpoint: Option<String>,
}

impl Worker {
    /// Create an offline worker bound to one scope.
    #[must_use]
    pub fn of(local_id: &str, model_lineage: &str, scope: Scope) -> Self {
        Self {
            local_id: local_id.to_owned(),
            model_lineage: model_lineage.to_owned(),
            scope,
            endpoint: None,
        }
    }

    /// Whether the worker is network-isolated.
    #[must_use]
    pub const fn is_offline(&self) -> bool {
        self.endpoint.is_none()
    }

    /// Whether this worker may mutate a given scope.
    #[must_use]
    pub const fn may_mutate(&self, scope: Scope) -> bool {
        self.scope.eq(&scope)
    }
}
