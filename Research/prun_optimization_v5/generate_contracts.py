import json
from pathlib import Path

R = Path(__file__).parent
S = R / "schemas"
E = R / "examples"
S.mkdir(exist_ok=True)
E.mkdir(exist_ok=True)
H = {"type": "string", "pattern": "^[a-f0-9]{64}$"}
T = {"type": "string", "format": "date-time"}
ID = {"type": "string", "minLength": 1, "maxLength": 128}
RES = {
    "type": "object",
    "additionalProperties": False,
    "required": [
        "input_tokens",
        "output_tokens",
        "cpu_core_seconds",
        "benchmark_seconds",
        "gpu_seconds",
        "storage_byte_days",
        "reference_requests",
    ],
    "properties": {
        k: {"type": "integer", "minimum": 0}
        for k in [
            "input_tokens",
            "output_tokens",
            "cpu_core_seconds",
            "benchmark_seconds",
            "gpu_seconds",
            "storage_byte_days",
            "reference_requests",
        ]
    },
}


def obj(fields, required=None, extra=False):
    return {
        "type": "object",
        "required": required or list(fields),
        "properties": fields,
        "additionalProperties": extra,
    }


def emit(n, fields, example, required=None):
    schema = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": f"https://example.invalid/prun-lab/{n}.v5.schema.json",
        "title": f"Prun Research Economy {n} v5",
        **obj(fields, required),
    }
    (S / f"{n}.v5.schema.json").write_text(json.dumps(schema, indent=2) + "\n")
    (E / f"{n}.v5.json").write_text(json.dumps(example, indent=2) + "\n")


