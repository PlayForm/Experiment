# v6.1 HTTP API, events and correspondence inventory

## Canonical contracts

- `api/public.openapi.yaml`: operator/observer-only projections, read operations
  and signed owner commands.
- `api/internal.openapi.yaml`: mTLS trusted-service-only command endpoints,
  never mounted/advertised to workers.
- `api/events.asyncapi.yaml`: event channels for asynchronous software services.
- `events/catalog.json`: authoritative event kinds and producer/consumer roles.
- `schemas/v61-*.schema.json`: the new custody/evolution/licensing/projection
  records; matching reference examples in `examples/`.
- `policy/v61-roles.json`: canonical additional authority matrix.
- `policy/v61-state-machines.yaml`: custody, feedback, and cross-repo release
  machines.

## New public projections (no LLM access; see curriculum addendum for 4 more)

| Method and path                       | Schema                    | Purpose                                     |
| ------------------------------------- | ------------------------- | ------------------------------------------- |
| `GET /v1/custody/requests`            | page custody-request      | Audit transfer attempts                     |
| `GET /v1/custody/receipts`            | page custody-receipt      | Who holds what and why                      |
| `GET /v1/custody/holders`             | page holder-capability    | Holder capability and capacity              |
| `GET /v1/policies/embargoes`          | page embargo-rule         | Authorized embargo overview                 |
| `GET /v1/policies/licenses`           | page license-grant        | Rights and EULA review                      |
| `GET /v1/policies/trade-graph`        | page trade-restriction    | Research group separation                   |
| `GET /v1/research/tracks`             | page research-track       | Opinions lineage and assignments            |
| `GET /v1/research/views`              | page research-view        | Materialized worker contexts, metadata only |
| `GET /v1/research/feedback`           | page feedback-record      | Release into scoped contexts                |
| `GET /v1/research/knowledge-releases` | page knowledge-release    | Cross-track / cross-repo disclosure         |
| `GET /v1/repositories/admissions`     | page repository-admission | Repo eligibility and rights                 |
| `GET /v1/audit/denials`               | page denial-audit         | Human/operator only; restricted             |
| `GET /v1/experiments/specifications`  | page experiment-spec      | Software-compiled experiment truth          |
| `GET /v1/storage/leases`              | page storage-lease        | Capacity, retention and custody             |
| `GET /v1/facts/timeline`              | page temporal-fact        | Bi-temporal audit navigation                |

All HTTP responses have standard typed errors; pagination has stable cursors and
authorization filtering. Public readers need query scopes even when service mTLS
is present: cryptographic identity is not role authorization.
Embargo/license/denial endpoints are **not** public-internet resources; 'public
API' here means the operator-facing laboratory API.

## New internal service operations (plus 4 curriculum operations below)

| Method | Path                                        | Request type           | Principal           |
| ------ | ------------------------------------------- | ---------------------- | ------------------- |
| POST   | `/internal/v1/repos/admit`                  | repository-admission   | source-adapter      |
| POST   | `/internal/v1/artifacts/policy/bind`        | artifact-policy        | custody-policy      |
| POST   | `/internal/v1/holders/attest`               | holder-capability      | holder-supervisor   |
| POST   | `/internal/v1/custody/preflight`            | custody-request        | custody-broker      |
| POST   | `/internal/v1/custody/decide`               | custody-decision       | policy-service      |
| POST   | `/internal/v1/custody/commit`               | custody-receipt        | custody-broker      |
| POST   | `/internal/v1/custody/reserve`              | storage-lease          | storage-broker      |
| POST   | `/internal/v1/licenses/register`            | license-grant          | license-registry    |
| POST   | `/internal/v1/embargoes/register`           | embargo-rule           | policy-service      |
| POST   | `/internal/v1/trade/restrict`               | trade-restriction      | policy-service      |
| POST   | `/internal/v1/research/track`               | research-track         | coordinator         |
| POST   | `/internal/v1/research/view/compile`        | research-view          | context-projector   |
| POST   | `/internal/v1/research/task-context`        | task-context-plan      | supervisor          |
| POST   | `/internal/v1/research/intent/ingest`       | experiment-intent      | supervisor          |
| POST   | `/internal/v1/experiments/spec/compile`     | experiment-spec        | experiment-compiler |
| POST   | `/internal/v1/research/candidates/register` | candidate-batch        | coordinator         |
| POST   | `/internal/v1/feedback/project`             | feedback-record        | feedback-projector  |
| POST   | `/internal/v1/knowledge/release`            | knowledge-release      | publication-bridge  |
| POST   | `/internal/v1/knowledge/compatibility`      | release-compatibility  | policy-service      |
| POST   | `/internal/v1/audit/denial`                 | denial-audit           | audit-service       |
| POST   | `/internal/v1/facts/append`                 | temporal-fact          | ledger              |
| POST   | `/internal/v1/workers/neutral-receipt`      | worker-neutral-receipt | supervisor          |

