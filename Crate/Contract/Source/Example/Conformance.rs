//! Structural conformance declaration between an example and a schema.
//!
//! `docs/26-acceptance-and-build-plan.md:60-63` requires examples to validate
//! against their schemas and all `$ref`s to resolve. This type records the
//! declared pairing and checks only that the stems agree as data; it is not a
//! JSON Schema engine and never executes an example.

/// A declared example-to-schema pairing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Conformance {
    /// Example stem.
    pub example: &'static str,
    /// Schema stem it conforms to.
    pub schema: &'static str,
}

impl Conformance {
    /// Build one conformance declaration.
    #[must_use]
    pub const fn of(example: &'static str, schema: &'static str) -> Self {
        Self { example, schema }
    }

    /// All declared pairings from the example registry.
    #[must_use]
    pub fn all() -> Vec<Self> {
        crate::example::Example::of()
            .into_iter()
            .map(|example| Self::of(example.stem, example.schema))
            .collect()
    }

    /// Structural check that the pairing is internally consistent.
    #[must_use]
    pub fn matches(&self) -> crate::Permission {
        match self.example == self.schema {
            true => crate::Permission::Allow,
            false => crate::Permission::Deny,
        }
    }
}
