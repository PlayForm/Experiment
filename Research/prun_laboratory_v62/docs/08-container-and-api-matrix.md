# Container provisioning, communication and additive internal APIs

| Container                  | Canonical mapping?                | Data keys?                         | CA/signing keys?                         | Observable data                     | Network                        | Input/output                                |
| -------------------------- | --------------------------------- | ---------------------------------- | ---------------------------------------- | ----------------------------------- | ------------------------------ | ------------------------------------------- |
| `schema-factory`           | Authored IR and resolver          | No DEKs                            | No root CA                               | Build/policy metadata               | Trusted IPC only               | Signed IR → compiled bundles                |
| `opaque-resolver`          | Scoped crosswalk in memory        | No                                 | No                                       | Resolve attempt metadata            | Trusted IPC only               | Opaque handle → authorized internal handle  |
| `key-broker`               | Minimal key-handle registry       | Yes, nonexportable where possible  | Only scoped online key                   | Key operation audit                 | Trusted IPC only               | Authorized operation → ciphertext/signature |
| `identity-issuer`          | Identity policy only              | No artifact DEKs                   | Online intermediate CA only              | Issuance audit                      | Trusted IPC                    | Attestation → short-lived workload identity |
| `policy-engine`            | Semantic entitlement IDs only     | No                                 | No                                       | Allow/deny + policy digest          | Trusted IPC                    | Verified facts → decision                   |
| `journal`                  | Event private IDs in cipher       | Encrypted blobs; no plaintext DEKs | Scoped journal signer                    | Restricted audit only               | Trusted IPC                    | Append/replay encrypted events              |
| `telemetry-control`        | No private crosswalk              | No                                 | No                                       | Plane 1 metadata                    | Ingest from trusted senders    | Redacted spans/counters                     |
| `telemetry-product`        | No private crosswalk              | No                                 | No                                       | Plane 2 profile/benchmark metadata  | Trusted ingest                 | Redacted measurements                       |
| `operator-gateway`         | Authorized view-only resolver     | No                                 | No                                       | Plane 0 projections                 | Operator mTLS                  | Human/GPUI/CLI read and permitted commands  |
| `offline-supervisor`       | Task-to-source mapping only       | No lab master keys                 | No                                       | Process state, sealed task receipts | Trusted IPC; worker file mount | Scoped task files ↔ sealed output           |
| `llm-worker`               | Never                             | Never                              | Never                                    | None                                | `--network none`               | Rust source + task + scratch only           |
| `builder/oracle/benchmark` | Artifact and protocol scoped refs | No                                 | Scoped evidence signer isolated from SUT | Product correctness and performance | Separately restricted          | Immutable inputs → attested evidence        |

These are distinct authority boundaries, not a requirement for one Docker
container per row. Running all in one Linux host with a privileged operator is
not equivalent to independent hardware security domains.

## Additive control operations (trusted Rust IPC / tonic, not worker HTTP)

- `Registry.PrepareDefinition(spec, expected_epoch) → staging_handle`: validates
  signed canonical IR and reference closure, no activation.
- `Registry.ActivateDefinition(staging_handle, expected_head) → activation_receipt`:
  atomic, signed state transition.
- `Resolver.Resolve(opaque_handle, caller_identity, purpose, proof) → scoped_internal_handle`:
  policy-enforced and audited.
- `Pseudonym.Issue(canonical_handle, audience, epoch, purpose) → opaque_token`:
  only after authorized disclosure; uniqueness enforced.
- `Capability.Issue(subject, resource_handle, action, purpose, proof_key, expiry) → signed_grant`:
  never for LLM worker control operations.
- `Capability.Check(grant, proof, context, nonce) → allow/deny`: fail closed,
  fresh and non-replayable.
- `Custody.Authorize(artifact, holder, purpose, grant) → reservation`: checks
  holder capability, license, embargo, quotas and key scope.
- `KeyBroker.Seal(recipient_key_handle, context_digest, plaintext) → ciphertext`:
  returns opaque ciphertext, no key export.
- `KeyBroker.Open(requester_proof, ciphertext, context_digest) → plaintext`:
  restricted to a preauthorized recipient endpoint; never returns to
  orchestration/telemetry.
- `Journal.Append(event_ciphertext, signed_producer_receipt, expected_prev_head) → signed_commitment`:
  compare-and-swap chain head.
- `Migration.Preview(signed_migration, expected_epoch) → compatibility_report`:
  dry run and negative checks.
- `Migration.Commit(staging_handle, expected_head) → new_epoch`: atomic after
  policy and compatibility.
- `Audit.Project(viewer, scope, purpose, time_range) → redacted_records`:
  mapping requests themselves audited.

All are proposed, **not implemented**. Transport mTLS identity, replay fencing,
message signatures, authorization, idempotency, and rate limits apply to each.
Their full private request types are represented by canonical IR, with a fixed
opaque encrypted envelope on the wire. The v6.1 external operator API remains
version-compatible; LLM workers cannot call these methods.
