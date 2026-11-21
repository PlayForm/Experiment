# PlayForm/Run — Autonomous Recursive Optimization Laboratory v3

**Status:** engineering design and validated interface samples, **not** a
working multi-agent system. **Builds upon**, and does not replace, the
[v1 Docker starter](../prun_lab/README.md) and
[v2 architecture](../prun_optimization_v2/REQUIREMENTS.md). This package is a
self-contained v3 architectural overlay; retain the prior bundles alongside it.

## Start here

1. `ARCHITECTURE.md` — topology, trust boundaries, state machine, and exact
   5-way fanout.
2. `REQUIREMENTS_V3.md` — extended, testable requirements and development
   phases.
3. `CONTAINER_MATRIX.md` — image types, resources, visibility, mounts, and
   measurement boundaries.
4. `AGENT_CONTRACTS.md` — role taxonomy, restricted context, prompt/session
   lifecycle, feedback formats.
5. `RESEARCH_GATE.md` — duplicate prevention, novelty, evidence reuse,
   anti-overfitting.
6. `TELEMETRY.md` — control/product/measurement separation and OpenTelemetry
   implementation.
7. `STORAGE.md` — event sourcing, prompt/response storage, data retention and
   proof lineage.
8. `DASHBOARD.md` — Rust+GPUI/Ely desktop, optional WebView, two-board signal
   cockpit.
9. `RUNBOOK.md` — rollout, campaign recovery, acceptance tests and
   implementation roadmap.

Machine-readable examples:

- `interfaces/event-envelope.v3.schema.json`
- `interfaces/proposal.v3.schema.json`
- `examples/proposal.v3.json`
- `examples/event.v3.json`
- `examples/campaign-overlay.v3.json`
- `playbooks/fanout-5.yaml`
- `sql/v3_migration.sql` (extension to the v2 SQLite schema)
- `telemetry/otel-collector-optional.yaml` (optional **Go-based** Collector
  reference, not required in the default Rust control plane)

## Key decisions

- **Parent agent != coordinator.** A deterministic Rust coordinator owns truth.
  The parent LLM is a strategic adviser with comprehensive _authorized_ read
  access, not the controller of evidence or access-control decisions.
- **Patcher returns a committed patch first.** Suggested optimization text
  without a patch cannot enter the benchmark queue.
- **Novelty gate before build, correctness gate before measurement, confirmation
  gate before promotion.** Similarity is an advisory signal; exact equality is
  provable only for specific canonical artifacts.
- **Five forks from an identical immutable base**, each patched in its own
  worktree. Measurement is exclusive on a given benchmark host even when
  construction is parallel.
- **Experimental evidence is never an LLM assertion.** All scorecards and
  approvals are computed from stored data by versioned Rust evaluators.
- **The clock domains remain distinct.** Control-plane spans may be continuous;
  product release timing must be minimally instrumented, while product
  diagnostic traces run in a different variant.
- **Data plane is separate from event plane.** The authoritative append-only
  event log and content-addressed blobs survive missing/dropped tracing data.
- **Agent contexts are disposable cached views, not authoritative memory.**
  Conversation continuity is rehydrated from an immutable task ledger, with
  strictly scoped feedback.
- **Rust-first** control plane and Rust desktop UI. The underlying Docker
  Engine, external models, Hyperfine, and optional SQLite/OpenTelemetry
  Collector remain third-party components. A pure-Rust embedded state store
  (`redb`) is an alternative to SQLite, at the cost of implementing additional
  indexes/query facilities.

## Implementation status

The v3 files are formal specifications, sample schemas and a DB migration. The
examples and migration can be locally syntax-checked. No model provider, Docker
runtime, GPUI app, benchmark trial, or autonomous agent has been executed in
this package. Actual deployment needs a Linux benchmark host, model adapter,
source checkout, and implementation work.
