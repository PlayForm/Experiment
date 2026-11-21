//! A typed column with its declared type, nullability and checks.

use super::check::Check;
use super::nullability::Nullability;
use super::scalar::Type;

/// One column of one table in the chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Column {
    /// Declared column name.
    pub name: String,
    /// Declared storage affinity.
    pub kind: Type,
    /// Whether NULL is admitted.
    pub null: Nullability,
    /// Structural constraints attached to the column.
    pub checks: Vec<Check>,
    /// Declared default, when the research schema supplies one.
    pub default: Option<String>,
}

impl Column {
    /// Declare a required column of a given affinity.
    #[must_use]
    pub fn of(name: &str, kind: Type) -> Self {
        Self {
            name: name.to_owned(),
            kind,
            null: Nullability::Required,
            checks: Vec::new(),
            default: None,
        }
    }

    /// Declare a 64-hex digest column.
    #[must_use]
    pub fn digest(name: &str) -> Self {
        Self::of(name, Type::Text).checked(Check::Digest(name.to_owned()))
    }

    /// Declare a UTC timestamp column.
    #[must_use]
    pub fn utc(name: &str) -> Self {
        Self::of(name, Type::Text)
    }

    /// Declare a text identifier column.
    #[must_use]
    pub fn identity(name: &str) -> Self {
        Self::of(name, Type::Text)
    }

    /// Declare an auto-increment sequence or non-negative counter column.
    #[must_use]
    pub fn counter(name: &str) -> Self {
        Self::of(name, Type::Integer).checked(Check::AtLeast {
            column: name.to_owned(),
            bound: 0,
        })
    }

    /// Permit NULL on this column.
    #[must_use]
    pub fn optional(mut self) -> Self {
        self.null = Nullability::Optional;
        self
    }

    /// Attach a structural check.
    #[must_use]
    pub fn checked(mut self, check: Check) -> Self {
        self.checks.push(check);
        self
    }

    /// Attach a declared default.
    #[must_use]
    pub fn defaulted(mut self, value: &str) -> Self {
        self.default = Some(value.to_owned());
        self
    }

    /// Report malformed declarations without panicking.
    #[must_use]
    pub fn validate(&self) -> Vec<crate::warning::Warning> {
        let empty = self.name.is_empty().then(|| crate::warning::Warning::Malformed {
            domain: "column",
            detail: "a column must have a name".to_owned(),
        });
        let bad_digest = self.checks.iter().filter_map(|check| match check {
            Check::Digest(column) if column != &self.name => {
                Some(crate::warning::Warning::Malformed {
                    domain: "check",
                    detail: format!("digest check names {column}, not {}", self.name),
                })
            }
            _ => None,
        });
        empty.into_iter().chain(bad_digest).collect()
    }

    /// Describe the column as one typed line.
    #[must_use]
    pub fn describe(&self) -> String {
        let checks = self
            .checks
            .iter()
            .map(Check::describe)
            .collect::<Vec<_>>()
            .join("; ");
        format!("{} {} {} {checks}", self.name, self.kind.affinity(), self.null.label())
    }
}
