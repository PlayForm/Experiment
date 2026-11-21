//! One typed event catalog entry.
//!
//! Every row of `events/catalog.json` carries a kind, family, payload schema,
//! producer, consumer, privacy class, authority flag and (for v6.1) a
//! `worker_visible` flag. All 87 entries are authoritative and
//! none is worker-visible. The entry is data: it never emits an event.

use crate::{schema, Permission};
use crate::event::{Kind, Privacy};

/// One authoritative event descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entry {
    /// Signed event kind.
    pub kind: Kind,
    /// Schema family that owns the payload.
    pub family: schema::Family,
    /// Inventory family tag from the catalog.
    pub tag: &'static str,
    /// Payload schema reference.
    pub schema: &'static str,
    /// Authorized producing service.
    pub producer: &'static str,
    /// Consuming service (`ledger` throughout).
    pub consumer: &'static str,
    /// Wire privacy class.
    pub privacy: Privacy,
    /// Whether the entry is authoritative.
    pub authoritative: Permission,
    /// Whether a worker may observe the entry.
    pub visible: Permission,
}

impl Entry {
    /// Build one catalog row from trusted research data.
    #[must_use]
    pub const fn of(
        kind: Kind,
        family: schema::Family,
        tag: &'static str,
        schema: &'static str,
        producer: &'static str,
        consumer: &'static str,
        privacy: Privacy,
        authoritative: Permission,
        visible: Permission,
    ) -> Self {
        Self { kind, family, tag, schema, producer, consumer, privacy, authoritative, visible }
    }

