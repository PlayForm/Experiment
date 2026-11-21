//! Source ancestry and merge provenance.
//! Grounded in `ARCHITECTURE.md:131-137` (a linear causal chain, fanout DAG,
//! and a merge that is a new integration candidate requiring entirely new
//! evidence) and `RESEARCH_GATE.md:17-23` (source ancestry is part of the
//! canonical fingerprint).

/// The immutable ancestry of a candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ancestry {
    /// Direct parent mutation id; absent for the frozen root.
    pub parent: Option<String>,
    /// All ancestors when this is an integration merge.
    pub merge_of: Vec<String>,
}

impl Ancestry {
    /// Root ancestry at a pinned baseline.
    #[must_use]
    pub fn root() -> Self {
        Self {
            parent: None,
            merge_of: Vec::new(),
        }
    }

    /// Child ancestry from one named parent.
    #[must_use]
    pub fn child(parent: &str) -> Self {
        Self {
            parent: Some(parent.to_owned()),
            merge_of: Vec::new(),
        }
    }

    /// Whether another candidate shares this direct parent.
    #[must_use]
    pub fn is_sibling_of(&self, other: &Self) -> bool {
        match (&self.parent, &other.parent) {
            (Some(left), Some(right)) => left == right,
            _ => false,
        }
    }

    /// A merge of two ancestors is a new candidate, never a sum of speedups.
    #[must_use]
    pub fn requires_new_evidence(&self) -> bool {
        self.merge_of.len() > 1
    }
}
