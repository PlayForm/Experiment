//! Plane-tagged metric descriptor with a strict attribute allowlist.
//!
//! Grounded in `docs/07-dual-telemetry.md:72-126`,
//! `docs/24-dual-observability-and-correspondence.md:51-58` and
//! `docs/25-failure-analysis.md` F21/F22: control and product metrics live in
//! separate storages; only allowlisted low-cardinality attributes are accepted;
//! prompt text, source, argv, paths, secrets, sibling IDs and embargo targets
//! are rejected at ingestion. A raw model claim is never a measurement metric.
//! Dropping a metric never changes authoritative evidence.

/// A metric descriptor bound to one telemetry plane.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metric {
    /// Metric name from the declared catalog.
    pub name: String,
    /// `control` or `product` telemetry plane.
    pub plane: String,
    /// Whether this metric can influence a release decision.
    pub authoritative: crate::Permission,
    /// Attribute keys rejected at ingestion.
    pub forbidden: Vec<&'static str>,
}

impl Metric {
    /// Declare a non-authoritative metric with the default forbidden set.
    #[must_use]
    pub fn of(name: String, plane: String) -> Result<Self, crate::Warning> {
        match plane.as_str() {
            "control" | "product" => Ok(Self {
                name,
                plane,
                authoritative: crate::Permission::Deny,
                forbidden: Self::denied_attributes().to_vec(),
            }),
            _ => Err(crate::Warning::InvalidValue {
                field: "telemetry plane",
                detail: format!("unknown telemetry plane {plane}"),
            }),
        }
    }

    /// Attributes that can never appear in a telemetry payload
    /// (`docs/07:120-126`, `docs/24:53-58`).
    #[must_use]
    pub const fn denied_attributes() -> [&'static str; 8] {
        [
            "prompt",
            "source",
            "argv",
            "file_path",
            "secret",
            "sibling_id",
            "embargo_target",
            "private_claim",
        ]
    }

    /// Whether an attribute is admissible at ingestion.
    #[must_use]
    pub fn admits(&self, key: &str) -> crate::Permission {
        let denied = self.forbidden.iter().any(|item| *item == key);
        crate::Permission::from(!denied)
    }
}
