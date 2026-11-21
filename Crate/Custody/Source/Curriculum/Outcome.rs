//! A trusted study outcome, including the honest inconclusive case.
//!
//! Mirrors `examples/v61-study-outcome.example.json` and `docs/27:15-17`:
//! facts come from trusted oracles and runners, never from the model's grades
//! or assertions. A paper cannot become evidence without signed measurement.

use crate::permission::Permission;
use crate::verdict::Verdict;
use crate::warning::Warning;

/// The recorded result of one assignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outcome {
    /// `study_outcome_id` of this record.
    pub outcome_id: String,
    /// `study_assignment_id` that was evaluated.
    pub assignment_id: String,
    /// `proposal_digest` of the sealed candidate.
    pub proposal_digest: String,
    /// `trusted_oracle_result_digest` of authoritative correctness.
    pub oracle_digest: String,
    /// `trusted_benchmark_result_digest`, absent when not measured.
    pub benchmark_digest: Option<String>,
    /// `verdict` from the trusted evaluator.
    pub verdict: Verdict,
    /// `reproducibility` state of the measurement.
    pub reproducibility: String,
    /// `published_insight_digest`, absent until sanitized release.
    pub published_insight: Option<String>,
    /// `evaluator_id`, always a trusted evaluator.
    pub evaluator_id: String,
    /// `policy_digest` applied by the evaluator.
    pub policy_digest: String,
    /// `recorded_at` canonical UTC instant.
    pub recorded_at: String,
}

impl Outcome {
    /// Record an inconclusive outcome without inventing a measurement.
    #[must_use]
    pub fn inconclusive(assignment: &str, oracle: &str, evaluator: &str, at: &str) -> Self {
        Self {
            outcome_id: String::new(),
            assignment_id: assignment.to_owned(),
            proposal_digest: String::new(),
            oracle_digest: oracle.to_owned(),
            benchmark_digest: None,
            verdict: Verdict::Inconclusive,
            reproducibility: "not-measured".to_owned(),
            published_insight: None,
            evaluator_id: evaluator.to_owned(),
            policy_digest: String::new(),
            recorded_at: at.to_owned(),
        }
    }

    /// Whether trusted evidence backs the recorded verdict.
    #[must_use]
    pub fn evidenced(&self) -> Permission {
        Permission::filled(&self.outcome_id)
            .and(Permission::filled(&self.assignment_id))
            .and(Permission::digest(&self.proposal_digest))
            .and(Permission::digest(&self.oracle_digest))
            .and(Permission::filled(&self.evaluator_id))
            .and(Permission::digest(&self.policy_digest))
            .and(Permission::filled(&self.recorded_at))
    }

    /// Findings for an outcome whose evidence is incomplete.
    #[must_use]
    pub fn validate(&self) -> Vec<Warning> {
        let digests = [
            ("proposal_digest", self.proposal_digest.as_str()),
            ("oracle_digest", self.oracle_digest.as_str()),
            ("policy_digest", self.policy_digest.as_str()),
        ]
        .into_iter()
        .filter_map(|(field, value)| {
            (!crate::digest::Digest::of(value).is_valid()).then(|| Warning::DigestShape {
                field,
                detail: crate::digest::Digest::of(value).describe(),
            })
        });
        let unnamed = [
            ("outcome id", self.outcome_id.is_empty()),
            ("assignment", self.assignment_id.is_empty()),
            ("evaluator", self.evaluator_id.is_empty()),
            ("recorded at", self.recorded_at.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, missing)| {
            missing.then(|| Warning::Malformed {
                domain: field,
                detail: "a study outcome must name this field".to_owned(),
            })
        });
        digests.chain(unnamed).collect()
    }
}
