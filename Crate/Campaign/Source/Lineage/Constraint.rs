//! The kinds of column constraint in `sql/lineage.sql`.
//!
//! Each `CREATE TABLE` uses `PRIMARY KEY`, `NOT NULL`, `CHECK`, `REFERENCES`,
//! `UNIQUE` and `AUTOINCREMENT`; these are typed so a model reader can see
//! which guarantees the schema claims without parsing SQL.

/// A declared column constraint.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Constraint {
    PrimaryKey,
    ForeignKey,
    NotNull,
    Check,
    Unique,
    AutoIncrement,
}

impl Constraint {
    /// The SQL token this represents.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::PrimaryKey => "PRIMARY KEY",
            Self::ForeignKey => "REFERENCES",
            Self::NotNull => "NOT NULL",
            Self::Check => "CHECK",
            Self::Unique => "UNIQUE",
            Self::AutoIncrement => "AUTOINCREMENT",
        }
    }
}
