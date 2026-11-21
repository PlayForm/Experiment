# Plane 0: external monitoring, GPUI and owner commands

The external interface is the **only** API a human or GPUI application needs. It
intentionally does not expose queue mutation, worker shell access, benchmark
result submission, arbitrary agent messaging, certificate issuance or raw
private conversation content.

- Transport: private loopback/Unix socket by default; when remote, mTLS with a
  human/client identity and transport encryption.
- Auth: read-only `observer`; owner/operator commands are separately authorized,
  audited and idempotent. Never forward these credentials to model workers.
- Consistency: API reads materialized projections at an explicit `snapshot_seq`.
  The signed event journal wins over read models. `GET /v1/events/stream` is
  resumable by sequence, not wall-clock timestamps.
- Data access: only metadata by default. Knowledge of an artifact hash does
  **not** grant access to its bytes.
- Paging: `cursor` is opaque; `limit` at most 500; returning a page snapshot
  doesn't imply the next page is from the same live state unless pinned by
  `snapshot_seq`.
- Failure: normalized `api-error` shape; no raw internal traceback or prompt
  content. Expected statuses: 400, 401, 403, 404, 409, 412, 429, 503.
  `request_id` is traceable in the control plane.
- Owner writes: a `command` is a declarative request validated against a pinned
  signed policy. It never executes arbitrary shell, code, or an untrusted agent
  instruction. Exactly two POSTs are exposed: `operator/commands` and policy
  validation. All state changes still occur in the deterministic coordinator.
- Streaming: SSE events contain only authorized ledger metadata, not agent
  prompts or raw source. Resume using `after_seq`; heartbeat and backpressure
  are explicit in the implementation. Overflow returns a cursor requiring
  paginated recovery.

## Canonical operation inventory

