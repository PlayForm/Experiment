//! Whether a column may hold SQL NULL, as a two-variant value.

/// Column nullability, from the explicit `NULL OR length(...) = 64` checks
/// and `NOT NULL` declarations in the chain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Nullability {
    /// `NOT NULL`: every committed row carries a value.
    Required,
    /// Null is an allowed value; a digest check may still constrain it.
    Optional,
}

impl Nullability {
    /// The declared spelling.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Required => "NOT NULL",
            Self::Optional => "NULL allowed",
        }
    }
}
