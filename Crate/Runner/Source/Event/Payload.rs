//! The typed data carried by each event.
//!
//! Grounded in `Struct/Event.rs:2-8`. The booleans of the raw wire enum are
//! converted here into `Stream` and `Outcome`, so a consumer of the typed view
//! never sees a bare bool.

use crate::event::event::Event;

/// Whether a line came from stdout or stderr (`Event.rs:4`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stream {
    /// `IsStderr: false`.
    Stdout,
    /// `IsStderr: true`; the CLI printer uses `eprintln!`
    /// (`Command.rs:50-55`).
    Stderr,
}

/// The success bit of progress and finish events (`Event.rs:5-6`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    /// `Success: true`.
    Success,
    /// `Success: false`; the CLI prints `✗ Failed:` (`Command.rs:60-62`).
    Failure,
}

/// The fields of an event, made structural.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Payload {
    /// `JobStarted` with its command count (`Event.rs:3`).
    Job { total: usize },
    /// `Line` with text and stream (`Event.rs:4`).
    Line { text: String, stream: Stream },
    /// `JobProgress` with counters and outcome (`Event.rs:5`).
    Progress {
        done: usize,
        total: usize,
        outcome: Outcome,
    },
    /// `JobFinished` with its outcome (`Event.rs:6`).
    Finish { outcome: Outcome },
    /// `IndexLockTimeout`, carrying only a directory on the wire (`Event.rs:7`).
    LockWait,
    /// `AllDone`, carrying nothing (`Event.rs:8`).
    None,
}

impl Payload {
    /// Project an event onto its typed payload.
    #[must_use]
    pub fn of(event: &Event) -> Self {
        match event {
            Event::JobStarted { total, .. } => Self::Job { total: *total },
            Event::Line { text, is_stderr, .. } => Self::Line {
                text: text.clone(),
                stream: match is_stderr {
                    true => Stream::Stderr,
                    false => Stream::Stdout,
                },
            },
            Event::JobProgress {
                done,
                total,
                success,
                ..
            } => Self::Progress {
                done: *done,
                total: *total,
                outcome: Outcome::from_flag(*success),
            },
            Event::JobFinished { success, .. } => Self::Finish {
                outcome: Outcome::from_flag(*success),
            },
            Event::IndexLockTimeout { .. } => Self::LockWait,
            Event::AllDone => Self::None,
        }
    }
}

impl Outcome {
    /// Resolve the SUT's success boolean into a typed outcome.
    #[must_use]
    pub const fn from_flag(success: bool) -> Self {
        match success {
            true => Self::Success,
            false => Self::Failure,
        }
    }
}
