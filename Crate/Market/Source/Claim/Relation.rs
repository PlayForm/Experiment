//! Typed relations between signed claims.
//!
//! Grounded in `sql/v5_migration.sql:83-97` (`supports`, `contradicts`,
//! `refines`, `replicates`, `cites`) and `claim.v5.schema.json:71-86` (the
//! supports and contradicts arrays). A relation is an explicit, signed edge;
//! semantic similarity never hard-proves equivalence (`OPERATIONS.md:64`).

use crate::Warning;

/// A directed claim-to-claim relation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Relation {
    /// Evidence or argument in favour (`sql:88`).
    Supports,
    /// A counterexample or conflicting opinion (`sql:89`, `README.md:41`).
    Contradicts,
    /// A narrower restatement (`sql:90`).
    Refines,
    /// A new independent confirmation (`sql:91`).
    Replicates,
    /// A source or paper citation (`sql:92`).
    Cites,
}

impl Relation {
    /// All relations in table order.
    pub const ALL: [Self; 5] = [
        Self::Supports,
        Self::Contradicts,
        Self::Refines,
        Self::Replicates,
        Self::Cites,
    ];

    /// Decode the SQL label, warning on an unknown one.
    pub fn from_label(label: &str) -> Result<Self, Warning> {
        match label {
            "supports" => Ok(Self::Supports),
            "contradicts" => Ok(Self::Contradicts),
            "refines" => Ok(Self::Refines),
            "replicates" => Ok(Self::Replicates),
            "cites" => Ok(Self::Cites),
            other => Err(Warning::InvalidValue {
                field: "claim relation",
                detail: format!("unknown relation {other}"),
            }),
        }
    }

    /// The SQL label of this relation.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Supports => "supports",
            Self::Contradicts => "contradicts",
            Self::Refines => "refines",
            Self::Replicates => "replicates",
            Self::Cites => "cites",
        }
    }

    /// Contradiction is protected dissent, not a defect (`MARKET.md:101-106`).
    #[must_use]
    pub const fn is_dissent(self) -> bool {
        matches!(self, Self::Contradicts)
    }
}
