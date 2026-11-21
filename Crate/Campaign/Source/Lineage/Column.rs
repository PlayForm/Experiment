//! A typed lineage column and its constraints.
//!
//! Grounded in the `CHECK` clauses of `sql/lineage.sql`:
//! `length(manifest_sha256) = 64` (`:11`), `length(sha256) = 64` (`:17`),
//! `byte_size >= 0` (`:19`), `purpose IN ('causal','merge')` (`:56`), the
//! split enum (`:80-87`), verdict enums (`:126`, `:152-154`, `:162-171`) and
//! foreign keys throughout.

use crate::lineage::Constraint;

/// One declared column of a lineage table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Column {
    pub name: String,
    pub sql_type: String,
    pub constraints: Vec<Constraint>,
    pub reference: Option<String>,
}

impl Column {
    /// Construct a column descriptor from typed parts.
    #[must_use]
    pub fn new(name: &str, sql_type: &str, constraints: &[Constraint]) -> Self {
        Self {
            name: name.to_owned(),
            sql_type: sql_type.to_owned(),
            constraints: constraints.to_vec(),
            reference: None,
        }
    }

    /// Attach a foreign-key target (`REFERENCES <table> (<column>)`).
    #[must_use]
    pub fn referencing(mut self, target: &str) -> Self {
        self.reference = Some(target.to_owned());
        self.constraints.push(Constraint::ForeignKey);
        self
    }

    /// Render the column declaration for a human reader. This is a string
    /// description, not emitted SQL.
    #[must_use]
    pub fn line(&self) -> String {
        let rules: Vec<&str> = self.constraints.iter().map(|item| item.key()).collect();
        match &self.reference {
            Some(target) => format!(
                "{} {} {} -> {}",
                self.name,
                self.sql_type,
                rules.join(" "),
                target
            ),
            None => format!("{} {} {}", self.name, self.sql_type, rules.join(" ")),
        }
    }

    /// The digest columns that must be exactly 64 hex characters.
    pub const DIGEST_COLUMNS: [&'static str; 8] = [
        "manifest_sha256",
        "sha256",
        "patch_sha256",
        "binary_sha256",
        "recipe_sha256",
        "fixture_sha256",
        "command_spec_sha256",
        "payload_sha256",
    ];
}
