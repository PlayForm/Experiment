# Constitutional invariants (v6)

## What is optimized

1. Runtime elapsed time, CPU consumption, peak and total memory usage, useful
   throughput and predictability for each declared workload/hardware class.
2. Behavioral equivalence with the approved reference contract and existing
   supported features. Tests are evidence; tests cannot prove absence of all
   defects.
3. Search is permitted to spend any authorized amount of build time and model
   tokens. These are **not** terms in the performance acceptance objective.
   Bounded physical capacity still requires fair scheduling and resource
   accounting.
4. Preserve the full Pareto frontier and targeted implementations; do not
   conflate best on one fixture/CPU with globally best.

## What may _never_ happen

- LLM/model/agent workers may not call public or internal APIs, connect to
  trusted service mesh, see ledger/telemetry/topology/PKI schemas, read other
  workers' correspondence, control Docker, sign benchmark facts or alter active
  policy.
- LLMs may propose code, tests, claims, papers, and requests **only as staged
  untrusted artifacts**. A requested action is never an executable command to
  the lab.
- The trusted kernel is deterministic software. It applies pinned policies,
  manifests, explicit owner commands and measured evidence; it does not
  interpret a model's natural-language instruction to change its rules.
- A patcher-supplied test cannot substitute for trusted reference, differential
  and coverage oracles.
- No experiment may be promoted on unsigned source-of-truth measurements or a
  verdict produced solely by an LLM.
- No LLM-generated patch automatically modifies the trusted coordinator, CA,
  builder policy, oracle policy, or evidence verifier.
- No production benchmark uses product tracing unless specifically measuring
  instrumentation overhead; trace-off is the authoritative baseline.
- Neither the coordinator's ledger nor OTLP telemetry may claim to know an
  unpublished/private agent's internal thought process; store only observable
  inputs, visible outputs, permitted local tool calls and metadata.

## What counts as separate domains

- Human observation and owner control (Plane 0).
- Control/orchestration evidence and telemetry (Plane 1).
- Product/SUT execution evidence and diagnostics (Plane 2).
- Offline, untrusted model execution (Boundary X; **not** a fourth trusted
  plane).

## Evidence hierarchy

`worker-opinion < supervisor-ingested-artifact < trusted-build < trusted-oracle < trusted-runner < independent-statistical-evaluation < release-gate`.

A paper describes evidence; a cryptographic stamp identifies the signer and
binds evidence; neither can manufacture a passing test. Preserve failure and
inconclusive states. Release is reversible, traceable, and cohort-scoped.

## Revisions

Every contract carries a version. Old attestations and samples remain valid for
their old pinned policy and format. New policies do not retroactively change
outcomes. Migrations are additive. Canonical artifacts are immutable;
corrections supersede rather than mutate them.
