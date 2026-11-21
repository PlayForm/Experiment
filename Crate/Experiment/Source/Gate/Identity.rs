//! Canonical patch identity: separate hashes for every immutable artifact.
//!
//! Grounded in `RESEARCH_GATE.md:7-16,53-68` (keyed source-tree digest,
//! normalized diff digest, build-recipe digest, binary digest, complete
//! experiment key; source, patch, binary, recipe, runtime policy, fixture,
//! protocol and environment each have their own identity) and
//! `REQUIREMENTS_V3.md` NOV-001/NOV-002/NOV-008. Same Git commit or feature set
//! never proves an identical binary.

/// The canonical identity of one submitted patch and its context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identity {
    /// Keyed digest of the full source tree at the parent commit.
    pub source_tree_sha256: String,
    /// Normalized diff digest, independent of formatting and imports.
    pub normalized_diff_sha256: String,
    /// Digest of the pinned build recipe.
    pub recipe_sha256: String,
    /// Digest of the produced binary; empty until built.
    pub binary_sha256: String,
    /// Digest of the runtime policy in force.
    pub runtime_policy_sha256: String,
    /// Digest of the fixture manifest.
    pub fixture_sha256: String,
    /// Digest of the measurement protocol.
    pub protocol_sha256: String,
    /// Digest of the environment cohort.
    pub environment_sha256: String,
    /// Version of the canonicalization algorithm, part of identity.
    pub fingerprint_version: String,
}

impl Identity {
    /// Whether this identity is complete enough for exact identity matching.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        [
            &self.source_tree_sha256,
            &self.normalized_diff_sha256,
            &self.recipe_sha256,
            &self.runtime_policy_sha256,
            &self.fixture_sha256,
            &self.protocol_sha256,
            &self.environment_sha256,
            &self.fingerprint_version,
        ]
        .iter()
        .all(|digest| !digest.is_empty())
    }

    /// Whether two identities are exactly equal on all declared hashes.
    #[must_use]
    pub fn exactly_matches(&self, other: &Self) -> bool {
        self.source_tree_sha256 == other.source_tree_sha256
            && self.normalized_diff_sha256 == other.normalized_diff_sha256
            && self.recipe_sha256 == other.recipe_sha256
            && self.binary_sha256 == other.binary_sha256
            && self.fingerprint_version == other.fingerprint_version
    }

    /// The canonicalization version, so indexes are rebuildable (NOV-008).
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.fingerprint_version
    }
}
