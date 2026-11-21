# PlayForm/Run: Autonomous Adaptive Performance Engineering Platform

**Specification:** PRUN-APE/2.0-draft  
**Status:** proposed design; not an implementation or measured performance
claim  
**Primary application:** https://github.com/PlayForm/Run  
**Relationship to existing starter:** extends `prun_lab.py`, `prun-bench.json`,
and `prun-bench.schema.json` v1; no existing keys are implicitly
reinterpreted.  
**North star:** experimentally proven, reproducible, portable, workload- and
hardware-aware performance with a complete record of every tested algorithm and
mutation.

## 0. Executive intent and terminology

Build a local-first automated performance-engineering environment around
PlayForm/Run. The environment discovers bottlenecks, decomposes the application
into interchangeable components, proposes one-change-at-a-time source mutations,
compiles combinations of source revisions and compiler configurations in
reproducible Docker builder environments, executes each candidate in comparable
Docker runners, checks semantic equivalence, records raw evidence, and advances
only measured, validated improvements. Local coding agents operate concurrently
in separate Git worktrees, but the experiments that share benchmark hardware are
scheduled to protect measurement quality. The system can be left unattended with
bounded budgets, recovery, auditability, and a human-controlled promotion gate.

The **product** is an optimized `Run` executable and optional hardware-specific
distributions. The **laboratory** is a separate experimental control plane.
**Heuristic analysis** happens in the product before work is dispatched, and its
own overhead is part of every benchmark. **Autotuning** happens mainly outside
production, with a compact validated policy transferred into the product.
**Mutation** is a tracked, reviewable change to source, features, compiler
configuration, training profile, or execution-plan selection. **Lineage** is
append-only evidence linking hypothesis → parent → mutation → artifact → test →
measurement → decision → descendants.

This project must never assume that a successful benchmark proves arbitrary
command semantics or that an optimizer can make a third-party executable itself
faster. Run can optimize its own traversal, matching, scheduling, orchestration,
and output overhead; the executed child program is a separate performance domain
unless an explicit user-approved transformation is supported.

## 1. Design laws (non-negotiable)

- **LAW-001 Semantic fidelity.** Preserve exact externally observable behavior
  under the documented compatibility contract: command invocation, working
  directory, argv/shell semantics, environment, filesystem scope, path matching,
  symlinks, exclusions, order where guaranteed, output, exit statuses, signals,
  and failure handling. When the current behavior is ambiguous, characterize it
  first with executable tests and document compatibility tiers.
- **LAW-002 Total-cost accounting.** Optimize and measure _full process
  end-to-end elapsed time_ including parsing, detection, planning, walking,
  matching, spawning, child completion, output, and cleanup. Any stage-only gain
  is secondary.
- **LAW-003 Zero hidden execution.** A heuristic analyzer must not run the
  user's command as a probe, change files, or make network requests without a
  separately explicit, narrowly scoped opt-in. Metadata inspection can also be
  expensive or sensitive; time-budget and scope it.
- **LAW-004 Controlled measurement.** A result cannot be promoted without
  validated fixtures, environmental fingerprint, stable baseline, raw samples,
  statistical evidence, and a passing correctness oracle.
- **LAW-005 Independent measurement.** Build and agent coding may parallelize on
  separate resources; candidates sharing a physical benchmark resource must not
  compete during timing. Disjoint CPU sets are necessary but not sufficient for
  full isolation due to shared caches, storage, NUMA and thermals.
- **LAW-006 Append-only provenance.** Never overwrite raw result files, mutation
  records, run manifests, or source artifact hashes. Derived summaries may be
  regenerated from immutable inputs.
- **LAW-007 Single-change causal studies.** Each causal mutation lane advances
  from one parent and aims to change one conceptual variable. Multi-change
  merges get new integration experiments and must never inherit a parent's
  performance claim automatically.
- **LAW-008 Safe fallback.** Unsupported architectures, low-confidence
  classifications, planner errors, failed probes and unknown commands use the
  verified reference strategy. The control plane cannot silently alter semantics
  to win benchmarks.
- **LAW-009 Local-first, offline-resumable.** Once toolchains, images and
  dependencies are cached, the experiment loop can resume without external
  services. Every external agent/model is optional and replaceable.
- **LAW-010 Resource governance.** An unattended run has explicit maximum wall
  duration, attempts, build CPU, benchmark CPU, memory, disk, network and
  process limits; resource exhaustion pauses the campaign gracefully.
- **LAW-011 No benchmark overfitting.** Split training/development/holdout
  workloads and distinct hardware cohorts. A candidate cannot use holdout data
  to guide the next search step; maintain a final sealed validation set.
- **LAW-012 Explicit portability.** Build-host capabilities, target-host
  capabilities, container-exposed capabilities and runtime-exposed capabilities
  are different fields. `target-cpu=native` is not a distribution policy.
- **LAW-013 No unexplained magic.** Every runtime decision is inspectable using
  an opt-in `--explain-plan` facility and records model/rule version, inputs,
  confidence, selected policy, predicted benefit and fallback rationale.
- **LAW-014 Security by architecture.** Agents edit only assigned worktrees;
  they do not obtain unrestricted Docker socket, privileged runner access, host
  home-directory mounts, unbounded network access, or direct promotion rights.
- **LAW-015 Negative evidence matters.** Track regressions, crashes, flaky
  tests, poor ideas, discarded hypotheses, and failed compiler combinations as
  first-class artifacts.
- **LAW-016 Energy and reliability are objectives, not accidental side
  effects.** Capture CPU-seconds, memory, filesystem I/O, process counts,
  failure rates, and optionally power/energy alongside elapsed time.
- **LAW-017 Configs are versioned APIs.** Evolution requires schema version
  bumps, migration commands, backward-compatible reading where possible, and
  explicit unsupported-version errors.

## 2. Product-level goals, non-goals, and performance philosophy

### Required results

1. Lowest feasible overall latency for trivial commands, shallow scans, large
   scans, and real workloads; minimize both constant overhead and scaling cost.
2. Fastest correct throughput and tail latency under heterogeneous resource
   constraints; avoid winning by saturating the machine irresponsibly.
3. Explainable adaptive selection among validated traversal, matching,
   execution, scheduling and output strategies.
4. A portable baseline binary plus optional architecture-specific binaries and,
   later, runtime multiversion dispatch.
5. Reproducible discovery of per-workload and per-hardware Pareto frontiers, not
   one artificial universal champion.
6. A durable optimization catalog suitable for release notes, independent
   replication, and future machine-specific tuning.
7. Human control over unsupported or potentially behavior-changing
   optimizations.

### Explicit non-goals

- Making `git status`, `ls`, or arbitrary child commands intrinsically faster by
  only modifying Run's internals.
- Assuming GPU acceleration helps filesystem crawling or process spawning; it is
  a research candidate with a strict measured benefit gate.
- Treating Docker as a real-time or perfectly noise-free measurement
  environment.
- Rewriting shell commands, skipping child invocations, reordering observable
  side effects, or changing error semantics without an explicit alternative
  contract.
- Benchmarking directly from uncommitted mutable source without recording its
  complete state.
- Allowing autonomous code agents to merge or release unreviewed changes
  automatically by default.

### Optimization tiers

- **P0 / no-regression:** reference algorithm, semantically verified.
- **P1 / isolated local:** measured stage optimizations preserving behavior.
- **P2 / integrated:** combinations of proven stage improvements, with fresh
  full-suite validation.
- **P3 / adaptive:** cost-aware runtime planner chooses from validated
  strategies.
