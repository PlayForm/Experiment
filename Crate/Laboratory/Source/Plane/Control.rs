//! Plane 1: the trusted Rust control and observability plane.
//!
//! Grounded in `docs/04-internal-api.md:1-24,26-55` and `README.md:11`: 53
//! internal authenticated service operations, mTLS identity mapped to an
//! allowlisted service role before payload parsing, stable idempotency keys,
//! compare-and-swap event appends, fencing tokens on leases, separately
//! authorized signers for claim/build/oracle/benchmark/evaluation, and no
//! generic `execute` endpoint. These routes are never mounted or advertised to
//! a worker. This file models the rules; it serves nothing.

/// The Plane 1 descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Control {
    /// Number of documented internal operations.
    pub operations: u8,
    /// Whether a mutating call must carry an idempotency key.
    pub idempotency_required: crate::Permission,
    /// Whether a report must bind a fencing token.
    pub fencing_required: crate::Permission,
}

impl Control {
    /// Build the canonical Plane 1 descriptor.
    #[must_use]
    pub const fn of() -> Self {
        Self {
            operations: crate::INTERNAL_OPERATIONS,
            idempotency_required: crate::Permission::Allow,
            fencing_required: crate::Permission::Allow,
        }
    }

    /// Replay with the same key returns the original receipt, not a new effect.
    #[must_use]
    pub const fn replay_is_idempotent(&self) -> crate::Permission {
        self.idempotency_required
    }

    /// A stale fencing token can never commit authoritative output.
    #[must_use]
    pub const fn stale_token_commits(&self) -> crate::Permission {
        crate::Permission::Deny
    }

    /// The signed-assertion signers are distinct by authority
    /// (`docs/09-cryptographic-trust.md:31-49`).
    #[must_use]
    pub const fn separate_signers() -> [&'static str; 6] {
        [
            "builder",
            "oracle",
            "runner",
            "evaluator",
            "publication-authority",
            "pki",
        ]
    }
}
