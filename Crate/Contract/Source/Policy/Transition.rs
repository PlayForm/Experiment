//! One named state-machine transition.
//!
//! Every transition in `policy/state-machines.yaml` and
//! `policy/v61-state-machines.yaml` is a `(from, to, guard)` triple. A guard is
//! an authenticated service or attested condition; no wall-clock reading
//! enters the transition (`docs/06-state-machines.md:78-90`).

/// One guarded edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Transition {
    /// Source state.
    pub from: &'static str,
    /// Destination state.
    pub to: &'static str,
    /// Authenticated guard or attested authority.
    pub guard: &'static str,
}

impl Transition {
    /// Build one transition.
    #[must_use]
    pub const fn of(from: &'static str, to: &'static str, guard: &'static str) -> Self {
        Self { from, to, guard }
    }

    /// Whether this edge matches a claimed pair.
    #[must_use]
    pub fn matches(&self, from: &str, to: &str) -> bool {
        self.from == from && self.to == to
    }
}