- **P4 / hardware-aware:** validated CPU/OS/filesystem cohorts and dispatch.
- **P5 / experimental:** advanced PGO/BOLT/SIMD and specialized accelerators,
  with fallback and independent confirmation.

## 3. Decompose the executable into replaceable stages

Target internal logical modules (actual Rust file refactoring depends on
detailed source inspection):

- `cli_contract`: parse options, preserve legacy aliases, normalize input
  without changing quoting/shell boundaries.
- `command_intent`: derive safe structural descriptors from executable name and
  supplied args; never infer hidden semantics as fact.
- `environment_probe`: bounded CPU/NUMA/memory/storage/OS/container probes, with
  immutable and per-invocation fields separated.
- `workload_fingerprint`: stable, privacy-aware, revocable workload class and
  feature vector.
- `planner`: deterministic rules or compact learned cost model; confidence
  thresholds, cost gates, explainable output.
- `walk`: reference walkdir, tuned walkdir, custom walkers, bounded producer
  pipeline, and pluggable traversal order where safe.
- `match`: glob compilation/reuse, pattern prefiltering, glob matching, path
  decoding, ignore/exclusion handling.
- `scheduler`: serial, bounded task queue, worker pool, Tokio/Rayon
  integrations, backpressure, cancellation.
- `executor`: process spawning, working directories, shell/argv compatibility,
  signals, timeouts, status propagation.
- `output`: streaming, buffering, ordering, backpressure, stderr and stdout
  handling.
- `telemetry`: opt-in diagnostic spans and counters, with negligible or
  compiled-out overhead in normal builds.
- `strategy_registry`: implementations, requirements, known-incompatible
  combinations, version identifiers.

**REQ-ARC-01** Each module must define input, output, observable side effects,
invariants, error semantics and resource ownership. **REQ-ARC-02** A stable
`ReferenceStrategy` must remain available in release builds, or in any
specialization build that can fail to meet its target assumptions.
**REQ-ARC-03** Alternate implementations use interfaces that permit per-stage
tests and whole-process tests. **REQ-ARC-04** Distinguish test instrumentation
from production instrumentation. **REQ-ARC-05** Track code size, cold-start
overhead and static initialization complexity as possible regressions.
**REQ-ARC-06** Record the degree of equivalence each optimization offers:
`exact`, `conditional`, or `experimental`; only `exact` is eligible for
automatic runtime selection under standard compatibility mode.

## 4. Heuristic analysis as a feature family, not a hardcoded switch

The runtime heuristic system is its own independently benchmarked feature group.
It must support fully controlled ablation experiments:

- `heuristic-off`: reference execution; no analyzer or planner.
- `heuristic-static`: inspect only already-parsed options and cheap
  process-visible facts.
- `heuristic-fs-bounded`: adds limited, read-only filesystem metadata sampling
  subject to a strict budget.
- `heuristic-hardware`: adds low-overhead hardware/OS/resource classification.
- `heuristic-cost-v1`: selects from a versioned, deterministic ruleset derived
  from experiments.
- `heuristic-cost-v2`: optionally uses a tiny calibrated cost model, with
  deterministic fallback.
- `heuristic-telemetry`: explains decisions, records predictions and counters;
  explicitly included in diagnostic overhead tests.
- `heuristic-shadow`: computes a plan but executes the reference plan; used for
  offline comparison and label collection.
- `heuristic-override`: allows forcing any eligible strategy to
  diagnose/reproduce outcomes.

**REQ-HEU-01** The analyzer must return
`Analysis {features, constraints, confidence, evidence, elapsed_ns, unknowns}`.
**REQ-HEU-02** The planner must return
`ExecutionPlan {stages, worker_limits, queue_limits, ordering_policy, output_policy, rationale, predicted_cost, eligibility, fallback, policy_version}`.
**REQ-HEU-03** Classifications shall include `known_cpu_bound`,
`likely_io_bound`, `short_spawn_bound`, `long_running`, `unknown`, and `mixed`,
with confidence. **REQ-HEU-04** A program name alone is never enough to assume
safe mutability or side-effect behavior. **REQ-HEU-05** There shall be no
speculative execution of any user command. **REQ-HEU-06** Static parsing must
not expand variables, resolve globbing through a shell, or change quote
semantics. **REQ-HEU-07** Metadata sampling is configurable and disabled by
default on remote/network filesystems until separately validated. **REQ-HEU-08**
The analyzer must cap depth, item count, bytes examined, syscalls, and elapsed
duration. **REQ-HEU-09** Analysis failure or time-budget exhaustion
transparently falls back. **REQ-HEU-10** A policy cannot select an algorithm
that violates required behavior or hardware compatibility. **REQ-HEU-11**
Collect optional plan explanations via `--explain-plan=json`, no default stdout
noise. **REQ-HEU-12** External environment changes invalidate cached decisions
via configurable TTL and fingerprint version. **REQ-HEU-13** Cache command
fingerprints without storing raw arguments, environment secrets, paths or
working-directory names by default; redaction must be demonstrable.
**REQ-HEU-14** Record `analysis_elapsed_ns`, `plan_elapsed_ns`,
`predicted_savings_ns`, and `observed_total_elapsed_ns`; choose adaptive
planning only if expected savings plausibly exceed analysis cost. **REQ-HEU-15**
Feature ablations must measure analyzer-only, planner-only, combined, shadow,
and fully-adaptive modes. **REQ-HEU-16** Runtime inference requires no remote
service or network connection.

### Workload feature vector (proposed)

- Command: executable family (optional local allowlist), argument count and
  approximate length, command count, user-requested parallelism,
  file-vs-directory targeting, requested output and ordering contract, shell or
  direct-exec mode (if exposed).
- Scope: root-count, observed directory density, expected depth bucket, supplied
  glob complexity, exclusion count, known match-density estimate, filesystem
  kind, root accessibility, symlink handling.
- Execution: potential child count bucket, measured spawn-overhead prior per OS,
  output-volume bucket, whether command is interactive or writes to
  stdout/stderr; unknown fields remain explicit unknowns.
- Resources: effective cpuset, cgroup CPU quota, physical vs logical core
  topology, memory headroom, NUMA topology, I/O budget, open-file and process
  limits, runtime feature availability.
- Historical prior: aggregate runtimes for the same _privacy-safe workload
  class_ on same hardware class; no persistent personal command history by
  default.

### Example policy logic (conceptual, not performance claims)

1. For a tiny, known-small traversal, prefer a low-startup-overhead serial
   walker if validated faster.
2. For many inexpensive independent child commands, use a bounded worker pool
   chosen from calibrated throughput/latency curves.
3. When output ordering is strict, reject unordered streaming variants
   regardless of performance.
4. If filesystem metadata is slow or network backed, skip expensive sampling and
   use a conservative plan.
5. If confidence is low, select reference; expose the reason under
   `--explain-plan`.

No numeric threshold belongs in source code without an accompanying experiment
ID, hardware and workload applicability range, and a test validating its
dispatch boundaries.

## 5. Algorithms and mutations to investigate

Every item below is a hypothesis queue, **not a promised speedup**:

- **Walker:** WalkDir baseline, DFS/BFS, parallel walker, directory-entry
  allocation reduction, different recursion mechanisms, OS-specific syscalls,
  batched directory processing when available, metadata avoidance, symlink
  decision caching, prune-first exclusion checks, early streaming vs full
  materialization.
- **Matcher:** compile globset once, fused exclusion/inclusion filters, literal
  fast paths, SIMD/string matching when beneficial, alternate path
  representations, binary-safe path handling, `OsStr` handling, cache matchers
  without introducing shared-state contention.
