//! The neutral telemetry field set and the forbidden-field list.
//!
//! `docs/07:9-14` declares the nine default OpenTelemetry fields and the
//! classes that must never be exported. `docs/07:12-14` requires rotating a
//! stable opaque ID per tenant, view and epoch because even an opaque ID can
//! link activities. This value names fields; it emits no span.

use crate::verdict::Verdict;

/// The neutral field catalog used by trusted telemetry senders.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Metadata;

impl Metadata {
    /// The nine neutral fields from `docs/07:9-10`.
    #[must_use]
    pub const fn neutral_fields() -> [&'static str; 9] {
        [
            "trace_id",
            "span_id",
            "cohort_pseudonym",
            "operation_class",
            "duration_ns",
            "result_class",
            "container_image_digest",
            "policy_epoch_bucket",
            "attestation_digest",
        ]
    }

    /// Field classes that must never cross the telemetry boundary
    /// (`docs/07:11-13`).
    #[must_use]
    pub const fn forbidden_fields() -> [&'static str; 8] {
        [
            "repository_path",
            "prompt_text",
            "schema_ref",
            "feature_name",
            "private_identity",
            "raw_error",
            "certificate_subject",
            "decrypted_event_type",
        ]
    }

    /// Whether a supplied field is allowed in a projection.
    #[must_use]
    pub fn permits(field: &str) -> Verdict {
        let allowed = Self::neutral_fields().contains(&field);
        let forbidden = Self::forbidden_fields().contains(&field);
        Verdict::of(allowed && !forbidden)
    }

    /// A stable ID is rotated per tenant, view and epoch; a view cannot treat
    /// one as a global identity (`docs/07:14`).
    #[must_use]
    pub const fn global_identity() -> bool {
        false
    }
}
