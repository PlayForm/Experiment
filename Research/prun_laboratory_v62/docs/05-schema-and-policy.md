# Confidential schema, event and policy factory

The compiler consumes an **authored, reviewed semantic IR** in a privately
encrypted registry. Example kinds are `SchemaResource`, `FieldSpec`,
`Reference`, `PolicyRule`, `EventType`, `Migration`, `Permission`,
`RetentionPolicy` and `DisclosureView`. Each has an immutable semantic
identifier and version.

Compilation pipeline:

1. Author and sign canonical IR and migrations (human-governed or trusted
   software; LLM submissions cannot alter trusted IR).
2. Validate uniqueness, types, full reference closure, canonicalization and
   backwards-compatibility rules; reject ambiguous or cyclic dependencies that
   cannot be resolved.
3. Compile Rust serde/typed validators and deterministic event/policy matching
   tables; compile an internal JSON Schema bundle if needed. A standard `$ref`
   can target an opaque `urn:lab:schema:<id>` only when the trusted resolver
   knows the mapping.
4. Produce an encrypted/private symbol table mapping canonical semantic IDs to
   internal code identifiers and view-scoped wire tokens. The key broker is not
   the compiler and need not reveal raw key material.
5. Sign a build manifest binding code digest, source IR digest, compiler
   version, resolver closure, policy version, key epoch and migration state.
6. Deploy to trusted services with staged compatibility checks and atomic
   activation after a signed authorization decision. Retain old schema readers
   until retention obligations expire.

The factory should not randomize internal executable field layouts at every
restart. That would impair interoperability and replay. Randomness belongs
primarily in externally exposed handles, encryption nonces and purpose-scoped
pseudonyms; semantics and validation remain deterministic.

Keep _authorization_ separate from encryption. A Cedar-style Rust policy engine
can evaluate principal, action, resource and context, but key release also
requires identity verification, possession proof, quota/holder qualification,
current grants and replay protection.

## On `required` arrays

The presence of a field name such as `embargo_id` in a _private design-time
schema_ does not itself permit bypass. Keeping such names readable in the
authoritative IR improves auditability. Conceal those names from workers and
public projections by encrypting the payload or compiling into a private binary;
don't invent random names as a substitute for enforced property requirements.

Public interfaces may use a fixed small envelope (`v`, `rt`, `e`, `body`) and
opaque values; all meaningful fields live in the authenticated ciphertext. JSON
Schema cannot express correctness of the cryptographic operations; tests and the
Rust runtime must enforce that.

## Compiled binaries do not automatically conceal schema names

A Rust executable may contain literal field names, assertion strings, `serde`
metadata, function names, debugging symbols and embedded schemas. Stripping
symbols does not reliably erase all those values. If a party is allowed to
inspect that binary, private names must not be compiled into it in plaintext.
Keep the semantic registry in a separately protected encrypted store and compile
only opaque numeric discriminants or runtime-resolved handles into components
that do not require the canonical names. Even then, authorized computations and
error behavior can leak structural information through probing and side
channels.
