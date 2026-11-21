# Dual-plane OpenTelemetry: control and SUT execution

OpenTelemetry observes; the signed event ledger and benchmark samples establish
canonical experimental facts. **Plane 0 is a read API**, not a third telemetry
exporter. An API call can be correlated to traces through request IDs but the
model container never receives or can access trace context.

## Physical and logical separation

| Property           | Plane 1: control                                                            | Plane 2: product                                                                   |
| ------------------ | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| Producer           | Trusted coordinator, supervisor, builder, oracle, scheduler, evaluator, PKI | SUT instrumentation in diagnostic build; benchmark host external counter collector |
| Gateway            | `otlp-control` separate Rust daemon                                         | `otlp-product` separate Rust daemon                                                |
| Authentication     | Service-specific mTLS                                                       | Diagnostic collector certificate                                                   |
| Storage            | Control span store                                                          | Product diagnostic store                                                           |
| Default            | Enabled                                                                     | Disabled during primary benchmark                                                  |
| Data               | Job states, queue, duration, counts, event IDs, digests                     | Traversal phases, matching, process spawn, CPU, RSS, sampled counters              |
| Must never contain | Prompt text, code, output, shell argv, key, raw tool arguments              | File path/command/env/stdout/stderr, private input data                            |
| UI                 | Operational graph / timelines                                               | Performance diagnosis / phase breakdown                                            |
| Failure policy     | Can degrade control visibility; journal remains authoritative               | Cannot affect trace-off primary benchmark evidence                                 |

## Correlation and context

Use low-cardinality exact identity keys: `lab.campaign_id`, `lab.experiment_id`,
`lab.mutation_id`, `lab.job_id`, `lab.task_id`, `sut.binary_digest`,
`sut.workload_digest`. These are **software-generated**, not model-provided. For
asynchronous handoffs, link producer and consumer spans with OpenTelemetry span
links; a span's parent need not be the previous logical event. The immutable
event ID and ledger sequence are more authoritative than any trace.

Trace IDs must never be passed to model prompts or exposed as infrastructure
guidance. The supervisor may correlate an opaque task ID to experiment IDs
internally without disclosing this map to the model.

## Control span catalog

- `lab.command.validate`
- `lab.state.transition`
- `lab.queue.dispatch`
- `lab.worker.lease`
- `lab.supervisor.stage`
- `lab.inference.invoke`
- `lab.worker.output.seal`
- `lab.novelty.check`
- `lab.build.prepare`
- `lab.build.execute`
- `lab.oracle.verify`
- `lab.benchmark.reserve`
- `lab.benchmark.run`
- `lab.evaluation.evaluate`
- `lab.feedback.prepare`
- `lab.pki.issue`
- `lab.pki.revoke`
- `lab.paper.sanitize`
- `lab.publication.review`
- `lab.cas.commit`
- `lab.event.append`

## Product diagnostic span catalog

- `sut.command.parse`
- `sut.heuristic.analyze`
- `sut.plan.select`
- `sut.walk.discover`
- `sut.glob.match`
- `sut.scheduler.enqueue`
- `sut.process.spawn`
- `sut.process.wait`
- `sut.output.forward`
- `sut.command.total`

## Control metrics

- `lab.jobs.queue.depth`
- `lab.jobs.active`
- `lab.leases.held`
- `lab.bench_host.exclusive_occupancy`
- `lab.agent.task_duration`
- `lab.inference.wall_duration`
- `lab.build.wall_duration`
- `lab.oracle.pass_count`
- `lab.oracle.failure_count`
- `lab.benchmark.contamination_count`
- `lab.ledger.append_latency`
- `lab.pki.credentials.issued`
- `lab.pki.credentials.revoked`
- `lab.artifact.bytes_total`
- `lab.telemetry.dropped_spans`

## Product metrics

- `sut.elapsed_ns`
- `sut.cpu_user_ns`
- `sut.cpu_system_ns`
- `sut.peak_rss_bytes`
- `sut.allocated_bytes`
- `sut.minor_faults`
- `sut.major_faults`
- `sut.files_visited`
- `sut.matches`
- `sut.children_spawned`
- `sut.errors`

## Instrumentation policy

- **Off**: compile out or disable target-internal telemetry; no exporter task,
  no allocation-heavy tracing inside timed path. External trusted runner still
  measures timing/CPU/RSS for authority.
- **Lite**: bounded counters in a separate explicit instrumentation experiment.
  Measure its overhead against `off`.
- **Diagnostic**: rich spans under an independently tagged build; not eligible
  for primary performance comparison.
- **Profile**: training-only PGO/perf/BOLT profiles with full artifact lineage.

Do not inject trace headers or OTel exporters into the LLM model. Inference
engine wall time may be recorded **externally** by its trusted supervisor,
including time-to-first-visible-token and turn completion, but those
measurements are _operational_, not performance acceptance criteria.

## Cardinality and privacy

Only fields in `policy/telemetry.json` are accepted. User-provided file paths,
command arguments, prompts, exception stack traces, extracted text, source
diffs, arbitrary labels and model output are rejected/redacted at the gateway; a
compiler-printed secret is not safe to log. An artifact hash is metadata and may
still be sensitive; expose hashed/pseudonymous tokens according to view policy.

## Dual trace reconciliation

A dashboard may show control event seq 10422 with the associated SUT diagnostic
run, but must label that run `diagnostic—not authoritative timing`. Missing
telemetry is rendered as missing—not silently interpolated. Traces are sampled
and best-effort; the signed measurement series is never sampled or lossy.

## Rust implementation

Use `tracing` and OpenTelemetry Rust SDK for trusted services; OTLP gRPC
receiver can be Rust `tonic` + `opentelemetry-proto`. A standard upstream
Collector may be used temporarily for diagnostics, but violates the strict
all-Rust-process requirement if included in the final laboratory. Pin a
semantic-convention version for stable naming and migrate attributes
intentionally.
