//! A typed table declaration from one step of the additive chain.

use super::column::Column;
use super::constraint::Constraint;
use super::index::Index;
use super::mutability::Mutability;

/// One table, its columns, constraints, indexes and mutation policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Table {
    /// Declared table name.
    pub name: String,
    /// Declared columns in schema order.
    pub columns: Vec<Column>,
    /// Primary, unique, check and reference constraints.
    pub constraints: Vec<Constraint>,
    /// Named lookup indexes declared with the table.
    pub indexes: Vec<Index>,
    /// Append-only or rebuildable-projection policy.
    pub mutability: Mutability,
}

impl Table {
    /// Declare a table with its columns and constraints.
    #[must_use]
    pub fn of(name: &str, columns: Vec<Column>, constraints: Vec<Constraint>) -> Self {
        Self {
            name: name.to_owned(),
            columns,
            constraints,
            indexes: Vec::new(),
            mutability: Mutability::Projection,
        }
    }

    /// Attach the indexes declared alongside this table.
    #[must_use]
    pub fn indexed(mut self, indexes: Vec<Index>) -> Self {
        self.indexes = indexes;
        self
    }

    /// Mark the table append-only, as the chain's triggers do.
    #[must_use]
    pub fn immutable(mut self) -> Self {
        self.mutability = Mutability::Immutable;
        self
    }

    /// Find a declared column by name.
    #[must_use]
    pub fn column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|column| column.name == name)
    }

    /// All digest identity columns on this table.
    #[must_use]
    pub fn digest_columns(&self) -> Vec<&Column> {
        self.columns
            .iter()
            .filter(|column| column.checks.iter().any(|check| matches!(check, super::check::Check::Digest(_) | super::check::Check::NullOrDigest(_))))
            .collect()
    }

    /// The declared primary key columns, when present.
    #[must_use]
    pub fn primary_key(&self) -> Vec<&str> {
        self.constraints
            .iter()
            .filter_map(|constraint| match constraint {
                Constraint::Primary(columns) => Some(columns.iter().map(String::as_str).collect()),
                _ => None,
            })
            .next()
            .unwrap_or_default()
    }

    /// Guard a mutation of this table by its declared policy.
    #[must_use]
    pub fn mutation(&self) -> crate::permission::Permission {
        self.mutability.mutation()
    }

    /// Report malformed declarations without panicking.
    #[must_use]
    pub fn validate(&self) -> Vec<crate::warning::Warning> {
        let empty = self.name.is_empty().then(|| crate::warning::Warning::Malformed {
            domain: "table",
            detail: "a table must have a name".to_owned(),
        });
        let duplicates = self
            .columns
            .iter()
            .enumerate()
            .filter_map(|(at, column)| {
                self.columns[at + 1..]
                    .iter()
                    .any(|other| other.name == column.name)
                    .then(|| crate::warning::Warning::Collision {
                        domain: "column",
                        id: format!("{}.{}", self.name, column.name),
                    })
            });
        let bad_columns = self
            .columns
            .iter()
            .flat_map(Column::validate)
            .collect::<Vec<_>>();
        empty.into_iter().chain(duplicates).chain(bad_columns).collect()
    }

    /// Describe the table as one typed line.
    #[must_use]
    pub fn describe(&self) -> String {
        let columns = self
            .columns
            .iter()
            .map(Column::describe)
            .collect::<Vec<_>>()
            .join(" | ");
        format!("{} [{}] {} {columns}", self.name, self.mutability.label(), self.constraints.len())
    }
}
