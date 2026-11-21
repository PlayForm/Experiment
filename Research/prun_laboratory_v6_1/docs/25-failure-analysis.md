# Pitfall and failure-mode register v6.1

| ID  | Severity | Failure                                                         | Deterministic control                                                        | Negative acceptance test                                                   |
| --- | -------- | --------------------------------------------------------------- | ---------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| F01 | Critical | Model learns internal host/API via environment/tool stdout      | Fresh offline namespace, env allowlist, no sockets/proxies, sanitize stderr  | Probe env, procfs, DNS, socket access; no secret/network route             |
| F02 | Critical | LLM modifies a trusted oracle/benchmark                         | Trusted image/fixtures not writable by model; source of truth frozen         | Patch attempts to rewrite oracle; result remains unaffected                |
| F03 | Critical | Build script escapes model sandbox into trusted host            | Treat Rust build.rs, proc macros and compiler plugins as untrusted execution | Untrusted build attempts outbound and privileged filesystem access; denied |
| F04 | Critical | Unauthorized transfer of embargoed paper/source                 | Signed policy preflight before staging and recipient capability              | Cross-embargo transfer denied; no bytes staged                             |
| F05 | Critical | License/EULA incompatible with cross-repo reuse                 | SPDX+exact terms+review record; fail closed unknowns                         | Attempt to distribute restricted derivative; deny and audit                |
| F06 | Critical | Denial status leaks forbidden sibling existence                 | Fixed generic non-disclosing worker view and no lateral channel              | Compare worker views with/without embargo; forbidden metadata absent       |
| F07 | High     | Model claims own tests prove behavior                           | Worker test receipts untrusted; oracle authority separate                    | Forged test `pass` cannot promote candidate                                |
| F08 | High     | False benchmark speedup from shared CPU/cache or trace overhead | Exclusive lease, paired trials, ambient checks, trace-off                    | Concurrent interference marks run contaminated                             |
| F09 | High     | Two researchers share model context, appear independent         | View manifests, model lineage and task overlap; replicate independently      | Same model family tagged correlated; no independence claim                 |
| F10 | High     | Conflicting policy epochs during transfer                       | Policy digest bound to permit and commit; re-evaluate on stale               | Rotate embargo mid-transfer; fence and quarantine                          |
| F11 | High     | Host claims capacity then fills before receive                  | Reservation/fencing and target verified bytes                                | Oversized transfer fails without partial committed receipt                 |
| F12 | High     | CAS hash leaks existence of private document                    | Per-tenant access/index, no unauthenticated global `HEAD`                    | Unentitled user can't infer digest presence                                |
| F13 | High     | Content-addressed dedup conflates rights                        | Artifact bytes hash distinct from custody/policy state                       | Same binary across repos has different access grants                       |
| F14 | High     | Revocation treated as wiping prior model memory                 | Avoid premature disclosure; restrict future access/key use                   | Revocation does not claim impossible perfect erasure                       |
| F15 | High     | Policy service error mistakenly permits operation               | Fail closed on timeout/unknown enum/unavailable policy                       | Drop authorization service; transfer stays pending/quarantined             |
| F16 | High     | Neutral error lies about test outcomes                          | Acknowledge submission only; expose correct trusted status to owner          | Generic worker receipt never says test passed                              |
| F17 | High     | Paper contains source secret/license-prohibited code            | Sanitizer + license/evidence checks and human review where needed            | Inject secret into draft; publication denied                               |
| F18 | High     | Knowledge crossing repo treated as proven optimization          | New target repo baseline+oracle+benchmark and separate attestation           | Paper cannot directly promote target code                                  |
| F19 | High     | Source paths/hardlinks/symlinks escape allowed mount            | Canonicalize and openat2-like safe pathname policy; mount namespace          | Tar path traversal and symlink escape rejected                             |
| F20 | High     | Tool features read global cargo cache or credentials            | Controlled offline dependency archive, sandboxed HOME, env allowlist         | Malicious build.rs reads credential path; denied                           |
| F21 | High     | Evidence journal and OTLP disagree                              | Ledger canonical; telemetry diagnostic and droppable                         | Drop all OTLP; state remains correct/replayable                            |
| F22 | High     | Event ordering confused with wall clock                         | Aggregate sequence, causation ID, UTC uncertainty & monotonic timing         | Clock skew on two hosts preserves causal order                             |
| F23 | Medium   | Population converges on one unhelpful family                    | Fixed assignment diversity, specialty cohorts, exploration retention         | Duplicate proposals retained/reused without false novelty                  |
| F24 | Medium   | Covert communication in timings/artifact names                  | Controlled views, fixed output schema, rate/quota monitoring                 | Detect and quarantine output outside schema; cannot promise zero channels  |
| F25 | Medium   | Resource spending seen as benchmark quality                     | Performance acceptance excludes build tokens and inference cost              | Same candidate gets same verdict under different token budgets             |
| F26 | Medium   | Implicit end-user license terms changed by owner                | Versioned legal review and no retroactive grant                              | New terms block future transfer, preserve old record                       |
| F27 | Medium   | Experimental tests miss behavioral mismatch                     | Differential/property/fuzz/side effect tests and preserve uncertainty        | Deliberate mutation passes own test but fails trusted oracle               |
| F28 | Medium   | Infinite recursive loop without useful gain                     | Signed campaign limits/coverage/owner cancellation; no LLM control           | Endless model proposals cannot bypass campaign state                       |
| F29 | Medium   | Duplicate transfer executed after restart                       | Idempotency, unique transfer nonce, fencing, reconciliation                  | Crash before commit; exactly one valid custody receipt                     |
| F30 | Medium   | API becomes a covert LLM tool                                   | No routes/credentials mounted in worker; separate VPC/network namespace      | Agent-supplied HTTP URL has no route or service principal                  |

## Things this architecture cannot guarantee

- **No finite test suite proves arbitrary software completely bug-free.**
  Preserve unknown and inconclusive outcomes.
- **No software policy can erase facts a model already saw.** Minimize
  disclosures before authorizing them; keys can govern future access, not
  recollection.
- **Docker is not a cryptographic enclave from the host administrator.** To hide
  plaintext from the operator, use an external trusted key custodian and
  attested confidential-compute boundary, recognizing their hardware/platform
  limits.
- **A deterministic decision function does not produce deterministic performance
  timings** on an operating system. Treat noisy data statistically.
- **Creative independence cannot be guaranteed** when all workers use the same
  LLM and see overlapping source; keep opinions separated and require external
  experimental confirmation.
- **A signed report can still be wrong** if the signer is compromised or the
  measurement is flawed. Verification includes independent rebuilds, hardware
  calibration, sample integrity, signed fixtures and reproducibility.
- **EULA/licensing compliance cannot be reduced to SPDX strings alone**.
  Ambiguities and negotiated contractual rights require an authorized legal
  owner.
- **No useful service can eliminate all side channels** through timing, resource
  utilization or task selection; bound and document residual leakage.
