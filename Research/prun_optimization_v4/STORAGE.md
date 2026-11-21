# Durable storage, conversations, artifacts, and lineage

## 1. Data domains and authority

| Domain                                    | Authoritative?                                        | Format and write policy                             | Retention                                    |
| ----------------------------------------- | ----------------------------------------------------- | --------------------------------------------------- | -------------------------------------------- |
| Campaign/task/decision ledger             | **yes**                                               | append-only event envelopes, transactional sequence | permanent by policy                          |
| Source snapshots, patches, built binaries | **yes**                                               | SHA-256 CAS; read-only once published               | all decision-bearing objects                 |
| Oracle and benchmark raw samples          | **yes**                                               | immutable signed content hash; per-trial metadata   | all decision-bearing objects                 |
| Experiment results/analyses               | reproducible derived + immutable decision certificate | versioned analysis JSON, links to raw evidence      | preserve all decision versions               |
| Agent prompts and responses               | provenance evidence, not benchmark truth              | encrypted/redacted CAS blobs, ordered turn index    | configurable; never silently drop cited data |
| OTel control traces                       | diagnostic evidence                                   | OTLP batches plus query projections                 | tiered/sampled                               |
| OTel product traces                       | diagnostic evidence                                   | stage spans + optional flamegraphs                  | tiered; separate quota                       |
| Dashboard projections/leaderboards        | **no**, rebuildable                                   | materialized indexes/views                          | regenerate                                   |
| Build and container caches                | **no**                                                | OCIs/layers/build cache                             | garbage collect based on pins                |

A local SQLite WAL store (as in v2) plus a single-writer Rust API is an easy
first implementation. If avoiding _any_ non-Rust storage engine matters, `redb`
provides a pure-Rust ACID embedded KV store, but query/secondary-index
projections must be implemented manually. Neither option removes the need for a
content-addressed blob store and an event store.

## 2. Filesystem layout

```
.prun-lab/
  config/campaign.v3.json
  ledger/ledger.sqlite3                 # or redb in a Rust-only profile
  objects/sha256/ab/<full-digest>      # patches, prompts, timing JSON, binaries
  manifests/campaign/<campaign-id>/... # immutable signed view manifests
  worktrees/<task-id>/                 # scratch; never a source of truth
  traces/control/YYYY-MM-DD/...        # compressed OTel batches or OTLP export
  traces/product/YYYY-MM-DD/...        # never mixed with control traces
  indexes/                             # rebuildable search/telemetry/lineage projections
  queues/                              # only if needed for durable broker spool
  exports/                             # optional human reports; generated
```

## 3. Core identifiers and relations

`campaign_id -> generation_id -> work_order_id -> agent_session_id -> context_snapshot_id -> proposal_id -> mutation_id -> source_snapshot_sha -> build_key -> binary_sha -> experiment_id -> trial_id -> analysis_id -> attestation_id -> decision_id -> feedback_capsule_id -> next_mutation_id`.

Separate `causation_event_id` from `correlation_id`. The first points to the
exact preceding event that caused work; the second groups an entire generation
or experiment. Append events only after blobs are atomically persisted and
hash-verified. Use an event outbox pattern so a transaction cannot commit a
state change without a durable task dispatch intent; worker ACKs are idempotent.
A lease/heartbeat allows retry after a crash without double-accepting results.

## 4. Conversation lineage

Persist prompt templates and their versioned hash, role assignment, model
name/revision, generation settings, provider adapter version, exact _allowed
context bundle_, ordered turns, tool call/response hashes, truncated/compacted
turn IDs, stream timings and structured feedback capsules. Rehydration:

1. Coordinator chooses an active `agent_task_id` and a known role policy
   version.
2. Query only the authorized context ledger slice and source snapshot.
3. Compact if necessary with source-turn links and a digest proving what was
   omitted.
4. Record the exact resolved messages and permission digest in CAS before the
   model request.
5. Send request; stream responses; commit resulting transcript and usage
   timings.
6. Validate output against a role schema and check that referenced patch exists.

Storing a conversation is **not** the same as granting it to every future agent.
The agent's data view must remain an explicit capability decision even when full
history exists on disk.

## 5. Sealed holdout segregation

Store sealed fixture metadata, raw samples and detailed oracles in a dedicated
encrypted directory or DB access namespace; mutator views and generic
`GET /experiments` APIs must not leak through filenames, timing side channels,
result hashes correlated with known corpora, or detailed test failures. Only a
narrow confirmer role obtains the records. Export high-level attestation after
predetermined checkpoints; delay revealing detailed failure analysis until the
adaptive search phase is closed.

## 6. Invalidation, retention, and integrity

Source tree changed -> new build key; compiler/flags changed -> new build key;
binary hash changed -> never exact reuse; fixture/protocol/oracle/runner
environment changed -> re-run or classify as historical noncomparable evidence;
diagnostic instrumentation changed -> separate cohort. Keep a
`data_classification`, `retention_policy_id`, `redaction_policy_id`,
`source_citation_ids` and `crypto_key_id` per sensitive artifact. An audit
command must verify every referenced digest, event sequence, no orphaned
accepted decision and complete baseline-to-mutation lineage.

The SQL migration includes worker sessions, context snapshots, feedback,
proposal identity, comparisons, telemetry references and typed attestations. It
intentionally supplements the v2 schema instead of redefining its existing rows.
