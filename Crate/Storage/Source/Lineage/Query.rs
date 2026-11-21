//! A bounded ancestry query over typed edges.

use super::direction::Direction;
use super::edge::Edge;

/// A query rooted at one node with a direction and optional depth bound.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query {
    /// Root node identifier.
    pub root: String,
    /// Traversal direction.
    pub direction: Direction,
    /// Maximum depth, unlimited when absent.
    pub depth: Option<usize>,
}

impl Query {
    /// Declare a query.
    #[must_use]
    pub fn of(root: &str, direction: Direction, depth: Option<usize>) -> Self {
        Self {
            root: root.to_owned(),
            direction,
            depth,
        }
    }

    /// The node reached from `root` along an edge admitted by this query.
    #[must_use]
    pub fn step(&self, edge: &Edge) -> Option<String> {
        match self.direction {
            Direction::Ancestors => edge
                .arrives(&self.root)
                .then(|| edge.parent.id.clone()),
            Direction::Descendants => edge
                .departs(&self.root)
                .then(|| edge.child.id.clone()),
            Direction::Both => edge
                .arrives(&self.root)
                .then(|| edge.parent.id.clone())
                .or_else(|| edge.departs(&self.root).then(|| edge.child.id.clone())),
        }
    }

    /// Describe the query.
    #[must_use]
    pub fn describe(&self) -> String {
        self.depth.map_or_else(
            || format!("{} {} unlimited", self.root, self.direction.label()),
            |depth| format!("{} {} depth={depth}", self.root, self.direction.label()),
        )
    }
}