- **Scheduler:** serial, coarse-grained Rayon, asynchronous Tokio, fixed
  work-stealing pool, bounded queue, dynamic batch sizes, adaptive worker count,
  topology-aware scheduling, process launch rate limiting, resource-aware
  backpressure.
- **Executor:** spawn process without unnecessary intermediate shell only where
  semantics permit, cwd and env preparation, handle reuse where possible,
  process completion handling, launch batching without violating visibility,
  correct exit aggregation and child cancellation semantics.
- **Output:** direct streaming, channel buffering, bounded aggregation,
  guaranteed ordering modes, minimal copies, vectored writes where applicable,
  stderr/stdout separation.
- **Memory:** arena allocation, thread-local scratch, allocation-free fast
  paths, preallocation, small-vector representations, allocator swaps, reduced
  cloning, sharing immutable matcher state.
- **Runtime:** Tokio runtime variants, thread counts, scheduler init cost,
  partial async removal for tiny workloads, dual-mode fast path, rayon
  configuration, cancellation granularity.
- **Binary:** opt-level, LTO, codegen units, panic/strip settings (subject to
  correctness), target CPU, PGO, BOLT, size/speed tradeoffs, cold start and
  dynamic-library effects.
- **Architecture:** x86-64 baseline/v2/v3/v4 where supported, aarch64 variants,
  feature-detected paths, NUMA-aware experiment lanes, heterogeneous-core
  scheduling where exposed and supported.
- **GPU (future research only):** identify genuinely throughput-dominant compute
  kernels with enough batch size to amortize transfer and launch overhead.
  General directory traversal and child-process spawning stay CPU/OS tasks. GPU
  experiments require a CPU control arm and evidence of net end-to-end benefit.

**REQ-MUT-01** Every proposed mutation includes hypothesis, affected stage,
intended mechanism, baseline, expected improvement class, semantic risk,
acceptance thresholds and an explicit rollback path. **REQ-MUT-02** Mutations
are normalized into the smallest conceptual diff practical. **REQ-MUT-03**
Compiler-only mutations and source changes are distinct mutation types.
**REQ-MUT-04** A mutation may reference multiple parents for a merge but
requires a new merged-baseline measurement. **REQ-MUT-05** Failed and abandoned
branches remain queryable. **REQ-MUT-06** Generative agent edits must be
committed to private refs before benchmark construction. **REQ-MUT-07**
Combinations are tested after main effects to expose interaction effects, e.g.
bounded-spawn × glob caching × allocator; fractional factorial/covering arrays
can control combinatorial explosion. **REQ-MUT-08** Strategies and their known
invalid combinations must be registered declaratively rather than relying on
compile failure as discovery.

## 6. Formal experiment identity and fixed dimensions

```
Experiment = (
    source_commit, mutation_id, lineage_parent,
    cargo_feature_set, runtime_strategy_set,
    compiler_and_linker_config, target_arch,
    pgo_or_bolt_training_profile,
    builder_image_digest, runner_image_digest,
    workload_fixture_hash, workload_command_spec_hash,
    measurement_protocol_version, oracle_version,
    host_fingerprint, resource_allocation, env_fingerprint,
    randomization_block, trial_seed
)
```

The experiment identity is a deterministic SHA-256 over canonical, sorted,
versioned metadata. The human-readable name is **not** the identity. Different
trial randomizations share a _candidate configuration ID_ but get unique _run
IDs_. Repetitions must preserve raw timing vectors. A binary's SHA-256 and the
container's OCI digest are mandatory evidence. Inherited upstream dependencies
and Cargo.lock are hashed independently.

**REQ-ID-01** Distinguish immutable logical entities: `Campaign`, `Hypothesis`,
`Mutation`, `SourceSnapshot`, `BuildRecipe`, `BinaryArtifact`, `Workload`,
`Fixture`, `HardwareProfile`, `RunnerProfile`, `Experiment`, `Trial`, `Sample`,
`OracleResult`, `Analysis`, `Decision`, `Release`. **REQ-ID-02** Enforce foreign
keys and versioned ID types. **REQ-ID-03** For source changes capture patch
hash, committed parent SHA, changed-file list, tooling/agent version, patch
prompt/rationale, and a Git ref. **REQ-ID-04** Preserve Git SHA and
platform-specific artifact digest. **REQ-ID-05** Capture scheduling order,
warming state, env, cpuset, NUMA allocation, kernel/compiler versions,
Docker/OCI version and fixture content hash. **REQ-ID-06** Results are
immutable, summaries are derived and versioned. **REQ-ID-07** Provide schema
migrations and integrity checks, including missing-artifact detection.
**REQ-ID-08** Use deterministic workload IDs and semantic versioning for
oracles. **REQ-ID-09** Do not interpret `unknown` environmental values as equal.
**REQ-ID-10** Record data-retention and redaction metadata; keep raw
stdout/stderr only in explicitly approved fixtures.

## 7. Benchmark matrix and workload taxonomy

A benchmark suite must separate external CLI behavior, internal stages, and
learned planner decisions:

- **Level A: zero-work overhead.** `--help`, invalid arguments (if compatible),
  single no-match traversal, trivial command execution, a single matching
  directory. Cold start matters here.
- **Level B: synthetic controlled scaling.** Depth × breadth × match density ×
  file count × symlink topology × metadata cost × exclusion complexity × output
  size × number of commands.
- **Level C: realistic application.** Git operations on synthetic local
  repositories, metadata commands, CPU-bound child tasks, I/O-bound child tasks,
  controlled output-producing scripts, errors/cancellations.
- **Level D: stress/adversarial correctness.** Permission errors, symlink loops,
  odd filenames, huge fanout, mixed file/directory, directory mutations during
  traversal, nonzero child exit, subprocess trees, timeouts, backpressure, file
  descriptor exhaustion, cgroup limits.
- **Level E: environmental diversity.** tmpfs, container overlayfs, host
  ext4/XFS/btrfs when available, spinning disk/SATA/NVMe where available,
  network FS only with dedicated test controls, container vs host cross-checks,
  warm and cold page cache policies.
- **Level F: hardware generalization.** CPU family/model/microarchitecture
  cohort, architecture, core and SMT layouts, different memory bandwidth and
  NUMA, VPS and dedicated bare metal.
- **Level G: policy generalization.** Planner feature-ablations, wrong-prior
  stress, ambiguous commands, unseen trees, unseen hardware, mismatched
  performance histories and policy fallback.

**REQ-WLD-01** All destructive or writable child benchmarks operate on
disposable per-trial sandbox copies; never the developer's source tree.
**REQ-WLD-02** Exact oracle expectations include matched path set/multiplicity,
spawn count, per-child cwd, argv, ordering if promised, output data, exit
result, effects and errors. **REQ-WLD-03** Fixture creator and oracle hashes are
retained; random inputs store seeds. **REQ-WLD-04** Use a
training/development/holdout split with immutable split assignment and no agent
access to hidden holdout labels during search. **REQ-WLD-05** Workload weights
are declared up front and sensitivity-tested; there is no cherry-picking.
**REQ-WLD-06** Each scaling experiment varies one driver while holding others
constant. **REQ-WLD-07** Include null experiments expected to show no benefit.
**REQ-WLD-08** Different file-system implementations are separate cohorts, never
pooled implicitly. **REQ-WLD-09** Include multi-command invocation and `Run`'s
serial-vs-parallel flag behavior. **REQ-WLD-10** Include regression tests for
non-ASCII path bytes on platforms where applicable. **REQ-WLD-11** Include
expected child oversubscription cases, including commands which are themselves
multithreaded. **REQ-WLD-12** Label cold-cache tests with an actual
measured/controlled cache-reset method; do not silently assume a cache is cold.
**REQ-WLD-13** Reference execution and mutated execution must use equivalent
fixtures, not different copies with different contents or mounts.

