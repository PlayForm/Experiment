# Build order and local autonomous operating playbook

## Phase 0: freeze and characterize

Lock a known PlayForm/Run source commit; document actual CLI semantics and
process invocation; freeze correctness oracles and baseline fixtures; verify
Docker host isolation, CPU/SMT/NUMA, filesystem effects and measurement noise;
run a clean B0 benchmark. No agent mutates code before this passes.

## Phase 1: coordinator, CAS and event ledger

Implement Rust workspace `lab-core`, `lab-store`, `lab-api`, `lab-broker`,
`lab-statistics`. Add hashed blobs, append-only event log with replay, task
state machine, leases, idempotency and source view capabilities. Implement
experimental v3 SQL migration as an extension of v2. Add HTTP/SSE status read
API and controller CLI (`lab status`, `lab events`, `lab verify`, `lab replay`,
`lab pause`).

## Phase 2: one patcher loop

Start one warm agent worker with task-scoped source. Task: compile a simple
behavior-preserving isolated optimization. Worker commits patch and submits JSON
proposal. Gate checks exact and structural identity, frozen oracle runs, Docker
build produces hashed binary, Hyperfine runner queues a valid paired block.
Evaluator returns scoped feedback; original logical conversation continues
through a durable snapshot. Log all TTFT/TTFB and prompt digests.

## Phase 3: add five-way fanout and novelty gate

Enable 5 workers, one frozen B0, independent domains, build in parallel,
benchmark serially on dedicated hardware. Add retry/restart, throttling and
caching; ensure no shared task contexts. Implement exact build/run cache before
attempting similarity model lookup. Keep exploratory and sealed holdout strictly
separate.

## Phase 4: dual OpenTelemetry

Instrument control plane using Rust `tracing`/OTLP. Build Rust OTLP receiver or
explicitly opt for external Collector. Add trace-off/trace-lite/trace-diagnostic
product artifacts and versioned comparison harness. Show aggregate/stage traces
but never replace raw Hyperfine truth. Measure instrumentation overhead
experimentally.

## Phase 5: GPUI dashboard

Implement campaign cockpit, two telemetry boards, correlation spine, mutation
graph, evidence microscope, agent session inspector. Connect to backend read API
over SSE snapshots/cursor stream. Separate UI host/process from benchmark host
resources. Include explicit visual badges in the actual application for trace
mode and evidence status.

## Phase 6: extend to unattended seven-day cycle

Start budgets (wall time, max events, model calls, tokens, build CPU, trial
blocks, disk). On crash rehydrate from ledger. Provide automatic checkpoint,
lease recovery, redundant CAS verification, quarantine on detector alarms, and
human release gate. Track time-to-first-verified improvement, fraction of
duplicate proposals, rollback rate and evidence gaps.

## End-to-end proof tests

1. **Duplicate replay:** same patch + same build key gives exact cache hit,
   emits reuse event, no new model/build work; a changed benchmark environment
   must not be treated as directly comparable.
2. **Restricted read:** a patcher cannot read another worktree, whole database,
   hidden fixture or raw holdout via either filesystem or API.
3. **Duplicate generation:** five fanout tasks share exact source parent and
   fixed B0; each returns a unique tracked mutation or an explicitly
   dropped/reused decision.
4. **Tamper:** change stored raw JSON or try to spoof runner result ->
   hash/attestation check fails; original signed event stays intact.
5. **Crash:** kill coordinator between blob commit and dispatch; outbox replay
   yields one accepted result and at most one current lease.
6. **Measurement hygiene:** diagnostic and release trials use different binary
   mode hashes and never contribute samples to each other's leaderboard.
7. **Privacy:** prompts and raw commands are redacted/encrypted; OTel spans
   exclude raw source paths and raw user command strings by default.
8. **Agent restart:** context rehydration from immutable history permits
   revision in a new child mutation without dependence on an in-memory model
   conversation.
9. **Holdout contamination:** feedback capsule cannot show sealed validation
   details before the configured reveal boundary.
10. **UI evidence link:** from any speedup claim reach hash-verified source, raw
    samples, oracle, hardware fingerprint and exact benchmark protocol.

## Operational rules

- Prefer `stop accepting new work -> drain -> checkpoint -> pause` over abrupt
  termination.
- Quarantine a benchmark host on sustained CPU throttling, thermal drift, disk
  contention or shared host interference. Fresh baseline measurement is required
  after recovery.
- Never permit direct patcher invocation of a benchmark or build service;
  require a signed coordinator plan and receipt.
- Preserve rejected experiments and their original prompts, while respecting
  explicit data retention and sensitive source content policies.
- Promoting to project source, public release, or binary distribution always
  requires an independent correctness suite and explicit authorized approval.
