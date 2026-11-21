//! One immutable bi-temporal fact from the pinned temporal-fact example.
//!
//! Event time, observation time, validity and recording time are separate
//! quantities; a correction points at its predecessor rather than rewriting it
//! (`docs/21:39-46`, `examples/v61-temporal-fact.example.json`).

use super::recorded::Recorded;
use super::valid::Valid;
use crate::digest::Digest;
use crate::permission::Permission;
use crate::warning::Warning;

/// A fact whose truth and knowledge times are independently recorded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fact {
    /// `fact_id` of this immutable record.
    pub fact_id: String,
    /// `subject_id` the fact describes.
    pub subject_id: String,
    /// `event_time` when the observed event occurred.
    pub event_time: String,
    /// `observed_at` when the evidence was captured.
    pub observed_at: String,
    /// Validity interval of the claim.
    pub valid: Valid,
    /// Recording identity of the claim in the ledger.
    pub recorded: Recorded,
    /// `supersedes_fact_id`, set when this fact corrects an earlier one.
    pub supersedes_fact_id: Option<String>,
    /// `clock_domain`, e.g. `utc`.
    pub clock_domain: String,
    /// `clock_uncertainty_ns` of the cross-host observation.
    pub clock_uncertainty_ns: u64,
    /// `fact_payload_digest` of the sealed evidence.
    pub payload_digest: String,
}

impl Fact {
    /// Whether the fact is structurally admissible.
    #[must_use]
    pub fn validate(&self) -> Vec<Warning> {
        let absent = [
            ("fact id", self.fact_id.is_empty()),
            ("subject", self.subject_id.is_empty()),
            ("event time", self.event_time.is_empty()),
            ("observed at", self.observed_at.is_empty()),
            ("clock domain", self.clock_domain.is_empty()),
        ]
        .into_iter()
        .filter_map(|(field, missing)| {
            missing.then(|| Warning::Malformed {
                domain: field,
                detail: "a temporal fact must name this field".to_owned(),
            })
        });
        let digests = [
            ("payload_digest", self.payload_digest.as_str()),
            ("source_event_id", self.recorded.source_event_id.as_str()),
        ]
        .into_iter()
        .filter_map(|(field, value)| {
            (!Digest::of(value).is_valid()).then(|| Warning::DigestShape {
                field,
                detail: Digest::of(value).describe(),
            })
        });
        absent.chain(digests).collect()
    }

    /// Whether this fact is visible at a validity instant from a recorded view.
    #[must_use]
    pub fn visible(&self, at: &str, known_at: &str) -> Permission {
        self.valid
            .covers(at)
            .and(self.recorded.before(known_at))
            .and(Permission::filled(&self.fact_id))
    }

    /// Whether this fact is a correction of an earlier record.
    #[must_use]
    pub fn corrects(&self) -> bool {
        self.supersedes_fact_id.is_some()
    }
}
