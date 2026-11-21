//! Accepted forms of staged untrusted worker output.
//!
//! Grounded in `docs/11-agent-contracts.md:74-84` and
//! `docs/22-hidden-worker-and-correspondence.md:23-27`: a worker submits a
//! patch, a falsifiable research claim, proposed tests, a draft paper or an
//! integration proposal. Each is an untrusted artifact; a requested action is
//! never an executable command and a proposed test is never a trusted oracle
//! edit (`docs/02-pitfalls-and-verification.md:13-16`, `docs/15` BND-003).

/// A proposed artifact kind from the offline worker.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Proposal {
    /// Unified diff plus claimed changed symbols and base tree id.
    Patch,
    /// Falsifiable mechanism, conditions, contrary evidence and confidence.
    ResearchClaim,
    /// New cases and invariants; never trusted oracle edits.
    TestProposal,
    /// Claim-to-evidence narrative, private until sanitization.
    DraftPaper,
    /// Transfer hypothesis for a new repository; never auto-deployed.
    IntegrationProposal,
}

impl Proposal {
    /// Every proposal is untrusted regardless of how it is worded.
    #[must_use]
    pub const fn trust(self) -> crate::Permission {
        crate::Permission::Deny
    }

    /// The staged file name the supervisor expects in scratch output.
    #[must_use]
    pub const fn file_name(self) -> &'static str {
        match self {
            Self::Patch => "change.patch",
            Self::ResearchClaim => "claims.json",
            Self::TestProposal => "proposed-tests/",
            Self::DraftPaper => "paper-draft.md",
            Self::IntegrationProposal => "integration-proposal.json",
        }
    }

    /// Whether this proposal can ever authorize a release decision.
    #[must_use]
    pub const fn authorizes_release(self) -> crate::Permission {
        crate::Permission::Deny
    }
}