## 8. Correctness and equivalence oracles

A passing build or one `bench-signal` invocation does not imply equivalence.
Define a typed `Oracle` interface returning `Pass | Fail | Inconclusive` with a
rich diff. For deterministic fixtures compare paths and order under the
documented guarantee, child cwd and argv, per-child stdout/stderr, aggregated
statuses, signal handling, symlink/exclusion behavior, filesystem effects and
exit code. For nondeterministic concurrency compare only the contractually
unordered observations, using sorted multisets or happens-before constraints as
appropriate. Preserve stdout/stderr interleaving only when promised by the
baseline contract; otherwise specify admissible behavior.

**REQ-COR-01** Reference behavior is captured before optimization;
contradictions in reference behavior are escalated, not optimized away silently.
**REQ-COR-02** The oracle uses independent instrumentation and disposable
fixture copies. **REQ-COR-03** Differential testing, metamorphic testing,
property testing and fuzzing are pluggable. **REQ-COR-04** Always run a
preflight oracle on every build, extended oracles on candidate promotion,
stress/fuzz suites on integration. **REQ-COR-05** Never promote a candidate with
a failing correctness oracle, sanitizer/crash failure, undefined behavior, or a
confirmed security regression. **REQ-COR-06** Feature interactions trigger
integration-level equivalence tests. **REQ-COR-07** File race tests are
classified `non-deterministic` with documented allowed outcomes rather than
forcing unrealistic byte-for-byte equivalence. **REQ-COR-08** Keep
platform-specific oracle variants for Windows/macOS/Linux if cross-platform
product compatibility is required.

## 9. Measurement protocol and statistics

Timing truth hierarchy: (a) full command elapsed time as user experiences it,
(b) CPU and syscall counters for explanatory diagnosis, (c) stage-level
microbenchmarks for optimization leads. A specialized microbenchmark may
identify a hot path but cannot by itself prove product improvement. Builder time
and Docker image startup remain excluded from normal product benchmarks, but
container startup can be measured in a separate deployment-latency suite if
required.

- Use Hyperfine in a persistent pre-created runner with version-pinned options
  and raw JSON. Use an independently versioned parser to accommodate JSON schema
  changes. Execute the binary directly where possible rather than adding
  unintended shell overhead.
- Pair each candidate with a fresh baseline block. Randomize or counterbalance
  order, e.g. ABBA/BAAB blocks, across a stable benchmark window. Rebaseline
  periodically to observe drift.
- Ensure separate CPU families, hyperthreads, shared caches and NUMA
  interference are accounted for. `--cpuset-cpus` is an affinity constraint, not
  a guarantee of exclusive core ownership.
- Warm up both binary, filesystem and child workload according to the specified
  cache policy. Record readahead/cold cache assumptions and whether work changes
  the fixture.
- Measure medians, means, dispersion, raw samples, paired speedup, interval
  estimates and failure rates. Use robust estimators and confidence intervals
  (e.g. paired bootstrap) defined in a protocol version. Account for multiple
  comparisons; adaptively selected winners require an untouched confirmatory
  holdout.
- Detect and classify thermal drift, frequency changes, CPU steal, cgroup
  throttling, I/O saturation, memory pressure, major page faults and
  environmental interruptions. Mark contaminated trials, keep them, and
  reschedule with a documented reason.
- Use `perf stat`, system counters, tracing/flamegraphs in **separate diagnostic
  runs** so instrumentation does not bias primary timing.

**REQ-BEN-01** Primary metric: wall-clock end-to-end elapsed time; record
planning and analysis time. **REQ-BEN-02** Also collect CPU user/sys, RSS peak,
context switches, page faults, I/O bytes, spawn count and output bytes where
supported. **REQ-BEN-03** Require minimum repeats and warmups per workload, with
optional sequential stopping rules predetermined. **REQ-BEN-04** A performance
victory has a predeclared minimum practical effect size, confidence requirement,
acceptable regression guardrails, and holdout validation. **REQ-BEN-05** Report
per-workload speedups AND aggregate geometric mean with declared weights, along
with p95/p99 and worst-case regressions where sample quality supports these.
**REQ-BEN-06** Never compare debug and optimized release as interchangeable
candidates. **REQ-BEN-07** Mark noisy or insufficient-power results
`inconclusive`, not `win`. **REQ-BEN-08** Store binary size and memory
regressions. **REQ-BEN-09** Include an untouched reference implementation as a
continuous anchor. **REQ-BEN-10** Explicitly account for chance winners when
exploring hundreds or thousands of mutations. **REQ-BEN-11** Do not let policy
train on validation/holdout performance. **REQ-BEN-12** Label measurements by
hardware and filesystem cohort. **REQ-BEN-13** CPU profiling must not silently
require privileged host access; provide capability-detection and graceful
degradation.

### Success criteria (configurable defaults, not universal truths)

- Candidate semantic oracle passes fully; otherwise fail regardless of speed.
- Median paired overall speedup must exceed campaign-configured threshold, for
  example 2%, with a conservative confidence interval supporting a positive
  practical effect.
- No critical workload may regress beyond a declared threshold, for example 5%,
  without a product-level, explicit tradeoff decision.
- Runtime analysis cost must be fully included; adaptive strategy is selected
  for release only if improvement persists against the `heuristic-off` arm.
- Repeat final confirmations in different time blocks, on at least one
  independent held-out workload family; optionally on another machine in the
  target hardware class.

## 10. Optimization objectives and search strategies

There is no universally optimal single scalar. Maintain a Pareto frontier per
hardware+workload cohort with dimensions: wall time, CPU-seconds, peak RSS, p95
latency, code size, spawn count, optionally energy, plus feasibility
constraints. Provide explicit policy objectives: `latency`, `throughput`,
`balanced`, `resource_saver`, and `compatibility`. A scalar promotion score is
only a view over the Pareto front and must include its weights and constraints
in provenance.

Search ladder:

1. **Characterize:** profile untouched reference; measure variance; rank largest
   controllable cost centers.
2. **Ablate:** remove one subsystem's extra work or replace it with a minimal
   baseline; estimate overhead floor.
3. **Hypothesize:** agent describes causal mechanism and testable expected
   result.
4. **Local search:** one conceptual change per branch, low-cost smoke and
   focused tests.
5. **Confirmation:** broad end-to-end suite and repeat paired measurements.
6. **Interactions:** factorial or pairwise selection only among already good and
   semantically compatible variants.
7. **Model fitting:** learn policies from development data; evaluate on sealed
   holdout.
8. **Transfer:** validate hardware cohort and build architecture dispatch.

Algorithmic search can begin with deterministic grid/random/successive-halving
methods. Later, a persistent study engine (e.g., Optuna) may choose
compiler/queue/worker parameters, but it must respect configuration constraints,
recording of discarded trials, reproducibility and the distinction between
exploration and confirmation. Do not let model-driven suggestions silently
mutate semantic constraints.

## 11. Agents: autonomous yet controlled seven-day experimentation

The orchestration plane owns the lifecycle; coding agents are **replaceable
workers** that never directly decide benchmark truth. Example roles:

- `Scout`: source inventory and hot-path profiling; creates an evidence-backed
  hypothesis backlog.
- `Decomposer`: extracts module boundaries and reference interfaces while
  retaining behavior.
- `Mutator`: implements one scoped algorithm variation in its isolated Git
  worktree.
