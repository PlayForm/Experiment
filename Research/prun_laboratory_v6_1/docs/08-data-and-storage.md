# Storage, lineage, retention, correspondence, schemas

## Four authoritative layers and two rebuildable indexes

1. **Canonical event journal**: append-only signed `audit-event`, per-aggregate
   sequence and a global sequence; cryptographic hash chain across recorded
   events; explicit signature and credential trace; integrity verification on
   startup and checkpoint.
2. **Content-addressed file store**: source snapshots, source diffs, build
   artifacts, benchmark samples, tests, evidence, citations, private
   conversation transcripts, encrypted envelopes, signed attestations; bytes
   verified before commitment. Encryption classification is retained per
   artifact.
3. **Signed evidence registry**: subject digest, predicate digest, issuer scope,
   credential serial, policy digest, creation/expiry and attestation chain. A
   measurement attestation needs a trusted runner identity and complete
   experimental identity.
4. **Owner-governed policy/credential ledger**: immutable issuance/revocation
   records, effective policy version and owner commands.
5. **SQLite projections** (rebuildable): work queues, status, lineages,
   experiment views and dashboard indexes. Reference v2-v5 tables remain; v6
   adds namespaced protocol/correspondence tables.
6. **OTLP time-series/tracing stores** (rebuildable/observational): control and
   product separately; telemetry is not canonical experiment evidence.

## Artifact logical namespace and required metadata

`cas://sha256/<digest>` identifies bytes, not a grant. Use storage class
`sealed/private/internal/public`, recipient-key envelope and a cryptographic
commitment. Every reference includes media type, byte count, creation time and
authorized audience. The SHA digest alone must not authenticate a recipient.

Large raw files can be chunked into Merkle-addressed manifests with a verified
root. Compression, encryption and content hashing order must be documented per
representation: for example hash canonical plaintext inside an authorized
enclave, encrypt independently, then also hash ciphertext for transport
verification. Never expose private plaintext hashes to unauthorized observers if
they are vulnerable to dictionary attacks.

## Mandatory correspondence records

Every _visible_ model turn (`agent-turn`) stores model fingerprint, prompt
template digest, scoped context digest, task, conversation/segment/turn IDs,
authorized ciphertext artifact, local tool call references and timestamps. Every
tool call (`tool-call`) stores an allowlisted operation, private arguments
digest, result/counterexample digest, exit code and trusted policy receipt.
Every resume creates new segment ID and a new explicit context snapshot. No
hidden chain-of-thought recording claims.

Private transcripts are written by a trusted local supervisor to encrypted CAS;
the public API returns only approved metadata. Agents never obtain CAS APIs, CAS
credentials or a route to the storage service.

## Retention rules

- Event and signature logs: append-only until a governed archive/delete
  migration; no silent mutation of historic approvals.
- Measurement raw samples and replay inputs: retain as long as scientific claims
  are published; migration must preserve digest and readability.
- Prompt transcripts and private research: encrypted with named recipient
  access; retention/expiry policy configurable; avoid claiming confidentiality
  from host root.
- Diagnostic traces: bounded retention and lower priority than evidence, with
  explicit sampling.
- Release binaries/lineage: retain reproducible build context and source
  licenses.
- Public papers: may be corrected/retracted via superseding signed records; no
  silent overwrites.

## Database schema

`sql/v6_migration.sql` is additive to the earlier `v2/sql/lineage.sql`,
`v3/sql/v3_migration.sql`, `v5/sql/v5_migration.sql`. V6 creates
`v6_event_journal`, `v6_command_inbox`, `v6_projection_outbox`,
`v6_record_index`, `v6_artifact_grants`, `v6_correspondence`, `v6_tool_calls`,
`v6_task_mailboxes`, `v6_service_credentials`, `v6_credential_events`,
`v6_benchmark_leases`, `v6_telemetry_refs`, `v6_confirmation_access`,
`v6_service_outbox`. Immutable journal/conversation triggers reject
update/delete. Transactional authorization, balance enforcement and
hash/signature verification still require Rust implementation.

## Cross-repository tenancy

Identifiers bind to repository and campaign trust domains. Private source, raw
conversations, confidential fixtures and unpublished papers never flow between
projects without a separate audited sanitization/declassification event. Shared
knowledge cards contain abstract methods, bounded claims, source/experiment
citations and application constraints—not privileged source or decrypted
notebooks.
