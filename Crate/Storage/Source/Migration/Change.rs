//! One atomic schema change inside a migration step.

use super::additive::Additive;
use crate::schema::column::Column;
use crate::schema::constraint::Constraint;
use crate::schema::index::Index;
use crate::schema::table::Table;

/// A declared change; destructive variants are modelled but never applied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Change {
    /// `CREATE TABLE IF NOT EXISTS`.
    AddTable(Table),
    /// A new column on an existing table; not used by the chain itself.
    AddColumn { table: String, column: Column },
    /// `CREATE INDEX IF NOT EXISTS`.
    AddIndex(Index),
    /// A new constraint on a declared table.
    AddConstraint { table: String, constraint: Constraint },
    /// An immutability or authorization trigger added defensively.
    AddTrigger { table: String, name: String },
    /// `DROP TABLE`: destructive, reported only.
    DropTable(String),
    /// `DROP COLUMN`: destructive, reported only.
    DropColumn { table: String, column: String },
    /// `ALTER COLUMN`: destructive, reported only.
    AlterColumn { table: String, column: String },
    /// `RENAME COLUMN`: destructive, reported only.
    RenameColumn { table: String, from: String, to: String },
    /// `DROP INDEX`: destructive, reported only.
    DropIndex(String),
}

impl Change {
    /// Classify this change additively.
    #[must_use]
    pub fn additive(&self) -> Additive {
        match self {
            Self::AddTable(_)
            | Self::AddColumn { .. }
            | Self::AddIndex(_)
            | Self::AddConstraint { .. }
            | Self::AddTrigger { .. } => Additive::Additive,
            Self::DropTable(_)
            | Self::DropColumn { .. }
            | Self::AlterColumn { .. }
            | Self::RenameColumn { .. }
            | Self::DropIndex(_) => Additive::Destructive,
        }
    }

    /// The table this change targets, when it targets one.
    #[must_use]
    pub fn target(&self) -> &str {
        match self {
            Self::AddTable(table) => &table.name,
            Self::AddColumn { table, .. }
            | Self::AddConstraint { table, .. }
            | Self::AddTrigger { table, .. }
            | Self::DropColumn { table, .. }
            | Self::AlterColumn { table, .. }
            | Self::RenameColumn { table, .. } => table,
            Self::AddIndex(index) => &index.table,
            Self::DropTable(table) | Self::DropIndex(table) => table,
        }
    }

    /// Describe the change for a finding.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::AddTable(table) => format!("add table {}", table.name),
            Self::AddColumn { table, column } => format!("add column {table}.{}", column.name),
            Self::AddIndex(index) => format!("add index {}", index.describe()),
            Self::AddConstraint { table, constraint } => {
                format!("add constraint {table}: {}", constraint.describe())
            }
            Self::AddTrigger { table, name } => format!("add trigger {name} on {table}"),
            Self::DropTable(table) => format!("drop table {table}"),
            Self::DropColumn { table, column } => format!("drop column {table}.{column}"),
            Self::AlterColumn { table, column } => format!("alter column {table}.{column}"),
            Self::RenameColumn { table, from, to } => {
                format!("rename column {table}.{from} to {to}")
            }
            Self::DropIndex(name) => format!("drop index {name}"),
        }
    }
}
