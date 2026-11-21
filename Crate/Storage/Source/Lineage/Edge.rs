//! A directed parent-to-child relation in the lineage graph.

use super::child::Child;
use super::parent::Parent;
use super::purpose::Purpose;
use super::relation::Relation;

/// One edge of the ancestry or claim graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edge {
    /// Origin endpoint.
    pub parent: Parent,
    /// Destination endpoint.
    pub child: Child,
    /// Declared relation.
    pub relation: Relation,
}

impl Edge {
    /// Declare an edge.
    #[must_use]
    pub fn of(parent: Parent, child: Child, relation: Relation) -> Self {
        Self {
            parent,
            child,
            relation,
        }
    }

    /// Declare a mutation-parent edge from its purpose.
    #[must_use]
    pub fn mutation(parent: &str, child: &str, purpose: Purpose) -> Self {
        let relation = match purpose {
            Purpose::Causal => Relation::Causal,
            Purpose::Merge => Relation::Merge,
        };
        Self::of(Parent::of(parent, purpose), Child::of(child), relation)
    }

    /// Whether this edge departs from a named node.
    #[must_use]
    pub fn departs(&self, id: &str) -> bool {
        self.parent.id == id
    }

    /// Whether this edge arrives at a named node.
    #[must_use]
    pub fn arrives(&self, id: &str) -> bool {
        self.child.id == id
    }
}
