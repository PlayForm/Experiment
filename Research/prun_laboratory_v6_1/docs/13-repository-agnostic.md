# From PlayForm/Run to repository-agnostic optimization

## Repository adapter is not an LLM

A repository-specific adapter is a signed, versioned, immutable software module
with explicit entry points for dependency acquisition, source scoping,
deterministic building, trusted reference verification, candidate verification,
dataset preparation and benchmarking. It has no right to approve its own
outputs; service roles and policy grants remain separate.

An adapter might understand Rust/Cargo (including build.rs and features),
C/C++/CMake, Go, JS runtimes, etc. Adapters may be implemented as Rust host
plug-ins or constrained Wasm components. They are **executed as untrusted or
tightly scoped code** until reviewed; a novel adapter can itself execute
arbitrary build scripts.

## Required adapter manifest fields

- Repository identity, exact commit/tree and license policy.
- Language/toolchain closure and offline dependencies.
- Supported platform/architecture and minimum allowed host environment.
- Reference semantics: trusted tests, fixtures, observable outputs, side
  effects, concurrency and exit handling.
- Workload generators and fixture checksums, cache regimes, resource
  constraints, holdout split.
- Build profile matrix, feature gates, tuning dimensions, PGO inputs and
  trace-off target.
- Artifact formats, binary selection, execution harness, privilege and network
  constraints.
- Required trusted oracle authority and benchmark runner capability.

## Preventing repository leaks

LLM worker sees only allowlisted file tree (not `.git` secrets, CI credentials,
home directories or artifact registry). Dependencies and public references are
pre-fetched by trusted, audited ingestion **before** the offline model task.
Repository A cannot read source from repository B unless a separately
sanctioned, sanitized public/intended knowledge card is published.

## Transfer evidence is not portable speedup

A paper or knowledge card transferred from one repo to another is an _idea_ with
source evidence. New repo/host/workload requires fresh build, correctness, and
performance experiment before promotion. The knowledge graph may link those
independent experiments but must not merge their scores.

## Scaling without losing isolation

Containerize stateless offline agents and builders; scale reliable benchmark
hosts by dedicated physical hardware cohorts; split trust domains and CAS
namespaces by repository/tenant; use signed protocol compatibility versions. API
can provide a federation overview, but operators may read only authorized
projections. Do not expose a global wildcard worker task inbox or cross-tenant
search index.
