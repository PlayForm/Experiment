//! A projected report of loop progress.
//! Grounded in `26-acceptance-and-build-plan.md:57-60` (the operator inspects
//! status through public, read-only projections) and `ARCHITECTURE.md:93-96`
//! (the director receives generation summaries with budget and progress
//! signals). A report is never authoritative measurement.

/// A read-only projection of loop progress.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Report {
    /// Rendered operator lines.
    pub lines: Vec<String>,
}

impl Report {
    /// Build a report from already-trusted summary lines.
    #[must_use]
    pub fn of(lines: Vec<String>) -> Self {
        Self { lines }
    }

    /// A report is a projection, never a measurement authority.
    #[must_use]
    pub const fn is_projection(&self) -> bool {
        true
    }

    /// Whether the report carries any line.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
