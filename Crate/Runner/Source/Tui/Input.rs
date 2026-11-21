//! The input handler's transition table.
//!
//! Grounded in `Fn/Tui/Input.rs:5-57`: `Fn` returns `true` only for quit
//! keys, mouse events never quit (`:8-11`), and a left-button down at
//! `column < 40 && row >= 2` becomes `click_row(row - 2)` (`:50-56`).

use crate::tui::key::Key;

/// The action an input event applies to the TUI state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Input {
    /// Stop the render loop (`Input.rs:19,23`).
    Quit,
    /// Move the selection up (`Input.rs:27`).
    SelectUp,
    /// Move the selection down (`Input.rs:31`).
    SelectDown,
    /// Scroll the selected log up (`Input.rs:35`).
    ScrollUp,
    /// Scroll the selected log down (`Input.rs:39`).
    ScrollDown,
    /// Toggle auto-scroll (`Input.rs:43`).
    ToggleAutoScroll,
    /// Select a directory row from a mouse click (`Input.rs:55`).
    Click { row: usize },
    /// No action (`Input.rs:12,46`).
    Ignore,
}

impl Input {
    /// Resolve a key intent into a state transition.
    #[must_use]
    pub const fn from_key(key: Key) -> Self {
        match key {
            Key::Quit => Self::Quit,
            Key::SelectUp => Self::SelectUp,
            Key::SelectDown => Self::SelectDown,
            Key::ScrollUp => Self::ScrollUp,
            Key::ScrollDown => Self::ScrollDown,
            Key::ToggleAutoScroll => Self::ToggleAutoScroll,
            Key::Other => Self::Ignore,
        }
    }

    /// Resolve a left-button click, reproducing the `40`/`2` hit box.
    #[must_use]
    pub const fn from_click(column: u16, row: u16) -> Self {
        match column < 40 && row >= 2 {
            true => Self::Click {
                row: (row - 2) as usize,
            },
            false => Self::Ignore,
        }
    }

    /// Whether this transition ends the loop (`Input.rs:20,24`).
    #[must_use]
    pub const fn quits(self) -> bool {
        matches!(self, Self::Quit)
    }
}
