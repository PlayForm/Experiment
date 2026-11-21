//! A causal line of patches: `B0 -> P1 -> P1a -> P1b`.
//! Grounded in `ARCHITECTURE.md:131-136` (linear causal chain, fanout forms a
//! DAG, a merge is a new integration candidate with both ancestors) and in
//! `autonomous-week.yaml:6` (full lineage, one conceptual source mutation per
//! branch).

/// One step in an immutable patch lineage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Iteration {
    /// The pinned parent commit this step is based on.
    pub parent_commit: String,
    /// The mutation identifier minted for this step.
    pub mutation_id: String,
    /// Depth from the frozen root; zero for B0.
    pub depth: u32,
}

impl Iteration {
    /// The root iteration at a pinned commit.
    #[must_use]
    pub fn root(commit: &str) -> Self {
        Self {
            parent_commit: commit.to_owned(),
            mutation_id: "B0".to_owned(),
            depth: 0,
        }
    }

    /// A new child mutation in a new worktree; the parent is never rewritten.
    #[must_use]
    pub fn child(&self, mutation_id: &str) -> Self {
        Self {
            parent_commit: self.mutation_id.clone(),
            mutation_id: mutation_id.to_owned(),
            depth: self.depth + 1,
        }
    }

    /// Whether this is the frozen root.
    #[must_use]
    pub const fn is_root(&self) -> bool {
        self.depth == 0
    }
}
