# Dual-timescale feedback, inheritance and bi-temporal provenance

## Fast loop: per-candidate private experimental feedback

```
trusted source snapshot → isolated proposal → validated patch → isolated build
    → reference+differential+change-impact+fuzz tests → benchmark if correct
    → signed evaluator facts → scoped feedback projector
    → same logical research track's next permitted context snapshot
```

The software feedback projector maps an internal result into an allowlisted,
read-only `v61-feedback-record`. It strips sibling identities, rejected security
checks, embargoes, sealed holdout data, full policy descriptions, internal host
topology and privileged credentials. Where factual data are withheld, omit them
rather than substitute fabricated success. A worker can receive real authorized
compile diagnostics or a generic tests-failed indication without learning the
existence or type of policy enforcement.

No LLM schedules another iteration. The coordinator's signed fixed campaign
policy decides when it may assign one; agent requests are merely opinions. Each
new candidate gets its own immutable ID and baseline parent. Frozen baseline and
immediate parent comparisons are both retained.

## Slow loop: qualified knowledge transfer / research inheritance

```
trusted evidence → independent formal analysis → private draft
    → rights+embargo+confidentiality check → sanitized signed release
    → optional authorized integrator task → target repo source snapshot
    → entirely new correctness/performance evidence → lineage link
```

This loop can lag the fast loop by days or campaign generations. A prior
observation may be corrected later without rewriting history. The _publication
date_, _date a fact was measured_, _date it became valid_, and _date the target
cohort learned it_ are distinct.

## Bi-temporal facts + causal clock

Each fact has `event_time`, `observed_at`, `valid_from`, optional `valid_to`,
`recorded_at`, `source_event_id`, supersession pointer and clock uncertainty. A
hash-linked ledger sequence establishes order of authoritative receipt, while
UTC is only an approximate cross-host clock. Host monotonic clocks measure
duration, not cross-host causal order. OTLP trace span links aid navigation but
are not canonical causality.

## Permissioned knowledge snapshots

A research track sees a materialized **view**, not a live global search engine.
At admission the broker computes source/references/feedback access under policy
P. Before every new invocation it recomputes access for policy P' and rights
changes; a previously given artifact cannot be unshown, therefore forbidden
content must not be mounted in the first place. Later embargo changes stop new
deliveries and key uses, not memory in already completed model outputs.

Correspondence retention and paper dissemination are separated: encrypted raw
prompt/output and local tool transcripts go into the correspondence vault;
sanitized scientific statements go into independently signed publication
objects. The UI can correlate them with authorization, but the worker cannot
enumerate that linkage.

## Recursion bound and liveness

The trusted coordinator has a finite signed campaign lifecycle,
cancellation/drain rules, dynamic but policy-limited retries, and deterministic
admission guards. It must not run forever simply because a model keeps proposing
changes. Stopping is governed by owner policy, experiment coverage, hardware
availability and measured equivalence—not by optimizing token expenditure.
