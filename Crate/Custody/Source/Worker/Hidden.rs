//! The hidden boundary of an offline model worker.
//!
//! Only task-local opaque references are visible; the trusted supervisor owns
//! the mapping to internal task IDs. There are no routes, credentials, policy
//! views or peer identities (`docs/17:12-18`, `docs/22:1-11`,
//! `policy/v61-roles.json:4-12`).

use crate::network::Network;
use crate::permission::Permission;

/// The worker-visible shape of one offline assignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hidden {
    /// Task-local nonce, opaque outside the supervisor.
    pub local_nonce: String,
    /// Pinned source snapshot digest mounted read-only.
    pub source_snapshot_digest: String,
    /// Digest of the bounded task description.
    pub task_digest: String,
    /// Effective materialized view identifier.
    pub view_id: String,
    /// Network posture, required to be `none`.
    pub network_mode: Network,
    /// Number of service routes mounted; must stay zero.
    pub route_count: usize,
    /// Number of service credentials mounted; must stay zero.
    pub credential_count: usize,
}

impl Hidden {
    /// Construct an offline worker description with no services attached.
    #[must_use]
    pub fn of(nonce: &str, source: &str, task: &str, view: &str) -> Self {
        Self {
            local_nonce: nonce.to_owned(),
            source_snapshot_digest: source.to_owned(),
            task_digest: task.to_owned(),
            view_id: view.to_owned(),
            network_mode: Network::None,
            route_count: 0,
            credential_count: 0,
        }
    }

    /// Whether the boundary contains no privileged surface.
    #[must_use]
    pub fn sealed(&self) -> Permission {
        self.network_mode
            .offline()
            .and(Permission::filled(&self.local_nonce))
            .and(Permission::digest(&self.source_snapshot_digest))
            .and(Permission::filled(&self.view_id))
            .and(Permission::of(self.route_count == 0 && self.credential_count == 0))
    }

    /// A worker may never name its internal task ID.
    #[must_use]
    pub fn opaque(&self) -> bool {
        self.local_nonce
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    }
}
