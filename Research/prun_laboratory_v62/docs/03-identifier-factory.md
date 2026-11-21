# Identifier factory and canonical naming

## Internal typed constraints

Prefer descriptive, canonical, privately stored specifications:

```
IdentifierSpec { alphabet: AsciiRestricted, min_bytes: 1, max_bytes: 128,
                 start_class: AsciiAlnum, continuation_class: AsciiAlnumOr("_.:/-"),
                 normalization: NoTransform, namespace: OpaqueResource }
```

From _one_ type description produce both a Rust byte-level parser and its
optional JSON Schema `pattern` for validation at integration boundaries. Do not
allow worker-defined patterns to modify the authoritative parser. If Unicode
names are ever allowed, state the exact normalization (NFC, case-folding, width
constraints) and reject ambiguous equivalents; test byte vs character count
explicitly.

Example (illustrative Rust API, not compiled):

```rust
struct IdentifierSpec { max_len: usize, allow_slash: bool }
struct OpaqueHandle([u8; 32]);

fn validate_ascii_id(raw: &str) -> bool {
    let bytes = raw.as_bytes();
    if bytes.is_empty() || bytes.len() > 128 || !bytes[0].is_ascii_alphanumeric() {
        return false;
    }
    bytes[1..].iter().all(|b| b.is_ascii_alphanumeric() || b"_.:/-".contains(b))
}
```

This avoids regex compilation in the authorization hot path. A `regex-automata`
builder is still useful for optional generated validators and pattern-based
searches.

## Namespace families

- **Canonical ID:** high-integrity internal identifier, visible only inside the
  registry's authorization domain.
- **Opaque object handle:** 256-bit CSPRNG value with server-side uniqueness
  check, not derived from names.
- **View-scoped pseudonym:** keyed MAC of canonical identity, holder scope and
  epoch, domain-separated by purpose; never exposed as a decryption key.
- **Artifact digest:** cryptographic commitment to exact bytes _within a
  controlled namespace_; exposure can reveal content equality.
- **Capability ID:** issuer-generated identifier paired with a signature,
  proof-of-possession and a current server-side grant; possession of the ID
  alone gives no access.

Avoid a globally reusable pseudonym across dashboards, agents, publications and
telemetry. Correlation across views must require explicit authorization and
access to a private crosswalk. For each namespace, reserve space for
version/epoch without making it a human-readable resource name.

Never accept display labels, filesystem path strings, or JSON Schema references
as authority-bearing identifiers. Resolve from verified handles after
authorization. Canonicalization and uniqueness checks occur before insertion and
before grant evaluation.

## A concrete bug in the original regex proposal

The original `^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$` permits `A\n` under some
regex `$` semantics, because `$` can match before a final newline. The pattern
also permits `A/../`, which is valid as a label according to its character rules
but must never be interpreted directly as a filesystem path. The reference Rust
parser in `rust/lab-identifier/src/lib.rs` rejects the trailing newline at the
byte level. Its generated JSON Schema expression uses an explicit end-of-input
assertion; cross-engine conformance tests are still required before production
use.
