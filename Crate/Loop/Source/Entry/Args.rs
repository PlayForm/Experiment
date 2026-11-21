//! Typed arguments for the `prun loop` launch.
//! Grounded in the Integration crate's `prun loop` command and the bounded
//! budget knobs of `autonomous-week.yaml:9-22`. Unknown tokens degrade to a
//! warning instead of panicking.

use crate::warning::Warning;

use crate::propose::scope::Scope;

/// Parsed loop arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Args {
    /// Optional wall-clock hour override.
    pub hours_override: Option<u32>,
    /// Optional single-scope restriction.
    pub scope: Option<Scope>,
    /// Degradations collected while parsing.
    pub warnings: Vec<Warning>,
}

impl Args {
    /// Parse a token slice into typed arguments.
    #[must_use]
    pub fn parse(words: &[String]) -> Self {
        words
            .iter()
            .fold(
                Self {
                    hours_override: None,
                    scope: None,
                    warnings: Vec::new(),
                },
                |mut args, token| {
                    match token.strip_prefix("--scope=") {
                        Some(name) => match Scope::ALL.iter().find(|scope| scope.key() == name) {
                            Some(scope) => args.scope = Some(*scope),
                            None => args.warnings.push(Warning::of("unknown scope token")),
                        },
                        None => match token.strip_prefix("--hours=") {
                            Some(text) => match text.parse::<u32>() {
                                Ok(hours) => args.hours_override = Some(hours),
                                Err(()) => args.warnings.push(Warning::of("malformed hours token")),
                            },
                            None => args.warnings.push(Warning::of("unknown loop token")),
                        },
                    }
                    args
                },
            )
    }

    /// Whether the arguments parsed without degradation.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.warnings.is_empty()
    }
}