- `Oracle`: creates or expands independent semantic tests and adversarial cases.
- `Builder`: compiles pinned artifacts (may run in parallel with other
  builders).
- `Runner`: schedules exclusive measurement leases and collects raw timings.
- `Analyst`: performs paired comparisons, confound detection, ablation, causal
  attribution and statistical decisions.
- `PlannerTuner`: trains/updates bounded dispatch rules or models from
  development-only evidence.
- `Integrator`: attempts validated combinations; merge integration is an
  independent experiment.
- `Curator`: produces summaries, manifests, decision history and promotion
  requests.

**REQ-AGT-01** Coordinator uses durable state (not in-memory asyncio tasks
only). **REQ-AGT-02** Every task has an idempotency key, allowed worktree and
exact tool capability set. **REQ-AGT-03** Lock the measured hardware via a lease
with TTL, owner, heartbeat, and recovery after orphaned workers. **REQ-AGT-04**
Agent runs in its own nonprivileged container or strictly isolated local
worktree. **REQ-AGT-05** No agent edits the main branch directly. **REQ-AGT-06**
Agent retry limits, failure taxonomy and backoff are explicit. **REQ-AGT-07**
Persist events and flush artifacts before moving state. **REQ-AGT-08** One
conceptual patch per mutation is required; broad refactors are separate
campaigns. **REQ-AGT-09** CI and correctness gates run before expensive
benchmarks. **REQ-AGT-10** Baseline regression and holdout confirmation are
central services, not agent self-reported text. **REQ-AGT-11** Integrator only
merges valid source commits and creates a new lineage node. **REQ-AGT-12**
Support pause/resume/crash recovery and host reboot. **REQ-AGT-13** A human can
inspect and veto any branch, mutation, strategy, workload or release.
**REQ-AGT-14** Agents log prompts, tool invocations, versions and decisions with
redaction and retention controls. **REQ-AGT-15** Automatic release is off by
default. **REQ-AGT-16** Budget quotas: global time, wall-time per experiment,
attempts per hypothesis, cumulative CPU time, maximum concurrent agents/builds,
free disk thresholds, memory and network limits. **REQ-AGT-17** Agents must not
update the benchmark definition or correctness oracle to make their own mutation
pass without an independent approval pathway. **REQ-AGT-18** A candidate cannot
promote itself; only coordinator-owned validation and decision code can.

### Durable state machine

`PROPOSED → SCOPED → WORKTREE_CREATED → PATCHED → STATIC_CHECKED → TESTED → BUILT → ORACLE_PASSED → QUEUED → BENCHMARKED → ANALYZED → CONFIRMED → ELIGIBLE → REVIEWED → INTEGRATED`

Side exits: `REJECTED`, `INVALID`, `INCONCLUSIVE`, `RETRY_PENDING`, `PAUSED`,
`QUARANTINED`, `ABORTED`. Every transition emits an immutable event with actor,
artifact digest, timestamp, previous state, next state and reason. An
interrupted `BENCHMARKED` state cannot be assumed successful without checking
completed output integrity.

### A safe unattended campaign

Before execution: freeze source base, runner image, benchmark protocol, fixture
hashes, baseline and objective thresholds; verify free disk, network policy,
host temperature and hardware status; prefetch dependencies and pin toolchains.
During execution: generate hypotheses; prioritize by expected gain, confidence
and implementation cost; allocate one branch/worktree per mutation; build/test
concurrently; queue measurements exclusively; compare; confirm; retry or reject;
record; checkpoint. At completion: summarize Pareto frontier and remaining
opportunities, include `reproduce` commands for every proposed winner. If
budgets or system health are breached: checkpoint and stop cleanly, not exhaust
the host.

## 12. Docker and local architecture

Maintain separate container responsibilities:

- **`prun-builder`**: pinned Rust toolchain, Cargo dependency cache, LLVM tools,
  target triples, reproducible source snapshot and artifact extraction. Profiles
  are not presumed reproducible across compiler and target changes.
- **`prun-runner-release`**: minimal fixed benchmark runtime; candidate binaries
  mounted read-only; stable fixtures read-only except controlled writable
  workload copies; Hyperfine and lightweight metrics collector.
- **`prun-runner-diagnostic`**: `perf`/tracing/strace/flamegraph tools when
  allowed; isolated from timing runs.
- **`prun-runner-training`**: PGO instrumented binary execution on _training_
  workloads, writes `.profraw` to a controlled location; merges with matched
  LLVM tooling before final optimized build.
- **`prun-agent-worker`**: restricted source coding environment with only
  assigned worktree; no unrestricted Docker host socket.
- **`prun-coordinator`**: preferably a host-side least-privilege service
  managing jobs, database, logs, artifact CAS, CPU and disk leases; may run in a
  container only with carefully restricted permissions.

Builder and runner images are **invariants within a fair comparison block**, and
**variants across explicitly separate infrastructure studies**. Image digests
are pinned; `latest` and implicit network upgrades are forbidden during a
campaign. Use Docker Buildx Bake for reproducible build matrices where useful;
its parallel builds are not a substitute for benchmark scheduling. Building uses
Git commits or verified source snapshots; no live bind mounts of mutable code in
timing containers. The runner's benchmark starts after container initialization
and fixture preparation; Docker startup is not part of normal command latency.

**REQ-DOC-01** Record builder and runner OCI digests, architectures, host and
container CPU feature exposures. **REQ-DOC-02** Fix cgroup configuration,
cpuset, mems, pids, ulimits, mounts, capabilities and environment within a
comparable block. **REQ-DOC-03** Never share writable fixture volumes between
live trial groups. **REQ-DOC-04** Privileged access, if required for diagnostic
perf counters, is separate, explicit and never granted to agent coding
containers. **REQ-DOC-05** CPU contention detection includes shared host
resources (SMT sibling, storage, memory, power), not just cpuset overlap.
**REQ-DOC-06** Define offline and network-enabled fixture classes explicitly.
**REQ-DOC-07** Track images and artifacts via content-addressed names.
**REQ-DOC-08** Container creation and process-run measurement boundaries are
recorded in manifests. **REQ-DOC-09** Bind-mount visibility, overlayfs and mount
propagation behavior must not differ between candidates in a paired comparison.
**REQ-DOC-10** Disable agent/build activity on benchmark-isolated cores and
disks during measurement, or model and record residual interference.

## 13. Hardware fingerprinting and specialization

Keep three independent profiles: **physical host**, **effective execution
environment** (VM/container/cgroup) and **compiler target**. Record processor
vendor/family/model/stepping and ISA feature set where available, core topology,
SMT and NUMA, cgroup CPU quota and affinity, available RAM, cache hierarchy,
OS/kernel, filesystem and storage type, microcode when accessible,
temperature/frequency/energy permission flags, and optional GPU
model/driver/backend. Sensitive machine IDs, hostnames, serials and raw personal
paths are excluded by default. Hash machine-local identifiers only with consent
and documented retention.

Specialization modes:

1. **Portable baseline**: guaranteed supported ISA and reference runtime
   dispatch.
2. **Build-target specialization**: explicit target triple/CPU and feature
   flags, only for a declared deployment target. Build-script detection of the
   _builder_ must not silently substitute for the _target_.
3. **Runtime multiversioning**: a portable dispatcher selects versions compiled
   with `#[target_feature]` or separate architecture-specific modules/binaries
   after verifying runtime support; always keep safe fallback.
4. **Machine-local installer**: optional user-approved `prun tune --local`
   compiles/installs locally after checking target and running a bounded
   benchmark verification. Source code/toolchain provenance is shown.
