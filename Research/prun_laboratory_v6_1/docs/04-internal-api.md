# Plane 1 and Plane 2: trusted internal API

This OpenAPI contract describes precisely the authenticated **software-only**
API calls. These routes are not deployed in model/container namespaces, are not
mentioned in model task capsules, and do not accept model identities. mTLS
identity is mapped to an allowlisted **service role** _before_ parsing or acting
on payloads.

- Every mutating POST carries a stable request ID or idempotency key; replay
  returns original receipt or a conflict, not a second effect.
- An event append uses `expected_aggregate_seq` and `expected_last_hash` to
  guarantee compare-and-swap sequencing. Multi-writer ledger must serialize
  appends.
- Every lease heartbeat/report contains a fencing token where applicable; stale
  token reports cannot commit evidence. Other report types bind to the lease in
  their subject attestations even if the transport request omits the token.
- Claims, builds, oracles, benchmarks and evaluations are signed by separate
  authorized services. Identity scope, policy digest, source digest, protocol,
  fixture and hardware evidence are checked before accepting a report.
- Internal telemetry ingestion endpoints must run as separate adapters/gateways,
  even though OpenTelemetry-native OTLP is the eventual transport. Product
  diagnostic events never share the control gateway or database trust identity.
- No arbitrary JSON passthrough or generic `execute` endpoint. Exactly the
  versioned operations below may be implemented.

## Complete v6 internal operation inventory

| Method | Path                                    | Operation                | Body schema                             | Response schema   | Authorized roles                                            |
| ------ | --------------------------------------- | ------------------------ | --------------------------------------- | ----------------- | ----------------------------------------------------------- |
| `POST` | `/internal/v1/artifacts/commit`         | `commitArtifact`         | `artifact-commit-request.schema.json`   | `ack.schema.json` | supervisor, builder, oracle, runner, publisher, coordinator |
| `POST` | `/internal/v1/attestations/record`      | `recordAttestation`      | `attestation.schema.json`               | `ack.schema.json` | attestation-service                                         |
| `POST` | `/internal/v1/benchmark/lease`          | `reserveBenchHost`       | `benchmark-lease-request.schema.json`   | `ack.schema.json` | scheduler                                                   |
| `POST` | `/internal/v1/benchmark/request`        | `requestBenchmark`       | `benchmark-request.schema.json`         | `ack.schema.json` | coordinator                                                 |
| `POST` | `/internal/v1/benchmark/result`         | `reportBenchmark`        | `benchmark-result.schema.json`          | `ack.schema.json` | runner                                                      |
| `POST` | `/internal/v1/build/request`            | `requestBuild`           | `build-request.schema.json`             | `ack.schema.json` | coordinator                                                 |
| `POST` | `/internal/v1/build/result`             | `reportBuild`            | `build-result.schema.json`              | `ack.schema.json` | builder                                                     |
| `POST` | `/internal/v1/evaluation/result`        | `reportEvaluation`       | `evaluation-result.schema.json`         | `ack.schema.json` | evaluator                                                   |
| `POST` | `/internal/v1/feedback/prepare`         | `prepareFeedback`        | `feedback-capsule.schema.json`          | `ack.schema.json` | feedback-authority                                          |
| `POST` | `/internal/v1/identity/issue`           | `issueCredential`        | `identity-issuance-request.schema.json` | `ack.schema.json` | identity-authority                                          |
| `POST` | `/internal/v1/identity/revoke`          | `revokeCredential`       | `identity-revoke-request.schema.json`   | `ack.schema.json` | identity-authority                                          |
| `POST` | `/internal/v1/ledger/append`            | `appendLedgerEvent`      | `event-append-request.schema.json`      | `ack.schema.json` | coordinator, ledger-gateway                                 |
| `POST` | `/internal/v1/oracle/request`           | `requestOracle`          | `oracle-request.schema.json`            | `ack.schema.json` | coordinator                                                 |
| `POST` | `/internal/v1/oracle/result`            | `reportOracle`           | `oracle-result.schema.json`             | `ack.schema.json` | oracle                                                      |
| `POST` | `/internal/v1/proposals/claim`          | `ingestResearchClaim`    | `research-claim.schema.json`            | `ack.schema.json` | supervisor                                                  |
| `POST` | `/internal/v1/proposals/patch`          | `ingestPatch`            | `patch-proposal.schema.json`            | `ack.schema.json` | supervisor                                                  |
| `POST` | `/internal/v1/proposals/test`           | `ingestTestProposal`     | `test-proposal.schema.json`             | `ack.schema.json` | supervisor                                                  |
| `POST` | `/internal/v1/publication/attest`       | `publishAttestation`     | `attestation.schema.json`               | `ack.schema.json` | publication-authority                                       |
| `POST` | `/internal/v1/research/paper`           | `stagePaper`             | `paper-stage-request.schema.json`       | `ack.schema.json` | publisher                                                   |
| `POST` | `/internal/v1/source/snapshot`          | `recordSourceSnapshot`   | `source-snapshot.schema.json`           | `ack.schema.json` | source-adapter                                              |
| `POST` | `/internal/v1/task/stage`               | `stageScopedTask`        | `task-staging-request.schema.json`      | `ack.schema.json` | coordinator                                                 |
| `POST` | `/internal/v1/telemetry/control/ingest` | `ingestControlTelemetry` | `telemetry-batch.schema.json`           | `ack.schema.json` | otel-control-gateway                                        |
| `POST` | `/internal/v1/telemetry/product/ingest` | `ingestProductTelemetry` | `telemetry-batch.schema.json`           | `ack.schema.json` | otel-product-gateway                                        |
| `POST` | `/internal/v1/worker/claim`             | `claimWorkerTask`        | `worker-claim-request.schema.json`      | `ack.schema.json` | supervisor                                                  |
| `POST` | `/internal/v1/worker/complete`          | `completeWorkerTask`     | `worker-complete-request.schema.json`   | `ack.schema.json` | supervisor                                                  |
| `POST` | `/internal/v1/worker/heartbeat`         | `heartbeatWorkerLease`   | `worker-heartbeat.schema.json`          | `ack.schema.json` | supervisor                                                  |

