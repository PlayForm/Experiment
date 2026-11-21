//! The required positional `PATTERN` argument.
//!
//! Grounded in `Fn/Binary/Command.rs:56-62`: no short, no long, `value_name`
//! "PATTERN", `required(true)`, `display_order(6)`, and the help text
//! "The file or directory name that identifies a target directory.".
//! `Option.rs:48-51` reads it with `get_one::<String>("Pattern")`.

/// The name an entry's final component must equal to become a target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pattern {
    value: String,
}

impl Pattern {
    /// clap `value_name` (`Command.rs:59`).
    pub const VALUE_NAME: &'static str = "PATTERN";
    /// clap `display_order` (`Command.rs:58`).
    pub const DISPLAY_ORDER: u8 = 6;
    /// clap `required` (`Command.rs:60`).
    pub const REQUIRED: bool = true;

    /// Validate a required pattern. Empty is a typed fault, not a panic - the
    /// SUT's `expect` at `Option.rs:50` is modelled as graceful degradation.
    pub fn build(value: String) -> Result<Self, crate::fault::Fault> {
        match value.is_empty() {
            true => Err(crate::fault::Fault::Empty { field: "Pattern" }),
            false => Ok(Self { value }),
        }
    }

    /// The pattern text compared against `Path::file_name` by both executors.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}