5. **Profile-guided specialization**: instrumented training, matched profile and
   rustc version, optimized rebuild, holdout confirmation; optionally BOLT on
   supported target formats/toolchains.
6. **Accelerator plugin (research)**: GPU enumeration, backend capability,
   transfer cost, data locality, launch overhead and CPU fallback; never infer
   benefit from GPU presence alone.

**REQ-HW-01** Reject incompatible ISA deployment early; test explicit
unsupported-feature fallback. **REQ-HW-02** A specialized binary is tagged by
minimum ISA requirements and target OS/ABI. **REQ-HW-03** Keep per-class winners
separate, including heterogeneous CPU cores. **REQ-HW-04** Distinguish CPU-bound
Run overhead vs child process cost. **REQ-HW-05** Benchmark code size and
startup penalty from multiversioning. **REQ-HW-06** A local compilation workflow
must expose disk usage, toolchain availability and opt-in and have a portable
fallback if compilation fails. **REQ-HW-07** GPU candidates require workload
evidence that justifies data transfers and device dependency; otherwise remain
disabled. **REQ-HW-08** Train PGO on representative training workloads and
evaluate on held-out workloads. **REQ-HW-09** Record target feature detection
input and generated binary compatibility, not merely `native=true`.

## 14. Artifact storage, database, and lineage model

A local SQLite database in WAL mode is adequate for the first single-host
coordinator with controlled writes, plus immutable content-addressed files for
binaries, manifests, profiles, JSON, raw samples and logs. Use a single
transaction writer and short transactions; do not share a WAL database over
network filesystems. Later, a PostgreSQL-compatible backend and object storage
can support multi-host operation without changing logical IDs.

Append-only event stream is the system of record; normalized tables and
leaderboard/report views are derived. Artifact paths are
`objects/sha256/<first-two>/<digest>` and never keyed only by mutable names.
Write atomic temporary files, `fsync` where required, then rename before
committing the DB row. Provide a verification command to resolve dangling refs,
missing blobs and corrupted hashes. Retention policies may delete approved
temporary build caches, but must never delete essential evidence without a
recorded tombstone and human-controlled policy.

**REQ-LIN-01** Each mutation has a single causal parent when it is a one-change
test, and all merges enumerate all parents. **REQ-LIN-02** Every result links
both source and resulting binary hash. **REQ-LIN-03** Store all attempted
candidate settings, even invalid configurations. **REQ-LIN-04** Capture origin
agent ID, model/tool versions, hypothesis text, actual patch, and validation
outcome. **REQ-LIN-05** A derived decision links comparison baseline and both
sample sets, test oracle, statistical method and thresholds. **REQ-LIN-06** A
user can reconstruct any winner with `lineage show <id>` and
`reproduce <experiment_id>`. **REQ-LIN-07** Branch deletion never erases a
historical diff/artifact digest. **REQ-LIN-08** The logical evidence DAG
supports causal linear views per lane, parallel branch views, a merge graph and
per-stage chronology. **REQ-LIN-09** Keep architecture classes and workload
versions explicit in every leaderboard. **REQ-LIN-10** Raw measurements are
immutable and addressed by hash; summaries can be refreshed when analytics logic
improves.

### Minimal lineage proof chain

```
Campaign C
  └─ Hypothesis H (why change might help)
      └─ Mutation M (one conceptual patch; parent commit P)
          └─ Source S (immutable Git commit, patch digest)
              └─ Build B (recipe digest, target, toolchain, features)
                  └─ Artifact A (binary hash, OCI digest)
                      └─ Oracle O (semantic diff/pass result)
                          └─ Experiment E (fixture + host + runner protocol)
                              └─ Trial T_1 ... T_n (raw measurements)
                                  └─ Analysis X (paired estimate, CI, confounds)
                                      └─ Decision D (reject / inconclusive / promote)
                                          └─ Next mutation or integration request
```

The graph is not artificially forced into a single global line. Each causal lane
is linear, and an append-only ancestry DAG records forks and joins. A benchmark
result never becomes the parent of source code; rather, a new mutation
references its predecessor and the analysis that motivated it.

## 15. Interfaces and single-command automation

Proposed local controller CLI, initially one entry-point application with
versioned manifest and agent playbooks:

```
prun-lab init
prun-lab doctor --profile dedicated-linux
prun-lab inspect-source --ref Current
prun-lab decompose --stage walk --reference Current
prun-lab plan --manifest experiments.yaml
prun-lab fixtures generate --suite core
prun-lab baseline capture --suite core --host local
prun-lab feature list
prun-lab hypotheses list --stage scheduler
prun-lab mutate --hypothesis H-004 --agent local
prun-lab campaign start --playbook playbooks/autonomous-week.yaml
prun-lab campaign status --follow
prun-lab campaign pause|resume|stop <campaign-id>
prun-lab experiment reproduce <experiment-id>
prun-lab compare <baseline-id> <candidate-id> --paired
prun-lab lineage show <mutation-id> --ancestors --descendants
prun-lab leaderboard --hardware <class> --workload <suite>
prun-lab strategy train --data development --holdout sealed
prun-lab report --campaign <id> --format markdown,html,json
prun-lab release propose --candidate <id>
prun-lab garbage-collect --dry-run
prun-lab verify-artifacts
```

These are **design-contract CLI examples**, not commands supported by the
starter today. The current starter supports `init`, `plan`, `build`, `bench`,
and `all`. Preserve and wrap those commands while introducing the new interface;
don't claim they already exist.

Proposed product CLI (also not yet implemented):

```
Run --Pattern .git --Command 'git status' --strategy auto
Run --Pattern .git --Command 'git status' --strategy reference
Run --Pattern .git --Command 'git status' --explain-plan=json
Run --Pattern .git --Command 'git status' --strategy scheduler-bounded
Run --version --build-info
```

Avoid changing the legacy default until equivalence and benefit are proven; the
initial adaptive option should be opt-in or shadow mode.

**REQ-CLI-01** Stable machine-readable JSON output for plans, results, queries
and provenance. **REQ-CLI-02** Human-readable reports include workload-specific
evidence and failed experiments. **REQ-CLI-03** Direct CLI and optional
`cargo test bench` wrappers target the same coordinator; ordinary `cargo test`
must not start a week-long campaign. **REQ-CLI-04** A local conversation agent
is merely an API client; a durable coordinator persists beyond chat disconnects
and process restarts. **REQ-CLI-05** Dry-run and cost projection exist for all
campaigns. **REQ-CLI-06** All commands distinguish `designed`, `implemented`,
and `verified` capabilities in status output.

## 16. Runtime planner cost model

Separate _feasibility_ from _cost_:

```
Eligible(plan, workload, hardware, semantics) ∈ {yes, no, unknown}
T_total(plan) = T_parse + T_probe + T_plan + T_walk + T_match
              + T_queue + T_spawn + T_children + T_output + T_cleanup
```

Terms can overlap in pipelined execution; use the sum as an explanatory
decomposition only when stages are measured without overlap, and use actual
critical-path wall time as the decision objective. A fitted performance model
can estimate `T_total` or differences using sampled features and confidence
intervals, but output remains a prediction, never presented as a measured
guarantee. Runtime planner has a hard decision deadline (configurable, suggested
initial budget in microseconds or a fraction of baseline) and an explicit
fallback plan. For nontrivial commands a small expensive probe may be
worthwhile; for trivial commands no extra work should be required.

