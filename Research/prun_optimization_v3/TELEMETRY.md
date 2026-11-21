# Dual-clock telemetry: orchestration plane + product execution plane

## 1. Three forms of truth, two boards

1. **Authoritative experiment event ledger**: `PATCH_SUBMITTED`, `CHECK_PASSED`,
   `RESULT_RECORDED`, `AGENT_RESTARTED`, etc. Every durable transition is
   retained independent of telemetry sampling. It is _not_ an OpenTelemetry span
   store.
2. **Control-plane OpenTelemetry (Board A)**: continuous, distributed Rust
   `tracing` + OTLP from coordinator, model gateway, novelty gate, builder
   broker, oracle service, evaluator and API. Records scheduling, queue, tokens
   and tool latency, build durations, retries, stalls, resource use.
3. **Product-plane OpenTelemetry (Board B)**: optional diagnostic
   instrumentation of Run itself to understand parsing, planning, traversal,
   matching, scheduling, child spawn, stdout forwarding, shutdown. Must not be
   assumed to be free. **Timing truth** comes from uninstrumented/minimally
   instrumented controlled release runs and external counters; product traces
   from separate diagnostic runs are interpretive evidence.

**Avoid mixing clocks by default.** Use separate `service.namespace`,
`lab.plane`, endpoints, sampling settings, retention and dashboard panels. Join
with `campaign.id`, `experiment.id`, `mutation.id`, `artifact.sha256`,
`trial.id` and an explicit correlation event. Prefer OTel span links across
asynchronous work orders; producer/consumer spans need not share a parent and
should not artificially span a week-long campaign.

## 2. Compilation and execution modes

| Mode               | Compiled instrumentation                                                                                                    | Export                             | Use                            | May compare for primary speed?           |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------- | ---------------------------------- | ------------------------------ | ---------------------------------------- |
| `trace-off`        | `#[cfg]` eliminates product stage probes and exporters; preserve only baseline-essential counters outside measured hot path | none                               | release timing                 | **yes**, with similarly built candidates |
| `trace-lite`       | small preallocated/lower-frequency counters; controlled instrumentation, no network in measured path                        | after trial / local bounded buffer | calibrated overhead experiment | separate cohort only                     |
| `trace-diagnostic` | Rust `tracing` spans and selected events, OTLP to local collector, optional perf/ebpf                                       | nonblocking on diagnostic host     | attribution & flamegraphs      | **no** as proof of baseline speed        |
| `profile-build`    | PGO coverage or LLVM instrumentation                                                                                        | profile files                      | training set                   | **no** as proof of release speed         |

Set a _separate_ `instrumentation_mode` in build/run identity. Test
instrumentation bias by comparing `trace-off` to `trace-lite` on the same source
and known workloads; capture the delta as measured overhead, not as a universal
fixed subtraction. The diagnostic binary can change timing/cache/code layout; do
not insert results from it into release leaderboards.

## 3. Board A: control-plane spans and metrics

Suggested spans:

```
lab.campaign.run
  lab.generation.freeze
  lab.task.assign
  lab.agent.context.prepare
  lab.llm.request
  lab.llm.stream
  lab.patch.submit
  lab.novelty.evaluate
  lab.build.prepare / lab.build.compile
  lab.oracle.execute
  lab.bench.lease.acquire / lab.bench.block
  lab.analysis.evaluate
  lab.feedback.publish
  lab.lineage.commit
```

Most async steps are **not** a single nested trace. Each task attempt has a
short-lived trace; use `SpanLink` to connect task submission, worker
consumption, benchmark completion and returned feedback. Model each queue hop as
a transport step with a stable `work_order_id` and `causation_event_id`.

Metrics (low-cardinality labels: role, image class, workload family, host class,
status, policy version; never free-form prompt or command):

- `lab_tasks_queued`, `lab_tasks_inflight`, `lab_agent_active`,
  `lab_worker_idle`
- `lab_task_queue_delay_seconds`, `lab_lease_wait_seconds`,
  `lab_build_duration_seconds`, `lab_oracle_duration_seconds`,
  `lab_benchmark_wall_seconds`
- `lab_novelty_cache_hits_total`, `lab_duplicate_proposals_total`,
  `lab_similarity_review_total`, `lab_patch_success_total`
