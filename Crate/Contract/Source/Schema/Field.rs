//! One field of a schema definition.
//!
//! A field carries its wire name and one structural constraint. Required
//! membership lives on the enclosing `Definition` as a name list, mirroring
//! the schema's own `required` array (`schemas/ack.schema.json:36-43`).

use crate::schema::Constraint;

/// One declared schema field.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Field {
    /// Wire field name.
    pub name: &'static str,
    /// Structural constraint on the field value.
    pub constraint: Constraint,
}

impl Field {
    /// Build one field descriptor.
    #[must_use]
    pub const fn of(name: &'static str, constraint: Constraint) -> Self {
        Self { name, constraint }
    }

    /// Whether this field is a digest with the canonical 64-hex shape.
    #[must_use]
    pub fn is_digest(&self) -> bool {
        match self.constraint {
            Constraint::Pattern(pattern) => pattern == "^[a-f0-9]{64}$",
            _ => false,
        }
    }
}
