//! The parent research director: a replaceable reasoning edge, not the
//! coordinator.
//!
//! Grounded in `ARCHITECTURE.md:38-40,94-96`, `AGENT_CONTRACTS.md:7`
//! (Research director row: sees approved summaries and budgets; writes
//! hypothesis queue; forbidden from altering data or direct release),
//! `REQUIREMENTS_V3.md` AGT3-010 and `playbooks/fanout-5.yaml:96-104`.
//! It holds a wide read view and a proposal capability. It never edits
//! evidence, signs a measurement or changes active policy.

use crate::coordinator::authority::Authority;
use crate::director::arbitration::Arbitration;
use crate::director::proposal::Proposal;
use crate::director::view::View;
use crate::permission::Permission;
use crate::warning::Warning;

/// An untrusted parent reasoning agent operating through typed proposals.
#[derive(Clone, Debug)]
pub struct Director {
    /// Stable identity of the reasoning session.
    pub session: String,
    /// The role policy version this session is bound to.
    pub role_policy_sha256: String,
    /// The authorized read view.
    pub view: View,
}

impl Director {
    /// The authority under which a director acts.
    #[must_use]
    pub const fn authority() -> Authority {
        Authority::Director
    }

    /// Issue a research-change proposal. This is advice; the coordinator must
    /// validate it exactly as it validates a child proposal
    /// (`ARCHITECTURE.md:125`).
    #[must_use]
    pub fn propose(&self, arbitration: Arbitration, generation: String) -> Proposal {
        Proposal {
            session: self.session.clone(),
            role_policy_sha256: self.role_policy_sha256.clone(),
            arbitration,
            generation,
        }
    }

    /// Whether this session may edit authoritative evidence. It never may.
    #[must_use]
    pub const fn may_edit_evidence() -> Permission {
        Permission::Deny
    }

    /// Read one authorized summary from the view; unauthorized reads degrade to
    /// a warning rather than panicking.
    #[must_use]
    pub fn read(&self, key: &str) -> Result<String, Warning> {
        self.view.summary(key).ok_or_else(|| Warning::Unreachable {
            service: "director-view",
        })
    }
}
