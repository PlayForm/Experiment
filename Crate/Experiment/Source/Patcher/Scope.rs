//! Five disjoint mutation scopes for the fanout.
//!
//! Grounded in `playbooks/fanout-5.yaml:22-34` (slots 1-5: allocation/matcher,
//! scheduling/scheduler, filesystem/walker, syscall_overhead/process_executor,
//! heuristic_policy/planner), `ARCHITECTURE.md:69-71` (disjoint hypothesis
//! families, no shared conversational history or raw rival patches) and
//! `AGENT_CONTRACTS.md:115-122` (different research constraints, not personas).
//! A patcher touching the same symbol as a peer is an interaction risk, not
//! proof of equivalence (`RESEARCH_GATE.md:49`).

/// One of the five disjoint fanout scopes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Scope {
    /// Slot 1: allocation changes in the matcher.
    Allocation,
    /// Slot 2: scheduling changes in the scheduler.
    Scheduling,
    /// Slot 3: filesystem changes in the walker.
    Filesystem,
    /// Slot 4: syscall overhead changes in the process executor.
    SyscallOverhead,
    /// Slot 5: heuristic policy changes in the planner.
    HeuristicPolicy,
}

impl Scope {
    /// All five slots, in playbook order.
    pub const ALL: [Self; 5] = [
        Self::Allocation,
        Self::Scheduling,
        Self::Filesystem,
        Self::SyscallOverhead,
        Self::HeuristicPolicy,
    ];

    /// The mechanism-family token (`fanout-5.yaml:26-34`).
    #[must_use]
    pub const fn family(self) -> &'static str {
        match self {
            Self::Allocation => "allocation",
            Self::Scheduling => "scheduling",
            Self::Filesystem => "filesystem",
            Self::SyscallOverhead => "syscall_overhead",
            Self::HeuristicPolicy => "heuristic_policy",
        }
    }

    /// The granted source path (`fanout-5.yaml:26-34`).
    #[must_use]
    pub const fn path(self) -> &'static str {
        match self {
            Self::Allocation => "matcher",
            Self::Scheduling => "scheduler",
            Self::Filesystem => "walker",
            Self::SyscallOverhead => "process_executor",
            Self::HeuristicPolicy => "planner",
        }
    }

    /// The fanout slot number (1-5).
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Allocation => 1,
            Self::Scheduling => 2,
            Self::Filesystem => 3,
            Self::SyscallOverhead => 4,
            Self::HeuristicPolicy => 5,
        }
    }

    /// Whether two scopes are disjoint by construction. Distinct slots are
    /// disjoint; the same slot is not.
    #[must_use]
    pub const fn disjoint(self, other: Self) -> bool {
        !matches!(self, other)
    }
}
