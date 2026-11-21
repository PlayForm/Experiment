//! The institution graph as typed nodes and declared flows.
//!
//! Grounded in `SYSTEM.md:32-65` and `graphs/research-economy.mmd:1-27`: the
//! owner funds and constitutes, the coordinator and treasury feed the exchange,
//! researchers feed the claim registry, the patcher/oracle/builder/benchmark/
//! evaluator chain feeds evidence back to claims, the publication custodian
//! alone feeds the library, and the integrator loops back to the exchange. Raw
//! private notebooks never connect directly to the library, evaluator or
//! siblings (`:67-70`). Only declared flows are permitted.

use crate::Permission;

/// One institution node of the v5 research economy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Graph {
    /// Owner and constitution (`SYSTEM.md:36`, `mmd:2`).
    Owner,
    /// Treasury and policy (`SYSTEM.md:36`, `mmd:2`).
    Treasury,
    /// Offline PKI and online scoped issuers (`SYSTEM.md:37`, `mmd:3`).
    Identity,
    /// Deterministic coordinator and event ledger (`SYSTEM.md:38`, `mmd:2`).
    Coordinator,
    /// Research exchange with exact-duplicate gate (`SYSTEM.md:39`, `mmd:4`).
    Exchange,
    /// Private researcher in any stance (`SYSTEM.md:42-45`, `mmd:6-9`).
    Researcher,
    /// Signed claim registry (`SYSTEM.md:46-49`, `mmd:10-13`).
    ClaimRegistry,
    /// Patcher in an isolated worktree (`SYSTEM.md:50`, `mmd:14`).
    Patcher,
    /// Novelty and correctness oracle (`SYSTEM.md:51`, `mmd:15`).
    Oracle,
    /// Builder of immutable images and binaries (`SYSTEM.md:52`, `mmd:16`).
    Builder,
    /// Exclusive benchmark and sealed-holdout custodian (`SYSTEM.md:53`, `mmd:16`).
    Benchmark,
    /// Independent statistical evaluator (`SYSTEM.md:54`, `mmd:17`).
    Evaluator,
    /// Sanitizer, scientific editor and publication custodian (`SYSTEM.md:56`).
    Publisher,
    /// Signed corpus of internal and public releases (`SYSTEM.md:58`, `mmd:21`).
    Library,
    /// Transfer and integration researcher (`SYSTEM.md:59`, `mmd:22`).
    Integrator,
    /// Control and research OpenTelemetry planes (`SYSTEM.md:61`, `mmd:24`).
    Telemetry,
    /// Isolated product measurement telemetry (`SYSTEM.md:62`, `mmd:25`).
    ProductTelemetry,
    /// Rust GPUI research cockpit (`SYSTEM.md:63`, `mmd:26`).
    Cockpit,
}

impl Graph {
    /// All declared institution nodes in graph order.
    pub const ALL: [Self; 18] = [
        Self::Owner,
        Self::Treasury,
        Self::Identity,
        Self::Coordinator,
        Self::Exchange,
        Self::Researcher,
        Self::ClaimRegistry,
        Self::Patcher,
        Self::Oracle,
        Self::Builder,
        Self::Benchmark,
        Self::Evaluator,
        Self::Publisher,
        Self::Library,
        Self::Integrator,
        Self::Telemetry,
        Self::ProductTelemetry,
        Self::Cockpit,
    ];

    /// A stable label for this node.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Owner => "Owner + Constitution",
            Self::Treasury => "Treasury + Policy",
            Self::Identity => "CA + Identity + Encryption",
            Self::Coordinator => "Deterministic coordinator",
            Self::Exchange => "Research exchange",
            Self::Researcher => "Private researcher",
            Self::ClaimRegistry => "Signed claim registry",
            Self::Patcher => "Scoped patcher",
            Self::Oracle => "Verifier and oracle",
            Self::Builder => "Builder",
            Self::Benchmark => "Exclusive benchmark",
            Self::Evaluator => "Statistical evaluator",
            Self::Publisher => "Publication custodian",
            Self::Library => "Signed paper corpus",
            Self::Integrator => "Transfer researcher",
            Self::Telemetry => "Control and research telemetry",
            Self::ProductTelemetry => "Product measurement telemetry",
            Self::Cockpit => "GPUI observatory",
        }
    }

    /// Whether `other` is a declared outgoing flow from this node.
    ///
    /// Exception declared: this is a structural relation between two values,
    /// so it is a `match` over the node pair rather than a branch at a call
    /// site (`SYSTEM.md:34-65`, `graphs/research-economy.mmd:2-26`).
    #[must_use]
    pub const fn may_flow_to(self, other: Self) -> Permission {
        use self::Graph as G;
        use crate::Permission as P;
        match (self, other) {
            (G::Owner, G::Treasury | G::Identity | G::Coordinator)
            | (G::Coordinator, G::Exchange | G::Telemetry)
            | (G::Treasury, G::Exchange)
            | (G::Identity, G::Exchange)
            | (G::Exchange, G::Researcher)
            | (G::Researcher, G::ClaimRegistry)
            | (G::ClaimRegistry, G::Patcher | G::Publisher)
            | (G::Patcher, G::Oracle)
            | (G::Oracle, G::Builder)
            | (G::Builder, G::Benchmark)
            | (G::Benchmark, G::Evaluator | G::Telemetry | G::ProductTelemetry)
            | (G::Evaluator, G::ClaimRegistry | G::Publisher)
            | (G::Publisher, G::Library)
            | (G::Library, G::Integrator)
            | (G::Integrator, G::Exchange)
            | (G::Telemetry, G::Cockpit)
            | (G::ProductTelemetry, G::Cockpit) => P::Allow,
            _ => P::Deny,
        }
    }

    /// Raw notebook edges are never declared to the public library, the
    /// evaluator or a sibling researcher (`SYSTEM.md:67-70`).
    #[must_use]
    pub const fn notebook_to(self, other: Self) -> Permission {
        use self::Graph as G;
        match (self, other) {
            (G::Researcher, G::Library | G::Evaluator | G::Researcher) => Permission::Deny,
            _ => Permission::Deny,
        }
    }
}
