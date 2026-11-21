//! A named lookup index declared by a migration step.

/// A typed index over one table's columns.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Index {
    /// Declared index name.
    pub name: String,
    /// Table the index covers.
    pub table: String,
    /// Indexed columns in declared order.
    pub columns: Vec<String>,
}

impl Index {
    /// Declare an index.
    #[must_use]
    pub fn of(name: &str, table: &str, columns: &[&str]) -> Self {
        Self {
            name: name.to_owned(),
            table: table.to_owned(),
            columns: columns.iter().map(|column| (*column).to_owned()).collect(),
        }
    }

    /// Whether this index begins with the given correlation prefix.
    #[must_use]
    pub fn begins_with(&self, prefix: &[&str]) -> bool {
        prefix.len() <= self.columns.len()
            && prefix
                .iter()
                .zip(self.columns.iter())
                .all(|(left, right)| left == &right.as_str())
    }

    /// Describe the index.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("{} on {}({})", self.name, self.table, self.columns.join(","))
    }
}
