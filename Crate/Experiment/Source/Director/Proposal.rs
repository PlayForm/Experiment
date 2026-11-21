//! A director-authored research-change proposal.
//!
//! Grounded in `AGENT_CONTRACTS.md:7` (return object `ResearchPlan`),
//! `ARCHITECTURE.md:94-96` (trigger a fanout, cancel tracks, request
//! integration/confirmation) and `playbooks/fanout-5.yaml:96-104` (next-generation
//! actions). This is deliberately a different type from `interface::Proposal`,
//! which is the typed mirror of the formal proposal.v3 schema submitted by a
//! patcher. A director proposal is advice and carries no authority.

use crate::director::arbitration::Arbitration;

/// An advisory research change for the coordinator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    /// The director session that authored this advice.
    pub session: String,
    /// The role policy version bound to the session.
    pub role_policy_sha256: String,
    /// The requested arbitration outcome.
    pub arbitration: Arbitration,
    /// The generation this advice applies to.
    pub generation: String,
}

impl Proposal {
    /// Whether this proposal may edit raw samples. It never may
    /// (`REQUIREMENTS_V3.md` AGT3-010).
    #[must_use]
    pub const fn may_mutate_samples() -> bool {
        false
    }

    /// The generation this advice is scoped to, for lineage.
    #[must_use]
    pub fn scope(&self) -> &str {
        &self.generation
    }
}
