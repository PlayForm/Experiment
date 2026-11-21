//! Typed schema descriptors for the base, page and v6.1 families.
//!
//! Six representative definitions are described field-by-field: `sample`,
//! `ack`, `page`, `v61-custody-request`, `v61-worker-neutral-receipt` and
//! `envelope`. The complete set of 129 schema stems is recorded as a
//! namespace list so every family is grounded without copying all 129 files.
//! `additionalProperties: false` is modelled as `Permission::Deny`.

use crate::Permission;
use crate::schema::{Constraint, Draft, Family, Field};

/// One schema definition descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Definition {
    /// Schema stem, for example `v61-custody-request`.
    pub name: &'static str,
    /// Canonical `$id`.
    pub id: &'static str,
    /// Human-readable title.
    pub title: &'static str,
    /// Declared family.
    pub family: Family,
    /// Declared draft.
    pub draft: Draft,
    /// `additionalProperties` decision; deny is the corpus default.
    pub additional_properties: Permission,
    /// Declared fields.
    pub fields: &'static [Field],
    /// Required field names.
    pub required: &'static [&'static str],
}

impl Definition {
    /// Build one definition from trusted research data.
    #[must_use]
    pub const fn descriptor(
        name: &'static str,
        id: &'static str,
        title: &'static str,
        family: Family,
        draft: Draft,
        additional_properties: Permission,
        fields: &'static [Field],
        required: &'static [&'static str],
    ) -> Self {
        Self { name, id, title, family, draft, additional_properties, fields, required }
    }

    /// The six fully described representative definitions.
    #[must_use]
    pub fn of() -> Vec<Self> {
        vec![
        Definition::descriptor(
            "sample", "https://schemas.lab.invalid/v6/sample.schema.json", "sample", Family::Base, Draft::Draft202012,
            Permission::Deny,
            &[
            Field::of("sample_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("benchmark_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("pair_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("candidate_label", Constraint::Enum(&["baseline", "candidate"])),
            Field::of("order", Constraint::Minimum(0)),
            Field::of("elapsed_ns", Constraint::Minimum(0)),
            Field::of("cpu_user_ns", Constraint::Minimum(0)),
            Field::of("cpu_system_ns", Constraint::Minimum(0)),
            Field::of("peak_rss_bytes", Constraint::Minimum(0)),
            Field::of("minor_faults", Constraint::Minimum(0)),
            Field::of("major_faults", Constraint::Minimum(0)),
            Field::of("exit_code", Constraint::Type("integer")),
            Field::of("stdout_digest", Constraint::Pattern("^[a-f0-9]{64}$")),
            Field::of("stderr_digest", Constraint::Pattern("^[a-f0-9]{64}$")),
            Field::of("environment_digest", Constraint::Pattern("^[a-f0-9]{64}$")),
            Field::of("valid", Constraint::Type("boolean")),
            Field::of("exclusion_reason", Constraint::Type("string")),
            ],
            &["sample_id", "benchmark_id", "pair_id", "candidate_label", "order", "elapsed_ns", "cpu_user_ns", "cpu_system_ns", "peak_rss_bytes", "minor_faults", "major_faults", "exit_code", "stdout_digest", "stderr_digest", "environment_digest", "valid", "exclusion_reason"],
        ),
        Definition::descriptor(
            "ack", "https://schemas.lab.invalid/v6/ack.schema.json", "ack", Family::Base, Draft::Draft202012,
            Permission::Deny,
            &[
            Field::of("request_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("accepted", Constraint::Type("boolean")),
            Field::of("record_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("event_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("state", Constraint::Type("string")),
            Field::of("receipt_digest", Constraint::Pattern("^[a-f0-9]{64}$")),
            ],
            &["request_id", "accepted", "record_id", "event_id", "state", "receipt_digest"],
        ),
        Definition::descriptor(
            "page", "https://schemas.lab.invalid/v6/page.schema.json", "page", Family::Base, Draft::Draft202012,
            Permission::Deny,
            &[
            Field::of("data", Constraint::Type("array")),
            Field::of("next_cursor", Constraint::Type("string")),
            Field::of("snapshot_seq", Constraint::Minimum(0)),
            ],
            &["data", "next_cursor", "snapshot_seq"],
        ),
        Definition::descriptor(
            "v61-custody-request", "https://schemas.lab.invalid/v6.1/v61-custody-request.schema.json", "v61-custody-request", Family::V61, Draft::Draft202012,
            Permission::Deny,
            &[
            Field::of("request_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("idempotency_key", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("artifact_digest", Constraint::Pattern("^[a-f0-9]{64}$")),
            Field::of("artifact_policy_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("source_holder_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("target_holder_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("requested_purpose", Constraint::Enum(&["build", "test", "benchmark", "research", "publication", "replication", "transfer", "archive"])),
            Field::of("requested_bytes", Constraint::Minimum(0)),
            Field::of("lease_expires_at", Constraint::Format("date-time")),
            Field::of("repository_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("policy_revision", Constraint::Pattern("^[a-f0-9]{64}$")),
            Field::of("transfer_nonce", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            ],
            &["request_id", "idempotency_key", "artifact_digest", "artifact_policy_id", "source_holder_id", "target_holder_id", "requested_purpose", "requested_bytes", "lease_expires_at", "repository_id", "policy_revision", "transfer_nonce"],
        ),
        Definition::descriptor(
            "v61-worker-neutral-receipt", "https://schemas.lab.invalid/v6.1/v61-worker-neutral-receipt.schema.json", "v61-worker-neutral-receipt", Family::V61, Draft::Draft202012,
            Permission::Deny,
            &[
            Field::of("receipt_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("task_local_nonce", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("status", Constraint::Const("submission_received")),
            Field::of("receipt_digest", Constraint::Pattern("^[a-f0-9]{64}$")),
            Field::of("visible_message", Constraint::Const("Submission received.")),
            Field::of("issued_at", Constraint::Format("date-time")),
            ],
            &["receipt_id", "task_local_nonce", "status", "receipt_digest", "visible_message", "issued_at"],
        ),
        Definition::descriptor(
            "envelope", "https://schemas.lab.invalid/v6/envelope.schema.json", "envelope", Family::Base, Draft::Draft202012,
            Permission::Deny,
            &[
            Field::of("envelope_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("protocol_version", Constraint::ConstNumber(6)),
            Field::of("message_type", Constraint::Type("string")),
            Field::of("sender_credential_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("recipient_service_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("correlation_id", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("idempotency_key", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("created_utc", Constraint::Format("date-time")),
            Field::of("expires_utc", Constraint::Format("date-time")),
            Field::of("nonce", Constraint::Pattern("^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$")),
            Field::of("ciphertext", Constraint::Reference(crate::schema::Reference::of("./artifact-ref.schema.json"))),
            Field::of("ciphertext_digest", Constraint::Pattern("^[a-f0-9]{64}$")),
            Field::of("sender_signature_b64", Constraint::Type("string")),
            Field::of("signature_scope_digest", Constraint::Pattern("^[a-f0-9]{64}$")),
            Field::of("visibility", Constraint::Enum(&["metadata-only", "sealed"])),
            Field::of("tracing_correlation_digest", Constraint::Pattern("^[a-f0-9]{64}$")),
            ],
            &["envelope_id", "protocol_version", "message_type", "sender_credential_id", "recipient_service_id", "correlation_id", "idempotency_key", "created_utc", "expires_utc", "nonce", "ciphertext", "ciphertext_digest", "sender_signature_b64", "signature_scope_digest", "visibility", "tracing_correlation_digest"],
        ),
        ]
    }

    /// Every schema stem in the corpus, as typed namespace data.
    pub const ALL: &'static [&'static str] = &[
    "ack",
    "actor-ref",
    "agent-turn",
    "api-error",
    "artifact-commit-request",
    "artifact-manifest",
    "artifact-ref",
    "attestation",
    "audit-event",
    "benchmark-lease-request",
    "benchmark-protocol",
    "benchmark-request",
    "benchmark-result",
    "build-request",
    "build-result",
    "campaign",
    "certificate-event",
    "command",
    "envelope",
    "evaluation-result",
    "event-append-request",
    "experiment",
    "feedback-capsule",
    "hardware-fingerprint",
    "host-status",
    "identity-issuance-request",
    "identity-revoke-request",
    "lease",
    "lineage",
    "metric-record",
    "mutation",
    "notification",
    "oracle-request",
    "oracle-result",
    "page",
    "page-agent-turn",
    "page-attestation",
    "page-audit-event",
    "page-benchmark-result",
    "page-campaign",
    "page-certificate-event",
    "page-experiment",
    "page-host-status",
    "page-metric-record",
    "page-mutation",
    "page-notification",
    "page-research-case",
    "page-research-paper",
    "page-sample",
    "page-tool-call",
    "page-trace-record",
    "page-v61-artifact-policy",
    "page-v61-boundary-observation",
    "page-v61-candidate-batch",
    "page-v61-curriculum",
    "page-v61-custody-receipt",
    "page-v61-custody-request",
    "page-v61-denial-audit",
    "page-v61-embargo-rule",
    "page-v61-experiment-intent",
    "page-v61-experiment-spec",
    "page-v61-feedback-record",
    "page-v61-holder-capability",
    "page-v61-knowledge-release",
    "page-v61-license-grant",
    "page-v61-physical-allocation",
    "page-v61-release-compatibility",
    "page-v61-repository-admission",
    "page-v61-research-track",
    "page-v61-research-view",
    "page-v61-storage-lease",
    "page-v61-study-assignment",
    "page-v61-study-outcome",
    "page-v61-temporal-fact",
    "page-v61-trade-restriction",
    "page-worker-status",
    "paper-stage-request",
    "patch-proposal",
    "policy-validation-result",
    "research-case",
    "research-claim",
    "research-paper",
    "sample",
    "source-policy",
    "source-snapshot",
    "status",
    "task-assignment",
    "task-bundle",
    "task-receipt",
    "task-staging-request",
    "telemetry-batch",
    "test-observation",
    "test-plan",
    "test-proposal",
    "tool-call",
    "trace-record",
    "v61-artifact-policy",
    "v61-boundary-observation",
    "v61-candidate-batch",
    "v61-curriculum",
    "v61-custody-decision",
    "v61-custody-receipt",
    "v61-custody-request",
    "v61-denial-audit",
    "v61-embargo-rule",
    "v61-experiment-intent",
    "v61-experiment-spec",
    "v61-feedback-record",
    "v61-holder-capability",
    "v61-knowledge-release",
    "v61-license-grant",
    "v61-offline-submission",
    "v61-offline-task",
    "v61-physical-allocation",
    "v61-release-compatibility",
    "v61-repository-admission",
    "v61-research-track",
    "v61-research-view",
    "v61-storage-lease",
    "v61-study-assignment",
    "v61-study-outcome",
    "v61-task-context-plan",
    "v61-temporal-fact",
    "v61-trade-restriction",
    "v61-worker-neutral-receipt",
    "worker-claim-request",
    "worker-complete-request",
    "worker-heartbeat",
    "worker-status",
    ];

    /// Whether a named field is required.
    #[must_use]
    pub fn requires(&self, name: &str) -> bool {
        self.required.iter().any(|field| *field == name)
    }
}
