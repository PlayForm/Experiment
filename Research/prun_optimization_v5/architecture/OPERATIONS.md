# Unattended Research Economy Operations

## One-week campaign execution

1. Freeze policy, repository source commit, workload manifests, oracle and CPU
   cohort. Create campaign identities and budget pool.
2. Discover first atomic research cases; deduplicate by source parent, mechanism
   and previous experiment fingerprints.
3. Open sealed, role-diverse research opportunities with small initial credit
   caps.
4. Accept preregistered signed bids and issue atomic reservations; run
   independent researchers, collect distinct claims.
5. Assign limited patch work to validated proposals; produce immutable source
   changes; build in parallel.
6. Oracle-check semantics and enforce safe resource caps; benchmark candidates
   with exclusive runner leases.
7. Publish blinded statistical feedback, retain all negative/invalid results,
   and allow bounded refinement cycles.
8. Independent critics and replicas review the evidence and review holdout
   usage.
9. Authorized publication bridge creates private records; release sanitized
   public papers only with declassification grant.
10. Integrator reads released knowledge and creates new target-specific research
    cases, subject to fresh budget and validation.
11. End epoch, reconcile metered resource costs, write signed receipts, rotate
    credentials and snapshot indexes.
12. Stop at configured total budget, stale evidence, repeated invalid oracles,
    external provider outage, operator stop or measured information-value
    threshold.

## Reliability

- Durable outbox and idempotency key across all queue->worker dispatch; fencing
  tokens for leases; monotonic sequence on observations.
- Explicit typed failure modes: `provider_unavailable`, `signature_invalid`,
  `budget_exceeded`, `duplicate_bid`, `oracle_failed`, `benchmark_contaminated`,
  `publication_denied`, `context_leak_suspected`, `expired_lease`,
  `unknown_repo_adapter`.
- Every tool call and external reference is a scoped capability, never a shell
  command from arbitrary prompt without verification. A researcher cannot create
  unrestricted Docker tasks.
- The root CA, offline master keys and sealed holdout manager must live outside
  the general agent runtime.
- Container resource limits, cgroups/pinning and rate limits; no unmetered jobs.
- On crash, replay the event ledger, reconstruct reservations, reclaim expired
  leases only after verifying the worker is no longer authorized to commit, and
  preserve abandoned patches.
- Refuse to start a campaign that lacks oracle, measurement protocol, privacy
  policy, source permission, or at least one benchmarkable baseline.

## Minimum acceptance tests

- Blind researchers cannot query siblings' drafts via API, worktree, logs or
  telemetry.
- A researcher cannot spend beyond reserved resources or forge a settlement;
  racing bids cannot double-book CPU seconds.
- A CA-issued identity may prove sender, but has no power to attest experimental
  success.
- Replayed message rejected; expired/revoked credential rejected; unauthorized
  consumer cannot decrypt sealed capsule.
- Publishing requires independent science and sanitation attestations; no public
  release for unknown private/third-party repo.
- Two apparently equivalent ideas are compared; exact identities dedupe,
  semantic similarity never hard-proves equivalence.
- Every paper's claim resolves to immutable experiment or cited-source evidence
  and back to the atomic mutation.
- Null results and retractions remain visible after restart.
- Restart during benchmark, ledger settlement or paper release yields no
  duplicate expense, experiment or public paper.
- Run exactly one week with strict budget cutoff; no recursive agent self-spawn
  unless admitted by policy.
- A destination repository must independently reproduce any performance transfer
  claim.

## Implementation plan

P0: integrate v4 trust prototype's identity envelope format with v3 task IDs and
verify independently. P1: design/validate v5 schemas and additive SQL tables (in
this package). P2: implement Rust treasury with transactional reservations and
deterministic exchange; test race conditions and replay (sample CLI here is NOT
a full treasury). P3: role-configurable Rust agent worker and model adapter;
contract tests with fake LLM and network failpoints. P4: evidence/claim
registry, restricted researcher notebooks, blind assignments and signed
predictions. P5: publication bridge and policy-driven sanitization, citation
checker, signed internal preprint creation. P6: separately authorized
integration researcher; target-repository adapter/WASM extensions. P7: GPUI
console, meta-research, multi-host cohorts and federation.
