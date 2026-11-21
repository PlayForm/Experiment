# Making the Laboratory Repository-Agnostic

## 1. Reusable control plane; repository-specific adapters

The same Rust coordinator/treasury/research workers can serve many languages,
but no meaningful performance optimization is possible without valid workload,
build and behavioral equivalence definitions for each repository. Define a
`RepositoryAdapter v1` as a versioned, declarative manifest with optional
sandboxed Wasmtime/WASI plugin for nontrivial fixture generation and result
parsing. See https://docs.wasmtime.dev/ for Wasmtime's Rust embedding and
Component Model.

Adapters declare:

- repository origin and permitted Git refs; owner/visibility, license,
  authorization to benchmark/modify/publish;
- code review scope, source paths and semantic stages;
- language/toolchain, image digests, hermetic build recipes, build platforms,
  compiler flag dimensions;
- artifact extraction and executable entry points;
- correctness oracle with trusted fixtures,
  exit-code/stdout/stderr/FS-side-effects/network contracts;
- deterministic workload definitions, setup/teardown, warm/cold cache policy,
  sample size and resource allocation;
- performance metrics and direction, confounders, machine-cohort constraints and
  sample protocol;
- source-level feature switches and validation strategies; Git worktree and
  patch constraints;
- privacy/reuse policy (internal, public metadata, sanitized papers), network
  policy and secret handling;
- optional acceleration candidates and profiling tools.

## 2. Adapter compatibility

Use JSON Schema for manifest validation, versioned WIT/Protobuf for RPC/plug-in
interfaces, and a `capability_manifest` stating exactly which features the
adapter supports. A public Python project may support only CLI latency and
correctness; a Rust CLI may support CPU flags, `perf`, LTO and PGO; a GPU
project may require CUDA/ROCm-specific builders and exclusive accelerator
leases. Don't pretend universal metrics exist.

## 3. Transportable knowledge

Represent reusable ideas in a cross-repository ontology, not as hard-coded Rust
feature names:

`stage = traversal | matching | parsing | scheduling | spawning | I/O | allocation | compression | dispatch | memory | vectorization | batching | GPU kernel | compilation`.

Each knowledge card includes applicability predicate (`platform`,
`bottleneck signature`, `correctness constraints`, `required capability`) and
_negative transfer_ evidence. A source repository's speedup is NOT portable
evidence for a destination. New target repo -> separate hypothesis -> isolated
branch -> oracle -> benchmark -> publish transfer report.

## 4. Deployment modes

**Single-machine developer appliance:** one Compose project containing Rust
coordinator, Rust trust relay, Hickory DNS, research/patcher image, isolated
builder, correctness oracle, exclusive runner, scoped publisher and storage;
external LLM inference may be on host or via an API adapter. 'One command'
starts a multi-container trust-separated application, not one giant privileged
container.

**Multi-host deployment:** tenant/repository partitioning in coordinator
storage, worker pools labeled by architecture, signing domain and benchmark host
cohort; artifact store and event bus can be distributed; source data stays
within tenant policy. Requires distributed leases with fencing tokens,
consistent event commits, audit-log replication and per-tenant quotas. Host
benchmark consistency is maintained via dedicated measurement slots, not shared
noisy CI workloads.

**Global/open collaboration:** signed public paper exchange and adapter registry
with opt-in source access; separate administrative domains, cross-CA trust
policy and explicit publication licenses. Do not scan or execute arbitrary
Internet repositories without permission.

## 5. Practical onboarding

`lab repo add <git-url> --adapter adapter.json` -> parse and statically check
manifest -> verify owner consent -> provision scoped secrets -> run reference
oracle -> establish baseline measurement -> freeze cohort -> enable patch
exploration and researchers. If no reliable correctness oracle, lab runs
exploratory profiling only and cannot call candidates 'improvements'.

## 6. Model provider abstraction

The research agent image handles signed tasks and model-adapter capability
descriptors (`local_http`, `local_gguf`, `remote_api`), with per-turn token caps
and throughput instrumentation. The same OCI image can run proponent, skeptic,
librarian and editor via signed role manifests. Avoid assuming any model
supports tool-use, large contexts or schema-constrained output; minimal protocol
supports structured task in, structured answer out, deterministic validation and
repair retries with bounded budgets. Small models can submit short evidence
references, classify cases, and flag uncertainty; expensive models are reserved
by treasury policy for complex cases.
