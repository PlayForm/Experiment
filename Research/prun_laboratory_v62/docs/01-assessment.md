# Proposal assessment: what is valuable, what must change

## 1. Generate regular expressions through Rust

Accept as a developer-experience and type-safety improvement, **not** as a
security guarantee. A regex builder is safe only when input constraints are
bounded, escaped where needed, and validated. Prefer typed `IdentifierSpec`
constraints and a generated validator on byte slices for ASCII identifiers. The
schema can expose a pattern generated from those constraints; never make
arbitrary LLM-supplied regex part of the trusted authorization path. Compile
once, limit input length, and assert agreement among the Rust validator, JSON
Schema validator, and test vectors. Rust `regex`/`regex-automata` uses finite
automata with bounded search complexity but size/compilation resource limits
still matter.

## 2. Encrypt paths, references, policy and field names

Accept as _confidential schema metadata_. Do not encrypt `$ref` values in-place
and expect standard JSON Schema to work: `$ref` is a URI reference. Use a sealed
canonical schema registry and a deterministic compile/resolve stage that emits
internal handles or opaque URNs for authorized services. At network boundaries
put the entire semantic payload inside authenticated encryption and expose a
stable generic envelope; transport still needs visible protocol version and an
opaque routing handle. An opaque identifier is not inherently secret or
unguessable authorization.

## 3. Factory + memory arena for policy fields and event types

Accept only with immutable authored schema versions, deterministic compiler
behavior, validation against historical versions, and policy tests. Use arenas
for temporary parse/compile state; freeze long-lived read-only snapshots and
atomically swap versions after successful migration. Do not retain references
across arena reclamation. Factory-generated schema field tokens may be
per-holder or per-epoch pseudonyms, but _every_ consuming service must agree on
semantic meaning through the private registry. A factory must not independently
improvise authorization semantics.

## 4. One seed to reconstruct the system

Reject as a universal root for encryption, certificate issuance, identity, and
event history: one compromise would expose every domain and make independent
revocation impossible. Instead use one **genesis commitment** to attest the
starting state, independently generated secret material for separate key
authorities, and explicit epoch records. HKDF may derive domain-separated
subkeys from a _specific securely held key_ for an approved scope; it cannot
create independent entropy. Preserve an encrypted append-only journal, sealed
snapshots and external recovery shares. Memory-only history is lost after
reboot.

## 5. A key plus usage seed as zero-knowledge

Reject the terminology. Possessing two secrets is not a zero-knowledge proof. To
prevent a stolen token from granting authority use proof-of-possession-bound,
short-lived capabilities and trusted policy evaluation; non-exportable
HSM/TPM-backed keys are appropriate for privileged signing operations. Real ZK
proofs are justified only if a verifier must establish a property without
learning its inputs. With a compromised authorized endpoint, plaintext can be
observed after decryption. There is no system that is 'completely non-hackable.'

## Collision warning

Do not waive collisions for object identities, idempotency, cache keys,
authorization, commitments or policy lookup. Identical tokens for different
resources are a critical incident. Use OS-backed randomness for ephemeral IDs,
cryptographically strong hashes/MACs with domain separation for derived
pseudonyms, uniqueness enforcement and audit on collision. Also design for
semantic collisions: two different namespaces that accidentally canonicalize to
the same bytes.
