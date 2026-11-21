//! CSV summary document renderer.
//!
//! Grounded in `prun_lab.py:630-635`: the report writes a header from the
//! first row's keys followed by one line per result entry, into
//! `summary.csv`.

use super::Row;

/// An ordered set of CSV summary rows.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Csv {
    pub rows: Vec<Row>,
}

impl Csv {
    /// Filename written per session (prun_lab.py:630).
    pub const NAME: &'static str = "summary.csv";

    /// Render header and rows as CSV text.
    #[must_use]
    pub fn render(&self) -> String {
        let header = Row::HEADERS.join(",");
        let lines = std::iter::once(header)
            .chain(self.rows.iter().map(|row| row.cells().join(",")))
            .collect::<Vec<_>>();
        format!("{}\n", lines.join("\n"))
    }
}
