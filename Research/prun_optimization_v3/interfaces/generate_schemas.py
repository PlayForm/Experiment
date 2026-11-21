#!/usr/bin/env python3
"""Generate documented, draft 2020-12 JSON Schemas and matching illustrative events."""

import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
sha = {"type": "string", "pattern": "^[0-9a-f]{64}$"}
nonempty = {"type": "string", "minLength": 1}
id = nonempty
items = {"type": "array", "items": nonempty, "uniqueItems": True}


def schema(name, required, props):
    return {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": f"https://prun.local/schemas/{name}.v3.json",
        "title": f"PlayForm/Run {name} v3",
        "type": "object",
        "additionalProperties": False,
        "required": required,
        "properties": props,
    }


event = schema(
    "event-envelope",
    [
        "schema_version",
        "event_id",
        "event_kind",
        "aggregate_id",
        "aggregate_seq",
        "occurred_utc",
        "actor",
        "idempotency_key",
        "payload_sha256",
        "correlation_id",
    ],
    {
        "schema_version": {"const": 3},
        "event_id": id,
        "event_kind": id,
        "campaign_id": id,
        "aggregate_id": id,
        "aggregate_seq": {"type": "integer", "minimum": 1},
        "occurred_utc": {"type": "string", "format": "date-time"},
        "actor": {
            "type": "object",
            "additionalProperties": False,
            "required": ["id", "role"],
            "properties": {"id": id, "role": id},
        },
        "idempotency_key": id,
        "payload_sha256": sha,
        "correlation_id": id,
        "causation_event_id": {"type": ["string", "null"]},
        "trace_context": {"type": ["string", "null"]},
        "source_artifacts": {"type": "array", "items": sha},
        "view_policy_id": {"type": ["string", "null"]},
    },
)

proposal = schema(
    "proposal",
    [
        "schema_version",
        "proposal_id",
        "work_order_id",
        "generation_id",
        "parent_source_commit",
        "patch_sha256",
        "source_tree_sha256",
        "mechanism_family",
        "mechanism_hypothesis",
        "changed_paths",
        "build_spec_sha256",
        "role_policy_sha256",
        "novelty_fingerprint_version",
        "semantic_contract_sha256",
    ],
    {
        "schema_version": {"const": 3},
        "proposal_id": id,
        "work_order_id": id,
        "generation_id": id,
        "parent_source_commit": nonempty,
        "patch_sha256": sha,
        "source_tree_sha256": sha,
        "mechanism_family": {
            "enum": [
                "allocation",
                "scheduling",
                "filesystem",
                "matching",
                "process_executor",
                "heuristic_policy",
                "serialization",
                "syscall_overhead",
                "compiler",
                "integration",
                "other",
            ]
        },
        "mechanism_hypothesis": nonempty,
        "changed_paths": items,
        "changed_symbols": items,
        "build_spec_sha256": sha,
        "role_policy_sha256": sha,
        "novelty_fingerprint_version": id,
        "semantic_contract_sha256": sha,
        "risk_notes": {"type": "array", "items": nonempty},
        "expected_workload_effect": {
            "type": "array",
            "items": {
                "type": "object",
                "additionalProperties": False,
                "required": ["workload_family", "prediction"],
                "properties": {
                    "workload_family": id,
                    "prediction": {
                        "enum": ["speedup", "neutral", "regression", "unknown"]
                    },
                },
            },
        },
        "parent_mutation_id": {"type": ["string", "null"]},
        "prompt_context_sha256": sha,
        "agent_session_id": id,
    },
)

h = "a" * 64
b = "b" * 64
c = "c" * 64
d = "d" * 64
samples = {
    "event.v3.json": {
        "schema_version": 3,
        "event_id": "ev-001",
        "event_kind": "PATCH_SUBMITTED",
        "aggregate_id": "mutation-0001",
        "aggregate_seq": 1,
        "occurred_utc": "2026-11-21T01:00:00Z",
        "actor": {"id": "agent-001", "role": "patcher"},
        "idempotency_key": "task-001-submit-1",
        "payload_sha256": h,
        "correlation_id": "generation-001",
        "causation_event_id": None,
        "trace_context": None,
        "source_artifacts": [b],
        "view_policy_id": "worker-scoped-v1",
    },
    "proposal.v3.json": {
        "schema_version": 3,
        "proposal_id": "proposal-001",
        "work_order_id": "task-001",
        "generation_id": "generation-001",
        "parent_source_commit": "BASELINE_COMMIT_PIN_ON_DEPLOYMENT",
        "patch_sha256": h,
        "source_tree_sha256": b,
        "mechanism_family": "scheduling",
        "mechanism_hypothesis": "A bounded queue can reduce allocation and overscheduling overhead on many tiny command jobs while preserving spawn and failure semantics.",
        "changed_paths": ["Source/scheduler.rs"],
        "changed_symbols": ["scheduler::dispatch"],
        "build_spec_sha256": c,
        "role_policy_sha256": d,
        "novelty_fingerprint_version": "canonical-rust-v1",
        "semantic_contract_sha256": c,
        "risk_notes": [
            "May change ordering where output order is observable",
            "May regress when child commands block on I/O",
        ],
        "expected_workload_effect": [
            {"workload_family": "tiny-many-children", "prediction": "speedup"},
            {"workload_family": "single-child", "prediction": "neutral"},
        ],
        "parent_mutation_id": None,
        "prompt_context_sha256": d,
        "agent_session_id": "agent-session-001",
    },
}
for name, content in [
    ("event-envelope.v3.schema.json", event),
    ("proposal.v3.schema.json", proposal),
]:
    (root / "interfaces" / name).write_text(json.dumps(content, indent=2) + "\n")
