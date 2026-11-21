//! The append-only ancestry DAG over mutations.
//!
//! Grounded in `sql/lineage.sql:35-58` (mutations and the `mutation_parents`
//! many-to-many table with `purpose IN ('causal','merge')`),
//! `REQUIREMENTS.md:736-750` (REQ-LIN-01, REQ-LIN-08) and `:769-772` (each
//! causal lane is linear; forks and joins are recorded in an append-only DAG).
//! Edges are typed records; no graph is written to a database.
//!
//! Exception declared: `Edge` is declared beside `Ancestry` because it is the
//! edge of this one graph and has no independent lifecycle.

use std::collections::BTreeMap;

use crate::{lineage::Purpose, Warning};

/// One parent claim between two mutations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edge {
    pub child: String,
    pub parent: String,
    pub purpose: Purpose,
}

/// An append-only ancestry graph.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ancestry {
    pub edges: Vec<Edge>,
}

impl Ancestry {
    /// Append an edge. Duplicate claims are folded by keeping the first.
    pub fn add(&mut self, child: &str, parent: &str, purpose: Purpose) {
        let present = self
            .edges
            .iter()
            .any(|edge| edge.child == child && edge.parent == parent && edge.purpose == purpose);
        match present {
            true => {}
            false => self.edges.push(Edge {
                child: child.to_owned(),
                parent: parent.to_owned(),
                purpose,
            }),
        }
    }

    /// Parents of a mutation, by purpose.
    #[must_use]
    pub fn parents_of(&self, child: &str, purpose: Purpose) -> Vec<&str> {
        self.edges
            .iter()
            .filter(|edge| edge.child == child && edge.purpose == purpose)
            .map(|edge| edge.parent.as_str())
            .collect()
    }

    /// Whether a path of parent edges leads from `start` to `target`.
    /// Bounded by `depth` so a malformed graph cannot recurse forever.
    fn reaches(
        start: &str,
        target: &str,
        parents: &BTreeMap<&str, Vec<&str>>,
        depth: usize,
    ) -> bool {
        match depth {
            0 => false,
            _ => parents.get(start).is_some_and(|list| {
                list.iter()
                    .any(|parent| *parent == target || Self::reaches(parent, target, parents, depth - 1))
            }),
        }
    }

    /// Warn on cycles and on causal edges with more than one parent
    /// (REQ-LIN-01, REQ-LIN-08). Nothing is mutated by validation.
    pub fn validate(&self, warnings: &mut Vec<Warning>) {
        let parents: BTreeMap<&str, Vec<&str>> =
            self.edges.iter().fold(BTreeMap::new(), |mut map, edge| {
                map.entry(edge.parent.as_str())
                    .or_default()
                    .push(edge.child.as_str());
                map
            });
        self.edges.iter().for_each(|edge| {
            match Self::reaches(&edge.parent, &edge.child, &parents, self.edges.len()) {
                true => warnings.push(Warning::Collision {
                    domain: "ancestry.cycle",
                    id: edge.child.clone(),
                }),
                false => {}
            }
        });
        let causal_counts = self
            .edges
            .iter()
            .filter(|edge| edge.purpose == Purpose::Causal)
            .fold(BTreeMap::<&str, usize>::new(), |mut tally, edge| {
                *tally.entry(edge.child.as_str()).or_default() += 1;
                tally
            });
        causal_counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .for_each(|(child, _)| warnings.push(Warning::Bound {
                domain: "ancestry.causal",
                key: format!("{child} has more than one causal parent"),
            }));
    }
}