Evaluate `regret = T_chosen - T_oracle_best_eligible` on development/holdout
suites, plus classifier calibration, misclassification severity, overhead and
fallback frequency. Oracle-best is an offline benchmark reference, not
accessible to the production runtime. Report how often a heuristic is faster
than the best fixed strategy, not merely faster than the slowest baseline.
Measure the overhead of reading and loading rules/models, cold and warm cache,
and cost of plan explanation (diagnostic only).

## 17. Experimental feature flag model

Cargo features should be mostly additive and represent available
**capabilities**, not necessarily a mutually exclusive active behavior. Actual
selection of one active walker or one scheduler belongs in runtime strategy
config, compile-time `cfg` configured by distinct build recipes, or explicit
mutually exclusive, validated strategy groups. Never assume
`--features walker-a,walker-b` means the test will select both or that
conflicting options are legal; Cargo's feature unification and additive design
require separation of concepts.

Feature dimensions:

- `capabilities`: implementations compiled into artifact (`walker-reference`,
  `walker-v2`, `scheduler-bounded`, `planner-static` ...)
- `selection`: chosen runtime strategy or policy (`reference`, `static-v1`,
  `cost-v2` ...)
- `build`: Rust toolchain, profile, target, flags, LTO, allocator, PGO/BOLT
  training profile
- `measurement`: timing instrumentation, tracing, perf counter collection,
  warmup protocol
- `constraints`: valid OS/ISA, required permissions, preserved semantic tier,
  incompatible combinations
- `provenance`: source ref, mutation ID, parent, experiment, evidence, approval

**REQ-FEA-01** Declare feature groups and exclusivity outside Cargo in
experiment manifests. **REQ-FEA-02** A feature combo must compile and pass its
own oracle. **REQ-FEA-03** Feature IDs and strategy IDs are versioned.
**REQ-FEA-04** An ablation record must identify exactly which capabilities were
active, which were built but unused, and which decisions the planner made.
**REQ-FEA-05** Ensure compiler-optimized-away code paths are tested by explicit
instrumentation or targeted execution tests. **REQ-FEA-06** Unknown/deprecated
features fail validation rather than being silently ignored.

## 18. Security, permissions, and unattended operation

The coordinator is trusted infrastructure; agents and generated code are
untrusted inputs. Use local disposable Git worktrees and write restrictions;
network disabled in bench runners by default; mount read-only except declared
scratch/output paths; drop capabilities and use seccomp/AppArmor where feasible;
no secrets or SSH keys in build contexts; no Docker socket exposed to coding
agents. The coordinator mediates all `docker build`, `docker run`, and source
snapshot operations. Treat user-supplied commands and files as untrusted and
avoid shell interpolation in the controller; encode commands as structured argv
or an explicitly declared shell command with quoting untouched by the planner.

Unattended week-long agents can make mistakes even if code compiles; define
approvals and irreversible-change protection. Agent source changes must never
disable or rewrite correctness tests as an unreviewed workaround, modify
immutable fixture digests, broaden permissions, or promote themselves. Disable
autonomous remote package updates and external publishing unless explicitly
granted. Maintain a stop signal and terminate orphan child processes. Agent
prompts must state what is allowed and not allowed, include the oracle contract,
and require a short hypothesis and expected effect.

**REQ-SEC-01** Profiles for `restricted`, `trusted-local`, and
`human-supervised` operation. **REQ-SEC-02** Default: no secrets, no network
access in runners, no privileged containers, no host Docker socket in agent
containers. **REQ-SEC-03** Explicit workload allowlist; arbitrary shell commands
only under user-authorized sandbox policy. **REQ-SEC-04** Reproducible test
fixture copies; never run destructive workloads against arbitrary user path.
**REQ-SEC-05** Observability and logs redact sensitive paths/arguments.
**REQ-SEC-06** Artifact signature/integrity checks before release.
**REQ-SEC-07** Manual merge/release by default. **REQ-SEC-08** Fail-closed on
incompatible sandbox or missing permissions. **REQ-SEC-09** Record the exact
human approval and associated scope in lineage. **REQ-SEC-10** Emergency stop
cancels future tasks, stops active containers, leaves durable event history, and
preserves partial evidence.

## 19. Progression and acceptance gates

**Phase 0 — Evidence baseline:** snapshot upstream, capture CLI behavior,
commit/source hashes, create fixtures, set reproducible runner and benchmark
protocol. _Gate:_ two repeated baseline windows agree within configured noise;
no known missing required oracle.

**Phase 1 — Characterize and decompose:** add detailed profile stage tracing
behind a diagnostic build, characterize cost center, extract reference modules,
build correctness oracle. _Gate:_ refactor matches reference behavior; no
unacceptable performance regression; baseline build still available.

**Phase 2 — Mutation lab:** one-worktree-per-agent, mutation registry, build
matrix, immutable result store, single-parent experiment lanes. _Gate:_
automatic edit → build → oracle → benchmark → verdict works locally and survives
forced termination/restart.

**Phase 3 — Heuristic feature family:** static and hardware features, shadow
planner, plan introspection, ablation suite, safe reference fallback. _Gate:_
zero semantic regressions and measured positive total-runtime value on holdout;
planning overhead under threshold.

**Phase 4 — Persistent autonomy:** idempotent scheduler, leases, budgeted
unattended campaigns, safe agent sandbox, dashboard and daily evidence
snapshots. _Gate:_ fault-injection tests (agent crash, runner crash, DB lock,
full disk threshold, host restart) recover with no orphaned mutation or lost raw
samples.

**Phase 5 — Search and combination:** interaction matrix, Pareto archive,
Optuna-like model-backed search, confidence controls, sealed holdout and
independent remeasurement. _Gate:_ winners confirmed on fresh workload data and
paired baselines.

**Phase 6 — Hardware specialization:** hardware catalog, explicit cross-target
builds, runtime ISA compatibility checks, machine-local build option, PGO and
BOLT experiments. _Gate:_ portable baseline works everywhere supported;
specializations never select unsupported instructions and outperform baseline on
target cohort.

**Phase 7 — Production release:** signed artifact metadata, reproducible build
instructions, understandable explain-plan CLI, opt-in tuning and fallback
policy, explicit compatibility statement. _Gate:_ reproducible evidence for each
advertised speedup, review of behavior and sandbox security, rollback release
ready.

### Acceptance scenarios (must be automated)

- Same mutation built twice with identical recipe produces auditable binary and
  build hashes; if bit-for-bit reproducibility is not guaranteed, record all
  inputs and mark the limitation.
- Reject a mutation that skips command invocation even if 10x faster.
- Detect a 1% change as inconclusive when noise exceeds signal; resist p-hacking
  after many trials.
- Restore the coordinator after a crash during a benchmark: incomplete trials
  are retained but not promoted.
- Detect a shared SMT sibling or competing build process as an invalid benchmark
  environment or a separate cohort.
- Show complete ancestry and one-command reproduction for any promoted
  candidate.
- Force the heuristic planner into `unknown` and observe correct reference
  fallback without side effects.
- Reject a CPU-specific binary on a host lacking the required ISA, choose
  portable alternative.
- Validate analyzer runtime is included in end-to-end timing; win disappears if
  analyzer overhead erases speedup.
- Detect overfitting: development winner fails fresh holdout and is not
  promoted.
- Build candidate on one architecture, run on another compatible architecture;
  never infer build target from container host alone.
- Pause a week-long campaign at configured budget, preserving DB, patches,
  trials and artifact integrity.

## 20. Threats to validity and unresolved research questions

1. What does upstream guarantee about command order and stdout/stderr
   interleaving, if anything? Characterize with source and tests before changing
   scheduler.
2. Does `Run` execute command strings through a shell, tokenization, or direct
   `Command` argv? The planner must preserve actual behavior; no rewriting until
   characterized.
