//! The `-R/--Root <DIRECTORY>` argument and its documented default.
//!
//! Grounded in `Fn/Binary/Command.rs:37-45` (short R, long "Root",
//! `value_name` "DIRECTORY", `default_value(".")`, display order 4) and
//! `README.md:117,157-161`. `Option.rs:41` reads it with `get_one`.

/// The directory the walker starts from (`Entry.rs:45`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Root {
    value: String,
}

impl Root {
    /// The clap default (`Command.rs:44`, `README.md:117`).
    pub const DEFAULT: &'static str = ".";
    /// clap `value_name` (`Command.rs:42`).
    pub const VALUE_NAME: &'static str = "DIRECTORY";
    /// clap `display_order` (`Command.rs:41`).
    pub const DISPLAY_ORDER: u8 = 4;
    /// Short and long spellings (`Command.rs:39-40`).
    pub const SHORT: char = 'R';
    /// Long spelling (`Command.rs:40`).
    pub const LONG: &'static str = "Root";

    /// Validate a root path without touching the filesystem.
    pub fn build(value: String) -> Result<Self, crate::fault::Fault> {
        match value.is_empty() {
            true => Err(crate::fault::Fault::Empty { field: "Root" }),
            false => Ok(Self { value }),
        }
    }

    /// The root as passed to `WalkDir::new` (`Entry.rs:45`).
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl Default for Root {
    fn default() -> Self {
        Self {
            value: Self::DEFAULT.to_owned(),
        }
    }
}
