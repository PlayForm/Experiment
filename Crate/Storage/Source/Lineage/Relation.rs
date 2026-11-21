//! Every typed relation recordable between two lineage nodes.

/// A relation used by mutation ancestry, claim links and superseding facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    /// Causal mutation parent.
    Causal,
    /// Merge mutation parent.
    Merge,
    /// A claim supports another.
    Supports,
    /// A claim contradicts another.
    Contradicts,
    /// A claim refines another.
    Refines,
    /// A claim replicates another.
    Replicates,
    /// A claim cites another.
    Cites,
    /// A later fact supersedes an earlier one.
    Supersedes,
}

impl Relation {
    /// The declared relation value.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Causal => "causal",
            Self::Merge => "merge",
            Self::Supports => "supports",
            Self::Contradicts => "contradicts",
            Self::Refines => "refines",
            Self::Replicates => "replicates",
            Self::Cites => "cites",
            Self::Supersedes => "supersedes",
        }
    }

    /// Whether this relation is a superseding amendment rather than a link.
    #[must_use]
    pub fn amends(&self) -> crate::permission::Permission {
        match self {
            Self::Supersedes => crate::permission::Permission::Allow,
            _ => crate::permission::Permission::Deny,
        }
    }
}
