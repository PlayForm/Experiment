//! The authored semantic IR kinds of the private registry.
//!
//! Mirrors the exact kind list in `docs/05:3`: `SchemaResource`, `FieldSpec`,
//! `Reference`, `PolicyRule`, `EventType`, `Migration`, `Permission`,
//! `RetentionPolicy` and `DisclosureView`. Each carries an immutable semantic
//! identifier and version (`docs/05:5-7`).

/// One immutable semantic IR kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ir {
    /// A canonical schema resource.
    SchemaResource,
    /// A single field specification.
    FieldSpec,
    /// A bounded internal reference.
    Reference,
    /// A Cedar-like policy rule.
    PolicyRule,
    /// A canonical event type.
    EventType,
    /// A signed migration description.
    Migration,
    /// A named permission.
    Permission,
    /// A retention obligation.
    RetentionPolicy,
    /// An authorized disclosure view.
    DisclosureView,
}

impl Ir {
    /// Stable kind name from `docs/05:3`.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::SchemaResource => "SchemaResource",
            Self::FieldSpec => "FieldSpec",
            Self::Reference => "Reference",
            Self::PolicyRule => "PolicyRule",
            Self::EventType => "EventType",
            Self::Migration => "Migration",
            Self::Permission => "Permission",
            Self::RetentionPolicy => "RetentionPolicy",
            Self::DisclosureView => "DisclosureView",
        }
    }

    /// All nine kinds in authored order.
    #[must_use]
    pub const fn all() -> [Self; 9] {
        [
            Self::SchemaResource,
            Self::FieldSpec,
            Self::Reference,
            Self::PolicyRule,
            Self::EventType,
            Self::Migration,
            Self::Permission,
            Self::RetentionPolicy,
            Self::DisclosureView,
        ]
    }

    /// Resolve a supplied kind name from signed IR only.
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        Self::all()
            .into_iter()
            .find(|candidate| candidate.label() == label)
    }
}
