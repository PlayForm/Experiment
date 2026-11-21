//! Every table declared by the v2 -> v3 -> v5 -> v6 -> v6.1 chain.
//!
//! Each constructor returns typed declarations only. The v2 file states the
//! WAL mode, the lineage and evidence tables and their immutability triggers;
//! v3 adds work orders, leases, agent turns and attestations; v5 adds the
//! research economy with seven non-fungible resource dimensions
//! (`sql/prerequisites/v5_migration.sql:118-149`); v6 adds the signed journal
//! and correspondence indexes; v6.1 adds custody, tenancy and study tables
//! while modifying no existing table (`sql/v61_migration.sql:1-2`).

use super::check::Check;
use super::column::Column;
use super::constraint::Constraint;
use super::index::Index;
use super::scalar::Type;
use super::table::Table;
use crate::migration::version::Version;

/// The canonical catalog of typed table declarations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Catalog;

impl Catalog {
    /// A closed enumeration-of-values column.
    fn choices(name: &str, values: &[&str]) -> Column {
        Column::of(name, Type::Text).checked(Check::OneOf {
            column: name.to_owned(),
            values: values.iter().map(|value| (*value).to_owned()).collect(),
        })
    }

    /// A 0-or-1 authorization flag: a typed two-state value in SQL form.
    fn flag(name: &str) -> Column {
        Self::choices(name, &["0", "1"])
    }

    /// A non-negative byte, token or count column.
    fn bytes(name: &str) -> Column {
        Column::of(name, Type::Integer).checked(Check::AtLeast {
            column: name.to_owned(),
            bound: 0,
        })
    }

    /// A strictly positive fencing token column.
    fn fence(name: &str) -> Column {
        Column::of(name, Type::Integer).checked(Check::Positive(name.to_owned()))
    }

    /// A digest column that may be absent.
    fn optional_digest(name: &str) -> Column {
        Column::of(name, Type::Text)
            .optional()
            .checked(Check::NullOrDigest(name.to_owned()))
    }

    /// A column frozen to one declared constant.
    fn fixed(name: &str, value: &str) -> Column {
        Column::of(name, Type::Text).checked(Check::Exactly {
            column: name.to_owned(),
            value: value.to_owned(),
        })
    }

