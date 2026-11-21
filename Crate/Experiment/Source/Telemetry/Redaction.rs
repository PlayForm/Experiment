//! PII, path and command redaction before persistence or span export.
//!
//! Grounded in `REQUIREMENTS_V3.md` TEL-012/DAT-002/DAT-003,
//! `TELEMETRY.md:110-113,158` (protect output/path/command contents; record
//! counts or salted ids instead) and `AGENT_CONTRACTS.md:60-67` (sensitive text
//! encrypted at rest or redacted before persistence). Exception declared:
//! `Field` and `Classification` are typed forms of this policy's fields and
//! have no independent lifecycle outside `Redaction`.

/// A field class subject to redaction policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Field {
    /// Absolute or relative source path.
    Path,
    /// Raw user command string.
    Command,
    /// Model prompt or response text.
    Prompt,
    /// Source file contents.
    Source,
}

/// Retention classification of an artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Classification {
    /// Safe to export as a low-cardinality label.
    Public,
    /// Redact or encrypt at rest per policy.
    Sensitive,
    /// Sealed holdout; never exported to an agent view.
    Sealed,
}

/// A versioned redaction policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Redaction {
    /// Stable policy identifier recorded on each turn and capsule.
    pub policy_id: String,
    /// Field classes this policy redacts by default.
    pub fields: Vec<Field>,
    /// Classification applied to the artifact.
    pub classification: Classification,
}

impl Redaction {
    /// Whether a field is redacted by this policy.
    #[must_use]
    pub fn covers(&self, field: Field) -> bool {
        self.fields.contains(&field)
    }

    /// Whether source paths are excluded from span attributes by default
    /// (`TELEMETRY.md:110-113`).
    #[must_use]
    pub fn paths_excluded(&self) -> bool {
        self.covers(Field::Path)
    }

    /// Whether a sealed artifact may be exported to an agent. It never may.
    #[must_use]
    pub const fn may_export_sealed() -> bool {
        false
    }
}