sha = lambda x: x * 64
emit(
    "research-case",
    {
        "version": {"const": 5},
        "case_id": ID,
        "repo_id": ID,
        "campaign_id": ID,
        "parent_mutation_id": ID,
        "source_commit": ID,
        "source_scope_sha256": H,
        "workload_id": ID,
        "state": {
            "enum": [
                "opened",
                "blind_assigned",
                "preregistered",
                "measured",
                "evaluated",
                "reviewed",
                "published",
                "closed",
            ]
        },
        "question": {"type": "string", "minLength": 10},
        "hypothesis_family": ID,
        "roles": {
            "type": "array",
            "minItems": 2,
            "uniqueItems": True,
            "items": {
                "enum": [
                    "proponent",
                    "skeptic",
                    "alternative",
                    "replicator",
                    "methodologist",
                    "librarian",
                ]
            },
        },
        "privacy": {"enum": ["private", "tenant", "public"]},
        "holdout_policy": {"enum": ["sealed", "development_only", "none"]},
        "budget_ceiling": RES,
        "created_utc": T,
    },
    {
        "version": 5,
        "case_id": "case-1",
        "repo_id": "playform-run",
        "campaign_id": "campaign-1",
        "parent_mutation_id": "mutation-0",
        "source_commit": "abc1234",
        "source_scope_sha256": sha("a"),
        "workload_id": "wide-traversal",
        "state": "opened",
        "question": "Does an alternate directory walker improve traversal without changing results?",
        "hypothesis_family": "directory-walking",
        "roles": ["proponent", "skeptic", "alternative", "replicator"],
        "privacy": "private",
        "holdout_policy": "sealed",
        "budget_ceiling": {
            "input_tokens": 120000,
            "output_tokens": 40000,
            "cpu_core_seconds": 12000,
            "benchmark_seconds": 3000,
            "gpu_seconds": 0,
            "storage_byte_days": 1000000,
            "reference_requests": 200,
        },
        "created_utc": "2026-11-21T00:00:00Z",
    },
)
emit(
    "research-bid",
    {
        "version": {"const": 5},
        "bid_id": ID,
        "case_id": ID,
        "bidder_identity": ID,
        "stance": {
            "enum": [
                "proponent",
                "skeptic",
                "alternative",
                "replicator",
                "methodologist",
                "librarian",
            ]
        },
        "snapshot_hash": H,
        "proposal_hash": H,
        "requested": RES,
        "mechanism_key": ID,
        "prediction": obj(
            {
                "event": {"type": "string", "minLength": 8},
                "probability": {"type": "number", "minimum": 0, "maximum": 1},
                "registered_before_outcome": {"const": True},
            }
        ),
        "information_gain_basis": {"type": "string", "minLength": 8},
        "source_coverage_note": {"type": "string"},
        "model_cohort_id": ID,
        "created_utc": T,
        "signature_b64": {"type": "string", "minLength": 16},
    },
    {
        "version": 5,
        "bid_id": "bid-1",
        "case_id": "case-1",
        "bidder_identity": "researcher-proponent-1",
        "stance": "proponent",
        "snapshot_hash": sha("b"),
        "proposal_hash": sha("c"),
        "requested": {
            "input_tokens": 12000,
            "output_tokens": 4000,
            "cpu_core_seconds": 1000,
            "benchmark_seconds": 0,
            "gpu_seconds": 0,
            "storage_byte_days": 50000,
            "reference_requests": 10,
        },
        "mechanism_key": "walkdir-parallel",
        "prediction": {
            "event": "candidate passes oracle",
            "probability": 0.72,
            "registered_before_outcome": True,
        },
        "information_gain_basis": "Compare wide and deep trees against reference.",
        "source_coverage_note": "Only source traversal module",
        "model_cohort_id": "model-cohort-a",
        "created_utc": "2026-11-21T00:01:00Z",
        "signature_b64": "DEMO_UNVERIFIED_SIGNATURE",
    },
)
emit(
    "claim",
    {
        "version": {"const": 5},
        "claim_id": ID,
        "case_id": ID,
        "researcher_identity": ID,
        "stance": {
            "enum": [
                "proponent",
                "skeptic",
                "alternative",
                "replicator",
                "methodologist",
                "librarian",
            ]
        },
        "claim_type": {
            "enum": [
                "opinion",
                "hypothesis",
                "external_reference",
                "observation",
                "interpretation",
                "replication",
            ]
        },
        "body": {"type": "string", "minLength": 12},
        "confidence": {"type": "number", "minimum": 0, "maximum": 1},
        "supports": {"type": "array", "items": ID},
        "contradicts": {"type": "array", "items": ID},
        "evidence_artifact_sha256": {
            "type": ["string", "null"],
            "pattern": "^[a-f0-9]{64}$",
        },
        "source_ref_id": {"type": ["string", "null"]},
        "context_snapshot_sha256": H,
        "registered_utc": T,
        "signature_b64": {"type": "string", "minLength": 16},
    },
    {
        "version": 5,
        "claim_id": "claim-1",
        "case_id": "case-1",
        "researcher_identity": "researcher-skeptic-1",
        "stance": "skeptic",
        "claim_type": "hypothesis",
        "body": "Unbounded concurrency may increase context switches on deep trees.",
        "confidence": 0.61,
        "supports": [],
        "contradicts": [],
        "evidence_artifact_sha256": None,
        "source_ref_id": None,
        "context_snapshot_sha256": sha("d"),
        "registered_utc": "2026-11-21T00:02:00Z",
        "signature_b64": "DEMO_UNVERIFIED_SIGNATURE",
    },
)
emit(
    "paper",
    {
        "version": {"const": 5},
        "paper_id": ID,
        "repo_id": ID,
        "case_ids": {"type": "array", "minItems": 1, "uniqueItems": True, "items": ID},
        "status": {
            "enum": [
                "hypothesis_note",
                "internal_observation",
                "measured_lab_report",
                "replicated_report",
                "sanitized_preprint",
                "externally_reviewed",
                "corrected",
                "retracted",
            ]
        },
        "title": {"type": "string", "minLength": 12},
        "abstract": {"type": "string", "minLength": 20},
        "claim_links": {
            "type": "array",
            "minItems": 1,
            "items": obj(
                {
                    "claim_id": ID,
                    "evidence_ids": {"type": "array", "uniqueItems": True, "items": ID},
                    "evidence_status": {
                        "enum": [
                            "unsupported",
                            "external_reference",
                            "oracle_checked",
                            "measured",
                            "independently_replicated",
                        ]
                    },
                }
            ),
        },
        "baseline_artifact_sha256": H,
        "methods_artifact_sha256": H,
        "manuscript_sha256": H,
        "limitations": {"type": "array", "minItems": 1, "items": {"type": "string"}},
        "negative_results": {"type": "array", "items": ID},
        "sanitization_status": {"enum": ["pending", "passed", "denied"]},
        "declassification_grant_id": {"type": ["string", "null"]},
        "publisher_identity": ID,
        "created_utc": T,
        "signature_b64": {"type": "string", "minLength": 16},
    },
    {
        "version": 5,
        "paper_id": "paper-1",
        "repo_id": "playform-run",
        "case_ids": ["case-1"],
        "status": "internal_observation",
        "title": "Directory walker cost on wide trees",
        "abstract": "A provisional mechanism report that explicitly does not claim performance gains.",
        "claim_links": [
            {
                "claim_id": "claim-1",
                "evidence_ids": [],
                "evidence_status": "unsupported",
            }
        ],
        "baseline_artifact_sha256": sha("a"),
        "methods_artifact_sha256": sha("b"),
        "manuscript_sha256": sha("c"),
        "limitations": ["No independent replication has yet run."],
        "negative_results": [],
        "sanitization_status": "pending",
        "declassification_grant_id": None,
        "publisher_identity": "editor-1",
        "created_utc": "2026-11-21T00:04:00Z",
        "signature_b64": "DEMO_UNVERIFIED_SIGNATURE",
    },
)
emit(
    "repository-adapter",
    {
        "version": {"const": 5},
        "adapter_id": ID,
        "repo_id": ID,
        "origin": {"type": "string", "format": "uri"},
        "base_ref": ID,
        "authorized_owner": ID,
        "visibility": {"enum": ["public", "private", "internal"]},
        "license_policy": {
            "enum": ["public_allowed", "private_only", "review_required"]
        },
        "toolchain": obj(
            {
                "name": ID,
                "version": ID,
                "builder_image_digest": {
                    "type": "string",
                    "pattern": "^sha256:[a-f0-9]{64}$",
                },
            }
        ),
        "source_scope": {"type": "array", "minItems": 1, "items": ID},
        "commands": obj(
            {
                "build": {"type": "array", "minItems": 1, "items": {"type": "string"}},
                "oracle": {"type": "array", "minItems": 1, "items": {"type": "string"}},
                "benchmark": {
                    "type": "array",
                    "minItems": 1,
                    "items": {"type": "string"},
                },
            }
        ),
        "outputs": obj(
            {"artifact_path": ID, "metrics": {"type": "array", "items": ID}}
        ),
        "oracle_id": ID,
        "workloads": {"type": "array", "minItems": 1, "items": ID},
        "default_network": {"enum": ["off", "scoped_reference_only"]},
        "publication_policy": {
            "enum": ["internal_only", "manual_review", "preapproved_public"]
        },
        "adapter_artifact_sha256": H,
    },
    {
        "version": 5,
        "adapter_id": "rust-cli-v1",
        "repo_id": "playform-run",
        "origin": "https://github.com/PlayForm/Run",
        "base_ref": "main",
        "authorized_owner": "local-owner",
        "visibility": "public",
        "license_policy": "review_required",
        "toolchain": {
            "name": "cargo",
            "version": "pinned-via-lockfile",
            "builder_image_digest": "sha256:" + sha("a"),
        },
        "source_scope": ["Source/**"],
        "commands": {
            "build": ["cargo", "build", "--release"],
            "oracle": ["cargo", "test"],
            "benchmark": ["Run", "--help"],
        },
        "outputs": {
            "artifact_path": "Target/release/Run",
            "metrics": ["elapsed_seconds"],
        },
        "oracle_id": "reference-behavior-v1",
        "workloads": ["wide-traversal"],
        "default_network": "off",
        "publication_policy": "manual_review",
        "adapter_artifact_sha256": sha("c"),
    },
)
print("generated", len(list(S.glob("*.json"))), "schemas + examples")
