//! The eight role-specific message purposes of the v5 bridge.
//!
//! Grounded in the `V4_TRUST_BRIDGE.md:17-25` table: each row names a sender, a
//! message kind, an authorized recipient and the required evidence. These are
//! proposed envelope payload contracts, not v4 wire types. Sending a message
//! never grants funding, publication or release (`:28-31`).

/// A purpose-specific message kind with its evidence contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Purpose {
    /// `research.case.assign` (`:19`).
    CaseAssign {
        sender: &'static str,
        recipient: &'static str,
        evidence: &'static str,
    },
    /// `research.bid.submit` (`:20`).
    BidSubmit {
        sender: &'static str,
        recipient: &'static str,
        evidence: &'static str,
    },
    /// `research.claim.submit` (`:21`).
    ClaimSubmit {
        sender: &'static str,
        recipient: &'static str,
        evidence: &'static str,
    },
    /// `resource.reserve.receipt` (`:22`).
    ReserveReceipt {
        sender: &'static str,
        recipient: &'static str,
        evidence: &'static str,
    },
    /// `experiment.result.attest` (`:23`).
    ResultAttest {
        sender: &'static str,
        recipient: &'static str,
        evidence: &'static str,
    },
    /// `publication.submit` (`:24`).
    PublicationSubmit {
        sender: &'static str,
        recipient: &'static str,
        evidence: &'static str,
    },
    /// `publication.release` (`:25`).
    PublicationRelease {
        sender: &'static str,
        recipient: &'static str,
        evidence: &'static str,
    },
    /// `research.transfer.propose` (`:26`).
    TransferPropose {
        sender: &'static str,
        recipient: &'static str,
        evidence: &'static str,
    },
}

impl Purpose {
    /// All eight proposed payload purposes in table order.
    pub const ALL: [Self; 8] = [
        Self::CaseAssign {
            sender: "Coordinator",
            recipient: "Named researcher",
            evidence: "Signed task, budget limits, context scope",
        },
        Self::BidSubmit {
            sender: "Researcher",
            recipient: "Exchange",
            evidence: "Schema-validated bid and preregistered forecast",
        },
        Self::ClaimSubmit {
            sender: "Researcher",
            recipient: "Claim registry",
            evidence: "Claim ID, context commitment, signed opinion",
        },
        Self::ReserveReceipt {
            sender: "Treasurer",
            recipient: "Assigned worker + coordinator",
            evidence: "Transaction ID, policy version, idempotency ID",
        },
        Self::ResultAttest {
            sender: "Experiment evaluator",
            recipient: "Claim registry / approved researcher",
            evidence: "Oracle, signed samples and cohort identity",
        },
        Self::PublicationSubmit {
            sender: "Researcher",
            recipient: "Scoped scientific publisher",
            evidence: "Minimal recipient-encrypted evidence capsule",
        },
        Self::PublicationRelease {
            sender: "Publisher",
            recipient: "Corpus index",
            evidence: "Sanitized paper hash, declassification grant, release signature",
        },
        Self::TransferPropose {
            sender: "Integrator",
            recipient: "Research exchange",
            evidence: "Paper source, new destination case, budget request",
        },
    ];

    /// The wire message kind for this purpose.
    #[must_use]
    pub const fn message_type(self) -> &'static str {
        match self {
            Self::CaseAssign { .. } => "research.case.assign",
            Self::BidSubmit { .. } => "research.bid.submit",
            Self::ClaimSubmit { .. } => "research.claim.submit",
            Self::ReserveReceipt { .. } => "resource.reserve.receipt",
            Self::ResultAttest { .. } => "experiment.result.attest",
            Self::PublicationSubmit { .. } => "publication.submit",
            Self::PublicationRelease { .. } => "publication.release",
            Self::TransferPropose { .. } => "research.transfer.propose",
        }
    }

    /// Required evidence for this purpose (`V4_TRUST_BRIDGE.md:17-25`).
    #[must_use]
    pub const fn evidence(self) -> &'static str {
        match self {
            Self::CaseAssign { evidence, .. }
            | Self::BidSubmit { evidence, .. }
            | Self::ClaimSubmit { evidence, .. }
            | Self::ReserveReceipt { evidence, .. }
            | Self::ResultAttest { evidence, .. }
            | Self::PublicationSubmit { evidence, .. }
            | Self::PublicationRelease { evidence, .. }
            | Self::TransferPropose { evidence, .. } => evidence,
        }
    }
}