These are _service contracts_, not a claim that all endpoints exist in running
Rust services. Internal admission and decision services must re-evaluate
authorization; trusting a `verdict: allow` submitted by a caller would defeat
the architecture.

## API design rules

All POST calls require `Idempotency-Key`, signed request envelope, authenticated
caller service identity, expected policy epoch and bounded expiry. Signatures
cover canonical serialized payload bytes, request target, method, idempotency
key, credential ID, and nonce. Replay prevention uses a durable nonce table and
idempotent event IDs. 202 means queued/accepted _for processing_, not that work
succeeded. 409 means stale/duplicate conflict; 422 means invalid trusted
request; 403 indicates trusted-service denial, never forwarded to a model. Same
request/token yields same receipt; new request under stale lease is fenced.

Neither API accepts arbitrary shell commands, prompt text as policy, or
LLM-delivered attestation objects. All model exchanges stay in the non-HTTP
offline mailbox, whose only worker-visible contract is the scoped filesystem.

## Software-defined curriculum and offline mailbox amendments

The curriculum adds four operator read operations and four authenticated
internal write operations documented in the canonical OpenAPI:

- `GET /v1/research/curricula` ↔ `POST /internal/v1/curriculum/register` —
  `v61-curriculum`.
- `GET /v1/research/studies/assignments` ↔
  `POST /internal/v1/curriculum/study/assign` — `v61-study-assignment`.
- `GET /v1/research/studies/outcomes` ↔
  `POST /internal/v1/curriculum/study/outcome` — `v61-study-outcome`.
- `GET /v1/research/resources/allocations` ↔
  `POST /internal/v1/resources/allocate` — `v61-physical-allocation`.

These never expose a lesson scheduler to a model. The offline model receives
only `schemas/v61-offline-task.schema.json` and returns
`schemas/v61-offline-submission.schema.json`, through local files rather than an
HTTP API. See `docs/28-offline-io-protocol.md`.

## Software-defined curriculum and offline mailbox amendments

The curriculum adds four operator read operations and four authenticated
internal write operations documented in the canonical OpenAPI:

- `GET /v1/research/curricula` ↔ `POST /internal/v1/curriculum/register` —
  `v61-curriculum`.
- `GET /v1/research/studies/assignments` ↔
  `POST /internal/v1/curriculum/study/assign` — `v61-study-assignment`.
- `GET /v1/research/studies/outcomes` ↔
  `POST /internal/v1/curriculum/study/outcome` — `v61-study-outcome`.
- `GET /v1/research/resources/allocations` ↔
  `POST /internal/v1/resources/allocate` — `v61-physical-allocation`.

These never expose a lesson scheduler to a model. The offline model receives
only `schemas/v61-offline-task.schema.json` and returns
`schemas/v61-offline-submission.schema.json`, through local files rather than an
HTTP API. See `docs/28-offline-io-protocol.md`.
