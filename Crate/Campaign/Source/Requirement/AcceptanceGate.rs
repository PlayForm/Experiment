//! The eight progression gates as typed values with verification kinds.
//!
//! Grounded in `REQUIREMENTS.md:929-972` (Phases 0-7 and their gates) and the
//! acceptance scenarios at `:974-999`. A gate names what must be shown; this
//! crate never claims a gate passed.
//!
//! Exception declared: `Kind` is declared beside `Gate` because it is the
//! verification vocabulary of this one type and has no independent lifecycle.

/// How a gate's condition is verified.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Kind {
    /// Two repeated windows agree within a configured noise bound.
    Baseline,
    /// A refactor is compared against reference behaviour.
    Refactor,
    /// An edit → build → oracle → benchmark → verdict loop survives restart.
    Automation,
    /// An ablation or holdout comparison establishes value.
    Ablation,
    /// Fault-injection scenarios recover cleanly.
    FaultInjection,
    /// Fresh paired baselines confirm a winner.
    Search,
    /// Portability and ISA checks on a target cohort.
    Specialization,
    /// Reproducible evidence and a rollback release exist.
    Release,
}

impl Kind {
    /// Token for status output.
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Refactor => "refactor",
            Self::Automation => "automation",
            Self::Ablation => "ablation",
            Self::FaultInjection => "fault_injection",
            Self::Search => "search",
            Self::Specialization => "specialization",
            Self::Release => "release",
        }
    }
}

/// One phase gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Gate {
    pub phase: u8,
    pub name: &'static str,
    pub condition: &'static str,
    pub kind: Kind,
}

impl Gate {
    /// All eight gates in progression order (`REQUIREMENTS.md:931-972`).
    pub const ALL: [Self; 8] = [
        Self {
            phase: 0,
            name: "Evidence baseline",
            condition: "two repeated baseline windows agree within configured noise; no missing required oracle",
            kind: Kind::Baseline,
        },
        Self {
            phase: 1,
            name: "Characterize and decompose",
            condition: "refactor matches reference behavior; no unacceptable regression; baseline build available",
            kind: Kind::Refactor,
        },
        Self {
            phase: 2,
            name: "Mutation lab",
            condition: "edit -> build -> oracle -> benchmark -> verdict works locally and survives forced termination",
            kind: Kind::Automation,
        },
        Self {
            phase: 3,
            name: "Heuristic feature family",
            condition: "zero semantic regressions; positive total-runtime value on holdout; planning overhead under threshold",
            kind: Kind::Ablation,
        },
        Self {
            phase: 4,
            name: "Persistent autonomy",
            condition: "agent crash, runner crash, DB lock, disk and host restart recover with no orphaned mutation or lost sample",
            kind: Kind::FaultInjection,
        },
        Self {
            phase: 5,
            name: "Search and combination",
            condition: "winners confirmed on fresh workload data and paired baselines",
            kind: Kind::Search,
        },
        Self {
            phase: 6,
            name: "Hardware specialization",
            condition: "portable baseline works everywhere supported; specializations never select unsupported instructions",
            kind: Kind::Specialization,
        },
        Self {
            phase: 7,
            name: "Production release",
            condition: "reproducible evidence for each speedup; sandbox review; rollback release ready",
            kind: Kind::Release,
        },
    ];

    /// Recover a gate by phase number.
    #[must_use]
    pub fn from_phase(phase: u8) -> Option<Self> {
        Self::ALL.into_iter().find(|gate| gate.phase == phase)
    }

    /// Render the gate for a progression report.
    #[must_use]
    pub fn line(&self) -> String {
        format!("phase {}: {} [{}] - {}", self.phase, self.name, self.kind.key(), self.condition)
    }
}
