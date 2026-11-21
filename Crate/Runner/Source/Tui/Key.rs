//! The resolved meaning of a crossterm key press.
//!
//! Grounded in `Fn/Tui/Input.rs:16-48`: `q`/`Q` and Ctrl-`c` force quit;
//! `Up`/`k` select up; `Down`/`j` select down; `PageUp`/`PageDown` scroll;
//! `s`/`S` toggles autoscroll; every other key is ignored (`:46`).

/// One key intent, independent of the crossterm event that produced it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    /// Force the loop to break (`Input.rs:18-25`).
    Quit,
    /// `Up` or `k` (`Input.rs:26-29`).
    SelectUp,
    /// `Down` or `j` (`Input.rs:30-33`).
    SelectDown,
    /// `PageUp` (`Input.rs:34-37`).
    ScrollUp,
    /// `PageDown` (`Input.rs:38-41`).
    ScrollDown,
    /// `s` or `S` (`Input.rs:42-45`).
    ToggleAutoScroll,
    /// Unhandled input (`Input.rs:46`).
    Other,
}

impl Key {
    /// Resolve a character key, with its control modifier, into an intent.
    #[must_use]
    pub const fn from_char(character: char, control: bool) -> Self {
        match (character, control) {
            ('q' | 'Q', _) => Self::Quit,
            ('c', true) => Self::Quit,
            ('k' | 'K', _) => Self::SelectUp,
            ('j' | 'J', _) => Self::SelectDown,
            ('s' | 'S', _) => Self::ToggleAutoScroll,
            _ => Self::Other,
        }
    }

    /// Resolve the two arrow keys not reachable through `from_char`.
    #[must_use]
    pub const fn from_arrow(up: bool) -> Self {
        match up {
            true => Self::SelectUp,
            false => Self::SelectDown,
        }
    }
}
