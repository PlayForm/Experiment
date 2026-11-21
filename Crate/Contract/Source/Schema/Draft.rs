//! The JSON Schema dialect declared by every schema file.
//!
//! All `schemas/*.schema.json` files begin with
//! `$schema: https://json-schema.org/draft/2020-12/schema`
//! (`schemas/sample.schema.json:2`, `schemas/v61-custody-request.schema.json:2`).

use crate::Fault;

/// Declared JSON Schema dialect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Draft {
    /// JSON Schema 2020-12, the only declared dialect.
    Draft202012,
}

impl Draft {
    /// Meta-schema URI.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft202012 => "https://json-schema.org/draft/2020-12/schema",
        }
    }

    /// Decode a meta-schema URI, degrading to a typed fault.
    #[must_use]
    pub fn parse(value: &str) -> Result<Self, Fault> {
        match value {
            "https://json-schema.org/draft/2020-12/schema" => Ok(Self::Draft202012),
            other => Err(Fault::Shape {
                domain: "json schema draft",
                detail: format!("unsupported dialect {other}"),
            }),
        }
    }
}