for name, content in samples.items():
    (root / "examples" / name).write_text(json.dumps(content, indent=2) + "\n")

# V3 overlay references a V2 campaign by frozen hash; this example is a documented
# overlay, not secretly merged into the V2 schema or executable v1 controller.
overlay = {
    "schema_version": 3,
    "extends_campaign_schema": 2,
    "campaign_id": "local-adaptive-scheduler-v1",
    "status": "illustrative_overlay_not_executable",
    "scheduler": {
        "fanout": 5,
        "agent_pool_idle": 2,
        "max_agent_concurrency": 5,
        "max_parallel_measurements_per_bench_host": 1,
        "measurement_isolation": "dedicated_host_preferred",
        "control_cpu_set": "0-1",
        "bench_cpu_set": "2-5",
    },
    "agent_isolation": {
        "patcher_full_repo_access": False,
        "patcher_can_read_holdout": False,
        "patcher_can_read_peers": False,
        "patcher_can_benchmark": False,
        "patcher_requires_committed_patch": True,
        "source_view_policy": "worktree_scope_and_symbol_dependencies",
    },
    "novelty_gate": {
        "identity_version": "canonical-rust-v1",
        "exact_cache": True,
        "syntax_similarity": True,
        "embedding_similarity_optional": True,
        "model_advisory_only": True,
        "reuse_requires_same_protocol": True,
    },
    "telemetry": {
        "control_plane": "on",
        "product_timing_mode": "trace-off",
        "product_diagnostics_mode": "trace-diagnostic",
        "rust_otlp_receiver": True,
        "telemetry_is_not_evidence_ledger": True,
    },
    "results": {
        "full_trials_immutable": True,
        "feedback_only_own_development_results": True,
        "save_prompt_turns": True,
        "save_stream_timing": True,
        "signed_attestations": True,
        "manual_release_approval": True,
    },
}
(root / "examples" / "campaign-overlay.v3.json").write_text(
    json.dumps(overlay, indent=2) + "\n"
)

