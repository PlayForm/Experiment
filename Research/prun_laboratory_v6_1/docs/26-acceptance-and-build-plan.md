# Implementation phases and executable acceptance criteria

## All-Rust implementation components

- `lab-kernel`: Axum/Tonic front-end, event-sourced state machine, SQLite WAL
  (single writer) and policy epochs. No model inference in authoritative path.
- `lab-custody`: typed artifact inventory, tenant encryption, storage
  reservation, lease fences, checked mounts, two-phase transfer and signed
  receipts.
- `lab-policy`: embedded Rust Cedar policy for
  `(principal,action,resource,context)` plus strict algorithmic evaluation of
  pinned license/embargo/holder requirements. Deny overrides allow; any unknown
  fails closed. Cedar does not determine legal compatibility by itself.
- `lab-context`: offline source view compiler, access filtering, capability
  manifest, volatile scratch and feedback projection.
- `lab-agent-supervisor`: launches no-network model/implementor container,
  accepts artifacts, seals tool and correspondence logs; LLM process has no API
  access.
- `lab-source-adapter`: frozen source/dependency admission, repo/toolchain
  format readers, canonical path policy, license inventory.
- `lab-builder`, `lab-oracle`, `lab-runner`, `lab-evaluator`: isolated signed
  measurement pipeline; release trace-off, diagnostics separately.
- `lab-knowledge`: paper sanitization, signed evidence claims,
  revocation/embargo/license cross-repo bridge; never auto-promotes performance
  results.
- `lab-identity`: rustls mTLS, offline root/short-lived intermediary identities,
  ssh certs for humans, Hickory private DNS for **trusted services**, signed
  audit of issuance/revocation. Models never join this network.
- `lab-otel`: two separate pipelines, ledger-linked references and field
  allowlists.
- `lab-ui`: GPUI/Ely operator console for public API projections; implement once
  kernel is proven.

## Recommended first vertical slice (not a mock benchmark)

1. Owner pins a public Rust repo commit, approved license, offline dependencies
   and reference oracle; admission record sealed.
2. Source context compiler creates two different materialized task views for
   independent offline implementors.
3. Each worker locally inspects/compiles Rust and submits patch via scratch. No
   service route, CA key, env secret or sibling correspondence is accessible.
4. Supervisor seals each candidate; scope, duplicate and rights gates run
   without LLM involvement.
5. Trusted builders compile frozen parent + both candidates; oracle runs
   reference and new change-impact tests.
6. Runner obtains exclusive host lease, executes paired trace-off measurements,
   returns immutable raw samples and hardware metadata.
7. Evaluator applies the predeclared policy, preserving regressions and
   inconclusive outcomes; projector emits private scoped feedback without peer
   data.
8. Paper release across one repo boundary is denied under embargo, then succeeds
   only with a separately versioned owner-approved license/embargo change and
   redacted paper; target repo receives hypothesis only.
9. Simulate coordinator crash between reserve/stage/commit and verify receipt
   exactly once; simulate CA rotation, policy outage, storage exhaustion, trace
   pipeline failure and hardware contention.
10. A human uses only public GETs to inspect current status, full custodian
    history, evidence and denials; every model remains offline.

## Contract tests (pass/fail)

- JSON Schema 2020-12 metaschemas and examples validated, all `$ref`s resolve.
- Both OpenAPI files parse, every new operation has input/output schema and
  role, and no worker role is authorized on any internal/public operation.
- Event vocabulary is unique and each type has schema and producer role; event
  log consistency tests verify hash link / idempotency.
- v6.1 SQL migration applies after the earlier v2, v3, v5 and v6 migrations;
  append-only receipts, denial and feedback rows reject UPDATE and DELETE.
- Access policy denies an embargoed recipient even with perfect mTLS certificate
  and signature.
- Holder cannot receive a bytes transfer when capacity or encryption-at-rest is
  insufficient, and cannot commit under a stale fence.
- Cross-repo sanitized paper and raw source have different grants; a publication
  grant never automatically authorizes source redistribution or model training.
- A neutral worker submission receipt cannot be mistaken for acceptance,
  correctness, benchmark or publication.
- Physical benchmarking result is confirmed only by trusted runner/evaluator
  attestations, not by a worker's local `cargo test` output.
- No OTel payload includes prompt contents, sibling IDs or secret paths;
  drop/stop tracing must not alter authoritative measurement decisions.

## Completion honesty

The v6.1 package includes **contracts, examples, protocols, SQL and
validation**, not a compiled integrated Rust application. Dependencies, Docker
hardening, trust services, distributed host coordination and GPUI UI must be
implemented, built and tested against these contracts. Do not claim data
confidentiality from an operator who administers the host or proof of complete
software correctness.
