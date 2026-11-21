# PlayForm/Run Laboratory v6.2 — Cryptographic Schema Fabric

**Status:** Proposed additive design and machine-readable draft contracts. JSON
Schema validation checks are executable; no Rust service, cryptographic
protocol, enclave, Docker deployment, or external security audit has been
performed. This is **not** a claim of being unhackable.

Extends the existing v6.1 laboratory contracts without replacing their operator
API, trusted event machinery, two telemetry planes, or offline-worker protocol.

## Goals and trust guarantees

1. Treat canonical source, schema identifiers, policy names, relationships,
   repository identity, and private event kinds as confidential metadata where
   required.
2. Expose only scope-specific opaque identifiers to telemetry and authorized
   projections; a privileged observer receives special, audited views, not all
   private mappings by default.
3. No LLM process receives laboratory cryptographic keys, CA credentials, policy
   registry, internal APIs, private mapping table, or direct egress; it gets a
   pinned Rust source/task view and local tools only.
4. Confidentiality and authorization are separate: randomized names are not
   authority. No bearer token or seed is accepted alone as proof of
   authorization without binding to a trusted workload and current policy.
5. All critical identifiers are collision checked. The system fails closed on
   namespace collisions and version conflicts.
6. Fast in-memory projections and arenas are rebuilt from encrypted, signed,
   append-only, crash-recoverable state. No immutable history is held only in
   volatile memory.
7. Dedicated key custody: offline root, separately rotated
   service/message/data-encryption keys, non-exportable key operations where
   feasible, independent recovery materials. A genesis commitment is not a
   universal encryption master key.
8. Zero external egress and no lab access from LLM workers, but generated Rust
   build/test code is also untrusted and must be isolated from trusted host
   processes.

## File index

- `docs/01-assessment.md` — five proposal assessments and counterexamples.
- `docs/02-architecture.md` — four planes, opaque boundary, service roles and
  endpoints.
- `docs/03-identifier-factory.md` — typed identifiers, validation and
  pseudonyms.
- `docs/04-keys-and-cryptography.md` — key hierarchy, HPKE, limits of usage
  seeds and ZK.
- `docs/05-schema-and-policy.md` — private schema IR, deterministic compilation,
  compiled resolvers and Cedar-like policy separation.
- `docs/06-memory-and-lineage.md` — arena, encrypted event journal, epochs,
  migrations, recovery.
- `docs/07-observability-and-privacy.md` — projections, metadata/traffic
  leakage, telemetry separation.
- `docs/08-container-and-api-matrix.md` — container provisioning, additive
  service API contracts, no worker access.
- `docs/09-assurance-plan.md` — threat model and required negative/integration
  tests.
- `schemas/` — additive Draft 2020-12 schema definitions with deliberately
  descriptive authoring names; compiled deployments conceal naming in a private
  registry.
- `examples/` — validating sample records containing synthetic non-secret
  identifiers only.
- `tests/validate.py` — syntax, reference and negative validation tests;
  requires `jsonschema`.

**Implementation note:** Hiding source filenames in the public release of a
contract is not itself a security requirement. The factory's canonical
descriptions are secret at runtime; the published design is descriptive, while
deployed wire messages use generic encrypted envelopes. A public spec should be
safe to read.