    /// Tables declared by the v2 lineage migration.
    #[must_use]
    pub fn v2() -> Vec<Table> {
        vec![
            Table::of("campaigns", vec![
                Column::identity("id"),
                Column::of("schema_version", Type::Integer).checked(Check::AtLeast {
                    column: "schema_version".to_owned(), bound: 1,
                }),
                Column::utc("created_utc"),
                Column::digest("manifest_sha256"),
                Column::identity("state"),
            ], vec![Constraint::primary(&["id"])]),
            Table::of("artifacts", vec![
                Column::digest("sha256"),
                Column::identity("kind"),
                Self::bytes("byte_size"),
                Column::identity("relative_path"),
                Column::utc("created_utc"),
                Column::of("metadata_json", Type::Text).defaulted("{}"),
            ], vec![Constraint::primary(&["sha256"])]),
            Table::of("hypotheses", vec![
                Column::identity("id"),
                Column::identity("campaign_id"),
                Column::identity("stage"),
                Column::of("text", Type::Text),
                Column::identity("parent_analysis_id").optional(),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("campaign_id", "campaigns", "id"),
            ]),
            Table::of("mutations", vec![
                Column::identity("id"),
                Column::identity("campaign_id"),
                Column::identity("hypothesis_id").optional(),
                Column::identity("parent_mutation_id").optional(),
                Column::identity("kind"),
                Column::identity("changed_stage"),
                Column::identity("source_commit"),
                Column::identity("parent_commit"),
                Column::digest("patch_sha256"),
                Column::identity("author_agent_id").optional(),
                Column::identity("state"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("campaign_id", "campaigns", "id"),
                Constraint::reference("hypothesis_id", "hypotheses", "id"),
                Constraint::reference("parent_mutation_id", "mutations", "id"),
                Constraint::reference("patch_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_mutations_parent", "mutations", &["parent_mutation_id"]),
                Index::of("idx_mutations_campaign", "mutations", &["campaign_id"]),
            ]),
            Table::of("mutation_parents", vec![
                Column::identity("child_mutation_id"),
                Column::identity("parent_mutation_id"),
                Self::choices("purpose", &["causal", "merge"]),
            ], vec![
                Constraint::primary(&["child_mutation_id", "parent_mutation_id"]),
                Constraint::reference("child_mutation_id", "mutations", "id"),
                Constraint::reference("parent_mutation_id", "mutations", "id"),
            ]),
            Table::of("builds", vec![
                Column::identity("id"),
                Column::identity("mutation_id"),
                Column::digest("recipe_sha256"),
                Column::identity("source_commit"),
                Self::optional_digest("binary_sha256"),
                Column::digest("builder_image_digest"),
                Column::identity("target_triple"),
                Column::identity("target_cpu"),
                Column::identity("status"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("mutation_id", "mutations", "id"),
                Constraint::reference("binary_sha256", "artifacts", "sha256"),
            ]),
            Table::of("workloads", vec![
                Column::identity("id"),
                Column::digest("fixture_sha256"),
                Column::digest("command_spec_sha256"),
                Column::identity("oracle_version"),
                Self::choices("split", &["training", "development", "holdout", "sealed_holdout"]),
                Column::of("metadata_json", Type::Text),
            ], vec![Constraint::primary(&["id"])]),
            Table::of("environments", vec![
                Column::identity("id"),
                Column::digest("hardware_fingerprint_sha256"),
                Column::identity("os_kernel"),
                Column::identity("runner_oci_digest"),
                Column::of("cpu_allocation_json", Type::Text),
                Column::digest("container_settings_sha256"),
                Column::of("metadata_json", Type::Text),
            ], vec![Constraint::primary(&["id"])]),
            Table::of("experiments", vec![
                Column::identity("id"),
                Column::identity("campaign_id"),
                Column::identity("build_id"),
                Column::identity("workload_id"),
                Column::identity("environment_id"),
                Column::digest("protocol_sha256"),
                Column::identity("configured_candidate_id"),
                Column::identity("status"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("campaign_id", "campaigns", "id"),
                Constraint::reference("build_id", "builds", "id"),
                Constraint::reference("workload_id", "workloads", "id"),
                Constraint::reference("environment_id", "environments", "id"),
                Constraint::unique(&["build_id", "workload_id", "environment_id", "protocol_sha256"]),
            ]).indexed(vec![
                Index::of("idx_experiments_workload", "experiments", &["workload_id"]),
            ]),
            Table::of("oracle_results", vec![
                Column::identity("id"),
                Column::identity("experiment_id"),
                Column::identity("oracle_version"),
                Self::choices("verdict", &["pass", "fail", "inconclusive"]),
                Self::optional_digest("evidence_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("experiment_id", "experiments", "id"),
                Constraint::reference("evidence_sha256", "artifacts", "sha256"),
            ]),
            Table::of("trials", vec![
                Column::identity("id"),
                Column::identity("experiment_id"),
                Column::identity("reference_experiment_id").optional(),
                Column::of("order_seed", Type::Integer),
                Column::of("paired_block_index", Type::Integer),
                Self::optional_digest("raw_sample_sha256"),
                Column::identity("status"),
                Column::identity("invalid_reason").optional(),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("experiment_id", "experiments", "id"),
                Constraint::reference("reference_experiment_id", "experiments", "id"),
                Constraint::reference("raw_sample_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_trials_experiment", "trials", &["experiment_id"]),
            ]),
            Table::of("analyses", vec![
                Column::identity("id"),
                Column::identity("experiment_id"),
                Column::identity("baseline_experiment_id"),
                Column::identity("oracle_result_id"),
                Column::identity("analysis_method_version"),
                Column::digest("output_sha256"),
                Self::choices("verdict", &["win", "loss", "tie", "inconclusive", "invalid"]),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("experiment_id", "experiments", "id"),
                Constraint::reference("baseline_experiment_id", "experiments", "id"),
                Constraint::reference("oracle_result_id", "oracle_results", "id"),
                Constraint::reference("output_sha256", "artifacts", "sha256"),
            ]),
            Table::of("decisions", vec![
                Column::identity("id"),
                Column::identity("analysis_id"),
                Self::choices("action", &["reject", "retry", "confirm", "eligible",
                    "review_required", "promote", "revert"]),
                Column::of("reason", Type::Text),
                Column::identity("actor"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("analysis_id", "analyses", "id"),
            ]),
            Table::of("events", vec![
                Column::counter("sequence"),
                Column::identity("event_id"),
                Column::identity("campaign_id").optional(),
                Column::identity("entity_kind"),
                Column::identity("entity_id"),
                Column::identity("event_kind"),
                Column::counter("event_schema_version"),
                Column::digest("payload_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["sequence"]),
                Constraint::unique(&["event_id"]),
                Constraint::reference("campaign_id", "campaigns", "id"),
                Constraint::reference("payload_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_events_campaign", "events", &["campaign_id", "sequence"]),
            ]).immutable(),
        ]
    }

    /// Tables declared by the v3 additive migration.
    #[must_use]
    pub fn v3() -> Vec<Table> {
        vec![
            Table::of("generations", vec![
                Column::identity("id"),
                Column::identity("campaign_id"),
                Column::identity("frozen_baseline_build_id"),
                Column::digest("generation_spec_sha256"),
                Column::identity("status"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("campaign_id", "campaigns", "id"),
                Constraint::reference("frozen_baseline_build_id", "builds", "id"),
                Constraint::reference("generation_spec_sha256", "artifacts", "sha256"),
            ]),
            Table::of("work_orders", vec![
                Column::identity("id"),
                Column::identity("campaign_id"),
                Column::identity("generation_id").optional(),
                Column::identity("role"),
                Column::digest("scope_sha256"),
                Column::digest("task_spec_sha256"),
                Column::identity("idempotency_key"),
                Column::identity("state"),
                Column::identity("lease_owner").optional(),
                Column::utc("lease_deadline_utc").optional(),
                Column::counter("attempts").defaulted("0"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::unique(&["idempotency_key"]),
                Constraint::reference("campaign_id", "campaigns", "id"),
                Constraint::reference("generation_id", "generations", "id"),
                Constraint::reference("scope_sha256", "artifacts", "sha256"),
                Constraint::reference("task_spec_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_orders_campaign", "work_orders", &["campaign_id", "generation_id", "state"]),
            ]),
            Table::of("agent_sessions", vec![
                Column::identity("id"),
                Column::identity("work_order_id"),
                Column::digest("role_policy_sha256"),
                Column::digest("prompt_template_sha256"),
                Column::identity("provider_model_id"),
                Column::digest("parameters_sha256"),
                Column::utc("started_utc"),
                Column::utc("ended_utc").optional(),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("work_order_id", "work_orders", "id"),
                Constraint::reference("role_policy_sha256", "artifacts", "sha256"),
                Constraint::reference("prompt_template_sha256", "artifacts", "sha256"),
                Constraint::reference("parameters_sha256", "artifacts", "sha256"),
            ]),
            Table::of("context_snapshots", vec![
                Column::identity("id"),
                Column::identity("agent_session_id"),
                Column::digest("allowed_view_sha256"),
                Column::digest("resolved_context_sha256"),
                Self::optional_digest("compaction_sha256"),
                Column::counter("turn_first"),
                Column::counter("turn_last"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("agent_session_id", "agent_sessions", "id"),
                Constraint::reference("allowed_view_sha256", "artifacts", "sha256"),
                Constraint::reference("resolved_context_sha256", "artifacts", "sha256"),
                Constraint::reference("compaction_sha256", "artifacts", "sha256"),
            ]),
            Table::of("agent_turns", vec![
                Column::identity("id"),
                Column::identity("agent_session_id"),
                Column::identity("context_snapshot_id").optional(),
                Column::counter("turn_seq"),
                Self::choices("kind", &["system", "developer", "user", "tool_call",
                    "tool_result", "assistant", "feedback"]),
                Column::digest("content_sha256"),
                Column::identity("privacy_class"),
                Self::optional_digest("model_metrics_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::unique(&["agent_session_id", "turn_seq"]),
                Constraint::reference("agent_session_id", "agent_sessions", "id"),
                Constraint::reference("context_snapshot_id", "context_snapshots", "id"),
                Constraint::reference("content_sha256", "artifacts", "sha256"),
                Constraint::reference("model_metrics_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_turns_session", "agent_turns", &["agent_session_id", "turn_seq"]),
            ]).immutable(),
            Table::of("proposals", vec![
                Column::identity("id"),
                Column::identity("work_order_id"),
                Column::identity("mutation_id").optional(),
                Column::identity("parent_source_commit"),
                Column::digest("patch_sha256"),
                Column::digest("proposal_sha256"),
                Column::identity("mechanism_key"),
                Self::optional_digest("changed_symbols_sha256"),
                Column::identity("status"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("work_order_id", "work_orders", "id"),
                Constraint::reference("mutation_id", "mutations", "id"),
                Constraint::reference("patch_sha256", "artifacts", "sha256"),
                Constraint::reference("proposal_sha256", "artifacts", "sha256"),
                Constraint::reference("changed_symbols_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_proposals_patch", "proposals", &["patch_sha256", "parent_source_commit"]),
                Index::of("idx_proposals_mechanism", "proposals", &["mechanism_key"]),
            ]),
            Table::of("novelty_verdicts", vec![
                Column::identity("id"),
                Column::identity("proposal_id"),
                Column::identity("gate_version"),
                Column::digest("exact_identity_sha256"),
                Self::optional_digest("similarity_report_sha256"),
                Self::choices("verdict", &["NOVEL", "EXACT_CACHE_HIT", "NEAR_DUPLICATE",
                    "CONFLICT", "INSUFFICIENT_PROOF", "INVALID"]),
                Column::identity("related_proposal_id").optional(),
                Column::digest("reason_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("proposal_id", "proposals", "id"),
                Constraint::reference("related_proposal_id", "proposals", "id"),
                Constraint::reference("similarity_report_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_novelty_proposal", "novelty_verdicts", &["proposal_id", "created_utc"]),
            ]),
            Table::of("feedback_capsules", vec![
                Column::identity("id"),
                Column::identity("work_order_id"),
                Column::identity("proposal_id").optional(),
                Column::identity("source_analysis_id").optional(),
                Column::digest("view_policy_sha256"),
                Column::digest("content_sha256"),
                Self::flag("redaction_passed"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("work_order_id", "work_orders", "id"),
                Constraint::reference("proposal_id", "proposals", "id"),
                Constraint::reference("source_analysis_id", "analyses", "id"),
                Constraint::reference("view_policy_sha256", "artifacts", "sha256"),
                Constraint::reference("content_sha256", "artifacts", "sha256"),
            ]).immutable(),
            Table::of("attestations", vec![
                Column::identity("id"),
                Column::identity("subject_kind"),
                Column::identity("subject_id"),
                Self::choices("stamp_kind", &["scope_checked", "novelty_checked", "oracle_passed",
                    "bench_integrity_passed", "analysis_confirmed", "integration_eligible",
                    "release_approved"]),
                Column::digest("policy_sha256"),
                Column::digest("evidence_sha256"),
                Column::identity("issuer"),
                Column::identity("signature"),
                Column::identity("valid_cohort"),
                Column::utc("created_utc"),
            ], vec![Constraint::primary(&["id"])]).indexed(vec![
                Index::of("idx_attest_subject", "attestations", &["subject_kind", "subject_id", "stamp_kind"]),
            ]).immutable(),
            Table::of("telemetry_references", vec![
                Column::identity("id"),
                Self::choices("domain", &["control", "product", "measurement"]),
                Column::identity("event_id").optional(),
                Column::identity("experiment_id").optional(),
                Column::identity("trial_id").optional(),
                Column::identity("trace_id").optional(),
                Column::identity("span_id").optional(),
                Self::optional_digest("payload_sha256"),
                Column::identity("instrumentation_mode"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("event_id", "events", "event_id"),
                Constraint::reference("experiment_id", "experiments", "id"),
                Constraint::reference("trial_id", "trials", "id"),
                Constraint::reference("payload_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_telemetry_trial", "telemetry_references", &["trial_id", "domain"]),
            ]),
            Table::of("dispatch_outbox", vec![
                Column::identity("id"),
                Column::identity("event_id"),
                Column::identity("work_order_id"),
                Column::identity("operation"),
                Column::digest("payload_sha256"),
                Column::utc("accepted_utc"),
                Column::utc("delivered_utc").optional(),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::unique(&["event_id"]),
                Constraint::reference("event_id", "events", "event_id"),
                Constraint::reference("work_order_id", "work_orders", "id"),
                Constraint::reference("payload_sha256", "artifacts", "sha256"),
            ]),
        ]
    }

    /// Tables declared by the v5 research-economy migration.
    #[must_use]
    pub fn v5() -> Vec<Table> {
        vec![
            Table::of("research_cases", vec![
                Column::identity("id"),
                Column::identity("campaign_id"),
                Column::identity("parent_mutation_id").optional(),
                Column::identity("repository_id"),
                Column::digest("question_sha256"),
                Column::digest("scope_sha256"),
                Column::identity("workload_id").optional(),
                Self::choices("privacy_class", &["private", "tenant", "public"]),
                Self::optional_digest("preregistration_sha256"),
                Column::identity("status"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("campaign_id", "campaigns", "id"),
                Constraint::reference("parent_mutation_id", "mutations", "id"),
                Constraint::reference("question_sha256", "artifacts", "sha256"),
                Constraint::reference("scope_sha256", "artifacts", "sha256"),
                Constraint::reference("workload_id", "workloads", "id"),
                Constraint::reference("preregistration_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_case_repo", "research_cases", &["repository_id", "campaign_id"]),
            ]),
            Table::of("researcher_assignments", vec![
                Column::identity("id"),
                Column::identity("case_id"),
                Column::identity("work_order_id").optional(),
                Column::identity("actor_identity"),
                Self::choices("role", &["proponent", "skeptic", "alternative", "replicator",
                    "methodologist", "librarian", "editor", "integrator", "market_analyst"]),
                Column::identity("model_cohort"),
                Column::digest("privacy_scope_sha256"),
                Self::flag("blinded_phase"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("case_id", "research_cases", "id"),
                Constraint::reference("work_order_id", "work_orders", "id"),
                Constraint::reference("privacy_scope_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_assignment_case", "researcher_assignments", &["case_id", "role"]),
            ]),
            Table::of("research_bids", vec![
                Column::identity("id"),
                Column::identity("assignment_id"),
                Column::identity("case_id"),
                Column::digest("bid_payload_sha256"),
                Column::digest("requested_resource_sha256"),
                Column::digest("prediction_sha256"),
                Column::digest("signature_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::unique(&["assignment_id", "bid_payload_sha256"]),
                Constraint::reference("assignment_id", "researcher_assignments", "id"),
                Constraint::reference("case_id", "research_cases", "id"),
                Constraint::reference("bid_payload_sha256", "artifacts", "sha256"),
                Constraint::reference("requested_resource_sha256", "artifacts", "sha256"),
                Constraint::reference("prediction_sha256", "artifacts", "sha256"),
                Constraint::reference("signature_sha256", "artifacts", "sha256"),
            ]).immutable(),
            Table::of("research_claims", vec![
                Column::identity("id"),
                Column::identity("case_id"),
                Column::identity("assignment_id"),
                Self::choices("claim_type", &["opinion", "hypothesis", "external_reference",
                    "observation", "interpretation", "replication"]),
                Column::digest("claim_sha256"),
                Column::digest("context_sha256"),
                Column::digest("signature_sha256"),
                Column::identity("supersedes_claim_id").optional(),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("case_id", "research_cases", "id"),
                Constraint::reference("assignment_id", "researcher_assignments", "id"),
                Constraint::reference("supersedes_claim_id", "research_claims", "id"),
                Constraint::reference("claim_sha256", "artifacts", "sha256"),
                Constraint::reference("context_sha256", "artifacts", "sha256"),
                Constraint::reference("signature_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_claim_case", "research_claims", &["case_id", "claim_type"]),
            ]).immutable(),
            Table::of("claim_links", vec![
                Column::identity("from_claim_id"),
                Column::identity("to_claim_id"),
                Self::choices("relation", &["supports", "contradicts", "refines", "replicates", "cites"]),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["from_claim_id", "to_claim_id", "relation"]),
                Constraint::reference("from_claim_id", "research_claims", "id"),
                Constraint::reference("to_claim_id", "research_claims", "id"),
            ]),
            Table::of("research_references", vec![
                Column::identity("id"),
                Column::identity("claim_id"),
                Column::identity("uri"),
                Column::utc("retrieved_utc"),
                Column::digest("source_snapshot_sha256"),
                Column::digest("citation_scope_sha256"),
                Column::identity("license_status"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("claim_id", "research_claims", "id"),
                Constraint::reference("source_snapshot_sha256", "artifacts", "sha256"),
                Constraint::reference("citation_scope_sha256", "artifacts", "sha256"),
            ]),
            Table::of("resource_treasuries", vec![
                Column::identity("id"),
                Column::identity("campaign_id"),
                Column::digest("policy_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("campaign_id", "campaigns", "id"),
                Constraint::reference("policy_sha256", "artifacts", "sha256"),
            ]),
            Table::of("resource_transactions", vec![
                Column::counter("seq"),
                Column::identity("transaction_id"),
                Column::identity("treasury_id"),
                Column::identity("assignment_id").optional(),
                Column::identity("bid_id").optional(),
                Column::identity("request_id"),
                Self::choices("kind", &["fund", "reserve", "consume", "release", "expire", "adjust"]),
                Self::bytes("input_tokens").defaulted("0"),
                Self::bytes("output_tokens").defaulted("0"),
                Self::bytes("cpu_core_seconds").defaulted("0"),
                Self::bytes("benchmark_seconds").defaulted("0"),
                Self::bytes("gpu_seconds").defaulted("0"),
                Self::bytes("storage_byte_days").defaulted("0"),
                Self::bytes("reference_requests").defaulted("0"),
                Column::digest("policy_sha256"),
                Column::digest("evidence_sha256"),
                Column::identity("issuer_identity"),
                Column::digest("signature_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["seq"]),
                Constraint::unique(&["transaction_id"]),
                Constraint::unique(&["request_id"]),
                Constraint::reference("treasury_id", "resource_treasuries", "id"),
                Constraint::reference("assignment_id", "researcher_assignments", "id"),
                Constraint::reference("bid_id", "research_bids", "id"),
                Constraint::reference("policy_sha256", "artifacts", "sha256"),
                Constraint::reference("evidence_sha256", "artifacts", "sha256"),
                Constraint::reference("signature_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_treasury_transactions", "resource_transactions", &["treasury_id", "seq"]),
            ]).immutable(),
            Table::of("resource_reservations", vec![
                Column::identity("id"),
                Column::identity("treasury_id"),
                Column::identity("bid_id"),
                Column::identity("initial_transaction_id"),
                Self::choices("current_state", &["reserved", "partially_consumed", "settled",
                    "released", "expired"]),
                Self::fence("fencing_token").defaulted("1"),
                Column::utc("expires_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::unique(&["initial_transaction_id"]),
                Constraint::reference("treasury_id", "resource_treasuries", "id"),
                Constraint::reference("bid_id", "research_bids", "id"),
                Constraint::reference("initial_transaction_id", "resource_transactions", "transaction_id"),
            ]),
            Table::of("paper_manuscripts", vec![
                Column::identity("id"),
                Column::identity("repository_id"),
                Column::digest("manuscript_sha256"),
                Column::digest("methods_sha256"),
                Column::digest("source_snapshot_sha256"),
                Self::choices("status", &["hypothesis_note", "internal_observation",
                    "measured_lab_report", "replicated_report", "sanitized_preprint",
                    "externally_reviewed", "corrected", "retracted"]),
                Column::identity("supersedes_id").optional(),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("supersedes_id", "paper_manuscripts", "id"),
                Constraint::reference("manuscript_sha256", "artifacts", "sha256"),
                Constraint::reference("methods_sha256", "artifacts", "sha256"),
                Constraint::reference("source_snapshot_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_papers_repo", "paper_manuscripts", &["repository_id", "status"]),
            ]).immutable(),
            Table::of("paper_claims", vec![
                Column::identity("manuscript_id"),
                Column::identity("claim_id"),
                Self::optional_digest("evidence_sha256"),
                Column::identity("strength"),
            ], vec![
                Constraint::primary(&["manuscript_id", "claim_id"]),
                Constraint::reference("manuscript_id", "paper_manuscripts", "id"),
                Constraint::reference("claim_id", "research_claims", "id"),
                Constraint::reference("evidence_sha256", "artifacts", "sha256"),
            ]),
            Table::of("paper_reviews", vec![
                Column::identity("id"),
                Column::identity("manuscript_id"),
                Column::identity("reviewer_assignment_id"),
                Self::choices("review_kind", &["methodology", "replication", "citations",
                    "sanitization", "release_policy"]),
                Self::choices("outcome", &["pass", "fail", "revise", "inconclusive"]),
                Column::digest("review_sha256"),
                Column::digest("signature_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::unique(&["manuscript_id", "reviewer_assignment_id", "review_kind"]),
                Constraint::reference("manuscript_id", "paper_manuscripts", "id"),
                Constraint::reference("reviewer_assignment_id", "researcher_assignments", "id"),
                Constraint::reference("review_sha256", "artifacts", "sha256"),
                Constraint::reference("signature_sha256", "artifacts", "sha256"),
            ]).immutable(),
            Table::of("publication_releases", vec![
                Column::identity("id"),
                Column::identity("manuscript_id"),
                Self::choices("visibility", &["tenant", "public"]),
                Column::digest("declassification_grant_sha256"),
                Column::digest("release_manifest_sha256"),
                Column::identity("issuer_identity"),
                Column::digest("signature_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::reference("manuscript_id", "paper_manuscripts", "id"),
                Constraint::reference("declassification_grant_sha256", "artifacts", "sha256"),
                Constraint::reference("release_manifest_sha256", "artifacts", "sha256"),
                Constraint::reference("signature_sha256", "artifacts", "sha256"),
            ]).indexed(vec![
                Index::of("idx_release_paper", "publication_releases", &["manuscript_id", "visibility"]),
            ]).immutable(),
            Table::of("integration_cases", vec![
                Column::identity("id"),
                Column::identity("source_release_id"),
                Column::identity("destination_repo_id"),
                Column::identity("destination_research_case_id"),
                Column::identity("integrator_assignment_id"),
                Column::digest("proposed_transfer_sha256"),
                Column::utc("created_utc"),
            ], vec![
                Constraint::primary(&["id"]),
                Constraint::unique(&["source_release_id", "destination_research_case_id"]),
                Constraint::reference("source_release_id", "publication_releases", "id"),
                Constraint::reference("destination_research_case_id", "research_cases", "id"),
                Constraint::reference("integrator_assignment_id", "researcher_assignments", "id"),
                Constraint::reference("proposed_transfer_sha256", "artifacts", "sha256"),
            ]),
        ]
    }

    /// Tables declared by the v6 protocol and correspondence migration.
    #[must_use]
    pub fn v6() -> Vec<Table> {
        vec![
            Table::of("v6_event_journal", vec![
                Column::counter("global_seq"),
                Column::identity("event_id"),
                Column::identity("event_kind"),
                Column::identity("aggregate_id"),
                Column::of("aggregate_seq", Type::Integer).checked(Check::Positive("aggregate_seq".to_owned())),
                Column::utc("recorded_utc"),
                Column::identity("actor_id"),
                Column::identity("credential_id"),
                Column::identity("correlation_id"),
                Column::identity("causation_event_id").optional(),
                Column::identity("idempotency_key"),
                Column::digest("payload_sha256"),
                Column::digest("previous_hash"),
                Column::digest("event_hash"),
                Column::digest("signature_artifact_sha256"),
                Self::choices("privacy_class", &["public-meta", "internal-meta", "restricted", "sealed"]),
            ], vec![
                Constraint::primary(&["global_seq"]),
                Constraint::unique(&["event_id"]),
                Constraint::unique(&["idempotency_key"]),
                Constraint::unique(&["event_hash"]),
                Constraint::unique(&["aggregate_id", "aggregate_seq"]),
            ]).indexed(vec![
                Index::of("v6_event_correlation_idx", "v6_event_journal", &["correlation_id", "global_seq"]),
                Index::of("v6_event_kind_idx", "v6_event_journal", &["event_kind", "global_seq"]),
            ]).immutable(),
            Table::of("v6_command_inbox", vec![
                Column::identity("command_id"),
                Column::identity("idempotency_key"),
                Column::identity("operator_identity"),
                Column::identity("command_kind"),
                Column::identity("target_id"),
                Column::counter("expected_revision"),
                Column::digest("payload_sha256"),
                Self::choices("state", &["received", "validated", "rejected", "committed"]),
                Column::identity("receipt_event_id").optional(),
                Column::utc("submitted_utc"),
            ], vec![
                Constraint::primary(&["command_id"]),
                Constraint::unique(&["idempotency_key"]),
                Constraint::reference("receipt_event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v6_projection_outbox", vec![
                Column::identity("event_id"),
                Column::counter("delivery_seq").defaulted("0"),
                Self::choices("delivery_state", &["pending", "delivered", "quarantined"])
                    .defaulted("pending"),
                Column::utc("last_delivery_utc").optional(),
            ], vec![
                Constraint::primary(&["event_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v6_record_index", vec![
                Column::identity("record_kind"),
                Column::identity("record_id"),
                Column::of("schema_version", Type::Integer).checked(Check::Exactly {
                    column: "schema_version".to_owned(), value: "6".to_owned(),
                }),
                Column::digest("json_sha256"),
                Column::identity("privacy_class"),
                Column::identity("subject_id"),
                Column::identity("event_id"),
            ], vec![
                Constraint::primary(&["record_kind", "record_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v6_artifact_grants", vec![
                Column::identity("grant_id"),
                Column::digest("artifact_sha256"),
                Column::identity("recipient_identity"),
                Column::identity("purpose"),
                Column::digest("grant_policy_sha256"),
                Column::utc("expires_utc"),
                Column::identity("revoked_event_id").optional(),
            ], vec![
                Constraint::primary(&["grant_id"]),
                Constraint::unique(&["artifact_sha256", "recipient_identity", "purpose"]),
                Constraint::reference("revoked_event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v6_correspondence", vec![
                Column::identity("turn_id"),
                Column::identity("conversation_id"),
                Column::identity("task_id"),
                Column::identity("segment_id"),
                Column::counter("turn_seq"),
                Column::digest("model_cohort_digest"),
                Column::digest("context_digest"),
                Column::digest("ciphertext_digest"),
                Column::identity("recipient_key_id"),
                Self::choices("privacy_class", &["restricted", "sealed"]),
                Column::identity("committed_event_id"),
            ], vec![
                Constraint::primary(&["turn_id"]),
                Constraint::unique(&["conversation_id", "turn_seq"]),
                Constraint::reference("committed_event_id", "v6_event_journal", "event_id"),
            ]).indexed(vec![
                Index::of("v6_correspondence_task", "v6_correspondence", &["task_id", "turn_seq"]),
            ]).immutable(),
            Table::of("v6_tool_calls", vec![
                Column::identity("tool_call_id"),
                Column::identity("turn_id"),
                Column::identity("allowed_tool_name"),
                Column::digest("arguments_ciphertext_digest"),
                Column::digest("result_ciphertext_digest"),
                Column::digest("policy_check_digest"),
                Column::identity("receipt_event_id"),
            ], vec![
                Constraint::primary(&["tool_call_id"]),
                Constraint::reference("turn_id", "v6_correspondence", "turn_id"),
                Constraint::reference("receipt_event_id", "v6_event_journal", "event_id"),
            ]).immutable(),
            Table::of("v6_task_mailboxes", vec![
                Column::identity("assignment_id"),
                Column::identity("task_id"),
                Column::identity("supervisor_id"),
                Column::digest("bundle_digest"),
                Column::identity("task_nonce"),
                Column::identity("lease_id"),
                Self::fence("fencing_token"),
                Self::choices("state", &["staged", "mounted", "running", "sealed",
                    "expired", "quarantined"]),
                Self::optional_digest("output_digest"),
            ], vec![
                Constraint::primary(&["assignment_id"]),
                Constraint::unique(&["task_id"]),
                Constraint::unique(&["task_nonce"]),
            ]),
            Table::of("v6_service_credentials", vec![
                Column::identity("serial"),
                Column::identity("subject_id"),
                Column::identity("issuer_id"),
                Column::identity("kind"),
                Column::digest("public_key_fingerprint"),
                Column::digest("scope_digest"),
                Column::utc("expires_utc"),
                Self::choices("status", &["active", "revoked", "expired"]),
            ], vec![Constraint::primary(&["serial"])]),
            Table::of("v6_credential_events", vec![
                Column::counter("seq"),
                Column::identity("serial"),
                Self::choices("action", &["issued", "renewed", "revoked", "expired", "denied"]),
                Column::identity("event_id"),
                Column::utc("occurred_utc"),
                Column::identity("reason_code"),
            ], vec![
                Constraint::primary(&["seq"]),
                Constraint::unique(&["event_id"]),
                Constraint::reference("serial", "v6_service_credentials", "serial"),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v6_benchmark_leases", vec![
                Column::identity("lease_id"),
                Column::identity("host_id"),
                Column::identity("experiment_id"),
                Column::identity("cpuset"),
                Self::fence("fencing_token"),
                Self::choices("state", &["held", "released", "revoked", "expired"]),
                Column::utc("acquired_utc"),
                Column::utc("expires_utc"),
                Column::digest("policy_digest"),
            ], vec![Constraint::primary(&["lease_id"])]),
            Table::of("v6_telemetry_refs", vec![
                Column::identity("telemetry_id"),
                Self::choices("plane", &["control", "product"]),
                Column::identity("experiment_id").optional(),
                Column::identity("task_id").optional(),
                Column::identity("event_id").optional(),
                Column::identity("trace_id").optional(),
                Column::identity("span_id").optional(),
                Column::digest("archive_sha256"),
                Column::identity("capture_mode"),
                Column::utc("recorded_utc"),
            ], vec![
                Constraint::primary(&["telemetry_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]).indexed(vec![
                Index::of("v6_telemetry_lookup", "v6_telemetry_refs", &["plane", "experiment_id", "recorded_utc"]),
            ]),
            Table::of("v6_confirmation_access", vec![
                Column::identity("access_id"),
                Column::identity("subject_id"),
                Column::identity("partition_id"),
                Column::digest("capability_digest"),
                Column::identity("requester_identity"),
                Self::flag("authorized"),
                Column::identity("reason_code"),
                Column::identity("event_id"),
            ], vec![
                Constraint::primary(&["access_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v6_service_outbox", vec![
                Column::identity("delivery_id"),
                Column::identity("event_id"),
                Column::identity("subscriber_identity"),
                Self::flag("delivered").defaulted("0"),
                Column::counter("attempts").defaulted("0"),
                Column::utc("last_try_utc").optional(),
            ], vec![
                Constraint::primary(&["delivery_id"]),
                Constraint::unique(&["event_id", "subscriber_identity"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
        ]
    }

    /// Tables declared by the v6.1 additive extension; no existing table is
    /// altered (`sql/v61_migration.sql:1-2`).
    #[must_use]
    pub fn v61() -> Vec<Table> {
        vec![
            Table::of("v61_repository_admissions", vec![
                Column::identity("admission_id"),
                Column::identity("repository_id"),
                Column::identity("tenant_id"),
                Column::digest("commit_digest"),
                Column::digest("policy_digest"),
                Column::digest("license_review_digest"),
                Self::choices("admission_status", &["pending", "admitted", "quarantined", "rejected"]),
                Column::digest("record_digest"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["admission_id"]),
                Constraint::unique(&["repository_id", "tenant_id", "commit_digest", "policy_digest"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_artifact_policies", vec![
                Column::identity("policy_id"),
                Column::digest("artifact_digest"),
                Column::identity("repository_id"),
                Column::identity("tenant_id"),
                Self::choices("classification", &["public", "internal", "restricted", "sealed"]),
                Column::identity("license_grant_id"),
                Column::digest("embargo_set_digest"),
                Column::digest("permitted_purposes_digest"),
                Column::digest("policy_digest"),
                Column::utc("effective_at"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["policy_id"]),
                Constraint::unique(&["policy_digest"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]).indexed(vec![
                Index::of("v61_artifact_policy_lookup", "v61_artifact_policies", &["artifact_digest", "tenant_id"]),
            ]).immutable(),
            Table::of("v61_holder_attestations", vec![
                Column::identity("attestation_id"),
                Column::identity("holder_id"),
                Column::identity("holder_role"),
                Column::identity("tenant_id"),
                Column::digest("environment_digest"),
                Self::bytes("capacity_bytes"),
                Column::of("available_bytes", Type::Integer).checked(Check::All(vec![
                    Check::AtLeast { column: "available_bytes".to_owned(), bound: 0 },
                    Check::AtMostColumn { column: "available_bytes".to_owned(), bound: "capacity_bytes".to_owned() },
                ])),
                Self::flag("encryption_required"),
                Column::identity("network_mode"),
                Column::digest("capability_digest"),
                Column::utc("expires_at"),
                Column::identity("recorded_event_id").optional(),
            ], vec![
                Constraint::primary(&["attestation_id"]),
                Constraint::reference("recorded_event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_license_grants", vec![
                Column::identity("grant_id"),
                Column::identity("repository_id"),
                Column::identity("spdx_expression"),
                Column::digest("license_text_digest"),
                Column::digest("eula_digest"),
                Self::choices("status", &["unreviewed", "compatible", "restricted",
                    "requires-review", "prohibited"]),
                Self::flag("model_training_allowed"),
                Self::flag("redistribution_allowed"),
                Column::digest("record_digest"),
                Column::identity("reviewed_by"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["grant_id"]),
                Constraint::unique(&["record_digest"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_embargo_rules", vec![
                Column::identity("embargo_id"),
                Column::identity("repository_id"),
                Column::digest("subject_digest"),
                Column::digest("target_set_digest"),
                Column::utc("release_after"),
                Column::digest("owner_signature_digest"),
                Column::digest("record_digest"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["embargo_id"]),
                Constraint::unique(&["record_digest"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_trade_restrictions", vec![
                Column::identity("restriction_id"),
                Column::identity("source_group"),
                Column::identity("target_group"),
                Self::choices("direction", &["a-to-b", "b-to-a", "both"]),
                Column::identity("kind"),
                Column::utc("effective_at"),
                Column::utc("expires_at"),
                Column::digest("policy_digest"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["restriction_id"]),
                Constraint::check(Check::NotEqual {
                    left: "source_group".to_owned(), right: "target_group".to_owned(),
                }),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]).indexed(vec![
                Index::of("v61_trade_lookup", "v61_trade_restrictions", &["source_group", "target_group", "effective_at"]),
            ]),
            Table::of("v61_custody_requests", vec![
                Column::identity("request_id"),
                Column::identity("idempotency_key"),
                Column::digest("artifact_digest"),
                Column::identity("source_holder"),
                Column::identity("target_holder"),
                Column::identity("purpose"),
                Column::digest("policy_digest"),
                Self::bytes("requested_bytes"),
                Column::identity("nonce"),
                Column::identity("submitted_event_id").optional(),
            ], vec![
                Constraint::primary(&["request_id"]),
                Constraint::unique(&["idempotency_key"]),
                Constraint::unique(&["nonce"]),
                Constraint::check(Check::NotEqual {
                    left: "source_holder".to_owned(), right: "target_holder".to_owned(),
                }),
                Constraint::reference("submitted_event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_custody_decisions", vec![
                Column::identity("decision_id"),
                Column::identity("request_id"),
                Self::choices("decision", &["allow", "deny", "defer", "quarantine"]),
                Column::identity("internal_reason_code"),
                Column::digest("policy_digest"),
                Column::digest("holder_attestation_digest"),
                Column::utc("issued_at"),
                Column::utc("expires_at"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["decision_id"]),
                Constraint::unique(&["request_id"]),
                Constraint::reference("request_id", "v61_custody_requests", "request_id"),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_storage_leases", vec![
                Column::identity("lease_id"),
                Column::identity("holder_id"),
                Column::digest("artifact_digest"),
                Self::bytes("byte_reservation"),
                Self::bytes("byte_actual").defaulted("0"),
                Column::identity("storage_class"),
                Self::choices("state", &["reserved", "active", "released", "quarantined"]),
                Self::fence("fence"),
                Column::utc("expires_at"),
                Column::identity("allocation_event_id").optional(),
            ], vec![
                Constraint::primary(&["lease_id"]),
                Constraint::unique(&["holder_id", "artifact_digest", "fence"]),
                Constraint::reference("allocation_event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_custody_attempts", vec![
                Column::identity("attempt_id"),
                Column::identity("request_id"),
                Column::identity("decision_id"),
                Column::identity("lease_id"),
                Self::choices("state", &["reserved", "staged", "verified", "committed",
                    "quarantined", "expired"]),
                Self::fence("fence"),
                Self::optional_digest("stage_digest"),
                Column::counter("expected_revision"),
            ], vec![
                Constraint::primary(&["attempt_id"]),
                Constraint::unique(&["request_id", "decision_id", "lease_id"]),
                Constraint::reference("request_id", "v61_custody_requests", "request_id"),
                Constraint::reference("decision_id", "v61_custody_decisions", "decision_id"),
                Constraint::reference("lease_id", "v61_storage_leases", "lease_id"),
            ]),
            Table::of("v61_custody_receipts", vec![
                Column::identity("receipt_id"),
                Column::identity("request_id"),
                Column::identity("attempt_id"),
                Column::digest("artifact_digest"),
                Column::digest("ciphertext_digest"),
                Column::digest("verified_digest"),
                Column::identity("target_holder"),
                Column::digest("holder_attestation_digest"),
                Column::utc("committed_at"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["receipt_id"]),
                Constraint::unique(&["attempt_id"]),
                Constraint::unique(&["request_id", "artifact_digest", "target_holder"]),
                Constraint::reference("request_id", "v61_custody_requests", "request_id"),
                Constraint::reference("attempt_id", "v61_custody_attempts", "attempt_id"),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]).immutable(),
            Table::of("v61_holdings", vec![
                Column::identity("holder_id"),
                Column::digest("artifact_digest"),
                Column::identity("receipt_id"),
                Column::digest("use_rights_digest"),
                Column::identity("key_scope"),
                Column::utc("acquired_at"),
                Column::utc("expires_at"),
                Column::utc("released_at").optional(),
            ], vec![
                Constraint::primary(&["holder_id", "artifact_digest", "receipt_id"]),
                Constraint::reference("receipt_id", "v61_custody_receipts", "receipt_id"),
            ]),
            Table::of("v61_research_tracks", vec![
                Column::identity("track_id"),
                Column::identity("repository_id"),
                Column::identity("role"),
                Column::identity("view_id"),
                Column::identity("trade_group"),
                Column::digest("model_lineage_digest"),
                Column::digest("assignment_policy_digest"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["track_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_research_views", vec![
                Column::identity("view_id"),
                Column::identity("worker_assignment_id"),
                Column::identity("repository_id"),
                Column::digest("source_snapshot_digest"),
                Column::digest("view_digest"),
                Column::digest("policy_digest"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["view_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_task_context_plans", vec![
                Column::identity("plan_id"),
                Column::identity("view_id"),
                Column::digest("mount_digest"),
                Column::digest("tools_digest"),
                Self::bytes("scratch_bytes"),
                Self::fixed("network_mode", "none"),
                Column::digest("supervisor_attestation"),
            ], vec![
                Constraint::primary(&["plan_id"]),
                Constraint::reference("view_id", "v61_research_views", "view_id"),
            ]),
            Table::of("v61_experiment_intents", vec![
                Column::identity("intent_id"),
                Column::identity("worker_assignment_id"),
                Column::digest("proposal_digest"),
                Column::digest("source_digest"),
                Column::digest("worker_claim_digest"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["intent_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_experiment_specs", vec![
                Column::identity("spec_id"),
                Column::identity("intent_id"),
                Column::digest("reference_digest"),
                Column::digest("candidate_digest"),
                Column::digest("oracle_policy_digest"),
                Column::digest("protocol_digest"),
                Self::fixed("compiled_by", "trusted-policy-engine"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["spec_id"]),
                Constraint::unique(&["intent_id", "candidate_digest", "protocol_digest"]),
                Constraint::reference("intent_id", "v61_experiment_intents", "intent_id"),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_candidate_batches", vec![
                Column::identity("batch_id"),
                Column::identity("repository_id"),
                Column::digest("baseline_digest"),
                Column::digest("protocol_digest"),
                Column::digest("variant_set_digest"),
                Column::digest("signed_campaign_policy_digest"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["batch_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_feedback_records", vec![
                Column::identity("feedback_id"),
                Column::identity("track_id"),
                Column::identity("spec_id"),
                Column::digest("source_evidence_digest"),
                Column::digest("filtered_projection_digest"),
                Column::digest("redaction_policy_digest"),
                Column::utc("released_at"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["feedback_id"]),
                Constraint::reference("track_id", "v61_research_tracks", "track_id"),
                Constraint::reference("spec_id", "v61_experiment_specs", "spec_id"),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]).immutable(),
            Table::of("v61_knowledge_releases", vec![
                Column::identity("release_id"),
                Column::identity("origin_repository"),
                Column::identity("target_repository"),
                Column::digest("source_evidence_set_digest"),
                Column::digest("sanitized_document_digest"),
                Column::digest("licensing_decision_digest"),
                Self::choices("state", &["pending", "authorized", "denied", "published", "revoked"]),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["release_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_transfer_compatibility", vec![
                Column::identity("compatibility_id"),
                Column::identity("release_id"),
                Column::identity("origin_repository"),
                Column::identity("target_repository"),
                Self::choices("verdict", &["allow", "deny", "needs-human-review"]),
                Column::identity("target_experiment_id").optional(),
                Column::utc("evaluated_at"),
                Column::digest("policy_digest"),
            ], vec![
                Constraint::primary(&["compatibility_id"]),
                Constraint::reference("release_id", "v61_knowledge_releases", "release_id"),
            ]),
            Table::of("v61_denial_audit", vec![
                Column::identity("audit_id"),
                Column::identity("request_id"),
                Column::identity("actor_id"),
                Column::identity("action"),
                Column::identity("reason_code"),
                Column::digest("artifact_digest"),
                Column::digest("policy_digest"),
                Self::fixed("worker_notified", "0"),
                Column::utc("recorded_at"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["audit_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]).immutable(),
            Table::of("v61_temporal_facts", vec![
                Column::identity("fact_id"),
                Column::identity("subject_id"),
                Column::utc("event_time"),
                Column::utc("observed_at"),
                Column::utc("valid_from"),
                Column::utc("valid_to").optional(),
                Column::utc("recorded_at"),
                Column::identity("supersedes_fact_id").optional(),
                Column::digest("fact_digest"),
                Column::identity("source_event_id").optional(),
            ], vec![
                Constraint::primary(&["fact_id"]),
                Constraint::reference("supersedes_fact_id", "v61_temporal_facts", "fact_id"),
                Constraint::reference("source_event_id", "v6_event_journal", "event_id"),
            ]).immutable(),
            Table::of("v61_neutral_receipts", vec![
                Column::identity("receipt_id"),
                Column::identity("task_local_nonce"),
                Column::digest("receipt_digest"),
                Self::fixed("response_class", "submission_received"),
                Column::utc("issued_at"),
            ], vec![
                Constraint::primary(&["receipt_id"]),
                Constraint::unique(&["task_local_nonce"]),
            ]),
            Table::of("v61_boundary_observations", vec![
                Column::identity("observation_id"),
                Column::identity("worker_assignment_id"),
                Column::identity("local_action"),
                Column::identity("trusted_internal_cause"),
                Column::identity("worker_feedback_class"),
                Column::digest("policy_digest"),
                Column::utc("logged_at"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["observation_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_curricula", vec![
                Column::identity("curriculum_id"),
                Column::identity("repository_id"),
                Column::digest("baseline_digest"),
                Column::digest("curriculum_policy_digest"),
                Column::digest("evaluation_policy_digest"),
                Column::digest("owner_signature_digest"),
                Column::utc("active_at"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["curriculum_id"]),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_study_assignments", vec![
                Column::identity("study_assignment_id"),
                Column::identity("curriculum_id"),
                Column::identity("research_track_id"),
                Column::identity("view_id"),
                Column::identity("challenge_family"),
                Column::digest("task_digest"),
                Column::identity("allocation_id"),
                Column::digest("assignment_policy_digest"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["study_assignment_id"]),
                Constraint::reference("curriculum_id", "v61_curricula", "curriculum_id"),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]),
            Table::of("v61_study_outcomes", vec![
                Column::identity("study_outcome_id"),
                Column::identity("study_assignment_id"),
                Column::digest("proposal_digest"),
                Column::digest("oracle_result_digest"),
                Self::optional_digest("benchmark_result_digest"),
                Self::choices("verdict", &["correct", "incorrect", "inconclusive", "not-applicable"]),
                Column::digest("evaluation_policy_digest"),
                Column::utc("recorded_at"),
                Column::identity("event_id").optional(),
            ], vec![
                Constraint::primary(&["study_outcome_id"]),
                Constraint::reference("study_assignment_id", "v61_study_assignments", "study_assignment_id"),
                Constraint::reference("event_id", "v6_event_journal", "event_id"),
            ]).immutable(),
            Table::of("v61_physical_allocations", vec![
                Column::identity("allocation_id"),
                Column::identity("research_track_id"),
                Column::identity("holder_id"),
                Self::bytes("cpu_cores"),
                Self::bytes("gpu_devices"),
                Self::bytes("memory_bytes"),
                Self::bytes("storage_bytes"),
                Self::fixed("excluded_from_performance_scoring", "1"),
                Column::digest("policy_digest"),
                Column::utc("active_from"),
                Column::utc("expires_at"),
            ], vec![Constraint::primary(&["allocation_id"])]),
        ]
    }

    /// Tables introduced at one version of the chain.
    #[must_use]
    pub fn at(version: Version) -> Vec<Table> {
        match version {
            Version::V2 => Self::v2(),
            Version::V3 => Self::v3(),
            Version::V5 => Self::v5(),
            Version::V6 => Self::v6(),
            Version::V61 => Self::v61(),
        }
    }

    /// Every table in apply order.
    #[must_use]
    pub fn all() -> Vec<Table> {
        [Version::V2, Version::V3, Version::V5, Version::V6, Version::V61]
            .into_iter()
            .flat_map(Self::at)
            .collect()
    }

    /// Look up one declared table by name.
    #[must_use]
    pub fn named(name: &str) -> Option<Table> {
        Self::all().into_iter().find(|table| table.name == name)
    }

    /// Count the tables declared at a version.
    #[must_use]
    pub fn count(version: Version) -> usize {
        Self::at(version).len()
    }
}
