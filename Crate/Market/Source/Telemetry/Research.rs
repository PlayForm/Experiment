//! Research and economy plane attributes.
//!
//! Grounded in `TELEMETRY.md:9-13`: bid size vector, resource reservation,
//! metered spend, model costs, token latency and time to first token, time to
//! first useful claim, citation retrieval, review queue, dissent coverage,
//! Brier calibration and paper release/retraction. Opaque IDs are used and no
//! private opinion enters telemetry.

/// The research/economy telemetry plane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Research;

impl Research {
    /// Declared research-plane attributes (`TELEMETRY.md:9-13`).
    pub const ATTRIBUTES: [&'static str; 12] = [
        "bid_size_vector",
        "resource_reservation",
        "metered_spend",
        "model_cost",
        "token_latency_tffb",
        "time_to_first_useful_claim",
        "citation_retrieval",
        "review_queue",
        "dissent_coverage",
        "brier_calibration",
        "paper_release",
        "paper_retraction",
    ];

    /// Correlation keys use opaque identifiers (`TELEMETRY.md:18-20`).
    pub const CORRELATION_KEYS: [&'static str; 8] = [
        "campaign_id",
        "repo_id",
        "research_case_id",
        "mutation_id",
        "experiment_id",
        "paper_id",
        "role_id",
        "resource_reservation_id",
    ];

    /// The plane this value belongs to.
    #[must_use]
    pub const fn plane() -> super::Plane {
        super::Plane::Research
    }

    /// Private opinions are never a research attribute (`TELEMETRY.md:13`).
    #[must_use]
    pub const fn may_include_opinion() -> crate::Permission {
        crate::Permission::Deny
    }
}
