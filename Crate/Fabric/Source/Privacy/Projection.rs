//! A redacted, scope-specific projection for telemetry or an operator view.
//!
//! `docs/07:3-14` keeps three planes plus a projected Plane 0 and forbids
//! exporting repository paths, prompts, schema refs or private identities.
//! `README.md:16-18` grants a privileged observer an audited view rather than
//! all private mappings by default.

use crate::verdict::Verdict;

/// One authorized projection over a scope and epoch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Projection {
    /// Scope whose opaque handle the projection is keyed to.
    pub scope: String,
    /// Key epoch in which the projection is valid.
    pub epoch: u64,
    /// Purpose bound to the projection.
    pub purpose: String,
    /// Neutral field names the projection may expose.
    pub fields: Vec<&'static str>,
}

impl Projection {
    /// Record a projection from trusted fields.
    #[must_use]
    pub fn of(scope: &str, epoch: u64, purpose: &str, fields: Vec<&'static str>) -> Self {
        Self {
            scope: scope.to_owned(),
            epoch,
            purpose: purpose.to_owned(),
            fields,
        }
    }

    /// Whether the projection uses only the declared neutral field set.
    #[must_use]
    pub fn neutral(&self) -> Verdict {
        let permitted = super::metadata::Metadata::neutral_fields();
        let clean = self
            .fields
            .iter()
            .all(|field| permitted.contains(field));
        Verdict::filled(&self.scope)
            .and(Verdict::filled(&self.purpose))
            .and(Verdict::of(self.epoch > 0))
            .and(Verdict::of(clean))
    }

    /// A projection never contains a private crosswalk mapping.
    #[must_use]
    pub const fn contains_mapping() -> bool {
        false
    }
}
