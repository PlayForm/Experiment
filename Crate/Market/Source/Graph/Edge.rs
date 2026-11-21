//! Declared edges of the v5 research-economy graph.
//!
//! Grounded in `graphs/research-economy.mmd:2-26`: every arrow in the source
//! is one typed edge. An undeclared edge is a structural violation, not a
//! runtime event; this type only reports whether the graph knows the arrow.

use super::Node;

/// One declared arrow in the research-economy graph.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Edge {
    /// Source node (`mmd:2-26`).
    pub from: Node,
    /// Destination node (`mmd:2-26`).
    pub to: Node,
}

impl Edge {
    /// Every declared edge, in `mmd` line order.
    pub const DECLARED: [Self; 25] = [
        Self { from: Node::Owner, to: Node::Treasury },
        Self { from: Node::Owner, to: Node::Identity },
        Self { from: Node::Treasury, to: Node::Exchange },
        Self { from: Node::Identity, to: Node::Exchange },
        Self { from: Node::Exchange, to: Node::Proponent },
        Self { from: Node::Exchange, to: Node::Skeptic },
        Self { from: Node::Exchange, to: Node::Rival },
        Self { from: Node::Exchange, to: Node::Replicator },
        Self { from: Node::Proponent, to: Node::Claims },
        Self { from: Node::Skeptic, to: Node::Claims },
        Self { from: Node::Rival, to: Node::Claims },
        Self { from: Node::Replicator, to: Node::Claims },
        Self { from: Node::Claims, to: Node::Patcher },
        Self { from: Node::Patcher, to: Node::Verifier },
        Self { from: Node::Verifier, to: Node::Builders },
        Self { from: Node::Evaluator, to: Node::Claims },
        Self { from: Node::Claims, to: Node::Publication },
        Self { from: Node::Evaluator, to: Node::Publication },
        Self { from: Node::Publication, to: Node::Corpus },
        Self { from: Node::Corpus, to: Node::Integrator },
        Self { from: Node::Integrator, to: Node::Exchange },
        Self { from: Node::Treasury, to: Node::ObservabilityControl },
        Self { from: Node::Builders, to: Node::ObservabilityProduct },
        Self { from: Node::ObservabilityControl, to: Node::Observatory },
        Self { from: Node::ObservabilityProduct, to: Node::Observatory },
    ];

    /// Whether this arrow is declared in the source graph.
    #[must_use]
    pub const fn is_declared(&self) -> bool {
        Self::DECLARED
            .iter()
            .any(|declared| declared.from == self.from && declared.to == self.to)
    }

    /// A graph edge never carries private notebook content
    /// (`SYSTEM.md:67-70`, `TELEMETRY.md:7-13`).
    #[must_use]
    pub const fn may_carry_plaintext() -> crate::Permission {
        crate::Permission::Deny
    }
}
