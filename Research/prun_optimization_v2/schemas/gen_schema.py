import json
from pathlib import Path

T = lambda typ, **kw: dict(type=typ, **kw)
S = lambda **kw: T("string", **kw)
I = lambda minimum=0: T("integer", minimum=minimum)
B = lambda: T("boolean")
A = lambda items, minimum=0: T("array", items=items, minItems=minimum)
O = lambda props, req=(), additional=False: {
    "type": "object",
    "properties": props,
    "required": list(req),
    "additionalProperties": additional,
}
ID = S(pattern=r"^[a-z0-9][a-z0-9._-]*$")
nonempty = S(minLength=1)
PROPS = {
    "project": O(
        {
            "name": nonempty,
            "repository": nonempty,
            "source_ref": nonempty,
            "target_binary": nonempty,
            "compatibility": S(enum=["exact", "conditional", "experimental"]),
        },
        ["name", "repository", "source_ref", "target_binary", "compatibility"],
    ),
    "campaign": O(
        {
            "id": ID,
            "description": S(),
            "mode": S(
                enum=["manual", "human_reviewed_autonomous", "fully_trusted_local"]
            ),
            "duration_hours": I(1),
            "workload_split_seed": I(),
            "objectives": O(
                {
                    "primary": S(enum=["paired_wall_time", "throughput", "cpu_time"]),
                    "weights": T("object", additionalProperties=T("number", minimum=0)),
                    "secondary": A(S()),
                    "min_practical_speedup_fraction": T("number", minimum=0, maximum=1),
                    "maximum_critical_regression_fraction": T(
                        "number", minimum=0, maximum=1
                    ),
                    "confidence_level": T(
                        "number", exclusiveMinimum=0, exclusiveMaximum=1
                    ),
                },
                [
                    "primary",
                    "weights",
                    "min_practical_speedup_fraction",
                    "maximum_critical_regression_fraction",
                    "confidence_level",
                ],
            ),
            "budgets": O(
                {
                    k: I(1)
                    for k in [
                        "max_mutations",
                        "max_builds",
                        "max_trial_blocks",
                        "max_concurrent_coding_agents",
                        "max_concurrent_builds",
                        "max_concurrent_benchmarks_per_host",
                        "max_disk_gb",
                        "min_free_disk_gb",
                        "max_agent_attempts_per_mutation",
                        "max_trial_runtime_seconds",
                    ]
                },
                [
                    "max_mutations",
                    "max_concurrent_benchmarks_per_host",
                    "min_free_disk_gb",
                ],
            ),
            "safety": O(
                {
                    k: B()
                    for k in [
                        "require_human_release_approval",
                        "allow_agent_oracle_edits",
                        "allow_network_in_bench_runner",
                        "allow_privileged_benchmark_runner",
                        "allow_agent_docker_socket",
                        "allow_unapproved_external_commands",
                        "stop_on_host_health_alarm",
                    ]
                },
                [
                    "require_human_release_approval",
                    "allow_agent_oracle_edits",
                    "allow_agent_docker_socket",
                ],
            ),
        },
        ["id", "mode", "duration_hours", "objectives", "budgets", "safety"],
    ),
    "containers": O(
        {
            "builder": nonempty,
            "runner_release": nonempty,
            "runner_diagnostic": nonempty,
            "training_runner": nonempty,
            "pin_image_digests_for_campaign": B(),
            "binary_mount_mode": S(enum=["read_only"]),
            "fixture_mount_mode": S(
                enum=["read_only_by_default", "disposable_writable_copies"]
            ),
        },
        [
            "builder",
            "runner_release",
            "runner_diagnostic",
            "pin_image_digests_for_campaign",
        ],
    ),
    "hardware": O(
        {
            "host_id": ID,
            "class_id": ID,
            "cpu_set": S(),
            "smt_policy": S(enum=["isolate_siblings", "allow_recorded", "unknown"]),
            "numa_policy": S(enum=["local", "interleaved", "auto", "unknown"]),
            "measurement_exclusive": B(),
            "physical_isolation_required_for_promotion": B(),
            "effective_cpu_quota": S(),
            "storage_cohort": ID,
            "hardware_profile_artifact": T(["string", "null"]),
        },
        ["host_id", "class_id", "measurement_exclusive", "storage_cohort"],
    ),
    "builds": A(
        O(
            {
                "id": ID,
                "cargo_profile": S(enum=["release", "dev"]),
                "target_cpu": nonempty,
                "features": A(S()),
                "runtime_strategy": nonempty,
                "rustflags": A(S()),
            },
            ["id", "cargo_profile", "target_cpu", "features", "runtime_strategy"],
        ),
        1,
    ),
    "feature_groups": A(
        O(
            {
                "id": ID,
                "kind": S(
                    enum=[
                        "runtime_strategy",
                        "cargo_capability",
                        "compiler_variant",
                        "measurement",
                    ]
                ),
                "members": A(nonempty, 1),
                "exclusive_active_selection": B(),
            },
            ["id", "kind", "members", "exclusive_active_selection"],
        )
    ),
    "heuristics": O(
        {
            "feature_family": ID,
            "modes": A(
                S(
                    enum=[
                        "off",
                        "static",
                        "fs-bounded",
                        "hardware",
                        "cost-v1",
                        "cost-v2",
                        "shadow",
                    ]
                ),
                1,
            ),
            "default_mode": nonempty,
            "analyzer_time_budget_us": I(),
            "filesystem_sampling_enabled": B(),
            "allow_active_command_probing": B(),
            "confidence_threshold": T("number", minimum=0, maximum=1),
            "fallback_strategy": nonempty,
            "persist_raw_commands": B(),
            "policy_artifact": T(["string", "null"]),
            "explain_plan_opt_in": B(),
        },
        [
            "feature_family",
            "modes",
            "default_mode",
            "analyzer_time_budget_us",
            "fallback_strategy",
        ],
    ),
    "fixtures": A(
        O(
            {
                "id": ID,
                "generator": nonempty,
                "seed": I(),
                "directories": I(1),
                "depth": I(1),
                "files_per_dir": I(),
                "match_density": T("number", minimum=0, maximum=1),
                "writable": B(),
            },
            [
                "id",
                "generator",
                "seed",
                "directories",
                "depth",
                "match_density",
                "writable",
            ],
        ),
        1,
    ),
    "workloads": A(
        O(
            {
                "id": ID,
                "fixture": ID,
                "command": A(nonempty, 1),
                "pattern": nonempty,
                "parallel": B(),
                "split": S(
                    enum=["training", "development", "holdout", "sealed_holdout"]
                ),
                "oracle": ID,
            },
            ["id", "fixture", "command", "pattern", "parallel", "split", "oracle"],
        ),
        1,
    ),
    "benchmark": O(
        {
            "engine": S(enum=["hyperfine"]),
            "engine_version": nonempty,
            "warmups_per_candidate": I(),
            "samples_per_block": I(2),
            "minimum_blocks": I(1),
            "paired_comparison": B(),
            "block_order": S(enum=["abba_with_seeded_reversal", "randomized_blocks"]),
            "periodic_reference_blocks": I(1),
            "profile_during_primary_timing": B(),
            "keep_raw_samples": B(),
            "holdout_visible_to_mutator": B(),
            "invalid_environment_policy": S(
                enum=["quarantine_and_reschedule", "fail_closed"]
            ),
        },
        [
            "engine",
            "engine_version",
            "samples_per_block",
            "paired_comparison",
            "block_order",
            "keep_raw_samples",
        ],
    ),
    "agents": O(
        {
            "provider": nonempty,
            "roles": A(ID, 1),
            "worktree_root": nonempty,
            "playbook": nonempty,
            "require_hypothesis_card": B(),
            "require_single_concept_patch": B(),
            "require_committed_snapshot": B(),
            "auto_merge": B(),
        },
        [
            "provider",
            "roles",
            "worktree_root",
            "require_single_concept_patch",
            "auto_merge",
        ],
    ),
    "storage": O(
        {
            "database": nonempty,
            "artifact_root": nonempty,
            "journal": nonempty,
            "append_only_evidence": B(),
            "store_agent_prompts": B(),
            "prompt_redaction": nonempty,
            "artifact_integrity": S(enum=["sha256"]),
            "schema_migration_mode": S(enum=["explicit"]),
        },
        [
            "database",
            "artifact_root",
            "journal",
            "append_only_evidence",
            "artifact_integrity",
        ],
    ),
}
sc = {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "https://example.invalid/prun-campaign.v2.schema.json",
    "title": "PlayForm/Run Adaptive Performance Campaign v2",
    "description": "Proposed specification only; not consumed by original prun_lab.py v1",
    "type": "object",
    "additionalProperties": False,
    "required": ["schema_version", *PROPS],
    "properties": {"schema_version": {"const": 2}, **PROPS},
}
Path(__file__).parent.joinpath("campaign.v2.schema.json").write_text(
    json.dumps(sc, indent=2) + "\n"
)
