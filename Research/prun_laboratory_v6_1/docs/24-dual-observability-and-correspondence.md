# Monitoring one API with two isolated observability domains

## Software truth versus diagnostic telemetry

The public dashboard consumes event-ledger-backed projections only. It can
expose event streams, lineage, custody/holder inventory, research case state,
storage pressure, denial incidents, documentation and artifact hashes, and
signed experiment outcomes. OTel _can_ explain latency but cannot authorize a
benchmark claim or change repository behavior. Model containers have no OTel SDK
configuration, telemetry exporter, service DNS, traceparent headers, or shared
logs.

### Plane 1: trusted laboratory control/orchestration

Spans: `lab.task.stage`, `lab.worker.assign`, `lab.worker.complete`,
`lab.artifact.preflight`, `lab.artifact.reserve`, `lab.artifact.stage`,
`lab.artifact.verify`, `lab.artifact.commit`, `lab.artifact.release`,
`lab.policy.evaluate`, `lab.license.check`, `lab.embargo.check`,
`lab.trade.check`, `lab.feedback.project`, `lab.knowledge.declassify`,
`lab.knowledge.transfer`, `lab.builder.build`, `lab.oracle.verify`,
`lab.runner.lease`, `lab.runner.measure`, `lab.evaluator.compare`,
`lab.ledger.append`, `lab.audit.record`.

Counters/histograms: claims submitted, duplication rate, policy denials by broad
owner-visible category, reservations/active storage, bytes transferred,
event-lag, backlog, profile storage, retry rates, certificate
issuance/revocations, local model invocation durations, experiment cohorts,
invalid/contaminated samples. Never treat a raw model claim as a measurement
metric.

### Plane 2: software under test

Trace-off release binaries for authoritative wall/CPU/memory measurements.
Diagnostic variants may emit `sut.parse`, `sut.plan`, `sut.walk`, `sut.match`,
`sut.schedule`, `sut.spawn`, `sut.output`, `sut.cleanup`; all overhead is
measured separately. Hardware counters include cycles, instructions, cache
misses, migrations, context switches, peak RSS, allocations and I/O as
platform-supported. Cross-host counter readings are not automatically
comparable.

### Correlation

Ledger `event_id`, `experiment_id`, `artifact_digest`, `source_tree_digest`,
`workload_id`, `runner_cohort`, `policy_digest`, and `correlation_id` link
projections. Only trusted control services see the global graph; the offline
worker gets local opaque references. Use explicit OTel span links for
asynchronous work, but never leak private content in span attributes. Trace
sampling/drop must not destroy evidence; evidence event journal remains
authoritative.

### Cardinality and privacy

Never place source code, prompt text, tool arguments, private claim bodies,
license text, credentials, HPKE plaintext, encryption key ID, embargo target,
banned sibling identity or file paths containing sensitive names into telemetry.
Even hashes can be linkable identifiers: restrict high-cardinality fields to
encrypted drilldown and use short opaque public IDs. Denial reason counts are
visible only to the human/operator audit tier.

## User experience

Four core GETs suffice to follow a campaign: `/v1/status`, `/v1/events/stream`,
`/v1/experiments/{id}`, `/v1/lineage/{id}`. Additional read endpoints are
optional drilldowns for custody, rights, research, tools and incidents. GPUI and
CLI use the same authenticated public API, no privileged data path through a
model.

An operator can answer: **what is running, who submitted what, which exact
source and policy were tested, what passed, why was an artifact blocked, who
possesses it now, and which evidence authorizes a publication?** A researcher
cannot query those questions except for source/task information explicitly
provided in its offline capsule.
