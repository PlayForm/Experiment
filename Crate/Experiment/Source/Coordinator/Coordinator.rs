//! The deterministic control service: scheduler, evidence and state owner.
//!
//! Grounded in `ARCHITECTURE.md:28-31,35-37,98-109`, `REQUIREMENTS_V3.md`
//! INV-001/AGT3-001/AGT3-008, and `RUNBOOK.md:10-18`. This is the only actor
//! that advances a transition or grants a capability token. No model runs in
//! this path: it consumes typed values and emits typed transitions, so an LLM
//! can neither sign a measurement nor change active policy.

use crate::coordinator::approval::Approval;
use crate::coordinator::authority::Authority;
use crate::coordinator::quota::Quota;
use crate::coordinator::token::Token;
use crate::coordinator::transition::Transition;
use crate::permission::Permission;
use crate::warning::Warning;

/// A durable coordinator projection, rebuilt from the event ledger after a
/// crash with no dependence on any LLM conversation (`INV-005`, A0).
#[derive(Clone, Debug)]
pub struct Coordinator {
    /// The frozen generation this projection governs.
    pub generation: String,
    /// The deterministic authority this projection exercises.
    pub authority: Authority,
    /// Resource and attempt limits for the generation.
    pub quota: Quota,
    /// Capability tokens granted so far.
    pub tokens: Vec<Token>,
    /// Evidence approval stamps recorded so far.
    pub approvals: Vec<Approval>,
    /// Append-only transition stream, in logical order.
    pub transitions: Vec<Transition>,
}

impl Coordinator {
    /// Start a fresh projection for one frozen generation.
    #[must_use]
    pub fn new(generation: String, authority: Authority, quota: Quota) -> Self {
        Self {
            generation,
            authority,
            quota,
            tokens: Vec::new(),
            approvals: Vec::new(),
            transitions: Vec::new(),
        }
    }

    /// Freeze the generation before any worker is assigned (fanout phase
    /// `freeze`, `playbooks/fanout-5.yaml:11-21`).
    pub fn freeze(&mut self) -> Result<(), Warning> {
        self.record(Transition::GenerationFrozen)
    }

    /// Grant a scoped capability token; duplicate ids are rejected so a retry
    /// cannot double-submit an accepted work order (`AGT3-011`).
    pub fn grant(&mut self, token: Token) -> Result<(), Warning> {
        let duplicate = self.tokens.iter().any(|held| held.id == token.id);
        match duplicate {
            true => Err(Warning::Collision {
                domain: "token",
                id: token.id,
            }),
            false => {
                self.tokens.push(token);
                Ok(())
            }
        }
    }

    /// Record an evidence approval stamp. Release always requires an explicit
    /// human signature (`RESEARCH_GATE.md:111`, `REQUIREMENTS_V3.md` AGT3-012).
    pub fn stamp(&mut self, approval: Approval) -> Result<(), Warning> {
        // Declared exception: this one match guard needs the coordinator's
        // authority to decide whether a human gate was satisfied; a two-variant
        // enum cannot express "release approved by a non-human actor".
        match approval.needs_human() {
            Permission::Allow
                if !matches!(self.authority, Authority::Human) =>
            {
                Err(Warning::Unreachable {
                    service: "human-release-gate",
                })
            }
            _ => {
                self.approvals.push(approval);
                Ok(())
            }
        }
    }

    /// Advance the state machine by one typed transition.
    pub fn advance(&mut self, transition: Transition) -> Result<(), Warning> {
        self.record(transition)
    }

    /// The deterministic actor may always append; a reasoning edge may not.
    pub fn may_append(&self) -> Permission {
        self.authority.may_edit_evidence()
    }

    /// Append one transition, refusing an unauthorized actor without panicking.
    fn record(&mut self, transition: Transition) -> Result<(), Warning> {
        match self.may_append() {
            Permission::Allow => {
                self.transitions.push(transition);
                Ok(())
            }
            Permission::Deny => Err(Warning::Unreachable {
                service: "coordinator-append",
            }),
        }
    }
}
