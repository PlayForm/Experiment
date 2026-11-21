//! Fail-closed admission of typed identifiers into the private registry.
//!
//! Canonicalization and uniqueness checks occur before insertion and before
//! grant evaluation (`docs/03:57-61`). A mocked generator that returns an
//! existing identifier must abort reservation without overwriting any record
//! or grant (`docs/09:19-20`). This registry holds typed descriptions only;
//! it inserts nothing and opens no store.

use crate::verdict::Verdict;
use super::collision::Collision;
use super::id::ID;

/// A read-only view of the registry used for structural reservation checks.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Registry {
    /// The digest of the signed registry revision this view models.
    pub revision_digest: String,
}

impl Registry {
    /// Classify a candidate against already admitted identifiers without
    /// mutating either value. Iteration is a named chain, not a loop.
    #[must_use]
    pub fn classify<'a>(
        &self,
        admitted: impl IntoIterator<Item = &'a ID>,
        candidate: &ID,
    ) -> Collision {
        admitted.into_iter().fold(Collision::Clear, |found, entry| {
            match found {
                Collision::Clear => match (entry.kind == candidate.kind, entry.value == candidate.value) {
                    (true, true) => Collision::Version,
                    (false, true) => Collision::Namespace,
                    _ => Collision::Clear,
                },
                other => other,
            }
        })
    }

    /// Admit a candidate only when no collision was found.
    #[must_use]
    pub fn reserve<'a>(
        &self,
        admitted: impl IntoIterator<Item = &'a ID>,
        candidate: &ID,
    ) -> Verdict {
        candidate
            .shaped()
            .and(candidate.same_slot(candidate))
            .after(self.classify(admitted, candidate))
    }

    /// A registry view is authoritative only when its signed revision digest
    /// has the required shape. No store is opened to check it.
    #[must_use]
    pub fn authoritative(&self) -> Verdict {
        Verdict::digest(&self.revision_digest)
    }
}
