# PlayForm/Run — Docker-native benchmarking laboratory

An initial, reproducible experiment controller for
[PlayForm/Run](https://github.com/PlayForm/Run).

## What it implements

- Three-dimensional build matrix: committed Git refs × compiler/profile
  configurations × Cargo feature sets.
- Builder image generated from each clean source snapshot, using `--locked`,
  Cargo's `release` or `dev` profile, custom `RUSTFLAGS`, and optional Cargo
  profile environment overrides.
- Two runner images, `release` and `diagnostic`, with pinned Hyperfine
  **2.0.0**. Compiler time and Docker startup time are **outside** measurements.
- Dynamic deployment by read-only mounting of versioned binaries into the same
  runner environment. You can rerun the same artifacts without rebuilding.
- Deterministic fixtures in a Docker named volume (not host-shared files),
  read-only during benchmarks.
- A mandatory preflight that invokes a `bench-signal` helper in each matched
  directory and verifies the expected number of matched directories before
  timing.
- Sequential Hyperfine comparisons for candidates within each runner group; JSON
  schema v2 raw samples, CSV summaries, configuration, hashes and metadata
  retained per session.

**Important:** This is a benchmark _orchestrator_. It does **not** modify
PlayForm/Run source, implement feature gates, or automatically discover the best
algorithm. No Docker execution or actual timing measurements were run in the
environment where this file was authored.

## Requirements

- Linux with Docker Engine and a running daemon, Git, Python 3.10+, internet
  connectivity for the initial container and Cargo dependency builds.
- A **local checkout** of PlayForm/Run. `git archive` is used for reproducible,
  committed snapshots; uncommitted source changes are intentionally not
  benchmarked.
- Set a host CPU set dedicated to benchmarking for highest-quality comparisons.
  On Linux, `lscpu -e` and `cat /sys/fs/cgroup/cpuset.cpus.effective` can help
  select permitted cores. Avoid overcommitting CPU cores or sharing them with
  the builder.
- For Docker Desktop on macOS or Windows, expect host-VM and filesystem
  variance; use Linux bare metal for performance conclusions.

## Start

Copy `prun_lab.py` into your checkout and run:

```bash
python3 prun_lab.py init           # creates prun-bench.json
python3 prun_lab.py plan           # lists the Cartesian matrix
python3 prun_lab.py all            # builds then benchmarks
python3 prun_lab.py bench          # re-benchmark previously built variants
python3 prun_lab.py --jobs 4 build # parallel builds (not measurements)
```

Alternatively, use this supplied `prun-bench.json` directly and skip `init`.

Results: `.prun-bench/latest_build.json`, `.prun-bench/artifacts/`, and
`.prun-bench/runs/<UTC timestamp>/summary.csv` plus per-workload Hyperfine JSON.
Set `--repo`, `--config`, and `--out` before the subcommand if necessary. The
controller itself uses only Python standard-library modules.

### Explicitly isolated parallel runner groups

Benchmark comparisons within each runner group remain sequential. Release and
diagnostic groups can run concurrently **only when you provide disjoint CPU
affinity**:

```json
"runner_cpusets": {"release": "2-3", "diagnostic": "4-5"}
```

Place this property inside `invariants`, then run:

```bash
python3 prun_lab.py --jobs 4 --parallel-runners all
```

The controller rejects missing or overlapping CPU sets. Dedicated host cores
still need to be reserved outside Docker; `--cpuset-cpus` is CPU affinity, not
exclusive physical core allocation.

## Extend the matrix

The JSON schema is `prun-bench.schema.json`. Add source refs and feature sets
like:

```json
"refs": [
  {"id": "main", "git_ref": "Current"},
  {"id": "walker", "git_ref": "bench/walker-v2"}
],
"feature_sets": [
  {"id": "default", "features": [], "no_default_features": false},
  {"id": "bounded", "features": ["bounded-spawn"], "no_default_features": false}
]
```

Make sure `bounded-spawn` is implemented and declared in the corresponding
source branch's Cargo.toml, or the build correctly fails. Use `exclude` entries
such as `main__debug__bounded` to skip nonsensical combinations.

For compiler tuning, try additional build configs, **one difference at a time**:

```json
{
	"id": "thinlto",
	"profile": "release",
	"rustflags": [],
	"profile_env": { "CARGO_PROFILE_RELEASE_LTO": "thin" }
}
```

The current upstream `.cargo/config.toml` already requests `opt-level=3`,
`codegen-units=1`, LTO, panic abort and stripping in release builds. Evaluate
whether changing them is faster; don't assume.

## Benchmark rules and limitations

- Runs are sequential inside each runner group to reduce CPU contention. Release
  and debug results live in separate groups and should not be compared as
  like-for-like optimized binaries.
- Workloads supplied initially are synthetic traversal + `/bin/true` spawning.
  These **do not** substitute for representative actual repository `git status`,
  IO-bound commands, or more realistic trees.
- `bench-signal` checks that all expected directories were visited and commands
  executed. It does **not** establish full functional equivalence between
  arbitrary optimized implementations. Add integration tests for exclusions,
  symlinks, error propagation, file mode, output and ordering before accepting
  an optimization.
- Hyperfine 2.x exports JSON schema version 2
  (`results[].summary.time_wall_clock.mean` and `results[].measurements`); the
  CSV report specifically implements that version.
- Fixture creation happens before benchmarking; Docker container creation, build
  and copy operations are **not** timed.
- Default `cpuset` is empty: this is convenient but less controlled. Pin it in
  the manifest for high-signal measurements.
- A single release runner contains all release variants, allowing fairer
  comparisons than invoking `docker run` separately per sample.
- Diagnostic images have `strace`, `procps`, `linux-perf`, but full `perf`
  support may require compatible host perf userspace and explicit host
  permissions. Do not enable those profiling tools while timing.
- CPU architecture tuning using `-Ctarget-cpu=native` is valid only when builder
  and benchmark host expose the same target instruction set. For remotely built
  images use a specific `-Ctarget-cpu=x86-64-v3` or another verified common
  target.
- The initial source snapshot may have no `Cargo.lock`: the controller generates
  one in Docker, saves it under `.prun-bench/locks/`, and reuses it for
  identical manifests in subsequent runs. Preserve these locks for cross-date
  comparisons.
- Current implementation does **not** implement PGO/BOLT, automated bisecting,
  distributed scheduling, NUMA isolation, cold-cache experiments,
  confidence-interval-based winner selection, or automatic feature-gate
  creation. These are recommended phase-two extensions.

## Suggested next branch family

1. `bench/walkdir-baseline`: retain upstream walker; add correctness integration
   tests and trace-level instrumentation.
2. `bench/glob-compile`: compile glob patterns once and simplify path-to-string
   conversions; measure sparse and dense matching.
3. `bench/queue-bounded`: cap process spawn concurrency and investigate
   scheduling overhead under many matches.
4. `bench/walker-variants`: compare walkdir with alternative traversal
   algorithms under identical exclusions/symlinks semantics.
5. `bench/allocator`: test the global allocator (system, mimalloc, jemalloc if
   appropriate), prioritizing allocation-heavy workloads.
6. `bench/pgo`: instrument -> train with realistic datasets ->
   `llvm-profdata merge` -> optimized rebuild -> benchmark on **different**
   holdout datasets.

For phase-two fully automated source-level optimization, require a correctness
suite, a benchmark dataset held out from tuning, and a regression threshold with
repeated measurements. Avoid selecting winners based on tiny differences from
noisy samples.

## Optional Cargo wrapper

Upstream sets `autotests=false`, so it will not auto-discover test files. To
expose an explicitly named test, add to Cargo.toml:

```toml
[[test]]
name = "docker_perf"
path = "Bench/docker_perf.rs"
```

Create `Bench/docker_perf.rs`:

```rust
#[test]
fn bench_docker_pipeline() {
    if std::env::var_os("RUN_DOCKER_BENCH").is_none() {
        eprintln!("Skipping Docker benchmark (set RUN_DOCKER_BENCH=1)");
        return;
    }
    let result = std::process::Command::new("python3")
        .args(["prun_lab.py", "all"])
        .status()
        .expect("could not start Docker benchmark controller");
    assert!(result.success(), "Docker benchmark failed");
}
```

Now `RUN_DOCKER_BENCH=1 cargo test bench -- --nocapture` invokes the same
pipeline, while ordinary tests don't unintentionally perform a lengthy benchmark
run. Prefer `python3 prun_lab.py all` for agents and automated jobs.
