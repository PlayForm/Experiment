//! The SUT's event wire type, reproduced exactly.
//!
//! Grounded in `Struct/Event.rs:2-8`. The boolean fields are a deliberate
//! exception to the bool-as-enum rule: this enum *is* the external contract
//! the benchmark observes, so its shape must stay byte-faithful. Typed views
//! are provided by `event::kind::Kind` and `event::payload::Payload`.

/// One message on the bounded `mpsc` channel (`Command.rs:35`, capacity 64).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    /// A directory began executing `total` commands (`Event.rs:3`).
    JobStarted { directory: String, total: usize },
    /// One output line, marked stdout or stderr (`Event.rs:4`).
    Line {
        directory: String,
        text: String,
        is_stderr: bool,
    },
    /// Progress after command `done` of `total` (`Event.rs:5`).
    JobProgress {
        directory: String,
        done: usize,
        total: usize,
        success: bool,
    },
    /// A directory finished, successful or not (`Event.rs:6`).
    JobFinished { directory: String, success: bool },
    /// The git index lock could not be acquired (`Event.rs:7`).
    IndexLockTimeout { directory: String },
    /// The executor has no more work (`Event.rs:8`).
    AllDone,
}

impl Event {
    /// The directory an event is scoped to, when it has one.
    #[must_use]
    pub fn directory(&self) -> Option<&str> {
        match self {
            Self::JobStarted { directory, .. }
            | Self::Line { directory, .. }
            | Self::JobProgress { directory, .. }
            | Self::JobFinished { directory, .. }
            | Self::IndexLockTimeout { directory } => Some(directory),
            Self::AllDone => None,
        }
    }

    /// Only `AllDone` terminates the CLI printer loop (`Command.rs:46-65`).
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Self::AllDone)
    }
}
