//! Nodes of the v5 research-economy graph.
//!
//! Grounded in `graphs/research-economy.mmd:2-26` and the fuller institution
//! graph in `architecture/SYSTEM.md:34-65`. Every node is a typed value; no
//! node runs a process here.

use crate::Permission;

/// One node in the research-economy graph.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Node {
    /// Owner plus constitution (`mmd:2`).
    Owner,
    /// Treasury plus policy (`mmd:2`).
    Treasury,
    /// CA, identity and encryption (`mmd:3`).
    Identity,
    /// Research exchange (`mmd:4`).
    Exchange,
    /// Proponent researcher (`mmd:6`).
    Proponent,
    /// Skeptic researcher (`mmd:7`).
    Skeptic,
    /// Independent rival researcher (`mmd:8`).
    Rival,
    /// Replication designer (`mmd:9`).
    Replicator,
    /// Signed hypothesis and claims (`mmd:10-13`).
    Claims,
    /// Scoped patcher (`mmd:14`).
    Patcher,
    /// Independent verifier and oracle (`mmd:15`).
    Verifier,
    /// Builders and isolated benchmarks (`mmd:16`).
    Builders,
    /// Statistical evaluator and evidence ledger (`mmd:17`).
    Evaluator,
    /// Authorized declassification and publication (`mmd:19-20`).
    Publication,
    /// Signed paper or knowledge corpus (`mmd:21`).
    Corpus,
    /// Independent transfer researcher (`mmd:22`).
    Integrator,
    /// OTel control and research planes (`mmd:24`).
    ObservabilityControl,
    /// OTel product measurement plane (`mmd:25`).
    ObservabilityProduct,
    /// Rust GPUI observatory (`mmd:26`).
    Observatory,
}

impl Node {
    /// All graph nodes in `mmd` declaration order.
    pub const ALL: [Self; 19] = [
        Self::Owner,
        Self::Treasury,
        Self::Identity,
        Self::Exchange,
        Self::Proponent,
        Self::Skeptic,
        Self::Rival,
        Self::Replicator,
        Self::Claims,
        Self::Patcher,
        Self::Verifier,
        Self::Builders,
        Self::Evaluator,
        Self::Publication,
        Self::Corpus,
        Self::Integrator,
        Self::ObservabilityControl,
        Self::ObservabilityProduct,
        Self::Observatory,
    ];

    /// Stable label from the Mermaid source.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Owner => "Owner + Constitution",
            Self::Treasury => "Treasury + Policy",
            Self::Identity => "CA + Identity + Encryption",
            Self::Exchange => "Research Exchange",
            Self::Proponent => "Proponent (private)",
            Self::Skeptic => "Skeptic (private)",
            Self::Rival => "Independent rival (private)",
            Self::Replicator => "Replication designer (private)",
            Self::Claims => "Signed hypothesis / claims",
            Self::Patcher => "Scoped Patcher",
            Self::Verifier => "Independent verifier and oracle",
            Self::Builders => "Builders and isolated benchmarks",
            Self::Evaluator => "Statistical evaluator and evidence ledger",
            Self::Publication => "Authorized declassification + publication",
            Self::Corpus => "Signed paper / knowledge corpus",
            Self::Integrator => "Independent transfer researcher",
            Self::ObservabilityControl => "OTel control and research planes",
            Self::ObservabilityProduct => "OTel measurement plane",
            Self::Observatory => "Rust GPUI Observatory",
        }
    }

    /// Whether this node produces measurement-grade evidence
    /// (`SYSTEM.md:93-99`, `mmd:15-17`).
    #[must_use]
    pub const fn produces_evidence(self) -> Permission {
        match self {
            Self::Verifier | Self::Builders | Self::Evaluator => Permission::Allow,
            _ => Permission::Deny,
        }
    }
}
