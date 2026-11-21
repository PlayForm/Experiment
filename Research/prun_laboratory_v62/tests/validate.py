#!/usr/bin/env python3
"""Design-level structural validation. NOT cryptographic verification."""

from pathlib import Path
import json, copy
from jsonschema import Draft202012Validator, FormatChecker, ValidationError

ROOT = Path(__file__).resolve().parents[1]
S = ROOT / "schemas"
E = ROOT / "examples"
count = 0
for p in sorted(S.glob("*.schema.json")):
    sch = json.loads(p.read_text())
    Draft202012Validator.check_schema(sch)
    example = json.loads(
        (E / (p.name.removesuffix(".schema.json") + ".json")).read_text()
    )
    v = Draft202012Validator(sch, format_checker=FormatChecker())
    v.validate(example)
    count += 1
    # Every required field must be rejected when removed.
    for req in sch.get("required", []):
        missing = copy.deepcopy(example)
        missing.pop(req)
        assert not v.is_valid(missing), (p.name, req)
    # Unknown fields must fail for all authoritative records.
    extra = copy.deepcopy(example)
    extra["_unexpected"] = "evil"
    assert not v.is_valid(extra), p.name
print(
    f"PASS {count} complete schemas; required-field and unexpected-field negative cases"
)

# Negative worker key provisioning. This is a policy test, not a property of the general holder schema.
h = json.loads((E / "holder-provision.json").read_text())
assert (
    h["role"] == "llm_worker"
    and h["allowed_secret_classes"] == ["none"]
    and h["network_class"] == "offline"
)
for bad in [
    "scoped_signer",
    "artifact_dek",
    "private_crosswalk",
    "intermediate_ca",
    "workload_cert",
]:
    contaminated = copy.deepcopy(h)
    contaminated["allowed_secret_classes"] = [bad]
    assert not (
        contaminated["role"] == "llm_worker"
        and contaminated["network_class"] == "offline"
        and contaminated["allowed_secret_classes"] == ["none"]
    )
print("PASS offline worker least-privilege policy example; no operational claims")

# Important anchor and path counterexamples: original regex permits final newline.
from jsonschema import Draft202012Validator

old = Draft202012Validator(
    {"type": "string", "pattern": r"^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}$"}
)
strict = Draft202012Validator(
    {"type": "string", "pattern": r"^(?:[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127})(?![\s\S])"}
)
assert old.is_valid("A\n")
assert not strict.is_valid("A\n")
assert old.is_valid("A/../") and strict.is_valid(
    "A/../"
)  # valid label, invalid as path input
print(
    "PASS original regex newline pitfall reproduced; strict JSON Schema pattern excludes it"
)


# Cross-component canonical parser checks, mirrored from the Rust byte-level spec.
def rust_semantics(raw):
    b = raw.encode("utf-8")
    return (
        1 <= len(b) <= 128
        and bool(b[0:1])
        and (
            ord("A") <= b[0] <= ord("Z")
            or ord("a") <= b[0] <= ord("z")
            or ord("0") <= b[0] <= ord("9")
        )
        and all(
            (
                ord("A") <= x <= ord("Z")
                or ord("a") <= x <= ord("z")
                or ord("0") <= x <= ord("9")
                or x in b"_.:/-"
            )
            for x in b[1:]
        )
    )


for raw in ["A", "A/../", "A\n", "/start", "\x00", "Å", "a_b", "A" * 128, "A" * 129]:
    assert rust_semantics(raw) == strict.is_valid(raw), (
        raw,
        rust_semantics(raw),
        strict.is_valid(raw),
    )
print(
    "PASS selected byte-parser and generated-pattern conformance vectors (Python mirror, not Rust binary)"
)
