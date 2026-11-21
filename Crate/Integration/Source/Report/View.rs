//! The rendered view of one composed invocation.
//!
//! Grounded in `Research/prun_lab/README.md` (59-60, 132-134): results are
//! retained as CSV plus Hyperfine JSON and metadata, and a report is a
//! projection, never an authoritative measurement. This view only formats the
//! typed output and warnings the delegated models produced; it promotes
//! nothing and claims no benchmark result.

use crate::compose::wiring::Wiring;
use crate::warning::failure::Failure;

/// A read-only, line-oriented view of a `Wiring`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct View {
    lines: Vec<String>,
}

impl View {
    /// Assemble a header, the crate chain, all warnings and the delegated
    /// output through one named iterator chain (no branch, no loop).
    #[must_use]
    pub fn of(wiring: &Wiring) -> Self {
        let header = [format!("prun: {} crate(s) composed", wiring.chain.len())];
        let chain = wiring
            .chain
            .iter()
            .map(|item| format!("  {} -> {}", item.package(), item.model()));
        let warnings = wiring.warnings.iter().map(Failure::warning);
        let output = wiring.output.iter().cloned();
        let lines = header
            .into_iter()
            .chain(chain)
            .chain(warnings)
            .chain(output)
            .collect();
        Self { lines }
    }

    /// The rendered lines in order.
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.lines.iter().map(String::as_str)
    }
}
