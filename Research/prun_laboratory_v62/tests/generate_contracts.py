from pathlib import Path
import json

ROOT = Path(__file__).resolve().parents[1]
SC = ROOT / "schemas"
EX = ROOT / "examples"
SC.mkdir(exist_ok=True)
EX.mkdir(exist_ok=True)
BASE = "https://contracts.example.invalid/v6.2/"  # Documentation-only placeholder. No network resolution is permitted.
ID = {"type": "string", "pattern": r"^(?:[A-Za-z0-9_-]{43})(?![\s\S])"}
HASH = {"type": "string", "pattern": r"^(?:[a-f0-9]{64})(?![\s\S])"}
TOK = {"type": "string", "pattern": r"^(?:[A-Za-z0-9_-]{16,4096})(?![\s\S])"}
TIM = {"type": "string", "format": "date-time"}


def obj(props, required=None):
    return {
        "type": "object",
        "additionalProperties": False,
        "properties": props,
        "required": required if required is not None else list(props),
    }


def write(n, desc, body, ex):
    schema = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": BASE + n + ".schema.json",
        "title": desc,
        **body,
    }
    (SC / (n + ".schema.json")).write_text(
        json.dumps(schema, indent=2, ensure_ascii=False) + "\n"
    )
    (EX / (n + ".json")).write_text(json.dumps(ex, indent=2, ensure_ascii=False) + "\n")


rid = "A" * 43
rid2 = "B" * 43
rid3 = "C" * 43
hx = "a" * 64
hx2 = "b" * 64


def enum(items):
    return {"type": "string", "enum": items}


write(
    "identifier-definition",
    "Private canonical identifier factory input",
    obj(
        {
            "semantic_kind": enum(
                [
                    "schema",
                    "event",
                    "artifact",
                    "policy",
                    "capability",
                    "correspondence",
                ]
            ),
            "constraint": obj(
                {
                    "alphabet": {"const": "ascii_restricted"},
                    "first": {"const": "ascii_alnum"},
                    "continuation_extra": {"const": "_.:/-"},
                    "min_bytes": {"type": "integer", "minimum": 1, "maximum": 128},
                    "max_bytes": {"type": "integer", "minimum": 1, "maximum": 128},
                    "canonicalization": {"const": "reject_noncanonical"},
                }
            ),
            "definition_hash": HASH,
            "revision": {"type": "integer", "minimum": 1},
        }
    ),
    {
        "semantic_kind": "artifact",
        "constraint": {
            "alphabet": "ascii_restricted",
            "first": "ascii_alnum",
            "continuation_extra": "_.:/-",
            "min_bytes": 1,
            "max_bytes": 128,
            "canonicalization": "reject_noncanonical",
        },
        "definition_hash": hx,
        "revision": 1,
    },
)

write(
    "opaque-envelope",
    "Boundary envelope; semantic content fully encrypted",
    obj(
        {
            "v": {"const": "6.2"},
            "rt": ID,
            "e": {"type": "integer", "minimum": 1},
            "enc": TOK,
            "ct": TOK,
            "s": TOK,
        }
    ),
    {"v": "6.2", "rt": rid, "e": 1, "enc": "A" * 24, "ct": "B" * 32, "s": "C" * 64},
)

write(
    "private-crosswalk",
    "Synthetic plaintext example; deployed record MUST be encrypted",
    obj(
        {
            "scope": ID,
            "opaque_handle": ID,
            "canonical_digest": HASH,
            "audience": ID,
            "epoch": {"type": "integer", "minimum": 1},
            "purpose": enum(
                ["measurement", "orchestration", "custody", "audit", "operator"]
            ),
            "mapping_cipher_digest": HASH,
            "expiry": TIM,
        }
    ),
    {
        "scope": rid,
        "opaque_handle": rid2,
        "canonical_digest": hx,
        "audience": rid3,
        "epoch": 1,
        "purpose": "custody",
        "mapping_cipher_digest": hx2,
        "expiry": "2027-01-01T00:00:00Z",
    },
)

