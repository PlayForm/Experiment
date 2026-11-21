# v6.1 → v6.2 compatibility and first build

No v6.1 endpoint, event, offline task/submission schema, or SQL table is
replaced by this proposal. Instead, add a new authority **plane -1** and a
generic encrypted message transport for service-to-service operations that
require confidential metadata. Services that require canonical event types and
legacy schema refs translate them only within the trusted private registry. The
public operator API may continue returning names for specifically authorized
views, rather than making every operator blind and destroying usability.

### Implementation sequence

1. Implement `lab-identifier` and a cross-language conformance suite; use typed
   identifiers rather than paths or pattern strings on the authorization path.
2. Implement a private canonical schema registry, bounded compiler IR and
   reference resolver. Reject remote `$ref` fetches and unknown namespaces.
   Produce deterministic signed manifests.
3. Implement opaque handle issuance with uniqueness enforcement and
   audience/epoch scoping; store the encrypted crosswalk separately from
   telemetry.
4. Implement the policy engine with owner-approved signed grants and current
   workload proof, before any key release.
5. Implement the key broker using mature Rust crypto libraries with independent
   key domains; bind HPKE encryption to message version, sender, recipient,
   purpose, and schema epoch.
6. Implement encrypted durable journal and memory projections; test
   power-failure/replay/rotation/rollback behavior.
7. Add service workloads and trust isolation one binary at a time; provision
   only the credentials each service requires. The offline LLM image receives no
   change.
8. Add redacted telemetry projections and private-mapping access audit. Do not
   modify trace-off SUT binaries merely to encrypt their instrumentation
   metadata; keep trace-off authoritative.
9. Run negative and chaos tests against all trust boundaries, including
   compromised builder scripts and host-level limitations.
10. Obtain independent cryptographic and security review before using the system
    for sensitive third-party repositories.

### Deployment prerequisites

- Dedicated host(s), fixed Docker/OCI runtime policy, rootless or stronger
  isolation for untrusted builds and binaries, denied host Docker socket access.
- Local inference weights, cached dependencies and no network egress for
  untrusted agents.
- Out-of-band issuer bootstrap, key recovery plan, encrypted backups and policy
  governance.
- Trusted operator identity and separate private schema/mapping audit access.
- Persistent artifact store independent of in-memory hot indexes.

**This is a design build order, not evidence that any of these Rust services
currently exist.**
