# Pitfalls / threat model / required verification

## A. Untrusted source and tests

1. **Malicious build scripts**: Rust `build.rs`, macros, dependency scripts,
   linkers or language-specific hooks execute code before tests. Run builders in
   untrusted-build containment with no secrets/Internet/Docker socket; only
   pre-fetched, pinned dependencies and minimal writable volumes.
2. **Semantic regression hidden by speedup**: smaller output, fewer directories,
   skipped spawned processes or altered exit behavior may look fast. Trusted
   reference oracle, count invariants, side-effect fixtures, exit/stdout/stderr
   comparisons, cancellation and symlink cases.
3. **LLM-authored tests as oracle**: a patcher may write a test that rewards its
   own change. Record it as _proposed_, run independently, and never treat it as
   a replacement for the frozen oracle. Test promotion to the trusted suite is a
   separate governed event.
4. **Test changes in the patch**: patch may delete or weaken tests, Cargo
   features, fixtures or benchmark hooks. Pin oracle/test bundle separately from
   candidate source; reject changes outside the allowed scope.
5. **Invisible feature interactions**: testing one change in isolation misses
   interactions. Maintain feature dependency graph, targeted n-way coverage and
   release-critical exhaustive sets.
6. **Nondeterminism and flakiness**: repeated property/fuzz seeds, deterministic
   fixture snapshots, timeouts and a `flaky/inconclusive` state; never equate
   retries with evidence of success.
7. **Undefined behavior of reference**: differential equivalence may preserve a
   bug or nondeterminism. Record explicit semantic spec, deviations and spec
   revisions; preserve old compatibility mode if required.

## B. Benchmark science

8. **CPU pinning is not exclusive core reservation**: `cpuset` can still share
   sibling hyperthreads, caches or memory bandwidth. Require physical-core
   topology, host-level scheduling controls and contamination checks.
9. **Thermal and governor drift**: record frequency policy, package temperature
   and stability before/after runs. Quarantine contaminated host windows.
10. **Cold vs warm cache**: do not erase a meaningful workload regime; declare
    both separately and measure them separately.
11. **Disk and network I/O nondeterminism**: fixed datasets and filesystems;
    prohibit uncontrolled external I/O in authority-grade runs; classify network
    filesystem tests separately.
12. **Statistical optional stopping**: agents repeatedly request new samples
    until one looks good. Predetermine confirmation sample/stop rules; do not
    use holdout as development data.
13. **Instrumentation skews runtime**: primary timed binary is trace-off;
    diagnostic build and instrumentation overhead experiments are separate.
14. **Global speedup illusion**: preserve full cohort/hardware matrix and Pareto
    frontier, including regressions and cases with no benefit.
15. **Comparison leakage**: randomized/paired baseline-vs-candidate runs on the
    same host reservation; preserve order, raw timing, CPU and RSS, invalidation
    reasons.
16. **Hyperfine alone cannot prove CPU efficiency**: collect trusted
    process-level resource counters and platform metadata in addition to elapsed
    timing.

## C. Laboratory control

17. **Forged measurements from agent**: never accept agent-originated
    `benchmark-result`; route requires runner-scoped mTLS identity and evidence
    artifact verification.
18. **Confused deputy**: a supervisor with network connectivity must not blindly
    relay raw model requests. Model writes declarative artifacts only;
    supervisor uses fixed typed ingestion rules.
19. **Signature != truth**: CA signs identities, runner signs measurements,
    evaluator signs verdict. A researcher signature only attests authorship of
    opinion.
20. **Race and duplicate jobs**: unique idempotency keys, compare-and-set
    aggregate revisions, fenced leases, signed receipts, retries with distinct
    attempt IDs.
21. **Replay across tenants or campaigns**: bind every signature to trust
    domain, subject ID, policy hash, nonce, expiry, candidate digest, fixture
    digest and experiment ID.
22. **Leaked identities in telemetry**: pseudonymous IDs in projection, no raw
    prompt, command args, path or source in metrics; metric attribute allowlist
    enforced at ingestion.
23. **Cryptographic key lifecycle**: offline root, short-lived intermediates,
    expiry/revocation, key rotation, issuance audit and independent recovery.
24. **DNS spoofing / PKI confusion**: Hickory DNS resolves service names; the CA
    verifies identities. DNSSEC does not replace mTLS authorization.
25. **Host operator confidentiality**: Docker is not an enclave. A privileged
    host operator can generally inspect processes/memory or plaintext mounts.
    End-to-end ciphertext protects against unauthorized service access, not
    necessarily host root.
26. **Metadata leakage**: counts, timings, object sizes, traffic patterns,
    routing and certificate issuance can reveal information even when payload
    encrypted.
27. **Private research exposed by dashboard**: dashboard shows only redacted
    projections. Deliberate declassification grants are separately authorized
    and audited.
28. **Unbounded corpus ingestion**: arbitrary repositories/dependencies can
    carry secrets or hostile instructions. Curate authorized sources, license
    scope, dependency closure and source sanitization.
29. **Telemetry/ledger coupling**: if OTLP is down, canonical event append still
    works. Control-plane observability may degrade but must not silently stop
    experiments unless policy marks host integrity insufficient.
30. **Auto-modification of trusted kernel**: any patch touching
    scheduler/oracle/CA/release/policy is routed to _different governed
    project_; cannot be promoted by its own laboratory.
31. **Model-provider escape**: using an external hosted inference endpoint
    violates offline requirement. All inference and weights must be local and
    pre-provisioned by a trusted administrator.
32. **Task privacy via prompt alone**: telling an agent not to use private
    information is weaker than not mounting it. Access minimization is
    structural.
33. **Publication laundering**: a well-written signed paper may cite weak
    evidence. Claim-to-attestation map, negative results and scope limitations
    mandatory.
34. **Cross-repo overgeneralization**: a method's measured gain in repository A
    does not become a claim for repository B without fresh testing.
35. **Schema drift**: undocumented endpoints or event kinds allow unsafe bypass.
    Contract conformance checks gate releases and every producer/consumer
    deployment.
36. **Inconsistent observation**: state read at time t differs from event stream
    arriving at t+Δ. Use ledger snapshot seq and resumed SSE cursor, never
    assume wall-clock ordering.

## Required destructive drills

- Kill coordinator, worker, runner and OTLP receiver at every state transition;
  restart and prove no duplicate measurement is promoted, no credits are
  double-charged and all outputs remain attributed.
- Feed worker output that looks like an internal API request or forged
  attestation; assert it is stored as untrusted text, not executed.
- Mount malicious Rust build.rs and child command; verify no network or secrets
  access from builder/runner.
- Trigger compiler output that includes credential-like strings; verify
  redaction and sealed storage instead of OTLP/log leakage.
- Create simultaneous benchmark jobs with overlapping CPU siblings; scheduler
  must block or quarantine.
- Revoke a service credential mid-flight; new calls rejected, completed signed
  evidence still historically attributable.
- Disable product OTLP; benchmark must still finish and produce authoritative
  samples.
- Attempt to retrieve a private conversation as an observer; must fail even if
  its artifact digest is known.
- Reissue identical commands with same idempotency key; get original receipt,
  not a second transition.
