//! The fourteen lineage tables in `sql/lineage.sql`.
//!
//! Every `CREATE TABLE IF NOT EXISTS` statement in that file is represented
//! by one variant. The logical evidence DAG is separate from the normalized
//! tables: reports are derived views, never the system of record
//! (`REQUIREMENTS.md:727-729`).

/// A table declared by the illustrative lineage schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Table {
    Campaigns,
    Artifacts,
    Hypotheses,
    Mutations,
    Parents,
    Builds,
    Workloads,
    Environments,
    Experiments,
    Oracle,
    Trials,
    Analyses,
    Decisions,
    Events,
}

impl Table {
    /// All fourteen tables in declaration order (`lineage.sql:6-189`).
    pub const ALL: [Self; 14] = [
        Self::Campaigns,
        Self::Artifacts,
        Self::Hypotheses,
        Self::Mutations,
        Self::Parents,
        Self::Builds,
        Self::Workloads,
        Self::Environments,
        Self::Experiments,
        Self::Oracle,
        Self::Trials,
        Self::Analyses,
        Self::Decisions,
        Self::Events,
    ];

    /// The SQL table name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Campaigns => "campaigns",
            Self::Artifacts => "artifacts",
            Self::Hypotheses => "hypotheses",
            Self::Mutations => "mutations",
            Self::Parents => "mutation_parents",
            Self::Builds => "builds",
            Self::Workloads => "workloads",
            Self::Environments => "environments",
            Self::Experiments => "experiments",
            Self::Oracle => "oracle_results",
            Self::Trials => "trials",
            Self::Analyses => "analyses",
            Self::Decisions => "decisions",
            Self::Events => "events",
        }
    }

    /// Whether the table is append-only by trigger (`lineage.sql:201-213`).
    #[must_use]
    pub const fn append_only(self) -> bool {
        matches!(self, Self::Events)
    }
}