| Method | Path                              | Operation                  | Body schema                 | Response schema                      | Authorized roles |
| ------ | --------------------------------- | -------------------------- | --------------------------- | ------------------------------------ | ---------------- |
| `GET`  | `/health/live`                    | `getLiveness`              | `—`                         | `status.schema.json`                 | observer         |
| `GET`  | `/health/ready`                   | `getReadiness`             | `—`                         | `status.schema.json`                 | observer         |
| `GET`  | `/v1/agents/conversations`        | `listConversationMetadata` | `—`                         | `page-agent-turn.schema.json`        | observer         |
| `GET`  | `/v1/agents/conversations/{id}`   | `getConversationMetadata`  | `—`                         | `artifact-manifest.schema.json`      | observer         |
| `GET`  | `/v1/agents/tool-calls`           | `listToolCallMetadata`     | `—`                         | `page-tool-call.schema.json`         | observer         |
| `GET`  | `/v1/agents/tool-calls/{id}`      | `getToolCall`              | `—`                         | `tool-call.schema.json`              | observer         |
| `GET`  | `/v1/artifacts/{digest}/metadata` | `getArtifactMetadata`      | `—`                         | `artifact-ref.schema.json`           | observer         |
| `GET`  | `/v1/attestations`                | `listAttestations`         | `—`                         | `page-attestation.schema.json`       | observer         |
| `GET`  | `/v1/attestations/{id}`           | `getAttestationsById`      | `—`                         | `attestation.schema.json`            | observer         |
| `GET`  | `/v1/audit/events`                | `listAuditEvents`          | `—`                         | `page-audit-event.schema.json`       | observer         |
| `GET`  | `/v1/audit/events/{id}`           | `getAuditEventsById`       | `—`                         | `audit-event.schema.json`            | observer         |
| `GET`  | `/v1/benchmarks`                  | `listBenchmarks`           | `—`                         | `page-benchmark-result.schema.json`  | observer         |
| `GET`  | `/v1/benchmarks/{id}`             | `getBenchmarksById`        | `—`                         | `benchmark-result.schema.json`       | observer         |
| `GET`  | `/v1/campaigns`                   | `listCampaigns`            | `—`                         | `page-campaign.schema.json`          | observer         |
| `GET`  | `/v1/campaigns/{id}`              | `getCampaignsById`         | `—`                         | `campaign.schema.json`               | observer         |
| `GET`  | `/v1/certificates`                | `listCertificates`         | `—`                         | `page-certificate-event.schema.json` | observer         |
| `GET`  | `/v1/certificates/{id}`           | `getCertificatesById`      | `—`                         | `certificate-event.schema.json`      | observer         |
| `GET`  | `/v1/events`                      | `getEventHistory`          | `—`                         | `page-audit-event.schema.json`       | observer         |
| `GET`  | `/v1/events/stream`               | `streamEvents`             | `—`                         | `text/event-stream`                  | observer         |
| `GET`  | `/v1/experiments`                 | `listExperiments`          | `—`                         | `page-experiment.schema.json`        | observer         |
| `GET`  | `/v1/experiments/{id}`            | `getExperimentsById`       | `—`                         | `experiment.schema.json`             | observer         |
| `GET`  | `/v1/experiments/{id}/samples`    | `getExperimentSamples`     | `—`                         | `page-sample.schema.json`            | observer         |
| `GET`  | `/v1/hosts`                       | `listHosts`                | `—`                         | `page-host-status.schema.json`       | observer         |
| `GET`  | `/v1/hosts/{id}`                  | `getHostsById`             | `—`                         | `host-status.schema.json`            | observer         |
| `GET`  | `/v1/lineage/{id}`                | `getLineage`               | `—`                         | `lineage.schema.json`                | observer         |
| `GET`  | `/v1/mutations`                   | `listMutations`            | `—`                         | `page-mutation.schema.json`          | observer         |
| `GET`  | `/v1/mutations/{id}`              | `getMutationsById`         | `—`                         | `mutation.schema.json`               | observer         |
| `GET`  | `/v1/notifications`               | `listNotifications`        | `—`                         | `page-notification.schema.json`      | observer         |
| `GET`  | `/v1/notifications/{id}`          | `getNotification`          | `—`                         | `notification.schema.json`           | observer         |
| `POST` | `/v1/operator/commands`           | `submitOwnerCommand`       | `command.schema.json`       | `ack.schema.json`                    | owner, operator  |
| `POST` | `/v1/operator/policies/validate`  | `validatePolicyCandidate`  | `source-policy.schema.json` | `ack.schema.json`                    | owner, operator  |
| `GET`  | `/v1/policies/{id}`               | `getPolicy`                | `—`                         | `source-policy.schema.json`          | observer         |
| `GET`  | `/v1/research/cases`              | `listResearchCases`        | `—`                         | `page-research-case.schema.json`     | observer         |
| `GET`  | `/v1/research/cases/{id}`         | `getResearchCasesById`     | `—`                         | `research-case.schema.json`          | observer         |
| `GET`  | `/v1/research/papers`             | `listResearchPapers`       | `—`                         | `page-research-paper.schema.json`    | observer         |
| `GET`  | `/v1/research/papers/{id}`        | `getResearchPapersById`    | `—`                         | `research-paper.schema.json`         | observer         |
| `GET`  | `/v1/status`                      | `getStatus`                | `—`                         | `status.schema.json`                 | observer         |
| `GET`  | `/v1/telemetry/control/metrics`   | `listControlMetrics`       | `—`                         | `page-metric-record.schema.json`     | observer         |
| `GET`  | `/v1/telemetry/control/traces`    | `listControlTraces`        | `—`                         | `page-trace-record.schema.json`      | observer         |
| `GET`  | `/v1/telemetry/product/metrics`   | `listProductMetrics`       | `—`                         | `page-metric-record.schema.json`     | observer         |
| `GET`  | `/v1/telemetry/product/traces`    | `listProductTraces`        | `—`                         | `page-trace-record.schema.json`      | observer         |
| `GET`  | `/v1/workers`                     | `listWorkers`              | `—`                         | `page-worker-status.schema.json`     | observer         |
| `GET`  | `/v1/workers/{id}`                | `getWorkersById`           | `—`                         | `worker-status.schema.json`          | observer         |

## Example monitoring session

```http
GET /v1/status HTTP/1.1
Host: lab-ui.lab.internal:9443
Accept: application/json
```

```http
GET /v1/events/stream?after_seq=10028 HTTP/1.1
Host: lab-ui.lab.internal:9443
Accept: text/event-stream
```

The display should indicate last received `event_id`, ledger sequence, whether
each telemetry plane is healthy, and whether the user is viewing a stale
projection. `200` responses with a degraded telemetry plane are permitted;
experiment evidence must remain independently verifiable.

## Intentional exclusions

No API accepts a model-generated internal command or raw benchmark score. No
observer endpoint returns arbitrary decrypted agent data. There is no endpoint
for direct Docker operations, CA signing, live model socket proxying, or
modification of trusted tests. Those functions live in trusted backend
implementations and purpose-specific policy-controlled service paths.
