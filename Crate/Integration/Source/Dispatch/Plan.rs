//! The `plan` dispatch: Cartesian matrix expansion and line rendering are
//! delegated entirely to `Crate/Benchmark`.
//!
//! Grounded in `Research/prun_lab/README.md` Start (51) and Extend (84-102):
//! refs x builds x feature_sets, exclusions, and a `builds x workloads` total.
//! The Benchmark controller already owns `Matrix::expand`, `Plan::line` and
//! `Plan::footer`; this module only assembles their typed results.

use benchmark::config::Manifest;

use crate::warning::failure::Failure;

/// A rendered plan: one line per variant and one footer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    /// Variant lines in the controller's deterministic product order.
    pub lines: Vec<String>,
    /// The `builds x workloads` summary line.
    pub footer: String,
}

impl Plan {
    /// Delegate to the Benchmark matrix and plan renderer. A typed `Fault`
    /// becomes a typed `Failure`; nothing is printed or panicked here.
    pub fn of(manifest: &Manifest) -> Result<Self, Failure> {
        let matrix = benchmark::plan::Matrix::expand(manifest).map_err(Failure::from)?;
        let lines = matrix
            .variants
            .iter()
            .map(benchmark::controller::Plan::line)
            .collect();
        let footer = benchmark::controller::Plan::footer(matrix.variants.len(), manifest.workloads.len());
        Ok(Self { lines, footer })
    }
}
