//! The full typed operation index.
//!
//! All 120 rows of `api/HTTP_INDEX.md:9-128` are reproduced as typed
//! `Operation` entries: 67 external (43 base + 24 v6.1 projections) and 53
//! internal (26 base + 27 v6.1 service writes). Operation IDs, methods, path
//! templates, schema references, allowed roles and pinned revisions are copied
//! from the canonical index; `api/public.openapi.yaml` and
//! `api/internal.openapi.yaml` carry the same IDs and `x-lab-roles`. No entry
//! is mounted as a route.

use crate::api::{Method, Operation, Plane};

/// The canonical operation catalog.
pub struct Index {
    operations: Vec<Operation>,
}

impl Index {
    /// Build the full v6.1 operation index.
    #[must_use]
    pub fn of() -> Self {
        Self {
            operations: vec![
        Operation::of(
            "getLiveness", Method::Get, "/health/live", Plane::External,
            "public", None, Some("status.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getReadiness", Method::Get, "/health/ready", Plane::External,
            "public", None, Some("status.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getStatus", Method::Get, "/v1/status", Plane::External,
            "public", None, Some("status.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listCampaigns", Method::Get, "/v1/campaigns", Plane::External,
            "public", None, Some("page-campaign.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getCampaignsById", Method::Get, "/v1/campaigns/{id}", Plane::External,
            "public", None, Some("campaign.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listExperiments", Method::Get, "/v1/experiments", Plane::External,
            "public", None, Some("page-experiment.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getExperimentsById", Method::Get, "/v1/experiments/{id}", Plane::External,
            "public", None, Some("experiment.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listMutations", Method::Get, "/v1/mutations", Plane::External,
            "public", None, Some("page-mutation.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getMutationsById", Method::Get, "/v1/mutations/{id}", Plane::External,
            "public", None, Some("mutation.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listResearchCases", Method::Get, "/v1/research/cases", Plane::External,
            "public", None, Some("page-research-case.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getResearchCasesById", Method::Get, "/v1/research/cases/{id}", Plane::External,
            "public", None, Some("research-case.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listResearchPapers", Method::Get, "/v1/research/papers", Plane::External,
            "public", None, Some("page-research-paper.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getResearchPapersById", Method::Get, "/v1/research/papers/{id}", Plane::External,
            "public", None, Some("research-paper.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listWorkers", Method::Get, "/v1/workers", Plane::External,
            "public", None, Some("page-worker-status.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getWorkersById", Method::Get, "/v1/workers/{id}", Plane::External,
            "public", None, Some("worker-status.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listHosts", Method::Get, "/v1/hosts", Plane::External,
            "public", None, Some("page-host-status.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getHostsById", Method::Get, "/v1/hosts/{id}", Plane::External,
            "public", None, Some("host-status.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listCertificates", Method::Get, "/v1/certificates", Plane::External,
            "public", None, Some("page-certificate-event.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getCertificatesById", Method::Get, "/v1/certificates/{id}", Plane::External,
            "public", None, Some("certificate-event.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listAttestations", Method::Get, "/v1/attestations", Plane::External,
            "public", None, Some("page-attestation.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getAttestationsById", Method::Get, "/v1/attestations/{id}", Plane::External,
            "public", None, Some("attestation.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listNotifications", Method::Get, "/v1/notifications", Plane::External,
            "public", None, Some("page-notification.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getNotification", Method::Get, "/v1/notifications/{id}", Plane::External,
            "public", None, Some("notification.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listAuditEvents", Method::Get, "/v1/audit/events", Plane::External,
            "public", None, Some("page-audit-event.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getAuditEventsById", Method::Get, "/v1/audit/events/{id}", Plane::External,
            "public", None, Some("audit-event.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listBenchmarks", Method::Get, "/v1/benchmarks", Plane::External,
            "public", None, Some("page-benchmark-result.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getBenchmarksById", Method::Get, "/v1/benchmarks/{id}", Plane::External,
            "public", None, Some("benchmark-result.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getLineage", Method::Get, "/v1/lineage/{id}", Plane::External,
            "public", None, Some("lineage.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getExperimentSamples", Method::Get, "/v1/experiments/{id}/samples", Plane::External,
            "public", None, Some("page-sample.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listConversationMetadata", Method::Get, "/v1/agents/conversations", Plane::External,
            "public", None, Some("page-agent-turn.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getConversationMetadata", Method::Get, "/v1/agents/conversations/{id}", Plane::External,
            "public", None, Some("artifact-manifest.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listToolCallMetadata", Method::Get, "/v1/agents/tool-calls", Plane::External,
            "public", None, Some("page-tool-call.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getToolCall", Method::Get, "/v1/agents/tool-calls/{id}", Plane::External,
            "public", None, Some("tool-call.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getArtifactMetadata", Method::Get, "/v1/artifacts/{digest}/metadata", Plane::External,
            "public", None, Some("artifact-ref.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listControlTraces", Method::Get, "/v1/telemetry/control/traces", Plane::External,
            "public", None, Some("page-trace-record.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listProductTraces", Method::Get, "/v1/telemetry/product/traces", Plane::External,
            "public", None, Some("page-trace-record.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listControlMetrics", Method::Get, "/v1/telemetry/control/metrics", Plane::External,
            "public", None, Some("page-metric-record.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "listProductMetrics", Method::Get, "/v1/telemetry/product/metrics", Plane::External,
            "public", None, Some("page-metric-record.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getEventHistory", Method::Get, "/v1/events", Plane::External,
            "public", None, Some("page-audit-event.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "getPolicy", Method::Get, "/v1/policies/{id}", Plane::External,
            "public", None, Some("source-policy.schema.json"), &["observer"], "6",
        ),
        Operation::of(
            "submitOwnerCommand", Method::Post, "/v1/operator/commands", Plane::External,
            "public", Some("command.schema.json"), Some("ack.schema.json"), &["owner", "operator"], "6",
        ),
        Operation::of(
            "validatePolicyCandidate", Method::Post, "/v1/operator/policies/validate", Plane::External,
            "public", Some("source-policy.schema.json"), Some("ack.schema.json"), &["owner", "operator"], "6",
        ),
        Operation::of(
            "streamEvents", Method::Get, "/v1/events/stream", Plane::External,
            "public", None, None, &["observer"], "6",
        ),
        Operation::of(
            "v61ListCustodyRequest", Method::Get, "/v1/custody/requests", Plane::External,
            "public", None, Some("page-v61-custody-request.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListCustodyReceipt", Method::Get, "/v1/custody/receipts", Plane::External,
            "public", None, Some("page-v61-custody-receipt.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListHolderCapability", Method::Get, "/v1/custody/holders", Plane::External,
            "public", None, Some("page-v61-holder-capability.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListEmbargoRule", Method::Get, "/v1/policies/embargoes", Plane::External,
            "public", None, Some("page-v61-embargo-rule.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListLicenseGrant", Method::Get, "/v1/policies/licenses", Plane::External,
            "public", None, Some("page-v61-license-grant.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListTradeRestriction", Method::Get, "/v1/policies/trade-graph", Plane::External,
            "public", None, Some("page-v61-trade-restriction.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListResearchTrack", Method::Get, "/v1/research/tracks", Plane::External,
            "public", None, Some("page-v61-research-track.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListResearchView", Method::Get, "/v1/research/views", Plane::External,
            "public", None, Some("page-v61-research-view.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListFeedbackRecord", Method::Get, "/v1/research/feedback", Plane::External,
            "public", None, Some("page-v61-feedback-record.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListKnowledgeRelease", Method::Get, "/v1/research/knowledge-releases", Plane::External,
            "public", None, Some("page-v61-knowledge-release.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListRepositoryAdmission", Method::Get, "/v1/repositories/admissions", Plane::External,
            "public", None, Some("page-v61-repository-admission.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListDenialAudit", Method::Get, "/v1/audit/denials", Plane::External,
            "public", None, Some("page-v61-denial-audit.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListExperimentSpec", Method::Get, "/v1/experiments/specifications", Plane::External,
            "public", None, Some("page-v61-experiment-spec.schema.json"), &["observer"], "6.1",
        ),
        Operation::of(
            "v61ListStorageLease", Method::Get, "/v1/storage/leases", Plane::External,
            "public", None, Some("page-v61-storage-lease.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListTemporalFact", Method::Get, "/v1/facts/timeline", Plane::External,
            "public", None, Some("page-v61-temporal-fact.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListCandidateBatch", Method::Get, "/v1/research/candidate-batches", Plane::External,
            "public", None, Some("page-v61-candidate-batch.schema.json"), &["observer"], "6.1",
        ),
        Operation::of(
            "v61ListExperimentIntent", Method::Get, "/v1/research/intentions", Plane::External,
            "public", None, Some("page-v61-experiment-intent.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListReleaseCompatibility", Method::Get, "/v1/research/transfer-compatibility", Plane::External,
            "public", None, Some("page-v61-release-compatibility.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListArtifactPolicy", Method::Get, "/v1/artifacts/policies", Plane::External,
            "public", None, Some("page-v61-artifact-policy.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListBoundaryObservation", Method::Get, "/v1/security/boundary-observations", Plane::External,
            "public", None, Some("page-v61-boundary-observation.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListCurriculumRecords", Method::Get, "/v1/research/curricula", Plane::External,
            "public", None, Some("page-v61-curriculum.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListStudyAssignmentRecords", Method::Get, "/v1/research/studies/assignments", Plane::External,
            "public", None, Some("page-v61-study-assignment.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListStudyOutcomeRecords", Method::Get, "/v1/research/studies/outcomes", Plane::External,
            "public", None, Some("page-v61-study-outcome.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "v61ListPhysicalAllocationRecords", Method::Get, "/v1/research/resources/allocations", Plane::External,
            "public", None, Some("page-v61-physical-allocation.schema.json"), &["operator"], "6.1",
        ),
        Operation::of(
            "claimWorkerTask", Method::Post, "/internal/v1/worker/claim", Plane::Internal,
            "internal", Some("worker-claim-request.schema.json"), Some("ack.schema.json"), &["supervisor"], "6",
        ),
        Operation::of(
            "heartbeatWorkerLease", Method::Post, "/internal/v1/worker/heartbeat", Plane::Internal,
            "internal", Some("worker-heartbeat.schema.json"), Some("ack.schema.json"), &["supervisor"], "6",
        ),
        Operation::of(
            "completeWorkerTask", Method::Post, "/internal/v1/worker/complete", Plane::Internal,
            "internal", Some("worker-complete-request.schema.json"), Some("ack.schema.json"), &["supervisor"], "6",
        ),
        Operation::of(
            "recordSourceSnapshot", Method::Post, "/internal/v1/source/snapshot", Plane::Internal,
            "internal", Some("source-snapshot.schema.json"), Some("ack.schema.json"), &["source-adapter"], "6",
        ),
        Operation::of(
            "stageScopedTask", Method::Post, "/internal/v1/task/stage", Plane::Internal,
            "internal", Some("task-staging-request.schema.json"), Some("ack.schema.json"), &["coordinator"], "6",
        ),
        Operation::of(
            "commitArtifact", Method::Post, "/internal/v1/artifacts/commit", Plane::Internal,
            "internal", Some("artifact-commit-request.schema.json"), Some("ack.schema.json"), &["supervisor", "builder", "oracle", "runner", "publisher", "coordinator"], "6",
        ),
        Operation::of(
            "ingestPatch", Method::Post, "/internal/v1/proposals/patch", Plane::Internal,
            "internal", Some("patch-proposal.schema.json"), Some("ack.schema.json"), &["supervisor"], "6",
        ),
        Operation::of(
            "ingestResearchClaim", Method::Post, "/internal/v1/proposals/claim", Plane::Internal,
            "internal", Some("research-claim.schema.json"), Some("ack.schema.json"), &["supervisor"], "6",
        ),
        Operation::of(
            "ingestTestProposal", Method::Post, "/internal/v1/proposals/test", Plane::Internal,
            "internal", Some("test-proposal.schema.json"), Some("ack.schema.json"), &["supervisor"], "6",
        ),
        Operation::of(
            "requestBuild", Method::Post, "/internal/v1/build/request", Plane::Internal,
            "internal", Some("build-request.schema.json"), Some("ack.schema.json"), &["coordinator"], "6",
        ),
        Operation::of(
            "reportBuild", Method::Post, "/internal/v1/build/result", Plane::Internal,
            "internal", Some("build-result.schema.json"), Some("ack.schema.json"), &["builder"], "6",
        ),
        Operation::of(
            "requestOracle", Method::Post, "/internal/v1/oracle/request", Plane::Internal,
            "internal", Some("oracle-request.schema.json"), Some("ack.schema.json"), &["coordinator"], "6",
        ),
        Operation::of(
            "reportOracle", Method::Post, "/internal/v1/oracle/result", Plane::Internal,
            "internal", Some("oracle-result.schema.json"), Some("ack.schema.json"), &["oracle"], "6",
        ),
        Operation::of(
            "reserveBenchHost", Method::Post, "/internal/v1/benchmark/lease", Plane::Internal,
            "internal", Some("benchmark-lease-request.schema.json"), Some("ack.schema.json"), &["scheduler"], "6",
        ),
        Operation::of(
            "requestBenchmark", Method::Post, "/internal/v1/benchmark/request", Plane::Internal,
            "internal", Some("benchmark-request.schema.json"), Some("ack.schema.json"), &["coordinator"], "6",
        ),
        Operation::of(
            "reportBenchmark", Method::Post, "/internal/v1/benchmark/result", Plane::Internal,
            "internal", Some("benchmark-result.schema.json"), Some("ack.schema.json"), &["runner"], "6",
        ),
        Operation::of(
            "reportEvaluation", Method::Post, "/internal/v1/evaluation/result", Plane::Internal,
            "internal", Some("evaluation-result.schema.json"), Some("ack.schema.json"), &["evaluator"], "6",
        ),
        Operation::of(
            "prepareFeedback", Method::Post, "/internal/v1/feedback/prepare", Plane::Internal,
            "internal", Some("feedback-capsule.schema.json"), Some("ack.schema.json"), &["feedback-authority"], "6",
        ),
        Operation::of(
            "stagePaper", Method::Post, "/internal/v1/research/paper", Plane::Internal,
            "internal", Some("paper-stage-request.schema.json"), Some("ack.schema.json"), &["publisher"], "6",
        ),
        Operation::of(
            "publishAttestation", Method::Post, "/internal/v1/publication/attest", Plane::Internal,
            "internal", Some("attestation.schema.json"), Some("ack.schema.json"), &["publication-authority"], "6",
        ),
        Operation::of(
            "issueCredential", Method::Post, "/internal/v1/identity/issue", Plane::Internal,
            "internal", Some("identity-issuance-request.schema.json"), Some("ack.schema.json"), &["identity-authority"], "6",
        ),
        Operation::of(
            "revokeCredential", Method::Post, "/internal/v1/identity/revoke", Plane::Internal,
            "internal", Some("identity-revoke-request.schema.json"), Some("ack.schema.json"), &["identity-authority"], "6",
        ),
        Operation::of(
            "recordAttestation", Method::Post, "/internal/v1/attestations/record", Plane::Internal,
            "internal", Some("attestation.schema.json"), Some("ack.schema.json"), &["attestation-service"], "6",
        ),
        Operation::of(
            "appendLedgerEvent", Method::Post, "/internal/v1/ledger/append", Plane::Internal,
            "internal", Some("event-append-request.schema.json"), Some("ack.schema.json"), &["coordinator", "ledger-gateway"], "6",
        ),
        Operation::of(
            "ingestControlTelemetry", Method::Post, "/internal/v1/telemetry/control/ingest", Plane::Internal,
            "internal", Some("telemetry-batch.schema.json"), Some("ack.schema.json"), &["otel-control-gateway"], "6",
        ),
        Operation::of(
            "ingestProductTelemetry", Method::Post, "/internal/v1/telemetry/product/ingest", Plane::Internal,
            "internal", Some("telemetry-batch.schema.json"), Some("ack.schema.json"), &["otel-product-gateway"], "6",
        ),
        Operation::of(
            "v61RecordRepositoryAdmission", Method::Post, "/internal/v1/repos/admit", Plane::Internal,
            "internal", Some("v61-repository-admission.schema.json"), Some("ack.schema.json"), &["source-adapter"], "6.1",
        ),
        Operation::of(
            "v61RecordArtifactPolicy", Method::Post, "/internal/v1/artifacts/policy/bind", Plane::Internal,
            "internal", Some("v61-artifact-policy.schema.json"), Some("ack.schema.json"), &["custody-policy"], "6.1",
        ),
        Operation::of(
            "v61RecordHolderCapability", Method::Post, "/internal/v1/holders/attest", Plane::Internal,
            "internal", Some("v61-holder-capability.schema.json"), Some("ack.schema.json"), &["holder-supervisor"], "6.1",
        ),
        Operation::of(
            "v61RecordCustodyRequest", Method::Post, "/internal/v1/custody/preflight", Plane::Internal,
            "internal", Some("v61-custody-request.schema.json"), Some("ack.schema.json"), &["custody-broker"], "6.1",
        ),
        Operation::of(
            "v61RecordCustodyDecision", Method::Post, "/internal/v1/custody/decide", Plane::Internal,
            "internal", Some("v61-custody-decision.schema.json"), Some("ack.schema.json"), &["policy-service"], "6.1",
        ),
        Operation::of(
            "v61RecordCustodyReceipt", Method::Post, "/internal/v1/custody/commit", Plane::Internal,
            "internal", Some("v61-custody-receipt.schema.json"), Some("ack.schema.json"), &["custody-broker"], "6.1",
        ),
        Operation::of(
            "v61RecordStorageLease", Method::Post, "/internal/v1/custody/reserve", Plane::Internal,
            "internal", Some("v61-storage-lease.schema.json"), Some("ack.schema.json"), &["storage-broker"], "6.1",
        ),
        Operation::of(
            "v61RecordLicenseGrant", Method::Post, "/internal/v1/licenses/register", Plane::Internal,
            "internal", Some("v61-license-grant.schema.json"), Some("ack.schema.json"), &["license-registry"], "6.1",
        ),
        Operation::of(
            "v61RecordEmbargoRule", Method::Post, "/internal/v1/embargoes/register", Plane::Internal,
            "internal", Some("v61-embargo-rule.schema.json"), Some("ack.schema.json"), &["policy-service"], "6.1",
        ),
        Operation::of(
            "v61RecordTradeRestriction", Method::Post, "/internal/v1/trade/restrict", Plane::Internal,
            "internal", Some("v61-trade-restriction.schema.json"), Some("ack.schema.json"), &["policy-service"], "6.1",
        ),
        Operation::of(
            "v61RecordResearchTrack", Method::Post, "/internal/v1/research/track", Plane::Internal,
            "internal", Some("v61-research-track.schema.json"), Some("ack.schema.json"), &["coordinator"], "6.1",
        ),
        Operation::of(
            "v61RecordResearchView", Method::Post, "/internal/v1/research/view/compile", Plane::Internal,
            "internal", Some("v61-research-view.schema.json"), Some("ack.schema.json"), &["context-projector"], "6.1",
        ),
        Operation::of(
            "v61RecordTaskContextPlan", Method::Post, "/internal/v1/research/task-context", Plane::Internal,
            "internal", Some("v61-task-context-plan.schema.json"), Some("ack.schema.json"), &["supervisor"], "6.1",
        ),
        Operation::of(
            "v61RecordExperimentIntent", Method::Post, "/internal/v1/research/intent/ingest", Plane::Internal,
            "internal", Some("v61-experiment-intent.schema.json"), Some("ack.schema.json"), &["supervisor"], "6.1",
        ),
        Operation::of(
            "v61RecordExperimentSpec", Method::Post, "/internal/v1/experiments/spec/compile", Plane::Internal,
            "internal", Some("v61-experiment-spec.schema.json"), Some("ack.schema.json"), &["experiment-compiler"], "6.1",
        ),
        Operation::of(
            "v61RecordCandidateBatch", Method::Post, "/internal/v1/research/candidates/register", Plane::Internal,
            "internal", Some("v61-candidate-batch.schema.json"), Some("ack.schema.json"), &["coordinator"], "6.1",
        ),
        Operation::of(
            "v61RecordFeedbackRecord", Method::Post, "/internal/v1/feedback/project", Plane::Internal,
            "internal", Some("v61-feedback-record.schema.json"), Some("ack.schema.json"), &["feedback-projector"], "6.1",
        ),
        Operation::of(
            "v61RecordKnowledgeRelease", Method::Post, "/internal/v1/knowledge/release", Plane::Internal,
            "internal", Some("v61-knowledge-release.schema.json"), Some("ack.schema.json"), &["publication-bridge"], "6.1",
        ),
        Operation::of(
            "v61RecordReleaseCompatibility", Method::Post, "/internal/v1/knowledge/compatibility", Plane::Internal,
            "internal", Some("v61-release-compatibility.schema.json"), Some("ack.schema.json"), &["policy-service"], "6.1",
        ),
        Operation::of(
            "v61RecordDenialAudit", Method::Post, "/internal/v1/audit/denial", Plane::Internal,
            "internal", Some("v61-denial-audit.schema.json"), Some("ack.schema.json"), &["audit-service"], "6.1",
        ),
        Operation::of(
            "v61RecordTemporalFact", Method::Post, "/internal/v1/facts/append", Plane::Internal,
            "internal", Some("v61-temporal-fact.schema.json"), Some("ack.schema.json"), &["ledger"], "6.1",
        ),
        Operation::of(
            "v61RecordWorkerNeutralReceipt", Method::Post, "/internal/v1/workers/neutral-receipt", Plane::Internal,
            "internal", Some("v61-worker-neutral-receipt.schema.json"), Some("ack.schema.json"), &["supervisor"], "6.1",
        ),
        Operation::of(
            "v61RecordBoundaryObservation", Method::Post, "/internal/v1/research/boundary/record", Plane::Internal,
            "internal", Some("v61-boundary-observation.schema.json"), Some("ack.schema.json"), &["supervisor"], "6.1",
        ),
        Operation::of(
            "v61RecordCurriculumRecord", Method::Post, "/internal/v1/curriculum/register", Plane::Internal,
            "internal", Some("v61-curriculum.schema.json"), Some("ack.schema.json"), &["curriculum-service"], "6.1",
        ),
        Operation::of(
            "v61RecordStudyAssignmentRecord", Method::Post, "/internal/v1/curriculum/study/assign", Plane::Internal,
            "internal", Some("v61-study-assignment.schema.json"), Some("ack.schema.json"), &["curriculum-service"], "6.1",
        ),
        Operation::of(
            "v61RecordStudyOutcomeRecord", Method::Post, "/internal/v1/curriculum/study/outcome", Plane::Internal,
            "internal", Some("v61-study-outcome.schema.json"), Some("ack.schema.json"), &["study-evaluator"], "6.1",
        ),
        Operation::of(
            "v61RecordPhysicalAllocationRecord", Method::Post, "/internal/v1/resources/allocate", Plane::Internal,
            "internal", Some("v61-physical-allocation.schema.json"), Some("ack.schema.json"), &["resource-scheduler"], "6.1",
        ),
            ],
        }
    }

    /// Number of declared operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Whether the index is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Iterate over all declared operations.
    pub fn iter(&self) -> impl Iterator<Item = &Operation> {
        self.operations.iter()
    }

    /// Find an operation by its stable ID.
    #[must_use]
    pub fn find(&self, id: &str) -> Option<&Operation> {
        self.operations.iter().find(|operation| operation.id == id)
    }

    /// Count operations on one plane.
    #[must_use]
    pub fn count_plane(&self, plane: Plane) -> usize {
        self.operations.iter().filter(|operation| operation.plane == plane).count()
    }

    /// Count operations pinned to one policy revision.
    #[must_use]
    pub fn count_revision(&self, revision: &str) -> usize {
        self.operations.iter().filter(|operation| operation.revision == revision).count()
    }

    /// Collect operation IDs declared more than once, for structural findings.
    #[must_use]
    pub fn duplicates(&self) -> Vec<&'static str> {
        self.operations
            .iter()
            .map(|operation| operation.id)
            .fold((Vec::new(), Vec::new()), |(seen, dupes), id| match seen.contains(&id) {
                true => (seen, [dupes, vec![id]].concat()),
                false => ([seen, vec![id]].concat(), dupes),
            })
            .1
    }

    /// Collect every schema reference this index depends on.
    #[must_use]
    pub fn references(&self) -> Vec<&'static str> {
        self.operations
            .iter()
            .flat_map(|operation| [operation.input, operation.output])
            .flatten()
            .collect()
    }
}
