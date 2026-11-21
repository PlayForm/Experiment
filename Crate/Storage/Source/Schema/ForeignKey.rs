//! A declared reference from one table column to another table's key.

/// A typed foreign key, never executed against a database.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForeignKey {
    /// The referencing column.
    pub column: String,
    /// The referenced table.
    pub table: String,
    /// The referenced column.
    pub target: String,
}

impl ForeignKey {
    /// Declare a reference `column -> table(target)`.
    #[must_use]
    pub fn of(column: &str, table: &str, target: &str) -> Self {
        Self {
            column: column.to_owned(),
            table: table.to_owned(),
            target: target.to_owned(),
        }
    }

    /// Describe the reference.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("{} -> {}({})", self.column, self.table, self.target)
    }
}
