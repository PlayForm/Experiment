//! Typed, non-panicking failures for the SUT model.
//!
//! The real binary uses `expect` on CLI lookups, glob compilation and worker
//! joins (`Option.rs:41,50,54`, `Entry.rs:40,43`, `Parallel.rs:142`). A
//! characterization model must instead return a value a caller can print:
//! malformed input degrades to a `Fault`, it never aborts the benchmark loop.

use std::fmt;

/// A typed, recoverable failure. Carried by `Result`, never a panic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fault {
    /// A required value was empty (`Command.rs:60,70`; `Process.rs:22`).
    Empty { field: &'static str },
    /// A glob pattern could not be compiled (`Entry.rs:40,43`).
    Glob { pattern: String, detail: String },
    /// A process could not spawn or exited non-zero (`Process.rs:32,56`).
    Process { command: String, detail: String },
    /// A git index lock never cleared (`Lock.rs:58`).
    LockTimeout { directory: String },
    /// A value failed a domain rule while modelling the surface.
    Malformed { field: String, detail: String },
}

impl Fault {
    /// Render one deterministic diagnostic line (no `if`: each variant is a
    /// structural arm).
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Empty { field } => format!("fault: {field} is empty"),
            Self::Glob { pattern, detail } => {
                format!("fault: glob '{pattern}' is invalid: {detail}")
            }
            Self::Process { command, detail } => {
                format!("fault: process '{command}' failed: {detail}")
            }
            Self::LockTimeout { directory } => {
                format!("fault: git index lock timed out in '{directory}'")
            }
            Self::Malformed { field, detail } => {
                format!("fault: {field} is malformed: {detail}")
            }
        }
    }
}

impl fmt::Display for Fault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.line())
    }
}

impl std::error::Error for Fault {}
