//! In-memory ancestry index; a rebuildable projection of the typed edges.

use super::edge::Edge;
use super::query::Query;

/// A set of lineage edges with typed traversal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ancestry {
    edges: Vec<Edge>,
}

impl Ancestry {
    /// Declare an ancestry index from its edges.
    #[must_use]
    pub fn of(edges: Vec<Edge>) -> Self {
        Self { edges }
    }

    /// All declared edges.
    #[must_use]
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Parents of one node.
    #[must_use]
    pub fn parents(&self, id: &str) -> Vec<&Edge> {
        self.edges.iter().filter(|edge| edge.arrives(id)).collect()
    }

    /// Children of one node.
    #[must_use]
    pub fn children(&self, id: &str) -> Vec<&Edge> {
        self.edges.iter().filter(|edge| edge.departs(id)).collect()
    }

    /// Walk the graph from a query root, preserving first-seen order.
    #[must_use]
    pub fn walk(&self, query: &Query) -> Vec<String> {
        let mut pending = vec![(query.root.clone(), 0usize)];
        let mut seen = vec![query.root.clone()];
        let mut reached = Vec::new();
        while let Some((node, depth)) = pending.pop() {
            let allowed = query.depth.map_or(true, |bound| depth < bound);
            match allowed {
                true => {
                    let local = Query::of(&node, query.direction, query.depth);
                    let next = self
                        .edges
                        .iter()
                        .filter_map(|edge| local.step(edge))
                        .filter(|candidate| !seen.contains(candidate))
                        .map(|candidate| (candidate, depth + 1))
                        .collect::<Vec<_>>();
                    next.into_iter().for_each(|(candidate, next_depth)| {
                        seen.push(candidate.clone());
                        reached.push(candidate.clone());
                        pending.push((candidate, next_depth));
                    });
                }
                false => {}
            }
        }
        reached
    }

    /// Report edges that claim a node as its own parent.
    #[must_use]
    pub fn cycles(&self) -> Vec<crate::warning::Warning> {
        self.edges
            .iter()
            .filter(|edge| edge.parent.id == edge.child.id)
            .map(|edge| crate::warning::Warning::Collision {
                domain: "lineage edge",
                id: edge.parent.id.clone(),
            })
            .collect()
    }
}
