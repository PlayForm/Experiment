# Agents, pre-prompts, subsequent prompts, and context lifecycle

## 1. Agent role taxonomy

| Role                          | Sees                                                                | Writes                                | Forbidden                                                 | Return object         |
| ----------------------------- | ------------------------------------------------------------------- | ------------------------------------- | --------------------------------------------------------- | --------------------- |
| Research director             | approved campaign-wide topology, verified result summaries, budgets | hypothesis queue and experiment plans | altering data, direct release                             | `ResearchPlan`        |
| Scout                         | limited code and diagnostic data                                    | hotspot reports                       | claiming speedup without run                              | `HotspotCard`         |
| Decomposer                    | assigned module + behavior oracle                                   | refactor patch (own branch)           | semantic refactor without oracle                          | `ModuleBoundaryPatch` |
| Patcher                       | task source slice, frozen baseline interface, own lineage feedback  | one committed mutation                | runner data, other agent branches, benchmark oracle edits | `PatchProposal`       |
| Diversifier                   | novelty summaries only                                              | bounded independent hypothesis seeds  | distributing other patchers' code                         | `DiversityAssignment` |
| Comparator/novelty classifier | canonical patch metadata and redacted similarity neighbor snippets  | advisory similarity verdict           | converting uncertainty into proven identity               | `SimilarityAdvisory`  |
| Oracle reviewer               | trusted semantic contract, candidate delta                          | test gap report to separate lane      | changing oracle inline with its evaluated candidate       | `OracleReview`        |
| Diagnostic analyst            | versioned profiles and performance attribution                      | explanation and next hypotheses       | modifying trial samples                                   | `Diagnosis`           |
| Statistician                  | immutable samples and protocol                                      | descriptive analysis                  | self-declared acceptance independent of evaluator         | `StatisticalAdvisory` |
| Integrator                    | approved candidate branches and compatibility matrix                | integration patch                     | inheriting performance claims from parents                | `IntegrationProposal` |
| Curator                       | complete authorized immutable records                               | summaries and dashboard annotations   | overwriting history                                       | `EvidenceDigest`      |

All roles can be one Rust agent-worker runtime with selected provider adapter,
system prompt template and typed JSON output contract. Model choice is an
**experiment variable**; freeze model identifier, provider endpoint,
temperature, seed when supported, context policy and decoding parameters per
agent task.

## 2. Patcher initial prompt contract (template)

System policy (versioned, hashable, never supplied by patcher):

> You are a scoped optimization worker. Optimize one assigned
> behavior-preserving Rust component. You may read only the assigned source
> snapshot. Provide exactly one conceptual code change as a committed patch, a
> short mechanism hypothesis, correctness risk analysis and anticipated
> workloads. Do not change benchmark definitions, trusted oracles,
> policy/configuration outside the granted scope, data exports or manifests. Do
> not claim speedups; only the benchmark service may measure. Do not execute
> target user commands as a heuristic probe. If you cannot comply with scope,
> return BLOCKED. Prior raw outcomes from other workers are intentionally
> unavailable.

Developer/task envelope: task ID, authorized scope paths, parent Git tree,
function contract digest, known bottleneck class, code symbols, forbidden
modules, target Rust version/features, local smoke tests, applicable issue
hints, experiment budget and API schema.

User/task question example:

> Propose and implement a bounded work-queue mutation in `scheduler` on the
> frozen baseline, preserving exit/output contract. Return patch hash and one
> measurable hypothesis. Avoid changes to walker, matcher, CLI, fixture code or
> benchmark harness.

**Subsequent feedback** contains the patcher's own candidate ID, verification
status, aggregate development-workload changes, confidence/regressions, crash
signatures, resource comparison and a narrowly scoped new task. It does not
include sealed holdout or other agent transcripts. A feedback capsule _must_
state whether metrics are exploratory or confirmatory, and carry result artifact
hashes.

## 3. Conversation storage and compaction

Store all records as independent content-addressed blobs: immutable
prompt-template version/hash; model request metadata; system/developer/user
messages; tool calls; tool results; stream token timing; model responses;
context summaries; compaction provenance; patcher task/view permission hash;
derived interpretation; and final accepted artifacts. Sensitive text is
encrypted at rest or redacted before persistence according to policy, with
raw-command privacy controls.

- `conversation_id`: persistent logical task thread; `turn_id` monotonic within
  thread; `session_id` may change across worker restarts.
- `context_snapshot_id`: exactly what was sent to model for a turn, including
  prompt version, allowed source bundle digests, messages and truncation map.
- `compaction_id`: digest of summary plus ids of all source turns; no silent
  truncation. Keep original turns when retention allows.
- `response_id`: generated content hash, generation settings, token usage, model
  revision, finish reason, and partial stream spans.
- `retry_attempt`: distinguish transport retry from new reasoning attempt;
  retries use idempotency keys so duplicate submissions cannot multiply
  experiments.
- `post_run_feedback`: separate immutable message, not an in-place edit of the
  prior model conversation.

Provider-specific server-side caches/session APIs should be treated as
acceleration only. Rehydration must work from the local source-of-truth ledger
without depending on provider continuity.

## 4. Token-to-first-token, not benchmark latency

Instrument `llm.request.start`, `llm.first_response_byte`, `llm.first_token`,
`llm.stream.complete`, `tool.request`, `tool.result`; record TTFT (time to first
generated token), TTFB, prefill input size, output tokens/s, cached input token
count where available, tool round-trip and context serialization duration. Keep
these telemetry emissions on the control-plane host, never on dedicated
benchmark CPUs.

Latency reduction priorities:

1. Send **structured evidence capsules** (small enough to parse) instead of
   complete Hyperfine logs or huge transcripts.
2. Keep scoped source bundles stable and cacheable; send patch-delta/context
   hashes rather than repeated full repository dumps.
3. Precompute deterministic novelty and benchmark summaries _before_ waking an
   LLM; short-circuit exact duplicates without inference.
4. Warm idle worker processes and local model gateway as appropriate; avoid
   speculative provider calls just to keep a remote session live.
5. Fan out five _independent_ tasks concurrently, but cap context size/model
   concurrency and leave benchmark cores unaffected.
6. Stream model outputs, then require schema-valid final result with patch
   artifact digest.
7. Separate human-facing director explanation (can be slow) from deterministic
   scheduling critical path.

## 5. Diversity strategy for agents sharing a model

Sharing the same LLM does not imply independent exploration. Give fanout members
different **research constraints**, not merely different superficial personas:
e.g. data layout, allocation and memory, bounded task scheduling, path matching,
syscall minimization, runtime policy. Require per-generation topic coverage,
explicit excluded approaches, non-overlapping module paths when possible and
independently seeded sampling. Keep candidate source and peer outcomes hidden
until their submissions are frozen. Adaptive replanning can use verified
aggregate novelty signals, never secretly reveal sealed holdouts.

Metrics: duplicate proposal rate, first-novel-patch latency, accepted distinct
strategy count, expected gain per 1k model tokens, fraction of unique changed
symbols, time-to-first-verified-improvement, cross-agent convergence matrix and
entropy over independent algorithm families. Diversity must not be optimized at
the expense of correctness or scientific validity.
