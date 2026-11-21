//! The role that owns a playbook phase.
//!
//! Grounded in `playbooks/autonomous-week.yaml:25,31,37,43,49,55,61,67,73,79`
//! and the ten agent roles at `REQUIREMENTS.md:559-578`.

/// The owning role of a phase.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Role {
    Runner,
    Scout,
    Mutator,
    Builder,
    Oracle,
    Analyst,
    Curator,
    Integrator,
}

impl Role {
    /// The exact YAML `role` token.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Runner => "runner",
            Self::Scout => "scout",
            Self::Mutator => "mutator",
            Self::Builder => "builder",
            Self::Oracle => "oracle",
            Self::Analyst => "analyst",
            Self::Curator => "curator",
            Self::Integrator => "integrator",
        }
    }

    /// Whether this role may promote a candidate. Only the coordinator may;
    /// no phase role may self-promote (REQ-AGT-18).
    #[must_use]
    pub const fn promotes(self) -> bool {
        false
    }
}
