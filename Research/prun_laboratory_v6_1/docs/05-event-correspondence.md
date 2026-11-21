# Every correspondence channel and event type

The complete authorized correspondence network is finite, not a generic
agent-to-agent chat bus. Every transition has exactly one typed
producer/consumer relationship, authorization scope, retention classification,
and append-only receipt. The trusted parent and all children have no chat
sockets to one another.

## Full correspondence inventory

| Sender                             | Recipient               | Medium                      | Schema / object                      | Guarantee                                                                    |
| ---------------------------------- | ----------------------- | --------------------------- | ------------------------------------ | ---------------------------------------------------------------------------- |
| `Owner`                            | `Public API`            | `mTLS HTTP`                 | `command`                            | Operator-authenticated command document; only declarative allowed operations |
| `Public API`                       | `Coordinator`           | `mTLS HTTP`                 | `command`                            | Validated owner command; no arbitrary shell                                  |
| `Coordinator`                      | `Event ledger`          | `transactional append`      | `audit-event`                        | Signed state transition and hash chain                                       |
| `Ledger`                           | `Projection service`    | `authenticated outbox`      | `audit-event`                        | At-least-once delivery; idempotent projection                                |
| `Projection service`               | `GPUI`                  | `mTLS HTTP/SSE`             | `page/status/audit-event`            | Metadata-only public authorized view                                         |
| `Coordinator`                      | `Supervisor`            | `mTLS claim`                | `task-bundle`                        | Scoped work order; model is never API recipient                              |
| `Supervisor`                       | `LLM worker`            | `read-only file mailbox`    | `task-bundle (filtered)`             | Only source/task/content; no topology, IDs can be opaque                     |
| `LLM worker`                       | `Supervisor`            | `scratch output directory`  | `patch/research-claim/test-proposal` | Untrusted data only; cannot run trusted control calls                        |
| `Supervisor`                       | `Conversation CAS`      | `encrypted artifact commit` | `agent-turn/tool-call`               | Visible turn transcript, local tool args/results stored encrypted            |
| `Supervisor`                       | `Ingestion gateway`     | `mTLS HTTP`                 | `task-receipt/patch-proposal`        | Hash checks, scope checks, size checks                                       |
| `Ingestion gateway`                | `Novelty gate`          | `trusted internal dispatch` | `patch-proposal`                     | Exact identity, structural similarity, feature conflicts                     |
| `Coordinator`                      | `Builder`               | `mTLS HTTP`                 | `build-request`                      | Immutable source/toolchain/dependency closure                                |
| `Builder`                          | `Artifact CAS`          | `mTLS HTTP`                 | `build-result`                       | Executable digest and build log artifacts                                    |
| `Coordinator`                      | `Oracle`                | `mTLS HTTP`                 | `oracle-request`                     | Independent frozen test suite                                                |
| `Oracle`                           | `Event ledger`          | `mTLS HTTP`                 | `oracle-result`                      | Tests/counterexamples attested by oracle                                     |
| `Coordinator`                      | `Benchmark scheduler`   | `trusted dispatch`          | `benchmark-request`                  | Frozen comparative protocol                                                  |
| `Scheduler`                        | `Benchmark host`        | `fenced lease`              | `lease`                              | Exclusive CPU and host resource reservation                                  |
| `Benchmark runner`                 | `Event ledger`          | `mTLS HTTP`                 | `benchmark-result`                   | Raw paired samples and environment attestations                              |
| `Evaluator`                        | `Event ledger`          | `mTLS HTTP`                 | `evaluation-result`                  | Predeclared statistics; no model verdict authority                           |
| `Feedback authority`               | `Supervisor`            | `mTLS HTTP/CAS`             | `feedback-capsule`                   | Scoped results, not holdout internals                                        |
| `Supervisor`                       | `LLM worker`            | `read-only file mailbox`    | `feedback-capsule (filtered)`        | Next iteration context supplied offline                                      |
| `Research workers`                 | `Supervisor`            | `scratch output directory`  | `research-claim`                     | Private authored opinion; no result authority                                |
| `Publisher`                        | `Publication authority` | `mTLS HTTP`                 | `research-paper`                     | Sanitized derivative, evidence map and release policy                        |
| `Publication authority`            | `Evidence ledger`       | `mTLS HTTP`                 | `attestation`                        | Release decision and signature                                               |
| `Integrator`                       | `Supervisor`            | `file mailbox`              | `research-case`                      | Transfer hypothesis only, not automatic promotion                            |
| `PKI issuer`                       | `Evidence ledger`       | `mTLS HTTP`                 | `certificate-event`                  | Issuance/renewal/revocation counts and reasons                               |
| `Trusted services`                 | `Control OTLP`          | `OTLP/gRPC mTLS`            | `trace-record/metric-record`         | Metadata, no content                                                         |
| `SUT diagnostic binary`            | `Product OTLP`          | `OTLP/gRPC mTLS`            | `trace-record/metric-record`         | Diagnostic mode only                                                         |
| `Control/Product telemetry stores` | `Projection service`    | `trusted read`              | `trace/metric projections`           | Correlated by IDs only, no storage authority                                 |

