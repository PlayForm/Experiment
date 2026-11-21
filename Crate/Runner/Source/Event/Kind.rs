//! The discriminant of each observed event.
//!
//! Grounded in `Struct/Event.rs:2-8`: `JobStarted`, `Line`, `JobProgress`,
//! `JobFinished`, `IndexLockTimeout`, `AllDone`. Keeping the discriminant
//! separate from the payload lets a benchmark count event kinds without
//! matching on their fields.

use crate::event::event::Event;

/// The kind of an event, independent of its data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Directory started.
    JobStarted,
    /// One stdout/stderr line.
    Line,
    /// One command completed in a directory.
    JobProgress,
    /// Directory finished.
    JobFinished,
    /// Git index lock timed out.
    IndexLockTimeout,
    /// All work is complete.
    AllDone,
}

impl Kind {
    /// All six kinds, in declaration order (`Event.rs:2-8`).
    pub const ALL: [Self; 6] = [
        Self::JobStarted,
        Self::Line,
        Self::JobProgress,
        Self::JobFinished,
        Self::IndexLockTimeout,
        Self::AllDone,
    ];

    /// Project an event onto its discriminant.
    #[must_use]
    pub const fn of(event: &Event) -> Self {
        match event {
            Event::JobStarted { .. } => Self::JobStarted,
            Event::Line { .. } => Self::Line,
            Event::JobProgress { .. } => Self::JobProgress,
            Event::JobFinished { .. } => Self::JobFinished,
            Event::IndexLockTimeout { .. } => Self::IndexLockTimeout,
            Event::AllDone => Self::AllDone,
        }
    }

    /// Whether this kind ends the stream (`Command.rs:64`).
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::AllDone)
    }
}
