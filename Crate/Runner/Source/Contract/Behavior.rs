//! The five axes a characterization run compares.
//!
//! Names and scope come from the card's `Contract/` brief; the concrete
//! rules on each axis are grounded in the cited SUT files on `Invariant`.

/// One observable axis of the system under test.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Behavior {
    /// Which entries become target directories.
    MatchSet,
    /// Which paths the walk rejects before matching.
    Exclusion,
    /// Event order and stdout/stderr routing.
    OutputOrdering,
    /// How spawn, lock and exit failures surface.
    ErrorPropagation,
    /// The `-F` file-versus-directory selection.
    FileMode,
}

impl Behavior {
    /// All five axes, in contract order.
    pub const ALL: [Self; 5] = [
        Self::MatchSet,
        Self::Exclusion,
        Self::OutputOrdering,
        Self::ErrorPropagation,
        Self::FileMode,
    ];

    /// One-line meaning of the axis.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::MatchSet => "entries whose file_name equals Pattern yield their parent",
            Self::Exclusion => "globset rejects excluded paths before the pattern match",
            Self::OutputOrdering => "events are ordered; stdout and stderr are routed apart",
            Self::ErrorPropagation => "spawn, lock timeout and non-zero exit produce typed faults",
            Self::FileMode => "-F selects files, otherwise directories containing the pattern",
        }
    }
}
