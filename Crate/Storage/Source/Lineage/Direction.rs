//! Traversal direction for a lineage query.

/// Which side of a node a query walks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// Walk from a node toward its parents.
    Ancestors,
    /// Walk from a node toward its children.
    Descendants,
    /// Walk both ways.
    Both,
}

impl Direction {
    /// Whether an edge is relevant to this direction.
    #[must_use]
    pub fn admits(&self, edge: &super::edge::Edge, root: &str) -> bool {
        match self {
            Self::Ancestors => edge.arrives(root),
            Self::Descendants => edge.departs(root),
            Self::Both => edge.arrives(root) || edge.departs(root),
        }
    }

    /// The declared direction name.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Ancestors => "ancestors",
            Self::Descendants => "descendants",
            Self::Both => "both",
        }
    }
}
