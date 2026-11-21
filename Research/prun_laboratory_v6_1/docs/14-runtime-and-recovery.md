# Runtime deployment, readiness, recovery and incident playbooks

## Minimal deployment order

1. Initialize offline root, configure constrained intermediates and
   identity/credential issuance journal.
2. Start signed policy store, CAS, immutable SQLite ledger and deterministic
   coordinator; verify integrity checkpoints.
3. Start trusted internal mTLS API and event-outbox dispatcher, then source
   adapters and sandboxed worker supervisors.
4. Start builder, oracle, exclusive benchmark scheduler/hosts and evaluator.
   Benchmark service must prove CPU resource topology before acceptance.
5. Start the separately authenticated control and product OTLP gateways; product
   OTLP may remain disabled.
6. Start read projection and public monitoring API; GPUI/CLI are just clients.
7. Mount local model weights offline and launch a worker to validate source-only
   task input and staged output, with no network, credentials or trusted API
   routes.
8. Run a zero-change reference campaign through build, oracle, benchmark and
   signed evidence; confirm restart/replay.
9. Only then allow a candidate patcher campaign and publication/integration
   workflows.

## Recovery scenarios

| Fault                                             | Required result                                                                                                                                  |
| ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Coordinator killed after commit but before reply  | Same idempotency request returns original event/receipt                                                                                          |
| Worker container dies after writing partial patch | Output marked partial/quarantined; never silently accepted                                                                                       |
| Supervisor dies while model running               | Lease expires/fenced; next supervisor cannot reuse old authority token                                                                           |
| Builder crashes after producing binary            | CAS commitment or retry by identical deterministic build key; provenance states ambiguity honestly                                               |
| Oracle process crashes                            | Infrastructure error, not correctness failure or pass                                                                                            |
| Bench host overheats or another process runs      | Samples marked contaminated; preserved, excluded per frozen protocol                                                                             |
| Product OTLP stops                                | Diagnostic data missing; trace-off benchmark and event ledger continue                                                                           |
| Control OTLP stops                                | Operating visibility degraded; ledger and state machine continue                                                                                 |
| Event subscriber misses messages                  | Replay outbox from ledger sequence; apply idempotently                                                                                           |
| CA issuer unavailable                             | No new credentials; existing sessions expire by policy; preserve signed historic results                                                         |
| Local model unavailable                           | Pending implementation tasks pause; trusted verification and API still work                                                                      |
| Private key suspected compromised                 | Revoke, rotate, record timeline and reissue; invalidate affected new evidence                                                                    |
| Host root compromised                             | Container isolation alone is insufficient; assume plaintext/key exposure where accessible, rotate secrets and rebuild trust from external anchor |

## No autonomous policy mutation

A proposal to change the core daemon, CA policy, trusted oracle, benchmark
analysis or publication authority becomes a **separate governed maintenance
project**. This optimization laboratory cannot accept its own LLM-generated
modifications to its measuring instruments. Owner-authorized releases of trusted
infrastructure require their own test environment and independent approval.

## Resource scheduling vs scoring

Build time and model token use are recorded for operational capacity but do
**not** weigh against runtime wins. Prevent starvation, excessive disk usage and
infinite unbounded work through human-signed campaign configuration; that is a
safety/reliability measure, not research efficiency scoring.

## Local-only boundaries

Never silently enable remote model API, plugin, editor assistant service,
telemetry exporter, DNS recursion to external authorities or unapproved
dependency fetches. Internet-enabled source acquisition, if needed for research,
must be a separate audited preparation phase outside the sealed laboratory.
