# PlayForm/Run — Adaptive Optimization Architecture v2 (design bundle)

## What this bundle is

This is a comprehensive **requirements and system-design extension** to the
previous Docker benchmark starter, not a replacement executable. The v1 starter
is a real benchmark orchestrator; the planned v2 concepts (heuristic planner,
agent scheduler, autonomous week, PGO and hardware-aware release) are **not yet
implemented**.

Read in order:

1. `REQUIREMENTS.md` — 22 sections, extensive numbered requirements, research
   agenda, semantic constraints, automation design, and acceptance gates.
2. `examples/campaign.v2.json` — concrete example showing
   source/build/feature/heuristic/hardware/workload/agent/safety dimensions.
3. `schemas/campaign.v2.schema.json` — Draft 2020-12 machine-readable schema for
   the example; validation requires a JSON Schema validator.
4. `playbooks/autonomous-week.yaml` — coordinator playbook contract for a
   bounded 168-hour experiment campaign, **not yet executable**.
5. `playbooks/AGENT_CONTRACT.md` — constrained local agent protocols for source
   mutation, measurement and analysis.
6. `sql/lineage.sql` — illustrative SQLite/WAL event and lineage tables;
   validate and migrate before deployment.

## Validation

- `python3 -m json.tool examples/campaign.v2.json >/dev/null`
- `python3 -m json.tool schemas/campaign.v2.schema.json >/dev/null`
- `python3 -c 'import json,jsonschema; jsonschema.Draft202012Validator(json.load(open("schemas/campaign.v2.schema.json"))).validate(json.load(open("examples/campaign.v2.json")))'`
- `sqlite3 :memory: < sql/lineage.sql` (or use Python stdlib `sqlite3`)

## Suggested concrete first pull requests

1. Characterization tests and exact CLI behavior contract from current upstream.
2. Separate internal stage interfaces while retaining reference implementation.
3. Schema v2 reader and migration from v1 plan representation (without silently
   reinterpreting v1 config).
4. Persistent local SQLite coordinator + CAS artifact storage + idempotent
   job/lease state machine.
5. Docker benchmark host health and contamination detection, randomized paired
   comparisons.
6. Heuristic static and shadow modes with reference fallback; instrument
   analysis overhead.
7. Narrow walker and scheduler experiments; enforce semantic oracles.
8. Worktree agent adapter with explicit permissions and bounded seven-day
   playbook.
9. Hardware-targeted and PGO experiments after held-out baseline suite is
   credible.

## Clarifying constraints

- Existing v1 code has not been modified in this bundle.
- The manifest references proposed Cargo features and runner strategies that are
  _not present_ in upstream; it will not build untouched upstream as-is.
- The YAML playbook, CLI names and Rust type names are future-facing interface
  contracts. The only existing local pipeline commands are those documented in
  the previous starter.
- No actual Docker experiment, performance measurement, agent run, or compiler
  autotuning was conducted in this environment.
