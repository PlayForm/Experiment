//! A falsifiable mechanism hypothesis bound to one atomic mutation.
//!
//! Grounded in `README.md:44-45`, `AGENTS.md:7-9` (falsifiable claims,
//! preregistered mechanism key, calibrated probability, alternative
//! explanations and citations), `MARKET.md:34` (mechanism key and novelty
//! fingerprint) and `SYSTEM.md:120` (a hypothesis is not source verification).

use crate::Permission;
use super::falsifier::Falsifier;

/// One independently proposed mechanism hypothesis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hypothesis {
    /// Deterministic mechanism key used for duplicate gating (`MARKET.md:34`).
    pub mechanism_key: &'static str,
    /// The falsifiable statement registered before outcomes (`AGENTS.md:9`).
    pub statement: &'static str,
    /// The declared falsifier status (`README.md:44`).
    pub falsifier: Falsifier,
    /// Author-supplied information-gain basis; advisory only (`MARKET.md:45`).
    pub information_gain_basis: &'static str,
}

impl Hypothesis {
    /// Construct a hypothesis value.
    #[must_use]
    pub const fn prototype() -> Self {
        Self {
            mechanism_key: "mechanism_key",
            statement: "a falsifiable statement about one atomic mutation",
            falsifier: Falsifier::Absent,
            information_gain_basis: "registered_basis",
        }
    }

    /// A hypothesis with no falsifier is not a preregistered claim
    /// (`AGENTS.md:9`, `PUBLISHING.md:15`).
    #[must_use]
    pub const fn is_preregistered(&self) -> Permission {
        match self.falsifier {
            Falsifier::Declared => Permission::Allow,
            Falsifier::Absent => Permission::Deny,
        }
    }

    /// The author cannot promote their own hypothesis to fact
    /// (`SYSTEM.md:44-45`, `README.md:44-45`).
    #[must_use]
    pub const fn author_may_promote() -> Permission {
        Permission::Deny
    }
}