3. What does upstream guarantee for errors, symlinks, exclusions, filters and
   concurrent directory changes? The oracle needs precise contracts.
4. How much of observed elapsed time is child execution versus Run overhead on
   representative real workloads? Without this, there is no justified planner
   budget.
5. Does the local target platform expose reliable CPU/NUMA and perf counters, or
   only container-virtualized views? Log capabilities and uncertainty.
6. Is a high-throughput filesystem benchmark dominated by page cache, overlayfs,
   disk seek, per-directory syscalls or application logic? Separate cohorts.
7. Can one build recipe support per-machine specialization without locking the
   installer to the builder's CPU? Prefer explicit target specification and
   runtime dispatch.
8. Do small policy models outperform a few transparent thresholds after
   accounting for inference cost and generalization? Verify empirically.
9. Which benchmark result summaries must be human-readable versus
   machine-queryable? Both are required; choose a canonical machine schema.
10. What is the minimum performance gain worth complexity, maintainability and
    support risk? Set campaign-specific promotion gates.
11. How should cross-platform support be maintained when initial measurement lab
    is Linux Docker? Preserve contract and separate test infrastructure for
    Windows/macOS.
12. When does an optimization affecting output ordering constitute a new
    explicit mode versus a transparent improvement? Always require
    contract-specific opt-in when changing observable behavior.

## 21. Reporting, discoverability, and productization

Required reports: source decomposition map; baseline bottleneck breakdown;
hardware profile; benchmark fixture catalog; candidate comparison; paired raw
distribution plots; Pareto frontier; feature ablation matrix; policy
confusion/regret and calibration; lineage history; regression dashboard;
rejected hypothesis index; by-machine release recommendations; weekly autonomous
campaign journal; final release evidence pack. Every report links to
machine-readable IDs, not just prose. Offer reports for `one commit`,
`one stage`, `one hardware cohort`, `one workload`, `one campaign`, and
`entire lineage`. A future visualization layer may render lineage and
performance frontiers interactively, but reports must remain useful in plain
Markdown/JSON offline.

### Release-level claims need exact scope

Bad: “Run is 10x faster.”  
Good: “On host class X, fixture suite Y v3, under runner protocol Z, variant A
reduced median overall elapsed time against reference B by N%, with confidence
interval [L,U], while all required semantic oracles passed.”

## 22. Bare-bones code decomposition, disassembly, and change-level observability

Because the application is open-source Rust, start from its **source**, not from
decompilation. Disassembly, DWARF symbol maps, compiler optimization reports,
perf samples and generated machine-code comparisons are useful later to explain
_why_ a measured change worked. Implement a reproducible pipeline:

1. **Source inventory:** per Rust module, function, generic instantiation,
   dependency, lines-of-code class, cyclomatic or structural complexity, and
   hot-path entry point. Map every public CLI option to affected control-flow
   branches and side effects.
2. **Call graph and dataflow:** identify parsing → matching → walking →
   scheduling → process management → output edges, allocations and
   synchronization primitives. Keep source-level and optimized-binary-level
   graphs distinct because inlining and monomorphization change structure.
3. **Phase accounting:** `hyperfine` for end-to-end, `perf stat` for counters,
   flamegraphs/perf sample profiles for hotspots, function-level Criterion/Divan
   or iai-callgrind style microbenchmarks for isolated kernels. Instrumentation
   overhead must be measured and labeled.
4. **Syscall/cost audit:** count directory opens/reads, metadata operations,
   spawn/exec/wait syscalls, context switches and allocation patterns where
   diagnostic tooling permits; distinguish application-induced activity from
   child activity.
5. **Machine-code audit:** record compiler flags, target ISA, symbol/debug
   mapping, code size and assembly diffs (e.g. `llvm-objdump` or `cargo asm` in
   a compatible environment), outlining/inlining changes and branch/cache
   counter behavior. Never infer a speedup from fewer instructions alone.
6. **Delta localization:** connect mutation ID to exact file/line ranges,
   symbols, behavior-contract clauses, fixture categories, source coverage
   regions, binary symbols where resolvable and affected flamegraph stack
   frames.
7. **Dependency minimization:** experiment with replacing heavy subsystems with
   narrower standard-library or tiny alternatives, but measure cold start,
   runtime, feature loss, portability, security and maintenance. Library removal
   is not automatically a speedup.
8. **Counterfactual ablations:** bypass each stage or substitute a no-op test
   double only in a clearly labeled _diagnostic benchmark_ to estimate the
   overhead floor. Do not use behavior-breaking no-op results in the production
   leaderboard.
9. **Performance attribution report:** every promoted mutation shall include a
   causal hypothesis, source diff, measured overall change, per-stage change if
   available, counter evidence, compiler-level notes where relevant, and
   plausible confounders.

**REQ-DEC-01** Maintain mapping
`mutation → files/hunks → functions/symbols → stages → test coverage → benchmark cases → analyses → release`.
**REQ-DEC-02** Record when a function is inlined or eliminated rather than
fabricating a direct binary symbol mapping. **REQ-DEC-03** Capture
toolchain/debug-info choices required for accurate profiles in a separate
diagnostic build. **REQ-DEC-04** A refactor that deletes functionality must be
labeled as a compatibility change unless the oracle proves preservation.
**REQ-DEC-05** Document code-size, dependency and binary startup wins separately
from wall-time gains. **REQ-DEC-06** Leave detailed disassembly and heavy
diagnostics outside the primary Hyperfine measurement path.

### Upstream documentation inconsistency to characterize

As reviewed on November 20, 2026, the repository README's performance examples
write `Run -P .git -C ...`, while its options list identifies `-P` as the
no-argument `--Parallel` flag and documents a separate required `--Pattern`.
Therefore, do not copy those example strings into correctness benchmarks
verbatim without confirming the real parser behavior and a specific source
commit. Make CLI characterization and README/example reconciliation an early
independent documentation or compatibility task.

## 23. Source references and further reading

- Upstream PlayForm/Run: https://github.com/PlayForm/Run
- Current Cargo manifest:
  https://raw.githubusercontent.com/PlayForm/Run/Current/Cargo.toml
- Current release profile:
  https://raw.githubusercontent.com/PlayForm/Run/Current/.cargo/config.toml
- Docker Buildx Bake and matrix: https://docs.docker.com/build/bake/matrices/
- Docker CPU/resource limits:
  https://docs.docker.com/engine/containers/resource_constraints/
- Hyperfine: https://github.com/sharkdp/hyperfine
- Cargo feature design: https://doc.rust-lang.org/cargo/reference/features.html
- Rust PGO: https://doc.rust-lang.org/rustc/profile-guided-optimization.html
- Rust code generation and runtime detection:
  https://doc.rust-lang.org/reference/attributes/codegen.html
- LLVM BOLT: https://llvm.googlesource.com/llvm-project/+/main/bolt/
- Linux CPU isolation: https://docs.kernel.org/admin-guide/cpu-isolation.html
- Git worktrees: https://git-scm.com/docs/git-worktree
- SQLite WAL: https://www.sqlite.org/wal.html
- Linux perf: https://man7.org/linux/man-pages/man1/perf-stat.1.html
- Optuna persistent studies:
  https://optuna.readthedocs.io/en/stable/tutorial/20_recipes/001_rdb.html

---

**Implementation boundary:** This file is a requirements and architecture
contract. It does not state that autonomous agents, heuristics, PGO, BOLT,
cross-architecture dispatch or week-long unattended control already exist in the
previously supplied starter. Those are phases to implement, test, and verify
against this specification.
