//! The five versions of the additive chain and their apply order.
//!
//! v4 is a trust/security delta with no SQL migration in the research corpus,
//! so it is absent from this chain (`docs/08-data-and-storage.md:70-80`).

/// One version of the persistent store schema.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Version {
    /// `sql/prerequisites/v2_lineage.sql` and `prun_optimization_v2/sql/lineage.sql`.
    V2 = 2,
    /// `sql/prerequisites/v3_migration.sql` and `prun_optimization_v3/sql/v3_migration.sql`.
    V3 = 3,
    /// `sql/prerequisites/v5_migration.sql` and `prun_optimization_v5/sql/v5_migration.sql`.
    V5 = 5,
    /// `sql/v6_migration.sql`: journal, inbox, outbox and correspondence.
    V6 = 6,
    /// `sql/v61_migration.sql`: custody, tenancy, curriculum and study extension.
    V61 = 61,
}

impl Version {
    /// All versions in apply order.
    #[must_use]
    pub fn all() -> [Self; 5] {
        [Self::V2, Self::V3, Self::V5, Self::V6, Self::V61]
    }

    /// The research filename that declares this version.
    #[must_use]
    pub fn source(&self) -> &'static str {
        match self {
            Self::V2 => "sql/prerequisites/v2_lineage.sql",
            Self::V3 => "sql/prerequisites/v3_migration.sql",
            Self::V5 => "sql/prerequisites/v5_migration.sql",
            Self::V6 => "sql/v6_migration.sql",
            Self::V61 => "sql/v61_migration.sql",
        }
    }

    /// The documented short label.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::V2 => "v2",
            Self::V3 => "v3",
            Self::V5 => "v5",
            Self::V6 => "v6",
            Self::V61 => "v6.1",
        }
    }

    /// The version that must precede this one, if any.
    #[must_use]
    pub fn predecessor(self) -> Option<Self> {
        match self {
            Self::V2 => None,
            Self::V3 => Some(Self::V2),
            Self::V5 => Some(Self::V3),
            Self::V6 => Some(Self::V5),
            Self::V61 => Some(Self::V6),
        }
    }

    /// The next version in the chain, if any.
    #[must_use]
    pub fn successor(self) -> Option<Self> {
        match self {
            Self::V2 => Some(Self::V3),
            Self::V3 => Some(Self::V5),
            Self::V5 => Some(Self::V6),
            Self::V6 => Some(Self::V61),
            Self::V61 => None,
        }
    }

    /// Parse a documented label, returning a warning for an unknown version.
    #[must_use]
    pub fn parse(label: &str) -> Result<Self, crate::warning::Warning> {
        match label {
            "v2" => Ok(Self::V2),
            "v3" => Ok(Self::V3),
            "v5" => Ok(Self::V5),
            "v6" => Ok(Self::V6),
            "v6.1" => Ok(Self::V61),
            other => Err(crate::warning::Warning::Drift {
                domain: "migration version",
                expected: 5,
                found: other.len() as u32,
            }),
        }
    }
}
