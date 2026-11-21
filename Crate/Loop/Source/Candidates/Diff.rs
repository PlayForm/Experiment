//! A typed diff, represented as data rather than applied.
//! Grounded in `RESEARCH_GATE.md:17-23` (canonical structural similarity
//! normalizes formatting and imports while preserving semantics) and in
//! `AGENT_CONTRACT.md:26-28` (one conceptual change, no sneaked test changes).

/// A diff as a list of typed hunk lines.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Diff {
    /// Prefixed hunk lines (`+`, `-`, ` `).
    pub hunks: Vec<String>,
}

impl Diff {
    /// An empty diff, before a worker submits a patch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of added lines.
    #[must_use]
    pub fn additions(&self) -> usize {
        self.hunks.iter().filter(|line| line.starts_with('+')).count()
    }

    /// Whether the diff changes anything.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.hunks.is_empty()
    }

    /// Whether the diff is a single conceptual change.
    #[must_use]
    pub fn is_atomic(&self) -> bool {
        !self.is_empty() && self.additions() < 4096
    }
}
