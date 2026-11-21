# v4 -> v5 Trust Bridge: Signed Claims, Opaque Research Capsules and Scoped Publishing

The v4 prototype has CLI operations for initializing a CA, issuing identity
bindings, SSH certificate issuance, encrypting and signing HPKE envelopes,
verifying and opening messages, and a length-prefixed mTLS relay (it is **not**
HTTP/HTTPS). The implementation has **not** been compiled or deployment-tested
in the current execution environment. Its current storage and relay code must be
reviewed, tested for race/replay/revocation/key-rotation and hardened before use
beyond development.

## Role-specific message types

Extend the v4 attested identity registry's `allowed_message_types` with
purpose-specific kinds. The following are proposed **envelope payload
contracts**, not already wired into v4 binaries:

| Sender               | Type                        | Authorized recipient                 | Required evidence                                               |
| -------------------- | --------------------------- | ------------------------------------ | --------------------------------------------------------------- |
| Coordinator          | `research.case.assign`      | Named researcher                     | Signed task, budget limits, context scope                       |
| Researcher           | `research.bid.submit`       | Exchange                             | Schema-validated bid and preregistered forecast                 |
| Researcher           | `research.claim.submit`     | Claim registry                       | Claim ID, context commitment, signed opinion                    |
| Treasurer            | `resource.reserve.receipt`  | Assigned worker + coordinator        | Transaction ID, policy version, idempotency ID                  |
| Experiment evaluator | `experiment.result.attest`  | Claim registry / approved researcher | Oracle, signed samples and cohort identity                      |
| Researcher           | `publication.submit`        | Scoped scientific publisher          | Minimal recipient-encrypted evidence capsule                    |
| Publisher            | `publication.release`       | Corpus index                         | Sanitized paper hash, declassification grant, release signature |
| Integrator           | `research.transfer.propose` | Research exchange                    | Paper source, new destination case, budget request              |

Use a `payload_schema_id`, message type-specific signer capability and policy
version. Authenticated identity alone does not authorize funding, publication or
release; reject a message unless both identity+role and required payload
attestations pass.

## Disclosure policy

The relay can see metadata (sender, recipient, kind, size, timestamps) and
ciphertext; it cannot read HPKE payloads **if it does not possess recipient
private keys and endpoints are not compromised**. Researcher notebooks are
encrypted for that researcher only. The science publisher receives only
artifacts expressly submitted to it. The treasury sees quantities, claim IDs and
signed receipts; it does not receive hidden reasoning. A researcher can use
local token budget without making every token public.

The public paper hash is content-addressed; the private artifact link can be
kept in an access-controlled repository-specific ledger. No telemetric attribute
should contain plaintext hypotheses or confidential code. The source-check
authority can examine scoped code; the publisher cannot silently read the entire
private repository.

## Revocation and key rotation

Short-lived keys per assignment, signed issuance log, issuer key ID and rotation
version, revocation list, expiration, replay cache and publication-timestamp
checks. Publication signatures must be verifiable against historical issuer keys
even after expiration; freshness of a worker's mTLS credential and historical
validity of a signed paper are separate questions.

## No conflation of CA signatures and scientific validation

Certificate issuance attests identity/key binding, not benchmark correctness;
experimental attestations have different principal, evidence policy and signer.
A published paper may have correct signatures and still be methodologically
invalid. The research economy rewards independent measurement and calibrated
prediction, not signature count.
