# Rust Optimization Laboratory — v6.1 Complete Control & Custody Contracts

**Status:** complete _design/contract draft_ with validated sample schemas, HTTP
API maps, signed-event catalog, additive database migrations and executable
contract checks. **Not** a deployed or security-audited Rust system.

## One external API, two telemetry planes, zero LLM control

- **Plane 0 — Human/GPUI/CLI public API:** 67 documented HTTP operations,
  authenticated readers and signed operator commands; models cannot reach it.
- **Plane 1 — Trusted Rust control/observability:** 53 internal authenticated
  service APIs, event-sourced state, signed custody and policy operations,
  encrypted correspondence archive, operational OTLP.
- **Plane 2 — PlayForm/Run performance evidence and SUT diagnostics:**
  authoritative trace-off benchmarks, isolated correctness/test mesh and
  separately characterized product trace variants.
- **Offline worker boundary X:** no network or privileged service access, only
  repository source snapshot, task, allowed local Rust tools, private feedback
  and output scratch. LLMs propose artifacts, never
  authorize/release/test/benchmark them.

## Constitutional amendment

The only untrusted _generative proposals_ in the code search loop come from
LLMs. Every selection, experiment compilation, verification, scheduling,
feedback release, artifact transfer, embargo, license, publication and policy
decision is trusted **software**. The control state machine is deterministic
given a pinned policy and ordered events; wall-clock/hardware measurements
remain noisy observations. The evolutionary effect is not an explicit coded
genetic algorithm; it is an outcome of untrusted variation, fixed evidence-based
selection and authorized inheritance.

A worker cannot know which repository peer is embargoed, why its artifacts were
blocked, or who holds privileged records. Its own submitted proposal receives an
opaque `submission_received` acknowledgement. The trusted audit ledger retains
real cause and owner incident status; there are no fabricated passing tests or
misleading success notices.

## New v6.1 documents

- `docs/17-v61-constitution.md` — authoritative new rules and trust boundaries.
- `docs/18-emergent-evolution.md` — open-ended evolution, independent opinion
  tracks, anti-collusion.
- `docs/19-artifact-sovereignty.md` — complete two-phase custody, holder
  qualification, storage and possession rights.
- `docs/20-embargo-licensing-and-trade.md` — SPDX, EULA, denied trade edges and
  legal review.
- `docs/21-two-timescale-propagation.md` — private fast feedback, slow
  sanctioned cross-repo knowledge, bitemporal facts.
- `docs/22-hidden-worker-and-correspondence.md` — no-network Rust implementation
  worker, local tools, encrypted turns and non-disclosing enforcement.
- `docs/23-interface-inventory.md` — all 24 new external and 27 new internal
  operations, plus protocol rules.
- `docs/24-dual-observability-and-correspondence.md` — metadata-only control and
  product telemetry.
- `docs/25-failure-analysis.md` — 30 critical/high/medium failure cases and
  explicit security limitations.
- `docs/26-acceptance-and-build-plan.md` — Rust service modules, staged
  integration and validation requirements.
- `docs/27-research-curriculum.md` — software-assigned creative research
  challenges and independent evaluation.
- `docs/28-offline-io-protocol.md` — the two and only two worker-visible JSON
  schemas and local files.
- `docs/27-research-curriculum.md` — software-assigned creative research
  challenges and independent evaluation.
- `docs/28-offline-io-protocol.md` — the two and only two worker-visible JSON
  schemas and local files.

Original `docs/00` through `docs/16` remain intact and part of this package.

## Machine-readable contracts

- `api/public.openapi.yaml` — **67 external HTTP operations** (v6+v6.1)
- `api/HTTP_INDEX.md` — generated inventory of every public and internal
  operation
- `events/EVENT_INDEX.md` — generated catalog of all signed event types
- `api/HTTP_INDEX.md` — generated inventory of every public and internal
  operation
- `events/EVENT_INDEX.md` — generated catalog of all signed event types
- `api/internal.openapi.yaml` — **53 trusted HTTP operations** (v6+v6.1)
- `api/events.asyncapi.yaml` — **15 signed event-bus channels**
- `events/catalog.json` — **87 event kinds** with producer/consumer/schema
- `schemas/` — **129 JSON Schema Draft 2020-12 definitions**, including 27 new
  trusted record types, 24 new paginated collection types and 2 model-visible
  offline records
- `examples/` — **112 validating examples**
- `policy/v61-roles.json`, `policy/v61-state-machines.yaml` — additional trust,
  capabilities and workflow state machines
- `sql/v61_migration.sql` — additive custody, rights, experiments, curriculum,
  feedback and bitemporal state after v2→v3→v5→v6
- `sql/prerequisites/` — copies of earlier SQL migrations for portable
  validation
- `tests/validate_package.py` — checks JSON schemas + examples, OpenAPI refs and
  operation IDs, event registry, state machines, full SQL migration and negative
  custody/immutability/idempotency tests

## Contract tests

Install Python testing tools (only for validation; production services are
intended to be Rust):

```bash
python -m pip install jsonschema referencing pyyaml
python tests/validate_package.py
```

## Example operator workflow (proposed, not a running API)

```bash
curl --cert operator.crt --key operator.key --cacert ca.crt \
	https://lab-ui.lab.internal:9443/v1/status
curl --cert operator.crt --key operator.key --cacert ca.crt \
	https://lab-ui.lab.internal:9443/v1/custody/receipts
curl --cert operator.crt --key operator.key --cacert ca.crt \
	https://lab-ui.lab.internal:9443/v1/experiments/specifications
```

**Do not expose this endpoint to the public Internet by default.** These are
authenticated human-operated _laboratory observer_ routes. The actual local LLM
workers have no HTTP client permissions or keys to access them.

## Security and correctness limits

This is a contract and schema package, **not** proof of host isolation,
confidentiality from a privileged observer, legal compliance, cryptographic
identity, full behavior preservation or live runtime speedups.
Build/test/benchmark and Rust services still must be implemented and subjected
to negative, integration and external security tests. No finite test suite can
prove the absence of all regressions.
