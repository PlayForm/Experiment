//! Independently produced evidence bound to a signed claim.
//!
//! Grounded in `PUBLISHING.md:74-80` (distinguish reference, observation,
//! inference, interpretation and replication; store source URI, retrieval
//! timestamp, content hash, license scope and exact supported claim) and
//! `README.md:41-43` (facts come only from signed, independently produced
//! evidence). An artifact signed by the claim's own author is not independent.

use crate::Digest;

use super::Kind;

/// Evidence of one of the five published quality classes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Evidence {
    /// External literature or documentation (`PUBLISHING.md:74`).
    Reference {
        /// Source URI (`PUBLISHING.md:78`).
        source_uri: String,
        /// Content-addressed retrieval snapshot (`PUBLISHING.md:78`).
        snapshot: Digest,
    },
    /// A trace or benchmark observation (`PUBLISHING.md:74`).
    Observation {
        /// Signed artifact digest (`claim.v5.schema.json:87-90`).
        artifact: Digest,
        /// Independent signer identity (`V4_TRUST_BRIDGE.md:26`).
        signer: String,
    },
    /// A model-derived claim (`PUBLISHING.md:76`).
    Inference {
        /// Artifact digest carrying the inference.
        artifact: Digest,
        /// Signer identity; inference is never a measurement.
        signer: String,
    },
    /// An attributed opinion (`PUBLISHING.md:76`).
    Interpretation {
        /// Artifact digest carrying the interpretation.
        artifact: Digest,
        /// Attributed author identity.
        signer: String,
    },
    /// A new independent experiment (`PUBLISHING.md:76`).
    Replication {
        /// Signed replication artifact digest.
        artifact: Digest,
        /// Independent replication signer.
        signer: String,
    },
}

impl Evidence {
    /// The claim kind this evidence can support.
    #[must_use]
    pub const fn kind(&self) -> Kind {
        match self {
            Self::Reference { .. } => Kind::ExternalReference,
            Self::Observation { .. } => Kind::Observation,
            Self::Inference { .. } => Kind::Hypothesis,
            Self::Interpretation { .. } => Kind::Interpretation,
            Self::Replication { .. } => Kind::Replication,
        }
    }

    /// Whether this evidence may establish a measured fact (`README.md:41-43`).
    #[must_use]
    pub const fn is_measured(&self) -> crate::Permission {
        match self {
            Self::Observation { .. } | Self::Replication { .. } => crate::Permission::Allow,
            Self::Reference { .. } | Self::Inference { .. } | Self::Interpretation { .. } => {
                crate::Permission::Deny
            }
        }
    }

    /// The signer of evidential bytes, when one exists.
    #[must_use]
    pub fn signer(&self) -> Option<&str> {
        match self {
            Self::Reference { .. } => None,
            Self::Observation { signer, .. }
            | Self::Inference { signer, .. }
            | Self::Interpretation { signer, .. }
            | Self::Replication { signer, .. } => Some(signer),
        }
    }

    /// The claim author may not attest their own evidence
    /// (`V4_TRUST_BRIDGE.md:30-31`, `AGENTS.md:17`).
    #[must_use]
    pub const fn author_may_attest() -> crate::Permission {
        crate::Permission::Deny
    }
}