## Canonical event vocabulary (57 events)

The content schema shown is the canonical content-addressed event payload; the
live wire message carries `audit-event` metadata and a hash reference. The event
ledger is append-only, signed, ordered by `global_seq`, and separate from
best-effort OTLP spans.

| Event kind                     | Domain     | Authorized producer   | Payload schema                             |
| ------------------------------ | ---------- | --------------------- | ------------------------------------------ |
| `CampaignCreated`              | campaign   | coordinator           | `../schemas/campaign.schema.json`          |
| `CampaignValidated`            | campaign   | policy-service        | `../schemas/campaign.schema.json`          |
| `CampaignActivated`            | campaign   | coordinator           | `../schemas/campaign.schema.json`          |
| `CampaignPaused`               | campaign   | coordinator           | `../schemas/campaign.schema.json`          |
| `CampaignDrained`              | campaign   | coordinator           | `../schemas/campaign.schema.json`          |
| `CampaignCompleted`            | campaign   | coordinator           | `../schemas/campaign.schema.json`          |
| `CampaignFailed`               | campaign   | coordinator           | `../schemas/campaign.schema.json`          |
| `SourceSnapshotCaptured`       | source     | source-adapter        | `../schemas/source-snapshot.schema.json`   |
| `SourceScopeValidated`         | source     | scope-validator       | `../schemas/source-snapshot.schema.json`   |
| `DependencyClosurePinned`      | source     | source-adapter        | `../schemas/artifact-manifest.schema.json` |
| `ResearchCaseOpened`           | research   | coordinator           | `../schemas/research-case.schema.json`     |
| `BlindAssignmentCreated`       | research   | coordinator           | `../schemas/task-assignment.schema.json`   |
| `ResearchClaimSubmitted`       | research   | supervisor            | `../schemas/research-claim.schema.json`    |
| `ResearchClaimRecorded`        | research   | knowledge-registry    | `../schemas/research-claim.schema.json`    |
| `ResearchDisagreementRecorded` | research   | knowledge-registry    | `../schemas/research-claim.schema.json`    |
| `PaperStaged`                  | research   | publication-service   | `../schemas/research-paper.schema.json`    |
| `PaperSanitized`               | research   | sanitizer             | `../schemas/research-paper.schema.json`    |
| `PaperReleased`                | research   | publication-authority | `../schemas/research-paper.schema.json`    |
| `PaperRetracted`               | research   | publication-authority | `../schemas/research-paper.schema.json`    |
| `TransferHypothesisOpened`     | research   | coordinator           | `../schemas/research-case.schema.json`     |
| `TaskStaged`                   | worker     | coordinator           | `../schemas/task-bundle.schema.json`       |
| `WorkerLeaseGranted`           | worker     | scheduler             | `../schemas/lease.schema.json`             |
| `WorkerStarted`                | worker     | supervisor            | `../schemas/worker-status.schema.json`     |
| `ModelTurnSealed`              | worker     | supervisor            | `../schemas/agent-turn.schema.json`        |
| `LocalToolCallSealed`          | worker     | supervisor            | `../schemas/tool-call.schema.json`         |
| `WorkerOutputStaged`           | worker     | supervisor            | `../schemas/task-receipt.schema.json`      |
| `WorkerTimedOut`               | worker     | supervisor            | `../schemas/worker-status.schema.json`     |
| `WorkerQuarantined`            | worker     | supervisor            | `../schemas/worker-status.schema.json`     |
| `FeedbackCapsulePrepared`      | worker     | feedback-authority    | `../schemas/feedback-capsule.schema.json`  |
| `PatchSubmitted`               | mutation   | supervisor            | `../schemas/patch-proposal.schema.json`    |
| `PatchRejectedAsDuplicate`     | mutation   | novelty-gate          | `../schemas/patch-proposal.schema.json`    |
| `PatchScopeRejected`           | mutation   | scope-validator       | `../schemas/patch-proposal.schema.json`    |
| `MutationAdmitted`             | mutation   | coordinator           | `../schemas/mutation.schema.json`          |
| `MutationRejected`             | mutation   | coordinator           | `../schemas/mutation.schema.json`          |
| `MutationPromoted`             | mutation   | release-gate          | `../schemas/mutation.schema.json`          |
| `BuildRequested`               | build      | coordinator           | `../schemas/build-request.schema.json`     |
| `BuildCompleted`               | build      | builder               | `../schemas/build-result.schema.json`      |
| `BuildFailed`                  | build      | builder               | `../schemas/build-result.schema.json`      |
| `OracleRequested`              | oracle     | coordinator           | `../schemas/oracle-request.schema.json`    |
| `OraclePassed`                 | oracle     | oracle                | `../schemas/oracle-result.schema.json`     |
| `OracleFailed`                 | oracle     | oracle                | `../schemas/oracle-result.schema.json`     |
| `OracleInconclusive`           | oracle     | oracle                | `../schemas/oracle-result.schema.json`     |
| `BenchmarkLeaseGranted`        | benchmark  | scheduler             | `../schemas/lease.schema.json`             |
| `BenchmarkRequested`           | benchmark  | coordinator           | `../schemas/benchmark-request.schema.json` |
| `BenchmarkCompleted`           | benchmark  | runner                | `../schemas/benchmark-result.schema.json`  |
| `BenchmarkContaminated`        | benchmark  | runner                | `../schemas/benchmark-result.schema.json`  |
| `BenchmarkLeaseExpired`        | benchmark  | scheduler             | `../schemas/lease.schema.json`             |
| `EvaluationCompleted`          | benchmark  | evaluator             | `../schemas/evaluation-result.schema.json` |
| `CredentialIssued`             | security   | pki                   | `../schemas/certificate-event.schema.json` |
| `CredentialRevoked`            | security   | pki                   | `../schemas/certificate-event.schema.json` |
| `AttestationRecorded`          | security   | attestation-service   | `../schemas/attestation.schema.json`       |
| `PolicyValidated`              | security   | policy-service        | `../schemas/source-policy.schema.json`     |
| `PolicyRejected`               | security   | policy-service        | `../schemas/source-policy.schema.json`     |
| `HostQuarantined`              | operations | host-verifier         | `../schemas/host-status.schema.json`       |
| `ArtifactCommitted`            | operations | cas                   | `../schemas/artifact-ref.schema.json`      |
| `TelemetryPipelineDegraded`    | operations | telemetry-gateway     | `../schemas/notification.schema.json`      |
| `EventReplayCompleted`         | operations | coordinator           | `../schemas/audit-event.schema.json`       |

