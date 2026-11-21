//! The 24 numbered sections of `REQUIREMENTS.md` as typed values.
//!
//! Section numbers and titles are transcribed from the document's own
//! headings (`:14`, `:43`, `:105`, `:152`, `:194`, `:280`, `:336`, `:376`,
//! `:426`, `:453`, `:522`, `:556`, `:627`, `:673`, `:718`, `:774`, `:832`,
//! `:860`, `:894`, `:929`, `:1001`, `:1034`, `:1055`, `:1120`).

/// One numbered section of the requirements bundle.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Section {
    Intent,
    Law,
    Goal,
    Arc,
    Heuristic,
    Mutation,
    Identity,
    Workload,
    Oracle,
    Bench,
    Objective,
    Agent,
    Docker,
    Hardware,
    Lineage,
    Interface,
    Cost,
    Feature,
    Security,
    Gate,
    Research,
    Report,
    Decomposition,
    Reference,
}

impl Section {
    /// All 24 sections in document order.
    pub const ALL: [Self; 24] = [
        Self::Intent,
        Self::Law,
        Self::Goal,
        Self::Arc,
        Self::Heuristic,
        Self::Mutation,
        Self::Identity,
        Self::Workload,
        Self::Oracle,
        Self::Bench,
        Self::Objective,
        Self::Agent,
        Self::Docker,
        Self::Hardware,
        Self::Lineage,
        Self::Interface,
        Self::Cost,
        Self::Feature,
        Self::Security,
        Self::Gate,
        Self::Research,
        Self::Report,
        Self::Decomposition,
        Self::Reference,
    ];

    /// The document's section number.
    #[must_use]
    pub const fn number(self) -> u8 {
        match self {
            Self::Intent => 0,
            Self::Law => 1,
            Self::Goal => 2,
            Self::Arc => 3,
            Self::Heuristic => 4,
            Self::Mutation => 5,
            Self::Identity => 6,
            Self::Workload => 7,
            Self::Oracle => 8,
            Self::Bench => 9,
            Self::Objective => 10,
            Self::Agent => 11,
            Self::Docker => 12,
            Self::Hardware => 13,
            Self::Lineage => 14,
            Self::Interface => 15,
            Self::Cost => 16,
            Self::Feature => 17,
            Self::Security => 18,
            Self::Gate => 19,
            Self::Research => 20,
            Self::Report => 21,
            Self::Decomposition => 22,
            Self::Reference => 23,
        }
    }

    /// Recover a section from its number; unknown numbers are `None`.
    #[must_use]
    pub fn from_number(number: u8) -> Option<Self> {
        Self::ALL.into_iter().find(|section| section.number() == number)
    }

    /// The heading text without its number.
    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::Intent => "Executive intent and terminology",
            Self::Law => "Design laws (non-negotiable)",
            Self::Goal => "Product-level goals, non-goals, and performance philosophy",
            Self::Arc => "Decompose the executable into replaceable stages",
            Self::Heuristic => "Heuristic analysis as a feature family, not a hardcoded switch",
            Self::Mutation => "Algorithms and mutations to investigate",
            Self::Identity => "Formal experiment identity and fixed dimensions",
            Self::Workload => "Benchmark matrix and workload taxonomy",
            Self::Oracle => "Correctness and equivalence oracles",
            Self::Bench => "Measurement protocol and statistics",
            Self::Objective => "Optimization objectives and search strategies",
            Self::Agent => "Agents: autonomous yet controlled seven-day experimentation",
            Self::Docker => "Docker and local architecture",
            Self::Hardware => "Hardware fingerprinting and specialization",
            Self::Lineage => "Artifact storage, database, and lineage model",
            Self::Interface => "Interfaces and single-command automation",
            Self::Cost => "Runtime planner cost model",
            Self::Feature => "Experimental feature flag model",
            Self::Security => "Security, permissions, and unattended operation",
            Self::Gate => "Progression and acceptance gates",
            Self::Research => "Threats to validity and unresolved research questions",
            Self::Report => "Reporting, discoverability, and productization",
            Self::Decomposition => "Bare-bones code decomposition, disassembly, and change-level observability",
            Self::Reference => "Source references and further reading",
        }
    }
}
