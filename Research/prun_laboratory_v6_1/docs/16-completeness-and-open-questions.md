# Completeness boundary, open engineering decisions and implementation phases

## What "complete schema" means here

This release provides a _finite, canonical v6 contract surface_: every current
HTTP method/path, request/response schema, event kind/payload, typed offline
correspondence record, role and policy namespace, state transition, telemetry
field vocabulary, storage table, and high-risk validation scenario defined by
the architecture. A future new capability must extend these contracts and pass
conformance tests. No schema can predefine every conceivable future plugin
behavior or guarantee correct implementations.

## Implemented **in the artifact package**

- Versioned OpenAPI for human-facing and internal service calls.
- AsyncAPI event-bus definition and complete 57-event catalog.
- Strict JSON Schemas and matching sample fixtures.
- Declared policies for access, state machines, telemetry and classification.
- Additive SQLite migration, immutable journal/correspondence triggers, test
  validator.
- Narrative trust, dashboard, agent, oracle, storage and recovery
  specifications.

## Not yet deployed/verified

- Rust coordinator, `axum` HTTP API, `tonic` OTLP receivers, Hickory DNS
  infrastructure, CA issuance daemon and on-host cryptographic enforcement.
- Offline model runtime and process-level sandbox, immutable builder,
  correctness oracle, exclusive host benchmark scheduler and evaluator.
- GPUI dashboard, cross-repository adapters, publication service and market
  allocator.
- End-to-end attestation signing, replay/rotation under real cryptographic keys,
  real Docker network/mount isolation, workload profiling and attack testing.

## Design decisions needing explicit owner sign-off before implementation

1. Local-only Unix-socket public API versus remote mTLS observer access.
2. PKI key custody and threat model (host owner trusted? hardware security
   module? dedicated isolated host?).
3. Privacy policy for agent notebooks, transcript decryption, retention and
   deletion.
4. Behavior specification for each upstream repository and approval procedure
   for adding new trusted tests.
5. Bench host topology and resource isolation, workload corpus and acceptable
   regression thresholds.
6. Which research papers are internal-only or public, and licensing/sanitization
   policy.
7. Exactly how to define the production Pareto selection policy and hardware
   specialization fallback behavior.
8. Whether SQLite single-host coordination suffices or federation requires
   distributed transactional infrastructure.

## Suggested implementation phases

- Phase A: compile schemas into Rust data types; implement fake Rust APIs, event
  replay, SQLite append atomics and the complete scenario without any model.
- Phase B: offline model supervisor + file mailbox, untrusted patch ingestion,
  source scope and malicious artifact tests.
- Phase C: immutable builders, oracle mesh, benchmark host with fixed CPU
  affinity/leases, raw evidence/attestation.
- Phase D: PKI/mTLS, DNS service discovery, optional SSH for _operators only_,
  encryption and certificate transparency audit.
- Phase E: two OTLP pipelines, read projections, GPUI console,
  privacy/cardinality fuzz tests.
- Phase F: independent research publishing, cross-repository adapters and
  recursive experimentation with frozen trusted policies.

Successful release requires end-to-end drill with malformed messages,
wrong-scope certificates, restarts, clock drift, offline model escape probes and
contaminated benchmarks. No generated document is itself proof of those
properties.
