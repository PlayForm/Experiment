# Validation, pitfalls, verification levels and acceptance gates

## Threat assumptions

**T1:** compromised LLM process or generated Rust code. Must not reach
controller/keys/holdouts/network. **T2:** compromised trusted online
microservice. Limit to its capability, audit and rotate. **T3:** curious
operator of ordinary dashboard. Redact mappings. **T4:** privileged host owner.
Docker and memory-only secrecy are insufficient; require separate hardware/TEE
if protected against host. **T5:** lost disk or backups. Encrypted artifacts and
separated key escrow. **T6:** malicious dependency/build script. Offline
build/test isolation, pinned toolchain and lockfile, no secret mounts. **T7:**
attacker sees protocol source. Authorization remains effective with known
design.

## Security-critical negative tests

1. Submit invalid identifiers: `../`, percent encoding, NUL, Unicode
   confusables, overlong text; ensure typed validator and schema implementation
   agree.
2. Exhaustively test field canonicalization and URI-reference resolution: no
   accidental traversal or remote fetch. Unknown external `$ref` is rejected.
3. Force collision by injecting a mocked ID generator; reservation must abort
   without overwriting any record or grant.
4. Create pseudonyms across two audiences and key epochs for the same canonical
   object: verify unlinkability by construction under distinct secrets, and that
   only authorized resolver can rejoin.
5. Swap ciphertext recipients, contexts, nonces, schema epochs, and signed event
   IDs: decryption or verification must fail.
6. Present a valid signed capability to wrong workload identity; require
   proof-of-possession and policy check; reject.
7. Expired, replayed or revoked capabilities never authorize operations,
   including after a reboot.
8. Attack schema migration with rollback, reordered event, truncated journal,
   duplicate activation and concurrent writers: reject or safely reconcile.
9. Inject a malicious tool output or shell metacharacters as schema names; no
   command execution, remote resolution or compiler input injection.
10. Run worker with no network: confirm no DNS, Docker socket, host CA keys,
    Unix privileged sockets, `/proc` host secrets or mount escape.
11. Attempt to access each telemetry plane from the worker and to recover
    private mapping via public aggregates; document any correlation leakage.
12. Ensure expired custody lease and revoked artifact rights prevent any _new_
    access; disclose limits of revocation of previously consumed plaintext.
13. For crash recovery, restore from sealed backup and verify old key epochs,
    schema resolver closure and exact lineage head.
14. Validate product correctness and performance benchmark equivalence
    independently; no trusted test or result may originate from a candidate
    patch.
15. Failure at each service boundary must be logged in protected audit without
    revealing hidden policy to the untrusted worker.

## Evidence levels

- **Design verified:** standards and contract relationships assessed,
  limitations documented.
- **Schema verified:** JSON Schema examples, required field and negative tests
  pass.
- **Code verified:** Rust unit/property/fuzz tests, static review, crypto
  libraries used correctly.
- **Deployment verified:** real Docker egress, mount, TLS, key isolation and
  host threat tests pass.
- **Security audited:** independent review, penetration tests and supply-chain
  checks completed.

This package demonstrates only **design review plus schema-level validation**.
Neither cryptographic nor operational security has been proven.

## Cross-field checks the schema does not perform

Before enabling a grant, Rust must compare actual timestamps
(`not_before < expires`), ensure the issuer key is trusted, authenticate the
caller's proof key, check key epoch and revocation, enforce a recognized
resource/action/purpose association, and verify the signature on a canonical
byte representation. Likewise, an identifier spec needs
`min_bytes <= max_bytes`; a migration needs `epoch_after > epoch_before`; a
holder provision must prohibit all key classes for `llm_worker`. These semantic
invariants are deliberately **not proven** by the example JSON Schemas. Do not
add a `signature` property and assume it was verified.

## Metadata-exposure tests

Compile a trusted Rust binary and run `strings`/symbol inspection against it. If
canonical private names or policies are embedded as plaintext constants, the
binary can leak them regardless of randomized `$ref` strings or `strip` flags.
Use a trusted encrypted registry and load authorized private resources at
runtime, not secrecy through binary obfuscation. Test DNS zone enumeration and
TLS certificate SAN content too; private service names should not be put in
publicly issued certificates. Cert issuance transparency should use protected
audit records instead of publicly exposing service identities.
