//! Source ancestry: frozen baseline, direct parent and integration DAG.
//!
//! Grounded in `ARCHITECTURE.md:132-136` (a lineage is a linear causal chain;
//! a fanout forms a DAG; a merge is a new integration candidate with both
//! ancestors, a new binary and entirely new evidence; do not inherit summed
//! speedups) and `REQUIREMENTS_V3.md` INV-004/AGT3-007. Every revision is a new
//! child mutation; earlier patches and tests stay immutable.
//!
//! Exception declared: `Lineage` is the typed form of the ancestry kind and
//! has no independent lifecycle outside `Ancestry`.

/// Whether an ancestry is a single chain or a multi-parent integration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lineage {
    /// One causal chain from the frozen baseline.
    Linear,
    /// A merge of two or more parents; all evidence is measured anew.
    Integration,
}

/// The causal ancestry of one candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ancestry {
    /// The frozen absolute baseline (B0), always present.
    pub baseline: String,
    /// The direct parent this candidate revises.
    pub parent: String,
    /// All ancestors, for DAG reconstruction.
    pub ancestors: Vec<String>,
    /// Whether this is a linear revision or an integration merge.
    pub lineage: Lineage,
}

impl Ancestry {
    /// Every candidate needs a direct-parent comparison as well as a frozen
    /// baseline comparison (`INV-004`, `EXP-005`).
    #[must_use]
    pub const fn direct_comparison_required() -> bool {
        true
    }

    /// A multi-parent integration must be measured anew; it cannot inherit a
    /// sum of parent speedups (`ARCHITECTURE.md:135-136`).
    #[must_use]
    pub fn may_inherit_speedup(&self) -> bool {
        !matches!(self.lineage, Lineage::Integration) && self.ancestors.len() <= 1
    }

    /// Whether this ancestry represents an integration candidate.
    #[must_use]
    pub const fn is_integration(&self) -> bool {
        matches!(self.lineage, Lineage::Integration)
    }
}
