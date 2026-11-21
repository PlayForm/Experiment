//! The `-C/--Command <COMMAND>` argument and its accumulation rule.
//!
//! Grounded in `Fn/Binary/Command.rs:63-73` (`ArgAction::Append`,
//! `required(true)`, `allow_hyphen_values(true)`, `value_name` "COMMAND",
//! display order 7) and `Option.rs:52-56`, which clones every value into a
//! `Vec<String>`. `README.md:119,138-144` shows repeated `-C` usage.

/// The ordered list of command strings executed in every target directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Command {
    values: Vec<String>,
}

impl Command {
    /// clap `value_name` (`Command.rs:69`).
    pub const VALUE_NAME: &'static str = "COMMAND";
    /// clap `display_order` (`Command.rs:68`).
    pub const DISPLAY_ORDER: u8 = 7;
    /// `allow_hyphen_values` (`Command.rs:71`).
    pub const ALLOW_HYPHEN_VALUES: bool = true;
    /// clap `required` (`Command.rs:70`).
    pub const REQUIRED: bool = true;

    /// Validate the required, append-style command list.
    pub fn build(values: Vec<String>) -> Result<Self, crate::fault::Fault> {
        match values.is_empty() {
            true => Err(crate::fault::Fault::Empty { field: "Command" }),
            false => Ok(Self { values }),
        }
    }

    /// Commands in the order the user supplied them.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.values.iter().map(String::as_str)
    }

    /// Number of commands, emitted as `Event::JobStarted.total`.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// True only for an unbuilt command list.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
