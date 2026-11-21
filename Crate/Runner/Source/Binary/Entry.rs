//! The outer entry point of the SUT.
//!
//! Grounded in `Source/Library.rs:14-16`: `(Struct::Binary::Command::Struct::Fn().Fn)().await`.
//! The returned struct holds a separator and a boxed closure producing a
//! pinned future (`Struct/Binary/Command.rs:10-13,21-77`).

/// The value produced by `Struct::Binary::Command::Struct::Fn()`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entry {
    separator: char,
}

impl Entry {
    /// Capture the OS path separator (`Command.rs:23`).
    #[must_use]
    pub const fn build(separator: char) -> Self {
        Self { separator }
    }

    /// The call chain from `main` to the awaited future (`Library.rs:16`).
    #[must_use]
    pub const fn chain() -> &'static [&'static str] {
        &[
            "Struct::Binary::Command::Struct::Fn()",
            ".Fn",
            "()",
            ".await",
        ]
    }

    /// `std::path::MAIN_SEPARATOR`, threaded into the options struct.
    #[must_use]
    pub const fn separator(&self) -> char {
        self.separator
    }
}
