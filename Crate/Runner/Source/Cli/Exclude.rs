//! The `-E/--Exclude <PATTERNS>` argument and its default glob.
//!
//! Grounded in `Fn/Binary/Command.rs:46-55` (`ArgAction::Append`,
//! `default_value("**/{node_modules,.git,target,dist,vendor}/**/*")`,
//! display order 5) and `Struct/Binary/Command/Option.rs:42-47`, which
//! flattens each supplied value with `split_whitespace()` into patterns.

/// The compiled list of exclusion globs fed to `GlobSetBuilder`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exclude {
    patterns: Vec<String>,
}

impl Exclude {
    /// The documented default (`README.md:118`, `Command.rs:54`).
    pub const DEFAULT_PATTERN: &'static str =
        "**/{node_modules,.git,target,dist,vendor}/**/*";
    /// clap `value_name` (`Command.rs:52`).
    pub const VALUE_NAME: &'static str = "PATTERNS";
    /// clap `display_order` (`Command.rs:51`).
    pub const DISPLAY_ORDER: u8 = 5;
    /// clap `action` (`Command.rs:50`).
    pub const ACTION_APPEND: bool = true;

    /// Apply the SUT's `flat_map(split_whitespace)` rule to raw `-E` values.
    #[must_use]
    pub fn build(values: Vec<String>) -> Self {
        let patterns = values
            .into_iter()
            .flat_map(|value| value.split_whitespace().map(str::to_owned))
            .collect();

        Self { patterns }
    }

    /// The clap default, already split into its single pattern.
    #[must_use]
    pub fn default_patterns() -> Self {
        Self::build(vec![Self::DEFAULT_PATTERN.to_owned()])
    }

    /// Patterns in declaration order, as handed to `Glob::new` (`Entry.rs:39`).
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.patterns.iter().map(String::as_str)
    }

    /// True when no exclusion globs remain (an empty `GlobSet`).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }
}

impl Default for Exclude {
    fn default() -> Self {
        Self::default_patterns()
    }
}
