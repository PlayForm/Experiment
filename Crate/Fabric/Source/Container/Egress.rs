//! Egress posture for each holder and boundary.
//!
//! `README.md:20-21,34-36` requires zero external egress from the worker and
//! no lab access from LLM workers. `docs/09:10` requires a live no-network
//! test. `docs/10:40-43` requires rootless isolation and no host Docker socket.

/// One declared egress posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Egress {
    /// No outbound channel at all; the worker class.
    None,
    /// Trusted IPC to specific broker endpoints.
    TrustedIpc,
    /// Operator mTLS projection only.
    OperatorMtls,
    /// All outbound access refused.
    Denied,
}

impl Egress {
    /// A worker is never granted any egress posture beyond none.
    #[must_use]
    pub const fn worker_allowed(&self) -> bool {
        false
    }

    /// Whether this posture permits a general network route.
    #[must_use]
    pub const fn general_network(&self) -> bool {
        matches!(self, Self::OperatorMtls)
    }

    /// Stable audit label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::TrustedIpc => "trusted-ipc",
            Self::OperatorMtls => "operator-mtls",
            Self::Denied => "denied",
        }
    }
}
