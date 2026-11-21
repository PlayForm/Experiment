//! The six authoritative semantic identifier kinds.
//!
//! Mirrors `semantic_kind.enum` in `schemas/identifier-definition.schema.json`
//! and its validating example. A kind is a namespace discriminant inside the
//! private registry, never a display label or a path (`docs/03:57-61`).

/// One privately stored semantic kind for a canonical identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Canonical schema resource in the sealed registry.
    Schema,
    /// Canonical laboratory event type.
    Event,
    /// Artifact or repository subject.
    Artifact,
    /// Policy rule or entitlement.
    Policy,
    /// Issuer-generated capability.
    Capability,
    /// Correspondence between scope-specific pseudonyms.
    Correspondence,
}

impl Kind {
    /// Wire spelling from the identifier-definition schema.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Schema => "schema",
            Self::Event => "event",
            Self::Artifact => "artifact",
            Self::Policy => "policy",
            Self::Capability => "capability",
            Self::Correspondence => "correspondence",
        }
    }

    /// All six kinds, in schema declaration order.
    #[must_use]
    pub const fn all() -> [Self; 6] {
        [
            Self::Schema,
            Self::Event,
            Self::Artifact,
            Self::Policy,
            Self::Capability,
            Self::Correspondence,
        ]
    }

    /// Resolve a supplied label without a branch at the call site.
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        Self::all()
            .into_iter()
            .find(|candidate| candidate.label() == label)
    }
}
