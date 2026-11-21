# Three-plane Observability and a GPUI Research Exchange Console

## 1. Separation

1. **Control/cryptographic plane**: task lease, CA issuance, recipient
   identities, certificate rotation, envelopes accepted/denied, coordinator
   scheduling, status transitions and ledger commits. No plaintext payloads or
   private prompts in OpenTelemetry attributes.
2. **Research/economy plane**: bid size vector, resource reservation, metered
   spend, model costs, token latency/TFFB, time to first useful claim, citation
   retrieval, review queue, dissent coverage, Brier calibration, paper
   release/retraction. Use opaque task/claim/paper IDs; no private opinions in
   telemetry.
3. **Product measurement plane**: only controlled product execution benchmark
   timing/counters, with minimal instrumentation in release trials; rich trace
   builds run separately and mark as non-equivalent to release.

Correlate via `campaign_id`, `repo_id`, `research_case_id`, `mutation_id`,
`experiment_id`, `paper_id`, `role_id`, `agent_session_id`,
`resource_reservation_id`; restrict attribute cardinality and
personally/confidentially identifying values. Use OTel span links for
asynchronous task causality, not one impossible span spanning weeks.

The append-only event ledger remains the authority even if OTel exporter drops
records. The Rust OTLP ingress service receives trace/metric/log streams;
optionally interface to a standard OpenTelemetry Collector, without claiming it
is written in Rust. Documentation:
https://opentelemetry.io/docs/languages/rust/getting-started/.

## 2. Research circuit panel / 'FPGA board'

```text
TOP NAV  Tenants | Campaigns | Research Exchange | Evidence | Papers | Trust | Settings
ROW A    Trust health  | Treasury balance | Network throughput | Active experiments
ROW B    Researchers by stance / budget / token speed | Dissent map | Bids, accepts, rejects
ROW C    Case lineage: parent -> 5 patches -> oracles -> paired trials -> paper -> transfer
ROW D    Resource market: token burn, CPU seconds, exclusive runner slots, unused reserves
ROW E    Scientific quality: calibrated forecasts, cited claims, null results, replications
ROW F    Sealed publication: release state, redaction, signed attestations, audit timeline
```

Use Rust-native GPUI with Ely components for the operator UI and a separate Rust
API backend. Optionally embed WebView for HTML technical papers; WebView cannot
become a bypass for authorization. All evidence expansions use server policy and
read-only projections; the GUI never receives private ciphertext decryption keys
by default.

## 3. Target indicators

- P50/P95 inference first token latency, time to validated claim, time from
  patch to benchmark verdict.
- CPU consumption and exclusive benchmark availability; resource utilization by
  agent/repo/model cohort.
- Calibration curves and Brier score by agent/model/prompt-version cohort.
- Citation validity rate, duplicate opportunity rejection rate, blind
  agreement/disagreement matrix.
- Repeatability and holdout leakage warnings; paper corrections and transfer
  success.
- Certificate issuance/revocation, failed authentication, signature validation,
  immutable audit chain gaps.

Keep alerts for abnormal cost, token spikes, repeated recursive spawns, sudden
claim consensus, suspicious identity reuse, trace payload leakage and benchmark
host contamination.
