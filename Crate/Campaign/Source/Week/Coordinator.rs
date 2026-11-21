//! Trusted coordinator contract for the bounded week.
//!
//! Grounded in `REQUIREMENTS.md:580-625` (durable state, leases with TTL,
//! checkpointing, human veto, coordinator-owned promotion), REQ-AGT-18
//! (`:601-602`), REQ-CLI-04 (`:826-827`) and `AGENT_CONTRACT.md:44-45`.
//! Agents and generated artifacts are untrusted inputs; this type encodes who
//! may promote, but promotes nothing.
//!
//! Exception declared: `Actor` and `Decision` are declared beside
//! `Coordinator` because they are its input and output vocabulary only.

use crate::agent::Permission;

/// Who is asking for an action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Actor {
    /// The trusted, durable coordinator service.
    Coordinator,
    /// An untrusted replaceable coding agent.
    Agent,
    /// A human reviewer with an explicit, recorded approval.
    Human,
}

/// The coordinator's decision about a request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    /// The actor may advance the phase.
    Advance,
    /// The actor may advance only with a recorded human approval.
    Review,
    /// The actor may not advance the phase.
    Refuse,
}

/// The authority boundary of the trusted coordinator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Coordinator {
    pub lease_ttl_seconds: u32,
    pub heartbeat_seconds: u32,
    /// Only the coordinator owns promotion (`REQ-AGT-18`).
    pub promotion: Permission,
    pub checkpoint_every_transition: Permission,
    pub emergency_stop: Permission,
    pub pause_resume: Permission,
}

impl Default for Coordinator {
    fn default() -> Self {
        Self {
            lease_ttl_seconds: 300,
            heartbeat_seconds: 30,
            promotion: Permission::Allow,
            checkpoint_every_transition: Permission::Allow,
            emergency_stop: Permission::Allow,
            pause_resume: Permission::Allow,
        }
    }
}

impl Coordinator {
    /// Decide whether an actor may promote a candidate. An agent is always
    /// refused; a human may only advance a human-gated phase.
    #[must_use]
    pub fn can_promote(&self, actor: Actor, human_gated: Permission) -> Decision {
        match (actor, human_gated, self.promotion) {
            (Actor::Agent, _, _) => Decision::Refuse,
            (Actor::Human, Permission::Allow, _) => Decision::Advance,
            (Actor::Coordinator, Permission::Deny, Permission::Allow) => Decision::Advance,
            (Actor::Coordinator, Permission::Allow, _) => Decision::Review,
            _ => Decision::Refuse,
        }
    }

    /// Whether a lost heartbeat revokes the lease and audits it
    /// (`autonomous-week.yaml:87`).
    #[must_use]
    pub fn lease_is_recoverable(&self) -> bool {
        self.heartbeat_seconds < self.lease_ttl_seconds
            && self.emergency_stop == Permission::Allow
    }

    /// Emergency stop cancels future work but leaves durable event history
    /// and partial evidence untouched (REQ-SEC-10).
    #[must_use]
    pub fn stops_without_erasure(&self) -> bool {
        self.emergency_stop == Permission::Allow && self.checkpoint_every_transition == Permission::Allow
    }
}
