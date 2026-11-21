//! Canonical patch identity, delegating to the v3 gate model.
//! Grounded in `RESEARCH_GATE.md:7-16,53-68` (keyed source-tree digest,
//! normalized diff digest, build-recipe, binary and complete experiment key;
//! same commit or feature set does not prove the same binary).

/// A canonical identity bound to the trusted v3 gate identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identity {
    /// The canonical identity owned by `experiment::gate::identity`.
    pub canonical: experiment::gate::identity::Identity,
    /// Version of the canonicalization algorithm, part of identity.
    pub fingerprint_version: String,
}

impl Identity {
    /// Wrap a canonical identity from the trusted gate crate.
    #[must_use]
    pub fn of(canonical: experiment::gate::identity::Identity) -> Self {
        let fingerprint_version = canonical.algorithm().to_owned();
        Self {
            canonical,
            fingerprint_version,
        }
    }

    /// Whether the identity has all required digests.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.canonical.is_complete()
    }

    /// Whether two identities are exactly equal, including the algorithm
    /// version; similarity never proves equivalence.
    #[must_use]
    pub fn exactly_matches(&self, other: &Self) -> bool {
        self.canonical.exactly_matches(&other.canonical)
            && self.fingerprint_version == other.fingerprint_version
    }
}
