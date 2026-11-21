//! The full benchmark mesh across workloads and hardware classes.
//!
//! Grounded in `docs/10-benchmark-mesh.md:1-16,41-49`,
//! `docs/00-system-constitution.md:5,14-15` and `docs/02:14,16` (paired,
//! randomized, same-reservation measurement). The mesh retains separate
//! hardware classes, declared cache regimes and the full Pareto frontier;
//! it never collapses to one global score or a single best fixture. Building,
//! running and signing samples are deliberately outside this model.

use crate::{
    mesh::{Candidate, Node, Pareto, Workload},
    Permission,
};

/// The declared cohort matrix of a laboratory campaign.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BenchmarkMesh {
    /// Declared workloads, each with a frozen fixture digest.
    pub workloads: Vec<Workload>,
    /// Physical hosts grouped by hardware class.
    pub nodes: Vec<Node>,
    /// Preserved multi-objective frontier.
    pub pareto: Pareto,
}

impl BenchmarkMesh {
    /// Create an empty mesh.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            workloads: Vec::new(),
            nodes: Vec::new(),
            pareto: Pareto::new(),
        }
    }

    /// Add a workload, refusing a duplicate identifier with a warning.
    #[must_use]
    pub fn add_workload(&mut self, workload: Workload) -> Result<Permission, crate::Warning> {
        let duplicate = self.workloads.iter().any(|item| item.id == workload.id);
        match duplicate {
            true => Err(crate::Warning::Duplicate {
                domain: "workload",
                id: workload.id,
            }),
            false => {
                self.workloads.push(workload);
                Ok(Permission::Allow)
            }
        }
    }

    /// Add a node, refusing one that cannot provide authority-grade isolation.
    #[must_use]
    pub fn add_node(&mut self, node: Node) -> Result<Permission, crate::Warning> {
        match node.authoritative() {
            Permission::Allow => {
                self.nodes.push(node);
                Ok(Permission::Allow)
            }
            Permission::Deny => Err(crate::Warning::Contaminated {
                reason: format!("node {:?} lacks exclusive physical isolation", node.id),
            }),
        }
    }

    /// Admit a candidate measurement onto the preserved frontier.
    #[must_use]
    pub fn admit(&mut self, candidate: Candidate) -> Result<Permission, crate::Warning> {
        self.pareto.admit(candidate)
    }

    /// Whether a claim about this mesh may rank one hardware class above
    /// another. Cross-class comparison is never automatic
    /// (`docs/13-repository-agnostic.md:40-45`).
    #[must_use]
    pub const fn cross_class_claim() -> Permission {
        Permission::Deny
    }
}
