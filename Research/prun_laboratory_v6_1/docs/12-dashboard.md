# GPUI dashboard as a read-only projection of the public API

## One API for all views

The desktop GUI and CLI have **zero special service privileges**. They request
status via `/v1/status`, reconcile events via `/v1/events` and
`/v1/events/stream`, and drill down via the typed `GET /v1/*` endpoints.
Owner-command panel is separately authenticated and sends only declarative
`command` documents. No model can access these endpoints or UI credentials.

## Screens

1. **Overview:** coordinator health, active campaigns, queue depth, current
   benchmark host leases, oracle failure rate, telemetry plane status, latest
   signed journal sequence and stale projection warning.
2. **Execution graph:** task -> proposal -> mutation -> build -> oracle ->
   benchmark -> evaluation -> publication with exact artifact/signature links.
   No model-generated explanation can rewrite this graph.
3. **Agent matrix:** role/cohort, idle/busy/retry/quarantine, number of private
   conversation turns, task outcome; private conversation plaintext hidden by
   default.
4. **Benchmark microscope:** baseline/candidate pairing, raw samples, error
   bars, CPU time, RSS, hardware cohort, drift/quarantine flags. Diagnostic
   traces explicitly labeled non-authoritative.
5. **Trust and certificate audit:** issued/renewed/revoked/denied certificates
   by role and reason; credential lifetime and issuer; no key material.
6. **Research/papers:** claims, independent opinions, evidence map,
   contradictions, publication stage and scope restrictions; unverified opinions
   visibly different from attested results.
7. **Dual observability:** control-plane service spans alongside separate SUT
   diagnostics; cross-link by signed experiment IDs, never mix timing as if same
   build.
8. **Lineage diff:** source-tree ancestry, per-mutation patch, tests added vs
   trusted tests, result hashes, merge interaction tests.
9. **Policy and operations:** effective signed policy version, status of offline
   worker network restrictions, host capacity, backup/checkpoint and protocol
   compatibility.

## Implementation direction

Rust `axum` or equivalent for API; Rust GPUI for desktop; Ely GPUI optional
visual component layer; an optional embedded WebView is merely a presentation
component. UI is not the state machine or trust authority. Support offline
viewing of already materialized projections, with an explicit "stale since seq"
marker.

## Event streaming behavior

SSE `id` is ledger sequence, `event` is approved event kind, `data` is a
redacted envelope. On reconnect, resume from last acknowledged sequence. Do not
depend on receiving every SSE frame: fetch `/v1/events?after_seq=...` if a gap
occurs. A dropped OTLP span should never appear as a missing authoritative
event.

## Observer privacy expectations

The screen can show `Turn 4, 1,203ms, encrypted 8.1KiB, successful result`
without knowing its text. It must not issue a decrypt call on behalf of an
observer. Any explicitly authorized plaintext viewer should be a separately
audited tool/role, not a hidden dashboard feature.
