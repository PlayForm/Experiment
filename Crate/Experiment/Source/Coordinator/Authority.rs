//! Authority separation: which actor may sign a measurement or change policy.
//!
//! Grounded in `ARCHITECTURE.md:28-31,119-125`, `AGENT_CONTRACTS.md:1-23`,
//! `REQUIREMENTS_V3.md` INV-001/AGT3-010/EXP-010. The central rule: an LLM is an
//! untrusted-with-respect-to-measurement adviser/worker; only the deterministic
//! coordinator, runner, evaluator or authorized human may sign a measurement
//! or change active policy. The parent director has a wide read view and a
//! proposal capability, not absolute root authority.

use crate::permission::Permission;

/// An actor in the laboratory's authority model.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Authority {
    /// The deterministic control service; owns truth and state.
    Coordinator,
    /// The parent reasoning agent; strategic adviser only.
    Director,
    /// A scoped coding agent producing one committed mutation.
    Patcher,
    /// A read-only hotspot reporter.
    Scout,
    /// A scoped evidence explainer.
    Critic,
    /// A deterministic comparator with optional advisory model.
    Comparator,
    /// The non-LLM builder.
    Builder,
    /// The trusted, frozen-commit correctness oracle.
    Oracle,
    /// The trusted, exclusive benchmark measurement executor.
    Runner,
    /// The non-LLM statistical evaluator.
    Evaluator,
    /// The complete-record evidence curator.
    Curator,
    /// An explicitly authorized human reviewer.
    Human,
}

impl Authority {
    /// All actors, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Coordinator,
        Self::Director,
        Self::Patcher,
        Self::Scout,
        Self::Critic,
        Self::Comparator,
        Self::Builder,
        Self::Oracle,
        Self::Runner,
        Self::Evaluator,
        Self::Curator,
        Self::Human,
    ];

    /// Whether this actor is a replaceable reasoning edge (an LLM caller).
    #[must_use]
    pub const fn is_reasoning_edge(self) -> bool {
        matches!(
            self,
            Self::Director | Self::Patcher | Self::Scout | Self::Critic | Self::Comparator
        )
    }

    /// Whether this actor may sign a measurement. No reasoning edge may.
    #[must_use]
    pub const fn may_sign_measurement(self) -> Permission {
        match self {
            Self::Runner | Self::Evaluator | Self::Coordinator => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Whether this actor may change active policy. Only the deterministic
    /// coordinator and an explicit human review may.
    #[must_use]
    pub const fn may_change_policy(self) -> Permission {
        match self {
            Self::Coordinator | Self::Human => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Whether this actor may edit authoritative evidence. The director is
    /// explicitly excluded (`ARCHITECTURE.md:40`).
    #[must_use]
    pub const fn may_edit_evidence(self) -> Permission {
        match self {
            Self::Coordinator | Self::Human => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// Whether this actor may hold an exclusive benchmark lease.
    #[must_use]
    pub const fn may_hold_benchmark_lease(self) -> Permission {
        match self {
            Self::Runner | Self::Coordinator => Permission::Allow,
            _ => Permission::Deny,
        }
    }
}
