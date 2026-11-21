//! Pinned state machines from both policy revisions.
//!
//! `policy/state-machines.yaml:1-143` declares five v6 machines (campaign,
//! mutation, worker, benchmark, publication) and `policy/v61-state-machines.yaml:1-96`
//! declares three v6.1 machines (custody, knowledge_release, feedback). Revisions
//! are additive: an old machine stays valid for its old pin
//! (`docs/00-system-constitution.md:57-62`).

use crate::policy::Transition;

/// One pinned state machine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateMachine {
    /// Machine name.
    pub name: &'static str,
    /// Policy revision that pins it.
    pub revision: &'static str,
    /// Initial state.
    pub initial: &'static str,
    /// Declared states.
    pub states: &'static [&'static str],
    /// Guarded transitions.
    pub transitions: &'static [Transition],
}

impl StateMachine {
    /// Build one state machine.
    #[must_use]
    pub const fn of(
        name: &'static str,
        revision: &'static str,
        initial: &'static str,
        states: &'static [&'static str],
        transitions: &'static [Transition],
    ) -> Self {
        Self { name, revision, initial, states, transitions }
    }

    /// All eight declared machines.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
        StateMachine::of("campaign", "6", "draft", &["active", "cancelled", "completed", "draft", "draining", "failed", "paused", "validated"], &[
                Transition::of("draft", "validated", "policy-service"),
                Transition::of("validated", "active", "coordinator-on-owner-command"),
                Transition::of("active", "paused", "coordinator-on-owner-command"),
                Transition::of("paused", "active", "coordinator-on-owner-command"),
                Transition::of("active", "draining", "coordinator-on-owner-command"),
                Transition::of("draining", "completed", "coordinator-on-all-jobs-terminal"),
                Transition::of("active", "failed", "coordinator-on-critical-fault"),
                Transition::of("paused", "cancelled", "coordinator-on-owner-command"),
                Transition::of("active", "cancelled", "coordinator-on-owner-command"),
        ]),
        StateMachine::of("mutation", "6", "proposed", &["admitted", "archived", "built", "eligible", "evaluated", "measured", "oracle_failed", "oracle_passed", "promoted", "proposed", "rejected"], &[
                Transition::of("proposed", "admitted", "scope-validator"),
                Transition::of("proposed", "rejected", "scope-validator"),
                Transition::of("admitted", "built", "builder-attested"),
                Transition::of("built", "oracle_failed", "oracle-attested"),
                Transition::of("built", "oracle_passed", "oracle-attested"),
                Transition::of("oracle_passed", "measured", "runner-attested"),
                Transition::of("measured", "evaluated", "evaluator-attested"),
                Transition::of("evaluated", "eligible", "policy-engine"),
                Transition::of("evaluated", "rejected", "policy-engine"),
                Transition::of("eligible", "promoted", "release-gate"),
                Transition::of("oracle_failed", "archived", "coordinator"),
                Transition::of("rejected", "archived", "coordinator"),
        ]),
        StateMachine::of("worker", "6", "idle", &["assigned", "failed", "idle", "quarantined", "running", "staging", "terminated"], &[
                Transition::of("idle", "assigned", "scheduler-lease"),
                Transition::of("assigned", "running", "supervisor"),
                Transition::of("running", "staging", "supervisor"),
                Transition::of("staging", "idle", "supervisor-on-artifact-seal"),
                Transition::of("running", "failed", "supervisor"),
                Transition::of("assigned", "failed", "supervisor"),
                Transition::of("failed", "quarantined", "host-verifier"),
                Transition::of("quarantined", "terminated", "operator"),
        ]),
        StateMachine::of("benchmark", "6", "queued", &["contaminated", "evaluated", "leased", "queued", "running", "sealed"], &[
                Transition::of("queued", "leased", "scheduler-exclusive"),
                Transition::of("leased", "running", "runner-start"),
                Transition::of("running", "sealed", "runner-evidence"),
                Transition::of("running", "contaminated", "runner-integrity"),
                Transition::of("sealed", "evaluated", "evaluator-attestation"),
                Transition::of("contaminated", "queued", "coordinator-retry-new-attempt"),
        ]),
        StateMachine::of("publication", "6", "draft", &["draft", "internal-released", "private-reviewed", "public-released", "rejected", "sanitization-pending", "sanitized"], &[
                Transition::of("draft", "private-reviewed", "review-authority"),
                Transition::of("private-reviewed", "sanitization-pending", "publisher"),
                Transition::of("sanitization-pending", "sanitized", "sanitizer"),
                Transition::of("sanitized", "internal-released", "publication-authority"),
                Transition::of("internal-released", "public-released", "publication-authority-on-owner-policy"),
                Transition::of("draft", "rejected", "publication-authority"),
                Transition::of("sanitization-pending", "rejected", "sanitizer"),
        ]),
        StateMachine::of("custody", "6.1", "requested", &["requested", "preflight", "reserved", "staged", "verified", "committed", "quarantined", "denied", "expired", "released"], &[
                Transition::of("requested", "preflight", "custody-broker"),
                Transition::of("preflight", "reserved", "policy-service"),
                Transition::of("preflight", "denied", "policy-service"),
                Transition::of("reserved", "staged", "storage-broker"),
                Transition::of("staged", "verified", "holder-supervisor"),
                Transition::of("verified", "committed", "custody-broker"),
                Transition::of("reserved", "quarantined", "custody-broker"),
                Transition::of("staged", "quarantined", "custody-broker"),
                Transition::of("committed", "released", "custody-broker"),
        ]),
        StateMachine::of("knowledge_release", "6.1", "draft", &["draft", "rights_pending", "sanitization_pending", "reviewed", "authorized", "published", "denied", "revoked"], &[
                Transition::of("draft", "rights_pending", "publication-bridge"),
                Transition::of("rights_pending", "sanitization_pending", "policy-service"),
                Transition::of("rights_pending", "denied", "policy-service"),
                Transition::of("sanitization_pending", "reviewed", "sanitizer"),
                Transition::of("reviewed", "authorized", "publication-bridge"),
                Transition::of("authorized", "published", "publication-bridge"),
                Transition::of("published", "revoked", "publication-bridge"),
        ]),
        StateMachine::of("feedback", "6.1", "pending", &["pending", "evidence_sealed", "rights_checked", "projected", "released", "blocked"], &[
                Transition::of("pending", "evidence_sealed", "evaluator"),
                Transition::of("evidence_sealed", "rights_checked", "policy-service"),
                Transition::of("rights_checked", "projected", "feedback-projector"),
                Transition::of("projected", "released", "feedback-projector"),
                Transition::of("rights_checked", "blocked", "policy-service"),
        ]),
        ]
    }

    /// Whether an edge is registered on this machine.
    #[must_use]
    pub fn permits(&self, from: &str, to: &str) -> crate::Permission {
        match self.transitions.iter().any(|edge| edge.matches(from, to)) {
            true => crate::Permission::Allow,
            false => crate::Permission::Deny,
        }
    }

    /// Whether a named state is declared.
    #[must_use]
    pub fn has_state(&self, state: &str) -> crate::Permission {
        match self.states.iter().any(|declared| *declared == state) {
            true => crate::Permission::Allow,
            false => crate::Permission::Deny,
        }
    }
}
