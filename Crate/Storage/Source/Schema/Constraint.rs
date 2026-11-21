//! Table-level constraints of the additive chain.

use super::check::Check;
use super::foreign_key::ForeignKey;

/// One primary, unique, check or referential constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constraint {
    /// Composite or single-column primary key.
    Primary(Vec<String>),
    /// Composite or single-column uniqueness rule.
    Unique(Vec<String>),
    /// A structural check.
    Check(Check),
    /// A declared reference to another table.
    Reference(ForeignKey),
}

impl Constraint {
    /// Declare a primary key over the named columns.
    #[must_use]
    pub fn primary(columns: &[&str]) -> Self {
        Self::Primary(columns.iter().map(|name| (*name).to_owned()).collect())
    }

    /// Declare a uniqueness rule over the named columns.
    #[must_use]
    pub fn unique(columns: &[&str]) -> Self {
        Self::Unique(columns.iter().map(|name| (*name).to_owned()).collect())
    }

    /// Declare a check constraint.
    #[must_use]
    pub fn check(check: Check) -> Self {
        Self::Check(check)
    }

    /// Declare a foreign key.
    #[must_use]
    pub fn reference(column: &str, table: &str, target: &str) -> Self {
        Self::Reference(ForeignKey::of(column, table, target))
    }

    /// Describe the constraint as one typed line.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::Primary(columns) => format!("primary({})", columns.join(",")),
            Self::Unique(columns) => format!("unique({})", columns.join(",")),
            Self::Check(check) => check.describe(),
            Self::Reference(reference) => format!("references {}", reference.describe()),
        }
    }
}
