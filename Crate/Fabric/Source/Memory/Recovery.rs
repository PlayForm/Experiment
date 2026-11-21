//! The ordered crash-recovery procedure and its fault cases.
//!
//! `docs/06:39-44` requires: verify signatures, sequence and previous-hash
//! chain; reject rollback or fork absent recovery policy; restore a sealed
//! snapshot; replay remaining committed records; recompile the warm registry.
//! `docs/09:44-45` requires an exact lineage head and old key epochs after
//! restore. No fault is injected and no failure becomes a pass.

/// One named recovery step or rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Recovery {
    /// Verify signatures, sequence and previous-hash chain.
    VerifyChain,
    /// Reject an unauthorized rollback.
    RejectRollback,
    /// Reject a fork of the chain head.
    RejectFork,
    /// Restore the sealed snapshot.
    RestoreSnapshot,
    /// Replay the remaining committed records.
    ReplayCommitted,
    /// Recompile the warm in-memory registry.
    RecompileWarm,
}

impl Recovery {
    /// The canonical order of recovery steps.
    #[must_use]
    pub const fn order() -> [Self; 6] {
        [
            Self::VerifyChain,
            Self::RejectRollback,
            Self::RejectFork,
            Self::RestoreSnapshot,
            Self::ReplayCommitted,
            Self::RecompileWarm,
        ]
    }

    /// Whether this step is a rejection rather than a restore action.
    #[must_use]
    pub const fn rejects(&self) -> bool {
        matches!(self, Self::RejectRollback | Self::RejectFork)
    }

    /// Stable audit label.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::VerifyChain => "verify-chain",
            Self::RejectRollback => "reject-rollback",
            Self::RejectFork => "reject-fork",
            Self::RestoreSnapshot => "restore-snapshot",
            Self::ReplayCommitted => "replay-committed",
            Self::RecompileWarm => "recompile-warm",
        }
    }
}
