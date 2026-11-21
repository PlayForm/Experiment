//! The typed example registry.
//!
//! Every example stem in `examples/` is paired with the schema stem it
//! conforms to. The registry is data: no example file is read and no instance
//! is validated at runtime. Grounded in `examples/sample.example.json:1-19`,
//! `examples/v61-custody-request.example.json:1-14` and the full example list.

use crate::example::Family;

/// One typed example descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Example {
    /// Example stem without extension.
    pub stem: &'static str,
    /// Example family.
    pub family: Family,
    /// Stem of the schema this example conforms to.
    pub schema: &'static str,
    /// Canonical schema filename.
    pub schema_file: &'static str,
}

impl Example {
    /// Build one example descriptor.
    #[must_use]
    pub const fn descriptor(
        stem: &'static str,
        family: Family,
        schema: &'static str,
        schema_file: &'static str,
    ) -> Self {
        Self { stem, family, schema, schema_file }
    }

    /// The complete example registry (112 entries).
    #[must_use]
    pub fn of() -> Vec<Self> {
        vec![
        Example::descriptor("ack", Family::Base, "ack", "ack.schema.json"),
        Example::descriptor("actor-ref", Family::Base, "actor-ref", "actor-ref.schema.json"),
        Example::descriptor("agent-turn", Family::Base, "agent-turn", "agent-turn.schema.json"),
        Example::descriptor("api-error", Family::Base, "api-error", "api-error.schema.json"),
        Example::descriptor("artifact-commit-request", Family::Base, "artifact-commit-request", "artifact-commit-request.schema.json"),
        Example::descriptor("artifact-manifest", Family::Base, "artifact-manifest", "artifact-manifest.schema.json"),
        Example::descriptor("artifact-ref", Family::Base, "artifact-ref", "artifact-ref.schema.json"),
        Example::descriptor("attestation", Family::Base, "attestation", "attestation.schema.json"),
        Example::descriptor("audit-event", Family::Base, "audit-event", "audit-event.schema.json"),
        Example::descriptor("benchmark-lease-request", Family::Base, "benchmark-lease-request", "benchmark-lease-request.schema.json"),
        Example::descriptor("benchmark-protocol", Family::Base, "benchmark-protocol", "benchmark-protocol.schema.json"),
        Example::descriptor("benchmark-request", Family::Base, "benchmark-request", "benchmark-request.schema.json"),
        Example::descriptor("benchmark-result", Family::Base, "benchmark-result", "benchmark-result.schema.json"),
        Example::descriptor("build-request", Family::Base, "build-request", "build-request.schema.json"),
        Example::descriptor("build-result", Family::Base, "build-result", "build-result.schema.json"),
        Example::descriptor("campaign", Family::Base, "campaign", "campaign.schema.json"),
        Example::descriptor("certificate-event", Family::Base, "certificate-event", "certificate-event.schema.json"),
        Example::descriptor("command", Family::Base, "command", "command.schema.json"),
        Example::descriptor("envelope", Family::Base, "envelope", "envelope.schema.json"),
        Example::descriptor("evaluation-result", Family::Base, "evaluation-result", "evaluation-result.schema.json"),
        Example::descriptor("event-append-request", Family::Base, "event-append-request", "event-append-request.schema.json"),
        Example::descriptor("experiment", Family::Base, "experiment", "experiment.schema.json"),
        Example::descriptor("feedback-capsule", Family::Base, "feedback-capsule", "feedback-capsule.schema.json"),
        Example::descriptor("hardware-fingerprint", Family::Base, "hardware-fingerprint", "hardware-fingerprint.schema.json"),
        Example::descriptor("host-status", Family::Base, "host-status", "host-status.schema.json"),
        Example::descriptor("identity-issuance-request", Family::Base, "identity-issuance-request", "identity-issuance-request.schema.json"),
        Example::descriptor("identity-revoke-request", Family::Base, "identity-revoke-request", "identity-revoke-request.schema.json"),
        Example::descriptor("lease", Family::Base, "lease", "lease.schema.json"),
        Example::descriptor("lineage", Family::Base, "lineage", "lineage.schema.json"),
        Example::descriptor("metric-record", Family::Base, "metric-record", "metric-record.schema.json"),
        Example::descriptor("mutation", Family::Base, "mutation", "mutation.schema.json"),
        Example::descriptor("notification", Family::Base, "notification", "notification.schema.json"),
        Example::descriptor("oracle-request", Family::Base, "oracle-request", "oracle-request.schema.json"),
        Example::descriptor("oracle-result", Family::Base, "oracle-result", "oracle-result.schema.json"),
        Example::descriptor("page", Family::Base, "page", "page.schema.json"),
        Example::descriptor("page-v61-artifact-policy", Family::PageV61, "page-v61-artifact-policy", "page-v61-artifact-policy.schema.json"),
        Example::descriptor("page-v61-boundary-observation", Family::PageV61, "page-v61-boundary-observation", "page-v61-boundary-observation.schema.json"),
        Example::descriptor("page-v61-candidate-batch", Family::PageV61, "page-v61-candidate-batch", "page-v61-candidate-batch.schema.json"),
        Example::descriptor("page-v61-curriculum", Family::PageV61, "page-v61-curriculum", "page-v61-curriculum.schema.json"),
        Example::descriptor("page-v61-custody-receipt", Family::PageV61, "page-v61-custody-receipt", "page-v61-custody-receipt.schema.json"),
        Example::descriptor("page-v61-custody-request", Family::PageV61, "page-v61-custody-request", "page-v61-custody-request.schema.json"),
        Example::descriptor("page-v61-denial-audit", Family::PageV61, "page-v61-denial-audit", "page-v61-denial-audit.schema.json"),
        Example::descriptor("page-v61-embargo-rule", Family::PageV61, "page-v61-embargo-rule", "page-v61-embargo-rule.schema.json"),
        Example::descriptor("page-v61-experiment-intent", Family::PageV61, "page-v61-experiment-intent", "page-v61-experiment-intent.schema.json"),
        Example::descriptor("page-v61-experiment-spec", Family::PageV61, "page-v61-experiment-spec", "page-v61-experiment-spec.schema.json"),
        Example::descriptor("page-v61-feedback-record", Family::PageV61, "page-v61-feedback-record", "page-v61-feedback-record.schema.json"),
        Example::descriptor("page-v61-holder-capability", Family::PageV61, "page-v61-holder-capability", "page-v61-holder-capability.schema.json"),
        Example::descriptor("page-v61-knowledge-release", Family::PageV61, "page-v61-knowledge-release", "page-v61-knowledge-release.schema.json"),
        Example::descriptor("page-v61-license-grant", Family::PageV61, "page-v61-license-grant", "page-v61-license-grant.schema.json"),
        Example::descriptor("page-v61-physical-allocation", Family::PageV61, "page-v61-physical-allocation", "page-v61-physical-allocation.schema.json"),
        Example::descriptor("page-v61-release-compatibility", Family::PageV61, "page-v61-release-compatibility", "page-v61-release-compatibility.schema.json"),
        Example::descriptor("page-v61-repository-admission", Family::PageV61, "page-v61-repository-admission", "page-v61-repository-admission.schema.json"),
        Example::descriptor("page-v61-research-track", Family::PageV61, "page-v61-research-track", "page-v61-research-track.schema.json"),
        Example::descriptor("page-v61-research-view", Family::PageV61, "page-v61-research-view", "page-v61-research-view.schema.json"),
        Example::descriptor("page-v61-storage-lease", Family::PageV61, "page-v61-storage-lease", "page-v61-storage-lease.schema.json"),
        Example::descriptor("page-v61-study-assignment", Family::PageV61, "page-v61-study-assignment", "page-v61-study-assignment.schema.json"),
        Example::descriptor("page-v61-study-outcome", Family::PageV61, "page-v61-study-outcome", "page-v61-study-outcome.schema.json"),
        Example::descriptor("page-v61-temporal-fact", Family::PageV61, "page-v61-temporal-fact", "page-v61-temporal-fact.schema.json"),
        Example::descriptor("page-v61-trade-restriction", Family::PageV61, "page-v61-trade-restriction", "page-v61-trade-restriction.schema.json"),
        Example::descriptor("paper-stage-request", Family::Base, "paper-stage-request", "paper-stage-request.schema.json"),
        Example::descriptor("patch-proposal", Family::Base, "patch-proposal", "patch-proposal.schema.json"),
        Example::descriptor("policy-validation-result", Family::Base, "policy-validation-result", "policy-validation-result.schema.json"),
        Example::descriptor("research-case", Family::Base, "research-case", "research-case.schema.json"),
        Example::descriptor("research-claim", Family::Base, "research-claim", "research-claim.schema.json"),
        Example::descriptor("research-paper", Family::Base, "research-paper", "research-paper.schema.json"),
        Example::descriptor("sample", Family::Base, "sample", "sample.schema.json"),
        Example::descriptor("source-policy", Family::Base, "source-policy", "source-policy.schema.json"),
        Example::descriptor("source-snapshot", Family::Base, "source-snapshot", "source-snapshot.schema.json"),
        Example::descriptor("status", Family::Base, "status", "status.schema.json"),
        Example::descriptor("task-assignment", Family::Base, "task-assignment", "task-assignment.schema.json"),
        Example::descriptor("task-bundle", Family::Base, "task-bundle", "task-bundle.schema.json"),
        Example::descriptor("task-receipt", Family::Base, "task-receipt", "task-receipt.schema.json"),
        Example::descriptor("task-staging-request", Family::Base, "task-staging-request", "task-staging-request.schema.json"),
        Example::descriptor("telemetry-batch", Family::Base, "telemetry-batch", "telemetry-batch.schema.json"),
        Example::descriptor("test-observation", Family::Base, "test-observation", "test-observation.schema.json"),
        Example::descriptor("test-plan", Family::Base, "test-plan", "test-plan.schema.json"),
        Example::descriptor("test-proposal", Family::Base, "test-proposal", "test-proposal.schema.json"),
        Example::descriptor("tool-call", Family::Base, "tool-call", "tool-call.schema.json"),
        Example::descriptor("trace-record", Family::Base, "trace-record", "trace-record.schema.json"),
        Example::descriptor("v61-artifact-policy", Family::V61, "v61-artifact-policy", "v61-artifact-policy.schema.json"),
        Example::descriptor("v61-boundary-observation", Family::V61, "v61-boundary-observation", "v61-boundary-observation.schema.json"),
        Example::descriptor("v61-candidate-batch", Family::V61, "v61-candidate-batch", "v61-candidate-batch.schema.json"),
        Example::descriptor("v61-curriculum", Family::V61, "v61-curriculum", "v61-curriculum.schema.json"),
        Example::descriptor("v61-custody-decision", Family::V61, "v61-custody-decision", "v61-custody-decision.schema.json"),
        Example::descriptor("v61-custody-receipt", Family::V61, "v61-custody-receipt", "v61-custody-receipt.schema.json"),
        Example::descriptor("v61-custody-request", Family::V61, "v61-custody-request", "v61-custody-request.schema.json"),
        Example::descriptor("v61-denial-audit", Family::V61, "v61-denial-audit", "v61-denial-audit.schema.json"),
        Example::descriptor("v61-embargo-rule", Family::V61, "v61-embargo-rule", "v61-embargo-rule.schema.json"),
        Example::descriptor("v61-experiment-intent", Family::V61, "v61-experiment-intent", "v61-experiment-intent.schema.json"),
        Example::descriptor("v61-experiment-spec", Family::V61, "v61-experiment-spec", "v61-experiment-spec.schema.json"),
        Example::descriptor("v61-feedback-record", Family::V61, "v61-feedback-record", "v61-feedback-record.schema.json"),
        Example::descriptor("v61-holder-capability", Family::V61, "v61-holder-capability", "v61-holder-capability.schema.json"),
        Example::descriptor("v61-knowledge-release", Family::V61, "v61-knowledge-release", "v61-knowledge-release.schema.json"),
        Example::descriptor("v61-license-grant", Family::V61, "v61-license-grant", "v61-license-grant.schema.json"),
        Example::descriptor("v61-offline-submission", Family::V61, "v61-offline-submission", "v61-offline-submission.schema.json"),
        Example::descriptor("v61-offline-task", Family::V61, "v61-offline-task", "v61-offline-task.schema.json"),
        Example::descriptor("v61-physical-allocation", Family::V61, "v61-physical-allocation", "v61-physical-allocation.schema.json"),
        Example::descriptor("v61-release-compatibility", Family::V61, "v61-release-compatibility", "v61-release-compatibility.schema.json"),
        Example::descriptor("v61-repository-admission", Family::V61, "v61-repository-admission", "v61-repository-admission.schema.json"),
        Example::descriptor("v61-research-track", Family::V61, "v61-research-track", "v61-research-track.schema.json"),
        Example::descriptor("v61-research-view", Family::V61, "v61-research-view", "v61-research-view.schema.json"),
        Example::descriptor("v61-storage-lease", Family::V61, "v61-storage-lease", "v61-storage-lease.schema.json"),
        Example::descriptor("v61-study-assignment", Family::V61, "v61-study-assignment", "v61-study-assignment.schema.json"),
        Example::descriptor("v61-study-outcome", Family::V61, "v61-study-outcome", "v61-study-outcome.schema.json"),
        Example::descriptor("v61-task-context-plan", Family::V61, "v61-task-context-plan", "v61-task-context-plan.schema.json"),
        Example::descriptor("v61-temporal-fact", Family::V61, "v61-temporal-fact", "v61-temporal-fact.schema.json"),
        Example::descriptor("v61-trade-restriction", Family::V61, "v61-trade-restriction", "v61-trade-restriction.schema.json"),
        Example::descriptor("v61-worker-neutral-receipt", Family::V61, "v61-worker-neutral-receipt", "v61-worker-neutral-receipt.schema.json"),
        Example::descriptor("worker-claim-request", Family::Base, "worker-claim-request", "worker-claim-request.schema.json"),
        Example::descriptor("worker-complete-request", Family::Base, "worker-complete-request", "worker-complete-request.schema.json"),
        Example::descriptor("worker-heartbeat", Family::Base, "worker-heartbeat", "worker-heartbeat.schema.json"),
        Example::descriptor("worker-status", Family::Base, "worker-status", "worker-status.schema.json"),
        ]
    }

    /// Count examples in one family.
    #[must_use]
    pub fn count_family(items: &[Self], family: Family) -> usize {
        items.iter().filter(|item| item.family == family).count()
    }
}
