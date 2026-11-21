//! Every CHECK shape used by the migration chain, as typed data.
//!
//! Digest columns check `length(x) = 64`; enumerated columns check `IN (...)`;
//! counters check a lower bound; custody checks compare two columns. No
//! expression here is executed against a database.

/// One structural CHECK constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Check {
    /// `length(column) = 64` for a SHA-256 hex digest.
    Digest(String),
    /// `length(column) = n` for a named non-digest width.
    HexLength { column: String, length: usize },
    /// `column IN (values)` for a closed state set.
    OneOf { column: String, values: Vec<String> },
    /// `column >= bound`.
    AtLeast { column: String, bound: i64 },
    /// `column <= bound`.
    AtMost { column: String, bound: i64 },
    /// `column <= other_column`, as in available bytes versus capacity.
    AtMostColumn { column: String, bound: String },
    /// `column > 0` for a fencing token.
    Positive(String),
    /// `left <> right`, as in distinct source and target holders.
    NotEqual { left: String, right: String },
    /// `column IS NULL OR length(column) = 64`.
    NullOrDigest(String),
    /// `column = value` for a frozen publisher or protocol constant.
    Exactly { column: String, value: String },
    /// All inner checks hold simultaneously.
    All(Vec<Check>),
}

impl Check {
    /// The column this check constrains, when it constrains exactly one.
    #[must_use]
    pub fn column(&self) -> Option<&str> {
        match self {
            Self::Digest(column)
            | Self::Positive(column)
            | Self::NullOrDigest(column) => Some(column),
            Self::HexLength { column, .. }
            | Self::OneOf { column, .. }
            | Self::AtLeast { column, .. }
            | Self::AtMost { column, .. }
            | Self::AtMostColumn { column, .. }
            | Self::Exactly { column, .. } => Some(column),
            Self::NotEqual { left, .. } => Some(left),
            Self::All(inner) => inner.first().and_then(Self::column),
        }
    }

    /// A stable human-readable description of the predicate.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::Digest(column) => format!("length({column}) = 64"),
            Self::HexLength { column, length } => format!("length({column}) = {length}"),
            Self::OneOf { column, values } => format!("{column} in {}", values.join("|")),
            Self::AtLeast { column, bound } => format!("{column} >= {bound}"),
            Self::AtMost { column, bound } => format!("{column} <= {bound}"),
            Self::AtMostColumn { column, bound } => format!("{column} <= {bound}"),
            Self::Positive(column) => format!("{column} > 0"),
            Self::NotEqual { left, right } => format!("{left} <> {right}"),
            Self::NullOrDigest(column) => format!("{column} null or length = 64"),
            Self::Exactly { column, value } => format!("{column} = {value}"),
            Self::All(inner) => inner.iter().map(Self::describe).collect::<Vec<_>>().join(" and "),
        }
    }
}
