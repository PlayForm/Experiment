//! The trust boundaries between service roles and the worker.
//!
//! `docs/08:3-20` assigns separate authority to each service and states these
//! are distinct boundaries, not a requirement of one container per row.
//! `docs/02:22-27` says plane -1 is a boundary, not a superuser image. The
//! worker's only input is a pinned source view, task and scratch directory.

use crate::network::Network;
use crate::verdict::Verdict;

/// One declared communication boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Boundary {
    /// Trusted IPC between broker services.
    TrustedIpc,
    /// Operator mTLS projection gateway.
    OperatorMtls,
    /// Task files and sealed output mounted into the offline worker.
    WorkerFileMount,
    /// Separately restricted builder, oracle and benchmark lane.
    SeparatelyRestricted,
    /// No permitted channel.
    Denied,
}

impl Boundary {
    /// Whether keys, policy registry or internal APIs cross this boundary.
    /// None of them ever do.
    #[must_use]
    pub const fn exposes_trusted_material(&self) -> bool {
        false
    }

    /// Whether a worker can reach this boundary. Only its scoped file mount
    /// is reachable, and it carries no privileged material.
    #[must_use]
    pub const fn worker_reachable(&self) -> bool {
        matches!(self, Self::WorkerFileMount)
    }

    /// The network posture implied by this boundary.
    #[must_use]
    pub const fn network(&self) -> Network {
        match self {
            Self::TrustedIpc => Network::Internal,
            Self::OperatorMtls => Network::Operator,
            Self::WorkerFileMount | Self::SeparatelyRestricted | Self::Denied => Network::Offline,
        }
    }

    /// A boundary is not an independent hardware security domain merely
    /// because it is a separate process (`docs/08:18-20`).
    #[must_use]
    pub const fn is_hardware_domain(&self) -> Verdict {
        Verdict::Deny
    }
}
