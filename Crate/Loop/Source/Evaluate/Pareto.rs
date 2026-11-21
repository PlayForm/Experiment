//! The full preserved Pareto frontier.
//! Grounded in `00-system-constitution.md:13-15` (preserve the full Pareto
//! frontier, do not conflate best-on-one-fixture with globally best) and
//! `18-emergent-evolution.md:9-12` (candidates join a workload/hardware Pareto
//! set; some become specialists or future reference snapshots).

/// A preserved set of non-dominated candidate identities.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Pareto {
    /// Digests of every member of the frontier.
    pub members: Vec<String>,
}

impl Pareto {
    /// Start from an empty frontier.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Preserve a candidate on the frontier; nothing is dropped here.
    #[must_use]
    pub fn preserve(mut self, digest: &str) -> Self {
        self.members.push(digest.to_owned());
        self
    }

    /// Whether a candidate is already preserved.
    #[must_use]
    pub fn contains(&self, digest: &str) -> bool {
        self.members.iter().any(|member| member == digest)
    }

    /// Whether the frontier is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}
