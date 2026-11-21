//! Structural constraints on one schema field.
//!
//! The corpus uses `type`, `enum`, `const`, `pattern`, `format`, `minimum` and
//! `$ref` (`schemas/sample.schema.json:8-80`,
//! `schemas/v61-worker-neutral-receipt.schema.json:16-25`). These are recorded
//! as typed variants; no JSON Schema evaluation is performed.

use crate::schema::Reference;

/// One structural constraint, without an evaluation engine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Constraint {
    /// Declared JSON type.
    Type(&'static str),
    /// Closed enumeration of string values.
    Enum(&'static [&'static str]),
    /// Fixed string value.
    Const(&'static str),
    /// Fixed numeric value.
    ConstNumber(i64),
    /// Regular expression source, compared structurally only.
    Pattern(&'static str),
    /// Format annotation, never resolved at runtime.
    Format(&'static str),
    /// Inclusive lower bound.
    Minimum(i64),
    /// Reference to another definition.
    Reference(Reference),
}

impl Constraint {
    /// Human-readable constraint family for findings.
    #[must_use]
    pub const fn kind(self) -> &'static str {
        match self {
            Self::Type(_) => "type",
            Self::Enum(_) => "enum",
            Self::Const(_) | Self::ConstNumber(_) => "const",
            Self::Pattern(_) => "pattern",
            Self::Format(_) => "format",
            Self::Minimum(_) => "minimum",
            Self::Reference(_) => "reference",
        }
    }
}
