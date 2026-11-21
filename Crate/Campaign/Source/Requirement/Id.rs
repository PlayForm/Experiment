//! A validated requirement identifier.
//!
//! The specification numbers 17 design laws `LAW-001`..`LAW-017` and groups
//! requirements as `REQ-<FAMILY>-<NN>` (`REQUIREMENTS.md:182-192`, `:216-242`,
//! `:321-334`, `:359-374`, `:405-424`, `:438-451`, `:488-506`, `:580-603`,
//! `:657-671`, `:705-716`, `:736-750`, `:822-830`, `:885-892`, `:915-927`,
//! `:974-999`, `:1099-1108`). An identifier is one word by house style;
//! the physical file keeps the card's `Id.rs` name.

use serde::{Deserialize, Serialize};

/// A parsed requirement code such as `LAW-003` or `REQ-HEU-14`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ID(String);

impl ID {
    /// Parse a code, rejecting malformed identifiers without panicking.
    pub fn parse(text: &str) -> Result<Self, crate::Fault> {
        let parts: Vec<&str> = text.split('-').collect();
        let shape = parts.len() >= 2 && (parts[0] == "LAW" || parts[0] == "REQ");
        let numeric = parts
            .last()
            .and_then(|part| part.split("..").next())
            .is_some_and(|part| part.parse::<u16>().is_ok());
        match shape && numeric {
            true => Ok(Self(text.to_owned())),
            false => Err(crate::Fault::Conflicting {
                domain: "requirement.id",
                detail: format!("unrecognised requirement code {text}"),
            }),
        }
    }

    /// The full code text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The family token, for example `LAW` or `HEU`.
    #[must_use]
    pub fn family(&self) -> &str {
        match self.0.starts_with("LAW-") {
            true => "LAW",
            false => self.0.split('-').nth(1).unwrap_or("REQ"),
        }
    }

    /// The numeric suffix.
    #[must_use]
    pub fn number(&self) -> u16 {
        self.0
            .rsplit('-')
            .next()
            .and_then(|part| part.split("..").next())
            .and_then(|part| part.parse::<u16>().ok())
            .unwrap_or(0)
    }

    /// Whether this code names a non-negotiable design law.
    #[must_use]
    pub fn is_law(&self) -> bool {
        self.0.starts_with("LAW-")
    }
}