    /// The complete catalog of signed event entries.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
        Entry::of(
            Kind::CampaignCreated, schema::Family::Base, "campaign",
            "../schemas/campaign.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CampaignValidated, schema::Family::Base, "campaign",
            "../schemas/campaign.schema.json", "policy-service", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CampaignActivated, schema::Family::Base, "campaign",
            "../schemas/campaign.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CampaignPaused, schema::Family::Base, "campaign",
            "../schemas/campaign.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CampaignDrained, schema::Family::Base, "campaign",
            "../schemas/campaign.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CampaignCompleted, schema::Family::Base, "campaign",
            "../schemas/campaign.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CampaignFailed, schema::Family::Base, "campaign",
            "../schemas/campaign.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::SourceSnapshotCaptured, schema::Family::Base, "source",
            "../schemas/source-snapshot.schema.json", "source-adapter", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::SourceScopeValidated, schema::Family::Base, "source",
            "../schemas/source-snapshot.schema.json", "scope-validator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::DependencyClosurePinned, schema::Family::Base, "source",
            "../schemas/artifact-manifest.schema.json", "source-adapter", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ResearchCaseOpened, schema::Family::Base, "research",
            "../schemas/research-case.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BlindAssignmentCreated, schema::Family::Base, "research",
            "../schemas/task-assignment.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ResearchClaimSubmitted, schema::Family::Base, "research",
            "../schemas/research-claim.schema.json", "supervisor", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ResearchClaimRecorded, schema::Family::Base, "research",
            "../schemas/research-claim.schema.json", "knowledge-registry", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ResearchDisagreementRecorded, schema::Family::Base, "research",
            "../schemas/research-claim.schema.json", "knowledge-registry", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PaperStaged, schema::Family::Base, "research",
            "../schemas/research-paper.schema.json", "publication-service", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PaperSanitized, schema::Family::Base, "research",
            "../schemas/research-paper.schema.json", "sanitizer", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PaperReleased, schema::Family::Base, "research",
            "../schemas/research-paper.schema.json", "publication-authority", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PaperRetracted, schema::Family::Base, "research",
            "../schemas/research-paper.schema.json", "publication-authority", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::TransferHypothesisOpened, schema::Family::Base, "research",
            "../schemas/research-case.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::TaskStaged, schema::Family::Base, "worker",
            "../schemas/task-bundle.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::WorkerLeaseGranted, schema::Family::Base, "worker",
            "../schemas/lease.schema.json", "scheduler", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::WorkerStarted, schema::Family::Base, "worker",
            "../schemas/worker-status.schema.json", "supervisor", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ModelTurnSealed, schema::Family::Base, "worker",
            "../schemas/agent-turn.schema.json", "supervisor", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::LocalToolCallSealed, schema::Family::Base, "worker",
            "../schemas/tool-call.schema.json", "supervisor", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::WorkerOutputStaged, schema::Family::Base, "worker",
            "../schemas/task-receipt.schema.json", "supervisor", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::WorkerTimedOut, schema::Family::Base, "worker",
            "../schemas/worker-status.schema.json", "supervisor", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::WorkerQuarantined, schema::Family::Base, "worker",
            "../schemas/worker-status.schema.json", "supervisor", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::FeedbackCapsulePrepared, schema::Family::Base, "worker",
            "../schemas/feedback-capsule.schema.json", "feedback-authority", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PatchSubmitted, schema::Family::Base, "mutation",
            "../schemas/patch-proposal.schema.json", "supervisor", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PatchRejectedAsDuplicate, schema::Family::Base, "mutation",
            "../schemas/patch-proposal.schema.json", "novelty-gate", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PatchScopeRejected, schema::Family::Base, "mutation",
            "../schemas/patch-proposal.schema.json", "scope-validator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::MutationAdmitted, schema::Family::Base, "mutation",
            "../schemas/mutation.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::MutationRejected, schema::Family::Base, "mutation",
            "../schemas/mutation.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::MutationPromoted, schema::Family::Base, "mutation",
            "../schemas/mutation.schema.json", "release-gate", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BuildRequested, schema::Family::Base, "build",
            "../schemas/build-request.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BuildCompleted, schema::Family::Base, "build",
            "../schemas/build-result.schema.json", "builder", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BuildFailed, schema::Family::Base, "build",
            "../schemas/build-result.schema.json", "builder", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::OracleRequested, schema::Family::Base, "oracle",
            "../schemas/oracle-request.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::OraclePassed, schema::Family::Base, "oracle",
            "../schemas/oracle-result.schema.json", "oracle", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::OracleFailed, schema::Family::Base, "oracle",
            "../schemas/oracle-result.schema.json", "oracle", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::OracleInconclusive, schema::Family::Base, "oracle",
            "../schemas/oracle-result.schema.json", "oracle", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BenchmarkLeaseGranted, schema::Family::Base, "benchmark",
            "../schemas/lease.schema.json", "scheduler", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BenchmarkRequested, schema::Family::Base, "benchmark",
            "../schemas/benchmark-request.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BenchmarkCompleted, schema::Family::Base, "benchmark",
            "../schemas/benchmark-result.schema.json", "runner", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BenchmarkContaminated, schema::Family::Base, "benchmark",
            "../schemas/benchmark-result.schema.json", "runner", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BenchmarkLeaseExpired, schema::Family::Base, "benchmark",
            "../schemas/lease.schema.json", "scheduler", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::EvaluationCompleted, schema::Family::Base, "benchmark",
            "../schemas/evaluation-result.schema.json", "evaluator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CredentialIssued, schema::Family::Base, "security",
            "../schemas/certificate-event.schema.json", "pki", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CredentialRevoked, schema::Family::Base, "security",
            "../schemas/certificate-event.schema.json", "pki", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::AttestationRecorded, schema::Family::Base, "security",
            "../schemas/attestation.schema.json", "attestation-service", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PolicyValidated, schema::Family::Base, "security",
            "../schemas/source-policy.schema.json", "policy-service", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PolicyRejected, schema::Family::Base, "security",
            "../schemas/source-policy.schema.json", "policy-service", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::HostQuarantined, schema::Family::Base, "operations",
            "../schemas/host-status.schema.json", "host-verifier", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ArtifactCommitted, schema::Family::Base, "operations",
            "../schemas/artifact-ref.schema.json", "cas", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::TelemetryPipelineDegraded, schema::Family::Base, "operations",
            "../schemas/notification.schema.json", "telemetry-gateway", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::EventReplayCompleted, schema::Family::Base, "operations",
            "../schemas/audit-event.schema.json", "coordinator", "ledger",
            Privacy::MetadataOnly, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::RepositoryAdmitted, schema::Family::Base, "source",
            "../schemas/v61-repository-admission.schema.json", "source-adapter", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ArtifactPolicyBound, schema::Family::V61, "custody",
            "../schemas/v61-artifact-policy.schema.json", "custody-policy", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::HolderQualified, schema::Family::V61, "custody",
            "../schemas/v61-holder-capability.schema.json", "holder-supervisor", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CustodyRequested, schema::Family::V61, "custody",
            "../schemas/v61-custody-request.schema.json", "custody-broker", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CustodyAuthorized, schema::Family::V61, "custody",
            "../schemas/v61-custody-decision.schema.json", "policy-service", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CustodyDenied, schema::Family::V61, "custody",
            "../schemas/v61-custody-decision.schema.json", "policy-service", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CustodyStorageReserved, schema::Family::V61, "custody",
            "../schemas/v61-storage-lease.schema.json", "storage-broker", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CustodyCommitted, schema::Family::V61, "custody",
            "../schemas/v61-custody-receipt.schema.json", "custody-broker", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CustodyQuarantined, schema::Family::V61, "custody",
            "../schemas/v61-custody-receipt.schema.json", "custody-broker", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::EmbargoRegistered, schema::Family::V61, "license",
            "../schemas/v61-embargo-rule.schema.json", "policy-service", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::LicenseGrantReviewed, schema::Family::V61, "license",
            "../schemas/v61-license-grant.schema.json", "license-registry", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::TradeEdgeRestricted, schema::Family::V61, "license",
            "../schemas/v61-trade-restriction.schema.json", "policy-service", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ResearchTrackAssigned, schema::Family::Base, "research",
            "../schemas/v61-research-track.schema.json", "coordinator", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ResearchViewCompiled, schema::Family::Base, "research",
            "../schemas/v61-research-view.schema.json", "context-projector", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::WorkerContextStaged, schema::Family::Base, "worker",
            "../schemas/v61-task-context-plan.schema.json", "supervisor", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ExperimentIntentIngested, schema::Family::Base, "research",
            "../schemas/v61-experiment-intent.schema.json", "supervisor", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ExperimentSpecCompiled, schema::Family::Base, "research",
            "../schemas/v61-experiment-spec.schema.json", "experiment-compiler", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::CandidateBatchRegistered, schema::Family::Base, "research",
            "../schemas/v61-candidate-batch.schema.json", "coordinator", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PrivateFeedbackProjected, schema::Family::V61, "feedback",
            "../schemas/v61-feedback-record.schema.json", "feedback-projector", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::KnowledgeReleaseRequested, schema::Family::V61, "knowledge",
            "../schemas/v61-knowledge-release.schema.json", "publication-bridge", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::KnowledgeReleaseAuthorized, schema::Family::V61, "knowledge",
            "../schemas/v61-knowledge-release.schema.json", "publication-bridge", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::KnowledgeCompatibilityRecorded, schema::Family::V61, "knowledge",
            "../schemas/v61-release-compatibility.schema.json", "policy-service", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PolicyDenialRecorded, schema::Family::Base, "security",
            "../schemas/v61-denial-audit.schema.json", "audit-service", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::BitemporalFactAppended, schema::Family::V61, "audit",
            "../schemas/v61-temporal-fact.schema.json", "ledger", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::WorkerNeutralReceiptIssued, schema::Family::Base, "worker",
            "../schemas/v61-worker-neutral-receipt.schema.json", "supervisor", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::WorkerBoundaryObserved, schema::Family::Base, "security",
            "../schemas/v61-boundary-observation.schema.json", "supervisor", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::ResearchCurriculumRegistered, schema::Family::V61, "curriculum",
            "../schemas/v61-curriculum.schema.json", "curriculum-service", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::StudyAssignmentCreated, schema::Family::V61, "curriculum",
            "../schemas/v61-study-assignment.schema.json", "curriculum-service", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::StudyOutcomeRecorded, schema::Family::V61, "curriculum",
            "../schemas/v61-study-outcome.schema.json", "study-evaluator", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        Entry::of(
            Kind::PhysicalAllocationGranted, schema::Family::V61, "curriculum",
            "../schemas/v61-physical-allocation.schema.json", "resource-scheduler", "ledger",
            Privacy::RestrictedMetadata, Permission::Allow, Permission::Deny,
        ),
        ]
    }
}