- `lab_llm_ttft_seconds`, `lab_llm_ttfb_seconds`, `lab_llm_tokens_in_total`,
  `lab_llm_tokens_out_total`, `lab_llm_cached_tokens_total`,
  `lab_llm_tool_roundtrip_seconds`
- `lab_measurement_contamination_total`, `lab_runner_cpu_throttled_seconds`,
  `lab_host_temperature_celsius` (where permitted), `lab_disk_free_bytes`
- `lab_evidence_ingest_latency_seconds`, `lab_event_projection_lag`,
  `lab_otlp_dropped_spans_total`, `lab_agent_restarts_total`,
  `lab_campaign_budget_remaining`

Do not put campaign/run/patch UUIDs into **metric labels**: extremely high
cardinality is harmful. Use trace/span attributes, event records, or query-index
fields for these IDs.

## 4. Board B: product stage signals

Suggested staged spans when instrumentation is enabled:

```
prun.invocation
  prun.args.parse
  prun.hardware.detect
  prun.heuristic.analyze
  prun.plan.select
  prun.walk / prun.match
  prun.schedule
  prun.child.spawn / prun.child.wait
  prun.output.forward
  prun.shutdown
```

Track `analysis_overhead_ns`, paths visited, matches, executor queue occupancy,
workers active, child spawn/exit counts, RSS, bytes read/written, output chunks,
idle wait, syscall category and scheduling wakeups. Protect output/path/command
contents: do not upload arbitrary user input or source paths as span attributes
by default. Record counts, sanitized classes or salted stable per-campaign IDs
instead.

Do not attach the benchmark controller's long-lived trace context as a parent of
every product span; use a `SpanLink` or shared correlation fields to avoid
sampling/lifetime distortion. A product run has its own trace and invocation ID.

## 5. Rust OTel implementation plan

Control-plane dependencies (pin tested compatible versions when implementing):
`tracing`, `tracing-subscriber`, `tracing-opentelemetry`, `opentelemetry`,
`opentelemetry_sdk`, `opentelemetry-otlp`. Use `#[instrument]` on service
operations and manually annotated phases, avoiding very high frequency per-item
spans. Propagate W3C `traceparent` + a typed task causation ID across broker
calls; use async span links for message consumption.

If the requirement is that **all authored backend code runs in Rust**, implement
an OTLP/HTTP or OTLP/gRPC receiving service using `axum` or `tonic` with
`opentelemetry-proto` message definitions. Validate payload limits, resource
descriptors and retention; index compact span rows plus store compressed raw
envelopes. This makes the OpenTelemetry Collector optional. The reference
`otel-collector-optional.yaml` demonstrates an off-the-shelf alternative, but
the official Collector is implemented in Go and therefore does not satisfy a
literal all-Rust-process requirement.

Provide Rust HTTP APIs: `GET /api/campaigns/{id}/events?cursor=...`,
`GET /api/experiments/{id}/traces`, `GET /api/compare?...`,
`GET /api/agents/{id}/conversation`, plus SSE (`/api/stream`) for monotonic
events, with cursor replay and occasional snapshots. The UI must never read the
DB or Docker socket directly.

## 6. Sampling / signal retention

- Control-plane: retain **all state-change events** in durable ledger, keep
  sampled spans/logs by policy (errors and slow tasks favored), aggregate
  metrics with defined rollups. Do not try to use tail sampling as an event
  ledger.
- Product-plane: capture diagnostic traces for a selected subset of explicitly
  launched diagnostic executions; retain all externally measured primary timing
  samples. Heavy traces and flamegraphs have independent retention and storage
  quotas.
- Enable sampling only on the correct pipeline. If multi-ingestor tail sampling
  is introduced, route all spans for a given trace ID to the same
  processor/collector.
- Record exporter failures and dropped spans; missing trace data must not
  invalidate raw timing facts, but it _does_ invalidate claims based on absent
  diagnostic spans.

## 7. FPGA-like dashboard metaphor

Treat each telemetry plane as a separate **clock domain**. Board A shows event
bus traffic, worker states, queues and token streams. Board B shows measured
execution stage times and operational counters. A correlation fabric at the
center maps immutable `trial_id`, `binary_sha256`, and `experiment_id` to both.
All units and time sources remain explicit (UTC event timestamps, monotonic
durations, NTP/clock drift if cross-host).

A panel switch may visualize `instrumentation overhead = trace-lite - trace-off`
from measured controlled trials, but it must never present diagnostic stage
timings as if they were uninstrumented wall-clock truth.
