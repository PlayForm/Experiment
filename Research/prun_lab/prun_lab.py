#!/usr/bin/env python3
"""PlayForm/Run Docker performance lab. Stdlib Python 3.10+; requires git + Docker Engine.

Run `python3 prun_lab.py init` once, edit prun-bench.json, then `python3 prun_lab.py all`.
Source refs are clean, committed Git snapshots. Building and benchmarking are separate phases.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import itertools
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import threading
from concurrent.futures import ThreadPoolExecutor
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

DEFAULT = {
    "schema_version": 1,
    "invariants": {
        "builder_base": "rust:slim-bookworm",
        "runner_base": "debian:bookworm-slim",
        "hyperfine_version": "2.0.0",
        "cpuset": "",  # Set to dedicated CPUs on a Linux host, e.g. "2-5".
        "runner_cpusets": {},  # For --parallel-runners, e.g. {"release":"2-3","diagnostic":"4-5"}.
        "memory": "4g",
        "warmups": 3,
        "runs": 15,
        "metrics": "time_wall_clock,time_cpu,time_user,time_system,memory_peak_resident",
        "environment": {"LC_ALL": "C"},
    },
    "refs": [{"id": "base", "git_ref": "HEAD"}],
    "builds": [
        {"id": "release", "profile": "release", "rustflags": [], "profile_env": {}},
        {
            "id": "native",
            "profile": "release",
            "rustflags": ["-Ctarget-cpu=native"],
            "profile_env": {},
        },
        {"id": "debug", "profile": "dev", "rustflags": [], "profile_env": {}},
    ],
    "feature_sets": [{"id": "default", "features": [], "no_default_features": False}],
    "fixtures": [
        {"id": "small", "directories": 32, "depth": 2, "noise_files": 3},
        {"id": "wide", "directories": 256, "depth": 3, "noise_files": 3},
    ],
    "workloads": [
        {
            "id": "small-serial",
            "fixture": "small",
            "pattern": "bench.marker",
            "parallel": False,
            "command": "/bin/true",
        },
        {
            "id": "small-parallel",
            "fixture": "small",
            "pattern": "bench.marker",
            "parallel": True,
            "command": "/bin/true",
        },
        {
            "id": "wide-serial",
            "fixture": "wide",
            "pattern": "bench.marker",
            "parallel": False,
            "command": "/bin/true",
        },
        {
            "id": "wide-parallel",
            "fixture": "wide",
            "pattern": "bench.marker",
            "parallel": True,
            "command": "/bin/true",
        },
    ],
    "exclude": [],
}

RUNNER_DOCKERFILE = r"""# syntax=docker/dockerfile:1
ARG RUNTIME_BASE=debian:bookworm-slim
FROM rust:slim-bookworm AS hyperfine-builder
ARG HYPERFINE_VERSION=2.0.0
RUN apt-get update && apt-get install -y --no-install-recommends build-essential pkg-config ca-certificates && rm -rf /var/lib/apt/lists/*
RUN cargo install --locked hyperfine --version "${HYPERFINE_VERSION}"
FROM ${RUNTIME_BASE} AS release
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates coreutils findutils python3 git && rm -rf /var/lib/apt/lists/*
COPY --from=hyperfine-builder /usr/local/cargo/bin/hyperfine /usr/local/bin/hyperfine
RUN printf '#!/bin/sh\nset -eu\ntouch "/checks/$(basename "$PWD")"\n' > /usr/local/bin/bench-signal && chmod 755 /usr/local/bin/bench-signal
ENV LC_ALL=C
WORKDIR /
FROM release AS diagnostic
RUN apt-get update && apt-get install -y --no-install-recommends procps strace linux-perf && rm -rf /var/lib/apt/lists/*
"""

# Runs *inside* the Docker runner. All hyperfine invocations happen inside the same
# allocated container. Each execution timed by hyperfine directly launches Run.
RUNNER_DRIVER = r"""
import json, os, pathlib, shlex, shutil, subprocess, sys
payload = json.loads(sys.argv[1]); fixtures = payload['fixtures']
checks = pathlib.Path('/checks'); checks.mkdir(exist_ok=True)

def command(binary, workload, validation=False):
    c = [binary, '--Root', '/data/' + workload['fixture'], '--Pattern', workload['pattern'],
         '--Command', 'bench-signal' if validation else workload['command']]
    if workload['parallel']: c.append('--Parallel')
    return c

for workload in payload['workloads']:
    fixture = fixtures[workload['fixture']]
    for v in payload['variants']:
        for p in checks.iterdir():
            if p.is_file(): p.unlink()
        cmd = command(v['path'], workload, True)
        try:
            p = subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True, timeout=180)
        except subprocess.TimeoutExpired:
            raise SystemExit('FAIL preflight timed out: ' + v['id'] + '/' + workload['id'])
        found = len([x for x in checks.iterdir() if x.is_file()])
        expected = fixture['directories']
        if p.returncode or found != expected:
            raise SystemExit(f'FAIL preflight {v["id"]}/{workload["id"]}: exit={p.returncode} touched={found}, expected={expected}, stderr={p.stderr[-1500:]}')
        print(f'PASS correctness {v["id"]}/{workload["id"]}: {found} directories', flush=True)
    path = '/results/' + payload['group'] + '--' + workload['id'] + '.json'
    argv = ['hyperfine', '--shell=none', '--metrics', payload['metrics'],
            '--warmup', str(payload['warmups']),
            '--runs', str(payload['runs']), '--export-json', path]
    for v in payload['variants']: argv += ['--command-name', v['id']]
    for v in payload['variants']: argv.append(shlex.join(command(v['path'], workload)))
    print('BENCH', payload['group'], workload['id'], flush=True)
    subprocess.run(argv, check=True)
"""

FIXTURE_DRIVER = r"""
import json, pathlib, sys
specs = json.loads(sys.argv[1]); root = pathlib.Path('/data')
for s in specs:
    base = root / s['id']; base.mkdir(parents=True, exist_ok=True)
    for i in range(s['directories']):
        p = base / ('shard_%03d' % (i % 32))
        for level in range(max(0, s['depth'] - 1)):
            p = p / ('level_%02d' % level)
        p = p / ('unit_%06d' % i); p.mkdir(parents=True, exist_ok=True)
        (p / 'bench.marker').write_bytes(b'')
        for j in range(s['noise_files']):
            (p / ('noise_%02d.txt' % j)).write_bytes((str(i) + '-' + str(j) + '\n').encode() * 8)
    print('Fixture', s['id'], s['directories'], flush=True)
"""


def run(args, *, cwd=None, input=None, capture=False):
    print("+", " ".join(str(x) for x in args), flush=True)
    p = subprocess.run(
        [str(x) for x in args],
        cwd=cwd,
        input=input,
        text=True,
        stdout=subprocess.PIPE if capture else None,
        check=True,
    )
    return p.stdout.strip() if capture else None


def digest(value):
    return hashlib.sha256(
        value if isinstance(value, bytes) else value.encode()
    ).hexdigest()


def slug(value):
    if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_.-]*", str(value)):
        raise ValueError("Invalid identifier: " + repr(value))
    return value


def load_config(path):
    c = json.loads(path.read_text())
    if c.get("schema_version") != 1:
        raise ValueError("Supported schema_version: 1")
    for k in ("refs", "builds", "feature_sets", "fixtures", "workloads"):
        if not c.get(k):
            raise ValueError("Missing nonempty " + k)
    for group in ("refs", "builds", "feature_sets", "fixtures", "workloads"):
        vals = [slug(x["id"]) for x in c[group]]
        if len(vals) != len(set(vals)):
            raise ValueError("Duplicate IDs in " + group)
    for w in c["workloads"]:
        if w["fixture"] not in {f["id"] for f in c["fixtures"]}:
            raise ValueError("Unknown fixture for " + w["id"])
    for b in c["builds"]:
        if b["profile"] not in ("release", "dev"):
            raise ValueError(
                "The initial runner supports Cargo profiles release and dev only"
            )
        if any(
            not re.fullmatch(r"CARGO_PROFILE_(RELEASE|DEV)_[A-Z0-9_]+", k)
            for k in b.get("profile_env", {})
        ):
            raise ValueError(
                "profile_env keys must be CARGO_PROFILE_RELEASE_* or CARGO_PROFILE_DEV_*"
            )
    if int(c["invariants"]["runs"]) < 2:
        raise ValueError("runs must be >= 2")
    return c


def matrix(c):
    out = []
    exclusions = set(c.get("exclude", []))
    for ref, build, feat in itertools.product(
        c["refs"], c["builds"], c["feature_sets"]
    ):
        ident = slug(ref["id"] + "__" + build["id"] + "__" + feat["id"])
        if ident in exclusions:
            continue
        out.append(
            {
                "id": ident,
                "ref": ref,
                "build": build,
                "features": feat,
                "profile": build["profile"],
                "runner": "release" if build["profile"] == "release" else "diagnostic",
            }
        )
    if not out:
        raise ValueError("All variants were excluded")
    return out


def git_commit(repo, ref):
    return run(
        ["git", "-C", repo, "rev-parse", "--verify", ref + "^{commit}"], capture=True
    )


def archive_compatible(repo, sha, dest):
    # Python 3.10/3.11 do not support TarFile.extractall(filter=...).
    p = subprocess.run(
        ["git", "-C", str(repo), "archive", "--format=tar", sha],
        stdout=subprocess.PIPE,
        check=True,
    )
    with tarfile.open(fileobj=io.BytesIO(p.stdout)) as t:
        for m in t.getmembers():
            target = (dest / m.name).resolve()
            if not target.is_relative_to(dest.resolve()):
                raise ValueError("Invalid source path: " + m.name)
            if m.issym() or m.islnk():
                # Git source archives can contain ordinary symlinks; keep source link only.
                if m.issym() and (
                    m.linkname.startswith("/") or ".." in Path(m.linkname).parts
                ):
                    raise ValueError("Unsafe symlink in archive: " + m.name)
        t.extractall(dest)


_lock_creation_lock = threading.Lock()


def prepare_lock(c, stage, out):
    with _lock_creation_lock:
        lock = stage / "Cargo.lock"
        lock_store = out / "locks"
        lock_store.mkdir(parents=True, exist_ok=True)
        key = digest((stage / "Cargo.toml").read_bytes())[:20]
        stored = lock_store / (key + ".lock")
        if not lock.exists():
            if stored.exists():
                shutil.copy2(stored, lock)
            else:
                print(
                    "Generating and retaining missing Cargo.lock for reproducible follow-up runs"
                )
                run(
                    [
                        "docker",
                        "run",
                        "--rm",
                        "--mount",
                        f"type=bind,src={stage},dst=/src",
                        "-w",
                        "/src",
                        c["invariants"]["builder_base"],
                        "cargo",
                        "generate-lockfile",
                    ]
                )
                shutil.copy2(lock, stored)
        else:
            if not stored.exists():
                shutil.copy2(lock, stored)
        return digest(lock.read_bytes())


def builder_dockerfile(c, v):
    b = v["build"]
    feat = v["features"]
    profile = b["profile"]
    outdir = "release" if profile == "release" else "debug"
    flags = " ".join(b.get("rustflags", []))
    cmd = ["cargo", "build", "--locked", "--profile", profile, "--bin", "Run"]
    if feat.get("no_default_features", False):
        cmd.append("--no-default-features")
    if feat.get("features", []):
        cmd += ["--features", ",".join(feat["features"])]
    env = "ENV RUSTFLAGS=" + json.dumps(flags) + "\n"
    for k, val in sorted(b.get("profile_env", {}).items()):
        env += "ENV " + k + "=" + json.dumps(str(val)) + "\n"
    return (
        f"FROM {c['invariants']['builder_base']} AS builder\n"
        "RUN apt-get update && apt-get install -y --no-install-recommends build-essential pkg-config ca-certificates git libssl-dev && rm -rf /var/lib/apt/lists/*\n"
        "WORKDIR /src\nCOPY . .\nENV CARGO_TARGET_DIR=/output CARGO_INCREMENTAL=0\n"
        + env
        + "RUN "
        + " ".join(cmd)
        + "\n"
        + f"RUN mkdir -p /package && cp /output/{outdir}/Run /package/Run\n"
    )


def image_id(image):
    return run(
        ["docker", "image", "inspect", "--format", "{{.Id}}", image], capture=True
    )


def setup_runners(c, out):
    infra = out / "infra"
    infra.mkdir(parents=True, exist_ok=True)
    dockerfile = infra / "runner.Dockerfile"
    dockerfile.write_text(RUNNER_DOCKERFILE)
    images = {}
    for target in ("release", "diagnostic"):
        tag = "prunlab-runner:" + target
        run(
            [
                "docker",
                "build",
                "-f",
                dockerfile,
                "--target",
                target,
                "-t",
                tag,
                "--build-arg",
                "RUNTIME_BASE=" + c["invariants"]["runner_base"],
                "--build-arg",
                "HYPERFINE_VERSION=" + c["invariants"]["hyperfine_version"],
                infra,
            ]
        )
        images[target] = {"tag": tag, "image_id": image_id(tag)}
    return images


def build_variant(c, repo, out, v):
    sha = git_commit(repo, v["ref"]["git_ref"])
    with tempfile.TemporaryDirectory(prefix="prunlab-src-") as td:
        stage = Path(td)
        archive_compatible(repo, sha, stage)
        lock_hash = prepare_lock(c, stage, out)
        dockerfile = builder_dockerfile(c, v)
        fingerprint = digest(sha + lock_hash + dockerfile)[:16]
        directory = out / "artifacts" / (v["id"] + "__" + fingerprint)
        binary = directory / "Run"
        tag = "prunlab-builder:" + digest(v["id"] + fingerprint)[:20]
        if not binary.exists():
            directory.mkdir(parents=True, exist_ok=True)
            with tempfile.TemporaryDirectory(prefix="prunlab-dockerfile-") as dt:
                df = Path(dt) / "Dockerfile"
                df.write_text(dockerfile)
                run(["docker", "build", "-f", df, "-t", tag, stage])
            container_id = run(["docker", "create", tag], capture=True)
            try:
                run(["docker", "cp", container_id + ":/package/Run", binary])
            finally:
                run(["docker", "rm", "-v", container_id])
        else:
            print("Using cached artifact", binary)
        binary.chmod(binary.stat().st_mode | 0o111)
        shutil.copy2(stage / "Cargo.lock", directory / "Cargo.lock")
        return {
            "id": v["id"],
            "sha": sha,
            "git_ref": v["ref"]["git_ref"],
            "profile": v["profile"],
            "runner": v["runner"],
            "binary_relpath": str(binary.relative_to(out / "artifacts")),
            "binary_sha256": digest(binary.read_bytes()),
            "cargo_lock_sha256": lock_hash,
            "build": v["build"],
            "features": v["features"],
        }


def build(c, repo, out, jobs=1):
    if shutil.which("docker") is None or shutil.which("git") is None:
        raise SystemExit("Docker and git must be installed on the host")
    out.mkdir(parents=True, exist_ok=True)
    variants = matrix(c)
    if jobs == 1:
        built = [build_variant(c, repo, out, v) for v in variants]
    else:
        with ThreadPoolExecutor(max_workers=jobs) as executor:
            built = list(
                executor.map(lambda v: build_variant(c, repo, out, v), variants)
            )
    images = setup_runners(c, out)
    cpu_model = ""
    if Path("/proc/cpuinfo").exists():
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.startswith("model name") or line.startswith("Hardware"):
                cpu_model = line.split(":", 1)[-1].strip()
                break
    host = {
        "platform": platform.platform(),
        "machine": platform.machine(),
        "cpu_model": cpu_model,
        "affinity": sorted(os.sched_getaffinity(0))
        if hasattr(os, "sched_getaffinity")
        else [],
    }
    meta = {
        "built_at_utc": datetime.now(timezone.utc).isoformat(),
        "host": host,
        "docker_version": run(
            ["docker", "version", "--format", "{{.Server.Version}}"], capture=True
        ),
        "rustc_version": run(
            [
                "docker",
                "run",
                "--rm",
                c["invariants"]["builder_base"],
                "rustc",
                "--version",
                "--verbose",
            ],
            capture=True,
        ),
        "invariants": c["invariants"],
        "variants": built,
        "runner_images": images,
    }
    (out / "latest_build.json").write_text(json.dumps(meta, indent=2) + "\n")
    print("Built", len(built), "variants; manifest:", out / "latest_build.json")
    return meta


def ensure_fixtures(c, image):
    spec = json.dumps(c["fixtures"], sort_keys=True)
    volume = "prunlab-fixtures-" + digest(spec)[:16]
    run(["docker", "volume", "create", volume])
    # The generator is idempotent and runs outside the timing window.
    run(
        [
            "docker",
            "run",
            "--rm",
            "--mount",
            f"type=volume,src={volume},dst=/data",
            image,
            "python3",
            "-c",
            FIXTURE_DRIVER,
            json.dumps(c["fixtures"]),
        ]
    )
    return volume


def parse_cpuset(value):
    result = set()
    for part in value.split(","):
        if not part:
            raise ValueError("Empty CPU set member")
        if "-" in part:
            lo, hi = map(int, part.split("-", 1))
            if lo > hi:
                raise ValueError("Invalid CPU range: " + part)
            result.update(range(lo, hi + 1))
        else:
            result.add(int(part))
    return result


def docker_limits(c, runner=None):
    inv = c["invariants"]
    args = [
        "--network",
        "none",
        "--memory",
        inv["memory"],
        "--memory-swap",
        inv["memory"],
        "--security-opt",
        "no-new-privileges",
    ]
    cpuset = inv.get("runner_cpusets", {}).get(runner) or inv.get("cpuset")
    if cpuset:
        args += ["--cpuset-cpus", cpuset]
    for k, val in sorted(inv.get("environment", {}).items()):
        args += ["-e", f"{k}={val}"]
    return args


def bench(c, out, parallel_runners=False):
    meta = json.loads((out / "latest_build.json").read_text())
    # Do not let a changed spec accidentally benchmark a stale variant matrix.
    expected = [(v["id"], v["profile"]) for v in matrix(c)]
    actual = [(v["id"], v["profile"]) for v in meta["variants"]]
    if expected != actual:
        raise SystemExit("Matrix changed since build: run `all` or `build` again")
    image = meta["runner_images"]["release"]["image_id"]
    volume = ensure_fixtures(c, image)
    if not c["invariants"].get("cpuset") and not c["invariants"].get("runner_cpusets"):
        print("WARNING: no dedicated --cpuset-cpus selected; measurements may be noisy")
    session = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    results = out / "runs" / session
    results.mkdir(parents=True, exist_ok=False)
    (results / "metadata.json").write_text(json.dumps(meta, indent=2) + "\n")
    (results / "config.json").write_text(json.dumps(c, indent=2) + "\n")
    groups = defaultdict(list)
    for v in meta["variants"]:
        groups[v["runner"]].append(v)
    if parallel_runners and len(groups) > 1:
        sets = c["invariants"].get("runner_cpusets", {})
        if any(not sets.get(r) for r in groups):
            raise ValueError(
                "--parallel-runners requires dedicated runner_cpusets for each runner group"
            )
        allocated = set()
        for runner in groups:
            cores = parse_cpuset(sets[runner])
            if allocated.intersection(cores):
                raise ValueError("Parallel runner CPU sets must not overlap")
            allocated.update(cores)

    def run_group(runner, variants):
        items = []
        for v in variants:
            p = Path("/variants") / v["binary_relpath"]
            items.append({"id": v["id"], "path": str(p)})
        payload = {
            "group": runner,
            "variants": items,
            "workloads": c["workloads"],
            "fixtures": {f["id"]: f for f in c["fixtures"]},
            "warmups": c["invariants"]["warmups"],
            "runs": c["invariants"]["runs"],
            "metrics": c["invariants"].get("metrics", "default"),
        }
        runner_image = meta["runner_images"][runner]["image_id"]
        args = (
            ["docker", "run", "--rm"]
            + docker_limits(c, runner)
            + [
                "--mount",
                f"type=volume,src={volume},dst=/data,readonly",
                "--mount",
                f"type=bind,src={(out / 'artifacts').resolve()},dst=/variants,readonly",
                "--mount",
                f"type=bind,src={results.resolve()},dst=/results",
                "--tmpfs",
                "/checks:rw,nosuid,nodev",
                runner_image,
                "python3",
                "-c",
                RUNNER_DRIVER,
                json.dumps(payload),
            ]
        )
        run(args)

    if parallel_runners and len(groups) > 1:
        with ThreadPoolExecutor(max_workers=len(groups)) as executor:
            list(executor.map(lambda pair: run_group(*pair), groups.items()))
    else:
        for runner, variants in groups.items():
            run_group(runner, variants)
    report(results)
    print("\nSaved benchmark run:", results)
    return results


def report(results):
    entries = []
    for path in sorted(results.glob("*.json")):
        if "--" not in path.name:
            continue
        group, workload = path.stem.split("--", 1)
        data = json.loads(path.read_text())
        items = data["results"]
        if data.get("schema_version") != 2:
            raise ValueError(
                "Expected Hyperfine 2.x JSON schema_version=2 in " + str(path)
            )
        baseline = items[0]["summary"]["time_wall_clock"]["mean"]
        for item in items:
            wall = item["summary"]["time_wall_clock"]
            raw = [m["time_wall_clock"]["value"] for m in item.get("measurements", [])]
            mean = wall["mean"]
            sd = wall.get("stddev") or 0.0
            entries.append(
                {
                    "runner": group,
                    "workload": workload,
                    "variant": item.get("name") or item["command"],
                    "runs": len(raw),
                    "mean_s": round(mean, 8),
                    "stddev_s": round(sd, 8),
                    "cv_percent": round(sd / mean * 100, 2) if mean else "",
                    "speedup_vs_first": round(baseline / mean, 3) if mean else "",
                }
            )
    target = results / "summary.csv"
    if entries:
        with target.open("w", newline="") as f:
            w = csv.DictWriter(f, fieldnames=entries[0].keys())
            w.writeheader()
            w.writerows(entries)
        for e in entries:
            print(
                f"{e['runner']:>10} {e['workload']:<17} {e['variant']:<38} "
                f"{e['mean_s']:.6f}s  {e['speedup_vs_first']}x  CV={e['cv_percent']}%"
            )
    else:
        print("No result JSON files in", results)
    return target


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--repo", type=Path, default=Path.cwd(), help="local PlayForm/Run checkout"
    )
    ap.add_argument("--config", type=Path, default=Path("prun-bench.json"))
    ap.add_argument("--out", type=Path, default=Path(".prun-bench"))
    ap.add_argument(
        "--jobs", type=int, default=1, help="parallel Docker builder jobs (default 1)"
    )
    ap.add_argument(
        "--parallel-runners",
        action="store_true",
        help="benchmark release/debug simultaneously ONLY on disjoint runner_cpusets",
    )
    sub = ap.add_subparsers(dest="action", required=True)
    for name in ("init", "plan", "build", "bench", "all"):
        sub.add_parser(name)
    rep = sub.add_parser("report")
    rep.add_argument("session", type=Path, help="results session directory")
    args = ap.parse_args()
    repo = args.repo.resolve()
    out = args.out.resolve()
    if args.action == "init":
        if args.config.exists():
            raise SystemExit("Config already exists: " + str(args.config))
        args.config.write_text(json.dumps(DEFAULT, indent=2) + "\n")
        print("Wrote", args.config)
        return
    if args.action == "report":
        print("Wrote", report(args.session.resolve()))
        return
    c = load_config(args.config)
    if args.action == "plan":
        for v in matrix(c):
            print(f"{v['id']:50} {v['ref']['git_ref']:20} {v['profile']}")
        print("Total", len(matrix(c)), "builds x", len(c["workloads"]), "workloads")
        return
    if args.jobs < 1:
        raise ValueError("--jobs must be >= 1")
    if args.action in ("build", "all"):
        build(c, repo, out, jobs=args.jobs)
    if args.action in ("bench", "all"):
        bench(c, out, parallel_runners=args.parallel_runners)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, FileNotFoundError, subprocess.CalledProcessError) as ex:
        print("ERROR:", ex, file=sys.stderr)
        sys.exit(1)
