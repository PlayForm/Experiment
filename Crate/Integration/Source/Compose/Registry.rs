//! The eleven crates merged by this card, in dependency order.
//!
//! Grounded in the root `Cargo.toml` member list and in the research chain
//! `Research/prun_lab/README.md` -> `prun_optimization_v2` -> v3 -> v4 -> v5
//! -> `prun_laboratory_v6_1` -> `prun_laboratory_v62`. Each variant names the
//! real package and the real public type that anchors it; these are typed
//! names, never re-implementations of the owning crate's behaviour.

/// A crate in the composed chain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Registry {
    Benchmark,
    Runner,
    Campaign,
    Experiment,
    Trust,
    Market,
    Laboratory,
    Contract,
    Storage,
    Custody,
    Fabric,
}

impl Registry {
    /// The root workspace order: v0 through v6.2.
    pub const ALL: [Self; 11] = [
        Self::Benchmark,
        Self::Runner,
        Self::Campaign,
        Self::Experiment,
        Self::Trust,
        Self::Market,
        Self::Laboratory,
        Self::Contract,
        Self::Storage,
        Self::Custody,
        Self::Fabric,
    ];

    /// The Cargo package name declared by each crate's manifest.
    #[must_use]
    pub const fn package(self) -> &'static str {
        match self {
            Self::Benchmark => "benchmark",
            Self::Runner => "runner",
            Self::Campaign => "campaign",
            Self::Experiment => "experiment",
            Self::Trust => "trust",
            Self::Market => "market",
            Self::Laboratory => "laboratory",
            Self::Contract => "contract",
            Self::Storage => "storage",
            Self::Custody => "custody",
            Self::Fabric => "fabric",
        }
    }

    /// One real public type that anchors each crate's model, by path.
    #[must_use]
    pub const fn model(self) -> &'static str {
        match self {
            Self::Benchmark => "benchmark::config::Manifest",
            Self::Runner => "runner::cli::argument::Argument",
            Self::Campaign => "campaign::campaign::Campaign",
            Self::Experiment => "experiment::evaluate::verdict::Verdict",
            Self::Trust => "trust::bridge::signature::Signature",
            Self::Market => "market::claim::claim::Claim",
            Self::Laboratory => "laboratory::constitution::invariant::Invariant",
            Self::Contract => "contract::catalog::Catalog",
            Self::Storage => "storage::migration::chain::Chain",
            Self::Custody => "custody::custody::receipt::Receipt",
            Self::Fabric => "fabric::identifier::id::Id",
        }
    }

    /// Resolve a package name to its registry variant. `None` is the typed
    /// "unknown crate" signal a caller renders as a `Failure::UnknownCrate`.
    #[must_use]
    pub fn find(name: &str) -> std::option::Option<Self> {
        Self::ALL
            .into_iter()
            .find(|item| item.package() == name)
    }
}
