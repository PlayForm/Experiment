# Correctness mesh, performance protocol, and release gates

## Test authority

The original repository's supported behavior is established through a stable
trusted oracle (documented spec, reference binary, frozen fixtures,
exit/stdout/stderr/side effects/process count and ordering guarantees).
LLM-authored tests can explore new cases but remain untrusted proposals until
separately reviewed and admitted. No benchmark winner is inferred from
self-written tests.

## Stages

1. Scope control and source integrity: patch exactly allowed symbols and files;
   reject test/manifest/toolchain/benchmark policy tampering outside authorized
   experiment dimensions.
2. Offline build with pinned toolchain/dependency closure; record exact source,
   binary hash, compiler flags, target ISA, PGO/profiling inputs, build output
   and container digest.
3. Reference suite with trusted fixtures and cross-variant compatibility matrix.
4. Change-specific tests derived from mutation mechanism; compare against
   reference or semantic contract and, where relevant, trusted security/fuzz
   suites.
5. Differential/metamorphic tests: randomized and adversarial file hierarchies,
   symlinks, loops, permissions, huge/sparse directories, mixed UTF encodings,
   error propagation and signal cancellation.
6. Benchmark lease: exclusive physical CPU topology, stable machine/environment,
   pinned cache regime and dataset; scheduler verifies absence of conflicts.
7. Paired baseline/candidate randomization: run both within same reservation,
   multiple rounds; record raw ordered samples and all invalidation reasons;
   latency/CPU/RSS/throughput as separate objectives.
8. Statistical evaluation against frozen protocol: measure effect, uncertainty
   and regressions across declared workload/hardware classes; holdout used only
   after nomination.
9. Promotion eligibility: correctness/compatibility pass plus policy-defined
   Pareto and regression constraints; specialized candidates allowed when
   general promotion fails.
10. Release cohort smoke/regression re-verification: production dispatch
    overhead included in full command time.

## Exactly what constitutes a measurement

A `sample` binds binary digest, workload digest, CPU set/host fingerprint, cache
state, ordering/pair, elapsed wall time, CPU user/system, peak RSS, page faults,
process exit, output digest and protocol. A `benchmark-result` binds all
samples, environment and trusted runner attestation; it is never produced by a
model.

## Hyperfine and alternate harnesses

Hyperfine is appropriate for complete command latency; supplement with trusted
OS process counters, careful cache/fixture controls, and per-stage diagnostic
profiles from separate builds. Criterion/Divan are appropriate for low-level
Rust microbenchmarks but cannot substitute for full end-to-end behavioral tests.

## Stop and failure handling

Compilation time and LLM token usage do not enter the _optimization score_.
Physical leases, fairness and finite scheduling capacity still matter. An
inconclusive result means further samples or archiving with uncertainty, not a
silently passed gate. A host drift event invalidates samples rather than
“correcting” them post hoc without a predeclared protocol.
