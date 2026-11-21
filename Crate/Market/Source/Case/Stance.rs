//! Researcher stances across the schema, SQL and role taxonomy.
//!
//! Grounded in `schemas/research-case.v5.schema.json:87-96` (six case roles),
//! `sql/v5_migration.sql:28-39` (nine assignment roles including editor,
//! integrator and market analyst), `architecture/AGENTS.md:7-19` (role
//! taxonomy and forbidden actions) and `architecture/MARKET.md:101-106`
//! (protected dissent).

use crate::Warning;

/// A signed role stance. One identifier names one role; the schema label is
/// kept on the wire only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Stance {
    /// Falsifiable mechanism proponent (`AGENTS.md:9`).
    Proponent,
    /// Adversarial counterexample seeker (`AGENTS.md:10`).
    Skeptic,
    /// Alternative-mechanism investigator (`AGENTS.md:11`).
    Alternative,
    /// Independent replication designer (`AGENTS.md:12`).
    Replicator,
    /// Independent methodology critic (`AGENTS.md:16`).
    Methodologist,
    /// Source and citation librarian (`AGENTS.md:15`).
    Librarian,
    /// Research editor (`sql/v5_migration.sql:36`).
    Editor,
    /// Transfer researcher (`sql/v5_migration.sql:37`).
    Integrator,
    /// Allocation-policy analyst (`sql/v5_migration.sql:38`).
    Analyst,
}

impl Stance {
    /// All nine assignment stances in SQL table order.
    pub const ALL: [Self; 9] = [
        Self::Proponent,
        Self::Skeptic,
        Self::Alternative,
        Self::Replicator,
        Self::Methodologist,
        Self::Librarian,
        Self::Editor,
        Self::Integrator,
        Self::Analyst,
    ];

    /// Decode a schema/SQL label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "proponent" => Ok(Self::Proponent),
            "skeptic" => Ok(Self::Skeptic),
            "alternative" => Ok(Self::Alternative),
            "replicator" => Ok(Self::Replicator),
            "methodologist" => Ok(Self::Methodologist),
            "librarian" => Ok(Self::Librarian),
            "editor" => Ok(Self::Editor),
            "integrator" => Ok(Self::Integrator),
            "market_analyst" => Ok(Self::Analyst),
            other => Err(Warning::InvalidValue {
                field: "stance",
                detail: format!("unknown role {other}"),
            }),
        }
    }

    /// The wire label of this stance.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Proponent => "proponent",
            Self::Skeptic => "skeptic",
            Self::Alternative => "alternative",
            Self::Replicator => "replicator",
            Self::Methodologist => "methodologist",
            Self::Librarian => "librarian",
            Self::Editor => "editor",
            Self::Integrator => "integrator",
            Self::Analyst => "market_analyst",
        }
    }

    /// Dissent and replication stances receive a protected minimum budget
    /// (`MARKET.md:101-106`, `playbooks/research-market-week.yaml:51-52`).
    #[must_use]
    pub const fn is_protected(self) -> bool {
        matches!(self, Self::Skeptic | Self::Replicator)
    }

    /// Whether the stance may appear in a case's role set
    /// (`schema:87-96` lists the first six).
    #[must_use]
    pub const fn is_case_role(self) -> bool {
        matches!(
            self,
            Self::Proponent
                | Self::Skeptic
                | Self::Alternative
                | Self::Replicator
                | Self::Methodologist
                | Self::Librarian
        )
    }
}
