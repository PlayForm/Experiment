//! Structural `$ref` descriptor.
//!
//! References in the corpus are relative (`./artifact-ref.schema.json`,
//! `../schemas/campaign.schema.json`) or absolute
//! (`https://schemas.lab.invalid/v6/...`); AsyncAPI uses local JSON pointers
//! (`#/components/messages/AuditEvent`). A malformed reference is a typed
//! fault; a reference whose target is absent becomes a validation finding, not
//! a panic (`docs/26-acceptance-and-build-plan.md:60-63`).

use crate::Fault;

/// A schema reference target.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Reference {
    target: &'static str,
}

impl Reference {
    /// Build a reference from trusted catalog data.
    #[must_use]
    pub const fn of(target: &'static str) -> Self {
        Self { target }
    }

    /// Validate a reference to a borrowed trusted spelling.
    #[must_use]
    pub fn parse(target: &'static str) -> Result<Self, Fault> {
        let shaped = target.starts_with("./")
            || target.starts_with("../")
            || target.starts_with("#/")
            || target.starts_with("https://schemas.lab.invalid/");
        match shaped {
            true => Ok(Self::of(target)),
            false => Err(Fault::Shape {
                domain: "schema reference",
                detail: format!("{target} has no recognized reference prefix"),
            }),
        }
    }

    /// Borrow the target string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.target
    }

    /// Whether this is a local JSON pointer.
    #[must_use]
    pub fn is_fragment(&self) -> bool {
        self.target.starts_with('#')
    }
}
