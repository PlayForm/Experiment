//! Typed causes underlying the thirty failure cases.
//!
//! A cause belongs to a restricted audit record; the worker receives a fixed
//! non-identifying status regardless of which cause fired
//! (`docs/25:3-34`, `docs/22:47-69`).

/// The mechanism a failure case exercises.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cause {
    /// Environment, path or credential leakage.
    Leak,
    /// Trusted oracle or fixture integrity.
    Oracle,
    /// Sandbox or untrusted build escape.
    Sandbox,
    /// License, EULA, embargo or rights violation.
    Rights,
    /// Disclosure of a restricted reason or identity.
    Disclosure,
    /// Evidence, measurement or attestation failure.
    Evidence,
    /// Failure of sibling or role isolation.
    Isolation,
    /// Policy epoch or ordering conflict.
    Epoch,
    /// Capacity, reservation or storage pressure.
    Capacity,
    /// Path or mount escape.
    Escape,
    /// Ledger sequence confused with wall-clock order.
    Causality,
    /// Convergence on one unhelpful family.
    Convergence,
    /// Covert timing or naming channel.
    Channel,
    /// Resource spending mistaken for quality.
    Cost,
    /// Unbounded recursive proposals.
    Recursion,
    /// Duplicate transfer after restart.
    Idempotency,
    /// API exposed as a covert worker tool.
    Endpoint,
    /// A policy or audit service could not be reached.
    Unreachable,
}

impl Cause {
    /// Whether the cause is ever safe to name in worker feedback.
    #[must_use]
    pub fn worker_visible(&self) -> bool {
        match self {
            Self::Leak
            | Self::Oracle
            | Self::Sandbox
            | Self::Rights
            | Self::Disclosure
            | Self::Evidence
            | Self::Isolation
            | Self::Epoch
            | Self::Capacity
            | Self::Escape
            | Self::Causality
            | Self::Convergence
            | Self::Channel
            | Self::Cost
            | Self::Recursion
            | Self::Idempotency
            | Self::Endpoint
            | Self::Unreachable => false,
        }
    }
}
