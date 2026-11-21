//! The frozen B0 reference used for every paired comparison.
//! Grounded in `ARCHITECTURE.md:66-68,82-88` (freeze the baseline binary and
//! compare paired against frozen B0 with reference drift checks) and in
//! `autonomous-week.yaml:24-29` (baseline capture requires fixture integrity,
//! host doctor and a pinned oracle version).

/// The immutable baseline a candidate is measured against.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Baseline {
    /// Pinned base commit.
    pub commit: String,
    /// Digest of the baseline binary; empty means not yet built.
    pub binary_sha256: String,
    /// Version of the measurement protocol in force at capture.
    pub protocol_sha256: String,
}

impl Baseline {
    /// Record a baseline before its binary exists.
    #[must_use]
    pub fn pinned(commit: &str, protocol_sha256: &str) -> Self {
        Self {
            commit: commit.to_owned(),
            binary_sha256: String::new(),
            protocol_sha256: protocol_sha256.to_owned(),
        }
    }

    /// Attach the trusted builder's binary digest.
    #[must_use]
    pub fn built(self, binary_sha256: &str) -> Self {
        Self {
            binary_sha256: binary_sha256.to_owned(),
            ..self
        }
    }

    /// Whether this baseline is complete enough to compare against.
    #[must_use]
    pub fn is_frozen(&self) -> bool {
        [self.commit.as_str(), self.binary_sha256.as_str(), self.protocol_sha256.as_str()]
            .iter()
            .all(|value| !value.is_empty())
    }

    /// Whether another baseline is the same pinned reference.
    #[must_use]
    pub fn matches(&self, other: &Self) -> bool {
        self.binary_sha256 == other.binary_sha256 && self.protocol_sha256 == other.protocol_sha256
    }
}
