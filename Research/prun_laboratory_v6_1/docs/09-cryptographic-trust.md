# Cryptographic trust, PGP/SSH/HTTPS and DNS

## A private trust fabric, not one magic certificate

- **Offline root CA** issues limited-lifetime intermediates. Offline root
  signing occurs outside routine worker/runtime containers. Online issuers have
  separate keys for TLS identity, SSH certificates, recipient-key attestation,
  and evidence/attestation signing.
- **TLS service identity**: Rustls-backed mTLS with service-specific SAN/URI
  identity and a separate policy engine. An authenticated TLS identity alone
  does not authorize an endpoint.
- **SSH administration**: OpenSSH-compatible short-lived user/host certificates
  via Rust implementation (`russh` or compatible library) for _trusted operators
  only_. There is no SSH server in the LLM containers.
- **Recipient message encryption**: signed outer envelope plus HPKE encryption
  for a named recipient; sender and recipient keys distinct from CA root.
  Routing layer verifies scope/expiry/nonce/digest, recipient verifies inner
  content after decryption. Coordinator need not decrypt private notebooks.
- **OpenPGP interoperability**: optional independent RFC 9580-compatible PGP
  format and certification adapter; do not call HPKE envelopes “PGP.” Key
  semantics/formats are different.
- **Hickory DNS**: private authoritative service discovery and optional DNSSEC.
  DNS name discovery alone is not proof of service identity; mTLS plus
  authorization are required.
- **Certificate transparency**: ledger records
  `requested/issued/renewed/revoked/expired/denied`, issuer, subject, scope
  digest, timestamp, serial, key fingerprint, reason and event ID. Raw
  certificates are content-addressed. Dashboards show counts without private
  keys.

## Signature and attestation matrix

| Signed assertion    | Signer                                | Must bind                                                        |
| ------------------- | ------------------------------------- | ---------------------------------------------------------------- |
| Task assignment     | Coordinator/scheduler                 | task digest, supervisor ID, lease/fence, expiry                  |
| Research proposal   | Assigned worker identity / supervisor | task ID, patch/claim digest, nonce, timestamp; opinion only      |
| Build result        | Builder                               | candidate source tree, toolchain, deps, recipe, binary hash      |
| Correctness result  | Oracle                                | candidate/reference binaries, trusted tests, fixture manifest    |
| Benchmark result    | Runner                                | binary pair, workload, hardware, raw samples, protocol, lease    |
| Evaluation result   | Evaluator                             | frozen analysis policy, raw sample hash, correctness attestation |
| Paper release       | Publication authority                 | sanitized paper bytes, claim-to-evidence map and scope           |
| Credential issuance | PKI issuer                            | serial, subject, scope, public key fingerprint, expiry, reason   |
| User policy/command | Owner identity                        | requested action, target, revision, policy digest                |

Use canonical deterministic serialization and domain-separated signature
contexts: `lab.v6.build`, `lab.v6.oracle`, `lab.v6.benchmark`, `lab.v6.publish`,
etc. Never accept a signature for a different context as equivalent. Revocation
affects new authorizations; historic evidence should retain the credential's
validity _at signing time_ plus revocation timeline.

## Message envelope

`schemas/envelope.schema.json` covers sender/recipient, nonce, expiry,
ciphertext artifact, signature scope, correlation and privacy class. Relay can
prove the sender authorized this routing decision, without proving plaintext
correctness. Never export content or key material through OTLP.

## Limits to confidentiality

A machine administrator with root access can generally inspect memory, mounted
plaintext and keys inside containers. A local owner who controls the host cannot
be cryptographically prevented from examining everything solely through Docker
or Rustls. The design keeps agent conversations opaque to the **normal observer
API and coordinator role**; stronger confidentiality from the host operator
would require separate trusted hardware, externally held keys or a mutually
distrusting host setup. Metadata (timing, source/target IDs, size and event
counts) may still leak information.

## Key rotation/failure

Short TTL service identities, regular rotation, event-sourced revocation,
fail-closed authentication, independently auditable issuer log, clock
synchronization and recovery ceremony. If issuer unavailable, running signed
measurements can complete according to policy, but no new identity issuance. CA
cannot grant itself benchmark or oracle signing roles automatically.
