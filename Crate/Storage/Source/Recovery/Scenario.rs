//! The documented recovery scenarios and their required honest outcomes.
//!
//! Every row of the recovery table is a typed value; no fault is injected and
//! no failure is silently converted into a passing measurement
//! (`docs/14-runtime-and-recovery.md:24-40`).

use super::outcome::Outcome;

/// One recovery scenario from the runtime playbook.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scenario {
    /// Coordinator committed but died before replying.
    CommitBeforeReply,
    /// Worker died after writing a partial patch.
    PartialPatch,
    /// Supervisor died while the model was running.
    SupervisorDeath,
    /// Builder died after producing a binary.
    BuilderCrash,
    /// Oracle process died.
    OracleCrash,
    /// Benchmark host overheated or gained a competing process.
    HostOverheat,
    /// Product OTLP stopped.
    ProductTelemetryDown,
    /// Control OTLP stopped.
    ControlTelemetryDown,
    /// An event subscriber missed messages.
    SubscriberMiss,
    /// The CA issuer was unavailable.
    IssuerUnavailable,
    /// The local model was unavailable.
    LocalModelUnavailable,
    /// A private key was suspected compromised.
    KeyCompromised,
    /// Host root was compromised.
    HostRootCompromised,
}

impl Scenario {
    /// The required result of this scenario.
    #[must_use]
    pub fn outcome(&self) -> Outcome {
        match self {
            Self::CommitBeforeReply | Self::SubscriberMiss => Outcome::Continue,
            Self::PartialPatch | Self::BuilderCrash | Self::HostOverheat
            | Self::KeyCompromised | Self::HostRootCompromised => Outcome::Quarantine,
            Self::OracleCrash => Outcome::Inconclusive,
            Self::ProductTelemetryDown | Self::ControlTelemetryDown
            | Self::IssuerUnavailable | Self::LocalModelUnavailable => Outcome::Degraded,
            Self::SupervisorDeath => Outcome::Quarantine,
        }
    }

    /// The playbook's required result in prose.
    #[must_use]
    pub fn requirement(&self) -> &'static str {
        match self {
            Self::CommitBeforeReply => {
                "same idempotency request returns the original event or receipt"
            }
            Self::PartialPatch => "output marked partial or quarantined, never silently accepted",
            Self::SupervisorDeath => "lease expires and is fenced; old token cannot be reused",
            Self::BuilderCrash => {
                "CAS commitment or identical deterministic rebuild; ambiguity stated honestly"
            }
            Self::OracleCrash => "infrastructure error, not a correctness failure or a pass",
            Self::HostOverheat => "samples contaminated, preserved and excluded by frozen protocol",
            Self::ProductTelemetryDown => "diagnostics missing; ledger and trace-off benchmark continue",
            Self::ControlTelemetryDown => "visibility degraded; ledger and state machine continue",
            Self::SubscriberMiss => "replay the outbox from ledger sequence, idempotently",
            Self::IssuerUnavailable => "no new credentials; signed historic results preserved",
            Self::LocalModelUnavailable => "pending tasks pause; trusted verification still works",
            Self::KeyCompromised => "revoke, rotate, record timeline, reissue; invalidate new evidence",
            Self::HostRootCompromised => {
                "assume plaintext and key exposure; rotate secrets and rebuild from external anchor"
            }
        }
    }
}
