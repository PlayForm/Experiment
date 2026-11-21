# v6.1 Constitutional amendment — deterministic software, opaque research workers

**Normative.** This amends v6 without weakening v6 prohibitions.

## Laws

1. **Only trusted Rust software operates the laboratory.** Admission, sampling,
   source snapshots, role assignment, experiment compilation, resource
   reservation, duplicate detection, attestations, correctness, benchmarking,
   comparison, retention, transfers, publication, and release gates are software
   services. Model output can only become an _untrusted proposal_.
2. **The model boundary is intentionally ignorant of the laboratory.** Model
   tasks contain a scoped repository tree at a pinned commit, local tools, a
   visible problem, optional approved references, and sanitized private
   feedback. No internal endpoints, DNS zone, OTel trace contexts, worker IDs
   outside opaque local references, certificate metadata, tests from sealed
   cohorts, other researchers' conversations, allocation ledgers, or approval
   rules.
3. **Optimization objective is the software under test**: correct behavior
   first; wall time, CPU work, memory/allocations, throughput, tail latency, and
   hardware-cohort specialization second. Compilation time and inference/token
   budgets are recorded for operations but cannot cause a candidate to be
   considered faster or slower. Runtime heuristic overhead **is** part of
   measured software time.
4. **Evolutionary search emerges from variation, measured selection, and
   inheritance.** There is no coded genetic algorithm, no crossover probability,
   no temperature optimizer, and no LLM-governed scheduler. The kernel's fixed,
   signed policy determines how candidate implementations are independently
   tested, preserved on a Pareto frontier, and made eligible as new reference
   source snapshots. This may cause evolutionary dynamics but promises neither
   global optimality nor guaranteed novelty.
5. **Agent independence is access-control-enforced.** Separate tasks and
   opinions; siblings don't receive peer output, experiment history, private
   verdicts, or aggregate private information unless a preauthorized
   declassification view explicitly permits it. Agents have no direct lateral
   communication or market trading. The kernel can share coarse public/approved
   metadata for diversity without disclosing restricted ancestry.
6. **Possession is a capability.** Before an artifact may be staged or mounted,
   verify sender/recipient, purpose, source ownership, content class,
   license/EULA, embargo, trade prohibition, retention, storage capacity,
   encryption, region/tenant policy, worker view, and transitive constraints. A
   valid signature never implies entitlement.
7. **Every transfer is two-stage and receipted.** Authorization/reservation
   happens before access; delivery/verification/commit happens after. Invalid,
   stale, uncommitted or orphan deliveries are quarantined. Every transition has
   an idempotency key, lease fence and an immutable event.
8. **Feedback is split in time.** Immediate private feedback contains only that
   worker's authorized verification/benchmark projection. Delayed dissemination
   across research tracks/repositories requires separate declassification,
   license and embargo checks and a new experiment on the recipient repo. Time
   of observation and time of knowledge/publication are separate.
9. **No worker receives enforcement diagnostics.** A worker gets only neutral
   submission acknowledgment or the ordinary bounded local file/tool status
   required for execution. Causes of restricted operations, embargoes,
   sanctions, revocation, violation classification and monitoring live in
   encrypted trusted audit records. Never return a false positive verification,
   false artifact delivery, or fabricated benchmark success.
10. **Humans remain accountable.** Operators can inspect authorized logs,
    incidents, policy changes, and custody chains through Plane 0. Owner
    commands can activate only prevalidated manifests and signed policies, not
    natural-language model instructions. License interpretation and rights
    approvals that cannot be mechanically resolved fail closed pending
    authorized human review.

## On the word deterministic

- The **state transition function** is deterministic given the ordered
  authoritative event log, pinned policy version, authenticated commands, and
  trusted signed external observations.
- Measurements, CPU scheduling noise, entropy, wall clock, compiler
  nondeterminism, model generation, and hardware failures are not physically
  deterministic. They are treated as timestamped observations with uncertainty
  and replayable inputs, not imagined to be deterministic.
- Model proposals need not be the only real-world source of nondeterminism; they
  are the **only untrusted generative input allowed to influence software
  candidates**.
- A fixed scheduler may use a pseudorandom trial order only if the seed is
  pinned, logged, and produced by trusted software. It must not be chosen by an
  LLM.

## Authority separation

| Function                            | Authorized principal                   | LLM worker rights                         |
| ----------------------------------- | -------------------------------------- | ----------------------------------------- |
| Campaign/workload policy            | Signed owner + trusted validator       | None                                      |
| Workload identification/profiling   | Trusted adapters and diagnostic runner | Read only permitted source/diagnostics    |
| Patch/test/research hypothesis      | Offline model worker                   | Stage untrusted files                     |
| Build and exact binary identity     | Builder authority                      | Local compile only, never official result |
| Correctness and differential oracle | Oracle authority                       | No trusted oracle access                  |
| Timing, CPU/RSS/cycles              | Runner authority                       | No official benchmark access              |
| Statistical promotion               | Fixed evaluator + release gate         | None                                      |
| Conversation archive and replay     | Correspondence broker                  | Own scoped local context only             |
| Artifact custody/license and trade  | Rust policy + vault                    | No transfer command                       |
| PKI and authenticated service mesh  | CA and service identity                | No keys/routes                            |
| Publication/knowledge transfer      | Publication bridge                     | Draft proposal only                       |
| Public API / GPUI / OTel            | Trusted Rust projection services       | No access                                 |
