//! Redaction that strips embargo and policy reasons before worker delivery.
//! Grounded in `17-v61-constitution.md:45-57` (no worker receives enforcement
//! diagnostics) and `PUBLISHING.md:53-70` (sensitive scans before release).

/// The set of fields stripped from a worker-facing capsule.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Redaction {
    /// Field names removed before delivery.
    pub stripped: Vec<String>,
}

impl Redaction {
    /// The fields that must never reach a worker.
    pub const FORBIDDEN: [&'static str; 5] = [
        "embargo_reason",
        "policy_reason",
        "sibling_id",
        "holdout_result",
        "violation_class",
    ];

    /// Start with an empty strip list.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Strip every forbidden field from the given names.
    #[must_use]
    pub fn apply(mut self, fields: &[String]) -> Self {
        self.stripped = fields
            .iter()
            .filter(|name| Self::FORBIDDEN.contains(&name.as_str()))
            .cloned()
            .collect();
        self
    }

    /// Whether the delivery is clean of all forbidden fields.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        Self::FORBIDDEN.iter().all(|name| !self.stripped.iter().any(|s| s == name))
    }
}
