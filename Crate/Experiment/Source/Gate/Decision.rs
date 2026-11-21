//! Versioned policy approval and rejection; advisory output has no authority.
//!
//! Grounded in `RESEARCH_GATE.md:26-32,95-117` (advisory classifier cannot erase
//! a novel patch or promote evidence; only versioned policies approve/reject;
//! manual overrides are recorded and policy-scoped; stamps are immutable signed
//! attestations), `ARCHITECTURE.md:46-51` and `REQUIREMENTS_V3.md` NOV-007.

use crate::gate::reason::Reason;
use crate::permission::Permission;

/// A gate decision, carrying its authority and provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Decision {
    /// A versioned deterministic policy approved the candidate.
    PolicyApproved {
        policy_version: String,
        algorithm: String,
    },
    /// A versioned deterministic policy rejected the candidate.
    PolicyRejected {
        policy_version: String,
        reasons: Vec<Reason>,
    },
    /// An advisory model only categorized an ambiguous overlap.
    AdvisoryOnly { algorithm: String },
    /// A human override was recorded and policy-scoped.
    ManualOverride {
        policy_version: String,
        justification: String,
    },
}

impl Decision {
    /// Whether this decision is authoritative for selection.
    #[must_use]
    pub const fn is_authoritative(&self) -> Permission {
        match self {
            Self::AdvisoryOnly { .. } => Permission::Deny,
            _ => Permission::Allow,
        }
    }

    /// Whether this decision may promote a candidate to the next stage.
    #[must_use]
    pub const fn may_promote(&self) -> Permission {
        match self {
            Self::PolicyApproved { .. } | Self::ManualOverride { .. } => Permission::Allow,
            _ => Permission::Deny,
        }
    }

    /// The policy version that issued the decision, if any.
    #[must_use]
    pub fn policy(&self) -> Option<&str> {
        match self {
            Self::PolicyApproved { policy_version, .. }
            | Self::PolicyRejected { policy_version, .. }
            | Self::ManualOverride { policy_version, .. } => Some(policy_version),
            Self::AdvisoryOnly { .. } => None,
        }
    }

    /// The reasons recorded on a rejection, empty for other outcomes.
    #[must_use]
    pub fn reasons(&self) -> &[Reason] {
        match self {
            Self::PolicyRejected { reasons, .. } => reasons,
            _ => &[],
        }
    }
}
