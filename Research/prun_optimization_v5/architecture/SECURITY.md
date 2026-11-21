# Integration with v4 Cryptographic Trust Fabric

## 1. Reuse v4, do not create a parallel CA

Use the v4 Rust CA/SSH/mTLS/HPKE prototype as a starting component, subject to
compilation, tests and security review. v5 does NOT imply that v4 has been
deployed, audited or that its keys are protected against the host operator.
Hickory handles DNS discovery and optional DNSSEC; DNS itself is NOT an identity
authority. The certificate authority binds identities/keys, not scientific
claims.

Use short-lived X.509 leaf credentials per _service/worker assignment_; SSH
certificates for administration; Ed25519 signatures for machine-readable claims
and attestations; recipient-only HPKE encrypted envelopes; optional OpenPGP for
archival/exchange interoperability. Avoid ambiguous canonical JSON: define a
versioned deterministic serialization/signing envelope (canonical JSON RFC 8785
or deterministic CBOR), domain separation and key/version rotation.

## 2. Principals, keys and separation

- Offline root: CA policy and bootstrap only; never in online relay.
- Online scoped issuer: short-lived worker identity under a constrained
  intermediate CA.
- Treasury signer: budget authorizations and settlements.
- Researcher signer: claims, bids, contributions.
- Experiment signer: oracle and measurement attestations.
- Publisher signer: sanitized paper release manifest.
- Integrator signer: transfer proposals, never evidence promotions.

An agent may have multiple roles only if an explicit signed policy permits it;
for independent review, enforce distinct assignment IDs, scopes and ideally
independent reviewer/model cohorts. Reject self-review even if the agent swaps a
container name.

## 3. Message envelope

**Public routing header:** message ID, protocol version, sender identity,
recipient identity, task ID (opaque), expiration, content type, size class,
signature/key ID, ciphertext commitment, replay-protection nonce. The
coordinator can verify identity, expiry, anti-replay and role permissions
without decrypting content. Avoid leaking actual research mechanisms in the
header.

**Encrypted payload:** hypothesis, source excerpt, budget bid rationale,
opinion, conversations, private benchmark analysis, paper preprint. Apply
recipient-specific encryption; for multi-recipient audiences, encrypt distinct
message keys to each, and log the recipient set. Recipients sign **their own
assertions**; public-signed attestations refer to artifact commitments. Reject
replay and require transaction idempotency.

## 4. Authorized publication bridge

The system cannot both keep a message secret from everyone, including the
publisher, and promise to review/sanitize its contents. The publisher is an
**authorized recipient** of the minimal submitted evidence only; it receives a
separate signed declassification request, decrypts within its own scoped
workload, sanitizes and signs a derivative. No omniscient observer exists by
design. The publication officer cannot read unrelated private notebooks.

The operator of an ordinary Docker host controls process memory, disks and
images, so hard secrecy _from the host operator_ is not guaranteed by
container-level encryption. For a stronger non-observer claim, use separate
administrative trust domains, external client-held keys, key custody outside the
host and possibly attested confidential compute; document residual threats.
Timing, identity, routing, message sizes and access patterns remain partly
observable.

## 5. Supply-chain and remote-repo safety

All third-party source is untrusted (build scripts, tests, prompts,
documentation, dependencies). Pin source and image digests; default to no
outbound network for executing unknown code; issue network egress only through a
scoped reference-search proxy; never give patchers Docker socket, online CA
keys, treasury signing keys or host SSH credentials. Use rootless worker
containers with seccomp/apparmor/user namespaces; for hostile public
repositories, require stronger VM/microVM isolation and explicit owner
authorization. No blanket guarantee Docker alone is an isolation boundary.

An LLM's fetched references are data, not instructions. Names in citations and
paper templates do not change roles or privileges. Research approvals and
release attestations are distinct cryptographic acts with separately enforceable
signer permissions.