feedback = schema(
    "feedback-capsule",
    [
        "schema_version",
        "capsule_id",
        "recipient_work_order_id",
        "proposal_id",
        "evaluation_status",
        "dataset_visibility",
        "baseline_binary_sha256",
        "candidate_binary_sha256",
        "evidence_sha256",
        "policy_sha256",
        "issued_utc",
    ],
    {
        "schema_version": {"const": 3},
        "capsule_id": id,
        "recipient_work_order_id": id,
        "proposal_id": id,
        "evaluation_status": {
            "enum": [
                "oracle_failed",
                "benchmark_win",
                "benchmark_loss",
                "inconclusive",
                "invalid",
                "cache_hit",
                "not_yet_measured",
            ]
        },
        "dataset_visibility": {"const": "own_development_only"},
        "baseline_binary_sha256": sha,
        "candidate_binary_sha256": sha,
        "evidence_sha256": sha,
        "policy_sha256": sha,
        "issued_utc": {"type": "string", "format": "date-time"},
        "speedup_ratio": {"type": ["number", "null"], "exclusiveMinimum": 0},
        "ci_low": {"type": ["number", "null"]},
        "ci_high": {"type": ["number", "null"]},
        "failure_categories": {"type": "array", "items": nonempty},
        "aggregate_hints": {"type": "array", "items": nonempty},
        "next_action": {
            "enum": [
                "revise_as_new_mutation",
                "stop_lane",
                "confirm_later",
                "await_review",
                "repair_correctness",
            ]
        },
        "sealed_holdout_included": {"const": False},
        "related_analysis_id": {"type": ["string", "null"]},
    },
)
attestation = schema(
    "attestation",
    [
        "schema_version",
        "attestation_id",
        "stamp_kind",
        "subject_kind",
        "subject_id",
        "evidence_sha256",
        "policy_sha256",
        "issuer",
        "valid_cohort",
        "issued_utc",
        "signature",
    ],
    {
        "schema_version": {"const": 3},
        "attestation_id": id,
        "stamp_kind": {
            "enum": [
                "scope_checked",
                "novelty_checked",
                "oracle_passed",
                "bench_integrity_passed",
                "analysis_confirmed",
                "integration_eligible",
                "release_approved",
            ]
        },
        "subject_kind": id,
        "subject_id": id,
        "evidence_sha256": sha,
        "policy_sha256": sha,
        "issuer": id,
        "valid_cohort": id,
        "issued_utc": {"type": "string", "format": "date-time"},
        "signature": {"type": "string", "minLength": 1},
        "signing_key_id": id,
        "expires_utc": {"type": ["string", "null"], "format": "date-time"},
        "prerequisite_attestations": {
            "type": "array",
            "items": id,
            "uniqueItems": True,
        },
    },
)
turn = schema(
    "agent-turn",
    [
        "schema_version",
        "conversation_id",
        "session_id",
        "turn_id",
        "seq",
        "kind",
        "role_policy_sha256",
        "context_snapshot_sha256",
        "content_sha256",
        "redaction_policy_id",
        "created_utc",
    ],
    {
        "schema_version": {"const": 3},
        "conversation_id": id,
        "session_id": id,
        "turn_id": id,
        "seq": {"type": "integer", "minimum": 0},
        "kind": {
            "enum": [
                "system",
                "developer",
                "user",
                "assistant",
                "tool_call",
                "tool_result",
                "feedback",
                "compaction",
            ]
        },
        "role_policy_sha256": sha,
        "context_snapshot_sha256": sha,
        "content_sha256": sha,
        "redaction_policy_id": id,
        "created_utc": {"type": "string", "format": "date-time"},
        "model_id": {"type": ["string", "null"]},
        "prompt_template_sha256": {"oneOf": [sha, {"type": "null"}]},
        "prior_turns": {"type": "array", "items": id},
        "tools_used": {"type": "array", "items": id},
        "ttft_ms": {"type": ["number", "null"], "minimum": 0},
        "ttfb_ms": {"type": ["number", "null"], "minimum": 0},
        "tokens_in": {"type": ["integer", "null"], "minimum": 0},
        "tokens_out": {"type": ["integer", "null"], "minimum": 0},
        "finish_reason": {"type": ["string", "null"]},
    },
)
new_schemas = {
    "feedback-capsule": feedback,
    "attestation": attestation,
    "agent-turn": turn,
}
for name, content in new_schemas.items():
    (root / "interfaces" / f"{name}.v3.schema.json").write_text(
        json.dumps(content, indent=2) + "\n"
    )
new_samples = {
    "feedback-capsule": {
        "schema_version": 3,
        "capsule_id": "feedback-001",
        "recipient_work_order_id": "task-001",
        "proposal_id": "proposal-001",
        "evaluation_status": "benchmark_loss",
        "dataset_visibility": "own_development_only",
        "baseline_binary_sha256": h,
        "candidate_binary_sha256": b,
        "evidence_sha256": c,
        "policy_sha256": d,
        "issued_utc": "2026-11-21T01:05:00Z",
        "speedup_ratio": 0.97,
        "ci_low": 0.95,
        "ci_high": 0.99,
        "failure_categories": [],
        "aggregate_hints": ["Scheduling overhead increased on small jobs"],
        "next_action": "revise_as_new_mutation",
        "sealed_holdout_included": False,
        "related_analysis_id": "analysis-001",
    },
    "attestation": {
        "schema_version": 3,
        "attestation_id": "att-001",
        "stamp_kind": "oracle_passed",
        "subject_kind": "build",
        "subject_id": "build-001",
        "evidence_sha256": h,
        "policy_sha256": b,
        "issuer": "oracle-service-v1",
        "valid_cohort": "linux-amd64-runner-cohort-A",
        "issued_utc": "2026-11-21T01:03:00Z",
        "signature": "DEMO_NOT_A_REAL_SIGNATURE",
        "signing_key_id": "local-test-key",
        "expires_utc": None,
        "prerequisite_attestations": [],
    },
    "agent-turn": {
        "schema_version": 3,
        "conversation_id": "conversation-001",
        "session_id": "agent-session-001",
        "turn_id": "turn-0007",
        "seq": 7,
        "kind": "feedback",
        "role_policy_sha256": a if False else h,
        "context_snapshot_sha256": b,
        "content_sha256": c,
        "redaction_policy_id": "scoped-feedback-v1",
        "created_utc": "2026-11-21T01:05:00Z",
        "model_id": None,
        "prompt_template_sha256": d,
        "prior_turns": ["turn-0006"],
        "tools_used": [],
        "ttft_ms": None,
        "ttfb_ms": None,
        "tokens_in": None,
        "tokens_out": None,
        "finish_reason": None,
    },
}
for name, data in new_samples.items():
    (root / "examples" / f"{name}.v3.json").write_text(
        json.dumps(data, indent=2) + "\n"
    )
