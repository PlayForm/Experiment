//! The ten playbook phases as typed records with exact YAML strings.
//!
//! Every field below is transcribed from `playbooks/autonomous-week.yaml:23-83`:
//! `id`, `role`, `action`, `requires`, `output` and `approval`. This is a typed
//! contract for a future coordinator; no phase is executed.

use crate::{
    week::{Approval, Kind, Role},
    Warning,
};

/// One playbook phase.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Phase {
    pub kind: Kind,
    pub role: Role,
    pub action: String,
    pub requires: Vec<String>,
    pub outputs: Vec<String>,
    pub approval: Approval,
}

impl Phase {
    /// Convert YAML tokens into owned strings without parsing any text.
    fn words(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| (*item).to_owned()).collect()
    }

    /// Build the ten phases in exact playbook order.
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self {
                kind: Kind::Baseline,
                role: Role::Runner,
                action: "baseline_capture".to_owned(),
                requires: Self::words(&["fixture_integrity", "host_doctor", "oracle_version_pinned"]),
                outputs: Self::words(&["baseline_artifact", "baseline_samples", "environment_fingerprint"]),
                approval: Approval::Automatic,
            },
            Self {
                kind: Kind::Scout,
                role: Role::Scout,
                action: "profile_reference_and_rank_bottlenecks".to_owned(),
                requires: Self::words(&["baseline"]),
                outputs: Self::words(&["hypothesis_cards", "stage_cost_decomposition"]),
                approval: Approval::Constrained,
            },
            Self {
                kind: Kind::Mutate,
                role: Role::Mutator,
                action: "implement_one_conceptual_diff_in_new_worktree".to_owned(),
                requires: Self::words(&["approved_hypothesis", "parent_commit"]),
                outputs: Self::words(&["git_commit", "patch_digest", "mutation_manifest"]),
                approval: Approval::Constrained,
            },
            Self {
                kind: Kind::Static,
                role: Role::Builder,
                action: "check_format_lints_build".to_owned(),
                requires: Self::words(&["committed_patch"]),
                outputs: Self::words(&["static_check_results", "built_binary_hash"]),
                approval: Approval::Automatic,
            },
            Self {
                kind: Kind::Semantic,
                role: Role::Oracle,
                action: "independent_equivalence_and_contract_tests".to_owned(),
                requires: Self::words(&["built_binary_hash", "fixed_oracle_version"]),
                outputs: Self::words(&["oracle_pass_fail", "oracle_diff"]),
                approval: Approval::Automatic,
            },
            Self {
                kind: Kind::Measure,
                role: Role::Runner,
                action: "exclusive_cpu_paired_hyperfine_blocks".to_owned(),
                requires: Self::words(&["semantic_gate_passed", "host_lease", "fixture_digest"]),
                outputs: Self::words(&["raw_sample_artifacts", "trial_environment", "reference_samples"]),
                approval: Approval::Automatic,
            },
            Self {
                kind: Kind::Analyze,
                role: Role::Analyst,
                action: "robust_paired_comparison".to_owned(),
                requires: Self::words(&["valid_samples"]),
                outputs: Self::words(&["effect_size", "confidence_interval", "regression_warnings", "verdict"]),
                approval: Approval::Threshold,
            },
            Self {
                kind: Kind::Confirm,
                role: Role::Runner,
                action: "heldout_validation".to_owned(),
                requires: Self::words(&["promising_development_result", "sealed_holdout"]),
                outputs: Self::words(&["confirmation_samples", "overall_verdict"]),
                approval: Approval::Eligible,
            },
            Self {
                kind: Kind::Curate,
                role: Role::Curator,
                action: "append_evidence_and_publish_local_report".to_owned(),
                requires: Self::words(&["verdict"]),
                outputs: Self::words(&["lineage_edges", "experiment_record", "report"]),
                approval: Approval::Recorded,
            },
            Self {
                kind: Kind::Integrate,
                role: Role::Integrator,
                action: "propose_mutation_merge_as_new_experiment".to_owned(),
                requires: Self::words(&["eligible_mutations", "compatibility_validation"]),
                outputs: Self::words(&["integration_branch", "fresh_combination_benchmark_request"]),
                approval: Approval::Human,
            },
        ]
    }

    /// The exact YAML `id` token.
    #[must_use]
    pub const fn key(&self) -> &'static str {
        self.kind.key()
    }

    /// Whether this is the final, human-gated phase.
    #[must_use]
    pub const fn is_final(&self) -> bool {
        matches!(self.kind, Kind::Integrate)
    }

    /// Warn on an incomplete phase contract rather than assuming defaults.
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        match (self.requires.is_empty(), self.outputs.is_empty()) {
            (true, _) => warnings.push(Warning::Empty {
                domain: "playbook.phase.requires",
            }),
            (_, true) => warnings.push(Warning::Empty {
                domain: "playbook.phase.output",
            }),
            _ => {}
        }
    }
}
