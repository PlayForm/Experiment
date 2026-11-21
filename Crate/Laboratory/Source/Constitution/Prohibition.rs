//! Typed prohibitions - the complete "what may never happen" list.
//!
//! Grounded in `docs/00-system-constitution.md:17-39`,
//! `docs/17-v61-constitution.md:81-96` (authority separation),
//! `docs/11-agent-contracts.md:86-89`, `docs/15-acceptance-tests.md:19-46`
//! (BND-001..024) and `docs/25-failure-analysis.md:3-34` (F01..F30). A worker
//! cannot call APIs, see schemas, control Docker, sign facts or alter policy;
//! a signature cannot manufacture truth; a paper cannot launder weak evidence.
//! Each prohibition is a typed value, never a comment the compiler ignores.

use std::fmt;

/// A rule that may never be violated by any component of this crate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Prohibition {
    /// A worker may not call any public or internal API.
    CallApi,
    /// A worker may not see ledger, telemetry, topology or PKI schemas.
    SeeSchema,
    /// A worker may not read another worker's correspondence.
    ReadSibling,
    /// A worker may not control Docker or any privileged socket.
    ControlDocker,
    /// A worker may not sign a benchmark or correctness fact.
    SignFact,
    /// A worker may not alter active policy.
    AlterPolicy,
    /// A generated artifact is not an executable command.
    ExecuteProposal,
    /// A patcher-authored test cannot replace the trusted oracle.
    SelfTestAsOracle,
    /// No promotion on unsigned source-of-truth measurement.
    PromoteUnsigned,
    /// No promotion on a verdict produced solely by an LLM.
    PromoteByOpinion,
    /// No generated patch may modify the kernel, CA, builder, oracle or verifier.
    PatchTrustedCore,
    /// No primary benchmark may use product tracing as authority.
    TraceInAuthority,
    /// No ledger or telemetry may claim private thought process.
    ClaimThought,
    /// No external hosted inference endpoint may be used.
    RemoteInference,
    /// No cross-repository gain claim without a fresh target experiment.
    CrossRepoGeneralization,
    /// No fabricated passing test, artifact delivery or benchmark success.
    FabricateSuccess,
    /// No policy-service error may be read as permission.
    ErrorAsPermission,
    /// No direct model-to-model transport exists.
    ModelToModelChat,
}

impl Prohibition {
    /// The principal this prohibition binds.
    #[must_use]
    pub const fn bound_principal(self) -> &'static str {
        match self {
            Self::CallApi
            | Self::SeeSchema
            | Self::ReadSibling
            | Self::ControlDocker
            | Self::SignFact
            | Self::AlterPolicy
            | Self::ExecuteProposal
            | Self::RemoteInference
            | Self::ModelToModelChat
            | Self::FabricateSuccess => "offline-worker",
            Self::SelfTestAsOracle | Self::PromoteByOpinion | Self::CrossRepoGeneralization => {
                "generated-artifact"
            }
            Self::PromoteUnsigned | Self::TraceInAuthority | Self::ClaimThought => {
                "trusted-kernel"
            }
            Self::PatchTrustedCore => "governed-maintenance-project",
            Self::ErrorAsPermission => "policy-service",
        }
    }

    /// Human-readable statement of the barred action.
    #[must_use]
    pub const fn line(self) -> &'static str {
        match self {
            Self::CallApi => "worker cannot call public or internal APIs",
            Self::SeeSchema => "worker cannot see ledger, telemetry, topology or PKI schemas",
            Self::ReadSibling => "worker cannot read sibling correspondence",
            Self::ControlDocker => "worker cannot control Docker or privileged sockets",
            Self::SignFact => "worker cannot sign benchmark or correctness facts",
            Self::AlterPolicy => "worker cannot alter active policy",
            Self::ExecuteProposal => "a staged artifact is never an executable command",
            Self::SelfTestAsOracle => "a patcher test never replaces the trusted oracle",
            Self::PromoteUnsigned => "no promotion on unsigned measurement",
            Self::PromoteByOpinion => "no promotion on an LLM-only verdict",
            Self::PatchTrustedCore => "no generated patch edits kernel, CA, builder or oracle",
            Self::TraceInAuthority => "product tracing is not authority-grade timing",
            Self::ClaimThought => "no claim about an unpublished agent's reasoning",
            Self::RemoteInference => "all inference is local and pre-provisioned",
            Self::CrossRepoGeneralization => "no gain claim without a fresh target experiment",
            Self::FabricateSuccess => "no fabricated pass, delivery or benchmark success",
            Self::ErrorAsPermission => "a policy error fails closed, never permits",
            Self::ModelToModelChat => "siblings have no direct chat or shared volume",
        }
    }
}

impl fmt::Display for Prohibition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.line())
    }
}