write(
    "proof-bound-grant",
    "Capability bound to an authenticated workload key and current policy",
    obj(
        {
            "grant_id": ID,
            "issuer": ID,
            "subject": ID,
            "resource": ID,
            "action": enum(
                [
                    "resolve",
                    "read",
                    "hold",
                    "stage",
                    "decrypt",
                    "append",
                    "audit",
                    "activate",
                ]
            ),
            "purpose": enum(
                ["build", "test", "measure", "research", "audit", "operator", "custody"]
            ),
            "proof_key_digest": HASH,
            "policy_digest": HASH,
            "epoch": {"type": "integer", "minimum": 1},
            "not_before": TIM,
            "expires": TIM,
            "signature": TOK,
        }
    ),
    {
        "grant_id": rid,
        "issuer": rid2,
        "subject": rid3,
        "resource": rid,
        "action": "read",
        "purpose": "custody",
        "proof_key_digest": hx,
        "policy_digest": hx2,
        "epoch": 1,
        "not_before": "2026-11-21T00:00:00Z",
        "expires": "2026-11-21T01:00:00Z",
        "signature": "S" * 72,
    },
)

write(
    "audit-commitment",
    "Signed append-only record commitment with lineage binding",
    obj(
        {
            "sequence": {"type": "integer", "minimum": 0},
            "previous": HASH,
            "event_ciphertext_digest": HASH,
            "producer": ID,
            "epoch": {"type": "integer", "minimum": 1},
            "policy_digest": HASH,
            "timestamp": TIM,
            "signature": TOK,
        }
    ),
    {
        "sequence": 1,
        "previous": "0" * 64,
        "event_ciphertext_digest": hx,
        "producer": rid,
        "epoch": 1,
        "policy_digest": hx2,
        "timestamp": "2026-11-21T00:00:00Z",
        "signature": "S" * 72,
    },
)

write(
    "migration-transition",
    "Signed immutable schema/policy migration transition",
    obj(
        {
            "migration_id": ID,
            "previous_head": HASH,
            "schema_before": HASH,
            "schema_after": HASH,
            "policy_after": HASH,
            "compiler_digest": HASH,
            "epoch_before": {"type": "integer", "minimum": 1},
            "epoch_after": {"type": "integer", "minimum": 2},
            "phase": enum(["prepared", "activated", "aborted"]),
            "approver": ID,
            "signature": TOK,
        }
    ),
    {
        "migration_id": rid,
        "previous_head": hx,
        "schema_before": hx,
        "schema_after": hx2,
        "policy_after": hx,
        "compiler_digest": hx,
        "epoch_before": 1,
        "epoch_after": 2,
        "phase": "prepared",
        "approver": rid2,
        "signature": "S" * 72,
    },
)

write(
    "holder-provision",
    "Trusted supervisor statement; NOT worker-asserted",
    obj(
        {
            "holder": ID,
            "image_digest": HASH,
            "role": enum(
                [
                    "schema_factory",
                    "opaque_resolver",
                    "key_broker",
                    "identity_issuer",
                    "policy_engine",
                    "journal",
                    "telemetry_control",
                    "telemetry_product",
                    "operator_gateway",
                    "offline_supervisor",
                    "llm_worker",
                    "builder",
                    "oracle",
                    "benchmark",
                ]
            ),
            "network_class": enum(["offline", "internal", "operator"]),
            "allowed_secret_classes": {
                "type": "array",
                "uniqueItems": True,
                "items": enum(
                    [
                        "none",
                        "workload_cert",
                        "scoped_signer",
                        "artifact_dek",
                        "intermediate_ca",
                        "private_crosswalk",
                    ]
                ),
            },
            "attestation_digest": HASH,
            "policy_digest": HASH,
            "valid_until": TIM,
        }
    ),
    {
        "holder": rid,
        "image_digest": hx,
        "role": "llm_worker",
        "network_class": "offline",
        "allowed_secret_classes": ["none"],
        "attestation_digest": hx2,
        "policy_digest": hx,
        "valid_until": "2026-11-22T00:00:00Z",
    },
)

write(
    "genesis-commitment",
    "Public commitment, not an encryption seed",
    obj(
        {
            "protocol_revision": {"const": "6.2"},
            "schema_digest": HASH,
            "policy_digest": HASH,
            "first_event_commitment": HASH,
            "issuer_public_key_digest": HASH,
            "created_at": TIM,
            "governance_signature": TOK,
        }
    ),
    {
        "protocol_revision": "6.2",
        "schema_digest": hx,
        "policy_digest": hx2,
        "first_event_commitment": "0" * 64,
        "issuer_public_key_digest": hx2,
        "created_at": "2026-11-21T00:00:00Z",
        "governance_signature": "G" * 72,
    },
)
print("Generated 8 additive schemas and examples")
