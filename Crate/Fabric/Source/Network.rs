//! Network class of a provisioned holder.
//!
//! Mirrors `network_class` in `schemas/holder-provision.schema.json` and the
//! least-privilege worker example checked by `tests/validate.py:34-55`. The
//! offline worker class carries no secret and no egress (`README.md:20-21`,
//! `docs/08:15`).

/// One declared network posture for a trusted or untrusted holder.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Network {
    /// No network and no egress; the LLM worker class.
    Offline,
    /// Trusted IPC only, no operator reachability.
    Internal,
    /// Operator mTLS projection and permitted commands.
    Operator,
}

impl Network {
    /// Wire spelling from the holder-provision schema.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Internal => "internal",
            Self::Operator => "operator",
        }
    }

    /// The only class an LLM worker may be provisioned with.
    #[must_use]
    pub const fn worker_class() -> Self {
        Self::Offline
    }

    /// Whether an offered class matches the provisioned class structurally.
    #[must_use]
    pub fn agrees(&self, offered: &str) -> crate::verdict::Verdict {
        crate::verdict::Verdict::of(offered == self.label())
    }

    /// Whether this class permits external egress. None does for a worker;
    /// operator reachability is a projection, not general egress.
    #[must_use]
    pub const fn egress(&self) -> bool {
        matches!(self, Self::Operator)
    }
}