## Private conversation record sequence

1. Supervisor generates or resumes a `task-bundle` with a task-scoped policy
   capsule and source manifest; only selected material is mounted into offline
   model container.
2. Trusted inference adapter supplies locally hosted model messages. The model
   sees a task pre-prompt and source snippets; not a ledger, routing address,
   tracing header, role token or global campaign name unless strictly necessary.
3. For every visible message, record `agent-turn` with context snapshot hash,
   model cohort, prompt template hash and ciphertext artifact reference. Never
   claim access to private internal reasoning tokens not explicitly emitted.
4. For every allowed local tool call, record `tool-call` with an allowlisted
   tool name, arguments digest, result artifact, exit code and policy check
   receipt. The worker may invoke sandboxed local tools, not network tools.
5. On interruption, the supervisor appends a `segment_id` boundary and later
   resumes from explicit authorized context snapshots. A compressed summary is
   an untrusted derived artifact, not a rewrite of the old transcript.
6. The supervisor seals a `task-receipt` and stages a patch/claim/test/document.
   Downstream trusted services inspect and validate those _artifacts_; the model
   has no channel to them.
7. Feedback enters only as a filtered `feedback-capsule`; exact sealed holdout
   details, siblings' private opinions and infrastructure information are
   excluded.

## Encryption and what an observer can see

The default API can show task lifecycle, role, count of messages, model cohort
hash, timestamp, ciphertext digest, test status and benchmark statistics. It
cannot show plaintext turns. A privileged _named decryptor_, distinct from a
dashboard observer, may access authorized encrypted artifact bytes through a
separately audited grant. Host root may still access plaintext during execution
unless isolation extends to hardware/operational controls.

The signed envelope **does not** independently prove that the result is true;
its signature and expiry only bind sender, recipient, nonce, ciphertext digest,
scope and receipt. The experiment authority must sign the result of actual
execution.
