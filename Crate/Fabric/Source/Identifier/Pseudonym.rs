//! A view-scoped, domain-separated opaque pseudonym.
//!
//! A pseudonym is a keyed MAC of a canonical identity, holder scope and epoch
//! (`docs/03:45-49`). It never acts as a decryption key, and two pseudonyms
//! for different audiences or epochs are unlinkable by construction; only an
//! authorized resolver holding the private crosswalk may rejoin them
//! (`docs/02:38-39`, `README.md:16-18`). No MAC is computed here.

use crate::verdict::Verdict;
use super::scope::Scope;

/// One opaque token bound to a single scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pseudonym {
    /// Opaque base64url token of exact route length.
    pub token: String,
    /// Scope under which the token was issued.
    pub scope: Scope,
}

impl Pseudonym {
    /// Record a pseudonym already issued by an authorized broker.
    #[must_use]
    pub fn of(token: &str, scope: Scope) -> Self {
        Self {
            token: token.to_owned(),
            scope,
        }
    }

    /// Whether the token has the opaque route shape.
    #[must_use]
    pub fn well_formed(&self) -> Verdict {
        Verdict::opaque(&self.token).and(self.scope.complete())
    }

    /// Two pseudonyms are unlinkable when audience or epoch differs, even for
    /// the same canonical object (`docs/09:24-27`).
    #[must_use]
    pub fn unlinkable(&self, other: &Self) -> Verdict {
        Verdict::of(
            self.scope.audience != other.scope.audience || self.scope.epoch != other.scope.epoch,
        )
    }

    /// A pseudonym is never a key and cannot decrypt anything.
    #[must_use]
    pub const fn as_key() -> Verdict {
        Verdict::Deny
    }
}
