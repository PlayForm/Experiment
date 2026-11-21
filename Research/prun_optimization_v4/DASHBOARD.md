# Rust desktop cockpit: two FPGA-style diagnostic boards

## 1. UI architecture

**Rust daemon** (`axum` REST + SSE, optional WebSocket where bidirectional
streaming matters) serves authenticated event projections, comparisons,
conversations and trace indexes. **Rust desktop app** (`GPUI`) renders native
widgets, using Ely GPUI as an optional component/theme library. **Optional
WebView** via `gpui-wry` or Wry for embedded reporting pages, plots, flamegraphs
or HTML-based documentation. Prefer native GPUI for the high-frequency
operational canvas; do not make experimental WebView integration a prerequisite
for the control plane.

GPUI and Ely are _UI layers_, not event ingestion, benchmark engines or
telemetry stores. Ely GPUI's public repository describes it as an early
component library; pin an exact commit and test Linux/macOS/Windows
compatibility in your deployment environment. If fully Rust-authored UI is a
hard goal, use native GPUI components for every panel and treat the WebView as
optional, since its document layer may involve HTML/JavaScript.

## 2. Main screen: two signal domains + correlation bus

```text
┌──────────────────────── Board A: LAB CONTROL CLOCK ─────────────────────────┐
│ Campaign 24h/168h  │ Agent pool: idle 2 / active 4 │ Queue: 18 │ Tokens/s  │
│ B0 frozen ✓ │ Generation 42 │ Gate hit rate 37% │ Build backlog 3          │
│ SCOUT ─ PATCHER-1 ─ GATE ─ BUILDER ─ ORACLE ─ LEASE ─ ANALYST ─ PATCHER-1   │
│         trace graph + conversation lane + worker state + exact event IDs     │
├──────────────────── Correlation fabric / provenance spine ──────────────────┤
│ experiment_id  trial_id  mutation_id  binary_sha  workload  env cohort      │
│ cache proof · attestation stamp · causal ancestry · source diff · replay     │
├───────────────────── Board B: RUN PRODUCT CLOCK ────────────────────────────┤
│ OFF: 4.30 ms │ TRACE-LITE: 4.51 ms │ DIAG: 5.38 ms [illustrative only]      │
│ parse ─ plan ─ walk ─ match ─ schedule ─ spawn ─ wait ─ output ─ shutdown     │
│ latency distribution / CPU counters / RSS / cache misses / I/O / regressions│
└────────────────────────────────────────────────────────────────────────────┘
```

The numbers above are **fictional UI labels**, not measured PlayForm/Run
performance. Any stage timeline from diagnostic runs must be labeled
`DIAGNOSTIC` and not presented as breakdown of the uninstrumented timing run.

## 3. Views

- **Campaign cockpit:** progress, phase, safe pause, resource budgets, container
  states, queue, retries, time-to-next-verified-result and exact replay state.
- **Agent spectrum:** role clusters, worker/prompt version, context-window
  usage, TTFT/TTFB, streaming throughput, call/cost budget, duplicate idea
  families, context changes and transcript view.
- **Mutation lineage:** single-parent timeline per lane, multi-parent
  integration DAG, exact source diff, feature matrix, accepted/rejected reasons
  and canonical patch search.
- **Comparison microscope:** B0 vs P1..P5, paired samples, histogram, confidence
  interval, effect threshold, run order, host drift, trace-off vs trace-lite
  overhead experiments and source/fixture hashes.
- **Product hardware board:** CPU core/NUMA, filesystem and machine cohort, ISA
  target, process stage bottlenecks, optional perf and GPU research lane.
- **Evidence ledger:** signed stamps, data-loss alerts, trace sampling status,
  artifact verification, holdout access audit, event-sequence cursor and replay
  controls.
- **Conversation forensics:** exact prompt template, authorized context
  snapshot, redaction status, turns, compaction mapping, tool results, final
  patch and feedback capsule.

## 4. Interactive controls and data guarantees

Use optimistic UI only for temporary visual feedback; an operation becomes
authoritative only after a confirmed state-change event from the coordinator.
Example buttons: Pause campaign, Resume, Drain agents, Freeze generation,
Revalidate evidence, Re-run B0, Quarantine trial, Compare candidates, Inspect
parent chain, Replay task, Approve release. Each command has role authorization,
idempotency token, audit event, optional confirmation and rollback semantics.

Load an immutable snapshot followed by SSE events with `Last-Event-ID` cursor.
On disconnect, re-sync and replay; do not silently skip event gaps. Downsample
high-frequency UI rendering (e.g., 10–30Hz presentation) without dropping
durable backing data. UI CPU/GPU should never contend with benchmark hardware
during precision measurements.

## 5. Rust implementation milestones

- `lab-api`: axum JSON endpoints and SSE stream; pagination and filtered
  evidence access.
- `lab-model`: typed Rust state projections for campaigns, agents, experiments
  and traces.
- `lab-ui`: GPUI desktop with Ely design system pinned at a verified commit;
  native graph components for signal buses, lane timeline and comparisons.
- `lab-webview-optional`: Wry integration for complex HTML reporting and
  external docs; strict origin restrictions and no privileged IPC by default.
- `lab-render-snapshots`: deterministic snapshots of UI views for bug reports;
  version tags and privacy filters.

The console is a **monitor and authorized control surface**; source-of-truth
state is the coordinator's event ledger, not the window process.
