# Novelty gate, feature overlap, exact result reuse, and evidence hygiene

## 1. Four-pass filtering (fast first)

**Pass 0: scope gate** (non-LLM, <build-time): verify patch parent, source
paths, approved feature family, contract digest, no evaluator/oracle
modifications, no unresolved build or network policy violations. Reject
`INVALID_SCOPE` before any model, compiler or benchmark time.

**Pass 1: exact identity:** keyed source-tree digest, normalized diff digest,
build-recipe digest, binary digest and complete experiment key. For a previously
**identical executable + environment + fixture + protocol + reset policy**,
reuse raw measurements as historical evidence with validity qualifiers. Exact
duplicate _experiment_ is not necessarily a current confirmation if environment
drift or independent replication is required.

**Pass 2: canonical structural similarity:** parse Rust code with pinned `syn` /
rust-analyzer-based syntax tooling. Normalize irrelevant formatting, equivalent
imports and fixed mechanical syntax while preserving conditional compilation and
semantics. Compute changed symbol sets, control-flow pattern tags, feature
flags, algorithm labels, dependency graph edges, and source ancestry. Candidate
fingerprint is a **hint**; unproven semantic equivalence must never be called
exact. Canonicalization algorithm version is part of identity.

**Pass 3: semantic/hypothesis similarity:** search prior cards by a structured
mechanism taxonomy and optional embedding similarity (offline local model or
deterministic token index); report nearest known experiments with cost/effect
and _sanitized_ summaries. An advisory classifier can decide likely overlap or
meaningful difference, but cannot erase a novel patch or promote evidence. If
suspected duplicate, candidate owner may justify different behavior/workload/cpu
class; independent deterministic adjudicator decides to skip, modify, or require
fresh test.

## 2. Feature collision rules

Three separate identities:

- `capability`: compile-time inclusion of an algorithm implementation.
- `strategy`: runtime chosen implementation and parameters.
- `mechanism`: causal change being investigated (e.g. removing allocations in
  matching).

Capabilities may coexist in one binary; strategies for the same exclusive
execution stage cannot both be active in the same run. Feature-set comparator
computes compatibility (`independent`, `overlapping`, `mutually_exclusive`,
`interacting`, `unknown`) and records interactions, Cargo `cfg` conditions,
changed symbol intersections and ABI/API contract effects. A patch touching the
same symbol as another is _not_ automatically equivalent; it is a
merge-conflict/interaction risk.

## 3. Experiment key and evidence reuse

Define:

```
run_key = sha256(canonical-json({
  binary_sha256, runtime_policy_sha256, command_spec_sha256,
  fixture_sha256, fixture_reset_digest, oracle_version,
  runner_image_digest, cpu_topology/cohort, effective_cgroup,
  kernel/fs cohort, environment_digest, measurement_protocol_sha256,
  telemetry_mode, cache_policy, iteration_policy
}))
```

`exact_build_key` additionally hashes source tree, lockfile, toolchain digest,
all Rust flags, Cargo features, target triple/CPU, PGO/profile artifacts and
reproducibility environment. Merely having the same Git commit or feature set
does not prove an identical binary.

Cache outcomes: `EXACT_RESULT_AVAILABLE`, `EXACT_BUILD_AVAILABLE`,
`PRIOR_ART_NOT_SAME_EXECUTABLE`, `NEAR_DUPLICATE`, `NEW`,
`REQUIRES_CONFIRMATION`, `CONFLICT`. Every reuse emits `EVIDENCE_REUSED` with
source experiment IDs and invalidation checks; never copy analysis verdicts to a
different `run_key` without recalculation.

## 4. Statistics and no-leakage boundaries

- Training/development results can guide the next mutation; sealed holdout is
  only evaluated by a dedicated confirmer at fixed checkpoints and its detailed
  results stay withheld until a campaign closes or a manually controlled reveal
  is approved.
- Use predeclared comparisons and practical-effect thresholds, paired order
  ABBA/BAAB, warmup/reset policy, sample counts and environment drift screening.
  Keep all raw samples and contamination flags.
- A candidate that looks better on the same workload family used for adaptation
  needs independent replication. Thousands of mutations create multiple-testing
  and winner's-curse risk; retain full experiment population and use
  confirmatory thresholds.
- Benchmark comparison is algorithmic/deterministic. An LLM can summarize but
  cannot produce the authoritative numeric verdict.
- Avoid leaking hidden oracle fixtures via test failure details; provide only
  minimal violation categories to patchers while retaining full details under
  trusted control.
- Negative evidence matters: preserve failed proposals and their contexts, but
  do not let the same model repeatedly train itself on unverified claims.

## 5. Evidence approval stamps

No single `approved=true` flag. Separate stamps:

- `novelty_checked`: identity and overlap analysis performed, policy version and
  comparator digest.
- `scope_checked`: only allowed files/effects changed.
- `oracle_passed`: correctness certificate linked to oracle
  fixture/implementation version.
- `bench_integrity_passed`: result artifact hashes, resource lease,
  contamination status, trial completeness.
- `analysis_confirmed`: confidence interval and practical threshold on sealed
  holdout or independent block.
- `integration_eligible`: all preceding requirements and review state.
- `release_approved`: explicit authorized human approval (off by default).

Each stamp is an immutable signed _attestation record_ with subject hashes,
issuer identity, policy version, event-log sequence and validity scope. HMAC or
public key signatures can be used depending on threat and multi-host
requirements. A stamp is not valid outside its declared protocol/hardware
cohort.
