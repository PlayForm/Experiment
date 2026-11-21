//! Typed crash scenarios and their required recovery outcomes.
//!
//! Grounded in `docs/14-runtime-and-recovery.md:24-40` (the recovery table),
//! `docs/06-state-machines.md:92-98` (do not assume an uncommitted external action
//! never occurred; irreconcilable outcomes are inconclusive or quarantined) and
//! `docs/25-failure-analysis.md` F29 (a crash before commit yields exactly one
//! valid receipt). Each scenario maps to a required result, never to an
//! automatic success. Nothing here is executed; it is a typed incident catalog.

/// A crash or outage scenario the laboratory must survive.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Crash {
    /// Coordinator died after commit but before reply.
    CoordinatorAfterCommit,
    /// Worker died after writing a partial patch.
    WorkerPartialPatch,
    /// Supervisor died while the model was running.
    SupervisorDied,
    /// Builder died after producing a binary.
    BuilderAfterBinary,
    /// Oracle process crashed.
    OracleCrashed,
    /// Bench host overheated or ran another process.
    HostContention,
    /// Product OTLP stopped.
    ProductOtelStopped,
    /// Control OTLP stopped.
    ControlOtelStopped,
    /// An event subscriber missed messages.
    SubscriberMissed,
    /// The CA issuer was unavailable.
    IssuerUnavailable,
    /// The local model was unavailable.
    ModelUnavailable,
    /// A private key was suspected compromised.
    KeyCompromised,
    /// Host root was compromised.
    HostRootCompromised,
}

impl Crash {
    /// The required recovery outcome for this scenario.
    #[must_use]
    pub const fn outcome(self) -> &'static str {
        match self {
            Self::CoordinatorAfterCommit => {
                "same idempotency key returns the original event and receipt"
            }
            Self::WorkerPartialPatch => "output marked partial or quarantined, never accepted",
            Self::SupervisorDied => "lease expires and is fenced; old token cannot reuse authority",
            Self::BuilderAfterBinary => {
                "CAS commitment or identical deterministic rebuild; ambiguity recorded honestly"
            }
            Self::OracleCrashed => "infrastructure error, not correctness failure or pass",
            Self::HostContention => "samples contaminated, preserved and excluded by frozen policy",
            Self::ProductOtelStopped => "diagnostics missing; trace-off benchmark and ledger continue",
            Self::ControlOtelStopped => "visibility degraded; ledger and state machine continue",
            Self::SubscriberMissed => "replay outbox from ledger sequence, applied idempotently",
            Self::IssuerUnavailable => "no new credentials; historic signed evidence stays valid",
            Self::ModelUnavailable => "agent tasks pause; trusted verification and API continue",
            Self::KeyCompromised => "revoke, rotate, record timeline, invalidate affected new evidence",
            Self::HostRootCompromised => {
                "container isolation insufficient; rotate secrets and rebuild from external anchor"
            }
        }
    }

    /// Whether recovery may fabricate a successful measurement.
    #[must_use]
    pub const fn fabricates_success(self) -> crate::Permission {
        crate::Permission::Deny
    }
}
