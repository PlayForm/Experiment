//! SPDX expression connectives preserved from the governing terms.
//!
//! Record `AND`, `OR` and `WITH` where applicable; a bare identifier is not a
//! substitute for the expression (`docs/20:16-18`).

/// One connective in a pinned SPDX expression.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operator {
    /// Both operands apply.
    And,
    /// Either operand may satisfy the expression.
    Or,
    /// An exception applies to the named license.
    With,
}

impl Operator {
    /// The SPDX spelling preserved verbatim in the expression.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
            Self::With => "WITH",
        }
    }
}
