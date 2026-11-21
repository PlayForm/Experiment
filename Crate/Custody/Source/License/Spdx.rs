//! A pinned SPDX expression with its connectives preserved.
//!
//! Mirrors the `spdx_expression` field of
//! `examples/v61-license-grant.example.json` and `docs/20:16-18`. Scanning may
//! flag terms but cannot determine contractual rights in all cases.

use crate::operator::Operator;
use crate::permission::Permission;

/// A verbatim SPDX expression, never a substitute for the governing text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Spdx {
    /// `expression` exactly as recorded, e.g. `Apache-2.0`.
    pub expression: String,
    /// Connectives actually present in the expression.
    pub operators: Vec<Operator>,
    /// Digest of the source-level LICENSE and NOTICE texts.
    pub text_digest: String,
}

impl Spdx {
    /// Whether the expression and its governing text were captured.
    #[must_use]
    pub fn complete(&self) -> Permission {
        Permission::filled(&self.expression).and(Permission::digest(&self.text_digest))
    }

    /// Whether the expression uses a named connective.
    #[must_use]
    pub fn uses(&self, operator: Operator) -> bool {
        self.operators.contains(&operator)
    }

    /// Whether the expression merely names a license without exceptions.
    #[must_use]
    pub fn bare(&self) -> bool {
        self.operators.is_empty()
    }

    /// The connectives as their SPDX spellings, for an operator review.
    #[must_use]
    pub fn labels(&self) -> Vec<&'static str> {
        self.operators.iter().map(Operator::label).collect()
    }
}