## API call lifecycle and expected receipts

`Claim`: the scheduler returns a task only when the trusted supervisor's
capability/host cohort permits it. A claim is a lease, not a model-initiated
request. The task bundle enters an offline mailbox, and the model receives only
file mounts.

`Complete`: supervisor seals the scratch output, computes hashes, captures
visible transcript/tool metadata, verifies file quotas and patch scope, and
submits a `task-receipt` before any ingestion. The model itself has no API
route.

`Build`: coordinator submits a `build-request`; builder returns a `build-result`
including exact toolchain/dependency closure/container digests. Trusted source
scope checked before processing. Build errors may lead to new task feedback but
never change the oracle's reference suite.

`Oracle`: trusted oracle runs against pinned reference and changeset-specific
protocols; `oracle-result` is not editable by a patcher. A model-authored test
enters only as `test-proposal` and cannot be the sole acceptance evidence.

`Benchmark`: scheduler reserves a physically isolated machine/cpu set; runner
executes frozen paired measurements, emits raw samples and signs the result. A
development run and a sealed confirmation run use separately authorized
policies.

`Evaluation`: deterministic evaluator applies a pinned statistical protocol and
emits verdicts. LLM prose may explain results but cannot sign them. `Feedback`:
software filters evidence to the exact assigned worker view and never sends
sealed holdout samples.

`Identity`: short-lived TLS/SSH/message certs issued by restricted authority
only. DNS discovery cannot make a service authorized. `Attestation`: validators
check issuer scope, subject digest, expiry and signature and append the receipt.

## Wire error/idempotency contract

- `400 BAD_REQUEST`: schema invalid; no side effect.
- `401 UNAUTHORIZED` / `403 FORBIDDEN`: missing credentials or role scope; no
  payload execution.
- `409 CONFLICT`: idempotency key reused with different request body / revision
  conflict.
- `412 PRECONDITION_FAILED`: aggregate revision or expected digest mismatch.
- `409 FENCING_REJECTED` logical code: expired lease or old fencing token.
- `503 UNAVAILABLE`: infrastructure unready; caller may retry with the **same**
  idempotency key.

A well-formed signed worker research claim may be recorded as an opinion; it
cannot be passed to an authority-grade `reportBenchmark` call, regardless of
content.
