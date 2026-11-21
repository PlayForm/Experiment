# OCI image and container taxonomy

An **image** is a reproducible toolchain/environment template. A **container**
is a task instance. A **role** is an authorization and data-view policy. Avoid
image-per-agent-personality; reuse one pinned `agent-worker` image with
different task capabilities and prompt templates. Separate images where
binary/OS capabilities or instrumentation genuinely differ.

| Image / service                                    | Long-lived?            | Editable source?             | Read access                                | Output/authority                          | Placement                          |
| -------------------------------------------------- | ---------------------- | ---------------------------- | ------------------------------------------ | ----------------------------------------- | ---------------------------------- |
| `lab-coordinator` (Rust)                           | yes                    | no                           | full authorized event/CAS projections      | schedule, lease, signed work orders       | control CPU host/container         |
| `lab-agent-idle` (Rust worker + provider adapter)  | yes / warm pool        | only on assignment           | role task bundle only                      | task ACK + patch proposal                 | control CPU                        |
| `lab-patcher` (same agent image + capability)      | job session            | yes, one worktree            | assigned scoped files and own feedback     | patch commit + proposal                   | control CPU                        |
| `lab-scout` (same image)                           | job                    | no                           | allowed hot-path/profiler summary          | hypotheses                                | control CPU                        |
| `lab-critic` (same image)                          | job                    | no                           | scoped, verified evidence                  | analysis suggestions                      | control CPU                        |
| `lab-comparator` (Rust gate, optional AI advisory) | yes/job                | no                           | canonical patch/feature/experiment index   | prior art verdict, never benchmark truth  | control CPU                        |
| `lab-builder`                                      | ephemeral              | no: immutable input snapshot | source snapshot, locked deps, build recipe | OCI layers, binary CAS                    | build CPU/disk                     |
| `lab-oracle`                                       | ephemeral              | no                           | candidate + trusted fixtures/tests         | correctness verdict                       | test CPU                           |
| `lab-runner-release`                               | persistent for session | no                           | frozen read-only binary/fixture            | raw timing + counters                     | dedicated benchmark CPU/disk       |
| `lab-runner-diagnostic`                            | ephemeral              | no                           | frozen candidate + instruments             | flamegraphs, spans, perf                  | diagnostic host/window, not timing |
| `lab-runner-training`                              | ephemeral              | no                           | profile-training fixtures                  | PGO `.profraw`                            | build/test host                    |
| `lab-statistics` (Rust)                            | yes/job                | no                           | validated raw timing artifacts             | signed analysis record                    | control CPU                        |
| `lab-otel-ingest` (Rust)                           | yes                    | no                           | OTLP control/diagnostic signal             | telemetry derived indexes                 | control CPU                        |
| `lab-artifact-gc`                                  | scheduled              | no                           | CAS refs + retention policy                | temporary cache cleanup, tombstone events | control CPU                        |
| `lab-ui` (GPUI desktop)                            | interactive            | no                           | authorized API view                        | audited buttons; no direct files          | desktop host                       |

## Worker API, not host Docker socket

Agents call `POST /tasks/{id}/submit-proposal` with a patch artifact digest and
metadata. The privileged **container-launcher capability broker** is outside the
agent containers and accepts only whitelisted, signed coordinator task specs. Do
not expose `/var/run/docker.sock` inside patcher, build, or test containers. The
parent _also_ uses the broker; unrestricted APIs must not be a conversational
privilege.

## Runtime invariants

- Record OCI image digest, image build recipe, user, seccomp, mounts, cgroup,
  CPU topology, env allowlist, bind path modes, network policy and timeouts.
- Agent containers: non-root, no new privileges, read-only image, writable
  assigned overlay/worktree, no sibling mounts, default network-disabled except
  explicitly allowlisted model gateway address. Restrict exposed source with
  **per-task filtered snapshot** where practical, not only instructions to avoid
  reading sibling files.
- Patcher may write only its worktree and submit a patch to broker. No direct
  credentials, no hidden test data, no benchmark raw samples unless explicitly
  authorized.
- Builder: pinned toolchain/dependencies, network disabled once fetched, no
  benchmark CPU entitlement, deterministic source digest in/out.
- Oracle: fixtures and test definitions from a separately approved immutable
  trust root; candidate binaries not allowed to redefine pass/fail criteria.
- Release timing runner: persistent initialized container, `--network none`
  unless a separately declared network workload, no telemetry exporter, no
  mutable source mount; fixture copies reset by an independent harness when
  needed. Idle on its reserved CPUs except benchmark workload. Primary output is
  Hyperfine raw data and independent process counters.
- Diagnostic runner: tracing/profiling tools and extra permissions only where
  necessary. Run separate from primary timing and clearly label every trace
  variant.
- Shared telemetry sidecars are kept off the physical resources reserved for
  timing where possible.
- Exclusive CPU reservation must account for sibling SMT, LLC sharing, NUMA,
  thermal/power and disk interference. `--cpuset-cpus` alone does not guarantee
  physical isolation.

## Frozen images vs variants

Images are _invariants within a comparison cohort_ and may vary only in
explicitly labeled infrastructure experiments. Source commits, Cargo features
and Rust flags change the **binary artifact**, not the release timing
environment. `trace-off`/`trace-lite`/`trace-diagnostic` are separately hashed
binaries; do not treat their results as interchangeable.

## Image deployment strategy

Prefer one immutable OCI `lab-agent-worker@sha256:...` for all agent roles with
dynamically assigned capability manifests. Keep the same image for a warm idle
patcher and a newly spawned patcher. Allocate fresh worktree overlays per task.
For model APIs the _agent process_ idles, not the model itself; a local model
server is a separate resource service, and provider conversation state must be
represented explicitly.
