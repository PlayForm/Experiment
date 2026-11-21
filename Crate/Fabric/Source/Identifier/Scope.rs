//! The view scope that separates otherwise linkable pseudonyms.
//!
//! Mirrors the view-scoped pseudonym inputs of `docs/03:44-49`, the audience
//! and epoch fields of `schemas/private-crosswalk.schema.json`, and the routing
//! metadata a broker may see but cannot use as authority (`docs/02:29-33`).

use crate::verdict::Verdict;

/// The domain-separated inputs to one authorized projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scope {
    /// Namespace or audience this pseudonym is minted for.
    pub audience: String,
    /// Key epoch in which the pseudonym is valid.
    pub epoch: u64,
    /// Purpose binding from the crosswalk schema.
    pub purpose: String,
    /// Holder or view identifier requesting resolution.
    pub holder: String,
}

impl Scope {
    /// Build a scope from trusted request fields.
    #[must_use]
    pub fn of(audience: &str, epoch: u64, purpose: &str, holder: &str) -> Self {
        Self {
            audience: audience.to_owned(),
            epoch,
            purpose: purpose.to_owned(),
            holder: holder.to_owned(),
        }
    }

    /// The domain-separation label for a keyed MAC over this scope. No MAC is
    /// computed here; the string only records which domain would be used
    /// (`docs/04:25-28`).
    #[must_use]
    pub fn domain_label(&self) -> String {
        format!(
            "v62|{}|{}|{}|{}",
            self.audience, self.epoch, self.purpose, self.holder
        )
    }

    /// Whether all four inputs are present and the epoch is nonzero.
    #[must_use]
    pub fn complete(&self) -> Verdict {
        Verdict::filled(&self.audience)
            .and(Verdict::filled(&self.purpose))
            .and(Verdict::filled(&self.holder))
            .and(Verdict::of(self.epoch > 0))
    }

    /// A routing token is not authority (`docs/01:24-25`, `docs/02:31-33`).
    #[must_use]
    pub const fn confers_authority() -> bool {
        false
    }
}
